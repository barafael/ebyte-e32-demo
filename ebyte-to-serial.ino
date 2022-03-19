/*

  This example shows how to connect to an EBYTE transceiver
  using a Teensy 3.2

  This code for for the receiver


  connections
  Module      Teensy
  M0          2
  M1          3
  Rx          1 (This is the MCU Tx line)
  Tx          0 (MCU Rx line)
  Aux         4
  Vcc         3V3
  Gnd         Gnd

*/

#include "EBYTE.h"

// connect to any of the Teensy Serial ports
#define ESerial Serial1

#define PIN_M0 2
#define PIN_M1 3
#define PIN_AX 4

// i recommend putting this code in a .h file and including it
// from both the receiver and sender modules

// these are just dummy variables, replace with your own
struct DATA {
  unsigned long Count;
  int Bits;
  float Volts;
  float Amps;

};

int Chan;
DATA MyData;
char BUFFER[6];
unsigned long Last;

// create the transceiver object, passing in the serial and pins
EBYTE Transceiver(&ESerial, PIN_M0, PIN_M1, PIN_AX);

void setup() {
  Serial.begin(9600);

  // wait for the serial to connect
  while (!Serial) {}

  // start the transceiver serial port--i have yet to get a different
  // baud rate to work--data sheet says to keep on 9600

  pinMode(LED_BUILTIN, OUTPUT);

  ESerial.begin(9600);

  Serial.println("Starting Reader");

  // this init will set the pinModes for you
  Transceiver.init();

  // all these calls are optional but shown to give examples of what you can do

  Transceiver.SetAirDataRate(0);
  Serial.println(Transceiver.GetAirDataRate());

  // Transceiver.SetAddressH(4);
  // Transceiver.SetAddressL(0);
  Transceiver.SetChannel(23);
  // save the parameters to the unit,
  Transceiver.SaveParameters(TEMPORARY);

  // you can print all parameters and is good for debugging
  // if your units will not communicate, print the parameters
  // for both sender and receiver and make sure air rates, channel
  // and address is the same
  Transceiver.PrintParameters();

}

void loop() {
  // if the transceiver serial is available, proces incoming data
  // you can also use ESerial.available()
  if (Transceiver.available()) {
    byte b = Transceiver.GetByte();
    Serial.print((char)b);
  }
  if (Serial.available()) {
    byte b = Serial.read();
    Transceiver.SendByte(b);
  }
  digitalWrite(LED_BUILTIN, !digitalRead(LED_BUILTIN));
}
