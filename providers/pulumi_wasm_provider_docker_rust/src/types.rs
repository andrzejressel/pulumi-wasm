
#[derive(serde::Serialize)]
pub struct CacheFrom {
    #[serde(rename = "images")]
    pub r#images: Option<Vec<String>>,
}


#[derive(serde::Serialize)]
pub struct ContainerCapabilities {
    #[serde(rename = "adds")]
    pub r#adds: Option<Vec<String>>,
    #[serde(rename = "drops")]
    pub r#drops: Option<Vec<String>>,
}


#[derive(serde::Serialize)]
pub struct ContainerDevice {
    #[serde(rename = "containerPath")]
    pub r#container_path: Option<String>,
    #[serde(rename = "hostPath")]
    pub r#host_path: String,
    #[serde(rename = "permissions")]
    pub r#permissions: Option<String>,
}


#[derive(serde::Serialize)]
pub struct ContainerHealthcheck {
    #[serde(rename = "interval")]
    pub r#interval: Option<String>,
    #[serde(rename = "retries")]
    pub r#retries: Option<i32>,
    #[serde(rename = "startPeriod")]
    pub r#start_period: Option<String>,
    #[serde(rename = "tests")]
    pub r#tests: Vec<String>,
    #[serde(rename = "timeout")]
    pub r#timeout: Option<String>,
}


#[derive(serde::Serialize)]
pub struct ContainerHost {
    #[serde(rename = "host")]
    pub r#host: String,
    #[serde(rename = "ip")]
    pub r#ip: String,
}


#[derive(serde::Serialize)]
pub struct ContainerLabel {
    #[serde(rename = "label")]
    pub r#label: String,
    #[serde(rename = "value")]
    pub r#value: String,
}


#[derive(serde::Serialize)]
pub struct ContainerMount {
    #[serde(rename = "bindOptions")]
    pub r#bind_options: Option<crate::types::ContainerMountBindOptions>,
    #[serde(rename = "readOnly")]
    pub r#read_only: Option<bool>,
    #[serde(rename = "source")]
    pub r#source: Option<String>,
    #[serde(rename = "target")]
    pub r#target: String,
    #[serde(rename = "tmpfsOptions")]
    pub r#tmpfs_options: Option<crate::types::ContainerMountTmpfsOptions>,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "volumeOptions")]
    pub r#volume_options: Option<crate::types::ContainerMountVolumeOptions>,
}


#[derive(serde::Serialize)]
pub struct ContainerMountBindOptions {
    #[serde(rename = "propagation")]
    pub r#propagation: Option<String>,
}


#[derive(serde::Serialize)]
pub struct ContainerMountTmpfsOptions {
    #[serde(rename = "mode")]
    pub r#mode: Option<i32>,
    #[serde(rename = "sizeBytes")]
    pub r#size_bytes: Option<i32>,
}


#[derive(serde::Serialize)]
pub struct ContainerMountVolumeOptions {
    #[serde(rename = "driverName")]
    pub r#driver_name: Option<String>,
    #[serde(rename = "driverOptions")]
    pub r#driver_options: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "labels")]
    pub r#labels: Option<Vec<crate::types::ContainerMountVolumeOptionsLabel>>,
    #[serde(rename = "noCopy")]
    pub r#no_copy: Option<bool>,
}


#[derive(serde::Serialize)]
pub struct ContainerMountVolumeOptionsLabel {
    #[serde(rename = "label")]
    pub r#label: String,
    #[serde(rename = "value")]
    pub r#value: String,
}


#[derive(serde::Serialize)]
pub struct ContainerNetworkData {
    #[serde(rename = "gateway")]
    pub r#gateway: Option<String>,
    #[serde(rename = "globalIpv6Address")]
    pub r#global_ipv_6_address: Option<String>,
    #[serde(rename = "globalIpv6PrefixLength")]
    pub r#global_ipv_6_prefix_length: Option<i32>,
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Option<String>,
    #[serde(rename = "ipPrefixLength")]
    pub r#ip_prefix_length: Option<i32>,
    #[serde(rename = "ipv6Gateway")]
    pub r#ipv_6_gateway: Option<String>,
    #[serde(rename = "macAddress")]
    pub r#mac_address: Option<String>,
    #[serde(rename = "networkName")]
    pub r#network_name: Option<String>,
}


