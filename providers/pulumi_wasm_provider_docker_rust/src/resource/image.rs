//! `Image` builds a Docker image and pushes it Docker and OCI compatible registries.
//! This resource enables running Docker builds as part of a Pulumi deployment.
//! 
//! Note: This resource does not delete tags, locally or remotely, when destroyed.
//! 
//! ## Image name
//! 
//! The Image resource uses `imageName` to refer to a fully qualified Docker image name, by the format `repository:tag`.
//! Note that this does not include any digest information and thus will not cause any updates when passed to dependencies,
//! even when using `latest` tag. To trigger such updates, e.g. when referencing pushed images in container orchestration 
//! and management resources, please use the `repoDigest` Output instead, which is of the format 
//! `repository@<algorithm>:<hash>` and unique per build/push. 
//! As of Docker v4.4, `repoDigest` is now available for local Images.
//! 
//! ## Cross-platform builds
//! 
//! The Image resource supports cross-platform builds when the [Docker engine has cross-platform support enabled via emulators](https://docs.docker.com/build/building/multi-platform/#building-multi-platform-images).
//! The Image resource currently supports providing only a single operating system and architecture in the `platform` field, e.g.: `linux/amd64`.
//! To enable this support, you may need to install the emulators in the environment running your Pulumi program.
//! 
//! If you are using Linux, you may be using Docker Engine or Docker Desktop for Linux, depending on how you have installed Docker. The [FAQ for Docker Desktop for Linux](https://docs.docker.com/desktop/faqs/linuxfaqs/#context) describes the differences and how to select which Docker context is in use.
//! 
//! * For local development using Docker Desktop, this is enabled by default.
//! * For systems using Docker Engine, install the QEMU binaries and register them with using the docker image from [github.com/tonistiigi/binfmt](https://github.com/tonistiigi/binfmt):
//! 
//!   ```shell
//!   docker run --privileged --rm tonistiigi/binfmt --install all
//!   ```
//! * In a GitHub Actions workflow, the [docker/setup-qemu-action](https://github.com/docker/setup-qemu-action) can be used instead by adding this step to your workflow file. Example workflow usage:
//! 
//!   ```yaml
//!   name: Pulumi
//!   on:
//!     push:
//!       branches:
//!         - master
//!   jobs:
//!     up:
//!       name: Preview
//!       runs-on: ubuntu-latest
//!       steps:
//!           # This is the step added:
//!         - name: Set up QEMU
//!           uses: docker/setup-qemu-action@v2
//!           # The ordinary pulumi/actions workflow:
//!         - uses: actions/checkout@v3
//!         - uses: pulumi/actions@v4
//!           with:
//!             command: preview
//!             stack-name: org-name/stack-name
//!           env:
//!             PULUMI_ACCESS_TOKEN: ${{ secrets.PULUMI_ACCESS_TOKEN }}
//!   ```
//! 
//! 
//! ## Example Usage
//! ### A Docker image build
//! 
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as docker from "@pulumi/docker";
//! 
//! const demoImage = new docker.Image("demo-image", {
//!     build: {
//!         context: ".",
//!         dockerfile: "Dockerfile",
//!         platform: "linux/amd64",
//!     },
//!     imageName: "username/image:tag1",
//!     skipPush: true,
//! });
//! export const imageName = demoImage.imageName;
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_docker as docker
//! 
//! demo_image = docker.Image("demo-image",
//!     build=docker.DockerBuildArgs(
//!         context=".",
//!         dockerfile="Dockerfile",
//!         platform="linux/amd64",
//!     ),
//!     image_name="username/image:tag1",
//!     skip_push=True)
//! pulumi.export("imageName", demo_image.image_name)
//! ```
//! ### C#
//! ```csharp
//! using System.Collections.Generic;
//! using System.Linq;
//! using Pulumi;
//! using Docker = Pulumi.Docker;
//! 
//! return await Deployment.RunAsync(() => 
//! {
//!     var demoImage = new Docker.Image("demo-image", new()
//!     {
//!         Build = new Docker.Inputs.DockerBuildArgs
//!         {
//!             Context = ".",
//!             Dockerfile = "Dockerfile",
//!             Platform = "linux/amd64",
//!         },
//!         ImageName = "username/image:tag1",
//!         SkipPush = true,
//!     });
//! 
//!     return new Dictionary<string, object?>
//!     {
//!         ["imageName"] = demoImage.ImageName,
//!     };
//! });
//! 
//! ```
//! ### Go
//! ```go
//! package main
//! 
//! import (
//! 	"github.com/pulumi/pulumi-docker/sdk/v4/go/docker"
//! 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
//! )
//! 
//! func main() {
//! 	pulumi.Run(func(ctx *pulumi.Context) error {
//! 		demoImage, err := docker.NewImage(ctx, "demo-image", &docker.ImageArgs{
//! 			Build: &docker.DockerBuildArgs{
//! 				Context:    pulumi.String("."),
//! 				Dockerfile: pulumi.String("Dockerfile"),
//! 				Platform:   pulumi.String("linux/amd64"),
//! 			},
//! 			ImageName: pulumi.String("username/image:tag1"),
//! 			SkipPush:  pulumi.Bool(true),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		ctx.Export("imageName", demoImage.ImageName)
//! 		return nil
//! 	})
//! }
//! ```
//! ### YAML
//! ```yaml
//! config: {}
//! description: A Docker image build
//! name: image-yaml
//! outputs:
//!     imageName: ${demo-image.imageName}
//! resources:
//!     demo-image:
//!         options:
//!             version: v4.4.0
//!         properties:
//!             build:
//!                 context: .
//!                 dockerfile: Dockerfile
//!                 platform: linux/amd64
//!             imageName: username/image:tag1
//!             skipPush: true
//!         type: docker:Image
//! runtime: yaml
//! variables: {}
//! ```
//! ### Java
//! ```java
//! package generated_program;
//! 
//! import com.pulumi.Context;
//! import com.pulumi.Pulumi;
//! import com.pulumi.core.Output;
//! import com.pulumi.docker.Image;
//! import com.pulumi.docker.ImageArgs;
//! import com.pulumi.docker.inputs.DockerBuildArgs;
//! import java.util.List;
//! import java.util.ArrayList;
//! import java.util.Map;
//! import java.io.File;
//! import java.nio.file.Files;
//! import java.nio.file.Paths;
//! 
//! public class App {
//!     public static void main(String[] args) {
//!         Pulumi.run(App::stack);
//!     }
//! 
//!     public static void stack(Context ctx) {
//!         var demoImage = new Image("demoImage", ImageArgs.builder()        
//!             .build(DockerBuildArgs.builder()
//!                 .context(".")
//!                 .dockerfile("Dockerfile")
//!                 .platform("linux/amd64")
//!                 .build())
//!             .imageName("username/image:tag1")
//!             .skipPush(true)
//!             .build());
//! 
//!         ctx.export("imageName", demoImage.imageName());
//!     }
//! }
//! ```
//! ### A Docker image build and push
//! 
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as docker from "@pulumi/docker";
//! 
//! const demoPushImage = new docker.Image("demo-push-image", {
//!     build: {
//!         context: ".",
//!         dockerfile: "Dockerfile",
//!     },
//!     imageName: "docker.io/username/push-image:tag1",
//! });
//! export const imageName = demoPushImage.imageName;
//! export const repoDigest = demoPushImage.repoDigest;
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_docker as docker
//! 
//! demo_push_image = docker.Image("demo-push-image",
//!     build=docker.DockerBuildArgs(
//!         context=".",
//!         dockerfile="Dockerfile",
//!     ),
//!     image_name="docker.io/username/push-image:tag1")
//! pulumi.export("imageName", demo_push_image.image_name)
//! pulumi.export("repoDigest", demo_push_image.repo_digest)
//! ```
//! ### C#
//! ```csharp
//! using System.Collections.Generic;
//! using System.Linq;
//! using Pulumi;
//! using Docker = Pulumi.Docker;
//! 
//! return await Deployment.RunAsync(() => 
//! {
//!     var demoPushImage = new Docker.Image("demo-push-image", new()
//!     {
//!         Build = new Docker.Inputs.DockerBuildArgs
//!         {
//!             Context = ".",
//!             Dockerfile = "Dockerfile",
//!         },
//!         ImageName = "docker.io/username/push-image:tag1",
//!     });
//! 
//!     return new Dictionary<string, object?>
//!     {
//!         ["imageName"] = demoPushImage.ImageName,
//!         ["repoDigest"] = demoPushImage.RepoDigest,
//!     };
//! });
//! 
//! ```
//! ### Go
//! ```go
//! package main
//! 
//! import (
//! 	"github.com/pulumi/pulumi-docker/sdk/v4/go/docker"
//! 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
//! )
//! 
//! func main() {
//! 	pulumi.Run(func(ctx *pulumi.Context) error {
//! 		demoPushImage, err := docker.NewImage(ctx, "demo-push-image", &docker.ImageArgs{
//! 			Build: &docker.DockerBuildArgs{
//! 				Context:    pulumi.String("."),
//! 				Dockerfile: pulumi.String("Dockerfile"),
//! 			},
//! 			ImageName: pulumi.String("docker.io/username/push-image:tag1"),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		ctx.Export("imageName", demoPushImage.ImageName)
//! 		ctx.Export("repoDigest", demoPushImage.RepoDigest)
//! 		return nil
//! 	})
//! }
//! ```
//! ### YAML
//! ```yaml
//! config: {}
//! description: A Docker image build and push
//! name: image-push-yaml
//! outputs:
//!     imageName: ${demo-push-image.imageName}
//!     repoDigest: ${demo-push-image.repoDigest}
//! resources:
//!     demo-push-image:
//!         options:
//!             version: v4.4.0
//!         properties:
//!             build:
//!                 context: .
//!                 dockerfile: Dockerfile
//!             imageName: docker.io/username/push-image:tag1
//!         type: docker:Image
//! runtime: yaml
//! variables: {}
//! ```
//! ### Java
//! ```java
//! package generated_program;
//! 
//! import com.pulumi.Context;
//! import com.pulumi.Pulumi;
//! import com.pulumi.core.Output;
//! import com.pulumi.docker.Image;
//! import com.pulumi.docker.ImageArgs;
//! import com.pulumi.docker.inputs.DockerBuildArgs;
//! import java.util.List;
//! import java.util.ArrayList;
//! import java.util.Map;
//! import java.io.File;
//! import java.nio.file.Files;
//! import java.nio.file.Paths;
//! 
//! public class App {
//!     public static void main(String[] args) {
//!         Pulumi.run(App::stack);
//!     }
//! 
//!     public static void stack(Context ctx) {
//!         var demoPushImage = new Image("demoPushImage", ImageArgs.builder()        
//!             .build(DockerBuildArgs.builder()
//!                 .context(".")
//!                 .dockerfile("Dockerfile")
//!                 .build())
//!             .imageName("docker.io/username/push-image:tag1")
//!             .build());
//! 
//!         ctx.export("imageName", demoPushImage.imageName());
//!         ctx.export("repoDigest", demoPushImage.repoDigest());
//!     }
//! }
//! ```
//! ### Docker image build using caching with AWS Elastic Container Registry
//! 
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as aws from "@pulumi/aws";
//! import * as docker from "@pulumi/docker";
//! 
//! const ecrRepository = new aws.ecr.Repository("ecr-repository", {name: "docker-repository"});
//! const authToken = aws.ecr.getAuthorizationTokenOutput({
//!     registryId: ecrRepository.registryId,
//! });
//! const myAppImage = new docker.Image("my-app-image", {
//!     build: {
//!         args: {
//!             BUILDKIT_INLINE_CACHE: "1",
//!         },
//!         cacheFrom: {
//!             images: [pulumi.interpolate`${ecrRepository.repositoryUrl}:latest`],
//!         },
//!         context: "app/",
//!         dockerfile: "Dockerfile",
//!     },
//!     imageName: pulumi.interpolate`${ecrRepository.repositoryUrl}:latest`,
//!     registry: {
//!         password: pulumi.secret(authToken.apply(authToken => authToken.password)),
//!         server: ecrRepository.repositoryUrl,
//!     },
//! });
//! export const imageName = myAppImage.imageName;
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_aws as aws
//! import pulumi_docker as docker
//! 
//! ecr_repository = aws.ecr.Repository("ecr-repository", name="docker-repository")
//! auth_token = aws.ecr.get_authorization_token_output(registry_id=ecr_repository.registry_id)
//! my_app_image = docker.Image("my-app-image",
//!     build=docker.DockerBuildArgs(
//!         args={
//!             "BUILDKIT_INLINE_CACHE": "1",
//!         },
//!         cache_from=docker.CacheFromArgs(
//!             images=[ecr_repository.repository_url.apply(lambda repository_url: f"{repository_url}:latest")],
//!         ),
//!         context="app/",
//!         dockerfile="Dockerfile",
//!     ),
//!     image_name=ecr_repository.repository_url.apply(lambda repository_url: f"{repository_url}:latest"),
//!     registry=docker.RegistryArgs(
//!         password=pulumi.Output.secret(auth_token.password),
//!         server=ecr_repository.repository_url,
//!     ))
//! pulumi.export("imageName", my_app_image.image_name)
//! ```
//! ### C#
//! ```csharp
//! using System.Collections.Generic;
//! using System.Linq;
//! using Pulumi;
//! using Aws = Pulumi.Aws;
//! using Docker = Pulumi.Docker;
//! 
//! return await Deployment.RunAsync(() => 
//! {
//!     var ecrRepository = new Aws.Ecr.Repository("ecr-repository", new()
//!     {
//!         Name = "docker-repository",
//!     });
//! 
//!     var authToken = Aws.Ecr.GetAuthorizationToken.Invoke(new()
//!     {
//!         RegistryId = ecrRepository.RegistryId,
//!     });
//! 
//!     var myAppImage = new Docker.Image("my-app-image", new()
//!     {
//!         Build = new Docker.Inputs.DockerBuildArgs
//!         {
//!             Args = 
//!             {
//!                 { "BUILDKIT_INLINE_CACHE", "1" },
//!             },
//!             CacheFrom = new Docker.Inputs.CacheFromArgs
//!             {
//!                 Images = new[]
//!                 {
//!                     ecrRepository.RepositoryUrl.Apply(repositoryUrl => $"{repositoryUrl}:latest"),
//!                 },
//!             },
//!             Context = "app/",
//!             Dockerfile = "Dockerfile",
//!         },
//!         ImageName = ecrRepository.RepositoryUrl.Apply(repositoryUrl => $"{repositoryUrl}:latest"),
//!         Registry = new Docker.Inputs.RegistryArgs
//!         {
//!             Password = Output.CreateSecret(authToken.Apply(getAuthorizationTokenResult => getAuthorizationTokenResult.Password)),
//!             Server = ecrRepository.RepositoryUrl,
//!         },
//!     });
//! 
//!     return new Dictionary<string, object?>
//!     {
//!         ["imageName"] = myAppImage.ImageName,
//!     };
//! });
//! 
//! ```
//! ### Go
//! ```go
//! package main
//! 
//! import (
//! 	"fmt"
//! 
//! 	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ecr"
//! 	"github.com/pulumi/pulumi-docker/sdk/v4/go/docker"
//! 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
//! )
//! 
//! func main() {
//! 	pulumi.Run(func(ctx *pulumi.Context) error {
//! 		ecrRepository, err := ecr.NewRepository(ctx, "ecr-repository", &ecr.RepositoryArgs{
//! 			Name: pulumi.String("docker-repository"),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		authToken := ecr.GetAuthorizationTokenOutput(ctx, ecr.GetAuthorizationTokenOutputArgs{
//! 			RegistryId: ecrRepository.RegistryId,
//! 		}, nil)
//! 		myAppImage, err := docker.NewImage(ctx, "my-app-image", &docker.ImageArgs{
//! 			Build: &docker.DockerBuildArgs{
//! 				Args: pulumi.StringMap{
//! 					"BUILDKIT_INLINE_CACHE": pulumi.String("1"),
//! 				},
//! 				CacheFrom: &docker.CacheFromArgs{
//! 					Images: pulumi.StringArray{
//! 						ecrRepository.RepositoryUrl.ApplyT(func(repositoryUrl string) (string, error) {
//! 							return fmt.Sprintf("%v:latest", repositoryUrl), nil
//! 						}).(pulumi.StringOutput),
//! 					},
//! 				},
//! 				Context:    pulumi.String("app/"),
//! 				Dockerfile: pulumi.String("Dockerfile"),
//! 			},
//! 			ImageName: ecrRepository.RepositoryUrl.ApplyT(func(repositoryUrl string) (string, error) {
//! 				return fmt.Sprintf("%v:latest", repositoryUrl), nil
//! 			}).(pulumi.StringOutput),
//! 			Registry: &docker.RegistryArgs{
//! 				Password: pulumi.ToSecret(authToken.ApplyT(func(authToken ecr.GetAuthorizationTokenResult) (*string, error) {
//! 					return &authToken.Password, nil
//! 				}).(pulumi.StringPtrOutput)).(pulumi.StringOutput),
//! 				Server: ecrRepository.RepositoryUrl,
//! 			},
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		ctx.Export("imageName", myAppImage.ImageName)
//! 		return nil
//! 	})
//! }
//! ```
//! ### YAML
//! ```yaml
//! config: {}
//! description: Docker image build using caching with AWS Elastic Container Registry
//! name: image-caching-yaml
//! outputs:
//!     imageName: ${my-app-image.imageName}
//! resources:
//!     ecr-repository:
//!         properties:
//!             name: docker-repository
//!         type: aws:ecr:Repository
//!     my-app-image:
//!         options:
//!             version: v4.1.2
//!         properties:
//!             build:
//!                 args:
//!                     BUILDKIT_INLINE_CACHE: "1"
//!                 cacheFrom:
//!                     images:
//!                         - ${ecr-repository.repositoryUrl}:latest
//!                 context: app/
//!                 dockerfile: Dockerfile
//!             imageName: ${ecr-repository.repositoryUrl}:latest
//!             registry:
//!                 password:
//!                     fn::secret: ${authToken.password}
//!                 server: ${ecr-repository.repositoryUrl}
//!         type: docker:Image
//! runtime: yaml
//! variables:
//!     authToken:
//!         fn::aws:ecr:getAuthorizationToken:
//!             registryId: ${ecr-repository.registryId}
//! ```
//! ### Java
//! ```java
//! package generated_program;
//! 
//! import com.pulumi.Context;
//! import com.pulumi.Pulumi;
//! import com.pulumi.core.Output;
//! import com.pulumi.aws.ecr.Repository;
//! import com.pulumi.aws.ecr.RepositoryArgs;
//! import com.pulumi.aws.ecr.EcrFunctions;
//! import com.pulumi.aws.ecr.inputs.GetAuthorizationTokenArgs;
//! import com.pulumi.docker.Image;
//! import com.pulumi.docker.ImageArgs;
//! import com.pulumi.docker.inputs.DockerBuildArgs;
//! import com.pulumi.docker.inputs.CacheFromArgs;
//! import com.pulumi.docker.inputs.RegistryArgs;
//! import java.util.List;
//! import java.util.ArrayList;
//! import java.util.Map;
//! import java.io.File;
//! import java.nio.file.Files;
//! import java.nio.file.Paths;
//! 
//! public class App {
//!     public static void main(String[] args) {
//!         Pulumi.run(App::stack);
//!     }
//! 
//!     public static void stack(Context ctx) {
//!         var ecrRepository = new Repository("ecrRepository", RepositoryArgs.builder()        
//!             .name("docker-repository")
//!             .build());
//! 
//!         final var authToken = EcrFunctions.getAuthorizationToken(GetAuthorizationTokenArgs.builder()
//!             .registryId(ecrRepository.registryId())
//!             .build());
//! 
//!         var myAppImage = new Image("myAppImage", ImageArgs.builder()        
//!             .build(DockerBuildArgs.builder()
//!                 .args(Map.of("BUILDKIT_INLINE_CACHE", "1"))
//!                 .cacheFrom(CacheFromArgs.builder()
//!                     .images(ecrRepository.repositoryUrl().applyValue(repositoryUrl -> String.format("%s:latest", repositoryUrl)))
//!                     .build())
//!                 .context("app/")
//!                 .dockerfile("Dockerfile")
//!                 .build())
//!             .imageName(ecrRepository.repositoryUrl().applyValue(repositoryUrl -> String.format("%s:latest", repositoryUrl)))
//!             .registry(RegistryArgs.builder()
//!                 .password(Output.ofSecret(authToken.applyValue(getAuthorizationTokenResult -> getAuthorizationTokenResult).applyValue(authToken -> authToken.applyValue(getAuthorizationTokenResult -> getAuthorizationTokenResult.password()))))
//!                 .server(ecrRepository.repositoryUrl())
//!                 .build())
//!             .build());
//! 
//!         ctx.export("imageName", myAppImage.imageName());
//!     }
//! }
//! ```

