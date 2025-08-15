# Request and Response for `alias_edit`

## Schema

### Request
```json
{
    "head": {
        "protocol": "sparrow",
        "version": client version number as string,
        "origin": user session token as string,
        "timestamp": ISO 8601 timestamp as string,
        "type": "request/alias_edit"
    },
    "body": {
        "alias_new": new user alias as string
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
        "type": "response/alias_edit"
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