/// <!-- Bug: Type and Name are switched -->
/// Manages the lifecycle of a Docker container.
///
/// ## Example Usage
///
/// <!--Start PulumiCodeChooser -->
/// ### Typescript
/// ```typescript
/// import * as pulumi from "@pulumi/pulumi";
/// import * as docker from "@pulumi/docker";
///
/// // Find the latest Ubuntu precise image.
/// const ubuntuRemoteImage = new docker.RemoteImage("ubuntuRemoteImage", {name: "ubuntu:precise"});
/// // Start a container
/// const ubuntuContainer = new docker.Container("ubuntuContainer", {image: ubuntuRemoteImage.imageId});
/// ```
/// ### Python
/// ```python
/// import pulumi
/// import pulumi_docker as docker
///
/// # Find the latest Ubuntu precise image.
/// ubuntu_remote_image = docker.RemoteImage("ubuntuRemoteImage", name="ubuntu:precise")
/// # Start a container
/// ubuntu_container = docker.Container("ubuntuContainer", image=ubuntu_remote_image.image_id)
/// ```
/// ### C#
/// ```csharp
/// using System.Collections.Generic;
/// using System.Linq;
/// using Pulumi;
/// using Docker = Pulumi.Docker;
///
/// return await Deployment.RunAsync(() =>
/// {
///     // Find the latest Ubuntu precise image.
///     var ubuntuRemoteImage = new Docker.RemoteImage("ubuntuRemoteImage", new()
///     {
///         Name = "ubuntu:precise",
///     });
///
///     // Start a container
///     var ubuntuContainer = new Docker.Container("ubuntuContainer", new()
///     {
///         Image = ubuntuRemoteImage.ImageId,
///     });
///
/// });
/// ```
/// ### Go
/// ```go
/// package main
///
/// import (
/// 	"github.com/pulumi/pulumi-docker/sdk/v4/go/docker"
/// 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
/// )
///
/// func main() {
/// 	pulumi.Run(func(ctx *pulumi.Context) error {
/// 		// Find the latest Ubuntu precise image.
/// 		ubuntuRemoteImage, err := docker.NewRemoteImage(ctx, "ubuntuRemoteImage", &docker.RemoteImageArgs{
/// 			Name: pulumi.String("ubuntu:precise"),
/// 		})
/// 		if err != nil {
/// 			return err
/// 		}
/// 		// Start a container
/// 		_, err = docker.NewContainer(ctx, "ubuntuContainer", &docker.ContainerArgs{
/// 			Image: ubuntuRemoteImage.ImageId,
/// 		})
/// 		if err != nil {
/// 			return err
/// 		}
/// 		return nil
/// 	})
/// }
/// ```
/// ### Java
/// ```java
/// package generated_program;
///
/// import com.pulumi.Context;
/// import com.pulumi.Pulumi;
/// import com.pulumi.core.Output;
/// import com.pulumi.docker.RemoteImage;
/// import com.pulumi.docker.RemoteImageArgs;
/// import com.pulumi.docker.Container;
/// import com.pulumi.docker.ContainerArgs;
/// import java.util.List;
/// import java.util.ArrayList;
/// import java.util.Map;
/// import java.io.File;
/// import java.nio.file.Files;
/// import java.nio.file.Paths;
///
/// public class App {
///     public static void main(String[] args) {
///         Pulumi.run(App::stack);
///     }
///
///     public static void stack(Context ctx) {
///         var ubuntuRemoteImage = new RemoteImage("ubuntuRemoteImage", RemoteImageArgs.builder()        
///             .name("ubuntu:precise")
///             .build());
///
///         var ubuntuContainer = new Container("ubuntuContainer", ContainerArgs.builder()        
///             .image(ubuntuRemoteImage.imageId())
///             .build());
///
///     }
/// }
/// ```
/// ### YAML
/// ```yaml
/// resources:
///   # Start a container
///   ubuntuContainer:
///     type: docker:Container
///     properties:
///       image: ${ubuntuRemoteImage.imageId}
///   # Find the latest Ubuntu precise image.
///   ubuntuRemoteImage:
///     type: docker:RemoteImage
///     properties:
///       name: ubuntu:precise
/// ```
/// <!--End PulumiCodeChooser -->
///
/// ## Import
///
/// ### Example
///
/// Assuming you created a `container` as follows
///
/// #!/bin/bash
///
/// docker run --name foo -p8080:80 -d nginx
///
/// prints the container ID
///
/// 9a550c0f0163d39d77222d3efd58701b625d47676c25c686c95b5b92d1cba6fd
///
/// you provide the definition for the resource as follows
///
/// terraform
///
/// resource "docker_container" "foo" {
///
///   name  = "foo"
///
///   image = "nginx"
///
///   ports {
///
///     internal = "80"
///
///     external = "8080"
///
///   }
///
/// }
///
/// then the import command is as follows
///
/// #!/bin/bash
///
/// ```sh
/// $ pulumi import docker:index/container:Container foo 9a550c0f0163d39d77222d3efd58701b625d47676c25c686c95b5b92d1cba6fd
/// ```
///
pub mod container {

