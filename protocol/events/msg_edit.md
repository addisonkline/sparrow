# msg_edit

## Schema
```json
{
    "type": "msg_edit",
    "msg_id": "ULID as string",
    "timestamp": "string",
    "delete": "bool",
    "new_length": "number",
    "new_content": "string"
}
```

## Server

On receipt:
- If delete, removes message from chat log
- If not delete, replaces message content with new content
Finally, echoes event as a broadcast to all clients

## Client

On receipt:
- If delete, replaces message with "Message deleted"
- If not delete, replaces message content with new content and displays "Message edited at {ts}" alongside it

After sent, expects event to be echoed back