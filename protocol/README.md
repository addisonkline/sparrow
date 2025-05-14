# The Sparrow Protocol

**Version 0.0.1 (May 6, 2025)**

**Developed by Addison Kline (addison@charon-labs.com)**

>[!NOTE]
>This document contains information specifically pertaining to the Sparrow Protocol. For information about Sparrow more generally, visit the ```README``` in the root directory.

## Summary
The Sparrow Protocol is the means by which a Sparrow server and client communicate over a network. The categories of communications are defined below. 
- **Requests (req)**: Communications sent from a client to a server.
- **Responses (res)**: Communications sent from the server to a requesting client.
- **Events**: Communications sent between the server and connected clients.
  
Furthermore, The Sparrow Protocol defines a number of different methods, defined below.

## Technical Overview
The Sparrow Protocol is an application-layer internet protocol operating on top of WebSockets, utilizing TCP port 8848. All communications are  formatted using JSON, which is then serialized and encoded into bytes via UTF-8 for transmission.

## Methods
Below are all of the methods currently supported by the Sparrow Protocol. For more details regarding each method, consult the corresponding Markdown file(s) in ```protocol/req_res```.

### connect
Connect to a Sparrow server. The client gives a tag (user ID) to the server and receives a session key in return. A session key is required for any further communication between the client and server to be successful.

### disconnect
Disconnect from a Sparrow server. The client gives an active session key to the server, which deactivates it. 

## Events
Below are all of the events currently supported by the Sparrow Protocol. For more details regarding each event, consult the corresponding Markdown file(s) in ```protocol/events```.

### user_conn
The event broadcast to all clients by the server when a new user successfully connects.

### user_disconn
The event broadcast to all clients by the server when a connected user successully disconnects.

### msg_new
The event initiated by a client to send a new message. If successful, the server then broadcasts this event to all connected clients.

### msg_edit
The event initiated by a client to edit or delete an existing message. If successful, the server then broadcasts this event to all connected clients.

### alias_edit
The event initiated by a client to edit their in-server alias (nickname). If successful, the server then broadcasts this event to all connected clients.
