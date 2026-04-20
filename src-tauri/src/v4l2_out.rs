#![cfg(target_os = "linux")]
use anyhow::Result;
use std::fs::File;
use std::io::Write;
use v4l::{Device, format::FourCC, video::Output};

pub struct V4l2Out {
    file: File,
}

impl V4l2Out {
    pub fn new(device_path: &str, width: u32, height: u32) -> Result<Self> {
        let device = Device::with_path(device_path)?;
        let mut format = device.format()?;
        format.width = width;
        format.height = height;
        format.fourcc = FourCC::new(b"RGB3");
        device.set_format(&format)?;

        let file = std::fs::OpenOptions::new()
            .read(false)
            .write(true)
            .open(device_path)?;

        Ok(Self { file })
    }

    pub fn write_frame(&mut self, data: &[u8]) -> Result<()> {
        self.file.write_all(data)?;
        Ok(())
    }
}
