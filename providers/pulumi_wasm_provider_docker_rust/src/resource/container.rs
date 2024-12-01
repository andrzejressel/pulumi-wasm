//! <!-- Bug: Type and Name are switched -->
//! Manages the lifecycle of a Docker container.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let ubuntuContainer = container::create(
//!         "ubuntuContainer",
//!         ContainerArgs::builder().image("${ubuntuRemoteImage.imageId}").build_struct(),
//!     );
//!     let ubuntuRemoteImage = remote_image::create(
//!         "ubuntuRemoteImage",
//!         RemoteImageArgs::builder().name("ubuntu:precise").build_struct(),
//!     );
//! }
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ### Example
//! 
//! Assuming you created a `container` as follows
//! 
//! #!/bin/bash
//! 
//! docker run --name foo -p8080:80 -d nginx 
//! 
//! prints the container ID 
//! 
//! 9a550c0f0163d39d77222d3efd58701b625d47676c25c686c95b5b92d1cba6fd
//! 
//! you provide the definition for the resource as follows
//! 
//! terraform
//! 
//! resource "docker_container" "foo" {
//! 
//!   name  = "foo"
//! 
//!   image = "nginx"
//! 
//!   ports {
//! 
//!     internal = "80"
//! 
//!     external = "8080"
//! 
//!   }
//! 
//! }
//! 
//! then the import command is as follows
//! 
//! #!/bin/bash
//! 
//! ```sh
//! $ pulumi import docker:index/container:Container foo 9a550c0f0163d39d77222d3efd58701b625d47676c25c686c95b5b92d1cba6fd
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct ContainerArgs {
    /// If `true` attach to the container after its creation and waits the end of its execution. Defaults to `false`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub attach: pulumi_wasm_rust::Output<Option<bool>>,
    /// Add or drop certrain linux capabilities.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub capabilities: pulumi_wasm_rust::Output<Option<crate::types::ContainerCapabilities>>,
    /// Cgroup namespace mode to use for the container. Possible values are: `private`, `host`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub cgroupns_mode: pulumi_wasm_rust::Output<Option<String>>,
    /// The command to use to start the container. For example, to run `/usr/bin/myprogram -f baz.conf` set the command to be `["/usr/bin/myprogram","-f","baz.con"]`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub command: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The total number of milliseconds to wait for the container to reach status 'running'
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub container_read_refresh_timeout_milliseconds: pulumi_wasm_rust::Output<Option<i32>>,
    /// A comma-separated list or hyphen-separated range of CPUs a container can use, e.g. `0-1`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub cpu_set: pulumi_wasm_rust::Output<Option<String>>,
    /// CPU shares (relative weight) for the container.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub cpu_shares: pulumi_wasm_rust::Output<Option<i32>>,
    /// If defined will attempt to stop the container before destroying. Container will be destroyed after `n` seconds or on successful stop.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub destroy_grace_seconds: pulumi_wasm_rust::Output<Option<i32>>,
    /// Bind devices to the container.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub devices: pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerDevice>>>,
    /// DNS servers to use.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub dns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// DNS options used by the DNS provider(s), see `resolv.conf` documentation for valid list of options.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub dns_opts: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// DNS search domains that are used when bare unqualified hostnames are used inside of the container.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub dns_searches: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// Domain name of the container.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub domainname: pulumi_wasm_rust::Output<Option<String>>,
    /// The command to use as the Entrypoint for the container. The Entrypoint allows you to configure a container to run as an executable. For example, to run `/usr/bin/myprogram` when starting a container, set the entrypoint to be `"/usr/bin/myprogra"]`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub entrypoints: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// Environment variables to set in the form of `KEY=VALUE`, e.g. `DEBUG=0`
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub envs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// GPU devices to add to the container. Currently, only the value `all` is supported. Passing any other value will result in unexpected behavior.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub gpus: pulumi_wasm_rust::Output<Option<String>>,
    /// Additional groups for the container user
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub group_adds: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// A test to perform to check that the container is healthy
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub healthcheck: pulumi_wasm_rust::Output<Option<crate::types::ContainerHealthcheck>>,
    /// Hostname of the container.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub hostname: pulumi_wasm_rust::Output<Option<String>>,
    /// Hostname to add
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub hosts: pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerHost>>>,
    /// The ID of the image to back this container. The easiest way to get this value is to use the `docker.RemoteImage` resource as is shown in the example.
    #[builder(into)]
    pub image: pulumi_wasm_rust::Output<String>,
    /// Configured whether an init process should be injected for this container. If unset this will default to the `dockerd` defaults.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub init: pulumi_wasm_rust::Output<Option<bool>>,
    /// IPC sharing mode for the container. Possible values are: `none`, `private`, `shareable`, `container:<name|id>` or `host`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub ipc_mode: pulumi_wasm_rust::Output<Option<String>>,
    /// User-defined key/value metadata.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub labels: pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerLabel>>>,
    /// The logging driver to use for the container.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub log_driver: pulumi_wasm_rust::Output<Option<String>>,
    /// Key/value pairs to use as options for the logging driver.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub log_opts: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    /// Save the container logs (`attach` must be enabled). Defaults to `false`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub logs: pulumi_wasm_rust::Output<Option<bool>>,
    /// The maximum amount of times to an attempt a restart when `restart` is set to 'on-failure'.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub max_retry_count: pulumi_wasm_rust::Output<Option<i32>>,
    /// The memory limit for the container in MBs.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub memory: pulumi_wasm_rust::Output<Option<i32>>,
    /// The total memory limit (memory + swap) for the container in MBs. This setting may compute to `-1` after `pulumi up` if the target host doesn't support memory swap, when that is the case docker will use a soft limitation.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub memory_swap: pulumi_wasm_rust::Output<Option<i32>>,
    /// Specification for mounts to be added to containers created as part of the service.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub mounts: pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerMount>>>,
    /// If `true`, then the Docker container will be kept running. If `false`, then as long as the container exists, Terraform
    /// assumes it is successful. Defaults to `true`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub must_run: pulumi_wasm_rust::Output<Option<bool>>,
    /// The name or id of the network to use. You can use `name` or `id` attribute from a `docker.Network` resource.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    /// Network mode of the container.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub network_mode: pulumi_wasm_rust::Output<Option<String>>,
    /// The networks the container is attached to
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub networks_advanced: pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerNetworksAdvanced>>>,
    /// he PID (Process) Namespace mode for the container. Either `container:<name|id>` or `host`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub pid_mode: pulumi_wasm_rust::Output<Option<String>>,
    /// Publish a container's port(s) to the host.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub ports: pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerPort>>>,
    /// If `true`, the container runs in privileged mode.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub privileged: pulumi_wasm_rust::Output<Option<bool>>,
    /// Publish all ports of the container.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub publish_all_ports: pulumi_wasm_rust::Output<Option<bool>>,
    /// Whether the mount should be read-only.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub read_only: pulumi_wasm_rust::Output<Option<bool>>,
    /// If `true`, it will remove anonymous volumes associated with the container. Defaults to `true`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub remove_volumes: pulumi_wasm_rust::Output<Option<bool>>,
    /// The restart policy for the container. Must be one of 'no', 'on-failure', 'always', 'unless-stopped'. Defaults to `no`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub restart: pulumi_wasm_rust::Output<Option<String>>,
    /// If `true`, then the container will be automatically removed when it exits. Defaults to `false`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub rm: pulumi_wasm_rust::Output<Option<bool>>,
    /// Runtime to use for the container.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub runtime: pulumi_wasm_rust::Output<Option<String>>,
    /// List of string values to customize labels for MLS systems, such as SELinux. See https://docs.docker.com/engine/reference/run/#security-configuration.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub security_opts: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// Size of `/dev/shm` in MBs.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub shm_size: pulumi_wasm_rust::Output<Option<i32>>,
    /// If `true`, then the Docker container will be started after creation. If `false`, then the container is only created. Defaults to `true`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub start: pulumi_wasm_rust::Output<Option<bool>>,
    /// If `true`, keep STDIN open even if not attached (`docker run -i`). Defaults to `false`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub stdin_open: pulumi_wasm_rust::Output<Option<bool>>,
    /// Signal to stop a container (default `SIGTERM`).
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub stop_signal: pulumi_wasm_rust::Output<Option<String>>,
    /// Timeout (in seconds) to stop a container.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub stop_timeout: pulumi_wasm_rust::Output<Option<i32>>,
    /// Key/value pairs for the storage driver options, e.g. `size`: `120G`
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub storage_opts: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    /// A map of kernel parameters (sysctls) to set in the container.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub sysctls: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    /// A map of container directories which should be replaced by `tmpfs mounts`, and their corresponding mount options.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub tmpfs: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    /// If `true`, allocate a pseudo-tty (`docker run -t`). Defaults to `false`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub tty: pulumi_wasm_rust::Output<Option<bool>>,
    /// Ulimit options to add.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub ulimits: pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerUlimit>>>,
    /// Specifies files to upload to the container before starting it. Only one of `content` or `content_base64` can be set and at least one of them has to be set.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub uploads: pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerUpload>>>,
    /// User used for run the first process. Format is `user` or `user:group` which user and group can be passed literraly or by name.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub user: pulumi_wasm_rust::Output<Option<String>>,
    /// Sets the usernamespace mode for the container when usernamespace remapping option is enabled.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub userns_mode: pulumi_wasm_rust::Output<Option<String>>,
    /// Spec for mounting volumes in the container.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub volumes: pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerVolume>>>,
    /// If `true`, then the Docker container is waited for being healthy state after creation. If `false`, then the container health state is not checked. Defaults to `false`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub wait: pulumi_wasm_rust::Output<Option<bool>>,
    /// The timeout in seconds to wait the container to be healthy after creation. Defaults to `60`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub wait_timeout: pulumi_wasm_rust::Output<Option<i32>>,
    /// The working directory for commands to run in.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
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
    pub networks_advanced: pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerNetworksAdvanced>>>,
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
    pub storage_opts: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
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
pub fn create(name: &str, args: ContainerArgs) -> ContainerResult {

    let result = crate::bindings::pulumi::docker::container::invoke(name, &crate::bindings::pulumi::docker::container::Args {
        attach: &args.attach.get_inner(),
        capabilities: &args.capabilities.get_inner(),
        cgroupns_mode: &args.cgroupns_mode.get_inner(),
        command: &args.command.get_inner(),
        container_read_refresh_timeout_milliseconds: &args.container_read_refresh_timeout_milliseconds.get_inner(),
        cpu_set: &args.cpu_set.get_inner(),
        cpu_shares: &args.cpu_shares.get_inner(),
        destroy_grace_seconds: &args.destroy_grace_seconds.get_inner(),
        devices: &args.devices.get_inner(),
        dns: &args.dns.get_inner(),
        dns_opts: &args.dns_opts.get_inner(),
        dns_searches: &args.dns_searches.get_inner(),
        domainname: &args.domainname.get_inner(),
        entrypoints: &args.entrypoints.get_inner(),
        envs: &args.envs.get_inner(),
        gpus: &args.gpus.get_inner(),
        group_adds: &args.group_adds.get_inner(),
        healthcheck: &args.healthcheck.get_inner(),
        hostname: &args.hostname.get_inner(),
        hosts: &args.hosts.get_inner(),
        image: &args.image.get_inner(),
        init: &args.init.get_inner(),
        ipc_mode: &args.ipc_mode.get_inner(),
        labels: &args.labels.get_inner(),
        log_driver: &args.log_driver.get_inner(),
        log_opts: &args.log_opts.get_inner(),
        logs: &args.logs.get_inner(),
        max_retry_count: &args.max_retry_count.get_inner(),
        memory: &args.memory.get_inner(),
        memory_swap: &args.memory_swap.get_inner(),
        mounts: &args.mounts.get_inner(),
        must_run: &args.must_run.get_inner(),
        name: &args.name.get_inner(),
        network_mode: &args.network_mode.get_inner(),
        networks_advanced: &args.networks_advanced.get_inner(),
        pid_mode: &args.pid_mode.get_inner(),
        ports: &args.ports.get_inner(),
        privileged: &args.privileged.get_inner(),
        publish_all_ports: &args.publish_all_ports.get_inner(),
        read_only: &args.read_only.get_inner(),
        remove_volumes: &args.remove_volumes.get_inner(),
        restart: &args.restart.get_inner(),
        rm: &args.rm.get_inner(),
        runtime: &args.runtime.get_inner(),
        security_opts: &args.security_opts.get_inner(),
        shm_size: &args.shm_size.get_inner(),
        start: &args.start.get_inner(),
        stdin_open: &args.stdin_open.get_inner(),
        stop_signal: &args.stop_signal.get_inner(),
        stop_timeout: &args.stop_timeout.get_inner(),
        storage_opts: &args.storage_opts.get_inner(),
        sysctls: &args.sysctls.get_inner(),
        tmpfs: &args.tmpfs.get_inner(),
        tty: &args.tty.get_inner(),
        ulimits: &args.ulimits.get_inner(),
        uploads: &args.uploads.get_inner(),
        user: &args.user.get_inner(),
        userns_mode: &args.userns_mode.get_inner(),
        volumes: &args.volumes.get_inner(),
        wait: &args.wait.get_inner(),
        wait_timeout: &args.wait_timeout.get_inner(),
        working_dir: &args.working_dir.get_inner(),
    });

    ContainerResult {
        attach: crate::into_domain(result.attach),
        bridge: crate::into_domain(result.bridge),
        capabilities: crate::into_domain(result.capabilities),
        cgroupns_mode: crate::into_domain(result.cgroupns_mode),
        command: crate::into_domain(result.command),
        container_logs: crate::into_domain(result.container_logs),
        container_read_refresh_timeout_milliseconds: crate::into_domain(result.container_read_refresh_timeout_milliseconds),
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