#[derive(serde::Serialize)]
pub struct ContainerNetworksAdvanced {
    #[serde(rename = "aliases")]
    pub r#aliases: Option<Vec<String>>,
    #[serde(rename = "ipv4Address")]
    pub r#ipv_4_address: Option<String>,
    #[serde(rename = "ipv6Address")]
    pub r#ipv_6_address: Option<String>,
    #[serde(rename = "name")]
    pub r#name: String,
}


#[derive(serde::Serialize)]
pub struct ContainerPort {
    #[serde(rename = "external")]
    pub r#external: Option<i32>,
    #[serde(rename = "internal")]
    pub r#internal: i32,
    #[serde(rename = "ip")]
    pub r#ip: Option<String>,
    #[serde(rename = "protocol")]
    pub r#protocol: Option<String>,
}


#[derive(serde::Serialize)]
pub struct ContainerUlimit {
    #[serde(rename = "hard")]
    pub r#hard: i32,
    #[serde(rename = "name")]
    pub r#name: String,
    #[serde(rename = "soft")]
    pub r#soft: i32,
}


#[derive(serde::Serialize)]
pub struct ContainerUpload {
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    #[serde(rename = "contentBase64")]
    pub r#content_base_64: Option<String>,
    #[serde(rename = "executable")]
    pub r#executable: Option<bool>,
    #[serde(rename = "file")]
    pub r#file: String,
    #[serde(rename = "source")]
    pub r#source: Option<String>,
    #[serde(rename = "sourceHash")]
    pub r#source_hash: Option<String>,
}


#[derive(serde::Serialize)]
pub struct ContainerVolume {
    #[serde(rename = "containerPath")]
    pub r#container_path: Option<String>,
    #[serde(rename = "fromContainer")]
    pub r#from_container: Option<String>,
    #[serde(rename = "hostPath")]
    pub r#host_path: Option<String>,
    #[serde(rename = "readOnly")]
    pub r#read_only: Option<bool>,
    #[serde(rename = "volumeName")]
    pub r#volume_name: Option<String>,
}


#[derive(serde::Serialize)]
pub struct DockerBuild {
    #[serde(rename = "addHosts")]
    pub r#add_hosts: Option<Vec<String>>,
    #[serde(rename = "args")]
    pub r#args: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "builderVersion")]
    pub r#builder_version: Option<crate::types::BuilderVersion>,
    #[serde(rename = "cacheFrom")]
    pub r#cache_from: Option<crate::types::CacheFrom>,
    #[serde(rename = "context")]
    pub r#context: Option<String>,
    #[serde(rename = "dockerfile")]
    pub r#dockerfile: Option<String>,
    #[serde(rename = "network")]
    pub r#network: Option<String>,
    #[serde(rename = "platform")]
    pub r#platform: Option<String>,
    #[serde(rename = "target")]
    pub r#target: Option<String>,
}


#[derive(serde::Serialize)]
pub struct NetworkIpamConfig {
    #[serde(rename = "auxAddress")]
    pub r#aux_address: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "gateway")]
    pub r#gateway: Option<String>,
    #[serde(rename = "ipRange")]
    pub r#ip_range: Option<String>,
    #[serde(rename = "subnet")]
    pub r#subnet: Option<String>,
}


#[derive(serde::Serialize)]
pub struct NetworkLabel {
    #[serde(rename = "label")]
    pub r#label: String,
    #[serde(rename = "value")]
    pub r#value: String,
}


#[derive(serde::Serialize)]
pub struct PluginGrantPermission {
    #[serde(rename = "name")]
    pub r#name: String,
    #[serde(rename = "values")]
    pub r#values: Vec<String>,
}


