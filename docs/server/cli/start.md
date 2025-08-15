# CLI command `start`

Start up an existing server, but run it in the background.

## Contents

- [CLI command `start`](#cli-command-start)
  - [Contents](#contents)
  - [Arguments](#arguments)
    - [name](#name)
  - [Description](#description)

## Arguments

### name

Required string. The name of the server to start.

## Description

Assuming the given server name is valid, and that the server binary and the server file's ```config.toml``` are version-compatible, the server starts up in the background and does not output to the console. The command finishes executing as soon as the server starts up successfuly (or as soon as it fails).