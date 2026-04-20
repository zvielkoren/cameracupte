use anyhow::Result;
use turbojpeg::{Decompressor, Image, PixelFormat};

pub struct Decoder {
    decompressor: Decompressor,
}

impl Decoder {
    pub fn new() -> Result<Self> {
        Ok(Self {
            decompressor: Decompressor::new()?,
        })
    }

    pub fn decode(&mut self, mjpeg_data: &[u8]) -> Result<(Vec<u8>, usize, usize)> {
        let header = self.decompressor.read_header(mjpeg_data)?;
        let width = header.width;
        let height = header.height;
        let mut pixels = vec![0; 3 * width * height];
        
        let image = Image {
            pixels: pixels.as_mut_slice(),
            width,
            pitch: 3 * width,
            height,
            format: PixelFormat::RGB,
        };

        self.decompressor.decompress(mjpeg_data, image)?;
        Ok((pixels, width, height))
    }
}
