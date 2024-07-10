
#[derive(serde::Serialize)]
pub struct CacheFrom {
    #[serde(rename = "images")]
    pub r#images: Box<Option<Vec<String>>>,
}


#[derive(serde::Serialize)]
pub struct ContainerCapabilities {
    #[serde(rename = "adds")]
    pub r#adds: Box<Option<Vec<String>>>,
    #[serde(rename = "drops")]
    pub r#drops: Box<Option<Vec<String>>>,
}


#[derive(serde::Serialize)]
pub struct ContainerDevice {
    #[serde(rename = "containerPath")]
    pub r#container_path: Box<Option<String>>,
    #[serde(rename = "hostPath")]
    pub r#host_path: Box<String>,
    #[serde(rename = "permissions")]
    pub r#permissions: Box<Option<String>>,
}


#[derive(serde::Serialize)]
pub struct ContainerHealthcheck {
    #[serde(rename = "interval")]
    pub r#interval: Box<Option<String>>,
    #[serde(rename = "retries")]
    pub r#retries: Box<Option<i32>>,
    #[serde(rename = "startPeriod")]
    pub r#start_period: Box<Option<String>>,
    #[serde(rename = "tests")]
    pub r#tests: Box<Vec<String>>,
    #[serde(rename = "timeout")]
    pub r#timeout: Box<Option<String>>,
}


#[derive(serde::Serialize)]
pub struct ContainerHost {
    #[serde(rename = "host")]
    pub r#host: Box<String>,
    #[serde(rename = "ip")]
    pub r#ip: Box<String>,
}


#[derive(serde::Serialize)]
pub struct ContainerLabel {
    #[serde(rename = "label")]
    pub r#label: Box<String>,
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}


#[derive(serde::Serialize)]
pub struct ContainerMount {
    #[serde(rename = "bindOptions")]
    pub r#bind_options: Box<Option<crate::types::ContainerMountBindOptions>>,
    #[serde(rename = "readOnly")]
    pub r#read_only: Box<Option<bool>>,
    #[serde(rename = "source")]
    pub r#source: Box<Option<String>>,
    #[serde(rename = "target")]
    pub r#target: Box<String>,
    #[serde(rename = "tmpfsOptions")]
    pub r#tmpfs_options: Box<Option<crate::types::ContainerMountTmpfsOptions>>,
    #[serde(rename = "type")]
    pub r#type: Box<String>,
    #[serde(rename = "volumeOptions")]
    pub r#volume_options: Box<Option<crate::types::ContainerMountVolumeOptions>>,
}


#[derive(serde::Serialize)]
pub struct ContainerMountBindOptions {
    #[serde(rename = "propagation")]
    pub r#propagation: Box<Option<String>>,
}


#[derive(serde::Serialize)]
pub struct ContainerMountTmpfsOptions {
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<i32>>,
    #[serde(rename = "sizeBytes")]
    pub r#size_bytes: Box<Option<i32>>,
}


#[derive(serde::Serialize)]
pub struct ContainerMountVolumeOptions {
    #[serde(rename = "driverName")]
    pub r#driver_name: Box<Option<String>>,
    #[serde(rename = "driverOptions")]
    pub r#driver_options: Box<Option<std::collections::HashMap<String, String>>>,
    #[serde(rename = "labels")]
    pub r#labels: Box<Option<Vec<crate::types::ContainerMountVolumeOptionsLabel>>>,
    #[serde(rename = "noCopy")]
    pub r#no_copy: Box<Option<bool>>,
}


#[derive(serde::Serialize)]
pub struct ContainerMountVolumeOptionsLabel {
    #[serde(rename = "label")]
    pub r#label: Box<String>,
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}


#[derive(serde::Serialize)]
pub struct ContainerNetworkData {
    #[serde(rename = "gateway")]
    pub r#gateway: Box<Option<String>>,
    #[serde(rename = "globalIpv6Address")]
    pub r#global_ipv_6_address: Box<Option<String>>,
    #[serde(rename = "globalIpv6PrefixLength")]
    pub r#global_ipv_6_prefix_length: Box<Option<i32>>,
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Box<Option<String>>,
    #[serde(rename = "ipPrefixLength")]
    pub r#ip_prefix_length: Box<Option<i32>>,
    #[serde(rename = "ipv6Gateway")]
    pub r#ipv_6_gateway: Box<Option<String>>,
    #[serde(rename = "macAddress")]
    pub r#mac_address: Box<Option<String>>,
    #[serde(rename = "networkName")]
    pub r#network_name: Box<Option<String>>,
}


