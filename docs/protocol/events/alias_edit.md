# alias_edit

## Schema
```json
{
    "type": "alias_edit",
    "user_tag": "UUID as string",
    "new_alias": "string"
}
```

## Server

On receipt, replaces the alias for messages containing tag `user_tag` with `new_alias`
Then, echoes as broadcast to all clients

## Client

On receipt:
- If sent by self, notifies user that change was accepted
- If sent by other, replaces displayed alias on all messages containing tag `user_tag` with `new_alias`

After sent, expects event to be echoed back

