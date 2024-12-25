/// <!-- Bug: Type and Name are switched -->
/// Manages the lifecycle of a Docker container.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let ubuntuContainer = container::create(
///         "ubuntuContainer",
///         ContainerArgs::builder().image("${ubuntuRemoteImage.imageId}").build_struct(),
///     );
///     let ubuntuRemoteImage = remote_image::create(
///         "ubuntuRemoteImage",
///         RemoteImageArgs::builder().name("ubuntu:precise").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ### Example
///
/// Assuming you created a `container` as follows
///
/// ```sh
/// docker run --name foo -p8080:80 -d nginx
/// ```
///
/// prints the container ID
///
/// ```text
/// 9a550c0f0163d39d77222d3efd58701b625d47676c25c686c95b5b92d1cba6fd
/// ```
///
/// you provide the definition for the resource as follows
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foo = container::create(
///         "foo",
///         ContainerArgs::builder()
///             .image("nginx")
///             .name("foo")
///             .ports(
///                 vec![
///                     ContainerPort::builder().external(8080).internal(80).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// then the import command is as follows
///
/// ```sh
/// $ pulumi import docker:index/container:Container foo 9a550c0f0163d39d77222d3efd58701b625d47676c25c686c95b5b92d1cba6fd
/// ```
///
pub mod container {
    include!("resources/container.rs");
}
/// `Image` builds a Docker image and pushes it Docker and OCI compatible registries.
/// This resource enables running Docker builds as part of a Pulumi deployment.
///
/// Note: This resource does not delete tags, locally or remotely, when destroyed.
///
/// ## Image name
///
/// The Image resource uses `imageName` to refer to a fully qualified Docker image name, by the format `repository:tag`.
/// Note that this does not include any digest information and thus will not cause any updates when passed to dependencies,
/// even when using `latest` tag. To trigger such updates, e.g. when referencing pushed images in container orchestration
/// and management resources, please use the `repoDigest` Output instead, which is of the format
/// `repository@<algorithm>:<hash>` and unique per build/push.
/// As of Docker v4.4, `repoDigest` is now available for local Images.
///
/// ## Cross-platform builds
///
/// The Image resource supports cross-platform builds when the [Docker engine has cross-platform support enabled via emulators](https://docs.docker.com/build/building/multi-platform/#building-multi-platform-images).
/// The Image resource currently supports providing only a single operating system and architecture in the `platform` field, e.g.: `linux/amd64`.
/// To enable this support, you may need to install the emulators in the environment running your Pulumi program.
///
/// If you are using Linux, you may be using Docker Engine or Docker Desktop for Linux, depending on how you have installed Docker. The [FAQ for Docker Desktop for Linux](https://docs.docker.com/desktop/faqs/linuxfaqs/#context) describes the differences and how to select which Docker context is in use.
///
/// * For local development using Docker Desktop, this is enabled by default.
/// * For systems using Docker Engine, install the QEMU binaries and register them with using the docker image from [github.com/tonistiigi/binfmt](https://github.com/tonistiigi/binfmt):
///
///   ```shell
///   docker run --privileged --rm tonistiigi/binfmt --install all
///   ```
/// * In a GitHub Actions workflow, the [docker/setup-qemu-action](https://github.com/docker/setup-qemu-action) can be used instead by adding this step to your workflow file. Example workflow usage:
///
///   ```yaml
///   name: Pulumi
///   on:
///     push:
///       branches:
///         - master
///   jobs:
///     up:
///       name: Preview
///       runs-on: ubuntu-latest
///       steps:
///           # This is the step added:
///         - name: Set up QEMU
///           uses: docker/setup-qemu-action@v2
///           # The ordinary pulumi/actions workflow:
///         - uses: actions/checkout@v3
///         - uses: pulumi/actions@v4
///           with:
///             command: preview
///             stack-name: org-name/stack-name
///           env:
///             PULUMI_ACCESS_TOKEN: ${{ secrets.PULUMI_ACCESS_TOKEN }}
///   ```
///
///
/// ## Example Usage
/// ### A Docker image build
///
/// ```yaml
/// config: {}
/// description: A Docker image build
/// name: image-yaml
/// outputs:
///     imageName: ${demo-image.imageName}
/// resources:
///     demo-image:
///         options:
///             version: v4.4.0
///         properties:
///             build:
///                 context: .
///                 dockerfile: Dockerfile
///                 platform: linux/amd64
///             imageName: username/image:tag1
///             skipPush: true
///         type: docker:Image
/// runtime: yaml
/// variables: {}
/// ```
/// ### A Docker image build and push
///
/// ```yaml
/// config: {}
/// description: A Docker image build and push
/// name: image-push-yaml
/// outputs:
///     imageName: ${demo-push-image.imageName}
///     repoDigest: ${demo-push-image.repoDigest}
/// resources:
///     demo-push-image:
///         options:
///             version: v4.4.0
///         properties:
///             build:
///                 context: .
///                 dockerfile: Dockerfile
///             imageName: docker.io/username/push-image:tag1
///         type: docker:Image
/// runtime: yaml
/// variables: {}
/// ```
/// ### Docker image build using caching with AWS Elastic Container Registry
///
/// ```yaml
/// config: {}
/// description: Docker image build using caching with AWS Elastic Container Registry
/// name: image-caching-yaml
/// outputs:
///     imageName: ${my-app-image.imageName}
/// resources:
///     ecr-repository:
///         properties:
///             name: docker-repository
///         type: aws:ecr:Repository
///     my-app-image:
///         options:
///             version: v4.1.2
///         properties:
///             build:
///                 args:
///                     BUILDKIT_INLINE_CACHE: "1"
///                 cacheFrom:
///                     images:
///                         - ${ecr-repository.repositoryUrl}:latest
///                 context: app/
///                 dockerfile: Dockerfile
///             imageName: ${ecr-repository.repositoryUrl}:latest
///             registry:
///                 password:
///                     fn::secret: ${authToken.password}
///                 server: ${ecr-repository.repositoryUrl}
///         type: docker:Image
/// runtime: yaml
/// variables:
///     authToken:
///         fn::aws:ecr:getAuthorizationToken:
///             registryId: ${ecr-repository.registryId}
/// ```
pub mod image {
    include!("resources/image.rs");
}
/// <!-- Bug: Type and Name are switched -->
/// `docker.Network` provides a docker network resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let privateNetwork = network::create(
///         "privateNetwork",
///         NetworkArgs::builder().build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ### Example
///
/// Assuming you created a `network` as follows
///
/// ```shell
/// docker network create foo
/// ````
///
/// prints the long ID
///
/// ```text
/// 87b57a9b91ecab2db2a6dbf38df74c67d7c7108cbe479d6576574ec2cd8c2d73
/// ```
///
/// you provide the definition for the resource as follows
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foo = network::create("foo", NetworkArgs::builder().name("foo").build_struct());
/// }
/// ```
///
/// then the import command is as follows
///
/// ```sh
/// $ pulumi import docker:index/network:Network foo 87b57a9b91ecab2db2a6dbf38df74c67d7c7108cbe479d6576574ec2cd8c2d73
/// ```
///
pub mod network {
    include!("resources/network.rs");
}
/// <!-- Bug: Type and Name are switched -->
/// Manages the lifecycle of a Docker plugin.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   sample-volume-plugin:
///     type: docker:Plugin
///     properties:
///       alias: sample-volume-plugin
///       enableTimeout: 60
///       enabled: false
///       envs:
///         - DEBUG=1
///       forceDestroy: true
///       forceDisable: true
///       grantAllPermissions: true
/// ```
///
/// ## Import
///
/// #!/bin/bash
///
/// ```sh
/// $ pulumi import docker:index/plugin:Plugin sample-volume-plugin "$(docker plugin inspect -f {{.ID}} tiborvass/sample-volume-plugin:latest)"
/// ```
///
pub mod plugin {
    include!("resources/plugin.rs");
}
/// <!-- Bug: Type and Name are switched -->
/// Manages the lifecycle of docker image in a registry. You can upload images to a registry (= `docker push`) and also delete them again
///
/// ## Example Usage
///
/// Build an image with the `docker.RemoteImage` resource and then push it to a registry:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let helloworld = registry_image::create(
///         "helloworld",
///         RegistryImageArgs::builder().keep_remotely(true).build_struct(),
///     );
///     let image = remote_image::create(
///         "image",
///         RemoteImageArgs::builder()
///             .build(
///                 RemoteImageBuild::builder()
///                     .context("${path.cwd}/absolutePathToContextFolder")
///                     .build_struct(),
///             )
///             .name("registry.com/somename:1.0")
///             .build_struct(),
///     );
/// }
/// ```
pub mod registry_image {
    include!("resources/registry_image.rs");
}
/// <!-- Bug: Type and Name are switched -->
/// Pulls a Docker image to a given Docker host from a Docker Registry.
///  This resource will *not* pull new layers of the image automatically unless used in conjunction with docker.RegistryImage data source to update the `pull_triggers` field.
///
/// ## Example Usage
///
/// ### Basic
///
/// Finds and downloads the latest `ubuntu:precise` image but does not check
/// for further updates of the image
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let ubuntu = remote_image::create(
///         "ubuntu",
///         RemoteImageArgs::builder().name("ubuntu:precise").build_struct(),
///     );
/// }
/// ```
///
/// ### Dynamic updates
///
/// To be able to update an image dynamically when the `sha256` sum changes,
/// you need to use it in combination with `docker.RegistryImage` as follows:
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let ubuntuRegistryImage = get_registry_image::invoke(
///         GetRegistryImageArgs::builder().name("ubuntu:precise").build_struct(),
///     );
///     let ubuntuRemoteImage = remote_image::create(
///         "ubuntuRemoteImage",
///         RemoteImageArgs::builder()
///             .name("${ubuntuRegistryImage.name}")
///             .pull_triggers(vec!["${ubuntuRegistryImage.sha256Digest}",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Build
///
/// You can also use the resource to build an image.
/// In this case the image "zoo" and "zoo:develop" are built.
///
/// ```yaml
/// resources:
///   zoo:
///     type: docker:RemoteImage
///     properties:
///       name: zoo
///       build:
///         context: .
///         tags:
///           - zoo:develop
///         buildArg:
///           foo: zoo
///         label:
///           author: zoo
/// ```
///
/// You can use the `triggers` argument to specify when the image should be rebuild. This is for example helpful when you want to rebuild the docker image whenever the source code changes.
///
pub mod remote_image {
    include!("resources/remote_image.rs");
}
///
///
/// ## Import
///
/// ```sh
/// # Docker secret cannot be imported as the secret data, once set, is never exposed again.
/// ```
pub mod secret {
    include!("resources/secret.rs");
}
/// <!-- Bug: Type and Name are switched -->
/// This resource manages the lifecycle of a Docker service. By default, the creation, update and delete of services are detached.
/// With the Converge Config Name of the service
/// - `task_spec` (Block List, Min: 1, Max: 1) User modifiable task configuration (see below for nested schema)
///
/// ## Import
///
/// ### Example
///
/// Assuming you created a `service` as follows
///
/// ```shell
/// docker service create --name foo -p 8080:80 nginx
/// ```
///
/// prints this ID
///
/// ```text
/// 4pcphbxkfn2rffhbhe6czytgi
/// ```
///
/// you provide the definition for the resource as follows
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foo = service::create(
///         "foo",
///         ServiceArgs::builder()
///             .endpoint_spec(
///                 ServiceEndpointSpec::builder()
///                     .ports(
///                         vec![
///                             ServiceEndpointSpecPort::builder().publishedPort(8080)
///                             .targetPort(80).build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .task_spec(
///                 ServiceTaskSpec::builder()
///                     .containerSpec(
///                         ServiceTaskSpecContainerSpec::builder()
///                             .image("nginx")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// then the import command is as follows
///
/// ```sh
/// $ pulumi import docker:index/service:Service foo 4pcphbxkfn2rffhbhe6czytgi
/// ```
///
pub mod service {
    include!("resources/service.rs");
}
///
///
/// ## Import
///
/// ### Example
///
/// Assuming you created a `config` as follows
///
/// ```shell
/// printf '{"a":"b"}' | docker config create foo -
/// ```
///
/// prints the id
///
/// ```text
/// 08c26c477474478d971139f750984775a7f019dbe8a2e7f09d66a187c009e66d
/// ```
///
/// you provide the definition for the resource as follows
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foo = service_config::create(
///         "foo",
///         ServiceConfigArgs::builder()
///             .data("base64encode(\"{\\\"a\\\": \\\"b\\\"}\")")
///             .build_struct(),
///     );
/// }
/// ```
///
/// then the import command is as follows
///
/// ```sh
/// $ pulumi import docker:index/serviceConfig:ServiceConfig foo 08c26c477474478d971139f750984775a7f019dbe8a2e7f09d66a187c009e66d
/// ```
///
pub mod service_config {
    include!("resources/service_config.rs");
}
/// Creates a docker tag. It has the exact same functionality as the `docker tag` command. Deleting the resource will neither delete the source nor target images. The source image must exist on the machine running the docker daemon.
pub mod tag {
    include!("resources/tag.rs");
}
/// <!-- Bug: Type and Name are switched -->
/// Creates and destroys a volume in Docker. This can be used alongside docker.Container to prepare volumes that can be shared across containers.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let sharedVolume = volume::create(
///         "sharedVolume",
///         VolumeArgs::builder().build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ### Example
///
/// Assuming you created a `volume` as follows
///
/// #!/bin/bash
///
/// docker volume create
///
/// prints the long ID
///
/// 524b0457aa2a87dd2b75c74c3e4e53f406974249e63ab3ed9bf21e5644f9dc7d
///
/// you provide the definition for the resource as follows
///
/// terraform
///
/// resource "docker_volume" "foo" {
///
///   name = "524b0457aa2a87dd2b75c74c3e4e53f406974249e63ab3ed9bf21e5644f9dc7d"
///
/// }
///
/// then the import command is as follows
///
/// #!/bin/bash
///
/// ```sh
/// $ pulumi import docker:index/volume:Volume foo 524b0457aa2a87dd2b75c74c3e4e53f406974249e63ab3ed9bf21e5644f9dc7d
/// ```
///
pub mod volume {
    include!("resources/volume.rs");
}
pub mod functions {
    /// `docker.getLogs` provides logs from specific container
    pub mod get_logs {
        include!("functions/get_logs.rs");
    }
    /// `docker.Network` provides details about a specific Docker Network.
    ///
    /// ## Example Usage
    ///
    /// ```ignore
    /// use pulumi_wasm_rust::Output;
    /// use pulumi_wasm_rust::{add_export, pulumi_main};
    /// #[pulumi_main]
    /// fn test_main() -> Result<(), Error> {
    ///     let main = get_network::invoke(
    ///         GetNetworkArgs::builder().name("main").build_struct(),
    ///     );
    /// }
    /// ```
    pub mod get_network {
        include!("functions/get_network.rs");
    }
    /// Reads the local Docker plugin. The plugin must be installed locally.
    ///
    /// ## Example Usage
    ///
    /// ### With alias
    /// ```yaml
    /// variables:
    ///   byAlias:
    ///     fn::docker:getPlugin:
    ///       alias: "sample-volume-plugin:latest"
    /// ```
    ///
    /// ### With ID
    /// ```yaml
    /// variables:
    ///   byId:
    ///     fn::docker:getPlugin:
    ///       id: "e9a9db917b3bfd6706b5d3a66d4bceb9f"
    /// ```
    pub mod get_plugin {
        include!("functions/get_plugin.rs");
    }
    /// Reads the image metadata from a Docker Registry. Used in conjunction with the docker.RemoteImage resource to keep an image up to date on the latest available version of the tag.
    ///
    /// ## Example Usage
    ///
    /// ```ignore
    /// use pulumi_wasm_rust::Output;
    /// use pulumi_wasm_rust::{add_export, pulumi_main};
    /// #[pulumi_main]
    /// fn test_main() -> Result<(), Error> {
    ///     let ubuntuRegistryImage = get_registry_image::invoke(
    ///         GetRegistryImageArgs::builder().name("ubuntu:precise").build_struct(),
    ///     );
    ///     let ubuntuRemoteImage = remote_image::create(
    ///         "ubuntuRemoteImage",
    ///         RemoteImageArgs::builder()
    ///             .name("${ubuntuRegistryImage.name}")
    ///             .pull_triggers(vec!["${ubuntuRegistryImage.sha256Digest}",])
    ///             .build_struct(),
    ///     );
    /// }
    /// ```
    pub mod get_registry_image {
        include!("functions/get_registry_image.rs");
    }
    /// `docker.RemoteImage` provides details about a specific Docker Image which needs to be present on the Docker Host
    ///
    /// ## Example Usage
    ///
    /// ```ignore
    /// use pulumi_wasm_rust::Output;
    /// use pulumi_wasm_rust::{add_export, pulumi_main};
    /// #[pulumi_main]
    /// fn test_main() -> Result<(), Error> {
    ///     let digest = get_remote_image::invoke(
    ///         GetRemoteImageArgs::builder()
    ///             .name(
    ///                 "nginx@sha256:36b74457bccb56fbf8b05f79c85569501b721d4db813b684391d63e02287c0b2",
    ///             )
    ///             .build_struct(),
    ///     );
    ///     let latest = get_remote_image::invoke(
    ///         GetRemoteImageArgs::builder().name("nginx").build_struct(),
    ///     );
    ///     let specific = get_remote_image::invoke(
    ///         GetRemoteImageArgs::builder().name("nginx:1.17.6").build_struct(),
    ///     );
    ///     let tagAndDigest = get_remote_image::invoke(
    ///         GetRemoteImageArgs::builder()
    ///             .name(
    ///                 "nginx:1.19.1@sha256:36b74457bccb56fbf8b05f79c85569501b721d4db813b684391d63e02287c0b2",
    ///             )
    ///             .build_struct(),
    ///     );
    /// }
    /// ```
    pub mod get_remote_image {
        include!("functions/get_remote_image.rs");
    }
}
pub mod types {
    pub mod config {
        include!("types/config/registry_auth.rs");
    }
    include!("types/builder_version.rs");
    include!("types/cache_from.rs");
    include!("types/container_capabilities.rs");
    include!("types/container_device.rs");
    include!("types/container_healthcheck.rs");
    include!("types/container_host.rs");
    include!("types/container_label.rs");
    include!("types/container_mount.rs");
    include!("types/container_mount_bind_options.rs");
    include!("types/container_mount_tmpfs_options.rs");
    include!("types/container_mount_volume_options.rs");
    include!("types/container_mount_volume_options_label.rs");
    include!("types/container_network_data.rs");
    include!("types/container_networks_advanced.rs");
    include!("types/container_port.rs");
    include!("types/container_ulimit.rs");
    include!("types/container_upload.rs");
    include!("types/container_volume.rs");
    include!("types/docker_build.rs");
    include!("types/network_ipam_config.rs");
    include!("types/network_label.rs");
    include!("types/plugin_grant_permission.rs");
    include!("types/provider_registry_auth.rs");
    include!("types/registry.rs");
    include!("types/remote_image_build.rs");
    include!("types/remote_image_build_auth_config.rs");
    include!("types/remote_image_build_ulimit.rs");
    include!("types/secret_label.rs");
    include!("types/service_auth.rs");
    include!("types/service_converge_config.rs");
    include!("types/service_endpoint_spec.rs");
    include!("types/service_endpoint_spec_port.rs");
    include!("types/service_label.rs");
    include!("types/service_mode.rs");
    include!("types/service_mode_replicated.rs");
    include!("types/service_rollback_config.rs");
    include!("types/service_task_spec.rs");
    include!("types/service_task_spec_container_spec.rs");
    include!("types/service_task_spec_container_spec_config.rs");
    include!("types/service_task_spec_container_spec_dns_config.rs");
    include!("types/service_task_spec_container_spec_healthcheck.rs");
    include!("types/service_task_spec_container_spec_host.rs");
    include!("types/service_task_spec_container_spec_label.rs");
    include!("types/service_task_spec_container_spec_mount.rs");
    include!("types/service_task_spec_container_spec_mount_bind_options.rs");
    include!("types/service_task_spec_container_spec_mount_tmpfs_options.rs");
    include!("types/service_task_spec_container_spec_mount_volume_options.rs");
    include!("types/service_task_spec_container_spec_mount_volume_options_label.rs");
    include!("types/service_task_spec_container_spec_privileges.rs");
    include!("types/service_task_spec_container_spec_privileges_credential_spec.rs");
    include!("types/service_task_spec_container_spec_privileges_se_linux_context.rs");
    include!("types/service_task_spec_container_spec_secret.rs");
    include!("types/service_task_spec_log_driver.rs");
    include!("types/service_task_spec_networks_advanced.rs");
    include!("types/service_task_spec_placement.rs");
    include!("types/service_task_spec_placement_platform.rs");
    include!("types/service_task_spec_resources.rs");
    include!("types/service_task_spec_resources_limits.rs");
    include!("types/service_task_spec_resources_reservation.rs");
    include!("types/service_task_spec_resources_reservation_generic_resources.rs");
    include!("types/service_task_spec_restart_policy.rs");
    include!("types/service_update_config.rs");
    include!("types/volume_label.rs");
    include!("types/get_network_ipam_config.rs");
}
#[doc(hidden)]
pub mod constants {}
mod bindings {
    wit_bindgen::generate!(
        { inline :
        r"package component:pulumi-wasm@0.0.0-DEV;

world world-docker {
    import output-interface;
}

interface output-interface {

    resource output {
        constructor(value: string);
        map: func(function-name: string) -> output;
    }
    combine: func(outputs: list<borrow<output>>) -> output;
}


interface register-interface {
    use output-interface.{output};

    record object-field {
        name: string,
        value: borrow<output>
    }

    record result-field {
        name: string
    }

    record register-resource-result-field {
        name: string,
        output: output
    }

    record register-resource-request {
        %type: string,
        name: string,
        object: list<object-field>,
        results: list<result-field>
    }

    record register-resource-result {
        fields: list<register-resource-result-field>
    }

    register: func(request: register-resource-request) -> register-resource-result;

    record resource-invoke-result-field {
        name: string,
        output: output
    }

    record resource-invoke-request {
        token: string,
        object: list<object-field>,
        results: list<result-field>
    }

    record resource-invoke-result {
        fields: list<resource-invoke-result-field>
    }

    invoke: func(request: resource-invoke-request) -> resource-invoke-result;
}",
        with : { "component:pulumi-wasm/output-interface@0.0.0-DEV" :
        pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface } }
    );
}