#[derive(serde::Serialize)]
pub struct ContainerNetworksAdvanced {
    #[serde(rename = "aliases")]
    pub r#aliases: Box<Option<Vec<String>>>,
    #[serde(rename = "ipv4Address")]
    pub r#ipv_4_address: Box<Option<String>>,
    #[serde(rename = "ipv6Address")]
    pub r#ipv_6_address: Box<Option<String>>,
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}


#[derive(serde::Serialize)]
pub struct ContainerPort {
    #[serde(rename = "external")]
    pub r#external: Box<Option<i32>>,
    #[serde(rename = "internal")]
    pub r#internal: Box<i32>,
    #[serde(rename = "ip")]
    pub r#ip: Box<Option<String>>,
    #[serde(rename = "protocol")]
    pub r#protocol: Box<Option<String>>,
}


#[derive(serde::Serialize)]
pub struct ContainerUlimit {
    #[serde(rename = "hard")]
    pub r#hard: Box<i32>,
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[serde(rename = "soft")]
    pub r#soft: Box<i32>,
}


#[derive(serde::Serialize)]
pub struct ContainerUpload {
    #[serde(rename = "content")]
    pub r#content: Box<Option<String>>,
    #[serde(rename = "contentBase64")]
    pub r#content_base_64: Box<Option<String>>,
    #[serde(rename = "executable")]
    pub r#executable: Box<Option<bool>>,
    #[serde(rename = "file")]
    pub r#file: Box<String>,
    #[serde(rename = "source")]
    pub r#source: Box<Option<String>>,
    #[serde(rename = "sourceHash")]
    pub r#source_hash: Box<Option<String>>,
}


#[derive(serde::Serialize)]
pub struct ContainerVolume {
    #[serde(rename = "containerPath")]
    pub r#container_path: Box<Option<String>>,
    #[serde(rename = "fromContainer")]
    pub r#from_container: Box<Option<String>>,
    #[serde(rename = "hostPath")]
    pub r#host_path: Box<Option<String>>,
    #[serde(rename = "readOnly")]
    pub r#read_only: Box<Option<bool>>,
    #[serde(rename = "volumeName")]
    pub r#volume_name: Box<Option<String>>,
}


#[derive(serde::Serialize)]
pub struct DockerBuild {
    #[serde(rename = "addHosts")]
    pub r#add_hosts: Box<Option<Vec<String>>>,
    #[serde(rename = "args")]
    pub r#args: Box<Option<std::collections::HashMap<String, String>>>,
    #[serde(rename = "builderVersion")]
    pub r#builder_version: Box<Option<crate::types::BuilderVersion>>,
    #[serde(rename = "cacheFrom")]
    pub r#cache_from: Box<Option<crate::types::CacheFrom>>,
    #[serde(rename = "context")]
    pub r#context: Box<Option<String>>,
    #[serde(rename = "dockerfile")]
    pub r#dockerfile: Box<Option<String>>,
    #[serde(rename = "network")]
    pub r#network: Box<Option<String>>,
    #[serde(rename = "platform")]
    pub r#platform: Box<Option<String>>,
    #[serde(rename = "target")]
    pub r#target: Box<Option<String>>,
}


#[derive(serde::Serialize)]
pub struct NetworkIpamConfig {
    #[serde(rename = "auxAddress")]
    pub r#aux_address: Box<Option<std::collections::HashMap<String, String>>>,
    #[serde(rename = "gateway")]
    pub r#gateway: Box<Option<String>>,
    #[serde(rename = "ipRange")]
    pub r#ip_range: Box<Option<String>>,
    #[serde(rename = "subnet")]
    pub r#subnet: Box<Option<String>>,
}


#[derive(serde::Serialize)]
pub struct NetworkLabel {
    #[serde(rename = "label")]
    pub r#label: Box<String>,
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}


#[derive(serde::Serialize)]
pub struct PluginGrantPermission {
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}


