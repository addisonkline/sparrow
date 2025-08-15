# Request and Response for `connect`

## Schema

### Request
```json
{
    "head": {
        "protocol": "sparrow",
        "version": client version number as string,
        "origin": client IP address as string,
        "timestamp": ISO 8601 timestamp as string,
        "type": "request/connect"
    },
    "body": {
        "user_id": user ID for login as string,
        "public_key": user public key for login as string,
        "private_key": user private key for login as string
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
        "type": "response/connect"
    },
    "body": {
        "success": operation success value as boolean,
        "info": {
            "session_token": user's session token as string
        }
    }
}
```

## Server

![TODO]

## Client

![TODO]