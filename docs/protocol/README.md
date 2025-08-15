# The Sparrow Protocol

**Created: May 6, 2025**

**Last Updated: August 15, 2025**

## Contents

- [The Sparrow Protocol](#the-sparrow-protocol)
  - [Contents](#contents)
  - [Summary](#summary)
  - [Technical Overview](#technical-overview)
  - [Methods](#methods)
  - [Events](#events)

## Summary
The Sparrow Protocol is the means by which a Sparrow server and client communicate over a network. The categories of communications are defined below. 
- **Requests**: Communications sent from a client to a server.
- **Responses**: Communications sent from the server to a requesting client.
- **Events**: Communications sent between the server and connected clients.
  
Furthermore, The Sparrow Protocol defines a number of different methods, defined below.

## Technical Overview
The Sparrow Protocol is an application-layer internet protocol operating on top of WebSockets, utilizing TCP port 8848. All communications are formatted using JSON.

## Methods
A method is a type of request/response communication. Below are all of the methods currently supported by the Sparrow Protocol:

- [alias_edit](req_res/alias_edit.md): A user attempts to change their server alias.
- [connect](req_res/connect.md): A user attempts to connect to a given server.
- [disconnect](req_res/disconnect.md): A user connected to a given server attempts to disconnect.
- [msg_del](events/msg_del.md): A user attempts to delete an existing message.
- [msg_edit](events/msg_edit.md): A user attempts to edit an existing message.
- [msg_new](events/msg_new.md): A user attempts to send a new message.

## Events
An event is a form of communication that is broadcast to the server when triggered. Below are all of the events currently supported by the Sparrow Protocol:

- [msg_del](events/msg_del.md): An existing message is deleted.
- [msg_edit](events/msg_edit.md): An existing message is edited.
- [msg_new](events/msg_new.md): A new message is sent.
- [user_alias_edit](events/user_alias_edit.md): A user's alias is edited.
- [user_conn](events/user_conn.md): A user connects.
- [user_disconn](events/user_disconn.md): A user disconnects.
