Reads the local Docker plugin. The plugin must be installed locally.

## Example Usage

### With alias
<!--Start PulumiCodeChooser -->
```yaml
variables:
  byAlias:
    fn::docker:getPlugin:
      alias: "sample-volume-plugin:latest"
```
<!--End PulumiCodeChooser -->

### With ID
<!--Start PulumiCodeChooser -->
```yaml
variables:
  byId:
    fn::docker:getPlugin:
      id: "e9a9db917b3bfd6706b5d3a66d4bceb9f"
```
<!--End PulumiCodeChooser -->