#[derive(serde::Serialize)]
pub struct ProviderRegistryAuth {
    #[serde(rename = "address")]
    pub r#address: Box<String>,
    #[serde(rename = "authDisabled")]
    pub r#auth_disabled: Box<Option<bool>>,
    #[serde(rename = "configFile")]
    pub r#config_file: Box<Option<String>>,
    #[serde(rename = "configFileContent")]
    pub r#config_file_content: Box<Option<String>>,
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    #[serde(rename = "username")]
    pub r#username: Box<Option<String>>,
}


#[derive(serde::Serialize)]
pub struct Registry {
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    #[serde(rename = "server")]
    pub r#server: Box<Option<String>>,
    #[serde(rename = "username")]
    pub r#username: Box<Option<String>>,
}


#[derive(serde::Serialize)]
pub struct RemoteImageBuild {
    #[serde(rename = "authConfigs")]
    pub r#auth_configs: Box<Option<Vec<crate::types::RemoteImageBuildAuthConfig>>>,
    #[serde(rename = "buildArg")]
    pub r#build_arg: Box<Option<std::collections::HashMap<String, String>>>,
    #[serde(rename = "buildArgs")]
    pub r#build_args: Box<Option<std::collections::HashMap<String, String>>>,
    #[serde(rename = "buildId")]
    pub r#build_id: Box<Option<String>>,
    #[serde(rename = "cacheFroms")]
    pub r#cache_froms: Box<Option<Vec<String>>>,
    #[serde(rename = "cgroupParent")]
    pub r#cgroup_parent: Box<Option<String>>,
    #[serde(rename = "context")]
    pub r#context: Box<String>,
    #[serde(rename = "cpuPeriod")]
    pub r#cpu_period: Box<Option<i32>>,
    #[serde(rename = "cpuQuota")]
    pub r#cpu_quota: Box<Option<i32>>,
    #[serde(rename = "cpuSetCpus")]
    pub r#cpu_set_cpus: Box<Option<String>>,
    #[serde(rename = "cpuSetMems")]
    pub r#cpu_set_mems: Box<Option<String>>,
    #[serde(rename = "cpuShares")]
    pub r#cpu_shares: Box<Option<i32>>,
    #[serde(rename = "dockerfile")]
    pub r#dockerfile: Box<Option<String>>,
    #[serde(rename = "extraHosts")]
    pub r#extra_hosts: Box<Option<Vec<String>>>,
    #[serde(rename = "forceRemove")]
    pub r#force_remove: Box<Option<bool>>,
    #[serde(rename = "isolation")]
    pub r#isolation: Box<Option<String>>,
    #[serde(rename = "label")]
    pub r#label: Box<Option<std::collections::HashMap<String, String>>>,
    #[serde(rename = "labels")]
    pub r#labels: Box<Option<std::collections::HashMap<String, String>>>,
    #[serde(rename = "memory")]
    pub r#memory: Box<Option<i32>>,
    #[serde(rename = "memorySwap")]
    pub r#memory_swap: Box<Option<i32>>,
    #[serde(rename = "networkMode")]
    pub r#network_mode: Box<Option<String>>,
    #[serde(rename = "noCache")]
    pub r#no_cache: Box<Option<bool>>,
    #[serde(rename = "platform")]
    pub r#platform: Box<Option<String>>,
    #[serde(rename = "pullParent")]
    pub r#pull_parent: Box<Option<bool>>,
    #[serde(rename = "remoteContext")]
    pub r#remote_context: Box<Option<String>>,
    #[serde(rename = "remove")]
    pub r#remove: Box<Option<bool>>,
    #[serde(rename = "securityOpts")]
    pub r#security_opts: Box<Option<Vec<String>>>,
    #[serde(rename = "sessionId")]
    pub r#session_id: Box<Option<String>>,
    #[serde(rename = "shmSize")]
    pub r#shm_size: Box<Option<i32>>,
    #[serde(rename = "squash")]
    pub r#squash: Box<Option<bool>>,
    #[serde(rename = "suppressOutput")]
    pub r#suppress_output: Box<Option<bool>>,
    #[serde(rename = "tags")]
    pub r#tags: Box<Option<Vec<String>>>,
    #[serde(rename = "target")]
    pub r#target: Box<Option<String>>,
    #[serde(rename = "ulimits")]
    pub r#ulimits: Box<Option<Vec<crate::types::RemoteImageBuildUlimit>>>,
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}


