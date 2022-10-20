#include <Wire.h>
#define I2C_ADDR 1

int leds[] = {12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, A0};
int ledCount[] = {8, 12, 8, 16, 16};
byte numLeds = 12;

char buff[4];
int buffPos = 0;
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

  Serial.println("Listening for input...");
}

void loop() {

//  for(int i = 0; i <= 4; i++) {
//    for(int j = 0; j <= ledCount[i]; j++) {
//      if (i == 1) { 
//        setLed(0b10000000 | byte(j));
//        delay(100);
//        setLed(0b00000000 | byte(j));
//        continue;
//      }
//      Wire.beginTransmission(i);
//      Wire.write(0b10000000 | byte(j));
//      Wire.endTransmission();
//      delay(100);
//      Wire.beginTransmission(i);
//      Wire.write(0b00000000 | byte(j));
//      Wire.endTransmission();
//    }
//  }
  
  while (Serial.available() > 0) {
    char c = Serial.read();
    if (c == '\n' && buffPos == 4) {
      Serial.print(buff[0]);
      Serial.print(' ');
      Serial.print(buff[1]);
      Serial.print(buff[2]);
      Serial.print(' ');
      Serial.println(buff[3]);
      byte i2cAddr = charToNum(buff[0]);
      byte ledAddr = 10 * charToNum(buff[1]) + charToNum(buff[2]);
      int ledState_int = charToNum(buff[3]);
      bool ledState = ledState_int;
      byte data = ((byte)ledState << 7) | ledAddr;
      if (i2cAddr != I2C_ADDR) {
        Wire.beginTransmission(i2cAddr);
        Wire.write(data);
        Wire.endTransmission();
      } else {
        setLed(data);
      }
      buff[0] = '\0';
      buff[1] = '\0';
      buff[2] = '\0';
      buff[3] = '\0';
      buffPos = 0;
    } else if (buffPos < 4 && c != '\n') {
      buff[buffPos] = c;
      buffPos += 1;
    }
  }
}

byte charToNum(char c) {
  switch (c) {
    case '1': return 1;
    case '2': return 2;
    case '3': return 3;
    case '4': return 4;
    case '5': return 5;
    case '6': return 6;
    case '7': return 7;
    case '8': return 8;
    case '9': return 9;
    default: return 0;
  }
}
