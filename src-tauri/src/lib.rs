pub mod camera;
pub mod decoder;
#[cfg(target_os = "linux")]
pub mod v4l2_out;

use std::thread;
use std::sync::mpsc::{channel, Sender};
use std::sync::Mutex;
use std::process::Command;
use tauri::{AppHandle, State, Emitter};
use base64::{engine::general_purpose, Engine as _};

struct AppState {
    tx: Mutex<Option<Sender<CameraCmd>>>,
}

enum CameraCmd {
    SetIso(String),
    SetAperture(String),
    SetShutterSpeed(String),
    ManualFocus(i32),
    TriggerAutofocus,
    SetFocusMode(String),
    SetWhiteBalance(String),
    SetPictureStyle(String),
    SetExposureComp(String),
    SetMeteringMode(String),
    SetFlashMode(String),
    SetZoomPosition(u32, u32),
    StopStream,
    GetConfig,
}

impl CameraCmd {
    /// Human-readable label for error reporting
    fn label(&self) -> &'static str {
        match self {
            CameraCmd::SetIso(_) => "ISO",
            CameraCmd::SetAperture(_) => "Aperture",
            CameraCmd::SetShutterSpeed(_) => "Shutter Speed",
            CameraCmd::ManualFocus(_) => "Manual Focus",
            CameraCmd::TriggerAutofocus => "Autofocus",
            CameraCmd::SetFocusMode(_) => "Focus Mode",
            CameraCmd::SetWhiteBalance(_) => "White Balance",
            CameraCmd::SetPictureStyle(_) => "Picture Style",
            CameraCmd::SetExposureComp(_) => "Exposure Comp",
            CameraCmd::SetMeteringMode(_) => "Metering Mode",
            CameraCmd::SetFlashMode(_) => "Flash Mode",
            CameraCmd::SetZoomPosition(_, _) => "Zoom Position",
            CameraCmd::StopStream => "Stop",
            CameraCmd::GetConfig => "Config",
        }
    }
}

