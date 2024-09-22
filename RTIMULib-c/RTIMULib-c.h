#include <stdbool.h>

typedef struct humidity_t {
  bool humidityValid;
  float humidity;
  bool temperatureValid;
  float temperature;
} humidity_t;

typedef struct pressure_t {
  bool pressureValid;
  float pressure;
  bool temperatureValid;
  float temperature;
} pressure_t;

#ifdef __cplusplus
extern "C" {
#endif

  struct RTIMUSettings;
  typedef struct RTIMUSettings RTIMUSettings;

  void * rtimu_settings_init(const char *);

  struct RTHumidity;
  typedef struct RTHumidity RTHumidity;

  void * create_humidity(RTIMUSettings *);
  const char * humidity_name(RTHumidity *);
  bool humidity_init(RTHumidity *);
  humidity_t humidity_read(RTHumidity *);

  struct RTPressure;
  typedef struct RTPressure RTPressure;

  void * create_pressure(RTIMUSettings *);
  const char * pressure_name(RTPressure *);
  bool pressure_init(RTPressure *);
  pressure_t pressure_read(RTPressure *);

#ifdef __cplusplus
}
#endif
