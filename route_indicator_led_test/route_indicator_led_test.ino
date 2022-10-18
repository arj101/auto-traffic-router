int leds[] = {12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, A0};
int numLeds = 12;
int blinkDelay = 250;

void setup() {
  // put your setup code here, to run once:
  for(int i = 0; i < numLeds; i++) pinMode(leds[i], OUTPUT);
}

void loop() {
  // put your main code here, to run repeatedly:
  for(int i = 0; i < numLeds; i++) {
    digitalWrite(leds[i], HIGH);
    delay(blinkDelay);
    digitalWrite(leds[i], LOW);
    
  }
}
