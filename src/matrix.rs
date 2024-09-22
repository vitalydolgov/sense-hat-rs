use std::fs;
use std::fs::File;
use std::io;
use std::io::{Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use glob;

const SENSE_HAT_FB_NAME: &str = "RPi-Sense FB";

pub struct Matrix {
    f: File
}

impl Matrix {
    /// Creates matrix for provided `device_path`.
    pub fn from(device_path: &Path) -> io::Result<Matrix> {
	let f = File::options()
	    .write(true)
	    .open(device_path)?;
	Ok(Matrix { f })
    }

    /// Sets color in RGB format for pixel at `row` and `col`, zero-based indexing.
    pub fn set_pixel(&mut self, rgb: [u8; 3], row: u8, col: u8) -> io::Result<()> {
	let pixel = rgb565(rgb).to_ne_bytes();
	let offset = ((row * 8 + col) * 2) as u64;
	self.f.seek(SeekFrom::Start(offset))?;
	self.f.write(&pixel)?;
	Ok(())
    }

    /// Clears matrix (sets all pixels black).
    pub fn clear(&mut self) -> io::Result<()> {
	for i in 0..8 {
	    for j in 0..8 {
		self.set_pixel([0, 0, 0], i, j)?;
	    }
	}
	Ok(())
    }
}

fn rgb565(rgb: [u8; 3]) -> u16 {
    let r = ((rgb[0] >> 3) & 0x1f) as u16;
    let g = ((rgb[1] >> 2) & 0x3f) as u16;
    let b = ((rgb[2] >> 3) & 0x1f) as u16;
    (r << 11) + (g << 5) + b
}

/// Searches for LED matrix device.
pub fn device_path() -> Option<PathBuf> {
    let paths = glob::glob("/sys/class/graphics/fb*").unwrap(); // safe
    for entry in paths {
	match entry {
	    Ok(syspath) => {
		let file_name = syspath.join("name");
		let dev_name = match fs::read_to_string(&file_name) {
		    Ok(contents) => contents.trim().to_string(),
		    Err(_) => continue
		};
		if dev_name == SENSE_HAT_FB_NAME {
		    let dev_file_name = syspath.file_name().unwrap(); // safe
		    let dev_path = Path::new("/dev").join(dev_file_name);
		    return Some(dev_path);
		}
	    }
	    Err(_) => {
		eprintln!("error: cannot read /sys/class/graphics");
		return None
	    }
	}
    }
    None
}
