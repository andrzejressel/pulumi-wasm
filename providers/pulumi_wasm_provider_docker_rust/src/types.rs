#[derive(serde::Serialize)]
pub struct CacheFrom {
    /// Specifies cached images
    #[serde(rename = "images")]
    pub r#images: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct ContainerCapabilities {
    /// List of linux capabilities to add.
    #[serde(rename = "adds")]
    pub r#adds: Box<Option<Vec<String>>>,
    /// List of linux capabilities to drop.
    #[serde(rename = "drops")]
    pub r#drops: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct ContainerDevice {
    /// The path in the container where the device will be bound.
    #[serde(rename = "containerPath")]
    pub r#container_path: Box<Option<String>>,
    /// The path on the host where the device is located.
    #[serde(rename = "hostPath")]
    pub r#host_path: Box<String>,
    /// The cgroup permissions given to the container to access the device. Defaults to `rwm`.
    #[serde(rename = "permissions")]
    pub r#permissions: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct ContainerHealthcheck {
    /// Time between running the check (ms|s|m|h). Defaults to `0s`.
    #[serde(rename = "interval")]
    pub r#interval: Box<Option<String>>,
    /// Consecutive failures needed to report unhealthy. Defaults to `0`.
    #[serde(rename = "retries")]
    pub r#retries: Box<Option<i32>>,
    /// Start period for the container to initialize before counting retries towards unstable (ms|s|m|h). Defaults to `0s`.
    #[serde(rename = "startPeriod")]
    pub r#start_period: Box<Option<String>>,
    /// Command to run to check health. For example, to run `curl -f localhost/health` set the command to be `["CMD", "curl", "-f", "localhost/health"]`.
    #[serde(rename = "tests")]
    pub r#tests: Box<Vec<String>>,
    /// Maximum time to allow one check to run (ms|s|m|h). Defaults to `0s`.
    #[serde(rename = "timeout")]
    pub r#timeout: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct ContainerHost {
    /// Hostname to add
    #[serde(rename = "host")]
    pub r#host: Box<String>,
    /// IP address this hostname should resolve to.
    #[serde(rename = "ip")]
    pub r#ip: Box<String>,
}

#[derive(serde::Serialize)]
pub struct ContainerLabel {
    /// Name of the label
    #[serde(rename = "label")]
    pub r#label: Box<String>,
    /// Value of the label
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}

#[derive(serde::Serialize)]
pub struct ContainerMount {
    /// Optional configuration for the bind type.
    #[serde(rename = "bindOptions")]
    pub r#bind_options: Box<Option<crate::types::ContainerMountBindOptions>>,
    /// Whether the mount should be read-only.
    #[serde(rename = "readOnly")]
    pub r#read_only: Box<Option<bool>>,
    /// Mount source (e.g. a volume name, a host path).
    #[serde(rename = "source")]
    pub r#source: Box<Option<String>>,
    /// Container path
    #[serde(rename = "target")]
    pub r#target: Box<String>,
    /// Optional configuration for the tmpfs type.
    #[serde(rename = "tmpfsOptions")]
    pub r#tmpfs_options: Box<Option<crate::types::ContainerMountTmpfsOptions>>,
    /// The mount type
    #[serde(rename = "type")]
    pub r#type: Box<String>,
    /// Optional configuration for the volume type.
    #[serde(rename = "volumeOptions")]
    pub r#volume_options: Box<Option<crate::types::ContainerMountVolumeOptions>>,
}

#[derive(serde::Serialize)]
pub struct ContainerMountBindOptions {
    /// A propagation mode with the value.
    #[serde(rename = "propagation")]
    pub r#propagation: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct ContainerMountTmpfsOptions {
    /// The permission mode for the tmpfs mount in an integer.
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<i32>>,
    /// The size for the tmpfs mount in bytes.
    #[serde(rename = "sizeBytes")]
    pub r#size_bytes: Box<Option<i32>>,
}

#[derive(serde::Serialize)]
pub struct ContainerMountVolumeOptions {
    /// Name of the driver to use to create the volume.
    #[serde(rename = "driverName")]
    pub r#driver_name: Box<Option<String>>,
    /// key/value map of driver specific options.
    #[serde(rename = "driverOptions")]
    pub r#driver_options: Box<Option<std::collections::HashMap<String, String>>>,
    /// User-defined key/value metadata.
    #[serde(rename = "labels")]
    pub r#labels: Box<Option<Vec<crate::types::ContainerMountVolumeOptionsLabel>>>,
    /// Populate volume with data from the target.
    #[serde(rename = "noCopy")]
    pub r#no_copy: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct ContainerMountVolumeOptionsLabel {
    /// Name of the label
    #[serde(rename = "label")]
    pub r#label: Box<String>,
    /// Value of the label
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}

