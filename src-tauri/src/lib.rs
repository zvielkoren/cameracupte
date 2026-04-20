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
    StopStream,
    GetConfig,
}

#[tauri::command]
fn set_white_balance(state: State<'_, AppState>, wb: String) -> Result<(), String> {
    if let Some(tx) = state.tx.lock().unwrap().as_ref() {
        tx.send(CameraCmd::SetWhiteBalance(wb)).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn set_picture_style(state: State<'_, AppState>, style: String) -> Result<(), String> {
    if let Some(tx) = state.tx.lock().unwrap().as_ref() {
        tx.send(CameraCmd::SetPictureStyle(style)).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn set_exposure_compensation(state: State<'_, AppState>, ev: String) -> Result<(), String> {
    if let Some(tx) = state.tx.lock().unwrap().as_ref() {
        tx.send(CameraCmd::SetExposureComp(ev)).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn set_metering_mode(state: State<'_, AppState>, mode: String) -> Result<(), String> {
    if let Some(tx) = state.tx.lock().unwrap().as_ref() {
        tx.send(CameraCmd::SetMeteringMode(mode)).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn set_flash_mode(state: State<'_, AppState>, mode: String) -> Result<(), String> {
    if let Some(tx) = state.tx.lock().unwrap().as_ref() {
        tx.send(CameraCmd::SetFlashMode(mode)).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn debug_camera(state: State<'_, AppState>) -> Result<(), String> {
    if let Some(tx) = state.tx.lock().unwrap().as_ref() {
        tx.send(CameraCmd::GetConfig).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn stop_camera(state: State<'_, AppState>) -> Result<(), String> {
    if let Some(tx) = state.tx.lock().unwrap().as_ref() {
        tx.send(CameraCmd::StopStream).map_err(|e| e.to_string())?;
    }
    *state.tx.lock().unwrap() = None;
    Ok(())
}

#[tauri::command]
fn create_virtual_device() -> Result<String, String> {
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
fn set_iso(state: State<'_, AppState>, iso: String) -> Result<(), String> {
    if let Some(tx) = state.tx.lock().unwrap().as_ref() {
        tx.send(CameraCmd::SetIso(iso)).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn set_aperture(state: State<'_, AppState>, aperture: String) -> Result<(), String> {
    if let Some(tx) = state.tx.lock().unwrap().as_ref() {
        tx.send(CameraCmd::SetAperture(aperture)).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn set_shutter_speed(state: State<'_, AppState>, speed: String) -> Result<(), String> {
    if let Some(tx) = state.tx.lock().unwrap().as_ref() {
        tx.send(CameraCmd::SetShutterSpeed(speed)).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn manual_focus(state: State<'_, AppState>, direction: i32) -> Result<(), String> {
    if let Some(tx) = state.tx.lock().unwrap().as_ref() {
        tx.send(CameraCmd::ManualFocus(direction)).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn trigger_autofocus(state: State<'_, AppState>) -> Result<(), String> {
    if let Some(tx) = state.tx.lock().unwrap().as_ref() {
        tx.send(CameraCmd::TriggerAutofocus).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn set_focus_mode(state: State<'_, AppState>, mode: String) -> Result<(), String> {
    if let Some(tx) = state.tx.lock().unwrap().as_ref() {
        tx.send(CameraCmd::SetFocusMode(mode)).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn start_camera(app: AppHandle, state: State<'_, AppState>, device_path: String) -> Result<String, String> {
    let (tx, rx) = channel();
    *state.tx.lock().unwrap() = Some(tx);

    thread::spawn(move || {
        let mut cam_manager: Option<camera::CameraManager> = None;
        let mut decoder = decoder::Decoder::new().expect("Failed to init decoder");
        
        #[cfg(target_os = "linux")]
        let mut v4l2_out: Option<v4l2_out::V4l2Out> = None;

        println!("Persistent camera pipeline started.");

        loop {
            // ... (command handling logic stays the same) ...
            while let Ok(cmd) = rx.try_recv() {
                match cmd {
                    CameraCmd::StopStream => { println!("Stopping..."); return; }
                    _ => {
                        if let Some(ref cm) = cam_manager {
                            match cmd {
                                CameraCmd::SetIso(iso) => { let _ = cm.set_iso(&iso); }
                                CameraCmd::SetAperture(ap) => { let _ = cm.set_aperture(&ap); }
                                CameraCmd::SetShutterSpeed(speed) => { let _ = cm.set_shutter_speed(&speed); }
                                CameraCmd::ManualFocus(dir) => { let _ = cm.manual_focus(dir); }
                                CameraCmd::TriggerAutofocus => { let _ = cm.trigger_autofocus(); }
                                CameraCmd::SetFocusMode(mode) => { let _ = cm.set_focus_mode(&mode); }
                                CameraCmd::SetWhiteBalance(wb) => { let _ = cm.set_white_balance(&wb); }
                                CameraCmd::SetPictureStyle(style) => { let _ = cm.set_picture_style(&style); }
                                CameraCmd::SetExposureComp(ev) => { let _ = cm.set_exposure_compensation(&ev); }
                                CameraCmd::SetMeteringMode(mode) => { let _ = cm.set_metering_mode(&mode); }
                                CameraCmd::SetFlashMode(mode) => { let _ = cm.set_flash_mode(&mode); }
                                CameraCmd::GetConfig => {
                                    if let Ok(config) = cm.get_config() {
                                        println!("--- CAMERA CONFIG ---");
                                        for (k, v) in config { println!("{}: {}", k, v); }
                                        println!("---------------------");
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }

            if cam_manager.is_none() {
                match camera::CameraManager::new() {
                    Ok(cm) => {
                        println!("Camera connected!");
                        let _ = app.emit("camera-status", "Connected");
                        cam_manager = Some(cm);
                    }
                    Err(e) => {
                        eprintln!("Camera detection failed: {}", e);
                        let _ = app.emit("camera-status", format!("Searching... ({})", e));
                        thread::sleep(std::time::Duration::from_secs(2));
                        continue;
                    }
                }
            }

            if let Some(ref cm) = cam_manager {
                match cm.capture_preview() {
                    Ok(mjpeg_data) => {
                        let b64 = general_purpose::STANDARD.encode(&mjpeg_data);
                        let _ = app.emit("preview-frame", b64);

                        #[cfg(target_os = "linux")]
                        {
                            if let Ok((pixels, w, h)) = decoder.decode(&mjpeg_data) {
                                // Lazy-init V4L2 when we know resolution
                                if v4l2_out.is_none() && device_path.starts_with("/dev/video") {
                                    v4l2_out = v4l2_out::V4l2Out::new(&device_path, w as u32, h as u32).ok();
                                }
                                
                                if let Some(ref mut out) = v4l2_out {
                                    let _ = out.write_frame(&pixels);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Capture error: {}", e);
                        cam_manager = None;
                        #[cfg(target_os = "linux")]
                        { v4l2_out = None; }
                        thread::sleep(std::time::Duration::from_secs(1));
                    }
                }
            }
        }
    });

    Ok("Pipeline started".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState { tx: Mutex::new(None) })
        .invoke_handler(tauri::generate_handler![
            create_virtual_device, 
            start_camera, 
            stop_camera,
            debug_camera,
            set_iso, 
            set_aperture,
            set_shutter_speed,
            manual_focus,
            trigger_autofocus,
            set_focus_mode,
            set_white_balance,
            set_picture_style,
            set_exposure_compensation,
            set_metering_mode,
            set_flash_mode
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
