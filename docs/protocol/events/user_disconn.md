# Event `user_disconn`

## Schema
```json
{
    "head": {
        "protocol": "sparrow",
        "version": server version number as string,
        "origin": server URI as string,
        "timestamp": ISO 8601 timestamp as string,
        "type": "event/user_disconn"
    },
    "body": {
        "user_id": disconnected user ID as string,
        "reason": reason for disconnect as string
    }
}
```

## Server

![TODO]

## Client

![TODO]