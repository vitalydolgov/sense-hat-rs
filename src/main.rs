mod matrix;
use matrix::Matrix;

mod sensors;
use sensors::Settings;
use sensors::humidity;
use sensors::humidity::HumiditySensor;
use sensors::pressure;
use sensors::pressure::PressureSensor;

fn main() {
    let mut matrix = prepare_led_matrix()
	.expect("error: cannot initialize matrix");
    draw_gradient(&mut matrix);

    unsafe {
	let settings = get_settings();
	let humidity_sensor = prepare_humidity_sensor(&settings)
	    .expect("error: cannot initialize humidity sensor");
	print_humidity_data(&humidity_sensor);
	let pressure_sensor = prepare_pressure_sensor(&settings)
	    .expect("error: cannot initialize pressure sensor");
	print_pressure_data(&pressure_sensor);
    }
}

// LED matrix

fn prepare_led_matrix() -> std::io::Result<Matrix> {
    let mat_device = matrix::device_path()
	.expect("error: LED matrix not found");
    let mut mat = Matrix::from(&mat_device)?;
    mat.clear()?;
    Ok(mat)
}

fn draw_gradient(matrix: &mut Matrix) {
    let step = 0xff / 8;
    for i in 0..8 {
	for j in 0..8 {
	    let r = 0xff - step * i;
	    let g = 0x7f;
	    let b = 0x00 + step * j;
	    matrix.set_pixel([r, g, b], i, j).unwrap();
	}
    }
}

// Device settings (only sensors)

unsafe fn get_settings() -> Settings {
    let settings_path = sensors::settings_path()
	.expect("error: cannot get settings path");
    sensors::settings_init(&settings_path)
}

// Humidity

unsafe fn prepare_humidity_sensor(settings: &Settings) -> Result<HumiditySensor, sensors::Error> {
    let mut sensor = humidity::create_sensor(settings);
    humidity::sensor_init(&mut sensor)?;
    let sensor_name = humidity::sensor_name(&sensor)
	.expect("error: cannot read device name");
    println!("Using humidity sensor {}", sensor_name);
    Ok(sensor)
}

unsafe fn print_humidity_data(sensor: &HumiditySensor) {
    if let Ok(data) = humidity::sensor_read(sensor) {
	println!("Humidity: {:.2} %RH", data.humidity);
	println!("Temperature (humidity): {:.2}°C", data.temperature);
    }
}

// Pressure

unsafe fn prepare_pressure_sensor(settings: &Settings) -> Result<PressureSensor, sensors::Error> {
    let mut sensor = pressure::create_sensor(settings);
    pressure::sensor_init(&mut sensor)?;
    let sensor_name = pressure::sensor_name(&sensor)
	.expect("error: cannot read device name");
    println!("Using pressure sensor {}", sensor_name);
    Ok(sensor)
}

unsafe fn print_pressure_data(sensor: &PressureSensor) {
    if let Ok(data) = pressure::sensor_read(sensor) {
	println!("Pressure: {:.2} hPa", data.pressure);
	println!("Temperature (pressure): {:.2}°C", data.temperature);
    }
}