    pub struct ContainerArgs {
        /// If `true` attach to the container after its creation and waits the end of its execution. Defaults to `false`.
        pub attach: pulumi_wasm_rust::Output<Option<bool>>,
        /// Add or drop certrain linux capabilities.
        pub capabilities: pulumi_wasm_rust::Output<Option<crate::types::ContainerCapabilities>>,
        /// Cgroup namespace mode to use for the container. Possible values are: `private`, `host`.
        pub cgroupns_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The command to use to start the container. For example, to run `/usr/bin/myprogram -f baz.conf` set the command to be `["/usr/bin/myprogram","-f","baz.con"]`.
        pub command: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The total number of milliseconds to wait for the container to reach status 'running'
        pub container_read_refresh_timeout_milliseconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// A comma-separated list or hyphen-separated range of CPUs a container can use, e.g. `0-1`.
        pub cpu_set: pulumi_wasm_rust::Output<Option<String>>,
        /// CPU shares (relative weight) for the container.
        pub cpu_shares: pulumi_wasm_rust::Output<Option<i32>>,
        /// If defined will attempt to stop the container before destroying. Container will be destroyed after `n` seconds or on successful stop.
        pub destroy_grace_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// Bind devices to the container.
        pub devices: pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerDevice>>>,
        /// DNS servers to use.
        pub dns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// DNS options used by the DNS provider(s), see `resolv.conf` documentation for valid list of options.
        pub dns_opts: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// DNS search domains that are used when bare unqualified hostnames are used inside of the container.
        pub dns_searches: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Domain name of the container.
        pub domainname: pulumi_wasm_rust::Output<Option<String>>,
        /// The command to use as the Entrypoint for the container. The Entrypoint allows you to configure a container to run as an executable. For example, to run `/usr/bin/myprogram` when starting a container, set the entrypoint to be `"/usr/bin/myprogra"]`.
        pub entrypoints: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Environment variables to set in the form of `KEY=VALUE`, e.g. `DEBUG=0`
        pub envs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// GPU devices to add to the container. Currently, only the value `all` is supported. Passing any other value will result in unexpected behavior.
        pub gpus: pulumi_wasm_rust::Output<Option<String>>,
        /// Additional groups for the container user
        pub group_adds: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A test to perform to check that the container is healthy
        pub healthcheck: pulumi_wasm_rust::Output<Option<crate::types::ContainerHealthcheck>>,
        /// Hostname of the container.
        pub hostname: pulumi_wasm_rust::Output<Option<String>>,
        /// Hostname to add
        pub hosts: pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerHost>>>,
        /// The ID of the image to back this container. The easiest way to get this value is to use the `docker.RemoteImage` resource as is shown in the example.
        pub image: pulumi_wasm_rust::Output<String>,
        /// Configured whether an init process should be injected for this container. If unset this will default to the `dockerd` defaults.
        pub init: pulumi_wasm_rust::Output<Option<bool>>,
        /// IPC sharing mode for the container. Possible values are: `none`, `private`, `shareable`, `container:<name|id>` or `host`.
        pub ipc_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// User-defined key/value metadata.
        pub labels: pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerLabel>>>,
        /// The logging driver to use for the container.
        pub log_driver: pulumi_wasm_rust::Output<Option<String>>,
        /// Key/value pairs to use as options for the logging driver.
        pub log_opts: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        /// Save the container logs (`attach` must be enabled). Defaults to `false`.
        pub logs: pulumi_wasm_rust::Output<Option<bool>>,
        /// The maximum amount of times to an attempt a restart when `restart` is set to 'on-failure'.
        pub max_retry_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// The memory limit for the container in MBs.
        pub memory: pulumi_wasm_rust::Output<Option<i32>>,
        /// The total memory limit (memory + swap) for the container in MBs. This setting may compute to `-1` after `pulumi up` if the target host doesn't support memory swap, when that is the case docker will use a soft limitation.
        pub memory_swap: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specification for mounts to be added to containers created as part of the service.
        pub mounts: pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerMount>>>,
        /// If `true`, then the Docker container will be kept running. If `false`, then as long as the container exists, Terraform
        /// assumes it is successful. Defaults to `true`.
        pub must_run: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name or id of the network to use. You can use `name` or `id` attribute from a `docker.Network` resource.
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Network mode of the container.
        pub network_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The networks the container is attached to
        pub networks_advanced:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerNetworksAdvanced>>>,
        /// he PID (Process) Namespace mode for the container. Either `container:<name|id>` or `host`.
        pub pid_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// Publish a container's port(s) to the host.
        pub ports: pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerPort>>>,
        /// If `true`, the container runs in privileged mode.
        pub privileged: pulumi_wasm_rust::Output<Option<bool>>,
        /// Publish all ports of the container.
        pub publish_all_ports: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether the mount should be read-only.
        pub read_only: pulumi_wasm_rust::Output<Option<bool>>,
        /// If `true`, it will remove anonymous volumes associated with the container. Defaults to `true`.
        pub remove_volumes: pulumi_wasm_rust::Output<Option<bool>>,
        /// The restart policy for the container. Must be one of 'no', 'on-failure', 'always', 'unless-stopped'. Defaults to `no`.
        pub restart: pulumi_wasm_rust::Output<Option<String>>,
        /// If `true`, then the container will be automatically removed when it exits. Defaults to `false`.
        pub rm: pulumi_wasm_rust::Output<Option<bool>>,
        /// Runtime to use for the container.
        pub runtime: pulumi_wasm_rust::Output<Option<String>>,
        /// List of string values to customize labels for MLS systems, such as SELinux. See https://docs.docker.com/engine/reference/run/#security-configuration.
        pub security_opts: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Size of `/dev/shm` in MBs.
        pub shm_size: pulumi_wasm_rust::Output<Option<i32>>,
        /// If `true`, then the Docker container will be started after creation. If `false`, then the container is only created. Defaults to `true`.
        pub start: pulumi_wasm_rust::Output<Option<bool>>,
        /// If `true`, keep STDIN open even if not attached (`docker run -i`). Defaults to `false`.
        pub stdin_open: pulumi_wasm_rust::Output<Option<bool>>,
        /// Signal to stop a container (default `SIGTERM`).
        pub stop_signal: pulumi_wasm_rust::Output<Option<String>>,
        /// Timeout (in seconds) to stop a container.
        pub stop_timeout: pulumi_wasm_rust::Output<Option<i32>>,
        /// Key/value pairs for the storage driver options, e.g. `size`: `120G`
        pub storage_opts:
            pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        /// A map of kernel parameters (sysctls) to set in the container.
        pub sysctls: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        /// A map of container directories which should be replaced by `tmpfs mounts`, and their corresponding mount options.
        pub tmpfs: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        /// If `true`, allocate a pseudo-tty (`docker run -t`). Defaults to `false`.
        pub tty: pulumi_wasm_rust::Output<Option<bool>>,
        /// Ulimit options to add.
        pub ulimits: pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerUlimit>>>,
        /// Specifies files to upload to the container before starting it. Only one of `content` or `content_base64` can be set and at least one of them has to be set.
        pub uploads: pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerUpload>>>,
        /// User used for run the first process. Format is `user` or `user:group` which user and group can be passed literraly or by name.
        pub user: pulumi_wasm_rust::Output<Option<String>>,
        /// Sets the usernamespace mode for the container when usernamespace remapping option is enabled.
        pub userns_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// Spec for mounting volumes in the container.
        pub volumes: pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerVolume>>>,
        /// If `true`, then the Docker container is waited for being healthy state after creation. If `false`, then the container health state is not checked. Defaults to `false`.
        pub wait: pulumi_wasm_rust::Output<Option<bool>>,
        /// The timeout in seconds to wait the container to be healthy after creation. Defaults to `60`.
        pub wait_timeout: pulumi_wasm_rust::Output<Option<i32>>,
        /// The working directory for commands to run in.
        pub working_dir: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct ContainerResult {
        /// If `true` attach to the container after its creation and waits the end of its execution. Defaults to `false`.
        pub attach: pulumi_wasm_rust::Output<Option<bool>>,
        /// The network bridge of the container as read from its NetworkSettings.
        pub bridge: pulumi_wasm_rust::Output<String>,
        /// Add or drop certrain linux capabilities.
        pub capabilities: pulumi_wasm_rust::Output<Option<crate::types::ContainerCapabilities>>,
        /// Cgroup namespace mode to use for the container. Possible values are: `private`, `host`.
        pub cgroupns_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The command to use to start the container. For example, to run `/usr/bin/myprogram -f baz.conf` set the command to be `["/usr/bin/myprogram","-f","baz.con"]`.
        pub command: pulumi_wasm_rust::Output<Vec<String>>,
        /// The logs of the container if its execution is done (`attach` must be disabled).
        pub container_logs: pulumi_wasm_rust::Output<Option<String>>,
        /// The total number of milliseconds to wait for the container to reach status 'running'
        pub container_read_refresh_timeout_milliseconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// A comma-separated list or hyphen-separated range of CPUs a container can use, e.g. `0-1`.
        pub cpu_set: pulumi_wasm_rust::Output<Option<String>>,
        /// CPU shares (relative weight) for the container.
        pub cpu_shares: pulumi_wasm_rust::Output<Option<i32>>,
        /// If defined will attempt to stop the container before destroying. Container will be destroyed after `n` seconds or on successful stop.
        pub destroy_grace_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// Bind devices to the container.
        pub devices: pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerDevice>>>,
        /// DNS servers to use.
        pub dns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// DNS options used by the DNS provider(s), see `resolv.conf` documentation for valid list of options.
        pub dns_opts: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// DNS search domains that are used when bare unqualified hostnames are used inside of the container.
        pub dns_searches: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Domain name of the container.
        pub domainname: pulumi_wasm_rust::Output<Option<String>>,
        /// The command to use as the Entrypoint for the container. The Entrypoint allows you to configure a container to run as an executable. For example, to run `/usr/bin/myprogram` when starting a container, set the entrypoint to be `"/usr/bin/myprogra"]`.
        pub entrypoints: pulumi_wasm_rust::Output<Vec<String>>,
        /// Environment variables to set in the form of `KEY=VALUE`, e.g. `DEBUG=0`
        pub envs: pulumi_wasm_rust::Output<Vec<String>>,
        /// The exit code of the container if its execution is done (`must_run` must be disabled).
        pub exit_code: pulumi_wasm_rust::Output<i32>,
        /// GPU devices to add to the container. Currently, only the value `all` is supported. Passing any other value will result in unexpected behavior.
        pub gpus: pulumi_wasm_rust::Output<Option<String>>,
        /// Additional groups for the container user
        pub group_adds: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A test to perform to check that the container is healthy
        pub healthcheck: pulumi_wasm_rust::Output<Option<crate::types::ContainerHealthcheck>>,
        /// Hostname of the container.
        pub hostname: pulumi_wasm_rust::Output<String>,
        /// Hostname to add
        pub hosts: pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerHost>>>,
        /// The ID of the image to back this container. The easiest way to get this value is to use the `docker.RemoteImage` resource as is shown in the example.
        pub image: pulumi_wasm_rust::Output<String>,
        /// Configured whether an init process should be injected for this container. If unset this will default to the `dockerd` defaults.
        pub init: pulumi_wasm_rust::Output<bool>,
        /// IPC sharing mode for the container. Possible values are: `none`, `private`, `shareable`, `container:<name|id>` or `host`.
        pub ipc_mode: pulumi_wasm_rust::Output<String>,
        /// User-defined key/value metadata.
        pub labels: pulumi_wasm_rust::Output<Vec<crate::types::ContainerLabel>>,
        /// The logging driver to use for the container.
        pub log_driver: pulumi_wasm_rust::Output<String>,
        /// Key/value pairs to use as options for the logging driver.
        pub log_opts: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        /// Save the container logs (`attach` must be enabled). Defaults to `false`.
        pub logs: pulumi_wasm_rust::Output<Option<bool>>,
        /// The maximum amount of times to an attempt a restart when `restart` is set to 'on-failure'.
        pub max_retry_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// The memory limit for the container in MBs.
        pub memory: pulumi_wasm_rust::Output<Option<i32>>,
        /// The total memory limit (memory + swap) for the container in MBs. This setting may compute to `-1` after `pulumi up` if the target host doesn't support memory swap, when that is the case docker will use a soft limitation.
        pub memory_swap: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specification for mounts to be added to containers created as part of the service.
        pub mounts: pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerMount>>>,
        /// If `true`, then the Docker container will be kept running. If `false`, then as long as the container exists, Terraform
        /// assumes it is successful. Defaults to `true`.
        pub must_run: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name or id of the network to use. You can use `name` or `id` attribute from a `docker.Network` resource.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The data of the networks the container is connected to.
        pub network_datas: pulumi_wasm_rust::Output<Vec<crate::types::ContainerNetworkData>>,
        /// Network mode of the container.
        pub network_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The networks the container is attached to
        pub networks_advanced:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerNetworksAdvanced>>>,
        /// he PID (Process) Namespace mode for the container. Either `container:<name|id>` or `host`.
        pub pid_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// Publish a container's port(s) to the host.
        pub ports: pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerPort>>>,
        /// If `true`, the container runs in privileged mode.
        pub privileged: pulumi_wasm_rust::Output<Option<bool>>,
        /// Publish all ports of the container.
        pub publish_all_ports: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether the mount should be read-only.
        pub read_only: pulumi_wasm_rust::Output<Option<bool>>,
        /// If `true`, it will remove anonymous volumes associated with the container. Defaults to `true`.
        pub remove_volumes: pulumi_wasm_rust::Output<Option<bool>>,
        /// The restart policy for the container. Must be one of 'no', 'on-failure', 'always', 'unless-stopped'. Defaults to `no`.
        pub restart: pulumi_wasm_rust::Output<Option<String>>,
        /// If `true`, then the container will be automatically removed when it exits. Defaults to `false`.
        pub rm: pulumi_wasm_rust::Output<Option<bool>>,
        /// Runtime to use for the container.
        pub runtime: pulumi_wasm_rust::Output<String>,
        /// List of string values to customize labels for MLS systems, such as SELinux. See https://docs.docker.com/engine/reference/run/#security-configuration.
        pub security_opts: pulumi_wasm_rust::Output<Vec<String>>,
        /// Size of `/dev/shm` in MBs.
        pub shm_size: pulumi_wasm_rust::Output<i32>,
        /// If `true`, then the Docker container will be started after creation. If `false`, then the container is only created. Defaults to `true`.
        pub start: pulumi_wasm_rust::Output<Option<bool>>,
        /// If `true`, keep STDIN open even if not attached (`docker run -i`). Defaults to `false`.
        pub stdin_open: pulumi_wasm_rust::Output<Option<bool>>,
        /// Signal to stop a container (default `SIGTERM`).
        pub stop_signal: pulumi_wasm_rust::Output<String>,
        /// Timeout (in seconds) to stop a container.
        pub stop_timeout: pulumi_wasm_rust::Output<i32>,
        /// Key/value pairs for the storage driver options, e.g. `size`: `120G`
        pub storage_opts:
            pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        /// A map of kernel parameters (sysctls) to set in the container.
        pub sysctls: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        /// A map of container directories which should be replaced by `tmpfs mounts`, and their corresponding mount options.
        pub tmpfs: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        /// If `true`, allocate a pseudo-tty (`docker run -t`). Defaults to `false`.
        pub tty: pulumi_wasm_rust::Output<Option<bool>>,
        /// Ulimit options to add.
        pub ulimits: pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerUlimit>>>,
        /// Specifies files to upload to the container before starting it. Only one of `content` or `content_base64` can be set and at least one of them has to be set.
        pub uploads: pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerUpload>>>,
        /// User used for run the first process. Format is `user` or `user:group` which user and group can be passed literraly or by name.
        pub user: pulumi_wasm_rust::Output<Option<String>>,
        /// Sets the usernamespace mode for the container when usernamespace remapping option is enabled.
        pub userns_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// Spec for mounting volumes in the container.
        pub volumes: pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerVolume>>>,
        /// If `true`, then the Docker container is waited for being healthy state after creation. If `false`, then the container health state is not checked. Defaults to `false`.
        pub wait: pulumi_wasm_rust::Output<Option<bool>>,
        /// The timeout in seconds to wait the container to be healthy after creation. Defaults to `60`.
        pub wait_timeout: pulumi_wasm_rust::Output<Option<i32>>,
        /// The working directory for commands to run in.
        pub working_dir: pulumi_wasm_rust::Output<Option<String>>,
    }

    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    pub fn container(name: &str, args: ContainerArgs) -> ContainerResult {
        let result = crate::bindings::pulumi::docker::container::invoke(
            name,
            &crate::bindings::pulumi::docker::container::Args {
                attach: args.attach.get_inner(),
                capabilities: args.capabilities.get_inner(),
                cgroupns_mode: args.cgroupns_mode.get_inner(),
                command: args.command.get_inner(),
                container_read_refresh_timeout_milliseconds: args
                    .container_read_refresh_timeout_milliseconds
                    .get_inner(),
                cpu_set: args.cpu_set.get_inner(),
                cpu_shares: args.cpu_shares.get_inner(),
                destroy_grace_seconds: args.destroy_grace_seconds.get_inner(),
                devices: args.devices.get_inner(),
                dns: args.dns.get_inner(),
                dns_opts: args.dns_opts.get_inner(),
                dns_searches: args.dns_searches.get_inner(),
                domainname: args.domainname.get_inner(),
                entrypoints: args.entrypoints.get_inner(),
                envs: args.envs.get_inner(),
                gpus: args.gpus.get_inner(),
                group_adds: args.group_adds.get_inner(),
                healthcheck: args.healthcheck.get_inner(),
                hostname: args.hostname.get_inner(),
                hosts: args.hosts.get_inner(),
                image: args.image.get_inner(),
                init: args.init.get_inner(),
                ipc_mode: args.ipc_mode.get_inner(),
                labels: args.labels.get_inner(),
                log_driver: args.log_driver.get_inner(),
                log_opts: args.log_opts.get_inner(),
                logs: args.logs.get_inner(),
                max_retry_count: args.max_retry_count.get_inner(),
                memory: args.memory.get_inner(),
                memory_swap: args.memory_swap.get_inner(),
                mounts: args.mounts.get_inner(),
                must_run: args.must_run.get_inner(),
                name: args.name.get_inner(),
                network_mode: args.network_mode.get_inner(),
                networks_advanced: args.networks_advanced.get_inner(),
                pid_mode: args.pid_mode.get_inner(),
                ports: args.ports.get_inner(),
                privileged: args.privileged.get_inner(),
                publish_all_ports: args.publish_all_ports.get_inner(),
                read_only: args.read_only.get_inner(),
                remove_volumes: args.remove_volumes.get_inner(),
                restart: args.restart.get_inner(),
                rm: args.rm.get_inner(),
                runtime: args.runtime.get_inner(),
                security_opts: args.security_opts.get_inner(),
                shm_size: args.shm_size.get_inner(),
                start: args.start.get_inner(),
                stdin_open: args.stdin_open.get_inner(),
                stop_signal: args.stop_signal.get_inner(),
                stop_timeout: args.stop_timeout.get_inner(),
                storage_opts: args.storage_opts.get_inner(),
                sysctls: args.sysctls.get_inner(),
                tmpfs: args.tmpfs.get_inner(),
                tty: args.tty.get_inner(),
                ulimits: args.ulimits.get_inner(),
                uploads: args.uploads.get_inner(),
                user: args.user.get_inner(),
                userns_mode: args.userns_mode.get_inner(),
                volumes: args.volumes.get_inner(),
                wait: args.wait.get_inner(),
                wait_timeout: args.wait_timeout.get_inner(),
                working_dir: args.working_dir.get_inner(),
            },
        );

        ContainerResult {
            attach: crate::into_domain(result.attach),
            bridge: crate::into_domain(result.bridge),
            capabilities: crate::into_domain(result.capabilities),
            cgroupns_mode: crate::into_domain(result.cgroupns_mode),
            command: crate::into_domain(result.command),
            container_logs: crate::into_domain(result.container_logs),
            container_read_refresh_timeout_milliseconds: crate::into_domain(
                result.container_read_refresh_timeout_milliseconds,
            ),
            cpu_set: crate::into_domain(result.cpu_set),
            cpu_shares: crate::into_domain(result.cpu_shares),
            destroy_grace_seconds: crate::into_domain(result.destroy_grace_seconds),
            devices: crate::into_domain(result.devices),
            dns: crate::into_domain(result.dns),
            dns_opts: crate::into_domain(result.dns_opts),
            dns_searches: crate::into_domain(result.dns_searches),
            domainname: crate::into_domain(result.domainname),
            entrypoints: crate::into_domain(result.entrypoints),
            envs: crate::into_domain(result.envs),
            exit_code: crate::into_domain(result.exit_code),
            gpus: crate::into_domain(result.gpus),
            group_adds: crate::into_domain(result.group_adds),
            healthcheck: crate::into_domain(result.healthcheck),
            hostname: crate::into_domain(result.hostname),
            hosts: crate::into_domain(result.hosts),
            image: crate::into_domain(result.image),
            init: crate::into_domain(result.init),
            ipc_mode: crate::into_domain(result.ipc_mode),
            labels: crate::into_domain(result.labels),
            log_driver: crate::into_domain(result.log_driver),
            log_opts: crate::into_domain(result.log_opts),
            logs: crate::into_domain(result.logs),
            max_retry_count: crate::into_domain(result.max_retry_count),
            memory: crate::into_domain(result.memory),
            memory_swap: crate::into_domain(result.memory_swap),
            mounts: crate::into_domain(result.mounts),
            must_run: crate::into_domain(result.must_run),
            name: crate::into_domain(result.name),
            network_datas: crate::into_domain(result.network_datas),
            network_mode: crate::into_domain(result.network_mode),
            networks_advanced: crate::into_domain(result.networks_advanced),
            pid_mode: crate::into_domain(result.pid_mode),
            ports: crate::into_domain(result.ports),
            privileged: crate::into_domain(result.privileged),
            publish_all_ports: crate::into_domain(result.publish_all_ports),
            read_only: crate::into_domain(result.read_only),
            remove_volumes: crate::into_domain(result.remove_volumes),
            restart: crate::into_domain(result.restart),
            rm: crate::into_domain(result.rm),
            runtime: crate::into_domain(result.runtime),
            security_opts: crate::into_domain(result.security_opts),
            shm_size: crate::into_domain(result.shm_size),
            start: crate::into_domain(result.start),
            stdin_open: crate::into_domain(result.stdin_open),
            stop_signal: crate::into_domain(result.stop_signal),
            stop_timeout: crate::into_domain(result.stop_timeout),
            storage_opts: crate::into_domain(result.storage_opts),
            sysctls: crate::into_domain(result.sysctls),
            tmpfs: crate::into_domain(result.tmpfs),
            tty: crate::into_domain(result.tty),
            ulimits: crate::into_domain(result.ulimits),
            uploads: crate::into_domain(result.uploads),
            user: crate::into_domain(result.user),
            userns_mode: crate::into_domain(result.userns_mode),
            volumes: crate::into_domain(result.volumes),
            wait: crate::into_domain(result.wait),
            wait_timeout: crate::into_domain(result.wait_timeout),
            working_dir: crate::into_domain(result.working_dir),
        }
    }
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
/// ### Typescript
/// ```typescript
/// import * as pulumi from "@pulumi/pulumi";
/// import * as docker from "@pulumi/docker";
///
/// const demoImage = new docker.Image("demo-image", {
///     build: {
///         context: ".",
///         dockerfile: "Dockerfile",
///         platform: "linux/amd64",
///     },
///     imageName: "username/image:tag1",
///     skipPush: true,
/// });
/// export const imageName = demoImage.imageName;
/// ```
/// ### Python
/// ```python
/// import pulumi
/// import pulumi_docker as docker
///
/// demo_image = docker.Image("demo-image",
///     build=docker.DockerBuildArgs(
///         context=".",
///         dockerfile="Dockerfile",
///         platform="linux/amd64",
///     ),
///     image_name="username/image:tag1",
///     skip_push=True)
/// pulumi.export("imageName", demo_image.image_name)
/// ```
/// ### C#
/// ```csharp
/// using System.Collections.Generic;
/// using System.Linq;
/// using Pulumi;
/// using Docker = Pulumi.Docker;
///
/// return await Deployment.RunAsync(() =>
/// {
///     var demoImage = new Docker.Image("demo-image", new()
///     {
///         Build = new Docker.Inputs.DockerBuildArgs
///         {
///             Context = ".",
///             Dockerfile = "Dockerfile",
///             Platform = "linux/amd64",
///         },
///         ImageName = "username/image:tag1",
///         SkipPush = true,
///     });
///
///     return new Dictionary<string, object?>
///     {
///         ["imageName"] = demoImage.ImageName,
///     };
/// });
///
/// ```
/// ### Go
/// ```go
/// package main
///
/// import (
/// 	"github.com/pulumi/pulumi-docker/sdk/v4/go/docker"
/// 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
/// )
///
/// func main() {
/// 	pulumi.Run(func(ctx *pulumi.Context) error {
/// 		demoImage, err := docker.NewImage(ctx, "demo-image", &docker.ImageArgs{
/// 			Build: &docker.DockerBuildArgs{
/// 				Context:    pulumi.String("."),
/// 				Dockerfile: pulumi.String("Dockerfile"),
/// 				Platform:   pulumi.String("linux/amd64"),
/// 			},
/// 			ImageName: pulumi.String("username/image:tag1"),
/// 			SkipPush:  pulumi.Bool(true),
/// 		})
/// 		if err != nil {
/// 			return err
/// 		}
/// 		ctx.Export("imageName", demoImage.ImageName)
/// 		return nil
/// 	})
/// }
/// ```
/// ### YAML
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
/// ### Java
/// ```java
/// package generated_program;
///
/// import com.pulumi.Context;
/// import com.pulumi.Pulumi;
/// import com.pulumi.core.Output;
/// import com.pulumi.docker.Image;
/// import com.pulumi.docker.ImageArgs;
/// import com.pulumi.docker.inputs.DockerBuildArgs;
/// import java.util.List;
/// import java.util.ArrayList;
/// import java.util.Map;
/// import java.io.File;
/// import java.nio.file.Files;
/// import java.nio.file.Paths;
///
/// public class App {
///     public static void main(String[] args) {
///         Pulumi.run(App::stack);
///     }
///
///     public static void stack(Context ctx) {
///         var demoImage = new Image("demoImage", ImageArgs.builder()        
///             .build(DockerBuildArgs.builder()
///                 .context(".")
///                 .dockerfile("Dockerfile")
///                 .platform("linux/amd64")
///                 .build())
///             .imageName("username/image:tag1")
///             .skipPush(true)
///             .build());
///
///         ctx.export("imageName", demoImage.imageName());
///     }
/// }
/// ```
/// ### A Docker image build and push
///
/// ### Typescript
/// ```typescript
/// import * as pulumi from "@pulumi/pulumi";
/// import * as docker from "@pulumi/docker";
///
/// const demoPushImage = new docker.Image("demo-push-image", {
///     build: {
///         context: ".",
///         dockerfile: "Dockerfile",
///     },
///     imageName: "docker.io/username/push-image:tag1",
/// });
/// export const imageName = demoPushImage.imageName;
/// export const repoDigest = demoPushImage.repoDigest;
/// ```
/// ### Python
/// ```python
/// import pulumi
/// import pulumi_docker as docker
///
/// demo_push_image = docker.Image("demo-push-image",
///     build=docker.DockerBuildArgs(
///         context=".",
///         dockerfile="Dockerfile",
///     ),
///     image_name="docker.io/username/push-image:tag1")
/// pulumi.export("imageName", demo_push_image.image_name)
/// pulumi.export("repoDigest", demo_push_image.repo_digest)
/// ```
/// ### C#
/// ```csharp
/// using System.Collections.Generic;
/// using System.Linq;
/// using Pulumi;
/// using Docker = Pulumi.Docker;
///
/// return await Deployment.RunAsync(() =>
/// {
///     var demoPushImage = new Docker.Image("demo-push-image", new()
///     {
///         Build = new Docker.Inputs.DockerBuildArgs
///         {
///             Context = ".",
///             Dockerfile = "Dockerfile",
///         },
///         ImageName = "docker.io/username/push-image:tag1",
///     });
///
///     return new Dictionary<string, object?>
///     {
///         ["imageName"] = demoPushImage.ImageName,
///         ["repoDigest"] = demoPushImage.RepoDigest,
///     };
/// });
///
/// ```
/// ### Go
/// ```go
/// package main
///
/// import (
/// 	"github.com/pulumi/pulumi-docker/sdk/v4/go/docker"
/// 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
/// )
///
/// func main() {
/// 	pulumi.Run(func(ctx *pulumi.Context) error {
/// 		demoPushImage, err := docker.NewImage(ctx, "demo-push-image", &docker.ImageArgs{
/// 			Build: &docker.DockerBuildArgs{
/// 				Context:    pulumi.String("."),
/// 				Dockerfile: pulumi.String("Dockerfile"),
/// 			},
/// 			ImageName: pulumi.String("docker.io/username/push-image:tag1"),
/// 		})
/// 		if err != nil {
/// 			return err
/// 		}
/// 		ctx.Export("imageName", demoPushImage.ImageName)
/// 		ctx.Export("repoDigest", demoPushImage.RepoDigest)
/// 		return nil
/// 	})
/// }
/// ```
/// ### YAML
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
/// ### Java
/// ```java
/// package generated_program;
///
/// import com.pulumi.Context;
/// import com.pulumi.Pulumi;
/// import com.pulumi.core.Output;
/// import com.pulumi.docker.Image;
/// import com.pulumi.docker.ImageArgs;
/// import com.pulumi.docker.inputs.DockerBuildArgs;
/// import java.util.List;
/// import java.util.ArrayList;
/// import java.util.Map;
/// import java.io.File;
/// import java.nio.file.Files;
/// import java.nio.file.Paths;
///
/// public class App {
///     public static void main(String[] args) {
///         Pulumi.run(App::stack);
///     }
///
///     public static void stack(Context ctx) {
///         var demoPushImage = new Image("demoPushImage", ImageArgs.builder()        
///             .build(DockerBuildArgs.builder()
///                 .context(".")
///                 .dockerfile("Dockerfile")
///                 .build())
///             .imageName("docker.io/username/push-image:tag1")
///             .build());
///
///         ctx.export("imageName", demoPushImage.imageName());
///         ctx.export("repoDigest", demoPushImage.repoDigest());
///     }
/// }
/// ```
/// ### Docker image build using caching with AWS Elastic Container Registry
///
/// ### Typescript
/// ```typescript
/// import * as pulumi from "@pulumi/pulumi";
/// import * as aws from "@pulumi/aws";
/// import * as docker from "@pulumi/docker";
///
/// const ecrRepository = new aws.ecr.Repository("ecr-repository", {name: "docker-repository"});
/// const authToken = aws.ecr.getAuthorizationTokenOutput({
///     registryId: ecrRepository.registryId,
/// });
/// const myAppImage = new docker.Image("my-app-image", {
///     build: {
///         args: {
///             BUILDKIT_INLINE_CACHE: "1",
///         },
///         cacheFrom: {
///             images: [pulumi.interpolate`${ecrRepository.repositoryUrl}:latest`],
///         },
///         context: "app/",
///         dockerfile: "Dockerfile",
///     },
///     imageName: pulumi.interpolate`${ecrRepository.repositoryUrl}:latest`,
///     registry: {
///         password: pulumi.secret(authToken.apply(authToken => authToken.password)),
///         server: ecrRepository.repositoryUrl,
///     },
/// });
/// export const imageName = myAppImage.imageName;
/// ```
/// ### Python
/// ```python
/// import pulumi
/// import pulumi_aws as aws
/// import pulumi_docker as docker
///
/// ecr_repository = aws.ecr.Repository("ecr-repository", name="docker-repository")
/// auth_token = aws.ecr.get_authorization_token_output(registry_id=ecr_repository.registry_id)
/// my_app_image = docker.Image("my-app-image",
///     build=docker.DockerBuildArgs(
///         args={
///             "BUILDKIT_INLINE_CACHE": "1",
///         },
///         cache_from=docker.CacheFromArgs(
///             images=[ecr_repository.repository_url.apply(lambda repository_url: f"{repository_url}:latest")],
///         ),
///         context="app/",
///         dockerfile="Dockerfile",
///     ),
///     image_name=ecr_repository.repository_url.apply(lambda repository_url: f"{repository_url}:latest"),
///     registry=docker.RegistryArgs(
///         password=pulumi.Output.secret(auth_token.password),
///         server=ecr_repository.repository_url,
///     ))
/// pulumi.export("imageName", my_app_image.image_name)
/// ```
/// ### C#
/// ```csharp
/// using System.Collections.Generic;
/// using System.Linq;
/// using Pulumi;
/// using Aws = Pulumi.Aws;
/// using Docker = Pulumi.Docker;
///
/// return await Deployment.RunAsync(() =>
/// {
///     var ecrRepository = new Aws.Ecr.Repository("ecr-repository", new()
///     {
///         Name = "docker-repository",
///     });
///
///     var authToken = Aws.Ecr.GetAuthorizationToken.Invoke(new()
///     {
///         RegistryId = ecrRepository.RegistryId,
///     });
///
///     var myAppImage = new Docker.Image("my-app-image", new()
///     {
///         Build = new Docker.Inputs.DockerBuildArgs
///         {
///             Args =
///             {
///                 { "BUILDKIT_INLINE_CACHE", "1" },
///             },
///             CacheFrom = new Docker.Inputs.CacheFromArgs
///             {
///                 Images = new[]
///                 {
///                     ecrRepository.RepositoryUrl.Apply(repositoryUrl => $"{repositoryUrl}:latest"),
///                 },
///             },
///             Context = "app/",
///             Dockerfile = "Dockerfile",
///         },
///         ImageName = ecrRepository.RepositoryUrl.Apply(repositoryUrl => $"{repositoryUrl}:latest"),
///         Registry = new Docker.Inputs.RegistryArgs
///         {
///             Password = Output.CreateSecret(authToken.Apply(getAuthorizationTokenResult => getAuthorizationTokenResult.Password)),
///             Server = ecrRepository.RepositoryUrl,
///         },
///     });
///
///     return new Dictionary<string, object?>
///     {
///         ["imageName"] = myAppImage.ImageName,
///     };
/// });
///
/// ```
/// ### Go
/// ```go
/// package main
///
/// import (
/// 	"fmt"
///
/// 	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ecr"
/// 	"github.com/pulumi/pulumi-docker/sdk/v4/go/docker"
/// 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
/// )
///
/// func main() {
/// 	pulumi.Run(func(ctx *pulumi.Context) error {
/// 		ecrRepository, err := ecr.NewRepository(ctx, "ecr-repository", &ecr.RepositoryArgs{
/// 			Name: pulumi.String("docker-repository"),
/// 		})
/// 		if err != nil {
/// 			return err
/// 		}
/// 		authToken := ecr.GetAuthorizationTokenOutput(ctx, ecr.GetAuthorizationTokenOutputArgs{
/// 			RegistryId: ecrRepository.RegistryId,
/// 		}, nil)
/// 		myAppImage, err := docker.NewImage(ctx, "my-app-image", &docker.ImageArgs{
/// 			Build: &docker.DockerBuildArgs{
/// 				Args: pulumi.StringMap{
/// 					"BUILDKIT_INLINE_CACHE": pulumi.String("1"),
/// 				},
/// 				CacheFrom: &docker.CacheFromArgs{
/// 					Images: pulumi.StringArray{
/// 						ecrRepository.RepositoryUrl.ApplyT(func(repositoryUrl string) (string, error) {
/// 							return fmt.Sprintf("%v:latest", repositoryUrl), nil
/// 						}).(pulumi.StringOutput),
/// 					},
/// 				},
/// 				Context:    pulumi.String("app/"),
/// 				Dockerfile: pulumi.String("Dockerfile"),
/// 			},
/// 			ImageName: ecrRepository.RepositoryUrl.ApplyT(func(repositoryUrl string) (string, error) {
/// 				return fmt.Sprintf("%v:latest", repositoryUrl), nil
/// 			}).(pulumi.StringOutput),
/// 			Registry: &docker.RegistryArgs{
/// 				Password: pulumi.ToSecret(authToken.ApplyT(func(authToken ecr.GetAuthorizationTokenResult) (*string, error) {
/// 					return &authToken.Password, nil
/// 				}).(pulumi.StringPtrOutput)).(pulumi.StringOutput),
/// 				Server: ecrRepository.RepositoryUrl,
/// 			},
/// 		})
/// 		if err != nil {
/// 			return err
/// 		}
/// 		ctx.Export("imageName", myAppImage.ImageName)
/// 		return nil
/// 	})
/// }
/// ```
/// ### YAML
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
/// ### Java
/// ```java
/// package generated_program;
///
/// import com.pulumi.Context;
/// import com.pulumi.Pulumi;
/// import com.pulumi.core.Output;
/// import com.pulumi.aws.ecr.Repository;
/// import com.pulumi.aws.ecr.RepositoryArgs;
/// import com.pulumi.aws.ecr.EcrFunctions;
/// import com.pulumi.aws.ecr.inputs.GetAuthorizationTokenArgs;
/// import com.pulumi.docker.Image;
/// import com.pulumi.docker.ImageArgs;
/// import com.pulumi.docker.inputs.DockerBuildArgs;
/// import com.pulumi.docker.inputs.CacheFromArgs;
/// import com.pulumi.docker.inputs.RegistryArgs;
/// import java.util.List;
/// import java.util.ArrayList;
/// import java.util.Map;
/// import java.io.File;
/// import java.nio.file.Files;
/// import java.nio.file.Paths;
///
/// public class App {
///     public static void main(String[] args) {
///         Pulumi.run(App::stack);
///     }
///
///     public static void stack(Context ctx) {
///         var ecrRepository = new Repository("ecrRepository", RepositoryArgs.builder()        
///             .name("docker-repository")
///             .build());
///
///         final var authToken = EcrFunctions.getAuthorizationToken(GetAuthorizationTokenArgs.builder()
///             .registryId(ecrRepository.registryId())
///             .build());
///
///         var myAppImage = new Image("myAppImage", ImageArgs.builder()        
///             .build(DockerBuildArgs.builder()
///                 .args(Map.of("BUILDKIT_INLINE_CACHE", "1"))
///                 .cacheFrom(CacheFromArgs.builder()
///                     .images(ecrRepository.repositoryUrl().applyValue(repositoryUrl -> String.format("%s:latest", repositoryUrl)))
///                     .build())
///                 .context("app/")
///                 .dockerfile("Dockerfile")
///                 .build())
///             .imageName(ecrRepository.repositoryUrl().applyValue(repositoryUrl -> String.format("%s:latest", repositoryUrl)))
///             .registry(RegistryArgs.builder()
///                 .password(Output.ofSecret(authToken.applyValue(getAuthorizationTokenResult -> getAuthorizationTokenResult).applyValue(authToken -> authToken.applyValue(getAuthorizationTokenResult -> getAuthorizationTokenResult.password()))))
///                 .server(ecrRepository.repositoryUrl())
///                 .build())
///             .build());
///
///         ctx.export("imageName", myAppImage.imageName());
///     }
/// }
/// ```
pub mod image {

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
    pub fn image(name: &str, args: ImageArgs) -> ImageResult {
        let result = crate::bindings::pulumi::docker::image::invoke(
            name,
            &crate::bindings::pulumi::docker::image::Args {
                build: args.build.get_inner(),
                build_on_preview: args.build_on_preview.get_inner(),
                image_name: args.image_name.get_inner(),
                registry: args.registry.get_inner(),
                skip_push: args.skip_push.get_inner(),
            },
        );

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
}

/// <!-- Bug: Type and Name are switched -->
/// `docker.Network` provides a docker network resource.
///
/// ## Example Usage
///
/// <!--Start PulumiCodeChooser -->
/// ### Typescript
/// ```typescript
/// import * as pulumi from "@pulumi/pulumi";
/// import * as docker from "@pulumi/docker";
///
/// const privateNetwork = new docker.Network("privateNetwork", {});
/// ```
/// ### Python
/// ```python
/// import pulumi
/// import pulumi_docker as docker
///
/// private_network = docker.Network("privateNetwork")
/// ```
/// ### C#
/// ```csharp
/// using System.Collections.Generic;
/// using System.Linq;
/// using Pulumi;
/// using Docker = Pulumi.Docker;
///
/// return await Deployment.RunAsync(() =>
/// {
///     var privateNetwork = new Docker.Network("privateNetwork");
///
/// });
/// ```
/// ### Go
/// ```go
/// package main
///
/// import (
/// 	"github.com/pulumi/pulumi-docker/sdk/v4/go/docker"
/// 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
/// )
///
/// func main() {
/// 	pulumi.Run(func(ctx *pulumi.Context) error {
/// 		_, err := docker.NewNetwork(ctx, "privateNetwork", nil)
/// 		if err != nil {
/// 			return err
/// 		}
/// 		return nil
/// 	})
/// }
/// ```
/// ### Java
/// ```java
/// package generated_program;
///
/// import com.pulumi.Context;
/// import com.pulumi.Pulumi;
/// import com.pulumi.core.Output;
/// import com.pulumi.docker.Network;
/// import java.util.List;
/// import java.util.ArrayList;
/// import java.util.Map;
/// import java.io.File;
/// import java.nio.file.Files;
/// import java.nio.file.Paths;
///
/// public class App {
///     public static void main(String[] args) {
///         Pulumi.run(App::stack);
///     }
///
///     public static void stack(Context ctx) {
///         var privateNetwork = new Network("privateNetwork");
///
///     }
/// }
/// ```
/// ### YAML
/// ```yaml
/// resources:
///   privateNetwork:
///     type: docker:Network
/// ```
/// <!--End PulumiCodeChooser -->
///
/// ## Import
///
/// ### Example
///
/// Assuming you created a `network` as follows
///
/// #!/bin/bash
///
/// docker network create foo
///
/// prints the long ID
///
/// 87b57a9b91ecab2db2a6dbf38df74c67d7c7108cbe479d6576574ec2cd8c2d73
///
/// you provide the definition for the resource as follows
///
/// terraform
///
/// resource "docker_network" "foo" {
///
///   name = "foo"
///
/// }
///
/// then the import command is as follows
///
/// #!/bin/bash
///
/// ```sh
/// $ pulumi import docker:index/network:Network foo 87b57a9b91ecab2db2a6dbf38df74c67d7c7108cbe479d6576574ec2cd8c2d73
/// ```
///
pub mod network {

