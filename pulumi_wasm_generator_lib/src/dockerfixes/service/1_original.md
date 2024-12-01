<!-- Bug: Type and Name are switched -->
This resource manages the lifecycle of a Docker service. By default, the creation, update and delete of services are detached.
 With the Converge Config Name of the service
- `task_spec` (Block List, Min: 1, Max: 1) User modifiable task configuration (see below for nested schema)

## Import

### Example

Assuming you created a `service` as follows

#!/bin/bash

docker service create --name foo -p 8080:80 nginx

prints th ID

4pcphbxkfn2rffhbhe6czytgi

you provide the definition for the resource as follows

terraform

resource "docker_service" "foo" {

  name = "foo"

  task_spec {

    container_spec {

      image = "nginx"

    }

  }

  endpoint_spec {

    ports {

      target_port    = "80"

      published_port = "8080"

    }

  }

}

then the import command is as follows

#!/bin/bash

```sh
$ pulumi import docker:index/service:Service foo 4pcphbxkfn2rffhbhe6czytgi
```

