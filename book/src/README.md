<h1 align="center">Introduction</h1>
<hr>

**KaratsubaLabs Project Onigiri** is intended to be a highly scriptable and
modular home automation system. It provides a central server to manage home automation devices (such as sensors, motors, lcd displays) as well as a scripting API to interact with these devices.

The goal of this project is to provide a way for people to build their own home automation systems in a code-centric way. **Onigiri** is the glue that joins your devices and your scripts. This means that **Onigiri** is very easy to extend, configure, and integrate into other existing systems. Take note that if you want a home automation system working out of the box, this is not for you.

## Features

**Onigiri** comes bundled with a [rust sdk](https://github.com/KaratsubaLabs/onigiri-server/tree/master/onigiri_sdk) to facilitate scripting behaviors for these devices. It is completely possible to create an sdk for other languages using the scripting API, but only rust is supported for now.

The main device that is targeted is the ESP8266, although any device that can understand REST will be compatible. **Onigiri** has some prebuilt controller software that can be found [here](https://github.com/KaratsubaLabs/onigiri-firmware). You are of course highly encouraged to write your own.

