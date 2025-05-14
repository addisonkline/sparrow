# disconnect

## Schema

### disconnect_req

```json
{
    "protocol": "sparrow",
    "version": "0.0.1",
    "method": "disconnect",
    "body": {
        "session_key": "string"
    }
}
```

### disconnect_res

```json
{
    "protocol": "sparrow",
    "version": "0.0.1",
    "method": "disconnect",
    "return_code": "number",
    "message": "string",
    "body": { }
}
```

## Server

Upon receiving a ```disconnect_req``` from a client, the server first authenticates the client. If this is successful, the server then deactivates the client's ```session_key``` and removes them from the list of connected clients. Finally, a ```disconnect_res``` is returned to the client, containing a ```return_code``` of 0 and the ```message``` 'success', in cases of success. Otherwise, ```return_code``` is nonzero and ```message``` contains a detailed error message. 

A successful disconnection creates a ```user_disconn``` event.

## Client