# The Sparrow Protocol

**Created: May 6, 2025**

**Last Updated: May 15, 2025**

## Contents

- [The Sparrow Protocol](#the-sparrow-protocol)
  - [Contents](#contents)
  - [Summary](#summary)
  - [Technical Overview](#technical-overview)
  - [Methods](#methods)
  - [Events](#events)

## Summary
The Sparrow Protocol is the means by which a Sparrow server and client communicate over a network. The categories of communications are defined below. 
- **Requests (req)**: Communications sent from a client to a server.
- **Responses (res)**: Communications sent from the server to a requesting client.
- **Events**: Communications sent between the server and connected clients.
  
Furthermore, The Sparrow Protocol defines a number of different methods, defined below.

## Technical Overview
The Sparrow Protocol is an application-layer internet protocol operating on top of WebSockets, utilizing TCP port 8848. All communications are  formatted using JSON, which is then serialized and encoded into bytes via UTF-8 for transmission.

## Methods
A method is a type of request/response communication. Below are all of the methods currently supported by the Sparrow Protocol:

- [connect](req_res/connect.md): A client attempts to connect to a given server.
- [disconnect](req_res/disconnect.md): A client connected to a given server attempts to disconnect.

## Events
An event is a form of communication that is broadcast to the server when triggered. Below are all of the events currently supported by the Sparrow Protocol:

- [user_conn](events/user_conn.md): A new client connects.
- [user_disconn](events/user_disconn.md): A client disconnects.
- [msg_new](events/msg_new.md): A new message is sent.
- [msg_edit](events/msg_edit.md): An existing message is edited.
- [alias_edit](events/alias_edit.md): A client's alias is edited.
