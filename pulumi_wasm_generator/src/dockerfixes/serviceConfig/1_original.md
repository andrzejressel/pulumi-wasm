

## Import

### Example

Assuming you created a `config` as follows

#!/bin/bash

printf '{"a":"b"}' | docker config create foo -

prints the id 

08c26c477474478d971139f750984775a7f019dbe8a2e7f09d66a187c009e66d

you provide the definition for the resource as follows

terraform

resource "docker_config" "foo" {

  name = "foo"

  data = base64encode("{\"a\": \"b\"}")

}

then the import command is as follows

#!/bin/bash

```sh
$ pulumi import docker:index/serviceConfig:ServiceConfig foo 08c26c477474478d971139f750984775a7f019dbe8a2e7f09d66a187c009e66d
```