#[derive(serde::Serialize)]
pub struct ContainerNetworkData {
    /// The network gateway of the container.
    #[serde(rename = "gateway")]
    pub r#gateway: Box<Option<String>>,
    /// The IPV6 address of the container.
    #[serde(rename = "globalIpv6Address")]
    pub r#global_ipv_6_address: Box<Option<String>>,
    /// The IPV6 prefix length address of the container.
    #[serde(rename = "globalIpv6PrefixLength")]
    pub r#global_ipv_6_prefix_length: Box<Option<i32>>,
    /// The IP address of the container.
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Box<Option<String>>,
    /// The IP prefix length of the container.
    #[serde(rename = "ipPrefixLength")]
    pub r#ip_prefix_length: Box<Option<i32>>,
    /// The IPV6 gateway of the container.
    #[serde(rename = "ipv6Gateway")]
    pub r#ipv_6_gateway: Box<Option<String>>,
    /// The MAC address of the container.
    #[serde(rename = "macAddress")]
    pub r#mac_address: Box<Option<String>>,
    /// The name of the network
    #[serde(rename = "networkName")]
    pub r#network_name: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct ContainerNetworksAdvanced {
    /// The network aliases of the container in the specific network.
    #[serde(rename = "aliases")]
    pub r#aliases: Box<Option<Vec<String>>>,
    /// The IPV4 address of the container in the specific network.
    #[serde(rename = "ipv4Address")]
    pub r#ipv_4_address: Box<Option<String>>,
    /// The IPV6 address of the container in the specific network.
    #[serde(rename = "ipv6Address")]
    pub r#ipv_6_address: Box<Option<String>>,
    /// The name or id of the network to use. You can use `name` or `id` attribute from a `docker.Network` resource.
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}

#[derive(serde::Serialize)]
pub struct ContainerPort {
    /// Port exposed out of the container. If not given a free random port `>= 32768` will be used.
    #[serde(rename = "external")]
    pub r#external: Box<Option<i32>>,
    /// Port within the container.
    #[serde(rename = "internal")]
    pub r#internal: Box<i32>,
    /// IP address/mask that can access this port. Defaults to `0.0.0.0`.
    #[serde(rename = "ip")]
    pub r#ip: Box<Option<String>>,
    /// Protocol that can be used over this port. Defaults to `tcp`.
    #[serde(rename = "protocol")]
    pub r#protocol: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct ContainerUlimit {
    /// The hard limit
    #[serde(rename = "hard")]
    pub r#hard: Box<i32>,
    /// The name of the ulimit
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The soft limit
    #[serde(rename = "soft")]
    pub r#soft: Box<i32>,
}

#[derive(serde::Serialize)]
pub struct ContainerUpload {
    /// Literal string value to use as the object content, which will be uploaded as UTF-8-encoded text. Conflicts with `content_base64` & `source`
    #[serde(rename = "content")]
    pub r#content: Box<Option<String>>,
    /// Base64-encoded data that will be decoded and uploaded as raw bytes for the object content. This allows safely uploading non-UTF8 binary data, but is recommended only for larger binary content such as the result of the `base64encode` interpolation function. See here for the reason. Conflicts with `content` & `source`
    #[serde(rename = "contentBase64")]
    pub r#content_base_64: Box<Option<String>>,
    /// If `true`, the file will be uploaded with user executable permission. Defaults to `false`.
    #[serde(rename = "executable")]
    pub r#executable: Box<Option<bool>>,
    /// Path to the file in the container where is upload goes to
    #[serde(rename = "file")]
    pub r#file: Box<String>,
    /// A filename that references a file which will be uploaded as the object content. This allows for large file uploads that do not get stored in state. Conflicts with `content` & `content_base64`
    #[serde(rename = "source")]
    pub r#source: Box<Option<String>>,
    /// If using `source`, this will force an update if the file content has updated but the filename has not.
    #[serde(rename = "sourceHash")]
    pub r#source_hash: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct ContainerVolume {
    /// The path in the container where the volume will be mounted.
    #[serde(rename = "containerPath")]
    pub r#container_path: Box<Option<String>>,
    /// The container where the volume is coming from.
    #[serde(rename = "fromContainer")]
    pub r#from_container: Box<Option<String>>,
    /// The path on the host where the volume is coming from.
    #[serde(rename = "hostPath")]
    pub r#host_path: Box<Option<String>>,
    /// If `true`, this volume will be readonly. Defaults to `false`.
    #[serde(rename = "readOnly")]
    pub r#read_only: Box<Option<bool>>,
    /// The name of the docker volume which should be mounted.
    #[serde(rename = "volumeName")]
    pub r#volume_name: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct DockerBuild {
    /// Custom host-to-IP mappings to use while building (format: "host:ip")
    #[serde(rename = "addHosts")]
    pub r#add_hosts: Box<Option<Vec<String>>>,
    /// An optional map of named build-time argument variables to set during the Docker build. This flag allows you to pass build-time variables that can be accessed like environment variables inside the RUN instruction.
    #[serde(rename = "args")]
    pub r#args: Box<Option<std::collections::HashMap<String, String>>>,
    /// The version of the Docker builder.
    #[serde(rename = "builderVersion")]
    pub r#builder_version: Box<Option<crate::types::BuilderVersion>>,
    /// A list of image names to use as build cache. Images provided must have a cache manifest. Must provide authentication to cache registry.
    #[serde(rename = "cacheFrom")]
    pub r#cache_from: Box<Option<crate::types::CacheFrom>>,
    /// The path to the build context to use.
    #[serde(rename = "context")]
    pub r#context: Box<Option<String>>,
    /// The path to the Dockerfile to use.
    #[serde(rename = "dockerfile")]
    pub r#dockerfile: Box<Option<String>>,
    /// Set the networking mode for RUN instructions
    #[serde(rename = "network")]
    pub r#network: Box<Option<String>>,
    /// The architecture of the platform you want to build this image for, e.g. `linux/arm64`.
    #[serde(rename = "platform")]
    pub r#platform: Box<Option<String>>,
    /// The target of the Dockerfile to build
    #[serde(rename = "target")]
    pub r#target: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct NetworkIpamConfig {
    /// Auxiliary IPv4 or IPv6 addresses used by Network driver
    #[serde(rename = "auxAddress")]
    pub r#aux_address: Box<Option<std::collections::HashMap<String, String>>>,
    /// The IP address of the gateway
    #[serde(rename = "gateway")]
    pub r#gateway: Box<Option<String>>,
    /// The ip range in CIDR form
    #[serde(rename = "ipRange")]
    pub r#ip_range: Box<Option<String>>,
    /// The subnet in CIDR form
    #[serde(rename = "subnet")]
    pub r#subnet: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct NetworkLabel {
    /// Name of the label
    #[serde(rename = "label")]
    pub r#label: Box<String>,
    /// Value of the label
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}

