# CameraCupte PRO 📸

![CameraCupte Hero](/assets/hero.png)

**CameraCupte** is a professional, high-performance remote controller and AI virtual webcam suite for Canon EOS cameras (optimized for the EOS 600D). 

Built with **Rust (Tauri)** for maximum performance and **MediaPipe AI** for intelligent subject tracking, it transforms your DSLR into a professional streaming and cinematography rig.

## ✨ Key Features

- **🤖 AI Subject Tracking**: Neural-network driven focus that follows your face and eyes in real-time.
- **🌫️ Smart Portrait Mode**: GPU-accelerated background blur that follows you, creating a cinematic depth-of-field effect.
- **🎯 Precision Focus**: Manual micro-stepping (S1, S2, S3) and proactive "Smart Focus" peaking.
- **🎛️ Full Hardware Mastery**: Direct control over ISO, Aperture, Shutter Speed, White Balance, Picture Styles, and Exposure Compensation.
- **🎥 Virtual Webcam**: High-quality V4L2 output compatible with Zoom, OBS, Discord, and more.
- **⚡ Persistent Pipeline**: Auto-detects and auto-reconnects to your camera the moment it's plugged in.

## 🚀 Getting Started

### Prerequisites (Linux)
Ensure you have `v4l2loopback` installed for virtual webcam support:
```bash
sudo modprobe v4l2loopback devices=1 video_nr=9 card_label="CameraCupte" exclusive_caps=1
```

### Installation
1. Clone the repository
2. Install dependencies: `npm install`
3. Run in development: `npm run tauri dev`
4. Build for production: `npm run tauri build`

## 🛠️ Technology Stack
- **Frontend**: Vanilla HTML5/JS/CSS with Glassmorphic Design.
- **Backend**: Rust (Tauri v2).
- **Camera Protocol**: `gphoto2` (PTP).
- **AI Engine**: MediaPipe Face Detection.
- **Streaming**: V4L2 (Linux Virtual Video).

## 📄 License
MIT

---
Developed with ❤️ by [@Zviel](zviel.com) (@zvielkoren)for a professional Canon experience.
