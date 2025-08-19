# Request and Response for `msg_fetch`

## Schema

### Request
```json
{
    "head": {
        "protocol": "sparrow",
        "version": client version number as string,
        "origin": user session token as string,
        "timestamp": ISO 8601 timestamp as string,
        "type": "request/msg_fetch"
    },
    "body": {
        "timestamp": timestamp to fetch content after as a string
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
        "info": {
            "messages": [
                {
                    "timestamp": ISO 8601 timestamp as string,
                    "msg_id": message ID as string,
                    "user_id": sender ID as string,
                    "content": message content as string
                }
            ]
        }
    }
}
```

## Server

![TODO]

## Client

![TODO]