[Back](../README.md)

# File Intake

Receiving + storing pictures/files

We want to store incoming pictures in ObjSto, then send an event to our MQ that a new file has been added. This folder is the config for our ObjSto and MQ - incl. setup for the event-sending on file save.


## Local development

```
docker compose up
```

and

```
docker exec -it minio mc event add local/wru-bucket arn:minio:sqs::PRIMARY:amqp --event put
```

Then you can see the [MinIO UI](http://localhost:9001) and [RabbitMQ UI](http://localhost:15672).
