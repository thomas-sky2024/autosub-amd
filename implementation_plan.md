# Phối hợp tối ưu Whisper cho máy Mac Intel 2019 (AMD GPU)

Người dùng gặp tình trạng chương trình bị đơ giữa chừng trên máy Mac 2019 (AMD GPU). Tài liệu `auto-sub/amd.md` đã chỉ rõ đây là lỗi "Metal command buffer timeout" phổ biến trên dòng phần cứng này khi GPU bị quá tải hoặc driver không phản hồi kịp.

## Nhận định kỹ thuật

1.  **OpenCL vs Metal**: Nhận định về OpenCL là **không đúng** trên macOS hiện đại. Apple đã bỏ rơi OpenCL từ lâu. Metal là con đường duy nhất để tận dụng GPU hiệu quả, nhưng cần được điều tiết để tránh gây treo hệ thống (kernel panic hoặc GPU hang) trên các dòng AMD Radeon Pro đời cũ.
2.  **whisper-rs**: Phiên bản hiện tại (v0.16) đã ngưng phát triển (archived) và sử dụng core `whisper.cpp` cũ, thiếu các bản vá lỗi Metal cho dGPU.
3.  **Giải pháp**: Chuyển sang **whisper-cpp-plus**, một binding chủ động hơn, hỗ trợ phiên bản `whisper.cpp` mới nhất (v1.8.x) và cho phép kiểm soát số lượng layer offload lên GPU (`n_gpu_layers`).

## User Review Required

> [!IMPORTANT]
> Việc chuyển đổi engine sẽ thay đổi cách cấu hình tham số. Chúng ta sẽ áp dụng các giới hạn cứng cho máy Intel Mac để đảm bảo tính ổn định:
> - Giảm `n_gpu_layers` xuống mức an toàn (16-20 layers) thay vì offload toàn bộ.
> - Ưu tiên model `large-v3-turbo` vì nó nhẹ hơn rất nhiều ở phần decoder.

## Proposed Changes

### 1. Cấu hình Dependency

#### [MODIFY] [Cargo.toml](file:///Users/tt/Documents/vibe-coding/whisper/auto-sub/src-tauri/Cargo.toml)
- Gỡ bỏ `whisper-rs`.
- Thêm `whisper-cpp-plus = { version = "0.1", features = ["metal"] }`.

### 2. Tái cấu trúc Engine Native

#### [MODIFY] [whisper_native.rs](file:///Users/tt/Documents/vibe-coding/whisper/auto-sub/src-tauri/src/whisper_native.rs) [NEW_BINDING]
- Thay đổi import và logic khởi tạo sang `whisper-cpp-plus`.
- Thêm logic tự động nhận diện phần cứng (Intel + AMD) để điều chỉnh tham số:
  - Nếu là Intel Mac: Set `n_gpu_layers` mặc định là 16 (cho large-v3) hoặc 28 (cho turbo).
  - Sử dụng `num_cpus` hợp lý cho phần coordinator.
- Áp dụng các kỹ thuật xử lý timeout đã đề cập trong `amd.md`.

## Open Questions

- Bạn có muốn thêm một mục trong giao diện (Settings) để người dùng tự tay chỉnh `GPU Offload Layers` không? Điều này sẽ giúp người dùng có máy mạnh hơn 2019 vẫn tận dụng được tối đa.

## Verification Plan

### Automated Tests
- Chạy `cargo build` để đảm bảo tương thích binding mới.
- Kiểm tra log khởi tạo để xác nhận số layer offload được áp dụng chính xác.

### Manual Verification
- Chạy thử với model `large-v3-turbo` và `large-v3 q5` trên máy Mac Intel (nếu có thể) để quan sát mức độ chiếm dụng GPU.
- Xác nhận không còn tình trạng "status 5" trong log Metal của hệ thống.
