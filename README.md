# Heat recovery ventilation system

This project is split into two parts:
* `sensors` for code that runs on an STM32F103 blue pill for temperature, humidity, and CO2 data collection and driving the fan speeds with PWM.
* `web` for the easily accessible web frontend that displays sensor data, drives higher level control into the MCU and plots things.
