# Sparrow Client Specification

> [!IMPORTANT]
> This document describes general guidelines for how a Sparrow client should be implemented. For documentation specifically covering the Sparrow reference client, check out [that repository](https://github.com/addisonkline/sparrow-client).

**Created: May 15, 2025**

**Last Updated: May 15, 2025**

## Contents

- [Sparrow Client Specification](#sparrow-client-specification)
  - [Contents](#contents)
  - [Schema](#schema)
    - [File Organization](#file-organization)
    - [servers](#servers)

## Schema

### File Organization

```
sparrow/
    client/
        id.key
        id.did
        servers
    servers/
```

### servers

```json
[
    {
        "ip": "string",
        "port": "int, default = 8848",
        "online": "bool",
        "connected": "bool"
    }
]
```