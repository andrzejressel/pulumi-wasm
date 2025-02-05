<!-- Bug: Type and Name are switched -->
This resource manages the lifecycle of a Docker service. By default, the creation, update and delete of services are detached.
With the Converge Config Name of the service
- `task_spec` (Block List, Min: 1, Max: 1) User modifiable task configuration (see below for nested schema)

## Import

### Example

Assuming you created a `service` as follows

```shell
docker service create --name foo -p 8080:80 nginx
```

prints this ID

```text
4pcphbxkfn2rffhbhe6czytgi
```

you provide the definition for the resource as follows

<!--Start PulumiCodeChooser -->
```yaml
resources:
  foo:
    type: docker:Service
    name: foo
    properties:
      taskSpec:
        containerSpec:
          image: "nginx"
      endpointSpec:
        ports:
          - targetPort: 80
            publishedPort: 8080
```
<!--End PulumiCodeChooser -->

then the import command is as follows

```sh
$ pulumi import docker:index/service:Service foo 4pcphbxkfn2rffhbhe6czytgi
```