#[derive(serde::Serialize)]
pub struct ProviderRegistryAuth {
    #[serde(rename = "address")]
    pub r#address: String,
    #[serde(rename = "authDisabled")]
    pub r#auth_disabled: Option<bool>,
    #[serde(rename = "configFile")]
    pub r#config_file: Option<String>,
    #[serde(rename = "configFileContent")]
    pub r#config_file_content: Option<String>,
    #[serde(rename = "password")]
    pub r#password: Option<String>,
    #[serde(rename = "username")]
    pub r#username: Option<String>,
}


#[derive(serde::Serialize)]
pub struct Registry {
    #[serde(rename = "password")]
    pub r#password: Option<String>,
    #[serde(rename = "server")]
    pub r#server: Option<String>,
    #[serde(rename = "username")]
    pub r#username: Option<String>,
}


#[derive(serde::Serialize)]
pub struct RemoteImageBuild {
    #[serde(rename = "authConfigs")]
    pub r#auth_configs: Option<Vec<crate::types::RemoteImageBuildAuthConfig>>,
    #[serde(rename = "buildArg")]
    pub r#build_arg: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "buildArgs")]
    pub r#build_args: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "buildId")]
    pub r#build_id: Option<String>,
    #[serde(rename = "cacheFroms")]
    pub r#cache_froms: Option<Vec<String>>,
    #[serde(rename = "cgroupParent")]
    pub r#cgroup_parent: Option<String>,
    #[serde(rename = "context")]
    pub r#context: String,
    #[serde(rename = "cpuPeriod")]
    pub r#cpu_period: Option<i32>,
    #[serde(rename = "cpuQuota")]
    pub r#cpu_quota: Option<i32>,
    #[serde(rename = "cpuSetCpus")]
    pub r#cpu_set_cpus: Option<String>,
    #[serde(rename = "cpuSetMems")]
    pub r#cpu_set_mems: Option<String>,
    #[serde(rename = "cpuShares")]
    pub r#cpu_shares: Option<i32>,
    #[serde(rename = "dockerfile")]
    pub r#dockerfile: Option<String>,
    #[serde(rename = "extraHosts")]
    pub r#extra_hosts: Option<Vec<String>>,
    #[serde(rename = "forceRemove")]
    pub r#force_remove: Option<bool>,
    #[serde(rename = "isolation")]
    pub r#isolation: Option<String>,
    #[serde(rename = "label")]
    pub r#label: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "labels")]
    pub r#labels: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "memory")]
    pub r#memory: Option<i32>,
    #[serde(rename = "memorySwap")]
    pub r#memory_swap: Option<i32>,
    #[serde(rename = "networkMode")]
    pub r#network_mode: Option<String>,
    #[serde(rename = "noCache")]
    pub r#no_cache: Option<bool>,
    #[serde(rename = "platform")]
    pub r#platform: Option<String>,
    #[serde(rename = "pullParent")]
    pub r#pull_parent: Option<bool>,
    #[serde(rename = "remoteContext")]
    pub r#remote_context: Option<String>,
    #[serde(rename = "remove")]
    pub r#remove: Option<bool>,
    #[serde(rename = "securityOpts")]
    pub r#security_opts: Option<Vec<String>>,
    #[serde(rename = "sessionId")]
    pub r#session_id: Option<String>,
    #[serde(rename = "shmSize")]
    pub r#shm_size: Option<i32>,
    #[serde(rename = "squash")]
    pub r#squash: Option<bool>,
    #[serde(rename = "suppressOutput")]
    pub r#suppress_output: Option<bool>,
    #[serde(rename = "tags")]
    pub r#tags: Option<Vec<String>>,
    #[serde(rename = "target")]
    pub r#target: Option<String>,
    #[serde(rename = "ulimits")]
    pub r#ulimits: Option<Vec<crate::types::RemoteImageBuildUlimit>>,
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}


