[Back](../README.md)

# Processing

Processing pictures/files based on MQ events.

## Prerequisites

- A RabbitMQ instance which hands out events when an image is ready for processing
- A MinIO instance which stores an image ready to be processed

## Usage

Make a .env file by copying the .env.example

```
cp .env.example .env
```

Then specify the relevant variables inside the .env

Then run

```
nix develop
python main.py
```

TODO: write more
