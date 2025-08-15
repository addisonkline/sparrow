# CLI command `listen`

Open up a server that is running in the background.

## Contents

- [CLI command `listen`](#cli-command-listen)
  - [Contents](#contents)
  - [Arguments](#arguments)
    - [name](#name)
  - [Description](#description)

## Arguments

### name

Required string. The name of the server to begin listening to.

## Description

Assuming the server name is valid (i.e. corresponds to an existing server running in the background), the running server begins to output to the console. The server will then be accessible on the console, and runtime commands can be executed. Use the runtime commands [stop](../runtime/stop.md) or [leave](../runtime/leave.md) to return to the command line (or alternatively, press Ctrl+C).