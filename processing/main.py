#!/usr/bin/env python
import json
import os
import requests
from io import BytesIO

import pika
import face_recognition

import signal


RABBIT_MQ_HOST = os.getenv("RABBIT_MQ_HOST")
RABBIT_MQ_USERNAME = os.getenv("RABBIT_MQ_USERNAME")
RABBIT_MQ_PASSWORD = os.getenv("RABBIT_MQ_PASSWORD")
RABBIT_MQ_QUEUE_NAME = os.getenv("RABBIT_MQ_QUEUE_NAME")
MINIO_BASE_URL = os.getenv("MINIO_BASE_URL")


def callback(ch, method, properties, body):
    body_dict = json.loads(body.decode("utf8"))

    objsto_base_url = MINIO_BASE_URL

    file_url = f"{objsto_base_url}/{body_dict['Key']}"
    print(f"New file: {file_url}")

    response = requests.get(file_url)
    image = face_recognition.load_image_file(BytesIO(response.content))
    face_locations = face_recognition.face_locations(image)
    print(f"Face locations: {face_locations}")


def main():
    print(
    """
        How this program is going:

        ✅\tread in from MQ
        ✅\tdownload image from objsto based on MQ message
        ⚙️\tWIP: extract meaning from it (object, human, face, etc. recognition)
        ⚠️\tTODO: save results to a DB
        ✅\trepeat
    """
    )

    rmq_creds = pika.credentials.PlainCredentials(
        RABBIT_MQ_USERNAME, RABBIT_MQ_PASSWORD
    )

    connection = pika.BlockingConnection(
        pika.ConnectionParameters(
            host=RABBIT_MQ_HOST, credentials=rmq_creds
        )
    )

    def handle_sigterm(*args):
        connection.close()
        raise KeyboardInterrupt()

    signal.signal(signal.SIGTERM, handle_sigterm)

    channel = connection.channel()
    channel.queue_declare(queue=RABBIT_MQ_QUEUE_NAME, durable=True)
    channel.basic_consume(
        queue=RABBIT_MQ_QUEUE_NAME, on_message_callback=callback, auto_ack=True
    )

    print("👂 Waiting for messages. To exit press CTRL+C")
    channel.start_consuming()


if __name__ == "__main__":
    main()
