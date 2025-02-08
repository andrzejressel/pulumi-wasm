#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct RemoteImageBuild {
    /// The configuration for the authentication
    #[builder(into, default)]
    #[serde(rename = "authConfigs")]
    pub r#auth_configs: Box<Option<Vec<super::types::RemoteImageBuildAuthConfig>>>,
    /// Set build-time variables
    #[builder(into, default)]
    #[serde(rename = "buildArg")]
    pub r#build_arg: Box<Option<std::collections::HashMap<String, String>>>,
    /// Pairs for build-time variables in the form TODO
    #[builder(into, default)]
    #[serde(rename = "buildArgs")]
    pub r#build_args: Box<Option<std::collections::HashMap<String, String>>>,
    /// BuildID is an optional identifier that can be passed together with the build request. The same identifier can be used to gracefully cancel the build with the cancel request.
    #[builder(into, default)]
    #[serde(rename = "buildId")]
    pub r#build_id: Box<Option<String>>,
    /// Images to consider as cache sources
    #[builder(into, default)]
    #[serde(rename = "cacheFroms")]
    pub r#cache_froms: Box<Option<Vec<String>>>,
    /// Optional parent cgroup for the container
    #[builder(into, default)]
    #[serde(rename = "cgroupParent")]
    pub r#cgroup_parent: Box<Option<String>>,
    /// Value to specify the build context. Currently, only a `PATH` context is supported. You can use the helper function '${path.cwd}/context-dir'. Please see https://docs.docker.com/build/building/context/ for more information about build contexts.
    #[builder(into)]
    #[serde(rename = "context")]
    pub r#context: Box<String>,
    /// The length of a CPU period in microseconds
    #[builder(into, default)]
    #[serde(rename = "cpuPeriod")]
    pub r#cpu_period: Box<Option<i32>>,
    /// Microseconds of CPU time that the container can get in a CPU period
    #[builder(into, default)]
    #[serde(rename = "cpuQuota")]
    pub r#cpu_quota: Box<Option<i32>>,
    /// CPUs in which to allow execution (e.g., `0-3`, `0`, `1`)
    #[builder(into, default)]
    #[serde(rename = "cpuSetCpus")]
    pub r#cpu_set_cpus: Box<Option<String>>,
    /// MEMs in which to allow execution (`0-3`, `0`, `1`)
    #[builder(into, default)]
    #[serde(rename = "cpuSetMems")]
    pub r#cpu_set_mems: Box<Option<String>>,
    /// CPU shares (relative weight)
    #[builder(into, default)]
    #[serde(rename = "cpuShares")]
    pub r#cpu_shares: Box<Option<i32>>,
    /// Name of the Dockerfile. Defaults to `Dockerfile`.
    #[builder(into, default)]
    #[serde(rename = "dockerfile")]
    pub r#dockerfile: Box<Option<String>>,
    /// A list of hostnames/IP mappings to add to the container’s /etc/hosts file. Specified in the form ["hostname:IP"]
    #[builder(into, default)]
    #[serde(rename = "extraHosts")]
    pub r#extra_hosts: Box<Option<Vec<String>>>,
    /// Always remove intermediate containers
    #[builder(into, default)]
    #[serde(rename = "forceRemove")]
    pub r#force_remove: Box<Option<bool>>,
    /// Isolation represents the isolation technology of a container. The supported values are
    #[builder(into, default)]
    #[serde(rename = "isolation")]
    pub r#isolation: Box<Option<String>>,
    /// Set metadata for an image
    #[builder(into, default)]
    #[serde(rename = "label")]
    pub r#label: Box<Option<std::collections::HashMap<String, String>>>,
    /// User-defined key/value metadata
    #[builder(into, default)]
    #[serde(rename = "labels")]
    pub r#labels: Box<Option<std::collections::HashMap<String, String>>>,
    /// Set memory limit for build
    #[builder(into, default)]
    #[serde(rename = "memory")]
    pub r#memory: Box<Option<i32>>,
    /// Total memory (memory + swap), -1 to enable unlimited swap
    #[builder(into, default)]
    #[serde(rename = "memorySwap")]
    pub r#memory_swap: Box<Option<i32>>,
    /// Set the networking mode for the RUN instructions during build
    #[builder(into, default)]
    #[serde(rename = "networkMode")]
    pub r#network_mode: Box<Option<String>>,
    /// Do not use the cache when building the image
    #[builder(into, default)]
    #[serde(rename = "noCache")]
    pub r#no_cache: Box<Option<bool>>,
    /// Set platform if server is multi-platform capable
    #[builder(into, default)]
    #[serde(rename = "platform")]
    pub r#platform: Box<Option<String>>,
    /// Attempt to pull the image even if an older image exists locally
    #[builder(into, default)]
    #[serde(rename = "pullParent")]
    pub r#pull_parent: Box<Option<bool>>,
    /// A Git repository URI or HTTP/HTTPS context URI
    #[builder(into, default)]
    #[serde(rename = "remoteContext")]
    pub r#remote_context: Box<Option<String>>,
    /// Remove intermediate containers after a successful build. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "remove")]
    pub r#remove: Box<Option<bool>>,
    /// The security options
    #[builder(into, default)]
    #[serde(rename = "securityOpts")]
    pub r#security_opts: Box<Option<Vec<String>>>,
    /// Set an ID for the build session
    #[builder(into, default)]
    #[serde(rename = "sessionId")]
    pub r#session_id: Box<Option<String>>,
    /// Size of /dev/shm in bytes. The size must be greater than 0
    #[builder(into, default)]
    #[serde(rename = "shmSize")]
    pub r#shm_size: Box<Option<i32>>,
    /// If true the new layers are squashed into a new image with a single new layer
    #[builder(into, default)]
    #[serde(rename = "squash")]
    pub r#squash: Box<Option<bool>>,
    /// Suppress the build output and print image ID on success
    #[builder(into, default)]
    #[serde(rename = "suppressOutput")]
    pub r#suppress_output: Box<Option<bool>>,
    /// Name and optionally a tag in the 'name:tag' format
    #[builder(into, default)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Option<Vec<String>>>,
    /// Set the target build stage to build
    #[builder(into, default)]
    #[serde(rename = "target")]
    pub r#target: Box<Option<String>>,
    /// Configuration for ulimits
    #[builder(into, default)]
    #[serde(rename = "ulimits")]
    pub r#ulimits: Box<Option<Vec<super::types::RemoteImageBuildUlimit>>>,
    /// Version of the underlying builder to use
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
