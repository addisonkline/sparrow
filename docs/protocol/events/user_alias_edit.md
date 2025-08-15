# Event `user_alias_edit`

## Schema
```json
{
    "head": {
        "protocol": "sparrow",
        "version": server version number as string,
        "origin": server URI as string,
        "timestamp": ISO 8601 timestamp as string,
        "type": "event/user_alias_edit"
    },
    "body": {
        "user_id": ID of editing user as string,
        "new_alias": user's new alias as string
    }
}
```

## Server

![TODO]

## Client

![TODO]