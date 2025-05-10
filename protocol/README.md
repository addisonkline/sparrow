# The Sparrow Protocol

**Version 0.0.1 (May 6, 2025)**

**Developed by Addison Kline (addison@charon-labs.com)**

>[!NOTE]
>This document contains information specifically pertaining to the Sparrow Protocol. For information about Sparrow more generally, visit the ```README``` in the root directory.

## Summary
The Sparrow Protocol is the means by which a Sparrow server and client communicate over a network. The categories of communications are defined below. 
- **Requests (req)**: Communications sent from a client to a server.
- **Responses (res)**: Communications sent from the server to a requesting client.
- **Pushes (push)**: Communications sent from the server to all clients.
  
Furthermore, The Sparrow Protocol defines a number of different methods, defined below.

## Technical Overview
The Sparrow Protocol is an application-layer internet protocol operating on top of TCP, utilizing port 8848. All communications are  formatted using JSON, which is then serialized and encoded into bytes via UTF-8 for TCP transmission.

## Methods
Below are all of the methods currently supported by the Sparrow Protocol. For the exact format of each method's request/response, consult the corresponding ```json``` file(s) in this directory.

### connect
Connect to a Sparrow server. The client gives a tag (user ID) to the server and receives a session key in return. A session key is required for any further communication between the client and server to be successful.

### disconnect
Disconnect from a Sparrow server. The client gives an active session key to the server, which deactivates it. 

### msg_send
Send a message to a Sparrow server. The user provides text for the server to append to its list of preexisting messages, if any. 

### msg_fetch
Fetch all messages in the server since ```latest_message_id```. 

### msg_delete
Delete a given message from the server. Clients can only remove messages they themselves sent.
