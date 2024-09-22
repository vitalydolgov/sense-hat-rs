#include "RTIMULib-c.h"
#include "RTIMULib.h"

// Settings

void * rtimu_settings_init(const char *productType) {
  return new RTIMUSettings(productType);
}

// Humidity

void * create_humidity(RTIMUSettings *settings) {
  return RTHumidity::createHumidity(settings);
}

const char * humidity_name(RTHumidity *humidity) {
  return humidity->humidityName();
}

bool humidity_init(RTHumidity *humidity) {
  return humidity->humidityInit();
}

humidity_t humidity_read(RTHumidity *humidity) {
  RTIMU_DATA data = {};
  humidity->humidityRead(data);
  humidity_t result = {
    .humidityValid = data.humidityValid,
    .humidity = data.humidity,
    .temperatureValid = data.temperatureValid,
    .temperature = data.temperature
  };
  return result;
}

// Pressure

void * create_pressure(RTIMUSettings *settings) {
  return RTPressure::createPressure(settings);
}

const char * pressure_name(RTPressure *pressure) {
  return pressure->pressureName();
}

bool pressure_init(RTPressure *pressure) {
  return pressure->pressureInit();
}

pressure_t pressure_read(RTPressure *pressure) {
  RTIMU_DATA data = {};
  pressure->pressureRead(data);
  pressure_t result = {
    .pressureValid = data.pressureValid,
    .pressure = data.pressure,
    .temperatureValid = data.temperatureValid,
    .temperature = data.temperature
  };
  return result;
}
