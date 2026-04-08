//! Native Whisper — whisper-cpp-plus (whisper.cpp v1.8.x) + Streaming API
//! Tối ưu cho Mac Intel 2019 (AMD) và Mac M1/M2/M3/M4

use crate::error::{AutoSubError, Result};
use crate::subtitle::Segment;
use log::info;
use std::sync::Arc;
use tokio::sync::mpsc;
use whisper_cpp_plus::{FullParams, SamplingStrategy, WhisperContext, WhisperStream};
use std::sync::atomic::{AtomicBool, Ordering};
use num_cpus;

// ── Hardware Detection & Safe GPU Layers ─────────────────────────────────────

fn is_intel_mac_amd() -> bool {
    cfg!(target_arch = "x86_64") && !is_apple_silicon()
}

fn is_apple_silicon() -> bool {
    cfg!(target_arch = "aarch64") && cfg!(target_os = "macos")
}

fn get_safe_gpu_layers(variant: &ModelVariant) -> i32 {
    if is_intel_mac_amd() {
        match variant {
            ModelVariant::LargeV3Turbo => 28,
            ModelVariant::LargeV3 | ModelVariant::LargeV2 => 16,
            _ => 12,
        }
    } else if is_apple_silicon() {
        99
    } else {
        0
    }
}

// ── Types ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct WhisperNativeProgress {
    pub percent: f32,
    pub segment_count: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ModelVariant {
    LargeV2,
    LargeV3,
    LargeV3Turbo,
    Unknown,
}

impl ModelVariant {
    pub fn from_path(path: &str) -> Self {
        let p = path.to_lowercase();
        if p.contains("large-v3-turbo") || p.contains("large_v3_turbo") {
            Self::LargeV3Turbo
        } else if p.contains("large-v3") || p.contains("large_v3") {
            Self::LargeV3
        } else if p.contains("large-v2") || p.contains("large_v2") {
            Self::LargeV2
        } else {
            Self::Unknown
        }
    }

    pub fn recommended_threads() -> usize {
        if is_apple_silicon() {
            num_cpus::get_physical().max(1)
        } else {
            4
        }
    }
}

// ── Engine ────────────────────────────────────────────────────────────────────

pub struct WhisperEngine {
    ctx: Arc<WhisperContext>,
    variant: ModelVariant,
}

impl WhisperEngine {
    pub fn load(model_path: &str) -> Result<Self> {
        let variant = ModelVariant::from_path(model_path);
        let n_gpu_layers = get_safe_gpu_layers(&variant);

        info!("🔧 WhisperEngine init → Model: {:?} | GPU layers: {}", variant, n_gpu_layers);
        info!("🖥️  Hardware: Intel AMD → {}", is_intel_mac_amd());

        if is_intel_mac_amd() {
            std::env::set_var("GGML_METAL_N_CB", "2");
            std::env::set_var("GGML_METAL_MAX_BUFFER_SIZE", "2147483648");
            std::env::set_var("GGML_METAL_MPS", "0");
        }

        let ctx = WhisperContext::new(model_path)
            .map_err(|e| AutoSubError::WhisperDecode(format!("Load model failed: {}", e)))?;

        Ok(Self { ctx: Arc::new(ctx), variant })
    }

    pub async fn transcribe(
        &self,
        samples: Vec<f32>,
        language: &str,
        threads: Option<usize>,
        progress_tx: Option<mpsc::Sender<WhisperNativeProgress>>,
        abort_flag: Option<Arc<AtomicBool>>,
    ) -> Result<Vec<Segment>> {
        let ctx = self.ctx.clone();
        let language = if language == "auto" || language.is_empty() { None } else { Some(language) };
        let n_threads = threads.unwrap_or_else(ModelVariant::recommended_threads);

        let result = tokio::task::spawn_blocking(move || {
            let params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 })
                .language(language.unwrap_or(""))
                .n_threads(n_threads as i32)
                .token_timestamps(true)
                .split_on_word(true)
                .max_len(42)
                .thold_pt(0.6)
                .logprob_thold(-1.0)
                .print_special(false)
                .print_progress(false)
                .print_realtime(false)
                .print_timestamps(false);

            let mut stream = WhisperStream::new(&ctx, params)
                .map_err(|e| AutoSubError::WhisperDecode(format!("Stream init failed: {}", e)))?;

            let chunk_size = 16000;
            let mut segments = Vec::new();
            let total_samples = samples.len() as i64;
            let mut processed = 0usize;

            for chunk in samples.chunks(chunk_size) {
                if let Some(flag) = &abort_flag {
                    if flag.load(Ordering::Relaxed) {
                        return Err(AutoSubError::WhisperDecode("Operation aborted by user".into()));
                    }
                }

                stream.feed_audio(chunk);

                if let Some(new_segs) = stream.process_step()
                    .map_err(|e| AutoSubError::WhisperDecode(e.to_string()))?
                {
                    segments.extend(new_segs);
                }

                processed += chunk.len();
                if let Some(tx) = &progress_tx {
                    let percent = (processed as f64 / total_samples as f64 * 100.0) as f32;
                    let _ = tx.try_send(WhisperNativeProgress {
                        percent: percent.min(100.0),
                        segment_count: segments.len(),
                    });
                }
            }

            let final_segs = stream.flush()
                .map_err(|e| AutoSubError::WhisperDecode(e.to_string()))?;
            segments.extend(final_segs);

            // Convert sang Segment (đã cast f32)
            let result: Vec<Segment> = segments
                .into_iter()
                .map(|s| Segment {
                    start: s.start_ms as f32 / 1000.0,
                    end: s.end_ms as f32 / 1000.0,
                    text: s.text,
                    ..Default::default()
                })
                .collect();

            Ok(result)
        })
        .await
        .map_err(|e| AutoSubError::WhisperDecode(format!("Task panicked: {}", e)))??;

        Ok(result)
    }
}