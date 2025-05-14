# user_conn

## Schema
```json
{
    "type": "user_conn",
    "tag": "UUID as string",
    "alias": "string"
}
```

## Server

Should never recieve

Broadcasted to all clients when a user disconnects

## Client

Should never send

On receipt, display "User {alias} disconnected"