

## Import

### Example

Assuming you created a `config` as follows

```shell
printf '{"a":"b"}' | docker config create foo -
```

prints the id 

```text
08c26c477474478d971139f750984775a7f019dbe8a2e7f09d66a187c009e66d
```

you provide the definition for the resource as follows

<!--Start PulumiCodeChooser -->
```yaml
resources:
  foo:
    type: docker:ServiceConfig
    name: foo
    properties:
      data: 'base64encode("{\"a\": \"b\"}")'
```
<!--End PulumiCodeChooser -->

then the import command is as follows

```sh
$ pulumi import docker:index/serviceConfig:ServiceConfig foo 08c26c477474478d971139f750984775a7f019dbe8a2e7f09d66a187c009e66d
```

