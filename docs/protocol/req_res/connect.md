# connect

## Schema

### connect_req
```json
{
    "protocol": "sparrow",
    "version": "0.0.1",
    "method": "connect",
    "body": {
        "tag": "string"
    }
}
```

### connect_res
```json
{
    "protocol": "sparrow",
    "version": "0.0.1",
    "method": "connect",
    "return_code": "number",
    "message": "string",
    "body": {
        "session_key": "string"
    }
}
```

## Server

Upon receiving a ```connect_req``` from a client, the server attempts to authenticate the user. If this is successful, the ```connect_res``` contains the user's ```session_key``` and ```message``` is 'success'. Otherwise, no session key is included, ```return_code``` is nonzero, and ```message``` contains a detailed error message. 

A successful connection creates a ```user_conn``` event.

## Client

A client sends a ```connect_req```, which contains the user's ```tag```, to a server. The client will then receive a ```connect_res``` from the server, indicating whether or not the connection was successful.