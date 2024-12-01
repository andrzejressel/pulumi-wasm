Reads the local Docker plugin. The plugin must be installed locally.

## Example Usage

### With alias
```yaml
variables:
  byAlias:
    fn::docker:getPlugin:
      alias: "sample-volume-plugin:latest"
```

### With ID
```yaml
variables:
  byId:
    fn::docker:getPlugin:
      id: "e9a9db917b3bfd6706b5d3a66d4bceb9f"
```