#[tauri::command]
fn setwhitebalance(state: State<'_, AppState>, wb: String) -> Result<(), String> {
    if let Some(tx) = state.tx.lock().unwrap().as_ref() {
        tx.send(CameraCmd::SetWhiteBalance(wb)).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn setpicturestyle(state: State<'_, AppState>, style: String) -> Result<(), String> {
    if let Some(tx) = state.tx.lock().unwrap().as_ref() {
        tx.send(CameraCmd::SetPictureStyle(style)).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn setexposurecompensation(state: State<'_, AppState>, ev: String) -> Result<(), String> {
    if let Some(tx) = state.tx.lock().unwrap().as_ref() {
        tx.send(CameraCmd::SetExposureComp(ev)).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn setmeteringmode(state: State<'_, AppState>, mode: String) -> Result<(), String> {
    if let Some(tx) = state.tx.lock().unwrap().as_ref() {
        tx.send(CameraCmd::SetMeteringMode(mode)).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn setflashmode(state: State<'_, AppState>, mode: String) -> Result<(), String> {
    if let Some(tx) = state.tx.lock().unwrap().as_ref() {
        tx.send(CameraCmd::SetFlashMode(mode)).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn debugcamera(state: State<'_, AppState>) -> Result<(), String> {
    if let Some(tx) = state.tx.lock().unwrap().as_ref() {
        tx.send(CameraCmd::GetConfig).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn stopcamera(state: State<'_, AppState>) -> Result<(), String> {
    if let Some(tx) = state.tx.lock().unwrap().as_ref() {
        tx.send(CameraCmd::StopStream).map_err(|e| e.to_string())?;
    }
    *state.tx.lock().unwrap() = None;
    Ok(())
}

#[tauri::command]
fn createvirtualdevice() -> Result<String, String> {
    #[cfg(target_os = "linux")]
    {
        if std::path::Path::new("/dev/video9").exists() {
            return Ok("/dev/video9".to_string());
        }

        let output = Command::new("pkexec")
            .arg("modprobe")
            .arg("v4l2loopback")
            .arg("video_nr=9")
            .arg("card_label=CameraCupte Virtual Webcam")
            .arg("exclusive_caps=1")
            .output()
            .map_err(|e| e.to_string())?;

        if output.status.success() {
            Ok("/dev/video9".to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }
    #[cfg(not(target_os = "linux"))]
    {
        Ok("Virtual device not supported on this platform yet (Preview only)".to_string())
    }
}

#[tauri::command]
fn setiso(state: State<'_, AppState>, iso: String) -> Result<(), String> {
    if let Some(tx) = state.tx.lock().unwrap().as_ref() {
        tx.send(CameraCmd::SetIso(iso)).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn setaperture(state: State<'_, AppState>, aperture: String) -> Result<(), String> {
    if let Some(tx) = state.tx.lock().unwrap().as_ref() {
        tx.send(CameraCmd::SetAperture(aperture)).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn setshutterspeed(state: State<'_, AppState>, speed: String) -> Result<(), String> {
    if let Some(tx) = state.tx.lock().unwrap().as_ref() {
        tx.send(CameraCmd::SetShutterSpeed(speed)).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn manualfocus(state: State<'_, AppState>, direction: i32) -> Result<(), String> {
    if let Some(tx) = state.tx.lock().unwrap().as_ref() {
        tx.send(CameraCmd::ManualFocus(direction)).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn triggerautofocus(state: State<'_, AppState>) -> Result<(), String> {
    if let Some(tx) = state.tx.lock().unwrap().as_ref() {
        tx.send(CameraCmd::TriggerAutofocus).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn setfocusmode(state: State<'_, AppState>, mode: String) -> Result<(), String> {
    if let Some(tx) = state.tx.lock().unwrap().as_ref() {
        tx.send(CameraCmd::SetFocusMode(mode)).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn setzoomposition(state: State<'_, AppState>, x: u32, y: u32) -> Result<(), String> {
    if let Some(tx) = state.tx.lock().unwrap().as_ref() {
        tx.send(CameraCmd::SetZoomPosition(x, y)).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn startcamera(app: AppHandle, state: State<'_, AppState>, device_path: String) -> Result<String, String> {
    let rx = {
        let mut tx_lock = state.tx.lock().unwrap();
        if let Some(tx) = tx_lock.as_ref() {
            let _ = tx.send(CameraCmd::StopStream);
            std::thread::sleep(std::time::Duration::from_millis(300));
        }
        
        let (tx, rx) = channel();
        *tx_lock = Some(tx);
        rx
    };

    thread::spawn(move || {
        let mut cam_manager: Option<camera::CameraManager> = None;

        let mut decoder = decoder::Decoder::new().expect("Failed to init decoder");
        
        #[cfg(target_os = "linux")]
        let mut v4l2_out: Option<v4l2_out::V4l2Out> = None;

        println!("Robust persistent camera pipeline started.");

        let mut consecutive_errors = 0;

        loop {
            // ── 1. Drain ALL pending commands into a batch ──
            //    We collect them first so we can deduplicate and then
            //    execute them with the USB bus fully idle.
            let mut cmd_batch: Vec<CameraCmd> = Vec::new();
            while let Ok(cmd) = rx.try_recv() {
                cmd_batch.push(cmd);
            }

            if !cmd_batch.is_empty() {
                // Check for stop first
                if cmd_batch.iter().any(|c| matches!(c, CameraCmd::StopStream)) {
                    println!("Stopping...");
                    return;
                }

                if let Some(ref cm) = cam_manager {
                    // ── CRITICAL: Wait for any in-flight USB I/O to complete ──
                    thread::sleep(std::time::Duration::from_millis(300));

                    let deduped = dedup_commands(cmd_batch);

                    for cmd in &deduped {
                        let label = cmd.label();

                        let mut result = Err("not attempted".to_string());
                        for attempt in 0..3 {
                            if attempt > 0 {
                                let backoff = if attempt == 1 { 200 } else { 500 };
                                thread::sleep(std::time::Duration::from_millis(backoff));
                            }
                            result = execute_camera_cmd(cmd, cm, &app);
                            if result.is_ok() {
                                break;
                            }
                            eprintln!("[CMD] {} attempt {} failed, retrying...", label, attempt + 1);
                        }

                        match result {
                            Ok(()) => {
                                println!("[CMD] {} applied successfully", label);
                                let _ = app.emit("camera-status", format!("{} set OK", label));
                            }
                            Err(e) => {
                                let msg = format!("{} failed: {}", label, e);
                                eprintln!("[CMD] {}", msg);
                                let _ = app.emit("camera-error", msg);
                            }
                        }

                        thread::sleep(std::time::Duration::from_millis(100));
                    }

                    thread::sleep(std::time::Duration::from_millis(150));
                } else {
                    let _ = app.emit("camera-error", "Command ignored: No camera connected.");
                }

                continue;
            }

            // ── 2. Attempt connection if needed ──
            if cam_manager.is_none() {
                match camera::CameraManager::new() {
                    Ok(cm) => {
                        println!("Camera connected successfully!");
                        let _ = app.emit("camera-status", "Connected");
                        cam_manager = Some(cm);
                        consecutive_errors = 0;
                    }
                    Err(_e) => {
                        let _ = app.emit("camera-status", "Searching...".to_string());
                        thread::sleep(std::time::Duration::from_millis(2000));
                        continue;
                    }
                }
            }

            // ── 3. Capture frame (only when no commands are pending) ──
            if let Some(ref cm) = cam_manager {
                match cm.capture_preview() {
                    Ok(mjpeg_data) => {
                        consecutive_errors = 0; // Reset on success
                        let b64 = general_purpose::STANDARD.encode(&mjpeg_data);
                        let _ = app.emit("preview-frame", b64);

                        #[cfg(target_os = "linux")]
                        {
                            if let Ok((pixels, w, h)) = decoder.decode(&mjpeg_data) {
                                if v4l2_out.is_none() && device_path.starts_with("/dev/video") {
                                    println!("[PIPELINE] Initializing Virtual Webcam at {}...", device_path);
                                    match v4l2_out::V4l2Out::new(&device_path, w as u32, h as u32) {
                                        Ok(out) => {
                                            println!("[PIPELINE] Virtual Webcam ready!");
                                            v4l2_out = Some(out);
                                        }
                                        Err(e) => {
                                            println!("[PIPELINE] Virtual Webcam failed: {}. Continuing with preview only.", e);
                                        }
                                    }
                                }
                                if let Some(ref mut out) = v4l2_out {
                                    let _ = out.write_frame(&pixels);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        consecutive_errors += 1;
                        eprintln!("Pipeline I/O Error ({}): {}. Waiting before reconnect...", consecutive_errors, e);
                        if consecutive_errors > 5 {
                            eprintln!("Too many consecutive errors. Reconnecting to camera...");
                            cam_manager = None; 
                            #[cfg(target_os = "linux")]
                            { v4l2_out = None; }
                        }
                        thread::sleep(std::time::Duration::from_millis(800));
                    }
                }
            }
            
            // Balanced delay between frames
            thread::sleep(std::time::Duration::from_millis(30));
        }
    });

    Ok("Pipeline started".to_string())
}

/// Deduplicate a batch of commands — keep only the last occurrence of each
/// "kind" so we don't spam the camera with e.g. 5 ManualFocus calls.
fn dedup_commands(batch: Vec<CameraCmd>) -> Vec<CameraCmd> {
    let mut result: Vec<CameraCmd> = Vec::new();

    // Walk backwards so we encounter the "latest" command first
    let mut seen_iso = false;
    let mut seen_aperture = false;
    let mut seen_shutter = false;
    let mut seen_focus = false;
    let mut seen_af = false;
    let mut seen_focus_mode = false;
    let mut seen_wb = false;
    let mut seen_style = false;
    let mut seen_ev = false;
    let mut seen_metering = false;
    let mut seen_flash = false;
    let mut seen_config = false;

    for cmd in batch.into_iter().rev() {
        let dominated = match &cmd {
            CameraCmd::SetIso(_) => { let s = seen_iso; seen_iso = true; s }
            CameraCmd::SetAperture(_) => { let s = seen_aperture; seen_aperture = true; s }
            CameraCmd::SetShutterSpeed(_) => { let s = seen_shutter; seen_shutter = true; s }
            CameraCmd::ManualFocus(_) => { let s = seen_focus; seen_focus = true; s }
            CameraCmd::TriggerAutofocus => { let s = seen_af; seen_af = true; s }
            CameraCmd::SetFocusMode(_) => { let s = seen_focus_mode; seen_focus_mode = true; s }
            CameraCmd::SetWhiteBalance(_) => { let s = seen_wb; seen_wb = true; s }
            CameraCmd::SetPictureStyle(_) => { let s = seen_style; seen_style = true; s }
            CameraCmd::SetExposureComp(_) => { let s = seen_ev; seen_ev = true; s }
            CameraCmd::SetMeteringMode(_) => { let s = seen_metering; seen_metering = true; s }
            CameraCmd::SetFlashMode(_) => { let s = seen_flash; seen_flash = true; s }
            CameraCmd::SetZoomPosition(_, _) => false, // Always update AF point
            CameraCmd::GetConfig => { let s = seen_config; seen_config = true; s }
            CameraCmd::StopStream => false, // always keep
        };
        if !dominated {
            result.push(cmd);
        }
    }

    result.reverse(); // restore original order
    result
}

/// Execute a single camera command, returning Ok/Err.
fn execute_camera_cmd(cmd: &CameraCmd, cm: &camera::CameraManager, app: &AppHandle) -> Result<(), String> {
    match cmd {
        CameraCmd::SetIso(iso) => cm.set_iso(iso).map_err(|e| e.to_string()),
        CameraCmd::SetAperture(ap) => cm.set_aperture(ap).map_err(|e| e.to_string()),
        CameraCmd::SetShutterSpeed(speed) => cm.set_shutter_speed(speed).map_err(|e| e.to_string()),
        CameraCmd::ManualFocus(dir) => cm.manual_focus(*dir).map_err(|e| e.to_string()),
        CameraCmd::TriggerAutofocus => cm.trigger_autofocus().map_err(|e| e.to_string()),
        CameraCmd::SetFocusMode(mode) => cm.set_focus_mode(mode).map_err(|e| e.to_string()),
        CameraCmd::SetWhiteBalance(wb) => cm.set_white_balance(wb).map_err(|e| e.to_string()),
        CameraCmd::SetPictureStyle(style) => cm.set_picture_style(style).map_err(|e| e.to_string()),
        CameraCmd::SetExposureComp(ev) => cm.set_exposure_compensation(ev).map_err(|e| e.to_string()),
        CameraCmd::SetMeteringMode(mode) => cm.set_metering_mode(mode).map_err(|e| e.to_string()),
        CameraCmd::SetFlashMode(mode) => cm.set_flash_mode(mode).map_err(|e| e.to_string()),
        CameraCmd::SetZoomPosition(x, y) => cm.set_zoom_position(*x, *y).map_err(|e| e.to_string()),
        CameraCmd::GetConfig => {
            match cm.get_config() {
                Ok(config) => {
                    let _ = app.emit("camera-config", config);
                    Ok(())
                }
                Err(e) => Err(e.to_string()),
            }
        }
        CameraCmd::StopStream => Ok(()), // handled before this fn is called
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState { tx: Mutex::new(None) })
        .invoke_handler(tauri::generate_handler![
            createvirtualdevice, 
            startcamera, 
            stopcamera,
            debugcamera,
            setiso, 
            setaperture,
            setshutterspeed,
            manualfocus,
            triggerautofocus,
            setfocusmode,
            setwhitebalance,
            setpicturestyle,
            setexposurecompensation,
            setmeteringmode,
            setflashmode,
            setzoomposition
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
