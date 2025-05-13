# msg_new

## Schema
```json
{
    "type": "msg_new",
    "id": "ULID as string",
    "user_tag": "UUID as string",
    "user_alias": "string",
    "timestamp": "string",
    "length": "number",
    "content": "string"
}
```

## Server

On receipt, assigns id, adds message to chat log, and echoes the event as a broadcast to all clients

## Client

On receipt:
- If sent by self, displays "Sent" alongside the messagee
- If sent by other, display new message

After sent, expects event to be echoed back with assigned ID