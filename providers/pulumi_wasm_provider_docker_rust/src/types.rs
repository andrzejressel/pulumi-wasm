#[derive(serde::Serialize)]
pub struct CacheFrom {
    pub r#images: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct ContainerCapabilities {
    pub r#adds: Option<Vec<String>>,
    pub r#drops: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct ContainerDevice {
    pub r#containerPath: Option<String>,
    pub r#hostPath: String,
    pub r#permissions: Option<String>,
}

#[derive(serde::Serialize)]
pub struct ContainerHealthcheck {
    pub r#interval: Option<String>,
    pub r#retries: Option<i32>,
    pub r#startPeriod: Option<String>,
    pub r#tests: Vec<String>,
    pub r#timeout: Option<String>,
}

#[derive(serde::Serialize)]
pub struct ContainerHost {
    pub r#host: String,
    pub r#ip: String,
}

#[derive(serde::Serialize)]
pub struct ContainerLabel {
    pub r#label: String,
    pub r#value: String,
}

#[derive(serde::Serialize)]
pub struct ContainerMount {
    pub r#bindOptions: Option<crate::types::ContainerMountBindOptions>,
    pub r#readOnly: Option<bool>,
    pub r#source: Option<String>,
    pub r#target: String,
    pub r#tmpfsOptions: Option<crate::types::ContainerMountTmpfsOptions>,
    pub r#type: String,
    pub r#volumeOptions: Option<crate::types::ContainerMountVolumeOptions>,
}

#[derive(serde::Serialize)]
pub struct ContainerMountBindOptions {
    pub r#propagation: Option<String>,
}

#[derive(serde::Serialize)]
pub struct ContainerMountTmpfsOptions {
    pub r#mode: Option<i32>,
    pub r#sizeBytes: Option<i32>,
}

#[derive(serde::Serialize)]
pub struct ContainerMountVolumeOptions {
    pub r#driverName: Option<String>,
    pub r#driverOptions: Option<std::collections::HashMap<String, String>>,
    pub r#labels: Option<Vec<crate::types::ContainerMountVolumeOptionsLabel>>,
    pub r#noCopy: Option<bool>,
}

#[derive(serde::Serialize)]
pub struct ContainerMountVolumeOptionsLabel {
    pub r#label: String,
    pub r#value: String,
}

#[derive(serde::Serialize)]
pub struct ContainerNetworkData {
    pub r#gateway: Option<String>,
    pub r#globalIpv6Address: Option<String>,
    pub r#globalIpv6PrefixLength: Option<i32>,
    pub r#ipAddress: Option<String>,
    pub r#ipPrefixLength: Option<i32>,
    pub r#ipv6Gateway: Option<String>,
    pub r#macAddress: Option<String>,
    pub r#networkName: Option<String>,
}

#[derive(serde::Serialize)]
pub struct ContainerNetworksAdvanced {
    pub r#aliases: Option<Vec<String>>,
    pub r#ipv4Address: Option<String>,
    pub r#ipv6Address: Option<String>,
    pub r#name: String,
}

#[derive(serde::Serialize)]
pub struct ContainerPort {
    pub r#external: Option<i32>,
    pub r#internal: i32,
    pub r#ip: Option<String>,
    pub r#protocol: Option<String>,
}

#[derive(serde::Serialize)]
pub struct ContainerUlimit {
    pub r#hard: i32,
    pub r#name: String,
    pub r#soft: i32,
}

#[derive(serde::Serialize)]
pub struct ContainerUpload {
    pub r#content: Option<String>,
    pub r#contentBase64: Option<String>,
    pub r#executable: Option<bool>,
    pub r#file: String,
    pub r#source: Option<String>,
    pub r#sourceHash: Option<String>,
}

#[derive(serde::Serialize)]
pub struct ContainerVolume {
    pub r#containerPath: Option<String>,
    pub r#fromContainer: Option<String>,
    pub r#hostPath: Option<String>,
    pub r#readOnly: Option<bool>,
    pub r#volumeName: Option<String>,
}

#[derive(serde::Serialize)]
pub struct DockerBuild {
    pub r#addHosts: Option<Vec<String>>,
    pub r#args: Option<std::collections::HashMap<String, String>>,
    pub r#builderVersion: Option<crate::types::BuilderVersion>,
    pub r#cacheFrom: Option<crate::types::CacheFrom>,
    pub r#context: Option<String>,
    pub r#dockerfile: Option<String>,
    pub r#network: Option<String>,
    pub r#platform: Option<String>,
    pub r#target: Option<String>,
}

