# Sparrow Server Specification

**Created: May 15, 2025**

**Last Updated: May 15, 2025**

## Contents

- [Sparrow Server Specification](#sparrow-server-specification)
  - [Contents](#contents)
  - [Schema](#schema)
    - [File Organization](#file-organization)
    - [config.toml](#configtoml)
    - [data/members](#datamembers)
    - [data/messages](#datamessages)
    - [data/logs](#datalogs)
  - [Server CLI](#server-cli)
  - [Server Runtime](#server-runtime)

## Schema

### File Organization

```
sparrow/
    client/
    servers/
        example/
            config.toml
            data/
                members
                messages
                logs
```

### config.toml

```toml
[meta]
version = "version no. as string"
name = "string"
blurb = "string"
created_at = "datetime"

[hosting]
port = "int, default = 8848"
max_clients = "int, default = 100"

[security]
members_only = "bool, default = true"

[data]
enable_logging = "bool, default = true"
prune_after = "int, default = 30 (days)"
```

### data/members

```json
[
    {
        "tag": "DID as string",
        "alias": "string",
        "joined_at": "datetime",
        "level": "int",
        "online": "bool"
    }
]
```

### data/messages

```json
[
    {
        "id": "ULID as string",
        "user_tag": "DID as string",
        "timestamp": "datetime",
        "content": "string",
        "edited": "bool"
    }
]
```

### data/logs

```log
[{timestamp} {level} {function}] {message} 
```

## Server CLI

The ```sparrow``` binary is more than just a server. It is a program that can be used to create, configure, and run discrete Sparrow servers--concurrently, too, if you wish.

Specifically, the Sparrow command-line interface (CLI) has the following commands:
- [init](cli/init.md): Create a new server.
- [run](cli/run.md): Start up an existing server and output to the console until closed.
- [start](cli/start.md): Start up an existing server in the background.
- [listen](cli/listen.md): Open up a running server in the console.
- [stop](cli/stop.md): Close and save a server running in the background.
- [list](cli/list.md): List all the user's existing servers.

You can also view usage instructions for a given command by running the following:

```bash
sparrow {command} --help
```

## Server Runtime
When your server is running and you are listening to it in the console, you can execute **runtime commands**--special requests to the server thast only the server owner has access to.

> [!IMPORTANT]
> Sparrow runtime commands are not the same as communications between the server and client. These are specified in the [Sparrow Protocol documentation](../protocol/README.md).

Here are all the runtime commands:
- [stop](runtime/stop.md): Save and close the server.
- [leave](runtime/leave.md): Stop outputting the server to the console, but do not close it.
- [kick](runtime/kick.md): Disconnect a client from the server.
- [save](runtime/save.md): Save all current server state data.

Finally, all runtime commands and their usages can be given by running ```help```.