    pub struct NetworkArgs {
        /// Enable manual container attachment to the network.
        pub attachable: pulumi_wasm_rust::Output<Option<bool>>,
        /// Requests daemon to check for networks with same name.
        pub check_duplicate: pulumi_wasm_rust::Output<Option<bool>>,
        /// The driver of the Docker network. Possible values are `bridge`, `host`, `overlay`, `macvlan`. See [network docs](https://docs.docker.com/network/#network-drivers) for more details.
        pub driver: pulumi_wasm_rust::Output<Option<String>>,
        /// Create swarm routing-mesh network. Defaults to `false`.
        pub ingress: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether the network is internal.
        pub internal: pulumi_wasm_rust::Output<Option<bool>>,
        /// The IPAM configuration options
        pub ipam_configs: pulumi_wasm_rust::Output<Option<Vec<crate::types::NetworkIpamConfig>>>,
        /// Driver used by the custom IP scheme of the network. Defaults to `default`
        pub ipam_driver: pulumi_wasm_rust::Output<Option<String>>,
        /// Provide explicit options to the IPAM driver. Valid options vary with `ipam_driver` and refer to that driver's documentation for more details.
        pub ipam_options:
            pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        /// Enable IPv6 networking. Defaults to `false`.
        pub ipv6: pulumi_wasm_rust::Output<Option<bool>>,
        /// User-defined key/value metadata
        pub labels: pulumi_wasm_rust::Output<Option<Vec<crate::types::NetworkLabel>>>,
        /// The name of the Docker network.
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Only available with bridge networks. See [bridge options docs](https://docs.docker.com/engine/reference/commandline/network_create/#bridge-driver-options) for more details.
        pub options: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    }

