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
