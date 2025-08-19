# Event `user_register`

## Schema
```json
{
    "head": {
        "protocol": "sparrow",
        "version": server version number as string,
        "origin": server URI as string,
        "timestamp": ISO 8601 timestamp as string,
        "type": "event/user_register"
    },
    "body": {
        "user_id": registered user ID as string,
        "user_public_key": registered user public key as string
    }
}
```

## Server

![TODO]

## Client

![TODO]