    pub struct NetworkResult {
        /// Enable manual container attachment to the network.
        pub attachable: pulumi_wasm_rust::Output<Option<bool>>,
        /// Requests daemon to check for networks with same name.
        pub check_duplicate: pulumi_wasm_rust::Output<Option<bool>>,
        /// The driver of the Docker network. Possible values are `bridge`, `host`, `overlay`, `macvlan`. See [network docs](https://docs.docker.com/network/#network-drivers) for more details.
        pub driver: pulumi_wasm_rust::Output<String>,
        /// Create swarm routing-mesh network. Defaults to `false`.
        pub ingress: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether the network is internal.
        pub internal: pulumi_wasm_rust::Output<bool>,
        /// The IPAM configuration options
        pub ipam_configs: pulumi_wasm_rust::Output<Vec<crate::types::NetworkIpamConfig>>,
        /// Driver used by the custom IP scheme of the network. Defaults to `default`
        pub ipam_driver: pulumi_wasm_rust::Output<Option<String>>,
        /// Provide explicit options to the IPAM driver. Valid options vary with `ipam_driver` and refer to that driver's documentation for more details.
        pub ipam_options:
            pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        /// Enable IPv6 networking. Defaults to `false`.
        pub ipv6: pulumi_wasm_rust::Output<Option<bool>>,
        /// User-defined key/value metadata
        pub labels: pulumi_wasm_rust::Output<Option<Vec<crate::types::NetworkLabel>>>,
        /// The name of the Docker network.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Only available with bridge networks. See [bridge options docs](https://docs.docker.com/engine/reference/commandline/network_create/#bridge-driver-options) for more details.
        pub options: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Scope of the network. One of `swarm`, `global`, or `local`.
        pub scope: pulumi_wasm_rust::Output<String>,
    }

    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    pub fn network(name: &str, args: NetworkArgs) -> NetworkResult {
        let result = crate::bindings::pulumi::docker::network::invoke(
            name,
            &crate::bindings::pulumi::docker::network::Args {
                attachable: args.attachable.get_inner(),
                check_duplicate: args.check_duplicate.get_inner(),
                driver: args.driver.get_inner(),
                ingress: args.ingress.get_inner(),
                internal: args.internal.get_inner(),
                ipam_configs: args.ipam_configs.get_inner(),
                ipam_driver: args.ipam_driver.get_inner(),
                ipam_options: args.ipam_options.get_inner(),
                ipv6: args.ipv6.get_inner(),
                labels: args.labels.get_inner(),
                name: args.name.get_inner(),
                options: args.options.get_inner(),
            },
        );

        NetworkResult {
            attachable: crate::into_domain(result.attachable),
            check_duplicate: crate::into_domain(result.check_duplicate),
            driver: crate::into_domain(result.driver),
            ingress: crate::into_domain(result.ingress),
            internal: crate::into_domain(result.internal),
            ipam_configs: crate::into_domain(result.ipam_configs),
            ipam_driver: crate::into_domain(result.ipam_driver),
            ipam_options: crate::into_domain(result.ipam_options),
            ipv6: crate::into_domain(result.ipv6),
            labels: crate::into_domain(result.labels),
            name: crate::into_domain(result.name),
            options: crate::into_domain(result.options),
            scope: crate::into_domain(result.scope),
        }
    }
}

/// <!-- Bug: Type and Name are switched -->
/// Manages the lifecycle of a Docker plugin.
///
/// ## Example Usage
///
/// <!--Start PulumiCodeChooser -->
/// ### Typescript
/// ```typescript
/// import * as pulumi from "@pulumi/pulumi";
/// import * as docker from "@pulumi/docker";
///
/// const sample_volume_plugin = new docker.Plugin("sample-volume-plugin", {
///     alias: "sample-volume-plugin",
///     enableTimeout: 60,
///     enabled: false,
///     envs: ["DEBUG=1"],
///     forceDestroy: true,
///     forceDisable: true,
///     grantAllPermissions: true,
/// });
/// ```
/// ### Python
/// ```python
/// import pulumi
/// import pulumi_docker as docker
///
/// sample_volume_plugin = docker.Plugin("sample-volume-plugin",
///     alias="sample-volume-plugin",
///     enable_timeout=60,
///     enabled=False,
///     envs=["DEBUG=1"],
///     force_destroy=True,
///     force_disable=True,
///     grant_all_permissions=True)
/// ```
/// ### C#
/// ```csharp
/// using System.Collections.Generic;
/// using System.Linq;
/// using Pulumi;
/// using Docker = Pulumi.Docker;
///
/// return await Deployment.RunAsync(() =>
/// {
///     var sample_volume_plugin = new Docker.Plugin("sample-volume-plugin", new()
///     {
///         Alias = "sample-volume-plugin",
///         EnableTimeout = 60,
///         Enabled = false,
///         Envs = new[]
///         {
///             "DEBUG=1",
///         },
///         ForceDestroy = true,
///         ForceDisable = true,
///         GrantAllPermissions = true,
///     });
///
/// });
/// ```
/// ### Go
/// ```go
/// package main
///
/// import (
/// 	"github.com/pulumi/pulumi-docker/sdk/v4/go/docker"
/// 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
/// )
///
/// func main() {
/// 	pulumi.Run(func(ctx *pulumi.Context) error {
/// 		_, err := docker.NewPlugin(ctx, "sample-volume-plugin", &docker.PluginArgs{
/// 			Alias:         pulumi.String("sample-volume-plugin"),
/// 			EnableTimeout: pulumi.Int(60),
/// 			Enabled:       pulumi.Bool(false),
/// 			Envs: pulumi.StringArray{
/// 				pulumi.String("DEBUG=1"),
/// 			},
/// 			ForceDestroy:        pulumi.Bool(true),
/// 			ForceDisable:        pulumi.Bool(true),
/// 			GrantAllPermissions: pulumi.Bool(true),
/// 		})
/// 		if err != nil {
/// 			return err
/// 		}
/// 		return nil
/// 	})
/// }
/// ```
/// ### Java
/// ```java
/// package generated_program;
///
/// import com.pulumi.Context;
/// import com.pulumi.Pulumi;
/// import com.pulumi.core.Output;
/// import com.pulumi.docker.Plugin;
/// import com.pulumi.docker.PluginArgs;
/// import java.util.List;
/// import java.util.ArrayList;
/// import java.util.Map;
/// import java.io.File;
/// import java.nio.file.Files;
/// import java.nio.file.Paths;
///
/// public class App {
///     public static void main(String[] args) {
///         Pulumi.run(App::stack);
///     }
///
///     public static void stack(Context ctx) {
///         var sample_volume_plugin = new Plugin("sample-volume-plugin", PluginArgs.builder()        
///             .alias("sample-volume-plugin")
///             .enableTimeout(60)
///             .enabled(false)
///             .envs("DEBUG=1")
///             .forceDestroy(true)
///             .forceDisable(true)
///             .grantAllPermissions(true)
///             .build());
///
///     }
/// }
/// ```
/// ### YAML
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
/// <!--End PulumiCodeChooser -->
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

