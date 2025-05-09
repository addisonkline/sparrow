# The Sparrow Protocol

**Version 0.0.1 (May 6, 2025)**

**Developed by Addison Kline (addison@charon-labs.com)**

>[!NOTE]
>This document contains information specifically pertaining to the Sparrow Protocol. For information about Sparrow more generally, visit the ```README``` in the root directory.

## Summary
The Sparrow Protocol is the means by which a Sparrow server and client communicate over a network. Clients send requests to servers, and in turn, servers send corresponding responses back to clients. The Sparrow Protocol defines a number of different methods, which are specific types of queries given to a server by a client.

## Technical Overview
The Sparrow Protocol is an application-layer internet protocol operating on top of TCP, utilizing port 8848. Requests and responses are both formatted using JSON, which is then serialized and encoded into bytes via UTF-8 for TCP transmission.

## Methods
Below are all of the methods currently supported by the Sparrow Protocol. For the exact format of each method's request/response, consult the corresponding ```json``` file(s) in this directory.

### connect
Connect to a Sparrow server. The client gives a tag (user ID) to the server and receives a session key in return. A session key is required for any further communication between the client and server to be successful.

### disconnect
Disconnect from a Sparrow server. The client gives an active session key to the server, which deactivates it. 

### msg_send
Send a message to a Sparrow server. The user provides text for the server to append to its list of preexisting messages, if any. The length of the text provided must be specified by the client.

### msg_fetch
Fetch the _n_ most recent messages from the server, where _n_ is a number provided by the client. The length of the response must be specified by the server. 