#include <Wire.h>
#define I2C_ADDR 4

//int leds[] = {12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, A0, A1, A2, A3, 13};
//byte numLeds = 16;
int leds[] = {12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, A0, A1, A2, A3, 13};
byte numLeds = 16;

void setup() {
  Wire.begin(I2C_ADDR);
  Wire.onReceive(receiveEvent);
  Serial.begin(9600);
  for (byte i = 0; i < numLeds; i++) pinMode(leds[i], OUTPUT);
}

void loop() {
  delay(50);
}

void receiveEvent(int howMany) {
  while (Wire.available() > 0) {
    byte data = Wire.read();
    byte addr  =  data & 0b01111111;
    bool state = (data & 0b10000000) >> 7;
    if (addr < numLeds) digitalWrite(leds[addr], state);
  }
}
