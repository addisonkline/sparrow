# Request and Response for `msg_new`

## Schema

### Request
```json
{
    "head": {
        "protocol": "sparrow",
        "version": client version number as string,
        "origin": user session token as string,
        "timestamp": ISO 8601 timestamp as string,
        "type": "request/msg_new"
    },
    "body": {
        "content": message content as string
    }
}
```

### Response
```json
{
    "head": {
        "protocol": "sparrow",
        "version": server version number as string,
        "origin": server URI as string,
        "timestamp": ISO 8601 timestamp as string,
        "type": "response/msg_new"
    },
    "body": {
        "success": operation success value as boolean,
        "info": { }
    }
}
```

## Server

![TODO]

## Client

![TODO]