#[derive(serde::Serialize)]
pub struct PluginGrantPermission {
    /// The name of the permission
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The value of the permission
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct ProviderRegistryAuth {
    /// Address of the registry
    #[serde(rename = "address")]
    pub r#address: Box<String>,
    #[serde(rename = "authDisabled")]
    pub r#auth_disabled: Box<Option<bool>>,
    /// Path to docker json file for registry auth. Defaults to `~/.docker/config.json`. If `DOCKER_CONFIG` is set, the value of `DOCKER_CONFIG` is used as the path. `config_file` has predencen over all other options.
    #[serde(rename = "configFile")]
    pub r#config_file: Box<Option<String>>,
    /// Plain content of the docker json file for registry auth. `config_file_content` has precedence over username/password.
    #[serde(rename = "configFileContent")]
    pub r#config_file_content: Box<Option<String>>,
    /// Password for the registry. Defaults to `DOCKER_REGISTRY_PASS` env variable if set.
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    /// Username for the registry. Defaults to `DOCKER_REGISTRY_USER` env variable if set.
    #[serde(rename = "username")]
    pub r#username: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct Registry {
    /// The password to authenticate to the registry. Does not cause image rebuild when changed.
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    /// The URL of the Docker registry server
    #[serde(rename = "server")]
    pub r#server: Box<Option<String>>,
    /// The username to authenticate to the registry. Does not cause image rebuild when changed.
    #[serde(rename = "username")]
    pub r#username: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct RemoteImageBuild {
    /// The configuration for the authentication
    #[serde(rename = "authConfigs")]
    pub r#auth_configs: Box<Option<Vec<crate::types::RemoteImageBuildAuthConfig>>>,
    /// Set build-time variables
    #[serde(rename = "buildArg")]
    pub r#build_arg: Box<Option<std::collections::HashMap<String, String>>>,
    /// Pairs for build-time variables in the form TODO
    #[serde(rename = "buildArgs")]
    pub r#build_args: Box<Option<std::collections::HashMap<String, String>>>,
    /// BuildID is an optional identifier that can be passed together with the build request. The same identifier can be used to gracefully cancel the build with the cancel request.
    #[serde(rename = "buildId")]
    pub r#build_id: Box<Option<String>>,
    /// Images to consider as cache sources
    #[serde(rename = "cacheFroms")]
    pub r#cache_froms: Box<Option<Vec<String>>>,
    /// Optional parent cgroup for the container
    #[serde(rename = "cgroupParent")]
    pub r#cgroup_parent: Box<Option<String>>,
    /// Value to specify the build context. Currently, only a `PATH` context is supported. You can use the helper function '${path.cwd}/context-dir'. Please see https://docs.docker.com/build/building/context/ for more information about build contexts.
    #[serde(rename = "context")]
    pub r#context: Box<String>,
    /// The length of a CPU period in microseconds
    #[serde(rename = "cpuPeriod")]
    pub r#cpu_period: Box<Option<i32>>,
    /// Microseconds of CPU time that the container can get in a CPU period
    #[serde(rename = "cpuQuota")]
    pub r#cpu_quota: Box<Option<i32>>,
    /// CPUs in which to allow execution (e.g., `0-3`, `0`, `1`)
    #[serde(rename = "cpuSetCpus")]
    pub r#cpu_set_cpus: Box<Option<String>>,
    /// MEMs in which to allow execution (`0-3`, `0`, `1`)
    #[serde(rename = "cpuSetMems")]
    pub r#cpu_set_mems: Box<Option<String>>,
    /// CPU shares (relative weight)
    #[serde(rename = "cpuShares")]
    pub r#cpu_shares: Box<Option<i32>>,
    /// Name of the Dockerfile. Defaults to `Dockerfile`.
    #[serde(rename = "dockerfile")]
    pub r#dockerfile: Box<Option<String>>,
    /// A list of hostnames/IP mappings to add to the container’s /etc/hosts file. Specified in the form ["hostname:IP"]
    #[serde(rename = "extraHosts")]
    pub r#extra_hosts: Box<Option<Vec<String>>>,
    /// Always remove intermediate containers
    #[serde(rename = "forceRemove")]
    pub r#force_remove: Box<Option<bool>>,
    /// Isolation represents the isolation technology of a container. The supported values are
    #[serde(rename = "isolation")]
    pub r#isolation: Box<Option<String>>,
    /// Set metadata for an image
    #[serde(rename = "label")]
    pub r#label: Box<Option<std::collections::HashMap<String, String>>>,
    /// User-defined key/value metadata
    #[serde(rename = "labels")]
    pub r#labels: Box<Option<std::collections::HashMap<String, String>>>,
    /// Set memory limit for build
    #[serde(rename = "memory")]
    pub r#memory: Box<Option<i32>>,
    /// Total memory (memory + swap), -1 to enable unlimited swap
    #[serde(rename = "memorySwap")]
    pub r#memory_swap: Box<Option<i32>>,
    /// Set the networking mode for the RUN instructions during build
    #[serde(rename = "networkMode")]
    pub r#network_mode: Box<Option<String>>,
    /// Do not use the cache when building the image
    #[serde(rename = "noCache")]
    pub r#no_cache: Box<Option<bool>>,
    /// Set platform if server is multi-platform capable
    #[serde(rename = "platform")]
    pub r#platform: Box<Option<String>>,
    /// Attempt to pull the image even if an older image exists locally
    #[serde(rename = "pullParent")]
    pub r#pull_parent: Box<Option<bool>>,
    /// A Git repository URI or HTTP/HTTPS context URI
    #[serde(rename = "remoteContext")]
    pub r#remote_context: Box<Option<String>>,
    /// Remove intermediate containers after a successful build. Defaults to `true`.
    #[serde(rename = "remove")]
    pub r#remove: Box<Option<bool>>,
    /// The security options
    #[serde(rename = "securityOpts")]
    pub r#security_opts: Box<Option<Vec<String>>>,
    /// Set an ID for the build session
    #[serde(rename = "sessionId")]
    pub r#session_id: Box<Option<String>>,
    /// Size of /dev/shm in bytes. The size must be greater than 0
    #[serde(rename = "shmSize")]
    pub r#shm_size: Box<Option<i32>>,
    /// If true the new layers are squashed into a new image with a single new layer
    #[serde(rename = "squash")]
    pub r#squash: Box<Option<bool>>,
    /// Suppress the build output and print image ID on success
    #[serde(rename = "suppressOutput")]
    pub r#suppress_output: Box<Option<bool>>,
    /// Name and optionally a tag in the 'name:tag' format
    #[serde(rename = "tags")]
    pub r#tags: Box<Option<Vec<String>>>,
    /// Set the target build stage to build
    #[serde(rename = "target")]
    pub r#target: Box<Option<String>>,
    /// Configuration for ulimits
    #[serde(rename = "ulimits")]
    pub r#ulimits: Box<Option<Vec<crate::types::RemoteImageBuildUlimit>>>,
    /// Version of the underlying builder to use
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct RemoteImageBuildAuthConfig {
    /// the auth token
    #[serde(rename = "auth")]
    pub r#auth: Box<Option<String>>,
    /// the user emal
    #[serde(rename = "email")]
    pub r#email: Box<Option<String>>,
    /// hostname of the registry
    #[serde(rename = "hostName")]
    pub r#host_name: Box<String>,
    /// the identity token
    #[serde(rename = "identityToken")]
    pub r#identity_token: Box<Option<String>>,
    /// the registry password
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    /// the registry token
    #[serde(rename = "registryToken")]
    pub r#registry_token: Box<Option<String>>,
    /// the server address
    #[serde(rename = "serverAddress")]
    pub r#server_address: Box<Option<String>>,
    /// the registry user name
    #[serde(rename = "userName")]
    pub r#user_name: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct RemoteImageBuildUlimit {
    /// soft limit
    #[serde(rename = "hard")]
    pub r#hard: Box<i32>,
    /// type of ulimit, e.g. `nofile`
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// hard limit
    #[serde(rename = "soft")]
    pub r#soft: Box<i32>,
}

#[derive(serde::Serialize)]
pub struct SecretLabel {
    /// Name of the label
    #[serde(rename = "label")]
    pub r#label: Box<String>,
    /// Value of the label
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}

#[derive(serde::Serialize)]
pub struct ServiceAuth {
    /// The password
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    /// The address of the server for the authentication
    #[serde(rename = "serverAddress")]
    pub r#server_address: Box<String>,
    /// The username
    #[serde(rename = "username")]
    pub r#username: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct ServiceConvergeConfig {
    /// The interval to check if the desired state is reached `(ms|s)`. Defaults to `7s`.
    #[serde(rename = "delay")]
    pub r#delay: Box<Option<String>>,
    /// The timeout of the service to reach the desired state `(s|m)`. Defaults to `3m`
    #[serde(rename = "timeout")]
    pub r#timeout: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct ServiceEndpointSpec {
    /// The mode of resolution to use for internal load balancing between tasks
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
    /// List of exposed ports that this service is accessible on from the outside. Ports can only be provided if 'vip' resolution mode is used
    #[serde(rename = "ports")]
    pub r#ports: Box<Option<Vec<crate::types::ServiceEndpointSpecPort>>>,
}

#[derive(serde::Serialize)]
pub struct ServiceEndpointSpecPort {
    /// A random name for the port
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Rrepresents the protocol of a port: `tcp`, `udp` or `sctp`. Defaults to `tcp`.
    #[serde(rename = "protocol")]
    pub r#protocol: Box<Option<String>>,
    /// Represents the mode in which the port is to be published: 'ingress' or 'host'. Defaults to `ingress`.
    #[serde(rename = "publishMode")]
    pub r#publish_mode: Box<Option<String>>,
    /// The port on the swarm hosts
    #[serde(rename = "publishedPort")]
    pub r#published_port: Box<Option<i32>>,
    /// The port inside the container
    #[serde(rename = "targetPort")]
    pub r#target_port: Box<i32>,
}

#[derive(serde::Serialize)]
pub struct ServiceLabel {
    /// Name of the label
    #[serde(rename = "label")]
    pub r#label: Box<String>,
    /// Value of the label
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}

#[derive(serde::Serialize)]
pub struct ServiceMode {
    /// When `true`, tasks will run on every worker node. Conflicts with `replicated`
    #[serde(rename = "global")]
    pub r#global: Box<Option<bool>>,
    /// The replicated service mode
    #[serde(rename = "replicated")]
    pub r#replicated: Box<Option<crate::types::ServiceModeReplicated>>,
}

#[derive(serde::Serialize)]
pub struct ServiceModeReplicated {
    /// The amount of replicas of the service. Defaults to `1`
    #[serde(rename = "replicas")]
    pub r#replicas: Box<Option<i32>>,
}

#[derive(serde::Serialize)]
pub struct ServiceRollbackConfig {
    /// Delay between task rollbacks (ns|us|ms|s|m|h). Defaults to `0s`.
    #[serde(rename = "delay")]
    pub r#delay: Box<Option<String>>,
    /// Action on rollback failure: pause | continue. Defaults to `pause`.
    #[serde(rename = "failureAction")]
    pub r#failure_action: Box<Option<String>>,
    /// Failure rate to tolerate during a rollback. Defaults to `0.0`.
    #[serde(rename = "maxFailureRatio")]
    pub r#max_failure_ratio: Box<Option<String>>,
    /// Duration after each task rollback to monitor for failure (ns|us|ms|s|m|h). Defaults to `5s`.
    #[serde(rename = "monitor")]
    pub r#monitor: Box<Option<String>>,
    /// Rollback order: either 'stop-first' or 'start-first'. Defaults to `stop-first`.
    #[serde(rename = "order")]
    pub r#order: Box<Option<String>>,
    /// Maximum number of tasks to be rollbacked in one iteration. Defaults to `1`
    #[serde(rename = "parallelism")]
    pub r#parallelism: Box<Option<i32>>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpec {
    /// The spec for each container
    #[serde(rename = "containerSpec")]
    pub r#container_spec: Box<crate::types::ServiceTaskSpecContainerSpec>,
    /// A counter that triggers an update even if no relevant parameters have been changed. See the [spec](https://github.com/docker/swarmkit/blob/master/api/specs.proto#L126).
    #[serde(rename = "forceUpdate")]
    pub r#force_update: Box<Option<i32>>,
    /// Specifies the log driver to use for tasks created from this spec. If not present, the default one for the swarm will be used, finally falling back to the engine default if not specified
    #[serde(rename = "logDriver")]
    pub r#log_driver: Box<Option<crate::types::ServiceTaskSpecLogDriver>>,
    /// The networks the container is attached to
    #[serde(rename = "networksAdvanceds")]
    pub r#networks_advanceds: Box<Option<Vec<crate::types::ServiceTaskSpecNetworksAdvanced>>>,
    /// The placement preferences
    #[serde(rename = "placement")]
    pub r#placement: Box<Option<crate::types::ServiceTaskSpecPlacement>>,
    /// Resource requirements which apply to each individual container created as part of the service
    #[serde(rename = "resources")]
    pub r#resources: Box<Option<crate::types::ServiceTaskSpecResources>>,
    /// Specification for the restart policy which applies to containers created as part of this service.
    #[serde(rename = "restartPolicy")]
    pub r#restart_policy: Box<Option<crate::types::ServiceTaskSpecRestartPolicy>>,
    /// Runtime is the type of runtime specified for the task executor. See the [types](https://github.com/moby/moby/blob/master/api/types/swarm/runtime.go).
    #[serde(rename = "runtime")]
    pub r#runtime: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpec {
    /// Arguments to the command
    #[serde(rename = "args")]
    pub r#args: Box<Option<Vec<String>>>,
    /// The command/entrypoint to be run in the image. According to the [docker cli](https://github.com/docker/cli/blob/v20.10.7/cli/command/service/opts.go#L705) the override of the entrypoint is also passed to the `command` property and there is no `entrypoint` attribute in the `ContainerSpec` of the service.
    #[serde(rename = "commands")]
    pub r#commands: Box<Option<Vec<String>>>,
    /// References to zero or more configs that will be exposed to the service
    #[serde(rename = "configs")]
    pub r#configs: Box<Option<Vec<crate::types::ServiceTaskSpecContainerSpecConfig>>>,
    /// The working directory for commands to run in
    #[serde(rename = "dir")]
    pub r#dir: Box<Option<String>>,
    /// Specification for DNS related configurations in resolver configuration file (`resolv.conf`)
    #[serde(rename = "dnsConfig")]
    pub r#dns_config: Box<Option<crate::types::ServiceTaskSpecContainerSpecDnsConfig>>,
    /// A list of environment variables in the form VAR="value"
    #[serde(rename = "env")]
    pub r#env: Box<Option<std::collections::HashMap<String, String>>>,
    /// A list of additional groups that the container process will run as
    #[serde(rename = "groups")]
    pub r#groups: Box<Option<Vec<String>>>,
    /// A test to perform to check that the container is healthy
    #[serde(rename = "healthcheck")]
    pub r#healthcheck: Box<Option<crate::types::ServiceTaskSpecContainerSpecHealthcheck>>,
    /// The hostname to use for the container, as a valid RFC 1123 hostname
    #[serde(rename = "hostname")]
    pub r#hostname: Box<Option<String>>,
    /// A list of hostname/IP mappings to add to the container's hosts file
    #[serde(rename = "hosts")]
    pub r#hosts: Box<Option<Vec<crate::types::ServiceTaskSpecContainerSpecHost>>>,
    /// The image name to use for the containers of the service, like `nginx:1.17.6`. Also use the data-source or resource of `docker.RemoteImage` with the `repo_digest` or `docker.RegistryImage` with the `name` attribute for this, as shown in the examples.
    #[serde(rename = "image")]
    pub r#image: Box<String>,
    /// Isolation technology of the containers running the service. (Windows only). Defaults to `default`.
    #[serde(rename = "isolation")]
    pub r#isolation: Box<Option<String>>,
    /// User-defined key/value metadata
    #[serde(rename = "labels")]
    pub r#labels: Box<Option<Vec<crate::types::ServiceTaskSpecContainerSpecLabel>>>,
    /// Specification for mounts to be added to containers created as part of the service
    #[serde(rename = "mounts")]
    pub r#mounts: Box<Option<Vec<crate::types::ServiceTaskSpecContainerSpecMount>>>,
    /// Security options for the container
    #[serde(rename = "privileges")]
    pub r#privileges: Box<Option<crate::types::ServiceTaskSpecContainerSpecPrivileges>>,
    /// Mount the container's root filesystem as read only
    #[serde(rename = "readOnly")]
    pub r#read_only: Box<Option<bool>>,
    /// References to zero or more secrets that will be exposed to the service
    #[serde(rename = "secrets")]
    pub r#secrets: Box<Option<Vec<crate::types::ServiceTaskSpecContainerSpecSecret>>>,
    /// Amount of time to wait for the container to terminate before forcefully removing it (ms|s|m|h). If not specified or '0s' the destroy will not check if all tasks/containers of the service terminate.
    #[serde(rename = "stopGracePeriod")]
    pub r#stop_grace_period: Box<Option<String>>,
    /// Signal to stop the container
    #[serde(rename = "stopSignal")]
    pub r#stop_signal: Box<Option<String>>,
    /// Sysctls config (Linux only)
    #[serde(rename = "sysctl")]
    pub r#sysctl: Box<Option<std::collections::HashMap<String, String>>>,
    /// The user inside the container
    #[serde(rename = "user")]
    pub r#user: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecConfig {
    /// ID of the specific config that we're referencing
    #[serde(rename = "configId")]
    pub r#config_id: Box<String>,
    /// Name of the config that this references, but this is just provided for lookup/display purposes. The config in the reference will be identified by its ID
    #[serde(rename = "configName")]
    pub r#config_name: Box<Option<String>>,
    /// Represents the file GID. Defaults to `0`.
    #[serde(rename = "fileGid")]
    pub r#file_gid: Box<Option<String>>,
    /// Represents represents the FileMode of the file. Defaults to `0o444`.
    #[serde(rename = "fileMode")]
    pub r#file_mode: Box<Option<i32>>,
    /// Represents the final filename in the filesystem
    #[serde(rename = "fileName")]
    pub r#file_name: Box<String>,
    /// Represents the file UID. Defaults to `0`.
    #[serde(rename = "fileUid")]
    pub r#file_uid: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecDnsConfig {
    /// The IP addresses of the name servers
    #[serde(rename = "nameservers")]
    pub r#nameservers: Box<Vec<String>>,
    /// A list of internal resolver variables to be modified (e.g., `debug`, `ndots:3`, etc.)
    #[serde(rename = "options")]
    pub r#options: Box<Option<Vec<String>>>,
    /// A search list for host-name lookup
    #[serde(rename = "searches")]
    pub r#searches: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecHealthcheck {
    /// Time between running the check (ms|s|m|h). Defaults to `0s`.
    #[serde(rename = "interval")]
    pub r#interval: Box<Option<String>>,
    /// Consecutive failures needed to report unhealthy. Defaults to `0`
    #[serde(rename = "retries")]
    pub r#retries: Box<Option<i32>>,
    /// Start period for the container to initialize before counting retries towards unstable (ms|s|m|h). Defaults to `0s`.
    #[serde(rename = "startPeriod")]
    pub r#start_period: Box<Option<String>>,
    /// The test to perform as list
    #[serde(rename = "tests")]
    pub r#tests: Box<Vec<String>>,
    /// Maximum time to allow one check to run (ms|s|m|h). Defaults to `0s`.
    #[serde(rename = "timeout")]
    pub r#timeout: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecHost {
    /// The name of the host
    #[serde(rename = "host")]
    pub r#host: Box<String>,
    /// The ip of the host
    #[serde(rename = "ip")]
    pub r#ip: Box<String>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecLabel {
    /// Name of the label
    #[serde(rename = "label")]
    pub r#label: Box<String>,
    /// Value of the label
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecMount {
    /// Optional configuration for the bind type
    #[serde(rename = "bindOptions")]
    pub r#bind_options: Box<Option<crate::types::ServiceTaskSpecContainerSpecMountBindOptions>>,
    /// Whether the mount should be read-only
    #[serde(rename = "readOnly")]
    pub r#read_only: Box<Option<bool>>,
    /// Mount source (e.g. a volume name, a host path)
    #[serde(rename = "source")]
    pub r#source: Box<Option<String>>,
    /// Container path
    #[serde(rename = "target")]
    pub r#target: Box<String>,
    /// Optional configuration for the tmpfs type
    #[serde(rename = "tmpfsOptions")]
    pub r#tmpfs_options: Box<Option<crate::types::ServiceTaskSpecContainerSpecMountTmpfsOptions>>,
    /// The mount type
    #[serde(rename = "type")]
    pub r#type: Box<String>,
    /// Optional configuration for the volume type
    #[serde(rename = "volumeOptions")]
    pub r#volume_options: Box<Option<crate::types::ServiceTaskSpecContainerSpecMountVolumeOptions>>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecMountBindOptions {
    /// Bind propagation refers to whether or not mounts created within a given bind-mount or named volume can be propagated to replicas of that mount. See the [docs](https://docs.docker.com/storage/bind-mounts/#configure-bind-propagation) for details. Defaults to `rprivate`
    #[serde(rename = "propagation")]
    pub r#propagation: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecMountTmpfsOptions {
    /// The permission mode for the tmpfs mount in an integer
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<i32>>,
    /// The size for the tmpfs mount in bytes
    #[serde(rename = "sizeBytes")]
    pub r#size_bytes: Box<Option<i32>>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecMountVolumeOptions {
    /// Name of the driver to use to create the volume
    #[serde(rename = "driverName")]
    pub r#driver_name: Box<Option<String>>,
    /// key/value map of driver specific options
    #[serde(rename = "driverOptions")]
    pub r#driver_options: Box<Option<std::collections::HashMap<String, String>>>,
    /// User-defined key/value metadata
    #[serde(rename = "labels")]
    pub r#labels:
        Box<Option<Vec<crate::types::ServiceTaskSpecContainerSpecMountVolumeOptionsLabel>>>,
    /// Populate volume with data from the target
    #[serde(rename = "noCopy")]
    pub r#no_copy: Box<Option<bool>>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecMountVolumeOptionsLabel {
    /// Name of the label
    #[serde(rename = "label")]
    pub r#label: Box<String>,
    /// Value of the label
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecPrivileges {
    /// CredentialSpec for managed service account (Windows only)
    #[serde(rename = "credentialSpec")]
    pub r#credential_spec:
        Box<Option<crate::types::ServiceTaskSpecContainerSpecPrivilegesCredentialSpec>>,
    /// SELinux labels of the container
    #[serde(rename = "seLinuxContext")]
    pub r#se_linux_context:
        Box<Option<crate::types::ServiceTaskSpecContainerSpecPrivilegesSeLinuxContext>>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecPrivilegesCredentialSpec {
    /// Load credential spec from this file
    #[serde(rename = "file")]
    pub r#file: Box<Option<String>>,
    /// Load credential spec from this value in the Windows registry
    #[serde(rename = "registry")]
    pub r#registry: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecPrivilegesSeLinuxContext {
    /// Disable SELinux
    #[serde(rename = "disable")]
    pub r#disable: Box<Option<bool>>,
    /// SELinux level label
    #[serde(rename = "level")]
    pub r#level: Box<Option<String>>,
    /// SELinux role label
    #[serde(rename = "role")]
    pub r#role: Box<Option<String>>,
    /// SELinux type label
    #[serde(rename = "type")]
    pub r#type: Box<Option<String>>,
    /// SELinux user label
    #[serde(rename = "user")]
    pub r#user: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecSecret {
    /// Represents the file GID. Defaults to `0`
    #[serde(rename = "fileGid")]
    pub r#file_gid: Box<Option<String>>,
    /// Represents represents the FileMode of the file. Defaults to `0o444`
    #[serde(rename = "fileMode")]
    pub r#file_mode: Box<Option<i32>>,
    /// Represents the final filename in the filesystem
    #[serde(rename = "fileName")]
    pub r#file_name: Box<String>,
    /// Represents the file UID. Defaults to `0`
    #[serde(rename = "fileUid")]
    pub r#file_uid: Box<Option<String>>,
    /// ID of the specific secret that we're referencing
    #[serde(rename = "secretId")]
    pub r#secret_id: Box<String>,
    /// Name of the secret that this references, but this is just provided for lookup/display purposes. The config in the reference will be identified by its ID
    #[serde(rename = "secretName")]
    pub r#secret_name: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecLogDriver {
    /// The logging driver to use
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The options for the logging driver
    #[serde(rename = "options")]
    pub r#options: Box<Option<std::collections::HashMap<String, String>>>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecNetworksAdvanced {
    /// The network aliases of the container in the specific network.
    #[serde(rename = "aliases")]
    pub r#aliases: Box<Option<Vec<String>>>,
    /// An array of driver options for the network, e.g. `opts1=value`
    #[serde(rename = "driverOpts")]
    pub r#driver_opts: Box<Option<Vec<String>>>,
    /// The name/id of the network.
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecPlacement {
    /// An array of constraints. e.g.: `node.role==manager`
    #[serde(rename = "constraints")]
    pub r#constraints: Box<Option<Vec<String>>>,
    /// Maximum number of replicas for per node (default value is `0`, which is unlimited)
    #[serde(rename = "maxReplicas")]
    pub r#max_replicas: Box<Option<i32>>,
    /// Platforms stores all the platforms that the service's image can run on
    #[serde(rename = "platforms")]
    pub r#platforms: Box<Option<Vec<crate::types::ServiceTaskSpecPlacementPlatform>>>,
    /// Preferences provide a way to make the scheduler aware of factors such as topology. They are provided in order from highest to lowest precedence, e.g.: `spread=node.role.manager`
    #[serde(rename = "prefs")]
    pub r#prefs: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecPlacementPlatform {
    /// The architecture, e.g. `amd64`
    #[serde(rename = "architecture")]
    pub r#architecture: Box<String>,
    /// The operation system, e.g. `linux`
    #[serde(rename = "os")]
    pub r#os: Box<String>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecResources {
    /// Describes the resources which can be advertised by a node and requested by a task
    #[serde(rename = "limits")]
    pub r#limits: Box<Option<crate::types::ServiceTaskSpecResourcesLimits>>,
    /// An object describing the resources which can be advertised by a node and requested by a task
    #[serde(rename = "reservation")]
    pub r#reservation: Box<Option<crate::types::ServiceTaskSpecResourcesReservation>>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecResourcesLimits {
    /// The amounf of memory in bytes the container allocates
    #[serde(rename = "memoryBytes")]
    pub r#memory_bytes: Box<Option<i32>>,
    /// CPU shares in units of `1/1e9` (or `10^-9`) of the CPU. Should be at least `1000000`
    #[serde(rename = "nanoCpus")]
    pub r#nano_cpus: Box<Option<i32>>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecResourcesReservation {
    /// User-defined resources can be either Integer resources (e.g, `SSD=3`) or String resources (e.g, GPU=UUID1)
    #[serde(rename = "genericResources")]
    pub r#generic_resources:
        Box<Option<crate::types::ServiceTaskSpecResourcesReservationGenericResources>>,
    /// The amounf of memory in bytes the container allocates
    #[serde(rename = "memoryBytes")]
    pub r#memory_bytes: Box<Option<i32>>,
    /// CPU shares in units of 1/1e9 (or 10^-9) of the CPU. Should be at least `1000000`
    #[serde(rename = "nanoCpus")]
    pub r#nano_cpus: Box<Option<i32>>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecResourcesReservationGenericResources {
    /// The Integer resources
    #[serde(rename = "discreteResourcesSpecs")]
    pub r#discrete_resources_specs: Box<Option<Vec<String>>>,
    /// The String resources
    #[serde(rename = "namedResourcesSpecs")]
    pub r#named_resources_specs: Box<Option<Vec<String>>>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecRestartPolicy {
    /// Condition for restart
    #[serde(rename = "condition")]
    pub r#condition: Box<Option<String>>,
    /// Delay between restart attempts (ms|s|m|h)
    #[serde(rename = "delay")]
    pub r#delay: Box<Option<String>>,
    /// Maximum attempts to restart a given container before giving up (default value is `0`, which is ignored)
    #[serde(rename = "maxAttempts")]
    pub r#max_attempts: Box<Option<i32>>,
    /// The time window used to evaluate the restart policy (default value is `0`, which is unbounded) (ms|s|m|h)
    #[serde(rename = "window")]
    pub r#window: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct ServiceUpdateConfig {
    /// Delay between task updates `(ns|us|ms|s|m|h)`. Defaults to `0s`.
    #[serde(rename = "delay")]
    pub r#delay: Box<Option<String>>,
    /// Action on update failure: `pause`, `continue` or `rollback`. Defaults to `pause`.
    #[serde(rename = "failureAction")]
    pub r#failure_action: Box<Option<String>>,
    /// Failure rate to tolerate during an update. Defaults to `0.0`.
    #[serde(rename = "maxFailureRatio")]
    pub r#max_failure_ratio: Box<Option<String>>,
    /// Duration after each task update to monitor for failure (ns|us|ms|s|m|h). Defaults to `5s`.
    #[serde(rename = "monitor")]
    pub r#monitor: Box<Option<String>>,
    /// Update order: either 'stop-first' or 'start-first'. Defaults to `stop-first`.
    #[serde(rename = "order")]
    pub r#order: Box<Option<String>>,
    /// Maximum number of tasks to be updated in one iteration. Defaults to `1`
    #[serde(rename = "parallelism")]
    pub r#parallelism: Box<Option<i32>>,
}

#[derive(serde::Serialize)]
pub struct VolumeLabel {
    /// Name of the label
    #[serde(rename = "label")]
    pub r#label: Box<String>,
    /// Value of the label
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}

#[derive(serde::Serialize)]
pub struct GetNetworkIpamConfig {
    /// Auxiliary IPv4 or IPv6 addresses used by Network driver
    #[serde(rename = "auxAddress")]
    pub r#aux_address: Box<Option<std::collections::HashMap<String, String>>>,
    /// The IP address of the gateway
    #[serde(rename = "gateway")]
    pub r#gateway: Box<Option<String>>,
    /// The ip range in CIDR form
    #[serde(rename = "ipRange")]
    pub r#ip_range: Box<Option<String>>,
    /// The subnet in CIDR form
    #[serde(rename = "subnet")]
    pub r#subnet: Box<Option<String>>,
}

#[derive(serde::Serialize)]
pub struct RegistryAuth {
    /// Address of the registry
    #[serde(rename = "address")]
    pub r#address: Box<String>,
    #[serde(rename = "authDisabled")]
    pub r#auth_disabled: Box<Option<bool>>,
    /// Path to docker json file for registry auth. Defaults to `~/.docker/config.json`. If `DOCKER_CONFIG` is set, the value of `DOCKER_CONFIG` is used as the path. `config_file` has predencen over all other options.
    #[serde(rename = "configFile")]
    pub r#config_file: Box<Option<String>>,
    /// Plain content of the docker json file for registry auth. `config_file_content` has precedence over username/password.
    #[serde(rename = "configFileContent")]
    pub r#config_file_content: Box<Option<String>>,
    /// Password for the registry. Defaults to `DOCKER_REGISTRY_PASS` env variable if set.
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    /// Username for the registry. Defaults to `DOCKER_REGISTRY_USER` env variable if set.
    #[serde(rename = "username")]
    pub r#username: Box<Option<String>>,
}

pub type BuilderVersion = String;