#[derive(serde::Serialize)]
pub struct RemoteImageBuildAuthConfig {
    #[serde(rename = "auth")]
    pub r#auth: Option<String>,
    #[serde(rename = "email")]
    pub r#email: Option<String>,
    #[serde(rename = "hostName")]
    pub r#host_name: String,
    #[serde(rename = "identityToken")]
    pub r#identity_token: Option<String>,
    #[serde(rename = "password")]
    pub r#password: Option<String>,
    #[serde(rename = "registryToken")]
    pub r#registry_token: Option<String>,
    #[serde(rename = "serverAddress")]
    pub r#server_address: Option<String>,
    #[serde(rename = "userName")]
    pub r#user_name: Option<String>,
}


#[derive(serde::Serialize)]
pub struct RemoteImageBuildUlimit {
    #[serde(rename = "hard")]
    pub r#hard: i32,
    #[serde(rename = "name")]
    pub r#name: String,
    #[serde(rename = "soft")]
    pub r#soft: i32,
}


#[derive(serde::Serialize)]
pub struct SecretLabel {
    #[serde(rename = "label")]
    pub r#label: String,
    #[serde(rename = "value")]
    pub r#value: String,
}


#[derive(serde::Serialize)]
pub struct ServiceAuth {
    #[serde(rename = "password")]
    pub r#password: Option<String>,
    #[serde(rename = "serverAddress")]
    pub r#server_address: String,
    #[serde(rename = "username")]
    pub r#username: Option<String>,
}


#[derive(serde::Serialize)]
pub struct ServiceConvergeConfig {
    #[serde(rename = "delay")]
    pub r#delay: Option<String>,
    #[serde(rename = "timeout")]
    pub r#timeout: Option<String>,
}


#[derive(serde::Serialize)]
pub struct ServiceEndpointSpec {
    #[serde(rename = "mode")]
    pub r#mode: Option<String>,
    #[serde(rename = "ports")]
    pub r#ports: Option<Vec<crate::types::ServiceEndpointSpecPort>>,
}


#[derive(serde::Serialize)]
pub struct ServiceEndpointSpecPort {
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    #[serde(rename = "protocol")]
    pub r#protocol: Option<String>,
    #[serde(rename = "publishMode")]
    pub r#publish_mode: Option<String>,
    #[serde(rename = "publishedPort")]
    pub r#published_port: Option<i32>,
    #[serde(rename = "targetPort")]
    pub r#target_port: i32,
}


#[derive(serde::Serialize)]
pub struct ServiceLabel {
    #[serde(rename = "label")]
    pub r#label: String,
    #[serde(rename = "value")]
    pub r#value: String,
}


#[derive(serde::Serialize)]
pub struct ServiceMode {
    #[serde(rename = "global")]
    pub r#global: Option<bool>,
    #[serde(rename = "replicated")]
    pub r#replicated: Option<crate::types::ServiceModeReplicated>,
}


#[derive(serde::Serialize)]
pub struct ServiceModeReplicated {
    #[serde(rename = "replicas")]
    pub r#replicas: Option<i32>,
}


