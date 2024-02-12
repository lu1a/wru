#!/usr/bin/env python
import json
import sys
import os
import requests
from io import BytesIO

import pika
import face_recognition
from dotenv import load_dotenv


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


def main():
    load_dotenv()

    rmq_creds = pika.credentials.PlainCredentials(
        os.getenv("RABBIT_MQ_USERNAME"), os.getenv("RABBIT_MQ_PASSWORD")
    )

    connection = pika.BlockingConnection(
        pika.ConnectionParameters(
            host=os.getenv("RABBIT_MQ_HOST"), credentials=rmq_creds
        )
    )
    channel = connection.channel()

    channel.queue_declare(queue="wru-queue", durable=True)

    channel.basic_consume(
        queue="wru-queue", on_message_callback=callback, auto_ack=True
    )

    print("👂 Waiting for messages. To exit press CTRL+C")
    channel.start_consuming()


def callback(ch, method, properties, body):
    body_dict = json.loads(body.decode("utf8"))

    # Try to get the objsto base URL based on the complex JSON that MinIO provides, or default to localhost
    objsto_base_url = (
        body_dict.get("Records", [{}])[0]
        .get("responseElements", {})
        .get("x-minio-origin-endpoint", None)
        or "http://localhost:9000"
    )

    file_url = f"{objsto_base_url}/{body_dict['Key']}"
    print(f"New file: {file_url}")

    response = requests.get(file_url)
    image = face_recognition.load_image_file(BytesIO(response.content))
    face_locations = face_recognition.face_locations(image)
    print(f"Face locations: {face_locations}")


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print("Interrupted")
        try:
            sys.exit(0)
        except SystemExit:
            os._exit(0)
