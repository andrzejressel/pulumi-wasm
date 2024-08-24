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
