# Event `msg_edit`

## Schema
```json
{
    "head": {
        "protocol": "sparrow",
        "version": server version number as string,
        "origin": server URI as string,
        "timestamp": ISO 8601 timestamp as string,
        "type": "event/msg_edit"
    },
    "body": {
        "msg_id": ID of edited message as string,
        "user_id": user ID of message editor as string,
        "new_content": content of edited message as string
    }
}
```

## Server

![TODO]

## Client

![TODO]