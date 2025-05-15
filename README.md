# Sparrow

A decentralized, lightweight chat server designed to be practical yet powerful. Completely free and open-source.

## Contents

- [Sparrow](#sparrow)
  - [Contents](#contents)
  - [What is Sparrow?](#what-is-sparrow)
    - [The Sparrow Protocol](#the-sparrow-protocol)
    - [The Sparrow Server](#the-sparrow-server)
    - [Sparrow Clients](#sparrow-clients)
  - [Installing](#installing)
    - [Downloading the Binary](#downloading-the-binary)
    - [Building from Source](#building-from-source)
  - [Running Sparrow](#running-sparrow)
    - [Creating a Server](#creating-a-server)
    - [Running a Server](#running-a-server)
    - [Runtime Commands](#runtime-commands)
  - [Server Management](#server-management)
    - [Security and Safety](#security-and-safety)
    - [Running Multiple Servers](#running-multiple-servers)

## What is Sparrow?

Sparrow is an open-source, WebSocket-based chat server written in Rust and using the Sparrow Protocol. 

Sparrow is also **decentralized**--that means there is no central server in charge of storing user or server information. Your server and its data are controlled only by you.

### The Sparrow Protocol

> [!NOTE]
> Consult [docs/protocol/README.md](docs/protocol/README.md) for full details on the Sparrow Protocol and its implementation.

The Sparrow Protocol details the rules of/formatting for WebSocket-based communication between a Sparrow server and client.

Clients can send **requests** to a server, and in turn receive **responses**, which detail the result of the request. **Events** can be triggered, which are broadcast to all clients connected to the server.

### The Sparrow Server

> [!NOTE]
> Consult [docs/server/README.md](docs/server/README.md) for full details on the Sparrow server and its implementation.

The Sparrow server accepts connections from clients following the Sparrow Protocol. As stated above, it is implemented as a WebSocket-based chat server written in Rust. 

The complete source code for the Sparrow server can be found in the ```src``` directory.

### Sparrow Clients

> [!NOTE]
> Consult [docs/client/README.md](docs/client/README.md) for full details on Sparrow clients and their implementation.


A Sparrow client is what the end-user runs to connect to and communicate with Sparrow servers. Through the client, users send requests to the server and receive responses back. Triggered events will also be communicated to the client from the server.

As this section name implies, there is not just one single Sparrow client. There is a [reference implementation client maintained by Sparrow](https://github.com/addisonkline/sparrow-client), which is MIT-licensed. This means that, unlike the clients for some other chat servers out there, users are free to modify this client however they like, use a third-party client, or even make their own client from scratch and distribute it. When it comes to clients, we believe users should be in control of their own experience.

> [!IMPORTANT]
> Unlike the reference client, which has an MIT license, the Sparrow Protocol and implementation server are licensed under [AGPLv3](https://www.gnu.org/licenses/agpl-3.0.en.html). This is a copyleft license, which means that users are free to modify the protocol and server however they see fit, but any such derivative work must be open-source.

## Installing

To download/install Sparrow, you can either download the compiled binary for your system triple (e.g. ```x86_64-linux-gnu```), or you can dowload the source code and compile Sparrow yourself. Whatever you prefer.

### Downloading the Binary

>[!TODO]

### Building from Source

> [!IMPORTANT]
> Ensure you have both Git and Rust (version 1.86 or later) installed before beginning.

To build the Sparrow server binary from source, perform the following steps. 

First, clone the repository to the target system:

```bash
git clone https://github.com/addisonkline/sparrow
```

Next, compile the server code and save the generated binary to your desired path:

```bash
cargo build --release --target-dir {desired_path_here}
```

You should be good to go!

## Running Sparrow

### Creating a Server

With Sparrow installed in your target directory you specified earlier, ```cd``` there:

```bash
cd {sparrow_path}
```

(Alternatively, you can add Sparrow to your PATH. The process for doing this depends on your operating system.)

To initialize a new server with a given name and blurb (server description), run the following:

```bash
sparrow init --name {name} --blurb {blurb}
```

If this was successful, then congratulations, you made a new Sparrow server! However, you haven't started running it yet.

### Running a Server

Assuming your server with name ```name``` was created successfully, you can run it as follows:

```bash
sparrow run {name}
```

The server will continue running and accepting clients connections for as long as it stays running. To close it, type ```stop``` and press enter, or press Ctrl+C.

### Runtime Commands

Runtime commands allow the server owner (you) to interact with a live server without needing a client. These commands allow you to do things like stopping the server or kicking a client.

For a complete list of Sparrow server runtime commands, go [here](/docs/server/README.md##runtime-commands).

## Server Management

### Security and Safety

As with any kind of server, a Sparrow server can be vulnerable if you don't take specific precautions to protect it.

Every Sparrow server is members-only by default. This means that all attempted connections by an unapproved user will be refused. You can change this if you want--just go to the server's ```config.toml``` and set ```members_only``` to ```false```. Note that making this change opens your server up to any Sparrow client, meaning potentially-unwanted guests can connect. Make this change with extreme caution.

You have the ability to ban current members during runtime with the [ban command](docs/server/runtime/ban.md). 

You can also kick connected members without banning them via the [kick command](docs/server/runtime/kick.md). 

### Running Multiple Servers

> [!IMPORTANT]
> When being run at the same time, separate Sparrow servers need to run on separate ports. The default port is 8848. You can change a server's port by going to its ```config.toml``` and changing ```port``` to a valid, unoccupied port number.

The ```sparrow``` binary allows you to create, configure, and run multiple servers--even concurrently.

To run an existing server in the background:

```bash
sparrow start {server_name}
```

Stop it with:

```bash
sparrow stop {server_name}
```

You can also see which servers are running at any given time:

```bash
sparrow list
```