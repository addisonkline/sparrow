# cli command run

Run an existing server and output it to the console.

## Contents

- [cli command run](#cli-command-run)
  - [Contents](#contents)
  - [Arguments](#arguments)
    - [name](#name)
  - [Description](#description)

## Arguments

### name

Required string. The name of the server to run.

## Description

First, determines a server with the given name exists. Assuming the version of the server binary is compatible with said server's version in ```config.toml```, the server should start up successfully. The server will then be accessible on the console, and runtime commands can be executed. Use the runtime commands [stop](../runtime/stop.md) or [leave](../runtime/leave.md) to return to the command line (or alternatively, press Ctrl+C).