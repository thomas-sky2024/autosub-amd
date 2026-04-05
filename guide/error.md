lỗi khi build cho Mac Intel (qua Github Runner) 
   Compiling tower v0.5.3
   Compiling hyper-util v0.1.20
   Compiling anstyle-parse v1.0.0
   Compiling sigchld v0.2.4
   Compiling tokio-native-tls v0.3.1
error: failed to run custom build command for `sherpa-onnx-sys v1.12.34`
Caused by:
  process didn't exit successfully: `/Users/runner/work/autosub-amd/autosub-amd/src-tauri/target/release/build/sherpa-onnx-sys-fec8e1f161f16d55/build-script-build` (exit status: 101)
  --- stdout
  cargo:rerun-if-env-changed=SHERPA_ONNX_LIB_DIR
  cargo:rerun-if-env-changed=SHERPA_ONNX_ARCHIVE_DIR
  cargo:rerun-if-env-changed=DOCS_RS
  --- stderr
  Downloading sherpa-onnx libs from https://github.com/k2-fsa/sherpa-onnx/releases/download/v1.12.34/sherpa-onnx-v1.12.34-osx-x64-static-lib.tar.bz2
  thread 'main' (45295) panicked at /Users/runner/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sherpa-onnx-sys-1.12.34/build.rs:40:9:
  Failed to download sherpa-onnx archive from https://github.com/k2-fsa/sherpa-onnx/releases/download/v1.12.34/sherpa-onnx-v1.12.34-osx-x64-static-lib.tar.bz2: https://github.com/k2-fsa/sherpa-onnx/releases/download/v1.12.34/sherpa-onnx-v1.12.34-osx-x64-static-lib.tar.bz2: status code 502
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
warning: build failed, waiting for other jobs to finish...
failed to build app: failed to build app
       Error failed to build app: failed to build app
 ELIFECYCLE  Command failed with exit code 1.
Error: Command "pnpm ["tauri","build","--target","x86_64-apple-darwin"]" failed with exit code 1

=== lỗi khi tải yt-dlp
❌ Download error: ERROR: [youtube] 8jer0kCcsAg: Requested format is not available. Use --list-formats for a list of available formats