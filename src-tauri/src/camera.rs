use anyhow::Result;
use gphoto2::{widget::RadioWidget, Camera, Context};

pub struct CameraManager {
    context: Context,
    camera: Camera,
}

impl CameraManager {
    pub fn new() -> Result<Self> {
        println!("[CAMERA] Initializing gphoto2 context...");
        let context = Context::new()?;

        println!("[CAMERA] Searching for USB devices...");
        let cameras = context.list_cameras().wait()?;
        println!("[CAMERA] Found {} potential device(s).", cameras.len());
        
        if cameras.len() == 0 {
            return Err(anyhow::anyhow!("No cameras detected on USB. Please check cable and power."));
        }

        let camera_info = cameras
            .into_iter()
            .next()
            .ok_or_else(|| anyhow::anyhow!("No camera info found"))?;
            
        println!("[CAMERA] Attempting to connect to: {} at {}", camera_info.model, camera_info.port);
        let camera = context.get_camera(&camera_info).wait()?;
        println!("[CAMERA] Hardware connection established!");

        // Enable Viewfinder (Required for Canon Live View)
        println!("[CAMERA] Activating Live View (Viewfinder)...");
        if let Ok(widget) = camera.config_key::<RadioWidget>("viewfinder").wait() {
            let _ = widget.set_choice("1"); // 1 is 'on'
            if let Err(e) = camera.set_config(&widget).wait() {
                println!("[CAMERA] Warning: Could not set viewfinder: {}", e);
            } else {
                println!("[CAMERA] Viewfinder active!");
            }
        }

        // Enable Remote Control mode
        println!("[CAMERA] Initializing Remote Control mode...");
        if let Ok(widget) = camera.config_key::<RadioWidget>("eosremoterelease").wait() {
            let _ = widget.set_choice("None");
            let _ = camera.set_config(&widget).wait();
            println!("[CAMERA] Remote Control initialized.");
        }

        Ok(Self { context, camera })
    }

    pub fn capture_preview(&self) -> Result<Vec<u8>> {
        let file = self.camera.capture_preview().wait()?;
        let data = file.get_data(&self.context).wait()?;
        Ok(data.to_vec())
    }

    pub fn set_iso(&self, iso: &str) -> Result<()> {
        let widget = self.camera.config_key::<RadioWidget>("iso").wait()?;
        widget.set_choice(iso)?;
        self.camera.set_config(&widget).wait()?;
        Ok(())
    }

    pub fn set_aperture(&self, aperture: &str) -> Result<()> {
        let widget = self.camera.config_key::<RadioWidget>("aperture").wait()?;
        widget.set_choice(aperture)?;
        self.camera.set_config(&widget).wait()?;
        Ok(())
    }

    pub fn set_shutter_speed(&self, speed: &str) -> Result<()> {
        let widget = self
            .camera
            .config_key::<RadioWidget>("shutterspeed")
            .wait()?;
        widget.set_choice(speed)?;
        self.camera.set_config(&widget).wait()?;
        Ok(())
    }

    pub fn manual_focus(&self, direction: i32) -> Result<()> {
        let widget = self
            .camera
            .config_key::<RadioWidget>("manualfocusdrive")
            .wait()?;
            
        let choice = if direction > 0 {
            match direction.abs() {
                1 => "Near 1",
                2 => "Near 2",
                _ => "Near 3",
            }
        } else {
            match direction.abs() {
                1 => "Far 1",
                2 => "Far 2",
                _ => "Far 3",
            }
        };

        if let Err(e) = widget.set_choice(choice) {
            println!("[CAMERA] Warning: manual focus choice '{}' failed: {}.", choice, e);
            // Try fallback strings just in case this model uses numeric strings
            let fallback = if direction > 0 { direction.abs().to_string() } else { format!("-{}", direction.abs()) };
            if let Err(_) = widget.set_choice(&fallback) {
                 return Err(anyhow::anyhow!("Invalid manual focus choice"));
            }
        }
        
        self.camera.set_config(&widget).wait()?;
        Ok(())
    }

    pub fn set_zoom_position(&self, x: u32, y: u32) -> Result<()> {
        if let Ok(widget) = self.camera.config_key::<gphoto2::widget::TextWidget>("eoszoomposition").wait() {
            let val = format!("{},{}", x, y);
            if let Err(e) = widget.set_value(&val) {
                println!("[CAMERA] Warning: failed to set zoom position: {}", e);
            } else {
                let _ = self.camera.set_config(&widget).wait();
            }
        }
        Ok(())
    }

