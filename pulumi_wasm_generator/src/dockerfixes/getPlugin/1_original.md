Reads the local Docker plugin. The plugin must be installed locally.

## Example Usage

### With alias
data "docker.Plugin" "by_alias" {
  alias = "sample-volume-plugin:latest"
}

### With ID
data "docker.Plugin" "by_id" {
  id = "e9a9db917b3bfd6706b5d3a66d4bceb9f"
}
```
