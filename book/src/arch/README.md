<h1 align="center">Project Overview</h1>
<hr>

This section will provide a description of each component of **Project Onigiri**. This will help you create your mental model of what this project is and is not, and where your own code fits into the entire system.

## Devices
Firstly, we have the *devices*. These are the actual IOT 'gadgets' that you will have around your home, things like smart switches, window blind controllers, and lcd displays. These *devices* will have a REST API to interact with them. For now, these *devices* are all composed of an ESP8266 (a wifi enabled microcontroller), theoretically any wifi enabled board will work. While **Project Onigiri** provides some existing web server + controller software (avaliable in the [onigiri-firmware repo](https://github.com/KaratsubaLabs/onigiri-firmware)), you should be writing custom controller software for your own need.

Please refer to the dedicated chapter [Device Documentation](#) for writing your own controller code.

## Server
The *server* provides an abstraction layer over the *devices*. It acts as a proxy between the user application and each *device*.

The process to register a *device* is as follows:
- *device* boots up
- pings *server* to register itself (make itself known to the *server*)
- *server* records *device* details (like friendly name, API type, IP address)
- user application claim devices using their friendly name

The process for a user sending an action to a *device* is as follows:
- user (or sdk) makes REST call to *server*
- *server* checks credentials
- *server* fetches *device* IP address
- user's REST call is proxied to the corresponding *device*
- *device* handles request

## User Application
The user application is entirely your code. You make use of the rust sdk or straight up using the scripting API endpoints, to script the behavior of the *devices*. This is where you would define behaviors like
- Open the blinds and turn on the lights at 9 am if it is a weekday
- If the door gets opened and it is past midnight, trigger an intruder alert
- Using a cli, turn on the coffee machine
- Connect with smart watch so that if you press a button on the smart watch, the lights turn on

The sdk not only wraps the scripting API calls into nice handler functions and types, it provides a 'runtime' of sorts to write common behaviors like periodic actions and event listeners. For more information, checkout the [SDK Documentation](#) section.
