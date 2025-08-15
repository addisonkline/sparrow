# CLI command `list`

Show metadata for all owned servers.

## Contents

- [CLI command `list`](#cli-command-list)
  - [Contents](#contents)
  - [Options](#options)
    - [verbose](#verbose)
  - [Description](#description)

## Options

### verbose 

Optional boolean, default = false. Determines the amount of per-server information to display.

## Description

For each server, print its corresponding metadata to the console. Specifically, print the following:
- ```name``` string
- ```blurb``` string (verbose only)
- ```version``` string
- ```online``` boolean
- ```clients``` int (verbose only)