#[derive(serde::Serialize)]
pub struct RemoteImageBuildAuthConfig {
    #[serde(rename = "auth")]
    pub r#auth: Box<Option<String>>,
    #[serde(rename = "email")]
    pub r#email: Box<Option<String>>,
    #[serde(rename = "hostName")]
    pub r#host_name: Box<String>,
    #[serde(rename = "identityToken")]
    pub r#identity_token: Box<Option<String>>,
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    #[serde(rename = "registryToken")]
    pub r#registry_token: Box<Option<String>>,
    #[serde(rename = "serverAddress")]
    pub r#server_address: Box<Option<String>>,
    #[serde(rename = "userName")]
    pub r#user_name: Box<Option<String>>,
}


#[derive(serde::Serialize)]
pub struct RemoteImageBuildUlimit {
    #[serde(rename = "hard")]
    pub r#hard: Box<i32>,
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[serde(rename = "soft")]
    pub r#soft: Box<i32>,
}


#[derive(serde::Serialize)]
pub struct SecretLabel {
    #[serde(rename = "label")]
    pub r#label: Box<String>,
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}


#[derive(serde::Serialize)]
pub struct ServiceAuth {
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    #[serde(rename = "serverAddress")]
    pub r#server_address: Box<String>,
    #[serde(rename = "username")]
    pub r#username: Box<Option<String>>,
}


#[derive(serde::Serialize)]
pub struct ServiceConvergeConfig {
    #[serde(rename = "delay")]
    pub r#delay: Box<Option<String>>,
    #[serde(rename = "timeout")]
    pub r#timeout: Box<Option<String>>,
}


#[derive(serde::Serialize)]
pub struct ServiceEndpointSpec {
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
    #[serde(rename = "ports")]
    pub r#ports: Box<Option<Vec<crate::types::ServiceEndpointSpecPort>>>,
}


#[derive(serde::Serialize)]
pub struct ServiceEndpointSpecPort {
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    #[serde(rename = "protocol")]
    pub r#protocol: Box<Option<String>>,
    #[serde(rename = "publishMode")]
    pub r#publish_mode: Box<Option<String>>,
    #[serde(rename = "publishedPort")]
    pub r#published_port: Box<Option<i32>>,
    #[serde(rename = "targetPort")]
    pub r#target_port: Box<i32>,
}


#[derive(serde::Serialize)]
pub struct ServiceLabel {
    #[serde(rename = "label")]
    pub r#label: Box<String>,
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}


#[derive(serde::Serialize)]
pub struct ServiceMode {
    #[serde(rename = "global")]
    pub r#global: Box<Option<bool>>,
    #[serde(rename = "replicated")]
    pub r#replicated: Box<Option<crate::types::ServiceModeReplicated>>,
}


#[derive(serde::Serialize)]
pub struct ServiceModeReplicated {
    #[serde(rename = "replicas")]
    pub r#replicas: Box<Option<i32>>,
}


