# user_conn

## Schema
```json
{
    "type": "user_conn",
    "tag": "UUID as string",
    "alias": "string",
    "is_new_user": "bool"
}
```

## Server

Should never recieve

Broadcasted to all clients when a user connects, with "is_new_user" set to true if the user has not connected to the server before

## Client

Should never send

On receipt:
- If is new user, display "New user {alias} connected"
- Else, display "User {alias} connected"