#include <Wire.h>
#define I2C_ADDR 1

int leds[] = {12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, A0};
int ledCount[] = {8, 12, 8, 16, 16};
byte numLeds = 12;

void setLed(byte data) {
  byte addr  =  data & 0b01111111;
  if (addr >= numLeds) return;
  bool state = (data & 0b10000000) >> 7;
  digitalWrite(leds[addr], state);
}

void setup() {
  Wire.begin(I2C_ADDR);
  Serial.begin(115200);
  for (int i = 0; i < numLeds; i++) pinMode(leds[i], OUTPUT);
}

void loop() {

  for(int i = 0; i <= 4; i++) {
    for(int j = 0; j <= ledCount[i]; j++) {
      if (i == 1) { 
        setLed(0b10000000 | byte(j));
        delay(100);
        setLed(0b00000000 | byte(j));
        continue;
      }
      Wire.beginTransmission(i);
      Wire.write(0b10000000 | byte(j));
      Wire.endTransmission();
      delay(100);
      Wire.beginTransmission(i);
      Wire.write(0b00000000 | byte(j));
      Wire.endTransmission();
    }
  }
  
  if (Serial.available() > 0) {
    byte c = Serial.read();
    byte i2cAddr = (c & 0b01110000) >> 4;
    byte data = c & 0b10001111;
    if (i2cAddr != I2C_ADDR) {
      Wire.beginTransmission(i2cAddr);
      Wire.write(data);
      Wire.endTransmission();
    } else {
      setLed(data);
    }
  }
}