    pub struct PluginArgs {
        /// Docker Plugin alias
        pub alias: pulumi_wasm_rust::Output<Option<String>>,
        /// HTTP client timeout to enable the plugin
        pub enable_timeout: pulumi_wasm_rust::Output<Option<i32>>,
        /// If `true` the plugin is enabled. Defaults to `true`
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The environment variables in the form of `KEY=VALUE`, e.g. `DEBUG=0`
        pub envs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// If true, then the plugin is destroyed forcibly
        pub force_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// If true, then the plugin is disabled forcibly
        pub force_disable: pulumi_wasm_rust::Output<Option<bool>>,
        /// If true, grant all permissions necessary to run the plugin
        pub grant_all_permissions: pulumi_wasm_rust::Output<Option<bool>>,
        /// Grant specific permissions only
        pub grant_permissions:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::PluginGrantPermission>>>,
        /// The name of the permission
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct PluginResult {
        /// Docker Plugin alias
        pub alias: pulumi_wasm_rust::Output<String>,
        /// HTTP client timeout to enable the plugin
        pub enable_timeout: pulumi_wasm_rust::Output<Option<i32>>,
        /// If `true` the plugin is enabled. Defaults to `true`
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The environment variables in the form of `KEY=VALUE`, e.g. `DEBUG=0`
        pub envs: pulumi_wasm_rust::Output<Vec<String>>,
        /// If true, then the plugin is destroyed forcibly
        pub force_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// If true, then the plugin is disabled forcibly
        pub force_disable: pulumi_wasm_rust::Output<Option<bool>>,
        /// If true, grant all permissions necessary to run the plugin
        pub grant_all_permissions: pulumi_wasm_rust::Output<Option<bool>>,
        /// Grant specific permissions only
        pub grant_permissions:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::PluginGrantPermission>>>,
        /// The name of the permission
        pub name: pulumi_wasm_rust::Output<String>,
        /// Docker Plugin Reference
        pub plugin_reference: pulumi_wasm_rust::Output<String>,
    }

    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    pub fn plugin(name: &str, args: PluginArgs) -> PluginResult {
        let result = crate::bindings::pulumi::docker::plugin::invoke(
            name,
            &crate::bindings::pulumi::docker::plugin::Args {
                alias: args.alias.get_inner(),
                enable_timeout: args.enable_timeout.get_inner(),
                enabled: args.enabled.get_inner(),
                envs: args.envs.get_inner(),
                force_destroy: args.force_destroy.get_inner(),
                force_disable: args.force_disable.get_inner(),
                grant_all_permissions: args.grant_all_permissions.get_inner(),
                grant_permissions: args.grant_permissions.get_inner(),
                name: args.name.get_inner(),
            },
        );

        PluginResult {
            alias: crate::into_domain(result.alias),
            enable_timeout: crate::into_domain(result.enable_timeout),
            enabled: crate::into_domain(result.enabled),
            envs: crate::into_domain(result.envs),
            force_destroy: crate::into_domain(result.force_destroy),
            force_disable: crate::into_domain(result.force_disable),
            grant_all_permissions: crate::into_domain(result.grant_all_permissions),
            grant_permissions: crate::into_domain(result.grant_permissions),
            name: crate::into_domain(result.name),
            plugin_reference: crate::into_domain(result.plugin_reference),
        }
    }
}

/// <!-- Bug: Type and Name are switched -->
/// Manages the lifecycle of docker image in a registry. You can upload images to a registry (= `docker push`) and also delete them again
///
/// ## Example Usage
///
/// Build an image with the `docker.RemoteImage` resource and then push it to a registry:
///
/// <!--Start PulumiCodeChooser -->
/// ### Typescript
/// ```typescript
/// import * as pulumi from "@pulumi/pulumi";
/// import * as docker from "@pulumi/docker";
///
/// const helloworld = new docker.RegistryImage("helloworld", {keepRemotely: true});
/// const image = new docker.RemoteImage("image", {
///     name: "registry.com/somename:1.0",
///     build: {
///         context: `${path.cwd}/absolutePathToContextFolder`,
///     },
/// });
/// ```
/// ### Python
/// ```python
/// import pulumi
/// import pulumi_docker as docker
///
/// helloworld = docker.RegistryImage("helloworld", keep_remotely=True)
/// image = docker.RemoteImage("image",
///     name="registry.com/somename:1.0",
///     build=docker.RemoteImageBuildArgs(
///         context=f"{path['cwd']}/absolutePathToContextFolder",
///     ))
/// ```
/// ### C#
/// ```csharp
/// using System.Collections.Generic;
/// using System.Linq;
/// using Pulumi;
/// using Docker = Pulumi.Docker;
///
/// return await Deployment.RunAsync(() =>
/// {
///     var helloworld = new Docker.RegistryImage("helloworld", new()
///     {
///         KeepRemotely = true,
///     });
///
///     var image = new Docker.RemoteImage("image", new()
///     {
///         Name = "registry.com/somename:1.0",
///         Build = new Docker.Inputs.RemoteImageBuildArgs
///         {
///             Context = $"{path.Cwd}/absolutePathToContextFolder",
///         },
///     });
///
/// });
/// ```
/// ### Go
/// ```go
/// package main
///
/// import (
/// 	"fmt"
///
/// 	"github.com/pulumi/pulumi-docker/sdk/v4/go/docker"
/// 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
/// )
///
/// func main() {
/// 	pulumi.Run(func(ctx *pulumi.Context) error {
/// 		_, err := docker.NewRegistryImage(ctx, "helloworld", &docker.RegistryImageArgs{
/// 			KeepRemotely: pulumi.Bool(true),
/// 		})
/// 		if err != nil {
/// 			return err
/// 		}
/// 		_, err = docker.NewRemoteImage(ctx, "image", &docker.RemoteImageArgs{
/// 			Name: pulumi.String("registry.com/somename:1.0"),
/// 			Build: &docker.RemoteImageBuildArgs{
/// 				Context: pulumi.String(fmt.Sprintf("%v/absolutePathToContextFolder", path.Cwd)),
/// 			},
/// 		})
/// 		if err != nil {
/// 			return err
/// 		}
/// 		return nil
/// 	})
/// }
/// ```
/// ### Java
/// ```java
/// package generated_program;
///
/// import com.pulumi.Context;
/// import com.pulumi.Pulumi;
/// import com.pulumi.core.Output;
/// import com.pulumi.docker.RegistryImage;
/// import com.pulumi.docker.RegistryImageArgs;
/// import com.pulumi.docker.RemoteImage;
/// import com.pulumi.docker.RemoteImageArgs;
/// import com.pulumi.docker.inputs.RemoteImageBuildArgs;
/// import java.util.List;
/// import java.util.ArrayList;
/// import java.util.Map;
/// import java.io.File;
/// import java.nio.file.Files;
/// import java.nio.file.Paths;
///
/// public class App {
///     public static void main(String[] args) {
///         Pulumi.run(App::stack);
///     }
///
///     public static void stack(Context ctx) {
///         var helloworld = new RegistryImage("helloworld", RegistryImageArgs.builder()        
///             .keepRemotely(true)
///             .build());
///
///         var image = new RemoteImage("image", RemoteImageArgs.builder()        
///             .name("registry.com/somename:1.0")
///             .build(RemoteImageBuildArgs.builder()
///                 .context(String.format("%s/absolutePathToContextFolder", path.cwd()))
///                 .build())
///             .build());
///
///     }
/// }
/// ```
/// ### YAML
/// ```yaml
/// resources:
///   helloworld:
///     type: docker:RegistryImage
///     properties:
///       keepRemotely: true
///   image:
///     type: docker:RemoteImage
///     properties:
///       name: registry.com/somename:1.0
///       build:
///         context: ${path.cwd}/absolutePathToContextFolder
/// ```
/// <!--End PulumiCodeChooser -->
pub mod registry_image {

