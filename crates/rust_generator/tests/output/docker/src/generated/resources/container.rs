/// <!-- Bug: Type and Name are switched -->
/// Manages the lifecycle of a Docker container.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod container {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ContainerArgs {
        /// If `true` attach to the container after its creation and waits the end of its execution. Defaults to `false`.
        #[builder(into, default)]
        pub attach: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Add or drop certrain linux capabilities.
        #[builder(into, default)]
        pub capabilities: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::ContainerCapabilities>,
        >,
        /// Cgroup namespace mode to use for the container. Possible values are: `private`, `host`.
        #[builder(into, default)]
        pub cgroupns_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The command to use to start the container. For example, to run `/usr/bin/myprogram -f baz.conf` set the command to be `["/usr/bin/myprogram","-f","baz.con"]`.
        #[builder(into, default)]
        pub command: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The total number of milliseconds to wait for the container to reach status 'running'
        #[builder(into, default)]
        pub container_read_refresh_timeout_milliseconds: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// A comma-separated list or hyphen-separated range of CPUs a container can use, e.g. `0-1`.
        #[builder(into, default)]
        pub cpu_set: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// CPU shares (relative weight) for the container.
        #[builder(into, default)]
        pub cpu_shares: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// If defined will attempt to stop the container before destroying. Container will be destroyed after `n` seconds or on successful stop.
        #[builder(into, default)]
        pub destroy_grace_seconds: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Bind devices to the container.
        #[builder(into, default)]
        pub devices: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::ContainerDevice>>,
        >,
        /// DNS servers to use.
        #[builder(into, default)]
        pub dns: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// DNS options used by the DNS provider(s), see `resolv.conf` documentation for valid list of options.
        #[builder(into, default)]
        pub dns_opts: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// DNS search domains that are used when bare unqualified hostnames are used inside of the container.
        #[builder(into, default)]
        pub dns_searches: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Domain name of the container.
        #[builder(into, default)]
        pub domainname: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The command to use as the Entrypoint for the container. The Entrypoint allows you to configure a container to run as an executable. For example, to run `/usr/bin/myprogram` when starting a container, set the entrypoint to be `"/usr/bin/myprogra"]`.
        #[builder(into, default)]
        pub entrypoints: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Environment variables to set in the form of `KEY=VALUE`, e.g. `DEBUG=0`
        #[builder(into, default)]
        pub envs: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// GPU devices to add to the container. Currently, only the value `all` is supported. Passing any other value will result in unexpected behavior.
        #[builder(into, default)]
        pub gpus: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Additional groups for the container user
        #[builder(into, default)]
        pub group_adds: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A test to perform to check that the container is healthy
        #[builder(into, default)]
        pub healthcheck: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::ContainerHealthcheck>,
        >,
        /// Hostname of the container.
        #[builder(into, default)]
        pub hostname: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Hostname to add
        #[builder(into, default)]
        pub hosts: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::ContainerHost>>,
        >,
        /// The ID of the image to back this container. The easiest way to get this value is to use the `docker.RemoteImage` resource as is shown in the example.
        #[builder(into)]
        pub image: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configured whether an init process should be injected for this container. If unset this will default to the `dockerd` defaults.
        #[builder(into, default)]
        pub init: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// IPC sharing mode for the container. Possible values are: `none`, `private`, `shareable`, `container:<name|id>` or `host`.
        #[builder(into, default)]
        pub ipc_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// User-defined key/value metadata.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::ContainerLabel>>,
        >,
        /// The logging driver to use for the container.
        #[builder(into, default)]
        pub log_driver: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key/value pairs to use as options for the logging driver.
        #[builder(into, default)]
        pub log_opts: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Save the container logs (`attach` must be enabled). Defaults to `false`.
        #[builder(into, default)]
        pub logs: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The maximum amount of times to an attempt a restart when `restart` is set to 'on-failure'.
        #[builder(into, default)]
        pub max_retry_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The memory limit for the container in MBs.
        #[builder(into, default)]
        pub memory: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The total memory limit (memory + swap) for the container in MBs. This setting may compute to `-1` after `pulumi up` if the target host doesn't support memory swap, when that is the case docker will use a soft limitation.
        #[builder(into, default)]
        pub memory_swap: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specification for mounts to be added to containers created as part of the service.
        #[builder(into, default)]
        pub mounts: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::ContainerMount>>,
        >,
        /// If `true`, then the Docker container will be kept running. If `false`, then as long as the container exists, Terraform
        /// assumes it is successful. Defaults to `true`.
        #[builder(into, default)]
        pub must_run: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name or id of the network to use. You can use `name` or `id` attribute from a `docker.Network` resource.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Network mode of the container.
        #[builder(into, default)]
        pub network_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The networks the container is attached to
        #[builder(into, default)]
        pub networks_advanced: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::ContainerNetworksAdvanced>>,
        >,
        /// he PID (Process) Namespace mode for the container. Either `container:<name|id>` or `host`.
        #[builder(into, default)]
        pub pid_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Publish a container's port(s) to the host.
        #[builder(into, default)]
        pub ports: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::ContainerPort>>,
        >,
        /// If `true`, the container runs in privileged mode.
        #[builder(into, default)]
        pub privileged: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Publish all ports of the container.
        #[builder(into, default)]
        pub publish_all_ports: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether the mount should be read-only.
        #[builder(into, default)]
        pub read_only: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// If `true`, it will remove anonymous volumes associated with the container. Defaults to `true`.
        #[builder(into, default)]
        pub remove_volumes: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The restart policy for the container. Must be one of 'no', 'on-failure', 'always', 'unless-stopped'. Defaults to `no`.
        #[builder(into, default)]
        pub restart: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If `true`, then the container will be automatically removed when it exits. Defaults to `false`.
        #[builder(into, default)]
        pub rm: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Runtime to use for the container.
        #[builder(into, default)]
        pub runtime: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of string values to customize labels for MLS systems, such as SELinux. See https://docs.docker.com/engine/reference/run/#security-configuration.
        #[builder(into, default)]
        pub security_opts: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Size of `/dev/shm` in MBs.
        #[builder(into, default)]
        pub shm_size: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// If `true`, then the Docker container will be started after creation. If `false`, then the container is only created. Defaults to `true`.
        #[builder(into, default)]
        pub start: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// If `true`, keep STDIN open even if not attached (`docker run -i`). Defaults to `false`.
        #[builder(into, default)]
        pub stdin_open: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Signal to stop a container (default `SIGTERM`).
        #[builder(into, default)]
        pub stop_signal: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Timeout (in seconds) to stop a container.
        #[builder(into, default)]
        pub stop_timeout: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Key/value pairs for the storage driver options, e.g. `size`: `120G`
        #[builder(into, default)]
        pub storage_opts: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of kernel parameters (sysctls) to set in the container.
        #[builder(into, default)]
        pub sysctls: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of container directories which should be replaced by `tmpfs mounts`, and their corresponding mount options.
        #[builder(into, default)]
        pub tmpfs: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// If `true`, allocate a pseudo-tty (`docker run -t`). Defaults to `false`.
        #[builder(into, default)]
        pub tty: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Ulimit options to add.
        #[builder(into, default)]
        pub ulimits: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::ContainerUlimit>>,
        >,
        /// Specifies files to upload to the container before starting it. Only one of `content` or `content_base64` can be set and at least one of them has to be set.
        #[builder(into, default)]
        pub uploads: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::ContainerUpload>>,
        >,
        /// User used for run the first process. Format is `user` or `user:group` which user and group can be passed literraly or by name.
        #[builder(into, default)]
        pub user: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Sets the usernamespace mode for the container when usernamespace remapping option is enabled.
        #[builder(into, default)]
        pub userns_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Spec for mounting volumes in the container.
        #[builder(into, default)]
        pub volumes: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::ContainerVolume>>,
        >,
        /// If `true`, then the Docker container is waited for being healthy state after creation. If `false`, then the container health state is not checked. Defaults to `false`.
        #[builder(into, default)]
        pub wait: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The timeout in seconds to wait the container to be healthy after creation. Defaults to `60`.
        #[builder(into, default)]
        pub wait_timeout: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The working directory for commands to run in.
        #[builder(into, default)]
        pub working_dir: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ContainerResult {
        /// If `true` attach to the container after its creation and waits the end of its execution. Defaults to `false`.
        pub attach: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The network bridge of the container as read from its NetworkSettings.
        pub bridge: pulumi_gestalt_rust::Output<String>,
        /// Add or drop certrain linux capabilities.
        pub capabilities: pulumi_gestalt_rust::Output<
            Option<super::types::ContainerCapabilities>,
        >,
        /// Cgroup namespace mode to use for the container. Possible values are: `private`, `host`.
        pub cgroupns_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// The command to use to start the container. For example, to run `/usr/bin/myprogram -f baz.conf` set the command to be `["/usr/bin/myprogram","-f","baz.con"]`.
        pub command: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The logs of the container if its execution is done (`attach` must be disabled).
        pub container_logs: pulumi_gestalt_rust::Output<Option<String>>,
        /// The total number of milliseconds to wait for the container to reach status 'running'
        pub container_read_refresh_timeout_milliseconds: pulumi_gestalt_rust::Output<
            Option<i32>,
        >,
        /// A comma-separated list or hyphen-separated range of CPUs a container can use, e.g. `0-1`.
        pub cpu_set: pulumi_gestalt_rust::Output<Option<String>>,
        /// CPU shares (relative weight) for the container.
        pub cpu_shares: pulumi_gestalt_rust::Output<Option<i32>>,
        /// If defined will attempt to stop the container before destroying. Container will be destroyed after `n` seconds or on successful stop.
        pub destroy_grace_seconds: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Bind devices to the container.
        pub devices: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::ContainerDevice>>,
        >,
        /// DNS servers to use.
        pub dns: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// DNS options used by the DNS provider(s), see `resolv.conf` documentation for valid list of options.
        pub dns_opts: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// DNS search domains that are used when bare unqualified hostnames are used inside of the container.
        pub dns_searches: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Domain name of the container.
        pub domainname: pulumi_gestalt_rust::Output<Option<String>>,
        /// The command to use as the Entrypoint for the container. The Entrypoint allows you to configure a container to run as an executable. For example, to run `/usr/bin/myprogram` when starting a container, set the entrypoint to be `"/usr/bin/myprogra"]`.
        pub entrypoints: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Environment variables to set in the form of `KEY=VALUE`, e.g. `DEBUG=0`
        pub envs: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The exit code of the container if its execution is done (`must_run` must be disabled).
        pub exit_code: pulumi_gestalt_rust::Output<i32>,
        /// GPU devices to add to the container. Currently, only the value `all` is supported. Passing any other value will result in unexpected behavior.
        pub gpus: pulumi_gestalt_rust::Output<Option<String>>,
        /// Additional groups for the container user
        pub group_adds: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// A test to perform to check that the container is healthy
        pub healthcheck: pulumi_gestalt_rust::Output<
            Option<super::types::ContainerHealthcheck>,
        >,
        /// Hostname of the container.
        pub hostname: pulumi_gestalt_rust::Output<String>,
        /// Hostname to add
        pub hosts: pulumi_gestalt_rust::Output<Option<Vec<super::types::ContainerHost>>>,
        /// The ID of the image to back this container. The easiest way to get this value is to use the `docker.RemoteImage` resource as is shown in the example.
        pub image: pulumi_gestalt_rust::Output<String>,
        /// Configured whether an init process should be injected for this container. If unset this will default to the `dockerd` defaults.
        pub init: pulumi_gestalt_rust::Output<bool>,
        /// IPC sharing mode for the container. Possible values are: `none`, `private`, `shareable`, `container:<name|id>` or `host`.
        pub ipc_mode: pulumi_gestalt_rust::Output<String>,
        /// User-defined key/value metadata.
        pub labels: pulumi_gestalt_rust::Output<Vec<super::types::ContainerLabel>>,
        /// The logging driver to use for the container.
        pub log_driver: pulumi_gestalt_rust::Output<String>,
        /// Key/value pairs to use as options for the logging driver.
        pub log_opts: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Save the container logs (`attach` must be enabled). Defaults to `false`.
        pub logs: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The maximum amount of times to an attempt a restart when `restart` is set to 'on-failure'.
        pub max_retry_count: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The memory limit for the container in MBs.
        pub memory: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The total memory limit (memory + swap) for the container in MBs. This setting may compute to `-1` after `pulumi up` if the target host doesn't support memory swap, when that is the case docker will use a soft limitation.
        pub memory_swap: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Specification for mounts to be added to containers created as part of the service.
        pub mounts: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::ContainerMount>>,
        >,
        /// If `true`, then the Docker container will be kept running. If `false`, then as long as the container exists, Terraform
        /// assumes it is successful. Defaults to `true`.
        pub must_run: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name or id of the network to use. You can use `name` or `id` attribute from a `docker.Network` resource.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The data of the networks the container is connected to.
        pub network_datas: pulumi_gestalt_rust::Output<
            Vec<super::types::ContainerNetworkData>,
        >,
        /// Network mode of the container.
        pub network_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// The networks the container is attached to
        pub networks_advanced: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::ContainerNetworksAdvanced>>,
        >,
        /// he PID (Process) Namespace mode for the container. Either `container:<name|id>` or `host`.
        pub pid_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// Publish a container's port(s) to the host.
        pub ports: pulumi_gestalt_rust::Output<Option<Vec<super::types::ContainerPort>>>,
        /// If `true`, the container runs in privileged mode.
        pub privileged: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Publish all ports of the container.
        pub publish_all_ports: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether the mount should be read-only.
        pub read_only: pulumi_gestalt_rust::Output<Option<bool>>,
        /// If `true`, it will remove anonymous volumes associated with the container. Defaults to `true`.
        pub remove_volumes: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The restart policy for the container. Must be one of 'no', 'on-failure', 'always', 'unless-stopped'. Defaults to `no`.
        pub restart: pulumi_gestalt_rust::Output<Option<String>>,
        /// If `true`, then the container will be automatically removed when it exits. Defaults to `false`.
        pub rm: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Runtime to use for the container.
        pub runtime: pulumi_gestalt_rust::Output<String>,
        /// List of string values to customize labels for MLS systems, such as SELinux. See https://docs.docker.com/engine/reference/run/#security-configuration.
        pub security_opts: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Size of `/dev/shm` in MBs.
        pub shm_size: pulumi_gestalt_rust::Output<i32>,
        /// If `true`, then the Docker container will be started after creation. If `false`, then the container is only created. Defaults to `true`.
        pub start: pulumi_gestalt_rust::Output<Option<bool>>,
        /// If `true`, keep STDIN open even if not attached (`docker run -i`). Defaults to `false`.
        pub stdin_open: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Signal to stop a container (default `SIGTERM`).
        pub stop_signal: pulumi_gestalt_rust::Output<String>,
        /// Timeout (in seconds) to stop a container.
        pub stop_timeout: pulumi_gestalt_rust::Output<i32>,
        /// Key/value pairs for the storage driver options, e.g. `size`: `120G`
        pub storage_opts: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of kernel parameters (sysctls) to set in the container.
        pub sysctls: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of container directories which should be replaced by `tmpfs mounts`, and their corresponding mount options.
        pub tmpfs: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// If `true`, allocate a pseudo-tty (`docker run -t`). Defaults to `false`.
        pub tty: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Ulimit options to add.
        pub ulimits: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::ContainerUlimit>>,
        >,
        /// Specifies files to upload to the container before starting it. Only one of `content` or `content_base64` can be set and at least one of them has to be set.
        pub uploads: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::ContainerUpload>>,
        >,
        /// User used for run the first process. Format is `user` or `user:group` which user and group can be passed literraly or by name.
        pub user: pulumi_gestalt_rust::Output<Option<String>>,
        /// Sets the usernamespace mode for the container when usernamespace remapping option is enabled.
        pub userns_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// Spec for mounting volumes in the container.
        pub volumes: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::ContainerVolume>>,
        >,
        /// If `true`, then the Docker container is waited for being healthy state after creation. If `false`, then the container health state is not checked. Defaults to `false`.
        pub wait: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The timeout in seconds to wait the container to be healthy after creation. Defaults to `60`.
        pub wait_timeout: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The working directory for commands to run in.
        pub working_dir: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ContainerArgs,
    ) -> ContainerResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let attach_binding = args.attach.get_output(context);
        let capabilities_binding = args.capabilities.get_output(context);
        let cgroupns_mode_binding = args.cgroupns_mode.get_output(context);
        let command_binding = args.command.get_output(context);
        let container_read_refresh_timeout_milliseconds_binding = args
            .container_read_refresh_timeout_milliseconds
            .get_output(context);
        let cpu_set_binding = args.cpu_set.get_output(context);
        let cpu_shares_binding = args.cpu_shares.get_output(context);
        let destroy_grace_seconds_binding = args
            .destroy_grace_seconds
            .get_output(context);
        let devices_binding = args.devices.get_output(context);
        let dns_binding = args.dns.get_output(context);
        let dns_opts_binding = args.dns_opts.get_output(context);
        let dns_searches_binding = args.dns_searches.get_output(context);
        let domainname_binding = args.domainname.get_output(context);
        let entrypoints_binding = args.entrypoints.get_output(context);
        let envs_binding = args.envs.get_output(context);
        let gpus_binding = args.gpus.get_output(context);
        let group_adds_binding = args.group_adds.get_output(context);
        let healthcheck_binding = args.healthcheck.get_output(context);
        let hostname_binding = args.hostname.get_output(context);
        let hosts_binding = args.hosts.get_output(context);
        let image_binding = args.image.get_output(context);
        let init_binding = args.init.get_output(context);
        let ipc_mode_binding = args.ipc_mode.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let log_driver_binding = args.log_driver.get_output(context);
        let log_opts_binding = args.log_opts.get_output(context);
        let logs_binding = args.logs.get_output(context);
        let max_retry_count_binding = args.max_retry_count.get_output(context);
        let memory_binding = args.memory.get_output(context);
        let memory_swap_binding = args.memory_swap.get_output(context);
        let mounts_binding = args.mounts.get_output(context);
        let must_run_binding = args.must_run.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_mode_binding = args.network_mode.get_output(context);
        let networks_advanced_binding = args.networks_advanced.get_output(context);
        let pid_mode_binding = args.pid_mode.get_output(context);
        let ports_binding = args.ports.get_output(context);
        let privileged_binding = args.privileged.get_output(context);
        let publish_all_ports_binding = args.publish_all_ports.get_output(context);
        let read_only_binding = args.read_only.get_output(context);
        let remove_volumes_binding = args.remove_volumes.get_output(context);
        let restart_binding = args.restart.get_output(context);
        let rm_binding = args.rm.get_output(context);
        let runtime_binding = args.runtime.get_output(context);
        let security_opts_binding = args.security_opts.get_output(context);
        let shm_size_binding = args.shm_size.get_output(context);
        let start_binding = args.start.get_output(context);
        let stdin_open_binding = args.stdin_open.get_output(context);
        let stop_signal_binding = args.stop_signal.get_output(context);
        let stop_timeout_binding = args.stop_timeout.get_output(context);
        let storage_opts_binding = args.storage_opts.get_output(context);
        let sysctls_binding = args.sysctls.get_output(context);
        let tmpfs_binding = args.tmpfs.get_output(context);
        let tty_binding = args.tty.get_output(context);
        let ulimits_binding = args.ulimits.get_output(context);
        let uploads_binding = args.uploads.get_output(context);
        let user_binding = args.user.get_output(context);
        let userns_mode_binding = args.userns_mode.get_output(context);
        let volumes_binding = args.volumes.get_output(context);
        let wait_binding = args.wait.get_output(context);
        let wait_timeout_binding = args.wait_timeout.get_output(context);
        let working_dir_binding = args.working_dir.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "docker:index/container:Container".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "attach".into(),
                    value: attach_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "capabilities".into(),
                    value: capabilities_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cgroupnsMode".into(),
                    value: cgroupns_mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "command".into(),
                    value: command_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "containerReadRefreshTimeoutMilliseconds".into(),
                    value: container_read_refresh_timeout_milliseconds_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cpuSet".into(),
                    value: cpu_set_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cpuShares".into(),
                    value: cpu_shares_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destroyGraceSeconds".into(),
                    value: destroy_grace_seconds_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "devices".into(),
                    value: devices_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dns".into(),
                    value: dns_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dnsOpts".into(),
                    value: dns_opts_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dnsSearches".into(),
                    value: dns_searches_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainname".into(),
                    value: domainname_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "entrypoints".into(),
                    value: entrypoints_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "envs".into(),
                    value: envs_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gpus".into(),
                    value: gpus_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "groupAdds".into(),
                    value: group_adds_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "healthcheck".into(),
                    value: healthcheck_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hostname".into(),
                    value: hostname_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hosts".into(),
                    value: hosts_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "image".into(),
                    value: image_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "init".into(),
                    value: init_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipcMode".into(),
                    value: ipc_mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logDriver".into(),
                    value: log_driver_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logOpts".into(),
                    value: log_opts_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logs".into(),
                    value: logs_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxRetryCount".into(),
                    value: max_retry_count_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "memory".into(),
                    value: memory_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "memorySwap".into(),
                    value: memory_swap_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mounts".into(),
                    value: mounts_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mustRun".into(),
                    value: must_run_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkMode".into(),
                    value: network_mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networksAdvanced".into(),
                    value: networks_advanced_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pidMode".into(),
                    value: pid_mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ports".into(),
                    value: ports_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "privileged".into(),
                    value: privileged_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publishAllPorts".into(),
                    value: publish_all_ports_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "readOnly".into(),
                    value: read_only_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "removeVolumes".into(),
                    value: remove_volumes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "restart".into(),
                    value: restart_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rm".into(),
                    value: rm_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "runtime".into(),
                    value: runtime_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityOpts".into(),
                    value: security_opts_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "shmSize".into(),
                    value: shm_size_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "start".into(),
                    value: start_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "stdinOpen".into(),
                    value: stdin_open_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "stopSignal".into(),
                    value: stop_signal_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "stopTimeout".into(),
                    value: stop_timeout_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageOpts".into(),
                    value: storage_opts_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sysctls".into(),
                    value: sysctls_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tmpfs".into(),
                    value: tmpfs_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tty".into(),
                    value: tty_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ulimits".into(),
                    value: ulimits_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "uploads".into(),
                    value: uploads_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "user".into(),
                    value: user_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "usernsMode".into(),
                    value: userns_mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "volumes".into(),
                    value: volumes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "wait".into(),
                    value: wait_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "waitTimeout".into(),
                    value: wait_timeout_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workingDir".into(),
                    value: working_dir_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ContainerResult {
            attach: o.get_field("attach"),
            bridge: o.get_field("bridge"),
            capabilities: o.get_field("capabilities"),
            cgroupns_mode: o.get_field("cgroupnsMode"),
            command: o.get_field("command"),
            container_logs: o.get_field("containerLogs"),
            container_read_refresh_timeout_milliseconds: o
                .get_field("containerReadRefreshTimeoutMilliseconds"),
            cpu_set: o.get_field("cpuSet"),
            cpu_shares: o.get_field("cpuShares"),
            destroy_grace_seconds: o.get_field("destroyGraceSeconds"),
            devices: o.get_field("devices"),
            dns: o.get_field("dns"),
            dns_opts: o.get_field("dnsOpts"),
            dns_searches: o.get_field("dnsSearches"),
            domainname: o.get_field("domainname"),
            entrypoints: o.get_field("entrypoints"),
            envs: o.get_field("envs"),
            exit_code: o.get_field("exitCode"),
            gpus: o.get_field("gpus"),
            group_adds: o.get_field("groupAdds"),
            healthcheck: o.get_field("healthcheck"),
            hostname: o.get_field("hostname"),
            hosts: o.get_field("hosts"),
            image: o.get_field("image"),
            init: o.get_field("init"),
            ipc_mode: o.get_field("ipcMode"),
            labels: o.get_field("labels"),
            log_driver: o.get_field("logDriver"),
            log_opts: o.get_field("logOpts"),
            logs: o.get_field("logs"),
            max_retry_count: o.get_field("maxRetryCount"),
            memory: o.get_field("memory"),
            memory_swap: o.get_field("memorySwap"),
            mounts: o.get_field("mounts"),
            must_run: o.get_field("mustRun"),
            name: o.get_field("name"),
            network_datas: o.get_field("networkDatas"),
            network_mode: o.get_field("networkMode"),
            networks_advanced: o.get_field("networksAdvanced"),
            pid_mode: o.get_field("pidMode"),
            ports: o.get_field("ports"),
            privileged: o.get_field("privileged"),
            publish_all_ports: o.get_field("publishAllPorts"),
            read_only: o.get_field("readOnly"),
            remove_volumes: o.get_field("removeVolumes"),
            restart: o.get_field("restart"),
            rm: o.get_field("rm"),
            runtime: o.get_field("runtime"),
            security_opts: o.get_field("securityOpts"),
            shm_size: o.get_field("shmSize"),
            start: o.get_field("start"),
            stdin_open: o.get_field("stdinOpen"),
            stop_signal: o.get_field("stopSignal"),
            stop_timeout: o.get_field("stopTimeout"),
            storage_opts: o.get_field("storageOpts"),
            sysctls: o.get_field("sysctls"),
            tmpfs: o.get_field("tmpfs"),
            tty: o.get_field("tty"),
            ulimits: o.get_field("ulimits"),
            uploads: o.get_field("uploads"),
            user: o.get_field("user"),
            userns_mode: o.get_field("usernsMode"),
            volumes: o.get_field("volumes"),
            wait: o.get_field("wait"),
            wait_timeout: o.get_field("waitTimeout"),
            working_dir: o.get_field("workingDir"),
        }
    }
}