#[derive(serde::Serialize)]
pub struct ServiceRollbackConfig {
    #[serde(rename = "delay")]
    pub r#delay: Option<String>,
    #[serde(rename = "failureAction")]
    pub r#failure_action: Option<String>,
    #[serde(rename = "maxFailureRatio")]
    pub r#max_failure_ratio: Option<String>,
    #[serde(rename = "monitor")]
    pub r#monitor: Option<String>,
    #[serde(rename = "order")]
    pub r#order: Option<String>,
    #[serde(rename = "parallelism")]
    pub r#parallelism: Option<i32>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpec {
    #[serde(rename = "containerSpec")]
    pub r#container_spec: crate::types::ServiceTaskSpecContainerSpec,
    #[serde(rename = "forceUpdate")]
    pub r#force_update: Option<i32>,
    #[serde(rename = "logDriver")]
    pub r#log_driver: Option<crate::types::ServiceTaskSpecLogDriver>,
    #[serde(rename = "networksAdvanceds")]
    pub r#networks_advanceds: Option<Vec<crate::types::ServiceTaskSpecNetworksAdvanced>>,
    #[serde(rename = "placement")]
    pub r#placement: Option<crate::types::ServiceTaskSpecPlacement>,
    #[serde(rename = "resources")]
    pub r#resources: Option<crate::types::ServiceTaskSpecResources>,
    #[serde(rename = "restartPolicy")]
    pub r#restart_policy: Option<crate::types::ServiceTaskSpecRestartPolicy>,
    #[serde(rename = "runtime")]
    pub r#runtime: Option<String>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpec {
    #[serde(rename = "args")]
    pub r#args: Option<Vec<String>>,
    #[serde(rename = "commands")]
    pub r#commands: Option<Vec<String>>,
    #[serde(rename = "configs")]
    pub r#configs: Option<Vec<crate::types::ServiceTaskSpecContainerSpecConfig>>,
    #[serde(rename = "dir")]
    pub r#dir: Option<String>,
    #[serde(rename = "dnsConfig")]
    pub r#dns_config: Option<crate::types::ServiceTaskSpecContainerSpecDnsConfig>,
    #[serde(rename = "env")]
    pub r#env: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "groups")]
    pub r#groups: Option<Vec<String>>,
    #[serde(rename = "healthcheck")]
    pub r#healthcheck: Option<crate::types::ServiceTaskSpecContainerSpecHealthcheck>,
    #[serde(rename = "hostname")]
    pub r#hostname: Option<String>,
    #[serde(rename = "hosts")]
    pub r#hosts: Option<Vec<crate::types::ServiceTaskSpecContainerSpecHost>>,
    #[serde(rename = "image")]
    pub r#image: String,
    #[serde(rename = "isolation")]
    pub r#isolation: Option<String>,
    #[serde(rename = "labels")]
    pub r#labels: Option<Vec<crate::types::ServiceTaskSpecContainerSpecLabel>>,
    #[serde(rename = "mounts")]
    pub r#mounts: Option<Vec<crate::types::ServiceTaskSpecContainerSpecMount>>,
    #[serde(rename = "privileges")]
    pub r#privileges: Option<crate::types::ServiceTaskSpecContainerSpecPrivileges>,
    #[serde(rename = "readOnly")]
    pub r#read_only: Option<bool>,
    #[serde(rename = "secrets")]
    pub r#secrets: Option<Vec<crate::types::ServiceTaskSpecContainerSpecSecret>>,
    #[serde(rename = "stopGracePeriod")]
    pub r#stop_grace_period: Option<String>,
    #[serde(rename = "stopSignal")]
    pub r#stop_signal: Option<String>,
    #[serde(rename = "sysctl")]
    pub r#sysctl: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "user")]
    pub r#user: Option<String>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecConfig {
    #[serde(rename = "configId")]
    pub r#config_id: String,
    #[serde(rename = "configName")]
    pub r#config_name: Option<String>,
    #[serde(rename = "fileGid")]
    pub r#file_gid: Option<String>,
    #[serde(rename = "fileMode")]
    pub r#file_mode: Option<i32>,
    #[serde(rename = "fileName")]
    pub r#file_name: String,
    #[serde(rename = "fileUid")]
    pub r#file_uid: Option<String>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecDnsConfig {
    #[serde(rename = "nameservers")]
    pub r#nameservers: Vec<String>,
    #[serde(rename = "options")]
    pub r#options: Option<Vec<String>>,
    #[serde(rename = "searches")]
    pub r#searches: Option<Vec<String>>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecHealthcheck {
    #[serde(rename = "interval")]
    pub r#interval: Option<String>,
    #[serde(rename = "retries")]
    pub r#retries: Option<i32>,
    #[serde(rename = "startPeriod")]
    pub r#start_period: Option<String>,
    #[serde(rename = "tests")]
    pub r#tests: Vec<String>,
    #[serde(rename = "timeout")]
    pub r#timeout: Option<String>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecHost {
    #[serde(rename = "host")]
    pub r#host: String,
    #[serde(rename = "ip")]
    pub r#ip: String,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecLabel {
    #[serde(rename = "label")]
    pub r#label: String,
    #[serde(rename = "value")]
    pub r#value: String,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecMount {
    #[serde(rename = "bindOptions")]
    pub r#bind_options: Option<crate::types::ServiceTaskSpecContainerSpecMountBindOptions>,
    #[serde(rename = "readOnly")]
    pub r#read_only: Option<bool>,
    #[serde(rename = "source")]
    pub r#source: Option<String>,
    #[serde(rename = "target")]
    pub r#target: String,
    #[serde(rename = "tmpfsOptions")]
    pub r#tmpfs_options: Option<crate::types::ServiceTaskSpecContainerSpecMountTmpfsOptions>,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "volumeOptions")]
    pub r#volume_options: Option<crate::types::ServiceTaskSpecContainerSpecMountVolumeOptions>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecMountBindOptions {
    #[serde(rename = "propagation")]
    pub r#propagation: Option<String>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecMountTmpfsOptions {
    #[serde(rename = "mode")]
    pub r#mode: Option<i32>,
    #[serde(rename = "sizeBytes")]
    pub r#size_bytes: Option<i32>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecMountVolumeOptions {
    #[serde(rename = "driverName")]
    pub r#driver_name: Option<String>,
    #[serde(rename = "driverOptions")]
    pub r#driver_options: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "labels")]
    pub r#labels: Option<Vec<crate::types::ServiceTaskSpecContainerSpecMountVolumeOptionsLabel>>,
    #[serde(rename = "noCopy")]
    pub r#no_copy: Option<bool>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecMountVolumeOptionsLabel {
    #[serde(rename = "label")]
    pub r#label: String,
    #[serde(rename = "value")]
    pub r#value: String,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecPrivileges {
    #[serde(rename = "credentialSpec")]
    pub r#credential_spec: Option<crate::types::ServiceTaskSpecContainerSpecPrivilegesCredentialSpec>,
    #[serde(rename = "seLinuxContext")]
    pub r#se_linux_context: Option<crate::types::ServiceTaskSpecContainerSpecPrivilegesSeLinuxContext>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecPrivilegesCredentialSpec {
    #[serde(rename = "file")]
    pub r#file: Option<String>,
    #[serde(rename = "registry")]
    pub r#registry: Option<String>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecPrivilegesSeLinuxContext {
    #[serde(rename = "disable")]
    pub r#disable: Option<bool>,
    #[serde(rename = "level")]
    pub r#level: Option<String>,
    #[serde(rename = "role")]
    pub r#role: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    #[serde(rename = "user")]
    pub r#user: Option<String>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecSecret {
    #[serde(rename = "fileGid")]
    pub r#file_gid: Option<String>,
    #[serde(rename = "fileMode")]
    pub r#file_mode: Option<i32>,
    #[serde(rename = "fileName")]
    pub r#file_name: String,
    #[serde(rename = "fileUid")]
    pub r#file_uid: Option<String>,
    #[serde(rename = "secretId")]
    pub r#secret_id: String,
    #[serde(rename = "secretName")]
    pub r#secret_name: Option<String>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecLogDriver {
    #[serde(rename = "name")]
    pub r#name: String,
    #[serde(rename = "options")]
    pub r#options: Option<std::collections::HashMap<String, String>>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecNetworksAdvanced {
    #[serde(rename = "aliases")]
    pub r#aliases: Option<Vec<String>>,
    #[serde(rename = "driverOpts")]
    pub r#driver_opts: Option<Vec<String>>,
    #[serde(rename = "name")]
    pub r#name: String,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecPlacement {
    #[serde(rename = "constraints")]
    pub r#constraints: Option<Vec<String>>,
    #[serde(rename = "maxReplicas")]
    pub r#max_replicas: Option<i32>,
    #[serde(rename = "platforms")]
    pub r#platforms: Option<Vec<crate::types::ServiceTaskSpecPlacementPlatform>>,
    #[serde(rename = "prefs")]
    pub r#prefs: Option<Vec<String>>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecPlacementPlatform {
    #[serde(rename = "architecture")]
    pub r#architecture: String,
    #[serde(rename = "os")]
    pub r#os: String,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecResources {
    #[serde(rename = "limits")]
    pub r#limits: Option<crate::types::ServiceTaskSpecResourcesLimits>,
    #[serde(rename = "reservation")]
    pub r#reservation: Option<crate::types::ServiceTaskSpecResourcesReservation>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecResourcesLimits {
    #[serde(rename = "memoryBytes")]
    pub r#memory_bytes: Option<i32>,
    #[serde(rename = "nanoCpus")]
    pub r#nano_cpus: Option<i32>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecResourcesReservation {
    #[serde(rename = "genericResources")]
    pub r#generic_resources: Option<crate::types::ServiceTaskSpecResourcesReservationGenericResources>,
    #[serde(rename = "memoryBytes")]
    pub r#memory_bytes: Option<i32>,
    #[serde(rename = "nanoCpus")]
    pub r#nano_cpus: Option<i32>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecResourcesReservationGenericResources {
    #[serde(rename = "discreteResourcesSpecs")]
    pub r#discrete_resources_specs: Option<Vec<String>>,
    #[serde(rename = "namedResourcesSpecs")]
    pub r#named_resources_specs: Option<Vec<String>>,
}


