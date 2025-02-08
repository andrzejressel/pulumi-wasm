#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceTaskSpecContainerSpec {
    /// Arguments to the command
    #[builder(into, default)]
    #[serde(rename = "args")]
    pub r#args: Box<Option<Vec<String>>>,
    /// The command/entrypoint to be run in the image. According to the [docker cli](https://github.com/docker/cli/blob/v20.10.7/cli/command/service/opts.go#L705) the override of the entrypoint is also passed to the `command` property and there is no `entrypoint` attribute in the `ContainerSpec` of the service.
    #[builder(into, default)]
    #[serde(rename = "commands")]
    pub r#commands: Box<Option<Vec<String>>>,
    /// References to zero or more configs that will be exposed to the service
    #[builder(into, default)]
    #[serde(rename = "configs")]
    pub r#configs: Box<Option<Vec<super::types::ServiceTaskSpecContainerSpecConfig>>>,
    /// The working directory for commands to run in
    #[builder(into, default)]
    #[serde(rename = "dir")]
    pub r#dir: Box<Option<String>>,
    /// Specification for DNS related configurations in resolver configuration file (`resolv.conf`)
    #[builder(into, default)]
    #[serde(rename = "dnsConfig")]
    pub r#dns_config: Box<Option<super::types::ServiceTaskSpecContainerSpecDnsConfig>>,
    /// A list of environment variables in the form VAR="value"
    #[builder(into, default)]
    #[serde(rename = "env")]
    pub r#env: Box<Option<std::collections::HashMap<String, String>>>,
    /// A list of additional groups that the container process will run as
    #[builder(into, default)]
    #[serde(rename = "groups")]
    pub r#groups: Box<Option<Vec<String>>>,
    /// A test to perform to check that the container is healthy
    #[builder(into, default)]
    #[serde(rename = "healthcheck")]
    pub r#healthcheck: Box<Option<super::types::ServiceTaskSpecContainerSpecHealthcheck>>,
    /// The hostname to use for the container, as a valid RFC 1123 hostname
    #[builder(into, default)]
    #[serde(rename = "hostname")]
    pub r#hostname: Box<Option<String>>,
    /// A list of hostname/IP mappings to add to the container's hosts file
    #[builder(into, default)]
    #[serde(rename = "hosts")]
    pub r#hosts: Box<Option<Vec<super::types::ServiceTaskSpecContainerSpecHost>>>,
    /// The image name to use for the containers of the service, like `nginx:1.17.6`. Also use the data-source or resource of `docker.RemoteImage` with the `repo_digest` or `docker.RegistryImage` with the `name` attribute for this, as shown in the examples.
    #[builder(into)]
    #[serde(rename = "image")]
    pub r#image: Box<String>,
    /// Isolation technology of the containers running the service. (Windows only). Defaults to `default`.
    #[builder(into, default)]
    #[serde(rename = "isolation")]
    pub r#isolation: Box<Option<String>>,
    /// User-defined key/value metadata
    #[builder(into, default)]
    #[serde(rename = "labels")]
    pub r#labels: Box<Option<Vec<super::types::ServiceTaskSpecContainerSpecLabel>>>,
    /// Specification for mounts to be added to containers created as part of the service
    #[builder(into, default)]
    #[serde(rename = "mounts")]
    pub r#mounts: Box<Option<Vec<super::types::ServiceTaskSpecContainerSpecMount>>>,
    /// Security options for the container
    #[builder(into, default)]
    #[serde(rename = "privileges")]
    pub r#privileges: Box<Option<super::types::ServiceTaskSpecContainerSpecPrivileges>>,
    /// Mount the container's root filesystem as read only
    #[builder(into, default)]
    #[serde(rename = "readOnly")]
    pub r#read_only: Box<Option<bool>>,
    /// References to zero or more secrets that will be exposed to the service
    #[builder(into, default)]
    #[serde(rename = "secrets")]
    pub r#secrets: Box<Option<Vec<super::types::ServiceTaskSpecContainerSpecSecret>>>,
    /// Amount of time to wait for the container to terminate before forcefully removing it (ms|s|m|h). If not specified or '0s' the destroy will not check if all tasks/containers of the service terminate.
    #[builder(into, default)]
    #[serde(rename = "stopGracePeriod")]
    pub r#stop_grace_period: Box<Option<String>>,
    /// Signal to stop the container
    #[builder(into, default)]
    #[serde(rename = "stopSignal")]
    pub r#stop_signal: Box<Option<String>>,
    /// Sysctls config (Linux only)
    #[builder(into, default)]
    #[serde(rename = "sysctl")]
    pub r#sysctl: Box<Option<std::collections::HashMap<String, String>>>,
    /// The user inside the container
    #[builder(into, default)]
    #[serde(rename = "user")]
    pub r#user: Box<Option<String>>,
}
