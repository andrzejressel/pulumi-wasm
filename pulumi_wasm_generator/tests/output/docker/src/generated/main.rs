pub mod container {
    //! <!-- Bug: Type and Name are switched -->
    //! Manages the lifecycle of a Docker container.
    //!
    //! ## Example Usage
    //!
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
    //!
    //! ## Import
    //!
    //! ### Example
    //!
    //! Assuming you created a `container` as follows
    //!
    //! ```sh
    //! docker run --name foo -p8080:80 -d nginx
    //! ```
    //!
    //! prints the container ID
    //!
    //! ```text
    //! 9a550c0f0163d39d77222d3efd58701b625d47676c25c686c95b5b92d1cba6fd
    //! ```
    //!
    //! you provide the definition for the resource as follows
    //!
    //! ```ignore
    //! use pulumi_wasm_rust::Output;
    //! use pulumi_wasm_rust::{add_export, pulumi_main};
    //! #[pulumi_main]
    //! fn test_main() -> Result<(), Error> {
    //!     let foo = container::create(
    //!         "foo",
    //!         ContainerArgs::builder()
    //!             .image("nginx")
    //!             .name("foo")
    //!             .ports(
    //!                 vec![
    //!                     ContainerPort::builder().external(8080).internal(80).build_struct(),
    //!                 ],
    //!             )
    //!             .build_struct(),
    //!     );
    //! }
    //! ```
    //!
    //! then the import command is as follows
    //!
    //! ```sh
    //! $ pulumi import docker:index/container:Container foo 9a550c0f0163d39d77222d3efd58701b625d47676c25c686c95b5b92d1cba6fd
    //! ```
    //!
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
pub mod image {
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
    //! ### A Docker image build and push
    //!
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
    //! ### Docker image build using caching with AWS Elastic Container Registry
    //!
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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ImageArgs {
        /// The Docker build context
        #[builder(into, default)]
        pub build: pulumi_wasm_rust::Output<Option<super::types::DockerBuild>>,
        /// A flag to build an image on preview
        #[builder(into, default)]
        pub build_on_preview: pulumi_wasm_rust::Output<Option<bool>>,
        /// The image name, of the format repository[:tag], e.g. `docker.io/username/demo-image:v1`.
        /// This reference is not unique to each build and push.For the unique manifest SHA of a pushed docker image, or the local image ID, please use `repoDigest`.
        #[builder(into)]
        pub image_name: pulumi_wasm_rust::Output<String>,
        /// The registry to push the image to
        #[builder(into, default)]
        pub registry: pulumi_wasm_rust::Output<Option<super::types::Registry>>,
        /// A flag to skip a registry push.
        #[builder(into, default)]
        pub skip_push: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
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
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ImageArgs) -> ImageResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let build_binding = args.build.get_inner();
        let build_on_preview_binding = args.build_on_preview.get_inner();
        let image_name_binding = args.image_name.get_inner();
        let registry_binding = args.registry.get_inner();
        let skip_push_binding = args.skip_push.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "docker:index/image:Image".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "build".into(),
                    value: &build_binding,
                },
                register_interface::ObjectField {
                    name: "buildOnPreview".into(),
                    value: &build_on_preview_binding,
                },
                register_interface::ObjectField {
                    name: "imageName".into(),
                    value: &image_name_binding,
                },
                register_interface::ObjectField {
                    name: "registry".into(),
                    value: &registry_binding,
                },
                register_interface::ObjectField {
                    name: "skipPush".into(),
                    value: &skip_push_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "baseImageName".into(),
                },
                register_interface::ResultField {
                    name: "context".into(),
                },
                register_interface::ResultField {
                    name: "dockerfile".into(),
                },
                register_interface::ResultField {
                    name: "imageName".into(),
                },
                register_interface::ResultField {
                    name: "platform".into(),
                },
                register_interface::ResultField {
                    name: "registryServer".into(),
                },
                register_interface::ResultField {
                    name: "repoDigest".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ImageResult {
            base_image_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("baseImageName").unwrap(),
            ),
            context: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("context").unwrap(),
            ),
            dockerfile: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dockerfile").unwrap(),
            ),
            image_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageName").unwrap(),
            ),
            platform: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("platform").unwrap(),
            ),
            registry_server: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("registryServer").unwrap(),
            ),
            repo_digest: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("repoDigest").unwrap(),
            ),
        }
    }
}
pub mod network {
    //! <!-- Bug: Type and Name are switched -->
    //! `docker.Network` provides a docker network resource.
    //!
    //! ## Example Usage
    //!
    //! ```ignore
    //! use pulumi_wasm_rust::Output;
    //! use pulumi_wasm_rust::{add_export, pulumi_main};
    //! #[pulumi_main]
    //! fn test_main() -> Result<(), Error> {
    //!     let privateNetwork = network::create(
    //!         "privateNetwork",
    //!         NetworkArgs::builder().build_struct(),
    //!     );
    //! }
    //! ```
    //!
    //! ## Import
    //!
    //! ### Example
    //!
    //! Assuming you created a `network` as follows
    //!
    //! ```shell
    //! docker network create foo
    //! ````
    //!
    //! prints the long ID
    //!
    //! ```text
    //! 87b57a9b91ecab2db2a6dbf38df74c67d7c7108cbe479d6576574ec2cd8c2d73
    //! ```
    //!
    //! you provide the definition for the resource as follows
    //!
    //! ```ignore
    //! use pulumi_wasm_rust::Output;
    //! use pulumi_wasm_rust::{add_export, pulumi_main};
    //! #[pulumi_main]
    //! fn test_main() -> Result<(), Error> {
    //!     let foo = network::create("foo", NetworkArgs::builder().name("foo").build_struct());
    //! }
    //! ```
    //!
    //! then the import command is as follows
    //!
    //! ```sh
    //! $ pulumi import docker:index/network:Network foo 87b57a9b91ecab2db2a6dbf38df74c67d7c7108cbe479d6576574ec2cd8c2d73
    //! ```
    //!
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkArgs {
        /// Enable manual container attachment to the network.
        #[builder(into, default)]
        pub attachable: pulumi_wasm_rust::Output<Option<bool>>,
        /// Requests daemon to check for networks with same name.
        #[builder(into, default)]
        pub check_duplicate: pulumi_wasm_rust::Output<Option<bool>>,
        /// The driver of the Docker network. Possible values are `bridge`, `host`, `overlay`, `macvlan`. See [network docs](https://docs.docker.com/network/#network-drivers) for more details.
        #[builder(into, default)]
        pub driver: pulumi_wasm_rust::Output<Option<String>>,
        /// Create swarm routing-mesh network. Defaults to `false`.
        #[builder(into, default)]
        pub ingress: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether the network is internal.
        #[builder(into, default)]
        pub internal: pulumi_wasm_rust::Output<Option<bool>>,
        /// The IPAM configuration options
        #[builder(into, default)]
        pub ipam_configs: pulumi_wasm_rust::Output<
            Option<Vec<super::types::NetworkIpamConfig>>,
        >,
        /// Driver used by the custom IP scheme of the network. Defaults to `default`
        #[builder(into, default)]
        pub ipam_driver: pulumi_wasm_rust::Output<Option<String>>,
        /// Provide explicit options to the IPAM driver. Valid options vary with `ipam_driver` and refer to that driver's documentation for more details.
        #[builder(into, default)]
        pub ipam_options: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Enable IPv6 networking. Defaults to `false`.
        #[builder(into, default)]
        pub ipv6: pulumi_wasm_rust::Output<Option<bool>>,
        /// User-defined key/value metadata
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<Option<Vec<super::types::NetworkLabel>>>,
        /// The name of the Docker network.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Only available with bridge networks. See [bridge options docs](https://docs.docker.com/engine/reference/commandline/network_create/#bridge-driver-options) for more details.
        #[builder(into, default)]
        pub options: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
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
        pub ipam_configs: pulumi_wasm_rust::Output<Vec<super::types::NetworkIpamConfig>>,
        /// Driver used by the custom IP scheme of the network. Defaults to `default`
        pub ipam_driver: pulumi_wasm_rust::Output<Option<String>>,
        /// Provide explicit options to the IPAM driver. Valid options vary with `ipam_driver` and refer to that driver's documentation for more details.
        pub ipam_options: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Enable IPv6 networking. Defaults to `false`.
        pub ipv6: pulumi_wasm_rust::Output<Option<bool>>,
        /// User-defined key/value metadata
        pub labels: pulumi_wasm_rust::Output<Option<Vec<super::types::NetworkLabel>>>,
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
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: NetworkArgs) -> NetworkResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let attachable_binding = args.attachable.get_inner();
        let check_duplicate_binding = args.check_duplicate.get_inner();
        let driver_binding = args.driver.get_inner();
        let ingress_binding = args.ingress.get_inner();
        let internal_binding = args.internal.get_inner();
        let ipam_configs_binding = args.ipam_configs.get_inner();
        let ipam_driver_binding = args.ipam_driver.get_inner();
        let ipam_options_binding = args.ipam_options.get_inner();
        let ipv6_binding = args.ipv6.get_inner();
        let labels_binding = args.labels.get_inner();
        let name_binding = args.name.get_inner();
        let options_binding = args.options.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "docker:index/network:Network".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "attachable".into(),
                    value: &attachable_binding,
                },
                register_interface::ObjectField {
                    name: "checkDuplicate".into(),
                    value: &check_duplicate_binding,
                },
                register_interface::ObjectField {
                    name: "driver".into(),
                    value: &driver_binding,
                },
                register_interface::ObjectField {
                    name: "ingress".into(),
                    value: &ingress_binding,
                },
                register_interface::ObjectField {
                    name: "internal".into(),
                    value: &internal_binding,
                },
                register_interface::ObjectField {
                    name: "ipamConfigs".into(),
                    value: &ipam_configs_binding,
                },
                register_interface::ObjectField {
                    name: "ipamDriver".into(),
                    value: &ipam_driver_binding,
                },
                register_interface::ObjectField {
                    name: "ipamOptions".into(),
                    value: &ipam_options_binding,
                },
                register_interface::ObjectField {
                    name: "ipv6".into(),
                    value: &ipv6_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "options".into(),
                    value: &options_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "attachable".into(),
                },
                register_interface::ResultField {
                    name: "checkDuplicate".into(),
                },
                register_interface::ResultField {
                    name: "driver".into(),
                },
                register_interface::ResultField {
                    name: "ingress".into(),
                },
                register_interface::ResultField {
                    name: "internal".into(),
                },
                register_interface::ResultField {
                    name: "ipamConfigs".into(),
                },
                register_interface::ResultField {
                    name: "ipamDriver".into(),
                },
                register_interface::ResultField {
                    name: "ipamOptions".into(),
                },
                register_interface::ResultField {
                    name: "ipv6".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "options".into(),
                },
                register_interface::ResultField {
                    name: "scope".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NetworkResult {
            attachable: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("attachable").unwrap(),
            ),
            check_duplicate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("checkDuplicate").unwrap(),
            ),
            driver: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("driver").unwrap(),
            ),
            ingress: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ingress").unwrap(),
            ),
            internal: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("internal").unwrap(),
            ),
            ipam_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipamConfigs").unwrap(),
            ),
            ipam_driver: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipamDriver").unwrap(),
            ),
            ipam_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipamOptions").unwrap(),
            ),
            ipv6: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv6").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("options").unwrap(),
            ),
            scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scope").unwrap(),
            ),
        }
    }
}
pub mod plugin {
    //! <!-- Bug: Type and Name are switched -->
    //! Manages the lifecycle of a Docker plugin.
    //!
    //! ## Example Usage
    //!
    //! ```yaml
    //! resources:
    //!   sample-volume-plugin:
    //!     type: docker:Plugin
    //!     properties:
    //!       alias: sample-volume-plugin
    //!       enableTimeout: 60
    //!       enabled: false
    //!       envs:
    //!         - DEBUG=1
    //!       forceDestroy: true
    //!       forceDisable: true
    //!       grantAllPermissions: true
    //! ```
    //!
    //! ## Import
    //!
    //! #!/bin/bash
    //!
    //! ```sh
    //! $ pulumi import docker:index/plugin:Plugin sample-volume-plugin "$(docker plugin inspect -f {{.ID}} tiborvass/sample-volume-plugin:latest)"
    //! ```
    //!
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PluginArgs {
        /// Docker Plugin alias
        #[builder(into, default)]
        pub alias: pulumi_wasm_rust::Output<Option<String>>,
        /// HTTP client timeout to enable the plugin
        #[builder(into, default)]
        pub enable_timeout: pulumi_wasm_rust::Output<Option<i32>>,
        /// If `true` the plugin is enabled. Defaults to `true`
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The environment variables in the form of `KEY=VALUE`, e.g. `DEBUG=0`
        #[builder(into, default)]
        pub envs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// If true, then the plugin is destroyed forcibly
        #[builder(into, default)]
        pub force_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        /// If true, then the plugin is disabled forcibly
        #[builder(into, default)]
        pub force_disable: pulumi_wasm_rust::Output<Option<bool>>,
        /// If true, grant all permissions necessary to run the plugin
        #[builder(into, default)]
        pub grant_all_permissions: pulumi_wasm_rust::Output<Option<bool>>,
        /// Grant specific permissions only
        #[builder(into, default)]
        pub grant_permissions: pulumi_wasm_rust::Output<
            Option<Vec<super::types::PluginGrantPermission>>,
        >,
        /// The name of the permission
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
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
        pub grant_permissions: pulumi_wasm_rust::Output<
            Option<Vec<super::types::PluginGrantPermission>>,
        >,
        /// The name of the permission
        pub name: pulumi_wasm_rust::Output<String>,
        /// Docker Plugin Reference
        pub plugin_reference: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PluginArgs) -> PluginResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let alias_binding = args.alias.get_inner();
        let enable_timeout_binding = args.enable_timeout.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let envs_binding = args.envs.get_inner();
        let force_destroy_binding = args.force_destroy.get_inner();
        let force_disable_binding = args.force_disable.get_inner();
        let grant_all_permissions_binding = args.grant_all_permissions.get_inner();
        let grant_permissions_binding = args.grant_permissions.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "docker:index/plugin:Plugin".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "alias".into(),
                    value: &alias_binding,
                },
                register_interface::ObjectField {
                    name: "enableTimeout".into(),
                    value: &enable_timeout_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "envs".into(),
                    value: &envs_binding,
                },
                register_interface::ObjectField {
                    name: "forceDestroy".into(),
                    value: &force_destroy_binding,
                },
                register_interface::ObjectField {
                    name: "forceDisable".into(),
                    value: &force_disable_binding,
                },
                register_interface::ObjectField {
                    name: "grantAllPermissions".into(),
                    value: &grant_all_permissions_binding,
                },
                register_interface::ObjectField {
                    name: "grantPermissions".into(),
                    value: &grant_permissions_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "alias".into(),
                },
                register_interface::ResultField {
                    name: "enableTimeout".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "envs".into(),
                },
                register_interface::ResultField {
                    name: "forceDestroy".into(),
                },
                register_interface::ResultField {
                    name: "forceDisable".into(),
                },
                register_interface::ResultField {
                    name: "grantAllPermissions".into(),
                },
                register_interface::ResultField {
                    name: "grantPermissions".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "pluginReference".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PluginResult {
            alias: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alias").unwrap(),
            ),
            enable_timeout: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableTimeout").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            envs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("envs").unwrap(),
            ),
            force_destroy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("forceDestroy").unwrap(),
            ),
            force_disable: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("forceDisable").unwrap(),
            ),
            grant_all_permissions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("grantAllPermissions").unwrap(),
            ),
            grant_permissions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("grantPermissions").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            plugin_reference: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pluginReference").unwrap(),
            ),
        }
    }
}
pub mod registry_image {
    //! <!-- Bug: Type and Name are switched -->
    //! Manages the lifecycle of docker image in a registry. You can upload images to a registry (= `docker push`) and also delete them again
    //!
    //! ## Example Usage
    //!
    //! Build an image with the `docker.RemoteImage` resource and then push it to a registry:
    //!
    //! ```ignore
    //! use pulumi_wasm_rust::Output;
    //! use pulumi_wasm_rust::{add_export, pulumi_main};
    //! #[pulumi_main]
    //! fn test_main() -> Result<(), Error> {
    //!     let helloworld = registry_image::create(
    //!         "helloworld",
    //!         RegistryImageArgs::builder().keep_remotely(true).build_struct(),
    //!     );
    //!     let image = remote_image::create(
    //!         "image",
    //!         RemoteImageArgs::builder()
    //!             .build(
    //!                 RemoteImageBuild::builder()
    //!                     .context("${path.cwd}/absolutePathToContextFolder")
    //!                     .build_struct(),
    //!             )
    //!             .name("registry.com/somename:1.0")
    //!             .build_struct(),
    //!     );
    //! }
    //! ```
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegistryImageArgs {
        /// If `true`, the verification of TLS certificates of the server/registry is disabled. Defaults to `false`
        #[builder(into, default)]
        pub insecure_skip_verify: pulumi_wasm_rust::Output<Option<bool>>,
        /// If true, then the Docker image won't be deleted on destroy operation. If this is false, it will delete the image from the docker registry on destroy operation. Defaults to `false`
        #[builder(into, default)]
        pub keep_remotely: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the Docker image.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of arbitrary strings that, when changed, will force the `docker.RegistryImage` resource to be replaced. This can be used to repush a local image
        #[builder(into, default)]
        pub triggers: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
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
        pub triggers: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RegistryImageArgs) -> RegistryImageResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let insecure_skip_verify_binding = args.insecure_skip_verify.get_inner();
        let keep_remotely_binding = args.keep_remotely.get_inner();
        let name_binding = args.name.get_inner();
        let triggers_binding = args.triggers.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "docker:index/registryImage:RegistryImage".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "insecureSkipVerify".into(),
                    value: &insecure_skip_verify_binding,
                },
                register_interface::ObjectField {
                    name: "keepRemotely".into(),
                    value: &keep_remotely_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "triggers".into(),
                    value: &triggers_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "insecureSkipVerify".into(),
                },
                register_interface::ResultField {
                    name: "keepRemotely".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "sha256Digest".into(),
                },
                register_interface::ResultField {
                    name: "triggers".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RegistryImageResult {
            insecure_skip_verify: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("insecureSkipVerify").unwrap(),
            ),
            keep_remotely: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keepRemotely").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            sha256_digest: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sha256Digest").unwrap(),
            ),
            triggers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("triggers").unwrap(),
            ),
        }
    }
}
pub mod remote_image {
    //! <!-- Bug: Type and Name are switched -->
    //! Pulls a Docker image to a given Docker host from a Docker Registry.
    //!  This resource will *not* pull new layers of the image automatically unless used in conjunction with docker.RegistryImage data source to update the `pull_triggers` field.
    //!
    //! ## Example Usage
    //!
    //! ### Basic
    //!
    //! Finds and downloads the latest `ubuntu:precise` image but does not check
    //! for further updates of the image
    //!
    //! ```ignore
    //! use pulumi_wasm_rust::Output;
    //! use pulumi_wasm_rust::{add_export, pulumi_main};
    //! #[pulumi_main]
    //! fn test_main() -> Result<(), Error> {
    //!     let ubuntu = remote_image::create(
    //!         "ubuntu",
    //!         RemoteImageArgs::builder().name("ubuntu:precise").build_struct(),
    //!     );
    //! }
    //! ```
    //!
    //! ### Dynamic updates
    //!
    //! To be able to update an image dynamically when the `sha256` sum changes,
    //! you need to use it in combination with `docker.RegistryImage` as follows:
    //!
    //! ```ignore
    //! use pulumi_wasm_rust::Output;
    //! use pulumi_wasm_rust::{add_export, pulumi_main};
    //! #[pulumi_main]
    //! fn test_main() -> Result<(), Error> {
    //!     let ubuntuRegistryImage = get_registry_image::invoke(
    //!         GetRegistryImageArgs::builder().name("ubuntu:precise").build_struct(),
    //!     );
    //!     let ubuntuRemoteImage = remote_image::create(
    //!         "ubuntuRemoteImage",
    //!         RemoteImageArgs::builder()
    //!             .name("${ubuntuRegistryImage.name}")
    //!             .pull_triggers(vec!["${ubuntuRegistryImage.sha256Digest}",])
    //!             .build_struct(),
    //!     );
    //! }
    //! ```
    //!
    //! ### Build
    //!
    //! You can also use the resource to build an image.
    //! In this case the image "zoo" and "zoo:develop" are built.
    //!
    //! ```yaml
    //! resources:
    //!   zoo:
    //!     type: docker:RemoteImage
    //!     properties:
    //!       name: zoo
    //!       build:
    //!         context: .
    //!         tags:
    //!           - zoo:develop
    //!         buildArg:
    //!           foo: zoo
    //!         label:
    //!           author: zoo
    //! ```
    //!
    //! You can use the `triggers` argument to specify when the image should be rebuild. This is for example helpful when you want to rebuild the docker image whenever the source code changes.
    //!
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RemoteImageArgs {
        /// Configuration to build an image. Please see [docker build command reference](https://docs.docker.com/engine/reference/commandline/build/#options) too.
        #[builder(into, default)]
        pub build: pulumi_wasm_rust::Output<Option<super::types::RemoteImageBuild>>,
        /// Always remove intermediate containers
        #[builder(into, default)]
        pub force_remove: pulumi_wasm_rust::Output<Option<bool>>,
        /// If true, then the Docker image won't be deleted on destroy operation. If this is false, it will delete the image from the docker local storage on destroy operation.
        #[builder(into, default)]
        pub keep_locally: pulumi_wasm_rust::Output<Option<bool>>,
        /// type of ulimit, e.g. `nofile`
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Set platform if server is multi-platform capable
        #[builder(into, default)]
        pub platform: pulumi_wasm_rust::Output<Option<String>>,
        /// List of values which cause an image pull when changed. This is used to store the image digest from the registry when using the docker*registry*image.
        #[builder(into, default)]
        pub pull_triggers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A map of arbitrary strings that, when changed, will force the `docker.RemoteImage` resource to be replaced. This can be used to rebuild an image when contents of source code folders change
        #[builder(into, default)]
        pub triggers: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RemoteImageResult {
        /// Configuration to build an image. Please see [docker build command reference](https://docs.docker.com/engine/reference/commandline/build/#options) too.
        pub build: pulumi_wasm_rust::Output<Option<super::types::RemoteImageBuild>>,
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
        pub triggers: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RemoteImageArgs) -> RemoteImageResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let build_binding = args.build.get_inner();
        let force_remove_binding = args.force_remove.get_inner();
        let keep_locally_binding = args.keep_locally.get_inner();
        let name_binding = args.name.get_inner();
        let platform_binding = args.platform.get_inner();
        let pull_triggers_binding = args.pull_triggers.get_inner();
        let triggers_binding = args.triggers.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "docker:index/remoteImage:RemoteImage".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "build".into(),
                    value: &build_binding,
                },
                register_interface::ObjectField {
                    name: "forceRemove".into(),
                    value: &force_remove_binding,
                },
                register_interface::ObjectField {
                    name: "keepLocally".into(),
                    value: &keep_locally_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "platform".into(),
                    value: &platform_binding,
                },
                register_interface::ObjectField {
                    name: "pullTriggers".into(),
                    value: &pull_triggers_binding,
                },
                register_interface::ObjectField {
                    name: "triggers".into(),
                    value: &triggers_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "build".into(),
                },
                register_interface::ResultField {
                    name: "forceRemove".into(),
                },
                register_interface::ResultField {
                    name: "imageId".into(),
                },
                register_interface::ResultField {
                    name: "keepLocally".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "platform".into(),
                },
                register_interface::ResultField {
                    name: "pullTriggers".into(),
                },
                register_interface::ResultField {
                    name: "repoDigest".into(),
                },
                register_interface::ResultField {
                    name: "triggers".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RemoteImageResult {
            build: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("build").unwrap(),
            ),
            force_remove: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("forceRemove").unwrap(),
            ),
            image_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageId").unwrap(),
            ),
            keep_locally: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keepLocally").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            platform: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("platform").unwrap(),
            ),
            pull_triggers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pullTriggers").unwrap(),
            ),
            repo_digest: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("repoDigest").unwrap(),
            ),
            triggers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("triggers").unwrap(),
            ),
        }
    }
}
pub mod secret {
    //!
    //!
    //! ## Import
    //!
    //! ```sh
    //! # Docker secret cannot be imported as the secret data, once set, is never exposed again.
    //! ```
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SecretArgs {
        /// Base64-url-safe-encoded secret data
        #[builder(into)]
        pub data: pulumi_wasm_rust::Output<String>,
        /// User-defined key/value metadata
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<Option<Vec<super::types::SecretLabel>>>,
        /// User-defined name of the secret
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SecretResult {
        /// Base64-url-safe-encoded secret data
        pub data: pulumi_wasm_rust::Output<String>,
        /// User-defined key/value metadata
        pub labels: pulumi_wasm_rust::Output<Option<Vec<super::types::SecretLabel>>>,
        /// User-defined name of the secret
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SecretArgs) -> SecretResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let data_binding = args.data.get_inner();
        let labels_binding = args.labels.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "docker:index/secret:Secret".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "data".into(),
                    value: &data_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "data".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SecretResult {
            data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("data").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}
pub mod service {
    //! <!-- Bug: Type and Name are switched -->
    //! This resource manages the lifecycle of a Docker service. By default, the creation, update and delete of services are detached.
    //! With the Converge Config Name of the service
    //! - `task_spec` (Block List, Min: 1, Max: 1) User modifiable task configuration (see below for nested schema)
    //!
    //! ## Import
    //!
    //! ### Example
    //!
    //! Assuming you created a `service` as follows
    //!
    //! ```shell
    //! docker service create --name foo -p 8080:80 nginx
    //! ```
    //!
    //! prints this ID
    //!
    //! ```text
    //! 4pcphbxkfn2rffhbhe6czytgi
    //! ```
    //!
    //! you provide the definition for the resource as follows
    //!
    //! ```ignore
    //! use pulumi_wasm_rust::Output;
    //! use pulumi_wasm_rust::{add_export, pulumi_main};
    //! #[pulumi_main]
    //! fn test_main() -> Result<(), Error> {
    //!     let foo = service::create(
    //!         "foo",
    //!         ServiceArgs::builder()
    //!             .endpoint_spec(
    //!                 ServiceEndpointSpec::builder()
    //!                     .ports(
    //!                         vec![
    //!                             ServiceEndpointSpecPort::builder().publishedPort(8080)
    //!                             .targetPort(80).build_struct(),
    //!                         ],
    //!                     )
    //!                     .build_struct(),
    //!             )
    //!             .task_spec(
    //!                 ServiceTaskSpec::builder()
    //!                     .containerSpec(
    //!                         ServiceTaskSpecContainerSpec::builder()
    //!                             .image("nginx")
    //!                             .build_struct(),
    //!                     )
    //!                     .build_struct(),
    //!             )
    //!             .build_struct(),
    //!     );
    //! }
    //! ```
    //!
    //! then the import command is as follows
    //!
    //! ```sh
    //! $ pulumi import docker:index/service:Service foo 4pcphbxkfn2rffhbhe6czytgi
    //! ```
    //!
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceArgs {
        /// Configuration for the authentication for pulling the images of the service
        #[builder(into, default)]
        pub auth: pulumi_wasm_rust::Output<Option<super::types::ServiceAuth>>,
        /// A configuration to ensure that a service converges aka reaches the desired that of all task up and running
        #[builder(into, default)]
        pub converge_config: pulumi_wasm_rust::Output<
            Option<super::types::ServiceConvergeConfig>,
        >,
        /// Properties that can be configured to access and load balance a service
        #[builder(into, default)]
        pub endpoint_spec: pulumi_wasm_rust::Output<
            Option<super::types::ServiceEndpointSpec>,
        >,
        /// User-defined key/value metadata
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<Option<Vec<super::types::ServiceLabel>>>,
        /// Scheduling mode for the service
        #[builder(into, default)]
        pub mode: pulumi_wasm_rust::Output<Option<super::types::ServiceMode>>,
        /// Name of the service
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specification for the rollback strategy of the service
        #[builder(into, default)]
        pub rollback_config: pulumi_wasm_rust::Output<
            Option<super::types::ServiceRollbackConfig>,
        >,
        /// User modifiable task configuration
        #[builder(into)]
        pub task_spec: pulumi_wasm_rust::Output<super::types::ServiceTaskSpec>,
        /// Specification for the update strategy of the service
        #[builder(into, default)]
        pub update_config: pulumi_wasm_rust::Output<
            Option<super::types::ServiceUpdateConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct ServiceResult {
        /// Configuration for the authentication for pulling the images of the service
        pub auth: pulumi_wasm_rust::Output<Option<super::types::ServiceAuth>>,
        /// A configuration to ensure that a service converges aka reaches the desired that of all task up and running
        pub converge_config: pulumi_wasm_rust::Output<
            Option<super::types::ServiceConvergeConfig>,
        >,
        /// Properties that can be configured to access and load balance a service
        pub endpoint_spec: pulumi_wasm_rust::Output<super::types::ServiceEndpointSpec>,
        /// User-defined key/value metadata
        pub labels: pulumi_wasm_rust::Output<Vec<super::types::ServiceLabel>>,
        /// Scheduling mode for the service
        pub mode: pulumi_wasm_rust::Output<super::types::ServiceMode>,
        /// Name of the service
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specification for the rollback strategy of the service
        pub rollback_config: pulumi_wasm_rust::Output<
            Option<super::types::ServiceRollbackConfig>,
        >,
        /// User modifiable task configuration
        pub task_spec: pulumi_wasm_rust::Output<super::types::ServiceTaskSpec>,
        /// Specification for the update strategy of the service
        pub update_config: pulumi_wasm_rust::Output<
            Option<super::types::ServiceUpdateConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ServiceArgs) -> ServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let auth_binding = args.auth.get_inner();
        let converge_config_binding = args.converge_config.get_inner();
        let endpoint_spec_binding = args.endpoint_spec.get_inner();
        let labels_binding = args.labels.get_inner();
        let mode_binding = args.mode.get_inner();
        let name_binding = args.name.get_inner();
        let rollback_config_binding = args.rollback_config.get_inner();
        let task_spec_binding = args.task_spec.get_inner();
        let update_config_binding = args.update_config.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "docker:index/service:Service".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "auth".into(),
                    value: &auth_binding,
                },
                register_interface::ObjectField {
                    name: "convergeConfig".into(),
                    value: &converge_config_binding,
                },
                register_interface::ObjectField {
                    name: "endpointSpec".into(),
                    value: &endpoint_spec_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "mode".into(),
                    value: &mode_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "rollbackConfig".into(),
                    value: &rollback_config_binding,
                },
                register_interface::ObjectField {
                    name: "taskSpec".into(),
                    value: &task_spec_binding,
                },
                register_interface::ObjectField {
                    name: "updateConfig".into(),
                    value: &update_config_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "auth".into(),
                },
                register_interface::ResultField {
                    name: "convergeConfig".into(),
                },
                register_interface::ResultField {
                    name: "endpointSpec".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "mode".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "rollbackConfig".into(),
                },
                register_interface::ResultField {
                    name: "taskSpec".into(),
                },
                register_interface::ResultField {
                    name: "updateConfig".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ServiceResult {
            auth: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("auth").unwrap(),
            ),
            converge_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("convergeConfig").unwrap(),
            ),
            endpoint_spec: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpointSpec").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mode").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            rollback_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rollbackConfig").unwrap(),
            ),
            task_spec: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("taskSpec").unwrap(),
            ),
            update_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateConfig").unwrap(),
            ),
        }
    }
}
pub mod service_config {
    //!
    //!
    //! ## Import
    //!
    //! ### Example
    //!
    //! Assuming you created a `config` as follows
    //!
    //! ```shell
    //! printf '{"a":"b"}' | docker config create foo -
    //! ```
    //!
    //! prints the id
    //!
    //! ```text
    //! 08c26c477474478d971139f750984775a7f019dbe8a2e7f09d66a187c009e66d
    //! ```
    //!
    //! you provide the definition for the resource as follows
    //!
    //! ```ignore
    //! use pulumi_wasm_rust::Output;
    //! use pulumi_wasm_rust::{add_export, pulumi_main};
    //! #[pulumi_main]
    //! fn test_main() -> Result<(), Error> {
    //!     let foo = service_config::create(
    //!         "foo",
    //!         ServiceConfigArgs::builder()
    //!             .data("base64encode(\"{\\\"a\\\": \\\"b\\\"}\")")
    //!             .build_struct(),
    //!     );
    //! }
    //! ```
    //!
    //! then the import command is as follows
    //!
    //! ```sh
    //! $ pulumi import docker:index/serviceConfig:ServiceConfig foo 08c26c477474478d971139f750984775a7f019dbe8a2e7f09d66a187c009e66d
    //! ```
    //!
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceConfigArgs {
        /// Base64-url-safe-encoded config data
        #[builder(into)]
        pub data: pulumi_wasm_rust::Output<String>,
        /// User-defined name of the config
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ServiceConfigResult {
        /// Base64-url-safe-encoded config data
        pub data: pulumi_wasm_rust::Output<String>,
        /// User-defined name of the config
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ServiceConfigArgs) -> ServiceConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let data_binding = args.data.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "docker:index/serviceConfig:ServiceConfig".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "data".into(),
                    value: &data_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "data".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ServiceConfigResult {
            data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("data").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}
pub mod tag {
    //! Creates a docker tag. It has the exact same functionality as the `docker tag` command. Deleting the resource will neither delete the source nor target images. The source image must exist on the machine running the docker daemon.
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TagArgs {
        /// Name of the source image.
        #[builder(into)]
        pub source_image: pulumi_wasm_rust::Output<String>,
        /// Name of the target image.
        #[builder(into)]
        pub target_image: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
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
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TagArgs) -> TagResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let source_image_binding = args.source_image.get_inner();
        let target_image_binding = args.target_image.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "docker:index/tag:Tag".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "sourceImage".into(),
                    value: &source_image_binding,
                },
                register_interface::ObjectField {
                    name: "targetImage".into(),
                    value: &target_image_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "sourceImage".into(),
                },
                register_interface::ResultField {
                    name: "sourceImageId".into(),
                },
                register_interface::ResultField {
                    name: "targetImage".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TagResult {
            source_image: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceImage").unwrap(),
            ),
            source_image_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceImageId").unwrap(),
            ),
            target_image: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetImage").unwrap(),
            ),
        }
    }
}
pub mod volume {
    //! <!-- Bug: Type and Name are switched -->
    //! Creates and destroys a volume in Docker. This can be used alongside docker.Container to prepare volumes that can be shared across containers.
    //!
    //! ## Example Usage
    //!
    //! ```ignore
    //! use pulumi_wasm_rust::Output;
    //! use pulumi_wasm_rust::{add_export, pulumi_main};
    //! #[pulumi_main]
    //! fn test_main() -> Result<(), Error> {
    //!     let sharedVolume = volume::create(
    //!         "sharedVolume",
    //!         VolumeArgs::builder().build_struct(),
    //!     );
    //! }
    //! ```
    //!
    //! ## Import
    //!
    //! ### Example
    //!
    //! Assuming you created a `volume` as follows
    //!
    //! #!/bin/bash
    //!
    //! docker volume create
    //!
    //! prints the long ID
    //!
    //! 524b0457aa2a87dd2b75c74c3e4e53f406974249e63ab3ed9bf21e5644f9dc7d
    //!
    //! you provide the definition for the resource as follows
    //!
    //! terraform
    //!
    //! resource "docker_volume" "foo" {
    //!
    //!   name = "524b0457aa2a87dd2b75c74c3e4e53f406974249e63ab3ed9bf21e5644f9dc7d"
    //!
    //! }
    //!
    //! then the import command is as follows
    //!
    //! #!/bin/bash
    //!
    //! ```sh
    //! $ pulumi import docker:index/volume:Volume foo 524b0457aa2a87dd2b75c74c3e4e53f406974249e63ab3ed9bf21e5644f9dc7d
    //! ```
    //!
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VolumeArgs {
        /// Driver type for the volume. Defaults to `local`.
        #[builder(into, default)]
        pub driver: pulumi_wasm_rust::Output<Option<String>>,
        /// Options specific to the driver.
        #[builder(into, default)]
        pub driver_opts: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// User-defined key/value metadata
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<Option<Vec<super::types::VolumeLabel>>>,
        /// The name of the Docker volume (will be generated if not provided).
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct VolumeResult {
        /// Driver type for the volume. Defaults to `local`.
        pub driver: pulumi_wasm_rust::Output<String>,
        /// Options specific to the driver.
        pub driver_opts: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// User-defined key/value metadata
        pub labels: pulumi_wasm_rust::Output<Option<Vec<super::types::VolumeLabel>>>,
        /// The mountpoint of the volume.
        pub mountpoint: pulumi_wasm_rust::Output<String>,
        /// The name of the Docker volume (will be generated if not provided).
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: VolumeArgs) -> VolumeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let driver_binding = args.driver.get_inner();
        let driver_opts_binding = args.driver_opts.get_inner();
        let labels_binding = args.labels.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "docker:index/volume:Volume".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "driver".into(),
                    value: &driver_binding,
                },
                register_interface::ObjectField {
                    name: "driverOpts".into(),
                    value: &driver_opts_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "driver".into(),
                },
                register_interface::ResultField {
                    name: "driverOpts".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "mountpoint".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VolumeResult {
            driver: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("driver").unwrap(),
            ),
            driver_opts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("driverOpts").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            mountpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mountpoint").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}
pub mod functions {
    pub mod get_logs {
        #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
        #[builder(finish_fn = build_struct)]
        #[allow(dead_code)]
        pub struct GetLogsArgs {
            #[builder(into, default)]
            pub details: pulumi_wasm_rust::Output<Option<bool>>,
            /// Discard headers that docker appends to each log entry
            #[builder(into, default)]
            pub discard_headers: pulumi_wasm_rust::Output<Option<bool>>,
            #[builder(into, default)]
            pub follow: pulumi_wasm_rust::Output<Option<bool>>,
            /// If true populate computed value `logs_list_string`
            #[builder(into, default)]
            pub logs_list_string_enabled: pulumi_wasm_rust::Output<Option<bool>>,
            /// The name of the Docker Container
            #[builder(into)]
            pub name: pulumi_wasm_rust::Output<String>,
            #[builder(into, default)]
            pub show_stderr: pulumi_wasm_rust::Output<Option<bool>>,
            #[builder(into, default)]
            pub show_stdout: pulumi_wasm_rust::Output<Option<bool>>,
            #[builder(into, default)]
            pub since: pulumi_wasm_rust::Output<Option<String>>,
            #[builder(into, default)]
            pub tail: pulumi_wasm_rust::Output<Option<String>>,
            #[builder(into, default)]
            pub timestamps: pulumi_wasm_rust::Output<Option<bool>>,
            #[builder(into, default)]
            pub until: pulumi_wasm_rust::Output<Option<String>>,
        }
        #[allow(dead_code)]
        pub struct GetLogsResult {
            pub details: pulumi_wasm_rust::Output<Option<bool>>,
            /// Discard headers that docker appends to each log entry
            pub discard_headers: pulumi_wasm_rust::Output<Option<bool>>,
            pub follow: pulumi_wasm_rust::Output<Option<bool>>,
            /// The provider-assigned unique ID for this managed resource.
            pub id: pulumi_wasm_rust::Output<String>,
            /// If true populate computed value `logs_list_string`
            pub logs_list_string_enabled: pulumi_wasm_rust::Output<Option<bool>>,
            /// List of container logs, each element is a line.
            pub logs_list_strings: pulumi_wasm_rust::Output<Vec<String>>,
            /// The name of the Docker Container
            pub name: pulumi_wasm_rust::Output<String>,
            pub show_stderr: pulumi_wasm_rust::Output<Option<bool>>,
            pub show_stdout: pulumi_wasm_rust::Output<Option<bool>>,
            pub since: pulumi_wasm_rust::Output<Option<String>>,
            pub tail: pulumi_wasm_rust::Output<Option<String>>,
            pub timestamps: pulumi_wasm_rust::Output<Option<bool>>,
            pub until: pulumi_wasm_rust::Output<Option<String>>,
        }
        ///
        /// Registers a new resource with the given unique name and arguments
        ///
        #[allow(non_snake_case, unused_imports, dead_code)]
        pub fn invoke(args: GetLogsArgs) -> GetLogsResult {
            use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
            use std::collections::HashMap;
            let details_binding = args.details.get_inner();
            let discard_headers_binding = args.discard_headers.get_inner();
            let follow_binding = args.follow.get_inner();
            let logs_list_string_enabled_binding = args
                .logs_list_string_enabled
                .get_inner();
            let name_binding = args.name.get_inner();
            let show_stderr_binding = args.show_stderr.get_inner();
            let show_stdout_binding = args.show_stdout.get_inner();
            let since_binding = args.since.get_inner();
            let tail_binding = args.tail.get_inner();
            let timestamps_binding = args.timestamps.get_inner();
            let until_binding = args.until.get_inner();
            let request = register_interface::ResourceInvokeRequest {
                token: "docker:index/getLogs:getLogs".into(),
                object: Vec::from([
                    register_interface::ObjectField {
                        name: "details".into(),
                        value: &details_binding,
                    },
                    register_interface::ObjectField {
                        name: "discardHeaders".into(),
                        value: &discard_headers_binding,
                    },
                    register_interface::ObjectField {
                        name: "follow".into(),
                        value: &follow_binding,
                    },
                    register_interface::ObjectField {
                        name: "logsListStringEnabled".into(),
                        value: &logs_list_string_enabled_binding,
                    },
                    register_interface::ObjectField {
                        name: "name".into(),
                        value: &name_binding,
                    },
                    register_interface::ObjectField {
                        name: "showStderr".into(),
                        value: &show_stderr_binding,
                    },
                    register_interface::ObjectField {
                        name: "showStdout".into(),
                        value: &show_stdout_binding,
                    },
                    register_interface::ObjectField {
                        name: "since".into(),
                        value: &since_binding,
                    },
                    register_interface::ObjectField {
                        name: "tail".into(),
                        value: &tail_binding,
                    },
                    register_interface::ObjectField {
                        name: "timestamps".into(),
                        value: &timestamps_binding,
                    },
                    register_interface::ObjectField {
                        name: "until".into(),
                        value: &until_binding,
                    },
                ]),
                results: Vec::from([
                    register_interface::ResultField {
                        name: "details".into(),
                    },
                    register_interface::ResultField {
                        name: "discardHeaders".into(),
                    },
                    register_interface::ResultField {
                        name: "follow".into(),
                    },
                    register_interface::ResultField {
                        name: "id".into(),
                    },
                    register_interface::ResultField {
                        name: "logsListStringEnabled".into(),
                    },
                    register_interface::ResultField {
                        name: "logsListStrings".into(),
                    },
                    register_interface::ResultField {
                        name: "name".into(),
                    },
                    register_interface::ResultField {
                        name: "showStderr".into(),
                    },
                    register_interface::ResultField {
                        name: "showStdout".into(),
                    },
                    register_interface::ResultField {
                        name: "since".into(),
                    },
                    register_interface::ResultField {
                        name: "tail".into(),
                    },
                    register_interface::ResultField {
                        name: "timestamps".into(),
                    },
                    register_interface::ResultField {
                        name: "until".into(),
                    },
                ]),
            };
            let o = register_interface::invoke(&request);
            let mut hashmap: HashMap<String, _> = o
                .fields
                .into_iter()
                .map(|f| (f.name, f.output))
                .collect();
            GetLogsResult {
                details: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("details").unwrap(),
                ),
                discard_headers: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("discardHeaders").unwrap(),
                ),
                follow: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("follow").unwrap(),
                ),
                id: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("id").unwrap(),
                ),
                logs_list_string_enabled: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("logsListStringEnabled").unwrap(),
                ),
                logs_list_strings: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("logsListStrings").unwrap(),
                ),
                name: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("name").unwrap(),
                ),
                show_stderr: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("showStderr").unwrap(),
                ),
                show_stdout: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("showStdout").unwrap(),
                ),
                since: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("since").unwrap(),
                ),
                tail: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("tail").unwrap(),
                ),
                timestamps: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("timestamps").unwrap(),
                ),
                until: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("until").unwrap(),
                ),
            }
        }
    }
    pub mod get_network {
        #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
        #[builder(finish_fn = build_struct)]
        #[allow(dead_code)]
        pub struct GetNetworkArgs {
            /// The name of the Docker network.
            #[builder(into)]
            pub name: pulumi_wasm_rust::Output<String>,
        }
        #[allow(dead_code)]
        pub struct GetNetworkResult {
            /// The driver of the Docker network. Possible values are `bridge`, `host`, `overlay`, `macvlan`. See [network docs](https://docs.docker.com/network/#network-drivers) for more details.
            pub driver: pulumi_wasm_rust::Output<String>,
            /// The ID of this resource.
            pub id: pulumi_wasm_rust::Output<String>,
            /// If `true`, the network is internal.
            pub internal: pulumi_wasm_rust::Output<bool>,
            /// The IPAM configuration options
            pub ipam_configs: pulumi_wasm_rust::Output<
                Vec<super::super::types::GetNetworkIpamConfig>,
            >,
            /// The name of the Docker network.
            pub name: pulumi_wasm_rust::Output<String>,
            /// Only available with bridge networks. See [bridge options docs](https://docs.docker.com/engine/reference/commandline/network_create/#bridge-driver-options) for more details.
            pub options: pulumi_wasm_rust::Output<
                std::collections::HashMap<String, String>,
            >,
            /// Scope of the network. One of `swarm`, `global`, or `local`.
            pub scope: pulumi_wasm_rust::Output<String>,
        }
        ///
        /// Registers a new resource with the given unique name and arguments
        ///
        #[allow(non_snake_case, unused_imports, dead_code)]
        pub fn invoke(args: GetNetworkArgs) -> GetNetworkResult {
            use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
            use std::collections::HashMap;
            let name_binding = args.name.get_inner();
            let request = register_interface::ResourceInvokeRequest {
                token: "docker:index/getNetwork:getNetwork".into(),
                object: Vec::from([
                    register_interface::ObjectField {
                        name: "name".into(),
                        value: &name_binding,
                    },
                ]),
                results: Vec::from([
                    register_interface::ResultField {
                        name: "driver".into(),
                    },
                    register_interface::ResultField {
                        name: "id".into(),
                    },
                    register_interface::ResultField {
                        name: "internal".into(),
                    },
                    register_interface::ResultField {
                        name: "ipamConfigs".into(),
                    },
                    register_interface::ResultField {
                        name: "name".into(),
                    },
                    register_interface::ResultField {
                        name: "options".into(),
                    },
                    register_interface::ResultField {
                        name: "scope".into(),
                    },
                ]),
            };
            let o = register_interface::invoke(&request);
            let mut hashmap: HashMap<String, _> = o
                .fields
                .into_iter()
                .map(|f| (f.name, f.output))
                .collect();
            GetNetworkResult {
                driver: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("driver").unwrap(),
                ),
                id: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("id").unwrap(),
                ),
                internal: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("internal").unwrap(),
                ),
                ipam_configs: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("ipamConfigs").unwrap(),
                ),
                name: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("name").unwrap(),
                ),
                options: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("options").unwrap(),
                ),
                scope: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("scope").unwrap(),
                ),
            }
        }
    }
    pub mod get_plugin {
        #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
        #[builder(finish_fn = build_struct)]
        #[allow(dead_code)]
        pub struct GetPluginArgs {
            /// The alias of the Docker plugin. If the tag is omitted, `:latest` is complemented to the attribute value.
            #[builder(into, default)]
            pub alias: pulumi_wasm_rust::Output<Option<String>>,
            /// The ID of the plugin, which has precedence over the `alias` of both are given
            #[builder(into, default)]
            pub id: pulumi_wasm_rust::Output<Option<String>>,
        }
        #[allow(dead_code)]
        pub struct GetPluginResult {
            /// The alias of the Docker plugin. If the tag is omitted, `:latest` is complemented to the attribute value.
            pub alias: pulumi_wasm_rust::Output<Option<String>>,
            /// If `true` the plugin is enabled
            pub enabled: pulumi_wasm_rust::Output<bool>,
            /// The environment variables in the form of `KEY=VALUE`, e.g. `DEBUG=0`
            pub envs: pulumi_wasm_rust::Output<Vec<String>>,
            /// If true, grant all permissions necessary to run the plugin
            pub grant_all_permissions: pulumi_wasm_rust::Output<bool>,
            /// The ID of the plugin, which has precedence over the `alias` of both are given
            pub id: pulumi_wasm_rust::Output<Option<String>>,
            /// The plugin name. If the tag is omitted, `:latest` is complemented to the attribute value.
            pub name: pulumi_wasm_rust::Output<String>,
            /// The Docker Plugin Reference
            pub plugin_reference: pulumi_wasm_rust::Output<String>,
        }
        ///
        /// Registers a new resource with the given unique name and arguments
        ///
        #[allow(non_snake_case, unused_imports, dead_code)]
        pub fn invoke(args: GetPluginArgs) -> GetPluginResult {
            use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
            use std::collections::HashMap;
            let alias_binding = args.alias.get_inner();
            let id_binding = args.id.get_inner();
            let request = register_interface::ResourceInvokeRequest {
                token: "docker:index/getPlugin:getPlugin".into(),
                object: Vec::from([
                    register_interface::ObjectField {
                        name: "alias".into(),
                        value: &alias_binding,
                    },
                    register_interface::ObjectField {
                        name: "id".into(),
                        value: &id_binding,
                    },
                ]),
                results: Vec::from([
                    register_interface::ResultField {
                        name: "alias".into(),
                    },
                    register_interface::ResultField {
                        name: "enabled".into(),
                    },
                    register_interface::ResultField {
                        name: "envs".into(),
                    },
                    register_interface::ResultField {
                        name: "grantAllPermissions".into(),
                    },
                    register_interface::ResultField {
                        name: "id".into(),
                    },
                    register_interface::ResultField {
                        name: "name".into(),
                    },
                    register_interface::ResultField {
                        name: "pluginReference".into(),
                    },
                ]),
            };
            let o = register_interface::invoke(&request);
            let mut hashmap: HashMap<String, _> = o
                .fields
                .into_iter()
                .map(|f| (f.name, f.output))
                .collect();
            GetPluginResult {
                alias: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("alias").unwrap(),
                ),
                enabled: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("enabled").unwrap(),
                ),
                envs: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("envs").unwrap(),
                ),
                grant_all_permissions: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("grantAllPermissions").unwrap(),
                ),
                id: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("id").unwrap(),
                ),
                name: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("name").unwrap(),
                ),
                plugin_reference: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("pluginReference").unwrap(),
                ),
            }
        }
    }
    pub mod get_registry_image {
        #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
        #[builder(finish_fn = build_struct)]
        #[allow(dead_code)]
        pub struct GetRegistryImageArgs {
            /// If `true`, the verification of TLS certificates of the server/registry is disabled. Defaults to `false`
            #[builder(into, default)]
            pub insecure_skip_verify: pulumi_wasm_rust::Output<Option<bool>>,
            /// The name of the Docker image, including any tags. e.g. `alpine:latest`
            #[builder(into)]
            pub name: pulumi_wasm_rust::Output<String>,
        }
        #[allow(dead_code)]
        pub struct GetRegistryImageResult {
            /// The provider-assigned unique ID for this managed resource.
            pub id: pulumi_wasm_rust::Output<String>,
            /// If `true`, the verification of TLS certificates of the server/registry is disabled. Defaults to `false`
            pub insecure_skip_verify: pulumi_wasm_rust::Output<Option<bool>>,
            /// The name of the Docker image, including any tags. e.g. `alpine:latest`
            pub name: pulumi_wasm_rust::Output<String>,
            /// The content digest of the image, as stored in the registry.
            pub sha256_digest: pulumi_wasm_rust::Output<String>,
        }
        ///
        /// Registers a new resource with the given unique name and arguments
        ///
        #[allow(non_snake_case, unused_imports, dead_code)]
        pub fn invoke(args: GetRegistryImageArgs) -> GetRegistryImageResult {
            use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
            use std::collections::HashMap;
            let insecure_skip_verify_binding = args.insecure_skip_verify.get_inner();
            let name_binding = args.name.get_inner();
            let request = register_interface::ResourceInvokeRequest {
                token: "docker:index/getRegistryImage:getRegistryImage".into(),
                object: Vec::from([
                    register_interface::ObjectField {
                        name: "insecureSkipVerify".into(),
                        value: &insecure_skip_verify_binding,
                    },
                    register_interface::ObjectField {
                        name: "name".into(),
                        value: &name_binding,
                    },
                ]),
                results: Vec::from([
                    register_interface::ResultField {
                        name: "id".into(),
                    },
                    register_interface::ResultField {
                        name: "insecureSkipVerify".into(),
                    },
                    register_interface::ResultField {
                        name: "name".into(),
                    },
                    register_interface::ResultField {
                        name: "sha256Digest".into(),
                    },
                ]),
            };
            let o = register_interface::invoke(&request);
            let mut hashmap: HashMap<String, _> = o
                .fields
                .into_iter()
                .map(|f| (f.name, f.output))
                .collect();
            GetRegistryImageResult {
                id: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("id").unwrap(),
                ),
                insecure_skip_verify: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("insecureSkipVerify").unwrap(),
                ),
                name: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("name").unwrap(),
                ),
                sha256_digest: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("sha256Digest").unwrap(),
                ),
            }
        }
    }
    pub mod get_remote_image {
        #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
        #[builder(finish_fn = build_struct)]
        #[allow(dead_code)]
        pub struct GetRemoteImageArgs {
            /// The name of the Docker image, including any tags or SHA256 repo digests.
            #[builder(into)]
            pub name: pulumi_wasm_rust::Output<String>,
        }
        #[allow(dead_code)]
        pub struct GetRemoteImageResult {
            /// The provider-assigned unique ID for this managed resource.
            pub id: pulumi_wasm_rust::Output<String>,
            /// The name of the Docker image, including any tags or SHA256 repo digests.
            pub name: pulumi_wasm_rust::Output<String>,
            /// The image sha256 digest in the form of `repo[:tag]@sha256:<hash>`. It may be empty in the edge case where the local image was pulled from a repo, tagged locally, and then referred to in the data source by that local name/tag.
            pub repo_digest: pulumi_wasm_rust::Output<String>,
        }
        ///
        /// Registers a new resource with the given unique name and arguments
        ///
        #[allow(non_snake_case, unused_imports, dead_code)]
        pub fn invoke(args: GetRemoteImageArgs) -> GetRemoteImageResult {
            use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
            use std::collections::HashMap;
            let name_binding = args.name.get_inner();
            let request = register_interface::ResourceInvokeRequest {
                token: "docker:index/getRemoteImage:getRemoteImage".into(),
                object: Vec::from([
                    register_interface::ObjectField {
                        name: "name".into(),
                        value: &name_binding,
                    },
                ]),
                results: Vec::from([
                    register_interface::ResultField {
                        name: "id".into(),
                    },
                    register_interface::ResultField {
                        name: "name".into(),
                    },
                    register_interface::ResultField {
                        name: "repoDigest".into(),
                    },
                ]),
            };
            let o = register_interface::invoke(&request);
            let mut hashmap: HashMap<String, _> = o
                .fields
                .into_iter()
                .map(|f| (f.name, f.output))
                .collect();
            GetRemoteImageResult {
                id: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("id").unwrap(),
                ),
                name: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("name").unwrap(),
                ),
                repo_digest: pulumi_wasm_rust::__private::into_domain(
                    hashmap.remove("repoDigest").unwrap(),
                ),
            }
        }
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
    pulumi_wasm_rust::__private::wit_bindgen::generate!(
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
        pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface
        } }
    );
}
