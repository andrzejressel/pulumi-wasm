<!-- Bug: Type and Name are switched -->
Manages the lifecycle of a Docker container.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as docker from "@pulumi/docker";

// Find the latest Ubuntu precise image.
const ubuntuRemoteImage = new docker.RemoteImage("ubuntuRemoteImage", {name: "ubuntu:precise"});
// Start a container
const ubuntuContainer = new docker.Container("ubuntuContainer", {image: ubuntuRemoteImage.imageId});
```
```python
import pulumi
import pulumi_docker as docker

# Find the latest Ubuntu precise image.
ubuntu_remote_image = docker.RemoteImage("ubuntuRemoteImage", name="ubuntu:precise")
# Start a container
ubuntu_container = docker.Container("ubuntuContainer", image=ubuntu_remote_image.image_id)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Docker = Pulumi.Docker;

return await Deployment.RunAsync(() => 
{
    // Find the latest Ubuntu precise image.
    var ubuntuRemoteImage = new Docker.RemoteImage("ubuntuRemoteImage", new()
    {
        Name = "ubuntu:precise",
    });

    // Start a container
    var ubuntuContainer = new Docker.Container("ubuntuContainer", new()
    {
        Image = ubuntuRemoteImage.ImageId,
    });

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
		// Find the latest Ubuntu precise image.
		ubuntuRemoteImage, err := docker.NewRemoteImage(ctx, "ubuntuRemoteImage", &docker.RemoteImageArgs{
			Name: pulumi.String("ubuntu:precise"),
		})
		if err != nil {
			return err
		}
		// Start a container
		_, err = docker.NewContainer(ctx, "ubuntuContainer", &docker.ContainerArgs{
			Image: ubuntuRemoteImage.ImageId,
		})
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
import com.pulumi.docker.RemoteImage;
import com.pulumi.docker.RemoteImageArgs;
import com.pulumi.docker.Container;
import com.pulumi.docker.ContainerArgs;
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
        var ubuntuRemoteImage = new RemoteImage("ubuntuRemoteImage", RemoteImageArgs.builder()        
            .name("ubuntu:precise")
            .build());

        var ubuntuContainer = new Container("ubuntuContainer", ContainerArgs.builder()        
            .image(ubuntuRemoteImage.imageId())
            .build());

    }
}
```
```yaml
resources:
  # Start a container
  ubuntuContainer:
    type: docker:Container
    properties:
      image: ${ubuntuRemoteImage.imageId}
  # Find the latest Ubuntu precise image.
  ubuntuRemoteImage:
    type: docker:RemoteImage
    properties:
      name: ubuntu:precise
```
<!--End PulumiCodeChooser -->

## Import

### Example

Assuming you created a `container` as follows

#!/bin/bash

docker run --name foo -p8080:80 -d nginx 

prints the container ID 

9a550c0f0163d39d77222d3efd58701b625d47676c25c686c95b5b92d1cba6fd

you provide the definition for the resource as follows

terraform

resource "docker_container" "foo" {

  name  = "foo"

  image = "nginx"

  ports {

    internal = "80"

    external = "8080"

  }

}

then the import command is as follows

#!/bin/bash

```sh
$ pulumi import docker:index/container:Container foo 9a550c0f0163d39d77222d3efd58701b625d47676c25c686c95b5b92d1cba6fd
```

