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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ContainerArgs {
        /// If `true` attach to the container after its creation and waits the end of its execution. Defaults to `false`.
        #[builder(into, default)]
        pub attach: pulumi_wasm_rust::Output<Option<bool>>,
        /// Add or drop certrain linux capabilities.
        #[builder(into, default)]
        pub capabilities: pulumi_wasm_rust::Output<
            Option<super::types::ContainerCapabilities>,
        >,
        /// Cgroup namespace mode to use for the container. Possible values are: `private`, `host`.
        #[builder(into, default)]
        pub cgroupns_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The command to use to start the container. For example, to run `/usr/bin/myprogram -f baz.conf` set the command to be `["/usr/bin/myprogram","-f","baz.con"]`.
        #[builder(into, default)]
        pub command: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The total number of milliseconds to wait for the container to reach status 'running'
        #[builder(into, default)]
        pub container_read_refresh_timeout_milliseconds: pulumi_wasm_rust::Output<
            Option<i32>,
        >,
        /// A comma-separated list or hyphen-separated range of CPUs a container can use, e.g. `0-1`.
        #[builder(into, default)]
        pub cpu_set: pulumi_wasm_rust::Output<Option<String>>,
        /// CPU shares (relative weight) for the container.
        #[builder(into, default)]
        pub cpu_shares: pulumi_wasm_rust::Output<Option<i32>>,
        /// If defined will attempt to stop the container before destroying. Container will be destroyed after `n` seconds or on successful stop.
        #[builder(into, default)]
        pub destroy_grace_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// Bind devices to the container.
        #[builder(into, default)]
        pub devices: pulumi_wasm_rust::Output<
            Option<Vec<super::types::ContainerDevice>>,
        >,
        /// DNS servers to use.
        #[builder(into, default)]
        pub dns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// DNS options used by the DNS provider(s), see `resolv.conf` documentation for valid list of options.
        #[builder(into, default)]
        pub dns_opts: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// DNS search domains that are used when bare unqualified hostnames are used inside of the container.
        #[builder(into, default)]
        pub dns_searches: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Domain name of the container.
        #[builder(into, default)]
        pub domainname: pulumi_wasm_rust::Output<Option<String>>,
        /// The command to use as the Entrypoint for the container. The Entrypoint allows you to configure a container to run as an executable. For example, to run `/usr/bin/myprogram` when starting a container, set the entrypoint to be `"/usr/bin/myprogra"]`.
        #[builder(into, default)]
        pub entrypoints: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Environment variables to set in the form of `KEY=VALUE`, e.g. `DEBUG=0`
        #[builder(into, default)]
        pub envs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// GPU devices to add to the container. Currently, only the value `all` is supported. Passing any other value will result in unexpected behavior.
        #[builder(into, default)]
        pub gpus: pulumi_wasm_rust::Output<Option<String>>,
        /// Additional groups for the container user
        #[builder(into, default)]
        pub group_adds: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A test to perform to check that the container is healthy
        #[builder(into, default)]
        pub healthcheck: pulumi_wasm_rust::Output<
            Option<super::types::ContainerHealthcheck>,
        >,
        /// Hostname of the container.
        #[builder(into, default)]
        pub hostname: pulumi_wasm_rust::Output<Option<String>>,
        /// Hostname to add
        #[builder(into, default)]
        pub hosts: pulumi_wasm_rust::Output<Option<Vec<super::types::ContainerHost>>>,
        /// The ID of the image to back this container. The easiest way to get this value is to use the `docker.RemoteImage` resource as is shown in the example.
        #[builder(into)]
        pub image: pulumi_wasm_rust::Output<String>,
        /// Configured whether an init process should be injected for this container. If unset this will default to the `dockerd` defaults.
        #[builder(into, default)]
        pub init: pulumi_wasm_rust::Output<Option<bool>>,
        /// IPC sharing mode for the container. Possible values are: `none`, `private`, `shareable`, `container:<name|id>` or `host`.
        #[builder(into, default)]
        pub ipc_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// User-defined key/value metadata.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<Option<Vec<super::types::ContainerLabel>>>,
        /// The logging driver to use for the container.
        #[builder(into, default)]
        pub log_driver: pulumi_wasm_rust::Output<Option<String>>,
        /// Key/value pairs to use as options for the logging driver.
        #[builder(into, default)]
        pub log_opts: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Save the container logs (`attach` must be enabled). Defaults to `false`.
        #[builder(into, default)]
        pub logs: pulumi_wasm_rust::Output<Option<bool>>,
        /// The maximum amount of times to an attempt a restart when `restart` is set to 'on-failure'.
        #[builder(into, default)]
        pub max_retry_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// The memory limit for the container in MBs.
        #[builder(into, default)]
        pub memory: pulumi_wasm_rust::Output<Option<i32>>,
        /// The total memory limit (memory + swap) for the container in MBs. This setting may compute to `-1` after `pulumi up` if the target host doesn't support memory swap, when that is the case docker will use a soft limitation.
        #[builder(into, default)]
        pub memory_swap: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specification for mounts to be added to containers created as part of the service.
        #[builder(into, default)]
        pub mounts: pulumi_wasm_rust::Output<Option<Vec<super::types::ContainerMount>>>,
        /// If `true`, then the Docker container will be kept running. If `false`, then as long as the container exists, Terraform
        /// assumes it is successful. Defaults to `true`.
        #[builder(into, default)]
        pub must_run: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name or id of the network to use. You can use `name` or `id` attribute from a `docker.Network` resource.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Network mode of the container.
        #[builder(into, default)]
        pub network_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The networks the container is attached to
        #[builder(into, default)]
        pub networks_advanced: pulumi_wasm_rust::Output<
            Option<Vec<super::types::ContainerNetworksAdvanced>>,
        >,
        /// he PID (Process) Namespace mode for the container. Either `container:<name|id>` or `host`.
        #[builder(into, default)]
        pub pid_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// Publish a container's port(s) to the host.
        #[builder(into, default)]
        pub ports: pulumi_wasm_rust::Output<Option<Vec<super::types::ContainerPort>>>,
        /// If `true`, the container runs in privileged mode.
        #[builder(into, default)]
        pub privileged: pulumi_wasm_rust::Output<Option<bool>>,
        /// Publish all ports of the container.
        #[builder(into, default)]
        pub publish_all_ports: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether the mount should be read-only.
        #[builder(into, default)]
        pub read_only: pulumi_wasm_rust::Output<Option<bool>>,
        /// If `true`, it will remove anonymous volumes associated with the container. Defaults to `true`.
        #[builder(into, default)]
        pub remove_volumes: pulumi_wasm_rust::Output<Option<bool>>,
        /// The restart policy for the container. Must be one of 'no', 'on-failure', 'always', 'unless-stopped'. Defaults to `no`.
        #[builder(into, default)]
        pub restart: pulumi_wasm_rust::Output<Option<String>>,
        /// If `true`, then the container will be automatically removed when it exits. Defaults to `false`.
        #[builder(into, default)]
        pub rm: pulumi_wasm_rust::Output<Option<bool>>,
        /// Runtime to use for the container.
        #[builder(into, default)]
        pub runtime: pulumi_wasm_rust::Output<Option<String>>,
        /// List of string values to customize labels for MLS systems, such as SELinux. See https://docs.docker.com/engine/reference/run/#security-configuration.
        #[builder(into, default)]
        pub security_opts: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Size of `/dev/shm` in MBs.
        #[builder(into, default)]
        pub shm_size: pulumi_wasm_rust::Output<Option<i32>>,
        /// If `true`, then the Docker container will be started after creation. If `false`, then the container is only created. Defaults to `true`.
        #[builder(into, default)]
        pub start: pulumi_wasm_rust::Output<Option<bool>>,
        /// If `true`, keep STDIN open even if not attached (`docker run -i`). Defaults to `false`.
        #[builder(into, default)]
        pub stdin_open: pulumi_wasm_rust::Output<Option<bool>>,
        /// Signal to stop a container (default `SIGTERM`).
        #[builder(into, default)]
        pub stop_signal: pulumi_wasm_rust::Output<Option<String>>,
        /// Timeout (in seconds) to stop a container.
        #[builder(into, default)]
        pub stop_timeout: pulumi_wasm_rust::Output<Option<i32>>,
        /// Key/value pairs for the storage driver options, e.g. `size`: `120G`
        #[builder(into, default)]
        pub storage_opts: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of kernel parameters (sysctls) to set in the container.
        #[builder(into, default)]
        pub sysctls: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of container directories which should be replaced by `tmpfs mounts`, and their corresponding mount options.
        #[builder(into, default)]
        pub tmpfs: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// If `true`, allocate a pseudo-tty (`docker run -t`). Defaults to `false`.
        #[builder(into, default)]
        pub tty: pulumi_wasm_rust::Output<Option<bool>>,
        /// Ulimit options to add.
        #[builder(into, default)]
        pub ulimits: pulumi_wasm_rust::Output<
            Option<Vec<super::types::ContainerUlimit>>,
        >,
        /// Specifies files to upload to the container before starting it. Only one of `content` or `content_base64` can be set and at least one of them has to be set.
        #[builder(into, default)]
        pub uploads: pulumi_wasm_rust::Output<
            Option<Vec<super::types::ContainerUpload>>,
        >,
        /// User used for run the first process. Format is `user` or `user:group` which user and group can be passed literraly or by name.
        #[builder(into, default)]
        pub user: pulumi_wasm_rust::Output<Option<String>>,
        /// Sets the usernamespace mode for the container when usernamespace remapping option is enabled.
        #[builder(into, default)]
        pub userns_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// Spec for mounting volumes in the container.
        #[builder(into, default)]
        pub volumes: pulumi_wasm_rust::Output<
            Option<Vec<super::types::ContainerVolume>>,
        >,
        /// If `true`, then the Docker container is waited for being healthy state after creation. If `false`, then the container health state is not checked. Defaults to `false`.
        #[builder(into, default)]
        pub wait: pulumi_wasm_rust::Output<Option<bool>>,
        /// The timeout in seconds to wait the container to be healthy after creation. Defaults to `60`.
        #[builder(into, default)]
        pub wait_timeout: pulumi_wasm_rust::Output<Option<i32>>,
        /// The working directory for commands to run in.
        #[builder(into, default)]
        pub working_dir: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ContainerResult {
        /// If `true` attach to the container after its creation and waits the end of its execution. Defaults to `false`.
        pub attach: pulumi_wasm_rust::Output<Option<bool>>,
        /// The network bridge of the container as read from its NetworkSettings.
        pub bridge: pulumi_wasm_rust::Output<String>,
        /// Add or drop certrain linux capabilities.
        pub capabilities: pulumi_wasm_rust::Output<
            Option<super::types::ContainerCapabilities>,
        >,
        /// Cgroup namespace mode to use for the container. Possible values are: `private`, `host`.
        pub cgroupns_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The command to use to start the container. For example, to run `/usr/bin/myprogram -f baz.conf` set the command to be `["/usr/bin/myprogram","-f","baz.con"]`.
        pub command: pulumi_wasm_rust::Output<Vec<String>>,
        /// The logs of the container if its execution is done (`attach` must be disabled).
        pub container_logs: pulumi_wasm_rust::Output<Option<String>>,
        /// The total number of milliseconds to wait for the container to reach status 'running'
        pub container_read_refresh_timeout_milliseconds: pulumi_wasm_rust::Output<
            Option<i32>,
        >,
        /// A comma-separated list or hyphen-separated range of CPUs a container can use, e.g. `0-1`.
        pub cpu_set: pulumi_wasm_rust::Output<Option<String>>,
        /// CPU shares (relative weight) for the container.
        pub cpu_shares: pulumi_wasm_rust::Output<Option<i32>>,
        /// If defined will attempt to stop the container before destroying. Container will be destroyed after `n` seconds or on successful stop.
        pub destroy_grace_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// Bind devices to the container.
        pub devices: pulumi_wasm_rust::Output<
            Option<Vec<super::types::ContainerDevice>>,
        >,
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
        pub healthcheck: pulumi_wasm_rust::Output<
            Option<super::types::ContainerHealthcheck>,
        >,
        /// Hostname of the container.
        pub hostname: pulumi_wasm_rust::Output<String>,
        /// Hostname to add
        pub hosts: pulumi_wasm_rust::Output<Option<Vec<super::types::ContainerHost>>>,
        /// The ID of the image to back this container. The easiest way to get this value is to use the `docker.RemoteImage` resource as is shown in the example.
        pub image: pulumi_wasm_rust::Output<String>,
        /// Configured whether an init process should be injected for this container. If unset this will default to the `dockerd` defaults.
        pub init: pulumi_wasm_rust::Output<bool>,
        /// IPC sharing mode for the container. Possible values are: `none`, `private`, `shareable`, `container:<name|id>` or `host`.
        pub ipc_mode: pulumi_wasm_rust::Output<String>,
        /// User-defined key/value metadata.
        pub labels: pulumi_wasm_rust::Output<Vec<super::types::ContainerLabel>>,
        /// The logging driver to use for the container.
        pub log_driver: pulumi_wasm_rust::Output<String>,
        /// Key/value pairs to use as options for the logging driver.
        pub log_opts: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Save the container logs (`attach` must be enabled). Defaults to `false`.
        pub logs: pulumi_wasm_rust::Output<Option<bool>>,
        /// The maximum amount of times to an attempt a restart when `restart` is set to 'on-failure'.
        pub max_retry_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// The memory limit for the container in MBs.
        pub memory: pulumi_wasm_rust::Output<Option<i32>>,
        /// The total memory limit (memory + swap) for the container in MBs. This setting may compute to `-1` after `pulumi up` if the target host doesn't support memory swap, when that is the case docker will use a soft limitation.
        pub memory_swap: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specification for mounts to be added to containers created as part of the service.
        pub mounts: pulumi_wasm_rust::Output<Option<Vec<super::types::ContainerMount>>>,
        /// If `true`, then the Docker container will be kept running. If `false`, then as long as the container exists, Terraform
        /// assumes it is successful. Defaults to `true`.
        pub must_run: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name or id of the network to use. You can use `name` or `id` attribute from a `docker.Network` resource.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The data of the networks the container is connected to.
        pub network_datas: pulumi_wasm_rust::Output<
            Vec<super::types::ContainerNetworkData>,
        >,
        /// Network mode of the container.
        pub network_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The networks the container is attached to
        pub networks_advanced: pulumi_wasm_rust::Output<
            Option<Vec<super::types::ContainerNetworksAdvanced>>,
        >,
        /// he PID (Process) Namespace mode for the container. Either `container:<name|id>` or `host`.
        pub pid_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// Publish a container's port(s) to the host.
        pub ports: pulumi_wasm_rust::Output<Option<Vec<super::types::ContainerPort>>>,
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
        pub storage_opts: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of kernel parameters (sysctls) to set in the container.
        pub sysctls: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of container directories which should be replaced by `tmpfs mounts`, and their corresponding mount options.
        pub tmpfs: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// If `true`, allocate a pseudo-tty (`docker run -t`). Defaults to `false`.
        pub tty: pulumi_wasm_rust::Output<Option<bool>>,
        /// Ulimit options to add.
        pub ulimits: pulumi_wasm_rust::Output<
            Option<Vec<super::types::ContainerUlimit>>,
        >,
        /// Specifies files to upload to the container before starting it. Only one of `content` or `content_base64` can be set and at least one of them has to be set.
        pub uploads: pulumi_wasm_rust::Output<
            Option<Vec<super::types::ContainerUpload>>,
        >,
        /// User used for run the first process. Format is `user` or `user:group` which user and group can be passed literraly or by name.
        pub user: pulumi_wasm_rust::Output<Option<String>>,
        /// Sets the usernamespace mode for the container when usernamespace remapping option is enabled.
        pub userns_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// Spec for mounting volumes in the container.
        pub volumes: pulumi_wasm_rust::Output<
            Option<Vec<super::types::ContainerVolume>>,
        >,
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
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ContainerArgs) -> ContainerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let attach_binding = args.attach.get_inner();
        let capabilities_binding = args.capabilities.get_inner();
        let cgroupns_mode_binding = args.cgroupns_mode.get_inner();
        let command_binding = args.command.get_inner();
        let container_read_refresh_timeout_milliseconds_binding = args
            .container_read_refresh_timeout_milliseconds
            .get_inner();
        let cpu_set_binding = args.cpu_set.get_inner();
        let cpu_shares_binding = args.cpu_shares.get_inner();
        let destroy_grace_seconds_binding = args.destroy_grace_seconds.get_inner();
        let devices_binding = args.devices.get_inner();
        let dns_binding = args.dns.get_inner();
        let dns_opts_binding = args.dns_opts.get_inner();
        let dns_searches_binding = args.dns_searches.get_inner();
        let domainname_binding = args.domainname.get_inner();
        let entrypoints_binding = args.entrypoints.get_inner();
        let envs_binding = args.envs.get_inner();
        let gpus_binding = args.gpus.get_inner();
        let group_adds_binding = args.group_adds.get_inner();
        let healthcheck_binding = args.healthcheck.get_inner();
        let hostname_binding = args.hostname.get_inner();
        let hosts_binding = args.hosts.get_inner();
        let image_binding = args.image.get_inner();
        let init_binding = args.init.get_inner();
        let ipc_mode_binding = args.ipc_mode.get_inner();
        let labels_binding = args.labels.get_inner();
        let log_driver_binding = args.log_driver.get_inner();
        let log_opts_binding = args.log_opts.get_inner();
        let logs_binding = args.logs.get_inner();
        let max_retry_count_binding = args.max_retry_count.get_inner();
        let memory_binding = args.memory.get_inner();
        let memory_swap_binding = args.memory_swap.get_inner();
        let mounts_binding = args.mounts.get_inner();
        let must_run_binding = args.must_run.get_inner();
        let name_binding = args.name.get_inner();
        let network_mode_binding = args.network_mode.get_inner();
        let networks_advanced_binding = args.networks_advanced.get_inner();
        let pid_mode_binding = args.pid_mode.get_inner();
        let ports_binding = args.ports.get_inner();
        let privileged_binding = args.privileged.get_inner();
        let publish_all_ports_binding = args.publish_all_ports.get_inner();
        let read_only_binding = args.read_only.get_inner();
        let remove_volumes_binding = args.remove_volumes.get_inner();
        let restart_binding = args.restart.get_inner();
        let rm_binding = args.rm.get_inner();
        let runtime_binding = args.runtime.get_inner();
        let security_opts_binding = args.security_opts.get_inner();
        let shm_size_binding = args.shm_size.get_inner();
        let start_binding = args.start.get_inner();
        let stdin_open_binding = args.stdin_open.get_inner();
        let stop_signal_binding = args.stop_signal.get_inner();
        let stop_timeout_binding = args.stop_timeout.get_inner();
        let storage_opts_binding = args.storage_opts.get_inner();
        let sysctls_binding = args.sysctls.get_inner();
        let tmpfs_binding = args.tmpfs.get_inner();
        let tty_binding = args.tty.get_inner();
        let ulimits_binding = args.ulimits.get_inner();
        let uploads_binding = args.uploads.get_inner();
        let user_binding = args.user.get_inner();
        let userns_mode_binding = args.userns_mode.get_inner();
        let volumes_binding = args.volumes.get_inner();
        let wait_binding = args.wait.get_inner();
        let wait_timeout_binding = args.wait_timeout.get_inner();
        let working_dir_binding = args.working_dir.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "docker:index/container:Container".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "attach".into(),
                    value: &attach_binding,
                },
                register_interface::ObjectField {
                    name: "capabilities".into(),
                    value: &capabilities_binding,
                },
                register_interface::ObjectField {
                    name: "cgroupnsMode".into(),
                    value: &cgroupns_mode_binding,
                },
                register_interface::ObjectField {
                    name: "command".into(),
                    value: &command_binding,
                },
                register_interface::ObjectField {
                    name: "containerReadRefreshTimeoutMilliseconds".into(),
                    value: &container_read_refresh_timeout_milliseconds_binding,
                },
                register_interface::ObjectField {
                    name: "cpuSet".into(),
                    value: &cpu_set_binding,
                },
                register_interface::ObjectField {
                    name: "cpuShares".into(),
                    value: &cpu_shares_binding,
                },
                register_interface::ObjectField {
                    name: "destroyGraceSeconds".into(),
                    value: &destroy_grace_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "devices".into(),
                    value: &devices_binding,
                },
                register_interface::ObjectField {
                    name: "dns".into(),
                    value: &dns_binding,
                },
                register_interface::ObjectField {
                    name: "dnsOpts".into(),
                    value: &dns_opts_binding,
                },
                register_interface::ObjectField {
                    name: "dnsSearches".into(),
                    value: &dns_searches_binding,
                },
                register_interface::ObjectField {
                    name: "domainname".into(),
                    value: &domainname_binding,
                },
                register_interface::ObjectField {
                    name: "entrypoints".into(),
                    value: &entrypoints_binding,
                },
                register_interface::ObjectField {
                    name: "envs".into(),
                    value: &envs_binding,
                },
                register_interface::ObjectField {
                    name: "gpus".into(),
                    value: &gpus_binding,
                },
                register_interface::ObjectField {
                    name: "groupAdds".into(),
                    value: &group_adds_binding,
                },
                register_interface::ObjectField {
                    name: "healthcheck".into(),
                    value: &healthcheck_binding,
                },
                register_interface::ObjectField {
                    name: "hostname".into(),
                    value: &hostname_binding,
                },
                register_interface::ObjectField {
                    name: "hosts".into(),
                    value: &hosts_binding,
                },
                register_interface::ObjectField {
                    name: "image".into(),
                    value: &image_binding,
                },
                register_interface::ObjectField {
                    name: "init".into(),
                    value: &init_binding,
                },
                register_interface::ObjectField {
                    name: "ipcMode".into(),
                    value: &ipc_mode_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "logDriver".into(),
                    value: &log_driver_binding,
                },
                register_interface::ObjectField {
                    name: "logOpts".into(),
                    value: &log_opts_binding,
                },
                register_interface::ObjectField {
                    name: "logs".into(),
                    value: &logs_binding,
                },
                register_interface::ObjectField {
                    name: "maxRetryCount".into(),
                    value: &max_retry_count_binding,
                },
                register_interface::ObjectField {
                    name: "memory".into(),
                    value: &memory_binding,
                },
                register_interface::ObjectField {
                    name: "memorySwap".into(),
                    value: &memory_swap_binding,
                },
                register_interface::ObjectField {
                    name: "mounts".into(),
                    value: &mounts_binding,
                },
                register_interface::ObjectField {
                    name: "mustRun".into(),
                    value: &must_run_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networkMode".into(),
                    value: &network_mode_binding,
                },
                register_interface::ObjectField {
                    name: "networksAdvanced".into(),
                    value: &networks_advanced_binding,
                },
                register_interface::ObjectField {
                    name: "pidMode".into(),
                    value: &pid_mode_binding,
                },
                register_interface::ObjectField {
                    name: "ports".into(),
                    value: &ports_binding,
                },
                register_interface::ObjectField {
                    name: "privileged".into(),
                    value: &privileged_binding,
                },
                register_interface::ObjectField {
                    name: "publishAllPorts".into(),
                    value: &publish_all_ports_binding,
                },
                register_interface::ObjectField {
                    name: "readOnly".into(),
                    value: &read_only_binding,
                },
                register_interface::ObjectField {
                    name: "removeVolumes".into(),
                    value: &remove_volumes_binding,
                },
                register_interface::ObjectField {
                    name: "restart".into(),
                    value: &restart_binding,
                },
                register_interface::ObjectField {
                    name: "rm".into(),
                    value: &rm_binding,
                },
                register_interface::ObjectField {
                    name: "runtime".into(),
                    value: &runtime_binding,
                },
                register_interface::ObjectField {
                    name: "securityOpts".into(),
                    value: &security_opts_binding,
                },
                register_interface::ObjectField {
                    name: "shmSize".into(),
                    value: &shm_size_binding,
                },
                register_interface::ObjectField {
                    name: "start".into(),
                    value: &start_binding,
                },
                register_interface::ObjectField {
                    name: "stdinOpen".into(),
                    value: &stdin_open_binding,
                },
                register_interface::ObjectField {
                    name: "stopSignal".into(),
                    value: &stop_signal_binding,
                },
                register_interface::ObjectField {
                    name: "stopTimeout".into(),
                    value: &stop_timeout_binding,
                },
                register_interface::ObjectField {
                    name: "storageOpts".into(),
                    value: &storage_opts_binding,
                },
                register_interface::ObjectField {
                    name: "sysctls".into(),
                    value: &sysctls_binding,
                },
                register_interface::ObjectField {
                    name: "tmpfs".into(),
                    value: &tmpfs_binding,
                },
                register_interface::ObjectField {
                    name: "tty".into(),
                    value: &tty_binding,
                },
                register_interface::ObjectField {
                    name: "ulimits".into(),
                    value: &ulimits_binding,
                },
                register_interface::ObjectField {
                    name: "uploads".into(),
                    value: &uploads_binding,
                },
                register_interface::ObjectField {
                    name: "user".into(),
                    value: &user_binding,
                },
                register_interface::ObjectField {
                    name: "usernsMode".into(),
                    value: &userns_mode_binding,
                },
                register_interface::ObjectField {
                    name: "volumes".into(),
                    value: &volumes_binding,
                },
                register_interface::ObjectField {
                    name: "wait".into(),
                    value: &wait_binding,
                },
                register_interface::ObjectField {
                    name: "waitTimeout".into(),
                    value: &wait_timeout_binding,
                },
                register_interface::ObjectField {
                    name: "workingDir".into(),
                    value: &working_dir_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "attach".into(),
                },
                register_interface::ResultField {
                    name: "bridge".into(),
                },
                register_interface::ResultField {
                    name: "capabilities".into(),
                },
                register_interface::ResultField {
                    name: "cgroupnsMode".into(),
                },
                register_interface::ResultField {
                    name: "command".into(),
                },
                register_interface::ResultField {
                    name: "containerLogs".into(),
                },
                register_interface::ResultField {
                    name: "containerReadRefreshTimeoutMilliseconds".into(),
                },
                register_interface::ResultField {
                    name: "cpuSet".into(),
                },
                register_interface::ResultField {
                    name: "cpuShares".into(),
                },
                register_interface::ResultField {
                    name: "destroyGraceSeconds".into(),
                },
                register_interface::ResultField {
                    name: "devices".into(),
                },
                register_interface::ResultField {
                    name: "dns".into(),
                },
                register_interface::ResultField {
                    name: "dnsOpts".into(),
                },
                register_interface::ResultField {
                    name: "dnsSearches".into(),
                },
                register_interface::ResultField {
                    name: "domainname".into(),
                },
                register_interface::ResultField {
                    name: "entrypoints".into(),
                },
                register_interface::ResultField {
                    name: "envs".into(),
                },
                register_interface::ResultField {
                    name: "exitCode".into(),
                },
                register_interface::ResultField {
                    name: "gpus".into(),
                },
                register_interface::ResultField {
                    name: "groupAdds".into(),
                },
                register_interface::ResultField {
                    name: "healthcheck".into(),
                },
                register_interface::ResultField {
                    name: "hostname".into(),
                },
                register_interface::ResultField {
                    name: "hosts".into(),
                },
                register_interface::ResultField {
                    name: "image".into(),
                },
                register_interface::ResultField {
                    name: "init".into(),
                },
                register_interface::ResultField {
                    name: "ipcMode".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "logDriver".into(),
                },
                register_interface::ResultField {
                    name: "logOpts".into(),
                },
                register_interface::ResultField {
                    name: "logs".into(),
                },
                register_interface::ResultField {
                    name: "maxRetryCount".into(),
                },
                register_interface::ResultField {
                    name: "memory".into(),
                },
                register_interface::ResultField {
                    name: "memorySwap".into(),
                },
                register_interface::ResultField {
                    name: "mounts".into(),
                },
                register_interface::ResultField {
                    name: "mustRun".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkDatas".into(),
                },
                register_interface::ResultField {
                    name: "networkMode".into(),
                },
                register_interface::ResultField {
                    name: "networksAdvanced".into(),
                },
                register_interface::ResultField {
                    name: "pidMode".into(),
                },
                register_interface::ResultField {
                    name: "ports".into(),
                },
                register_interface::ResultField {
                    name: "privileged".into(),
                },
                register_interface::ResultField {
                    name: "publishAllPorts".into(),
                },
                register_interface::ResultField {
                    name: "readOnly".into(),
                },
                register_interface::ResultField {
                    name: "removeVolumes".into(),
                },
                register_interface::ResultField {
                    name: "restart".into(),
                },
                register_interface::ResultField {
                    name: "rm".into(),
                },
                register_interface::ResultField {
                    name: "runtime".into(),
                },
                register_interface::ResultField {
                    name: "securityOpts".into(),
                },
                register_interface::ResultField {
                    name: "shmSize".into(),
                },
                register_interface::ResultField {
                    name: "start".into(),
                },
                register_interface::ResultField {
                    name: "stdinOpen".into(),
                },
                register_interface::ResultField {
                    name: "stopSignal".into(),
                },
                register_interface::ResultField {
                    name: "stopTimeout".into(),
                },
                register_interface::ResultField {
                    name: "storageOpts".into(),
                },
                register_interface::ResultField {
                    name: "sysctls".into(),
                },
                register_interface::ResultField {
                    name: "tmpfs".into(),
                },
                register_interface::ResultField {
                    name: "tty".into(),
                },
                register_interface::ResultField {
                    name: "ulimits".into(),
                },
                register_interface::ResultField {
                    name: "uploads".into(),
                },
                register_interface::ResultField {
                    name: "user".into(),
                },
                register_interface::ResultField {
                    name: "usernsMode".into(),
                },
                register_interface::ResultField {
                    name: "volumes".into(),
                },
                register_interface::ResultField {
                    name: "wait".into(),
                },
                register_interface::ResultField {
                    name: "waitTimeout".into(),
                },
                register_interface::ResultField {
                    name: "workingDir".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ContainerResult {
            attach: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("attach").unwrap(),
            ),
            bridge: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bridge").unwrap(),
            ),
            capabilities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("capabilities").unwrap(),
            ),
            cgroupns_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cgroupnsMode").unwrap(),
            ),
            command: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("command").unwrap(),
            ),
            container_logs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("containerLogs").unwrap(),
            ),
            container_read_refresh_timeout_milliseconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("containerReadRefreshTimeoutMilliseconds").unwrap(),
            ),
            cpu_set: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cpuSet").unwrap(),
            ),
            cpu_shares: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cpuShares").unwrap(),
            ),
            destroy_grace_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destroyGraceSeconds").unwrap(),
            ),
            devices: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("devices").unwrap(),
            ),
            dns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dns").unwrap(),
            ),
            dns_opts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsOpts").unwrap(),
            ),
            dns_searches: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsSearches").unwrap(),
            ),
            domainname: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainname").unwrap(),
            ),
            entrypoints: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("entrypoints").unwrap(),
            ),
            envs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("envs").unwrap(),
            ),
            exit_code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("exitCode").unwrap(),
            ),
            gpus: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gpus").unwrap(),
            ),
            group_adds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("groupAdds").unwrap(),
            ),
            healthcheck: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("healthcheck").unwrap(),
            ),
            hostname: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostname").unwrap(),
            ),
            hosts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hosts").unwrap(),
            ),
            image: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("image").unwrap(),
            ),
            init: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("init").unwrap(),
            ),
            ipc_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipcMode").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            log_driver: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logDriver").unwrap(),
            ),
            log_opts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logOpts").unwrap(),
            ),
            logs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logs").unwrap(),
            ),
            max_retry_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxRetryCount").unwrap(),
            ),
            memory: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("memory").unwrap(),
            ),
            memory_swap: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("memorySwap").unwrap(),
            ),
            mounts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mounts").unwrap(),
            ),
            must_run: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mustRun").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_datas: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkDatas").unwrap(),
            ),
            network_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkMode").unwrap(),
            ),
            networks_advanced: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networksAdvanced").unwrap(),
            ),
            pid_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pidMode").unwrap(),
            ),
            ports: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ports").unwrap(),
            ),
            privileged: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privileged").unwrap(),
            ),
            publish_all_ports: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publishAllPorts").unwrap(),
            ),
            read_only: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("readOnly").unwrap(),
            ),
            remove_volumes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("removeVolumes").unwrap(),
            ),
            restart: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("restart").unwrap(),
            ),
            rm: pulumi_wasm_rust::__private::into_domain(hashmap.remove("rm").unwrap()),
            runtime: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("runtime").unwrap(),
            ),
            security_opts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityOpts").unwrap(),
            ),
            shm_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("shmSize").unwrap(),
            ),
            start: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("start").unwrap(),
            ),
            stdin_open: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stdinOpen").unwrap(),
            ),
            stop_signal: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stopSignal").unwrap(),
            ),
            stop_timeout: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stopTimeout").unwrap(),
            ),
            storage_opts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageOpts").unwrap(),
            ),
            sysctls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sysctls").unwrap(),
            ),
            tmpfs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tmpfs").unwrap(),
            ),
            tty: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tty").unwrap(),
            ),
            ulimits: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ulimits").unwrap(),
            ),
            uploads: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uploads").unwrap(),
            ),
            user: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("user").unwrap(),
            ),
            userns_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("usernsMode").unwrap(),
            ),
            volumes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("volumes").unwrap(),
            ),
            wait: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("wait").unwrap(),
            ),
            wait_timeout: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("waitTimeout").unwrap(),
            ),
            working_dir: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workingDir").unwrap(),
            ),
        }
    }
}