#[derive(serde::Serialize)]
pub struct ServiceRollbackConfig {
    #[serde(rename = "delay")]
    pub r#delay: Box<Option<String>>,
    #[serde(rename = "failureAction")]
    pub r#failure_action: Box<Option<String>>,
    #[serde(rename = "maxFailureRatio")]
    pub r#max_failure_ratio: Box<Option<String>>,
    #[serde(rename = "monitor")]
    pub r#monitor: Box<Option<String>>,
    #[serde(rename = "order")]
    pub r#order: Box<Option<String>>,
    #[serde(rename = "parallelism")]
    pub r#parallelism: Box<Option<i32>>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpec {
    #[serde(rename = "containerSpec")]
    pub r#container_spec: Box<crate::types::ServiceTaskSpecContainerSpec>,
    #[serde(rename = "forceUpdate")]
    pub r#force_update: Box<Option<i32>>,
    #[serde(rename = "logDriver")]
    pub r#log_driver: Box<Option<crate::types::ServiceTaskSpecLogDriver>>,
    #[serde(rename = "networksAdvanceds")]
    pub r#networks_advanceds: Box<Option<Vec<crate::types::ServiceTaskSpecNetworksAdvanced>>>,
    #[serde(rename = "placement")]
    pub r#placement: Box<Option<crate::types::ServiceTaskSpecPlacement>>,
    #[serde(rename = "resources")]
    pub r#resources: Box<Option<crate::types::ServiceTaskSpecResources>>,
    #[serde(rename = "restartPolicy")]
    pub r#restart_policy: Box<Option<crate::types::ServiceTaskSpecRestartPolicy>>,
    #[serde(rename = "runtime")]
    pub r#runtime: Box<Option<String>>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpec {
    #[serde(rename = "args")]
    pub r#args: Box<Option<Vec<String>>>,
    #[serde(rename = "commands")]
    pub r#commands: Box<Option<Vec<String>>>,
    #[serde(rename = "configs")]
    pub r#configs: Box<Option<Vec<crate::types::ServiceTaskSpecContainerSpecConfig>>>,
    #[serde(rename = "dir")]
    pub r#dir: Box<Option<String>>,
    #[serde(rename = "dnsConfig")]
    pub r#dns_config: Box<Option<crate::types::ServiceTaskSpecContainerSpecDnsConfig>>,
    #[serde(rename = "env")]
    pub r#env: Box<Option<std::collections::HashMap<String, String>>>,
    #[serde(rename = "groups")]
    pub r#groups: Box<Option<Vec<String>>>,
    #[serde(rename = "healthcheck")]
    pub r#healthcheck: Box<Option<crate::types::ServiceTaskSpecContainerSpecHealthcheck>>,
    #[serde(rename = "hostname")]
    pub r#hostname: Box<Option<String>>,
    #[serde(rename = "hosts")]
    pub r#hosts: Box<Option<Vec<crate::types::ServiceTaskSpecContainerSpecHost>>>,
    #[serde(rename = "image")]
    pub r#image: Box<String>,
    #[serde(rename = "isolation")]
    pub r#isolation: Box<Option<String>>,
    #[serde(rename = "labels")]
    pub r#labels: Box<Option<Vec<crate::types::ServiceTaskSpecContainerSpecLabel>>>,
    #[serde(rename = "mounts")]
    pub r#mounts: Box<Option<Vec<crate::types::ServiceTaskSpecContainerSpecMount>>>,
    #[serde(rename = "privileges")]
    pub r#privileges: Box<Option<crate::types::ServiceTaskSpecContainerSpecPrivileges>>,
    #[serde(rename = "readOnly")]
    pub r#read_only: Box<Option<bool>>,
    #[serde(rename = "secrets")]
    pub r#secrets: Box<Option<Vec<crate::types::ServiceTaskSpecContainerSpecSecret>>>,
    #[serde(rename = "stopGracePeriod")]
    pub r#stop_grace_period: Box<Option<String>>,
    #[serde(rename = "stopSignal")]
    pub r#stop_signal: Box<Option<String>>,
    #[serde(rename = "sysctl")]
    pub r#sysctl: Box<Option<std::collections::HashMap<String, String>>>,
    #[serde(rename = "user")]
    pub r#user: Box<Option<String>>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecConfig {
    #[serde(rename = "configId")]
    pub r#config_id: Box<String>,
    #[serde(rename = "configName")]
    pub r#config_name: Box<Option<String>>,
    #[serde(rename = "fileGid")]
    pub r#file_gid: Box<Option<String>>,
    #[serde(rename = "fileMode")]
    pub r#file_mode: Box<Option<i32>>,
    #[serde(rename = "fileName")]
    pub r#file_name: Box<String>,
    #[serde(rename = "fileUid")]
    pub r#file_uid: Box<Option<String>>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecDnsConfig {
    #[serde(rename = "nameservers")]
    pub r#nameservers: Box<Vec<String>>,
    #[serde(rename = "options")]
    pub r#options: Box<Option<Vec<String>>>,
    #[serde(rename = "searches")]
    pub r#searches: Box<Option<Vec<String>>>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecHealthcheck {
    #[serde(rename = "interval")]
    pub r#interval: Box<Option<String>>,
    #[serde(rename = "retries")]
    pub r#retries: Box<Option<i32>>,
    #[serde(rename = "startPeriod")]
    pub r#start_period: Box<Option<String>>,
    #[serde(rename = "tests")]
    pub r#tests: Box<Vec<String>>,
    #[serde(rename = "timeout")]
    pub r#timeout: Box<Option<String>>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecHost {
    #[serde(rename = "host")]
    pub r#host: Box<String>,
    #[serde(rename = "ip")]
    pub r#ip: Box<String>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecLabel {
    #[serde(rename = "label")]
    pub r#label: Box<String>,
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecMount {
    #[serde(rename = "bindOptions")]
    pub r#bind_options: Box<Option<crate::types::ServiceTaskSpecContainerSpecMountBindOptions>>,
    #[serde(rename = "readOnly")]
    pub r#read_only: Box<Option<bool>>,
    #[serde(rename = "source")]
    pub r#source: Box<Option<String>>,
    #[serde(rename = "target")]
    pub r#target: Box<String>,
    #[serde(rename = "tmpfsOptions")]
    pub r#tmpfs_options: Box<Option<crate::types::ServiceTaskSpecContainerSpecMountTmpfsOptions>>,
    #[serde(rename = "type")]
    pub r#type: Box<String>,
    #[serde(rename = "volumeOptions")]
    pub r#volume_options: Box<Option<crate::types::ServiceTaskSpecContainerSpecMountVolumeOptions>>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecMountBindOptions {
    #[serde(rename = "propagation")]
    pub r#propagation: Box<Option<String>>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecMountTmpfsOptions {
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<i32>>,
    #[serde(rename = "sizeBytes")]
    pub r#size_bytes: Box<Option<i32>>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecMountVolumeOptions {
    #[serde(rename = "driverName")]
    pub r#driver_name: Box<Option<String>>,
    #[serde(rename = "driverOptions")]
    pub r#driver_options: Box<Option<std::collections::HashMap<String, String>>>,
    #[serde(rename = "labels")]
    pub r#labels: Box<Option<Vec<crate::types::ServiceTaskSpecContainerSpecMountVolumeOptionsLabel>>>,
    #[serde(rename = "noCopy")]
    pub r#no_copy: Box<Option<bool>>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecMountVolumeOptionsLabel {
    #[serde(rename = "label")]
    pub r#label: Box<String>,
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecPrivileges {
    #[serde(rename = "credentialSpec")]
    pub r#credential_spec: Box<Option<crate::types::ServiceTaskSpecContainerSpecPrivilegesCredentialSpec>>,
    #[serde(rename = "seLinuxContext")]
    pub r#se_linux_context: Box<Option<crate::types::ServiceTaskSpecContainerSpecPrivilegesSeLinuxContext>>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecPrivilegesCredentialSpec {
    #[serde(rename = "file")]
    pub r#file: Box<Option<String>>,
    #[serde(rename = "registry")]
    pub r#registry: Box<Option<String>>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecPrivilegesSeLinuxContext {
    #[serde(rename = "disable")]
    pub r#disable: Box<Option<bool>>,
    #[serde(rename = "level")]
    pub r#level: Box<Option<String>>,
    #[serde(rename = "role")]
    pub r#role: Box<Option<String>>,
    #[serde(rename = "type")]
    pub r#type: Box<Option<String>>,
    #[serde(rename = "user")]
    pub r#user: Box<Option<String>>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecSecret {
    #[serde(rename = "fileGid")]
    pub r#file_gid: Box<Option<String>>,
    #[serde(rename = "fileMode")]
    pub r#file_mode: Box<Option<i32>>,
    #[serde(rename = "fileName")]
    pub r#file_name: Box<String>,
    #[serde(rename = "fileUid")]
    pub r#file_uid: Box<Option<String>>,
    #[serde(rename = "secretId")]
    pub r#secret_id: Box<String>,
    #[serde(rename = "secretName")]
    pub r#secret_name: Box<Option<String>>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecLogDriver {
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[serde(rename = "options")]
    pub r#options: Box<Option<std::collections::HashMap<String, String>>>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecNetworksAdvanced {
    #[serde(rename = "aliases")]
    pub r#aliases: Box<Option<Vec<String>>>,
    #[serde(rename = "driverOpts")]
    pub r#driver_opts: Box<Option<Vec<String>>>,
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecPlacement {
    #[serde(rename = "constraints")]
    pub r#constraints: Box<Option<Vec<String>>>,
    #[serde(rename = "maxReplicas")]
    pub r#max_replicas: Box<Option<i32>>,
    #[serde(rename = "platforms")]
    pub r#platforms: Box<Option<Vec<crate::types::ServiceTaskSpecPlacementPlatform>>>,
    #[serde(rename = "prefs")]
    pub r#prefs: Box<Option<Vec<String>>>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecPlacementPlatform {
    #[serde(rename = "architecture")]
    pub r#architecture: Box<String>,
    #[serde(rename = "os")]
    pub r#os: Box<String>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecResources {
    #[serde(rename = "limits")]
    pub r#limits: Box<Option<crate::types::ServiceTaskSpecResourcesLimits>>,
    #[serde(rename = "reservation")]
    pub r#reservation: Box<Option<crate::types::ServiceTaskSpecResourcesReservation>>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecResourcesLimits {
    #[serde(rename = "memoryBytes")]
    pub r#memory_bytes: Box<Option<i32>>,
    #[serde(rename = "nanoCpus")]
    pub r#nano_cpus: Box<Option<i32>>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecResourcesReservation {
    #[serde(rename = "genericResources")]
    pub r#generic_resources: Box<Option<crate::types::ServiceTaskSpecResourcesReservationGenericResources>>,
    #[serde(rename = "memoryBytes")]
    pub r#memory_bytes: Box<Option<i32>>,
    #[serde(rename = "nanoCpus")]
    pub r#nano_cpus: Box<Option<i32>>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecResourcesReservationGenericResources {
    #[serde(rename = "discreteResourcesSpecs")]
    pub r#discrete_resources_specs: Box<Option<Vec<String>>>,
    #[serde(rename = "namedResourcesSpecs")]
    pub r#named_resources_specs: Box<Option<Vec<String>>>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecRestartPolicy {
    #[serde(rename = "condition")]
    pub r#condition: Box<Option<String>>,
    #[serde(rename = "delay")]
    pub r#delay: Box<Option<String>>,
    #[serde(rename = "maxAttempts")]
    pub r#max_attempts: Box<Option<i32>>,
    #[serde(rename = "window")]
    pub r#window: Box<Option<String>>,
}


#[derive(serde::Serialize)]
pub struct ServiceUpdateConfig {
    #[serde(rename = "delay")]
    pub r#delay: Box<Option<String>>,
    #[serde(rename = "failureAction")]
    pub r#failure_action: Box<Option<String>>,
    #[serde(rename = "maxFailureRatio")]
    pub r#max_failure_ratio: Box<Option<String>>,
    #[serde(rename = "monitor")]
    pub r#monitor: Box<Option<String>>,
    #[serde(rename = "order")]
    pub r#order: Box<Option<String>>,
    #[serde(rename = "parallelism")]
    pub r#parallelism: Box<Option<i32>>,
}


#[derive(serde::Serialize)]
pub struct VolumeLabel {
    #[serde(rename = "label")]
    pub r#label: Box<String>,
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}


#[derive(serde::Serialize)]
pub struct getNetworkIpamConfig {
    #[serde(rename = "auxAddress")]
    pub r#aux_address: Box<Option<std::collections::HashMap<String, String>>>,
    #[serde(rename = "gateway")]
    pub r#gateway: Box<Option<String>>,
    #[serde(rename = "ipRange")]
    pub r#ip_range: Box<Option<String>>,
    #[serde(rename = "subnet")]
    pub r#subnet: Box<Option<String>>,
}


#[derive(serde::Serialize)]
pub struct registryAuth {
    #[serde(rename = "address")]
    pub r#address: Box<String>,
    #[serde(rename = "authDisabled")]
    pub r#auth_disabled: Box<Option<bool>>,
    #[serde(rename = "configFile")]
    pub r#config_file: Box<Option<String>>,
    #[serde(rename = "configFileContent")]
    pub r#config_file_content: Box<Option<String>>,
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    #[serde(rename = "username")]
    pub r#username: Box<Option<String>>,
}


pub type BuilderVersion = String;
