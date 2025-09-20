# SoftBoon

A programmable soft-start board for the Boondocker Dishy Dualie, to ease current surge at start-up.

This project is in no way associated or affiliated with Boondock Energy, Inc.

## Implementation

Small PCB designed for retrofit into an existing project.

<p float="center">
  <img src="/img/board_softboon-r1.png" width="40%" />
  <img src="/img/assembled_softboon-r1.png" width="40%" />
</p>

Schematic: [images/schematic_softboon-r1.pdf](images/schematic_softboon-r1.pdf)

Dual voltage regulators allow use of 7-35V on input (Boondocker is 10-30V).

An STM32F030K6 programmable microcontroller generates the timing logic as follows:

At initial power-on:
* DC-DC and all outputs off.

When the mechanical switch input is On (open)
* Wait 1 second
* Switch the DC-DC on.
* Wait 1 second
* Switch the Dishy PoE on (and Aux Output.)
* Wait 1 second
* Switch the Router PoE on.

When the mechanical switch is Off (closed)
* Immediately switch all outputs and DC-DC off.

### Board Connections

5-pin Header J3:
* "+" (Positive Supply): Recommended to be connected after Fuse and Diode arrangement on the input of the Boondocker to benefit from Reverse Polarity protection. Can tolerate 7-35V.
* "G" (Common/Ground): "Common" on the Dishy Dualie "Remote Shutdown" header.
* Enable 0: "Router Switch" on the Dishy Dualie "Remote Shutdown" header." Can tolerate 45V open, 500mA short.
* Enable 1: "DC-DC Switch" on the Dishy Dualie "Remote Shutdown" header." Can tolerate 45V open, 500mA short.
* Enable 2: "Dishy Switch" on the Dishy Dualie "Remote Shutdown" header." Can tolerate 45V open, 500mA short.

2-pin Header J2:
* Short pins to switch unit off (optional, leave disconnected for always-on). 3.3V @ 1.1mA when shorted.

### Firmware

Written in Rust using the Embassy Framework.

NB: Optimisation and LTO is required to fit the Flash of the microcontroller.

#### Inputs

* PA3 - Switch, 3.3V open, 1.5mA closed.

#### Outputs

* PA0 - Q1 Transistor - Router Switch
* PA1 - Q2 Transistor - DC-DC
* PA2 - Q3 Transistor - Dishy

## Authors

* Phil Crump <phil@philcrump.co.uk>