    pub struct RegistryImageArgs {
        /// If `true`, the verification of TLS certificates of the server/registry is disabled. Defaults to `false`
        pub insecure_skip_verify: pulumi_wasm_rust::Output<Option<bool>>,
        /// If true, then the Docker image won't be deleted on destroy operation. If this is false, it will delete the image from the docker registry on destroy operation. Defaults to `false`
        pub keep_remotely: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the Docker image.
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of arbitrary strings that, when changed, will force the `docker.RegistryImage` resource to be replaced. This can be used to repush a local image
        pub triggers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    }

    pub struct RegistryImageResult {
        /// If `true`, the verification of TLS certificates of the server/registry is disabled. Defaults to `false`
        pub insecure_skip_verify: pulumi_wasm_rust::Output<Option<bool>>,
        /// If true, then the Docker image won't be deleted on destroy operation. If this is false, it will delete the image from the docker registry on destroy operation. Defaults to `false`
        pub keep_remotely: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the Docker image.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The sha256 digest of the image.
        pub sha256_digest: pulumi_wasm_rust::Output<String>,
        /// A map of arbitrary strings that, when changed, will force the `docker.RegistryImage` resource to be replaced. This can be used to repush a local image
        pub triggers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    }

    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    pub fn registry_image(name: &str, args: RegistryImageArgs) -> RegistryImageResult {
        let result = crate::bindings::pulumi::docker::registry_image::invoke(
            name,
            &crate::bindings::pulumi::docker::registry_image::Args {
                insecure_skip_verify: args.insecure_skip_verify.get_inner(),
                keep_remotely: args.keep_remotely.get_inner(),
                name: args.name.get_inner(),
                triggers: args.triggers.get_inner(),
            },
        );

        RegistryImageResult {
            insecure_skip_verify: crate::into_domain(result.insecure_skip_verify),
            keep_remotely: crate::into_domain(result.keep_remotely),
            name: crate::into_domain(result.name),
            sha256_digest: crate::into_domain(result.sha256_digest),
            triggers: crate::into_domain(result.triggers),
        }
    }
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
/// <!--Start PulumiCodeChooser -->
/// ### Typescript
/// ```typescript
/// import * as pulumi from "@pulumi/pulumi";
/// import * as docker from "@pulumi/docker";
///
/// const ubuntu = new docker.RemoteImage("ubuntu", {name: "ubuntu:precise"});
/// ```
/// ### Python
/// ```python
/// import pulumi
/// import pulumi_docker as docker
///
/// ubuntu = docker.RemoteImage("ubuntu", name="ubuntu:precise")
/// ```
/// ### C#
/// ```csharp
/// using System.Collections.Generic;
/// using System.Linq;
/// using Pulumi;
/// using Docker = Pulumi.Docker;
///
/// return await Deployment.RunAsync(() =>
/// {
///     var ubuntu = new Docker.RemoteImage("ubuntu", new()
///     {
///         Name = "ubuntu:precise",
///     });
///
/// });
/// ```
/// ### Go
/// ```go
/// package main
///
/// import (
/// 	"github.com/pulumi/pulumi-docker/sdk/v4/go/docker"
/// 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
/// )
///
/// func main() {
/// 	pulumi.Run(func(ctx *pulumi.Context) error {
/// 		_, err := docker.NewRemoteImage(ctx, "ubuntu", &docker.RemoteImageArgs{
/// 			Name: pulumi.String("ubuntu:precise"),
/// 		})
/// 		if err != nil {
/// 			return err
/// 		}
/// 		return nil
/// 	})
/// }
/// ```
/// ### Java
/// ```java
/// package generated_program;
///
/// import com.pulumi.Context;
/// import com.pulumi.Pulumi;
/// import com.pulumi.core.Output;
/// import com.pulumi.docker.RemoteImage;
/// import com.pulumi.docker.RemoteImageArgs;
/// import java.util.List;
/// import java.util.ArrayList;
/// import java.util.Map;
/// import java.io.File;
/// import java.nio.file.Files;
/// import java.nio.file.Paths;
///
/// public class App {
///     public static void main(String[] args) {
///         Pulumi.run(App::stack);
///     }
///
///     public static void stack(Context ctx) {
///         var ubuntu = new RemoteImage("ubuntu", RemoteImageArgs.builder()        
///             .name("ubuntu:precise")
///             .build());
///
///     }
/// }
/// ```
/// ### YAML
/// ```yaml
/// resources:
///   ubuntu:
///     type: docker:RemoteImage
///     properties:
///       name: ubuntu:precise
/// ```
/// <!--End PulumiCodeChooser -->
///
/// ### Dynamic updates
///
/// To be able to update an image dynamically when the `sha256` sum changes,
/// you need to use it in combination with `docker.RegistryImage` as follows:
///
/// <!--Start PulumiCodeChooser -->
/// ### Typescript
/// ```typescript
/// import * as pulumi from "@pulumi/pulumi";
/// import * as docker from "@pulumi/docker";
///
/// const ubuntuRegistryImage = docker.getRegistryImage({
///     name: "ubuntu:precise",
/// });
/// const ubuntuRemoteImage = new docker.RemoteImage("ubuntuRemoteImage", {
///     name: ubuntuRegistryImage.then(ubuntuRegistryImage => ubuntuRegistryImage.name),
///     pullTriggers: [ubuntuRegistryImage.then(ubuntuRegistryImage => ubuntuRegistryImage.sha256Digest)],
/// });
/// ```
/// ### Python
/// ```python
/// import pulumi
/// import pulumi_docker as docker
///
/// ubuntu_registry_image = docker.get_registry_image(name="ubuntu:precise")
/// ubuntu_remote_image = docker.RemoteImage("ubuntuRemoteImage",
///     name=ubuntu_registry_image.name,
///     pull_triggers=[ubuntu_registry_image.sha256_digest])
/// ```
/// ### C#
/// ```csharp
/// using System.Collections.Generic;
/// using System.Linq;
/// using Pulumi;
/// using Docker = Pulumi.Docker;
///
/// return await Deployment.RunAsync(() =>
/// {
///     var ubuntuRegistryImage = Docker.GetRegistryImage.Invoke(new()
///     {
///         Name = "ubuntu:precise",
///     });
///
///     var ubuntuRemoteImage = new Docker.RemoteImage("ubuntuRemoteImage", new()
///     {
///         Name = ubuntuRegistryImage.Apply(getRegistryImageResult => getRegistryImageResult.Name),
///         PullTriggers = new[]
///         {
///             ubuntuRegistryImage.Apply(getRegistryImageResult => getRegistryImageResult.Sha256Digest),
///         },
///     });
///
/// });
/// ```
/// ### Go
/// ```go
/// package main
///
/// import (
/// 	"github.com/pulumi/pulumi-docker/sdk/v4/go/docker"
/// 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
/// )
///
/// func main() {
/// 	pulumi.Run(func(ctx *pulumi.Context) error {
/// 		ubuntuRegistryImage, err := docker.LookupRegistryImage(ctx, &docker.LookupRegistryImageArgs{
/// 			Name: "ubuntu:precise",
/// 		}, nil)
/// 		if err != nil {
/// 			return err
/// 		}
/// 		_, err = docker.NewRemoteImage(ctx, "ubuntuRemoteImage", &docker.RemoteImageArgs{
/// 			Name: pulumi.String(ubuntuRegistryImage.Name),
/// 			PullTriggers: pulumi.StringArray{
/// 				pulumi.String(ubuntuRegistryImage.Sha256Digest),
/// 			},
/// 		})
/// 		if err != nil {
/// 			return err
/// 		}
/// 		return nil
/// 	})
/// }
/// ```
/// ### Java
/// ```java
/// package generated_program;
///
/// import com.pulumi.Context;
/// import com.pulumi.Pulumi;
/// import com.pulumi.core.Output;
/// import com.pulumi.docker.DockerFunctions;
/// import com.pulumi.docker.inputs.GetRegistryImageArgs;
/// import com.pulumi.docker.RemoteImage;
/// import com.pulumi.docker.RemoteImageArgs;
/// import java.util.List;
/// import java.util.ArrayList;
/// import java.util.Map;
/// import java.io.File;
/// import java.nio.file.Files;
/// import java.nio.file.Paths;
///
/// public class App {
///     public static void main(String[] args) {
///         Pulumi.run(App::stack);
///     }
///
///     public static void stack(Context ctx) {
///         final var ubuntuRegistryImage = DockerFunctions.getRegistryImage(GetRegistryImageArgs.builder()
///             .name("ubuntu:precise")
///             .build());
///
///         var ubuntuRemoteImage = new RemoteImage("ubuntuRemoteImage", RemoteImageArgs.builder()        
///             .name(ubuntuRegistryImage.applyValue(getRegistryImageResult -> getRegistryImageResult.name()))
///             .pullTriggers(ubuntuRegistryImage.applyValue(getRegistryImageResult -> getRegistryImageResult.sha256Digest()))
///             .build());
///
///     }
/// }
/// ```
/// ### YAML
/// ```yaml
/// resources:
///   ubuntuRemoteImage:
///     type: docker:RemoteImage
///     properties:
///       name: ${ubuntuRegistryImage.name}
///       pullTriggers:
///         - ${ubuntuRegistryImage.sha256Digest}
/// variables:
///   ubuntuRegistryImage:
///     fn::invoke:
///       Function: docker:getRegistryImage
///       Arguments:
///         name: ubuntu:precise
/// ```
/// <!--End PulumiCodeChooser -->
///
/// ### Build
///
/// You can also use the resource to build an image.
/// In this case the image "zoo" and "zoo:develop" are built.
///
/// <!--Start PulumiCodeChooser -->
/// ### Typescript
/// ```typescript
/// import * as pulumi from "@pulumi/pulumi";
/// import * as docker from "@pulumi/docker";
///
/// const zoo = new docker.RemoteImage("zoo", {
///     name: "zoo",
///     build: {
///         context: ".",
///         tags: ["zoo:develop"],
///         buildArg: {
///             foo: "zoo",
///         },
///         label: {
///             author: "zoo",
///         },
///     },
/// });
/// ```
/// ### Python
/// ```python
/// import pulumi
/// import pulumi_docker as docker
///
/// zoo = docker.RemoteImage("zoo",
///     name="zoo",
///     build=docker.RemoteImageBuildArgs(
///         context=".",
///         tags=["zoo:develop"],
///         build_arg={
///             "foo": "zoo",
///         },
///         label={
///             "author": "zoo",
///         },
///     ))
/// ```
/// ### C#
/// ```csharp
/// using System.Collections.Generic;
/// using System.Linq;
/// using Pulumi;
/// using Docker = Pulumi.Docker;
///
/// return await Deployment.RunAsync(() =>
/// {
///     var zoo = new Docker.RemoteImage("zoo", new()
///     {
///         Name = "zoo",
///         Build = new Docker.Inputs.RemoteImageBuildArgs
///         {
///             Context = ".",
///             Tags = new[]
///             {
///                 "zoo:develop",
///             },
///             BuildArg =
///             {
///                 { "foo", "zoo" },
///             },
///             Label =
///             {
///                 { "author", "zoo" },
///             },
///         },
///     });
///
/// });
/// ```
/// ### Go
/// ```go
/// package main
///
/// import (
/// 	"github.com/pulumi/pulumi-docker/sdk/v4/go/docker"
/// 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
/// )
///
/// func main() {
/// 	pulumi.Run(func(ctx *pulumi.Context) error {
/// 		_, err := docker.NewRemoteImage(ctx, "zoo", &docker.RemoteImageArgs{
/// 			Name: pulumi.String("zoo"),
/// 			Build: &docker.RemoteImageBuildArgs{
/// 				Context: pulumi.String("."),
/// 				Tags: pulumi.StringArray{
/// 					pulumi.String("zoo:develop"),
/// 				},
/// 				BuildArg: pulumi.StringMap{
/// 					"foo": pulumi.String("zoo"),
/// 				},
/// 				Label: pulumi.StringMap{
/// 					"author": pulumi.String("zoo"),
/// 				},
/// 			},
/// 		})
/// 		if err != nil {
/// 			return err
/// 		}
/// 		return nil
/// 	})
/// }
/// ```
/// ### Java
/// ```java
/// package generated_program;
///
/// import com.pulumi.Context;
/// import com.pulumi.Pulumi;
/// import com.pulumi.core.Output;
/// import com.pulumi.docker.RemoteImage;
/// import com.pulumi.docker.RemoteImageArgs;
/// import com.pulumi.docker.inputs.RemoteImageBuildArgs;
/// import java.util.List;
/// import java.util.ArrayList;
/// import java.util.Map;
/// import java.io.File;
/// import java.nio.file.Files;
/// import java.nio.file.Paths;
///
/// public class App {
///     public static void main(String[] args) {
///         Pulumi.run(App::stack);
///     }
///
///     public static void stack(Context ctx) {
///         var zoo = new RemoteImage("zoo", RemoteImageArgs.builder()        
///             .name("zoo")
///             .build(RemoteImageBuildArgs.builder()
///                 .context(".")
///                 .tags("zoo:develop")
///                 .buildArg(Map.of("foo", "zoo"))
///                 .label(Map.of("author", "zoo"))
///                 .build())
///             .build());
///
///     }
/// }
/// ```
/// ### YAML
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
/// <!--End PulumiCodeChooser -->
///
/// You can use the `triggers` argument to specify when the image should be rebuild. This is for example helpful when you want to rebuild the docker image whenever the source code changes.
///
pub mod remote_image {

