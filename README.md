# Sparrow

A decentralized, lightweight chat server designed to be practical yet powerful. Completely free and open-source.

## Contents

1. [What is Sparrow?](##what-is-sparrow?)
2. [The Sparrow Protocol](##the-sparrow-protocol)
3. [The Sparrow Server](##the-sparrow-server)
4. [Installing](##installing)
    1. [Downloading the Binary](###downloading-the-binary)
    2. [Building from Source](###building-from-source)
5. [Running Sparrow](##running-sparrow)
    1. [Creating a Server](###creating-a-server)
    2. [Running a Server](###running-a-server)
    3. [Runtime Commands](###runtime-commands)

## What is Sparrow?

Sparrow is an open-source, WebSocket-based chat server written in Rust and using the Sparrow Protocol. 

Sparrow is also **decentralized**--that means there is no central server in charge of storing user or server information. Your server and its data are controlled only by you.

## The Sparrow Protocol

> [!NOTE]
> Consult ```docs/protocol/README.md``` for full details on the Sparrow Protocol and its implementation.

The Sparrow Protocol details the rules of/formatting for WebSocket-based communication between a Sparrow server and client.

## The Sparrow Server

> [!NOTE]
> Consult ```docs/server/README.md``` for full details on the Sparrow server and its implementation.

The complete source code for the Sparrow server can be found in the ```src``` directory.

## Installing

To download/install Sparrow, you can either download the compiled binary for your system triple (e.g. ```x86_64-linux-gnu```), or you can dowload the source code and compile Sparrow yourself. Whatever you prefer.

### Downloading the Binary

>[!TODO]

### Building from Source

To build the Sparrow server binary from source, perform the following steps. 

> [!IMPORTANT]
> Ensure you have both Git and Rust (version 1.86 or later) installed before beginning.

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

The server will continue running and accepting clients connections for as long as it stays running. To close it, press Ctrl+C.

### Runtime Commands

> [!TODO]