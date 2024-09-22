use std::fs;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use libc::c_void;
use thiserror::Error;

const SETTINGS_HOME_PATH: &str = ".config/sense_hat";
const SETTINGS_FILE_NAME: &str = "RTIMULib";

#[link(name = "RTIMULib")]
extern {
    fn rtimu_settings_init(productType: *const u8) -> *mut c_void;
}

/// Path to settings file in user's home folder, creates all necessary
/// intermediate directories. Returns `None` when target directory in
/// unavailable.
pub fn settings_path() -> Option<PathBuf> {
    let mut path = dirs::home_dir()?;
    path.push(SETTINGS_HOME_PATH);
    if !path.exists() {
	fs::create_dir(&path).ok()?;
    }
    path.push(SETTINGS_FILE_NAME);
    Some(path)
}

/// Initializes settings object for futher operation.
pub unsafe fn settings_init(path: &Path) -> Settings {
    let path_ptr = path.as_os_str().as_bytes().as_ptr();
    let ptr = rtimu_settings_init(path_ptr);
    Settings { ptr }
}

/// Wrapper for settings object in RTIMU library.
pub struct Settings {
    ptr: *const c_void
}

pub mod humidity {
    use std::ffi::CStr;
    use libc::c_void;

    #[link(name = "RTIMULib")]
    extern {
	fn create_humidity(settings: *const c_void) -> *const c_void;
	fn humidity_name(humidity: *const c_void) -> *const u8;
	fn humidity_init(humidity: *const c_void) -> bool;
	fn humidity_read(humidity: *const c_void) -> Humidity;
    }

    /// Creates sensor object for futher operation.
    pub unsafe fn create_sensor(settings: &super::Settings) -> HumiditySensor {
	let ptr = create_humidity(settings.ptr);
	HumiditySensor { ptr, initialized: false }
    }

    /// Official sensor name.
    pub unsafe fn sensor_name(sensor: &HumiditySensor) -> Option<&str> {
	let ptr = humidity_name(sensor.ptr);
	CStr::from_ptr(ptr).to_str().ok()
    }

    /// Prepares sensor for reading.
    pub unsafe fn sensor_init(sensor: &mut HumiditySensor) -> Result<(), super::Error> {
	if humidity_init(sensor.ptr) {
	    sensor.initialized = true;
	    Ok(())
	} else {
	    let error = super::Error { message: "Cannot initalize humidity sensor".to_string() };
	    return Err(error);
	}
    }

    /// Returns data from sensor, mind that before calling it's required to
    /// initialize sensor with [`sensor_init`].
    pub unsafe fn sensor_read(sensor: &HumiditySensor) -> Result<Humidity, super::Error> {
	if sensor.initialized {
	    Ok(humidity_read(sensor.ptr))
	} else {
	    let error = super::Error { message: "Sensor is not initialized".to_string() };
	    Err(error)
	}
    }

    /// Wrapper for sensor object in RTIMU library.
    pub struct HumiditySensor {
	ptr: *const c_void,
	initialized: bool
    }

    #[repr(C)]
    #[derive(Debug)]
    pub struct Humidity {
	pub humidity_valid: bool,
	pub humidity: f32,
	pub temperature_valid: bool,
	pub temperature: f32
    }
}

pub mod pressure {
    use std::ffi::CStr;
    use libc::c_void;

    #[link(name = "RTIMULib")]
    extern {
	fn create_pressure(settings: *const c_void) -> *const c_void;
	fn pressure_name(pressure: *const c_void) -> *const u8;
	fn pressure_init(pressure: *const c_void) -> bool;
	fn pressure_read(pressure: *const c_void) -> Pressure;
    }

    /// Creates sensor object for futher operation.
    pub unsafe fn create_sensor(settings: &super::Settings) -> PressureSensor {
	let ptr = create_pressure(settings.ptr);
	PressureSensor { ptr, initialized: false }
    }

    /// Official sensor name.
    pub unsafe fn sensor_name(sensor: &PressureSensor) -> Option<&str> {
	let ptr = pressure_name(sensor.ptr);
	CStr::from_ptr(ptr).to_str().ok()
    }

    /// Prepares sensor for reading.
    pub unsafe fn sensor_init(sensor: &mut PressureSensor) -> Result<(), super::Error> {
	if pressure_init(sensor.ptr) {
	    sensor.initialized = true;
	    Ok(())
	} else {
	    let error = super::Error { message: "Cannot initalize pressure sensor".to_string() };
	    return Err(error);
	}
    }

    /// Returns data from sensor, mind that before calling it's required to
    /// initialize sensor with [`sensor_init`].
    pub unsafe fn sensor_read(sensor: &PressureSensor) -> Result<Pressure, super::Error> {
	if sensor.initialized {
	    Ok(pressure_read(sensor.ptr))
	} else {
	    let error = super::Error { message: "Sensor is not initialized".to_string() };
	    Err(error)
	}
    }

    /// Wrapper for sensor object in RTIMU library.
    pub struct PressureSensor {
	ptr: *const c_void,
	initialized: bool
    }

    #[repr(C)]
    #[derive(Debug)]
    pub struct Pressure {
	pub pressure_valid: bool,
	pub pressure: f32,
	pub temperature_valid: bool,
	pub temperature: f32
    }
}

#[derive(Error, Debug)]
#[error("{message:}")]
pub struct Error {
    pub message: String
}
