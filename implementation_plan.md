# Canon Virtual Webcam Implementation Plan

This plan outlines the architecture and implementation for a Rust-based virtual webcam application targeting a Canon EOS 600D on Arch Linux. The goal is to provide a low-latency, concurrent pipeline for capturing MJPEG previews, decoding them, and writing them to a V4L2 loopback device, while exposing controls for ISO and Aperture.

## User Review Required

> [!IMPORTANT]
> **V4L2 Format Selection**: The plan currently assumes decoding the camera's MJPEG stream into YUYV or RGB before writing to `v4l2loopback`. While `v4l2loopback` *can* pass through MJPEG, many consumers (like web browsers) prefer uncompressed formats like YUYV. Please confirm if you want the pipeline to decode to YUYV using `turbojpeg`, or if you prefer to pipe the raw MJPEG directly.
> 
> **Camera Settings Mapping**: Canon PTP controls for ISO and Aperture require specific hex codes or string values. We will implement basic PTP command passing, but you may need to tweak the exact values for the 600D.

## Open Questions

> [!WARNING]
> 1. Which `/dev/videoX` device number should we use as the default loopback device?
> 2. Do you have a preferred resolution and framerate for the 600D's Live View output, or should we dynamically adapt to whatever the camera outputs?

## Proposed Changes

We will create a modular Rust project structure with `cargo`.

---

### Project Configuration

#### [NEW] Cargo.toml
The manifest will include the following key dependencies:
- `gphoto2`: For PTP camera interaction and Live View capture.
- `v4l`: To interact with the `/dev/videoX` loopback device.
- `tokio`: With `rt-multi-thread` and `macros` features for async task management and channels.
- `turbojpeg`: For hardware-accelerated/SIMD-optimized MJPEG decoding to YUYV/RGB.
- `anyhow`: For robust error handling.
- `tracing` / `tracing-subscriber`: For clean logging and debugging.

---

### Core Modules

#### [NEW] src/main.rs
The entry point. Sets up Tokio runtime, initializes the modules, and spawns the core concurrent tasks:
1. **Capture Task**: A dedicated thread (using `spawn_blocking` since `gphoto2` is synchronous) to pull Live View frames as fast as possible.
2. **Decoder & Output Task**: Receives MJPEG frames via a bounded channel, decodes them with `turbojpeg`, and writes to `v4l`.
3. **Control Task**: Listens for CLI commands or signals to change ISO/Aperture and sends them to the camera via a control channel.

#### [NEW] src/camera.rs
Handles all `gphoto2` operations.
- `CameraManager`: Struct to initialize the context, detect the Canon 600D, and enter Live View.
- `capture_preview`: Continuously fetches preview frames (MJPEG).
- `set_config`: Exposes methods to set PTP configuration values like ISO and Aperture using `gphoto2` config widgets.

#### [NEW] src/decoder.rs
Wraps `turbojpeg` functionality.
- Takes incoming MJPEG byte slices.
- Decompresses them into raw YUYV or RGB buffers. This step prevents the `v4l` consumer from having to handle MJPEG decoding, ensuring broader compatibility.

#### [NEW] src/v4l2_out.rs
Manages the `v4l2loopback` device via the `v4l` crate.
- Opens the specified `/dev/videoX` device.
- Sets the format (e.g., `FourCC::YUYV`), width, and height based on the first decoded frame.
- Provides a `write_frame` method to blast the raw uncompressed bytes into the loopback device.

## Verification Plan

### Automated/Compilation Tests
- `cargo build` to ensure all crates link properly against system libraries (`libgphoto2`, `libturbojpeg`).

### Manual Verification
1. Ensure the `v4l2loopback` kernel module is loaded: `sudo modprobe v4l2loopback exclusive_caps=1`.
2. Connect the Canon 600D via USB.
3. Run the Rust application: `cargo run --release`.
4. Open a webcam consumer like `ffplay /dev/videoX`, OBS Studio, or a web browser to verify the low-latency video stream.
5. Attempt to change ISO/Aperture via the application's control interface and verify the exposure changes in the video feed.