pub struct ImageArgs {
    /// The Docker build context
    pub build: pulumi_wasm_rust::Output<Option<crate::types::DockerBuild>>,
    /// A flag to build an image on preview
    pub build_on_preview: pulumi_wasm_rust::Output<Option<bool>>,
    /// The image name, of the format repository[:tag], e.g. `docker.io/username/demo-image:v1`.
    /// This reference is not unique to each build and push.For the unique manifest SHA of a pushed docker image, or the local image ID, please use `repoDigest`.
    pub image_name: pulumi_wasm_rust::Output<String>,
    /// The registry to push the image to
    pub registry: pulumi_wasm_rust::Output<Option<crate::types::Registry>>,
    /// A flag to skip a registry push.
    pub skip_push: pulumi_wasm_rust::Output<Option<bool>>,
}

pub struct ImageResult {
    /// The fully qualified image name that was pushed to the registry.
    pub base_image_name: pulumi_wasm_rust::Output<String>,
    /// The path to the build context to use.
    pub context: pulumi_wasm_rust::Output<String>,
    /// The location of the Dockerfile relative to the docker build context.
    pub dockerfile: pulumi_wasm_rust::Output<String>,
    /// The fully qualified image name
    pub image_name: pulumi_wasm_rust::Output<String>,
    /// The image's architecture and OS
    pub platform: pulumi_wasm_rust::Output<Option<String>>,
    /// The name of the registry server hosting the image.
    pub registry_server: pulumi_wasm_rust::Output<String>,
    /// **For pushed images:**
    /// The manifest digest of an image pushed to a registry, of the format repository@<algorithm>:<hash>, e.g. `username/demo-image@sha256:a6ae6dd8d39c5bb02320e41abf00cd4cb35905fec540e37d306c878be8d38bd3`.
    /// This reference is unique per image build and push. 
    /// Only available for images pushed to a registry.
    /// Use when passing a reference to a pushed image to container management resources.
    /// 
    /// **Local-only images**For local images, this field is the image ID of the built local image, of the format <algorithm>:<hash>, e.g `sha256:826a130323165bb0ccb0374ae774f885c067a951b51a6ee133577f4e5dbc4119` 
    pub repo_digest: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ImageArgs) -> ImageResult {

    let result = crate::bindings::pulumi::docker::image::invoke(name, &crate::bindings::pulumi::docker::image::Args {
        build: args.build.get_inner(),
        build_on_preview: args.build_on_preview.get_inner(),
        image_name: args.image_name.get_inner(),
        registry: args.registry.get_inner(),
        skip_push: args.skip_push.get_inner(),
    });

    ImageResult {
        base_image_name: crate::into_domain(result.base_image_name),
        context: crate::into_domain(result.context),
        dockerfile: crate::into_domain(result.dockerfile),
        image_name: crate::into_domain(result.image_name),
        platform: crate::into_domain(result.platform),
        registry_server: crate::into_domain(result.registry_server),
        repo_digest: crate::into_domain(result.repo_digest),
    }
}