#[derive(serde::Serialize)]
pub struct ServiceTaskSpecRestartPolicy {
    #[serde(rename = "condition")]
    pub r#condition: Option<String>,
    #[serde(rename = "delay")]
    pub r#delay: Option<String>,
    #[serde(rename = "maxAttempts")]
    pub r#max_attempts: Option<i32>,
    #[serde(rename = "window")]
    pub r#window: Option<String>,
}


#[derive(serde::Serialize)]
pub struct ServiceUpdateConfig {
    #[serde(rename = "delay")]
    pub r#delay: Option<String>,
    #[serde(rename = "failureAction")]
    pub r#failure_action: Option<String>,
    #[serde(rename = "maxFailureRatio")]
    pub r#max_failure_ratio: Option<String>,
    #[serde(rename = "monitor")]
    pub r#monitor: Option<String>,
    #[serde(rename = "order")]
    pub r#order: Option<String>,
    #[serde(rename = "parallelism")]
    pub r#parallelism: Option<i32>,
}


#[derive(serde::Serialize)]
pub struct VolumeLabel {
    #[serde(rename = "label")]
    pub r#label: String,
    #[serde(rename = "value")]
    pub r#value: String,
}


#[derive(serde::Serialize)]
pub struct getNetworkIpamConfig {
    #[serde(rename = "auxAddress")]
    pub r#aux_address: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "gateway")]
    pub r#gateway: Option<String>,
    #[serde(rename = "ipRange")]
    pub r#ip_range: Option<String>,
    #[serde(rename = "subnet")]
    pub r#subnet: Option<String>,
}


#[derive(serde::Serialize)]
pub struct registryAuth {
    #[serde(rename = "address")]
    pub r#address: String,
    #[serde(rename = "authDisabled")]
    pub r#auth_disabled: Option<bool>,
    #[serde(rename = "configFile")]
    pub r#config_file: Option<String>,
    #[serde(rename = "configFileContent")]
    pub r#config_file_content: Option<String>,
    #[serde(rename = "password")]
    pub r#password: Option<String>,
    #[serde(rename = "username")]
    pub r#username: Option<String>,
}


pub type BuilderVersion = String;