#[derive(serde::Serialize)]
pub struct NetworkIpamConfig {
    pub r#auxAddress: Option<std::collections::HashMap<String, String>>,
    pub r#gateway: Option<String>,
    pub r#ipRange: Option<String>,
    pub r#subnet: Option<String>,
}

#[derive(serde::Serialize)]
pub struct NetworkLabel {
    pub r#label: String,
    pub r#value: String,
}

#[derive(serde::Serialize)]
pub struct PluginGrantPermission {
    pub r#name: String,
    pub r#values: Vec<String>,
}

#[derive(serde::Serialize)]
pub struct ProviderRegistryAuth {
    pub r#address: String,
    pub r#authDisabled: Option<bool>,
    pub r#configFile: Option<String>,
    pub r#configFileContent: Option<String>,
    pub r#password: Option<String>,
    pub r#username: Option<String>,
}

#[derive(serde::Serialize)]
pub struct Registry {
    pub r#password: Option<String>,
    pub r#server: Option<String>,
    pub r#username: Option<String>,
}

#[derive(serde::Serialize)]
pub struct RemoteImageBuild {
    pub r#authConfigs: Option<Vec<crate::types::RemoteImageBuildAuthConfig>>,
    pub r#buildArg: Option<std::collections::HashMap<String, String>>,
    pub r#buildArgs: Option<std::collections::HashMap<String, String>>,
    pub r#buildId: Option<String>,
    pub r#cacheFroms: Option<Vec<String>>,
    pub r#cgroupParent: Option<String>,
    pub r#context: String,
    pub r#cpuPeriod: Option<i32>,
    pub r#cpuQuota: Option<i32>,
    pub r#cpuSetCpus: Option<String>,
    pub r#cpuSetMems: Option<String>,
    pub r#cpuShares: Option<i32>,
    pub r#dockerfile: Option<String>,
    pub r#extraHosts: Option<Vec<String>>,
    pub r#forceRemove: Option<bool>,
    pub r#isolation: Option<String>,
    pub r#label: Option<std::collections::HashMap<String, String>>,
    pub r#labels: Option<std::collections::HashMap<String, String>>,
    pub r#memory: Option<i32>,
    pub r#memorySwap: Option<i32>,
    pub r#networkMode: Option<String>,
    pub r#noCache: Option<bool>,
    pub r#platform: Option<String>,
    pub r#pullParent: Option<bool>,
    pub r#remoteContext: Option<String>,
    pub r#remove: Option<bool>,
    pub r#securityOpts: Option<Vec<String>>,
    pub r#sessionId: Option<String>,
    pub r#shmSize: Option<i32>,
    pub r#squash: Option<bool>,
    pub r#suppressOutput: Option<bool>,
    pub r#tags: Option<Vec<String>>,
    pub r#target: Option<String>,
    pub r#ulimits: Option<Vec<crate::types::RemoteImageBuildUlimit>>,
    pub r#version: Option<String>,
}

#[derive(serde::Serialize)]
pub struct RemoteImageBuildAuthConfig {
    pub r#auth: Option<String>,
    pub r#email: Option<String>,
    pub r#hostName: String,
    pub r#identityToken: Option<String>,
    pub r#password: Option<String>,
    pub r#registryToken: Option<String>,
    pub r#serverAddress: Option<String>,
    pub r#userName: Option<String>,
}

#[derive(serde::Serialize)]
pub struct RemoteImageBuildUlimit {
    pub r#hard: i32,
    pub r#name: String,
    pub r#soft: i32,
}

#[derive(serde::Serialize)]
pub struct SecretLabel {
    pub r#label: String,
    pub r#value: String,
}

#[derive(serde::Serialize)]
pub struct ServiceAuth {
    pub r#password: Option<String>,
    pub r#serverAddress: String,
    pub r#username: Option<String>,
}

#[derive(serde::Serialize)]
pub struct ServiceConvergeConfig {
    pub r#delay: Option<String>,
    pub r#timeout: Option<String>,
}

#[derive(serde::Serialize)]
pub struct ServiceEndpointSpec {
    pub r#mode: Option<String>,
    pub r#ports: Option<Vec<crate::types::ServiceEndpointSpecPort>>,
}