    pub struct RemoteImageArgs {
        /// Configuration to build an image. Please see [docker build command reference](https://docs.docker.com/engine/reference/commandline/build/#options) too.
        pub build: pulumi_wasm_rust::Output<Option<crate::types::RemoteImageBuild>>,
        /// Always remove intermediate containers
        pub force_remove: pulumi_wasm_rust::Output<Option<bool>>,
        /// If true, then the Docker image won't be deleted on destroy operation. If this is false, it will delete the image from the docker local storage on destroy operation.
        pub keep_locally: pulumi_wasm_rust::Output<Option<bool>>,
        /// type of ulimit, e.g. `nofile`
        pub name: pulumi_wasm_rust::Output<String>,
        /// Set platform if server is multi-platform capable
        pub platform: pulumi_wasm_rust::Output<Option<String>>,
        /// List of values which cause an image pull when changed. This is used to store the image digest from the registry when using the docker*registry*image.
        pub pull_triggers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A map of arbitrary strings that, when changed, will force the `docker.RemoteImage` resource to be replaced. This can be used to rebuild an image when contents of source code folders change
        pub triggers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    }

    pub struct RemoteImageResult {
        /// Configuration to build an image. Please see [docker build command reference](https://docs.docker.com/engine/reference/commandline/build/#options) too.
        pub build: pulumi_wasm_rust::Output<Option<crate::types::RemoteImageBuild>>,
        /// Always remove intermediate containers
        pub force_remove: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the image (as seen when executing `docker inspect` on the image). Can be used to reference the image via its ID in other resources.
        pub image_id: pulumi_wasm_rust::Output<String>,
        /// If true, then the Docker image won't be deleted on destroy operation. If this is false, it will delete the image from the docker local storage on destroy operation.
        pub keep_locally: pulumi_wasm_rust::Output<Option<bool>>,
        /// type of ulimit, e.g. `nofile`
        pub name: pulumi_wasm_rust::Output<String>,
        /// Set platform if server is multi-platform capable
        pub platform: pulumi_wasm_rust::Output<Option<String>>,
        /// List of values which cause an image pull when changed. This is used to store the image digest from the registry when using the docker*registry*image.
        pub pull_triggers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The image sha256 digest in the form of `repo[:tag]@sha256:<hash>`.
        pub repo_digest: pulumi_wasm_rust::Output<String>,
        /// A map of arbitrary strings that, when changed, will force the `docker.RemoteImage` resource to be replaced. This can be used to rebuild an image when contents of source code folders change
        pub triggers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    }

    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    pub fn remote_image(name: &str, args: RemoteImageArgs) -> RemoteImageResult {
        let result = crate::bindings::pulumi::docker::remote_image::invoke(
            name,
            &crate::bindings::pulumi::docker::remote_image::Args {
                build: args.build.get_inner(),
                force_remove: args.force_remove.get_inner(),
                keep_locally: args.keep_locally.get_inner(),
                name: args.name.get_inner(),
                platform: args.platform.get_inner(),
                pull_triggers: args.pull_triggers.get_inner(),
                triggers: args.triggers.get_inner(),
            },
        );

        RemoteImageResult {
            build: crate::into_domain(result.build),
            force_remove: crate::into_domain(result.force_remove),
            image_id: crate::into_domain(result.image_id),
            keep_locally: crate::into_domain(result.keep_locally),
            name: crate::into_domain(result.name),
            platform: crate::into_domain(result.platform),
            pull_triggers: crate::into_domain(result.pull_triggers),
            repo_digest: crate::into_domain(result.repo_digest),
            triggers: crate::into_domain(result.triggers),
        }
    }
}

///
///
/// ## Import
///
/// #!/bin/bash
///
/// Docker secret cannot be imported as the secret data, once set, is never exposed again.
///
pub mod secret {

