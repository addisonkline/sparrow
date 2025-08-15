# Runtime command `kick`

Close the connection between the server and a specific client.

## Contents

- [Runtime command `kick`](#runtime-command-kick)
  - [Contents](#contents)
  - [Arguments](#arguments)
    - [user](#user)
  - [Description](#description)

## Arguments

### user

Required string. The DID of the user to kick.

## Description

Assuming the user argument is valid (i.e. corresponds to the DID of a connected client), close their connection with the server. 