#[derive(serde::Serialize)]
pub struct ServiceEndpointSpecPort {
    pub r#name: Option<String>,
    pub r#protocol: Option<String>,
    pub r#publishMode: Option<String>,
    pub r#publishedPort: Option<i32>,
    pub r#targetPort: i32,
}

#[derive(serde::Serialize)]
pub struct ServiceLabel {
    pub r#label: String,
    pub r#value: String,
}

#[derive(serde::Serialize)]
pub struct ServiceMode {
    pub r#global: Option<bool>,
    pub r#replicated: Option<crate::types::ServiceModeReplicated>,
}

#[derive(serde::Serialize)]
pub struct ServiceModeReplicated {
    pub r#replicas: Option<i32>,
}

#[derive(serde::Serialize)]
pub struct ServiceRollbackConfig {
    pub r#delay: Option<String>,
    pub r#failureAction: Option<String>,
    pub r#maxFailureRatio: Option<String>,
    pub r#monitor: Option<String>,
    pub r#order: Option<String>,
    pub r#parallelism: Option<i32>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpec {
    pub r#containerSpec: crate::types::ServiceTaskSpecContainerSpec,
    pub r#forceUpdate: Option<i32>,
    pub r#logDriver: Option<crate::types::ServiceTaskSpecLogDriver>,
    pub r#networksAdvanceds: Option<Vec<crate::types::ServiceTaskSpecNetworksAdvanced>>,
    pub r#placement: Option<crate::types::ServiceTaskSpecPlacement>,
    pub r#resources: Option<crate::types::ServiceTaskSpecResources>,
    pub r#restartPolicy: Option<crate::types::ServiceTaskSpecRestartPolicy>,
    pub r#runtime: Option<String>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpec {
    pub r#args: Option<Vec<String>>,
    pub r#commands: Option<Vec<String>>,
    pub r#configs: Option<Vec<crate::types::ServiceTaskSpecContainerSpecConfig>>,
    pub r#dir: Option<String>,
    pub r#dnsConfig: Option<crate::types::ServiceTaskSpecContainerSpecDnsConfig>,
    pub r#env: Option<std::collections::HashMap<String, String>>,
    pub r#groups: Option<Vec<String>>,
    pub r#healthcheck: Option<crate::types::ServiceTaskSpecContainerSpecHealthcheck>,
    pub r#hostname: Option<String>,
    pub r#hosts: Option<Vec<crate::types::ServiceTaskSpecContainerSpecHost>>,
    pub r#image: String,
    pub r#isolation: Option<String>,
    pub r#labels: Option<Vec<crate::types::ServiceTaskSpecContainerSpecLabel>>,
    pub r#mounts: Option<Vec<crate::types::ServiceTaskSpecContainerSpecMount>>,
    pub r#privileges: Option<crate::types::ServiceTaskSpecContainerSpecPrivileges>,
    pub r#readOnly: Option<bool>,
    pub r#secrets: Option<Vec<crate::types::ServiceTaskSpecContainerSpecSecret>>,
    pub r#stopGracePeriod: Option<String>,
    pub r#stopSignal: Option<String>,
    pub r#sysctl: Option<std::collections::HashMap<String, String>>,
    pub r#user: Option<String>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecConfig {
    pub r#configId: String,
    pub r#configName: Option<String>,
    pub r#fileGid: Option<String>,
    pub r#fileMode: Option<i32>,
    pub r#fileName: String,
    pub r#fileUid: Option<String>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecDnsConfig {
    pub r#nameservers: Vec<String>,
    pub r#options: Option<Vec<String>>,
    pub r#searches: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecHealthcheck {
    pub r#interval: Option<String>,
    pub r#retries: Option<i32>,
    pub r#startPeriod: Option<String>,
    pub r#tests: Vec<String>,
    pub r#timeout: Option<String>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecHost {
    pub r#host: String,
    pub r#ip: String,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecLabel {
    pub r#label: String,
    pub r#value: String,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecMount {
    pub r#bindOptions: Option<crate::types::ServiceTaskSpecContainerSpecMountBindOptions>,
    pub r#readOnly: Option<bool>,
    pub r#source: Option<String>,
    pub r#target: String,
    pub r#tmpfsOptions: Option<crate::types::ServiceTaskSpecContainerSpecMountTmpfsOptions>,
    pub r#type: String,
    pub r#volumeOptions: Option<crate::types::ServiceTaskSpecContainerSpecMountVolumeOptions>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecMountBindOptions {
    pub r#propagation: Option<String>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecMountTmpfsOptions {
    pub r#mode: Option<i32>,
    pub r#sizeBytes: Option<i32>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecMountVolumeOptions {
    pub r#driverName: Option<String>,
    pub r#driverOptions: Option<std::collections::HashMap<String, String>>,
    pub r#labels: Option<Vec<crate::types::ServiceTaskSpecContainerSpecMountVolumeOptionsLabel>>,
    pub r#noCopy: Option<bool>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecMountVolumeOptionsLabel {
    pub r#label: String,
    pub r#value: String,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecPrivileges {
    pub r#credentialSpec:
        Option<crate::types::ServiceTaskSpecContainerSpecPrivilegesCredentialSpec>,
    pub r#seLinuxContext:
        Option<crate::types::ServiceTaskSpecContainerSpecPrivilegesSeLinuxContext>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecPrivilegesCredentialSpec {
    pub r#file: Option<String>,
    pub r#registry: Option<String>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecPrivilegesSeLinuxContext {
    pub r#disable: Option<bool>,
    pub r#level: Option<String>,
    pub r#role: Option<String>,
    pub r#type: Option<String>,
    pub r#user: Option<String>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecSecret {
    pub r#fileGid: Option<String>,
    pub r#fileMode: Option<i32>,
    pub r#fileName: String,
    pub r#fileUid: Option<String>,
    pub r#secretId: String,
    pub r#secretName: Option<String>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecLogDriver {
    pub r#name: String,
    pub r#options: Option<std::collections::HashMap<String, String>>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecNetworksAdvanced {
    pub r#aliases: Option<Vec<String>>,
    pub r#driverOpts: Option<Vec<String>>,
    pub r#name: String,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecPlacement {
    pub r#constraints: Option<Vec<String>>,
    pub r#maxReplicas: Option<i32>,
    pub r#platforms: Option<Vec<crate::types::ServiceTaskSpecPlacementPlatform>>,
    pub r#prefs: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecPlacementPlatform {
    pub r#architecture: String,
    pub r#os: String,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecResources {
    pub r#limits: Option<crate::types::ServiceTaskSpecResourcesLimits>,
    pub r#reservation: Option<crate::types::ServiceTaskSpecResourcesReservation>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecResourcesLimits {
    pub r#memoryBytes: Option<i32>,
    pub r#nanoCpus: Option<i32>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecResourcesReservation {
    pub r#genericResources:
        Option<crate::types::ServiceTaskSpecResourcesReservationGenericResources>,
    pub r#memoryBytes: Option<i32>,
    pub r#nanoCpus: Option<i32>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecResourcesReservationGenericResources {
    pub r#discreteResourcesSpecs: Option<Vec<String>>,
    pub r#namedResourcesSpecs: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct ServiceTaskSpecRestartPolicy {
    pub r#condition: Option<String>,
    pub r#delay: Option<String>,
    pub r#maxAttempts: Option<i32>,
    pub r#window: Option<String>,
}

#[derive(serde::Serialize)]
pub struct ServiceUpdateConfig {
    pub r#delay: Option<String>,
    pub r#failureAction: Option<String>,
    pub r#maxFailureRatio: Option<String>,
    pub r#monitor: Option<String>,
    pub r#order: Option<String>,
    pub r#parallelism: Option<i32>,
}

#[derive(serde::Serialize)]
pub struct VolumeLabel {
    pub r#label: String,
    pub r#value: String,
}

#[derive(serde::Serialize)]
pub struct getNetworkIpamConfig {
    pub r#auxAddress: Option<std::collections::HashMap<String, String>>,
    pub r#gateway: Option<String>,
    pub r#ipRange: Option<String>,
    pub r#subnet: Option<String>,
}

#[derive(serde::Serialize)]
pub struct registryAuth {
    pub r#address: String,
    pub r#authDisabled: Option<bool>,
    pub r#configFile: Option<String>,
    pub r#configFileContent: Option<String>,
    pub r#password: Option<String>,
    pub r#username: Option<String>,
}

pub type BuilderVersion = String;
