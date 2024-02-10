[Back](../README.md)

# Field Scout

Sending input pictures/files

To be run on an edge device on the field, which takes pictures (or generates files) every x seconds separately and saves them into a directory on-device. Our program watches the directory and sends to object storage when needed. Our program will also delete all sent images after a certain amount of time.

## Prerequisites

- At least one edge device with a camera or other sensor, such as a Raspberry Pi

## Development

Install rust on your machine, then

```
cargo run
```

## Usage

Compile the rust program according to the instruction set of the edge device, then load it on. Then get the camera or sensor of the device to create files in a directory of your choice, then run the field scout program pointing to that directory, and pointing to the right object storage instance somewhere over the internet. Place the device somewhere with internet, power, and a good view.

TODO: support for devices operating over Meshtastic instead of internet
