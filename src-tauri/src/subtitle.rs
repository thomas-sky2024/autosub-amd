//! Native Whisper — whisper-cpp-plus (non-streaming)
//! Tối ưu cho cả Mac Intel 2019 (AMD GPU) và Apple Silicon
//! Hỗ trợ build universal-apple-darwin

use crate::error::{AutoSubError, Result};
use crate::subtitle::Segment;
use log::info;
use std::sync::Arc;
use tokio::sync::mpsc;
use whisper_cpp_plus::{TranscriptionParams, WhisperContext};
use std::time::Instant;
use num_cpus;

// ── Hardware Detection ─────────────────────────────────────────────────────

fn is_apple_silicon() -> bool {
    cfg!(target_arch = "aarch64") && cfg!(target_os = "macos")
}

fn is_intel_mac_amd() -> bool {
    cfg!(target_arch = "x86_64") && !is_apple_silicon()
}

// ── Types ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct WhisperNativeProgress {
    pub percent: f32,
    pub segment_count: usize,
}

// ── Engine ─────────────────────────────────────────────────────────────────

pub struct WhisperEngine {
    ctx: Arc<WhisperContext>,
}

impl WhisperEngine {
    pub fn load(model_path: &str) -> Result<Self> {
        let hardware = if is_apple_silicon() {
            "Apple Silicon (M1/M2/M3/M4)"
        } else if is_intel_mac_amd() {
            "Intel 2019 + AMD GPU"
        } else {
            "Unknown"
        };

        info!("🔧 WhisperEngine init → Hardware: {}", hardware);

        if is_apple_silicon() {
            std::env::set_var("GGML_METAL_N_CB", "4");
            std::env::set_var("GGML_METAL_MAX_BUFFER_SIZE", "32212254720");
            std::env::set_var("GGML_METAL_USE_IO", "1");
            std::env::set_var("GGML_METAL_F16", "1");
        } else if is_intel_mac_amd() {
            std::env::set_var("GGML_METAL_N_CB", "2");
            std::env::set_var("GGML_METAL_MAX_BUFFER_SIZE", "2147483648");
            std::env::set_var("GGML_METAL_MPS", "0");
        }

        let ctx = WhisperContext::new(model_path)
            .map_err(|e| AutoSubError::WhisperDecode(format!("Load model failed: {}", e)))?;

        info!("✅ Model loaded successfully on Metal ({})", hardware);
        Ok(Self { ctx: Arc::new(ctx) })
    }

    pub async fn transcribe(
        &self,
        samples: Vec<f32>,
        language: &str,
        threads: Option<usize>,
        progress_tx: Option<mpsc::Sender<WhisperNativeProgress>>,
        _abort_flag: Option<Arc<std::sync::atomic::AtomicBool>>,
    ) -> Result<Vec<Segment>> {
        let ctx = self.ctx.clone();
        
        // Chuyển thành owned String để move vào spawn_blocking
        let lang: Option<String> = if language == "auto" || language.is_empty() {
            None
        } else {
            Some(language.to_string())
        };

        let n_threads = threads.unwrap_or_else(|| {
            if is_apple_silicon() {
                num_cpus::get_physical().max(1)
            } else {
                4
            }
        });

        let start = Instant::now();

        let result = tokio::task::spawn_blocking(move || -> Result<Vec<Segment>> {
            let params = TranscriptionParams::builder()
                .language(lang.as_deref().unwrap_or(""))
                .n_threads(n_threads as i32)
                .enable_timestamps()
                .build();

            let transcription = ctx.transcribe_with_params(&samples, params)
                .map_err(|e| AutoSubError::WhisperDecode(e.to_string()))?;

            let segments: Vec<Segment> = transcription
                .segments
                .into_iter()
                .map(|s| Segment {
                    start: s.start_ms as f32 / 1000.0,
                    end: s.end_ms as f32 / 1000.0,
                    text: s.text,
                    ..Default::default()
                })
                .collect();

            if let Some(tx) = &progress_tx {
                let _ = tx.try_send(WhisperNativeProgress {
                    percent: 100.0,
                    segment_count: segments.len(),
                });
            }

            info!("Transcription completed in {:.2}s on {}", 
                start.elapsed().as_secs_f32(), 
                if is_apple_silicon() { "Apple Silicon" } else { "Intel AMD" }
            );

            Ok(segments)
        })
        .await
        .map_err(|e| AutoSubError::WhisperDecode(format!("Task panicked: {}", e)))??;

        Ok(result)
    }
}