# Event `msg_del`

## Schema
```json
{
    "head": {
        "protocol": "sparrow",
        "version": server version number as string,
        "origin": server URI as string,
        "timestamp": ISO 8601 timestamp as string,
        "type": "event/msg_del"
    },
    "body": {
        "msg_id": ID of deleted message as string,
        "user_id": user ID of message deleter as string
    }
}
```

## Server

![TODO]

## Client

![TODO]