    pub fn trigger_autofocus(&self) -> Result<()> {
        let mut triggered = false;

        // Try autofocusdrive first
        if let Ok(widget) = self
            .camera
            .config_key::<RadioWidget>("autofocusdrive")
            .wait()
        {
            if let Err(e) = widget.set_choice("1") {
                println!("[CAMERA] Warning: autofocusdrive '1' failed: {}", e);
                // Try "On" as fallback
                let _ = widget.set_choice("On");
            }
            let _ = self.camera.set_config(&widget).wait();
            triggered = true;
        }

        // Fallback to eosremoterelease (Simulate half-press shutter)
        if let Ok(widget) = self
            .camera
            .config_key::<RadioWidget>("eosremoterelease")
            .wait()
        {
            // The choices vary by Canon model. Common ones are "Press Half", "Press 1", "Immediate".
            // Let's try "Press Half" first, then "Press 1"
            if let Err(e) = widget.set_choice("Press Half") {
                println!("[CAMERA] Warning: eosremoterelease 'Press Half' failed: {}", e);
                if let Err(_) = widget.set_choice("Press 1") {
                    let _ = widget.set_choice("Immediate"); // Another fallback
                }
            }
            let _ = self.camera.set_config(&widget).wait();
            std::thread::sleep(std::time::Duration::from_millis(400));
            
            let _ = widget.set_choice("Release Half");
            let _ = widget.set_choice("Release 1");
            let _ = widget.set_choice("None");
            let _ = self.camera.set_config(&widget).wait();
            triggered = true;
        }
        
        if triggered {
            Ok(())
        } else {
            Err(anyhow::anyhow!("No autofocus controls found"))
        }
    }

    pub fn set_exposure_compensation(&self, ev: &str) -> Result<()> {
        let widget = self
            .camera
            .config_key::<RadioWidget>("exposurecompensation")
            .wait()?;
        widget.set_choice(ev)?;
        self.camera.set_config(&widget).wait()?;
        Ok(())
    }

    pub fn set_metering_mode(&self, mode: &str) -> Result<()> {
        let widget = self
            .camera
            .config_key::<RadioWidget>("meteringmode")
            .wait()?;
        widget.set_choice(mode)?;
        self.camera.set_config(&widget).wait()?;
        Ok(())
    }

    pub fn set_flash_mode(&self, mode: &str) -> Result<()> {
        let widget = self.camera.config_key::<RadioWidget>("flashmode").wait()?;
        widget.set_choice(mode)?;
        self.camera.set_config(&widget).wait()?;
        Ok(())
    }

    pub fn set_focus_mode(&self, mode: &str) -> Result<()> {
        // mode is usually "Manual" or "One Shot" (AF)
        if let Ok(widget) = self.camera.config_key::<RadioWidget>("focusmode").wait() {
            if let Err(e) = widget.set_choice(mode) {
                println!("[CAMERA] Warning: Focus mode '{}' might not be supported ({}). Lens switch might be physical.", mode, e);
                return Ok(());
            }
            if let Err(e) = self.camera.set_config(&widget).wait() {
                println!("[CAMERA] Warning: Failed to apply focus mode: {}", e);
            }
        } else {
            println!("[CAMERA] Warning: 'focusmode' config not found on this camera.");
        }
        Ok(())
    }

    pub fn set_white_balance(&self, wb: &str) -> Result<()> {
        let widget = self
            .camera
            .config_key::<RadioWidget>("whitebalance")
            .wait()?;
        widget.set_choice(wb)?;
        self.camera.set_config(&widget).wait()?;
        Ok(())
    }

    pub fn set_picture_style(&self, style: &str) -> Result<()> {
        let widget = self
            .camera
            .config_key::<RadioWidget>("picturestyle")
            .wait()?;
        widget.set_choice(style)?;
        self.camera.set_config(&widget).wait()?;
        Ok(())
    }

    pub fn get_config(&self) -> Result<std::collections::HashMap<String, String>> {
        let mut config_map = std::collections::HashMap::new();

        // Add model name
        let abilities = self.camera.abilities();
        config_map.insert("model".to_string(), abilities.model().to_string());

        let keys = vec![
            "manualfocusdrive",
            "autofocusdrive",
            "focusmode",
            "eosremoterelease",
            "viewfinder",
            "iso",
            "aperture",
            "shutterspeed",
            "whitebalance",
            "drivemode",
            "focusmetermode",
        ];

        for key in keys {
            if let Ok(_widget) = self
                .camera
                .config_key::<gphoto2::widget::Widget>(key)
                .wait()
            {
                config_map.insert(key.to_string(), "exists".to_string());
            }
        }

        Ok(config_map)
    }
}