    pub struct SecretArgs {
        /// Base64-url-safe-encoded secret data
        pub data: pulumi_wasm_rust::Output<String>,
        /// User-defined key/value metadata
        pub labels: pulumi_wasm_rust::Output<Option<Vec<crate::types::SecretLabel>>>,
        /// User-defined name of the secret
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct SecretResult {
        /// Base64-url-safe-encoded secret data
        pub data: pulumi_wasm_rust::Output<String>,
        /// User-defined key/value metadata
        pub labels: pulumi_wasm_rust::Output<Option<Vec<crate::types::SecretLabel>>>,
        /// User-defined name of the secret
        pub name: pulumi_wasm_rust::Output<String>,
    }

    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    pub fn secret(name: &str, args: SecretArgs) -> SecretResult {
        let result = crate::bindings::pulumi::docker::secret::invoke(
            name,
            &crate::bindings::pulumi::docker::secret::Args {
                data: args.data.get_inner(),
                labels: args.labels.get_inner(),
                name: args.name.get_inner(),
            },
        );

        SecretResult {
            data: crate::into_domain(result.data),
            labels: crate::into_domain(result.labels),
            name: crate::into_domain(result.name),
        }
    }
}

/// <!-- Bug: Type and Name are switched -->
/// This resource manages the lifecycle of a Docker service. By default, the creation, update and delete of services are detached.
///  With the Converge Config Name of the service
/// - `task_spec` (Block List, Min: 1, Max: 1) User modifiable task configuration (see below for nested schema)
///
/// ## Import
///
/// ### Example
///
/// Assuming you created a `service` as follows
///
/// #!/bin/bash
///
/// docker service create --name foo -p 8080:80 nginx
///
/// prints th ID
///
/// 4pcphbxkfn2rffhbhe6czytgi
///
/// you provide the definition for the resource as follows
///
/// terraform
///
/// resource "docker_service" "foo" {
///
///   name = "foo"
///
///   task_spec {
///
///     container_spec {
///
///       image = "nginx"
///
///     }
///
///   }
///
///   endpoint_spec {
///
///     ports {
///
///       target_port    = "80"
///
///       published_port = "8080"
///
///     }
///
///   }
///
/// }
///
/// then the import command is as follows
///
/// #!/bin/bash
///
/// ```sh
/// $ pulumi import docker:index/service:Service foo 4pcphbxkfn2rffhbhe6czytgi
/// ```
///
pub mod service {

    pub struct ServiceArgs {
        /// Configuration for the authentication for pulling the images of the service
        pub auth: pulumi_wasm_rust::Output<Option<crate::types::ServiceAuth>>,
        /// A configuration to ensure that a service converges aka reaches the desired that of all task up and running
        pub converge_config: pulumi_wasm_rust::Output<Option<crate::types::ServiceConvergeConfig>>,
        /// Properties that can be configured to access and load balance a service
        pub endpoint_spec: pulumi_wasm_rust::Output<Option<crate::types::ServiceEndpointSpec>>,
        /// User-defined key/value metadata
        pub labels: pulumi_wasm_rust::Output<Option<Vec<crate::types::ServiceLabel>>>,
        /// Scheduling mode for the service
        pub mode: pulumi_wasm_rust::Output<Option<crate::types::ServiceMode>>,
        /// Name of the service
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specification for the rollback strategy of the service
        pub rollback_config: pulumi_wasm_rust::Output<Option<crate::types::ServiceRollbackConfig>>,
        /// User modifiable task configuration
        pub task_spec: pulumi_wasm_rust::Output<crate::types::ServiceTaskSpec>,
        /// Specification for the update strategy of the service
        pub update_config: pulumi_wasm_rust::Output<Option<crate::types::ServiceUpdateConfig>>,
    }

    pub struct ServiceResult {
        /// Configuration for the authentication for pulling the images of the service
        pub auth: pulumi_wasm_rust::Output<Option<crate::types::ServiceAuth>>,
        /// A configuration to ensure that a service converges aka reaches the desired that of all task up and running
        pub converge_config: pulumi_wasm_rust::Output<Option<crate::types::ServiceConvergeConfig>>,
        /// Properties that can be configured to access and load balance a service
        pub endpoint_spec: pulumi_wasm_rust::Output<crate::types::ServiceEndpointSpec>,
        /// User-defined key/value metadata
        pub labels: pulumi_wasm_rust::Output<Vec<crate::types::ServiceLabel>>,
        /// Scheduling mode for the service
        pub mode: pulumi_wasm_rust::Output<crate::types::ServiceMode>,
        /// Name of the service
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specification for the rollback strategy of the service
        pub rollback_config: pulumi_wasm_rust::Output<Option<crate::types::ServiceRollbackConfig>>,
        /// User modifiable task configuration
        pub task_spec: pulumi_wasm_rust::Output<crate::types::ServiceTaskSpec>,
        /// Specification for the update strategy of the service
        pub update_config: pulumi_wasm_rust::Output<Option<crate::types::ServiceUpdateConfig>>,
    }

    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    pub fn service(name: &str, args: ServiceArgs) -> ServiceResult {
        let result = crate::bindings::pulumi::docker::service::invoke(
            name,
            &crate::bindings::pulumi::docker::service::Args {
                auth: args.auth.get_inner(),
                converge_config: args.converge_config.get_inner(),
                endpoint_spec: args.endpoint_spec.get_inner(),
                labels: args.labels.get_inner(),
                mode: args.mode.get_inner(),
                name: args.name.get_inner(),
                rollback_config: args.rollback_config.get_inner(),
                task_spec: args.task_spec.get_inner(),
                update_config: args.update_config.get_inner(),
            },
        );

        ServiceResult {
            auth: crate::into_domain(result.auth),
            converge_config: crate::into_domain(result.converge_config),
            endpoint_spec: crate::into_domain(result.endpoint_spec),
            labels: crate::into_domain(result.labels),
            mode: crate::into_domain(result.mode),
            name: crate::into_domain(result.name),
            rollback_config: crate::into_domain(result.rollback_config),
            task_spec: crate::into_domain(result.task_spec),
            update_config: crate::into_domain(result.update_config),
        }
    }
}

///
///
/// ## Import
///
/// ### Example
///
/// Assuming you created a `config` as follows
///
/// #!/bin/bash
///
/// printf '{"a":"b"}' | docker config create foo -
///
/// prints the id
///
/// 08c26c477474478d971139f750984775a7f019dbe8a2e7f09d66a187c009e66d
///
/// you provide the definition for the resource as follows
///
/// terraform
///
/// resource "docker_config" "foo" {
///
///   name = "foo"
///
///   data = base64encode("{\"a\": \"b\"}")
///
/// }
///
/// then the import command is as follows
///
/// #!/bin/bash
///
/// ```sh
/// $ pulumi import docker:index/serviceConfig:ServiceConfig foo 08c26c477474478d971139f750984775a7f019dbe8a2e7f09d66a187c009e66d
/// ```
///
pub mod service_config {

    pub struct ServiceConfigArgs {
        /// Base64-url-safe-encoded config data
        pub data: pulumi_wasm_rust::Output<String>,
        /// User-defined name of the config
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct ServiceConfigResult {
        /// Base64-url-safe-encoded config data
        pub data: pulumi_wasm_rust::Output<String>,
        /// User-defined name of the config
        pub name: pulumi_wasm_rust::Output<String>,
    }

    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    pub fn service_config(name: &str, args: ServiceConfigArgs) -> ServiceConfigResult {
        let result = crate::bindings::pulumi::docker::service_config::invoke(
            name,
            &crate::bindings::pulumi::docker::service_config::Args {
                data: args.data.get_inner(),
                name: args.name.get_inner(),
            },
        );

        ServiceConfigResult {
            data: crate::into_domain(result.data),
            name: crate::into_domain(result.name),
        }
    }
}

/// Creates a docker tag. It has the exact same functionality as the `docker tag` command. Deleting the resource will neither delete the source nor target images. The source image must exist on the machine running the docker daemon.
pub mod tag {

    pub struct TagArgs {
        /// Name of the source image.
        pub source_image: pulumi_wasm_rust::Output<String>,
        /// Name of the target image.
        pub target_image: pulumi_wasm_rust::Output<String>,
    }

    pub struct TagResult {
        /// Name of the source image.
        pub source_image: pulumi_wasm_rust::Output<String>,
        /// ImageID of the source image in the format of `sha256:<<ID>>`
        pub source_image_id: pulumi_wasm_rust::Output<String>,
        /// Name of the target image.
        pub target_image: pulumi_wasm_rust::Output<String>,
    }

    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    pub fn tag(name: &str, args: TagArgs) -> TagResult {
        let result = crate::bindings::pulumi::docker::tag::invoke(
            name,
            &crate::bindings::pulumi::docker::tag::Args {
                source_image: args.source_image.get_inner(),
                target_image: args.target_image.get_inner(),
            },
        );

        TagResult {
            source_image: crate::into_domain(result.source_image),
            source_image_id: crate::into_domain(result.source_image_id),
            target_image: crate::into_domain(result.target_image),
        }
    }
}

/// <!-- Bug: Type and Name are switched -->
/// Creates and destroys a volume in Docker. This can be used alongside docker.Container to prepare volumes that can be shared across containers.
///
/// ## Example Usage
///
/// <!--Start PulumiCodeChooser -->
/// ### Typescript
/// ```typescript
/// import * as pulumi from "@pulumi/pulumi";
/// import * as docker from "@pulumi/docker";
///
/// const sharedVolume = new docker.Volume("sharedVolume", {});
/// ```
/// ### Python
/// ```python
/// import pulumi
/// import pulumi_docker as docker
///
/// shared_volume = docker.Volume("sharedVolume")
/// ```
/// ### C#
/// ```csharp
/// using System.Collections.Generic;
/// using System.Linq;
/// using Pulumi;
/// using Docker = Pulumi.Docker;
///
/// return await Deployment.RunAsync(() =>
/// {
///     var sharedVolume = new Docker.Volume("sharedVolume");
///
/// });
/// ```
/// ### Go
/// ```go
/// package main
///
/// import (
/// 	"github.com/pulumi/pulumi-docker/sdk/v4/go/docker"
/// 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
/// )
///
/// func main() {
/// 	pulumi.Run(func(ctx *pulumi.Context) error {
/// 		_, err := docker.NewVolume(ctx, "sharedVolume", nil)
/// 		if err != nil {
/// 			return err
/// 		}
/// 		return nil
/// 	})
/// }
/// ```
/// ### Java
/// ```java
/// package generated_program;
///
/// import com.pulumi.Context;
/// import com.pulumi.Pulumi;
/// import com.pulumi.core.Output;
/// import com.pulumi.docker.Volume;
/// import java.util.List;
/// import java.util.ArrayList;
/// import java.util.Map;
/// import java.io.File;
/// import java.nio.file.Files;
/// import java.nio.file.Paths;
///
/// public class App {
///     public static void main(String[] args) {
///         Pulumi.run(App::stack);
///     }
///
///     public static void stack(Context ctx) {
///         var sharedVolume = new Volume("sharedVolume");
///
///     }
/// }
/// ```
/// ### YAML
/// ```yaml
/// resources:
///   sharedVolume:
///     type: docker:Volume
/// ```
/// <!--End PulumiCodeChooser -->
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

    pub struct VolumeArgs {
        /// Driver type for the volume. Defaults to `local`.
        pub driver: pulumi_wasm_rust::Output<Option<String>>,
        /// Options specific to the driver.
        pub driver_opts:
            pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        /// User-defined key/value metadata
        pub labels: pulumi_wasm_rust::Output<Option<Vec<crate::types::VolumeLabel>>>,
        /// The name of the Docker volume (will be generated if not provided).
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct VolumeResult {
        /// Driver type for the volume. Defaults to `local`.
        pub driver: pulumi_wasm_rust::Output<String>,
        /// Options specific to the driver.
        pub driver_opts:
            pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        /// User-defined key/value metadata
        pub labels: pulumi_wasm_rust::Output<Option<Vec<crate::types::VolumeLabel>>>,
        /// The mountpoint of the volume.
        pub mountpoint: pulumi_wasm_rust::Output<String>,
        /// The name of the Docker volume (will be generated if not provided).
        pub name: pulumi_wasm_rust::Output<String>,
    }

    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    pub fn volume(name: &str, args: VolumeArgs) -> VolumeResult {
        let result = crate::bindings::pulumi::docker::volume::invoke(
            name,
            &crate::bindings::pulumi::docker::volume::Args {
                driver: args.driver.get_inner(),
                driver_opts: args.driver_opts.get_inner(),
                labels: args.labels.get_inner(),
                name: args.name.get_inner(),
            },
        );

        VolumeResult {
            driver: crate::into_domain(result.driver),
            driver_opts: crate::into_domain(result.driver_opts),
            labels: crate::into_domain(result.labels),
            mountpoint: crate::into_domain(result.mountpoint),
            name: crate::into_domain(result.name),
        }
    }
}
