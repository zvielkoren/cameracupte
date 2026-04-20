use anyhow::Result;
use gphoto2::{Camera, Context, widget::RadioWidget};

pub struct CameraManager {
    context: Context,
    camera: Camera,
}

impl CameraManager {
    pub fn new() -> Result<Self> {
        println!("Initializing gphoto2 context...");
        let context = Context::new()?;
        
        println!("Listing cameras...");
        let cameras = context.list_cameras().wait()?;
        if cameras.len() == 0 {
            return Err(anyhow::anyhow!("No cameras detected on USB"));
        }

        println!("Found {} camera(s). Connecting to the first one...", cameras.len());
        let camera_info = cameras.into_iter().next().ok_or_else(|| anyhow::anyhow!("No camera info found"))?;
        let camera = context.get_camera(&camera_info).wait()?;
        
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
        let widget = self.camera.config_key::<RadioWidget>("shutterspeed").wait()?;
        widget.set_choice(speed)?;
        self.camera.set_config(&widget).wait()?;
        Ok(())
    }

    pub fn manual_focus(&self, direction: i32) -> Result<()> {
        let widget = self.camera.config_key::<RadioWidget>("manualfocusdrive").wait()?;
        // Canon 600D uses string values like "1", "2", "3" or "Far 1", etc.
        // We'll try to map direction to these strings.
        let choice = if direction > 0 {
            match direction.abs() {
                1 => "1",
                2 => "2",
                _ => "3",
            }
        } else {
            match direction.abs() {
                1 => "-1",
                2 => "-2",
                _ => "-3",
            }
        };
        
        let _ = widget.set_choice(choice);
        self.camera.set_config(&widget).wait()?;
        Ok(())
    }

    pub fn trigger_autofocus(&self) -> Result<()> {
        // Try autofocusdrive first
        if let Ok(widget) = self.camera.config_key::<RadioWidget>("autofocusdrive").wait() {
            let _ = widget.set_choice("1");
            let _ = self.camera.set_config(&widget).wait();
        }

        // Fallback to eosremoterelease
        if let Ok(widget) = self.camera.config_key::<RadioWidget>("eosremoterelease").wait() {
            let _ = widget.set_choice("Press 1"); 
            let _ = self.camera.set_config(&widget).wait();
            std::thread::sleep(std::time::Duration::from_millis(300));
            let _ = widget.set_choice("None");
            let _ = self.camera.set_config(&widget).wait();
        }
        Ok(())
    }

    pub fn set_exposure_compensation(&self, ev: &str) -> Result<()> {
        let widget = self.camera.config_key::<RadioWidget>("exposurecompensation").wait()?;
        widget.set_choice(ev)?;
        self.camera.set_config(&widget).wait()?;
        Ok(())
    }

    pub fn set_metering_mode(&self, mode: &str) -> Result<()> {
        let widget = self.camera.config_key::<RadioWidget>("meteringmode").wait()?;
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
        let widget = self.camera.config_key::<RadioWidget>("focusmode").wait()?;
        widget.set_choice(mode)?;
        self.camera.set_config(&widget).wait()?;
        Ok(())
    }

    pub fn set_white_balance(&self, wb: &str) -> Result<()> {
        let widget = self.camera.config_key::<RadioWidget>("whitebalance").wait()?;
        widget.set_choice(wb)?;
        self.camera.set_config(&widget).wait()?;
        Ok(())
    }

    pub fn set_picture_style(&self, style: &str) -> Result<()> {
        let widget = self.camera.config_key::<RadioWidget>("picturestyle").wait()?;
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
            "manualfocusdrive", "autofocusdrive", "focusmode", 
            "eosremoterelease", "viewfinder", "iso", "aperture", 
            "shutterspeed", "whitebalance", "drivemode", "focusmetermode"
        ];

        for key in keys {
            if let Ok(widget) = self.camera.config_key::<gphoto2::widget::Widget>(key).wait() {
                config_map.insert(key.to_string(), "exists".to_string());
            }
        }
        
        Ok(config_map)
    }
}
