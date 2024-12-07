<!-- Bug: Type and Name are switched -->
`docker.Network` provides a docker network resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as docker from "@pulumi/docker";

const privateNetwork = new docker.Network("privateNetwork", {});
```
```python
import pulumi
import pulumi_docker as docker

private_network = docker.Network("privateNetwork")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Docker = Pulumi.Docker;

return await Deployment.RunAsync(() => 
{
    var privateNetwork = new Docker.Network("privateNetwork");

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-docker/sdk/v4/go/docker"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := docker.NewNetwork(ctx, "privateNetwork", nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.docker.Network;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var privateNetwork = new Network("privateNetwork");

    }
}
```
```yaml
resources:
  privateNetwork:
    type: docker:Network
```
<!--End PulumiCodeChooser -->

## Import

### Example

Assuming you created a `network` as follows

```shell
docker network create foo
````

prints the long ID

```text
87b57a9b91ecab2db2a6dbf38df74c67d7c7108cbe479d6576574ec2cd8c2d73
```

you provide the definition for the resource as follows

<!--Start PulumiCodeChooser -->
```yaml
resources:
  foo:
    type: docker:Network
    properties:
      name: "foo"
```
<!--End PulumiCodeChooser -->

then the import command is as follows

```sh
$ pulumi import docker:index/network:Network foo 87b57a9b91ecab2db2a6dbf38df74c67d7c7108cbe479d6576574ec2cd8c2d73
```

