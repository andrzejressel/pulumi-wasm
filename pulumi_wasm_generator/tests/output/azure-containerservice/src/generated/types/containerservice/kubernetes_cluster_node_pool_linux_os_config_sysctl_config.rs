#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KubernetesClusterNodePoolLinuxOsConfigSysctlConfig {
    /// The sysctl setting fs.aio-max-nr. Must be between `65536` and `6553500`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "fsAioMaxNr")]
    pub r#fs_aio_max_nr: Box<Option<i32>>,
    /// The sysctl setting fs.file-max. Must be between `8192` and `12000500`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "fsFileMax")]
    pub r#fs_file_max: Box<Option<i32>>,
    /// The sysctl setting fs.inotify.max_user_watches. Must be between `781250` and `2097152`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "fsInotifyMaxUserWatches")]
    pub r#fs_inotify_max_user_watches: Box<Option<i32>>,
    /// The sysctl setting fs.nr_open. Must be between `8192` and `20000500`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "fsNrOpen")]
    pub r#fs_nr_open: Box<Option<i32>>,
    /// The sysctl setting kernel.threads-max. Must be between `20` and `513785`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "kernelThreadsMax")]
    pub r#kernel_threads_max: Box<Option<i32>>,
    /// The sysctl setting net.core.netdev_max_backlog. Must be between `1000` and `3240000`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "netCoreNetdevMaxBacklog")]
    pub r#net_core_netdev_max_backlog: Box<Option<i32>>,
    /// The sysctl setting net.core.optmem_max. Must be between `20480` and `4194304`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "netCoreOptmemMax")]
    pub r#net_core_optmem_max: Box<Option<i32>>,
    /// The sysctl setting net.core.rmem_default. Must be between `212992` and `134217728`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "netCoreRmemDefault")]
    pub r#net_core_rmem_default: Box<Option<i32>>,
    /// The sysctl setting net.core.rmem_max. Must be between `212992` and `134217728`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "netCoreRmemMax")]
    pub r#net_core_rmem_max: Box<Option<i32>>,
    /// The sysctl setting net.core.somaxconn. Must be between `4096` and `3240000`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "netCoreSomaxconn")]
    pub r#net_core_somaxconn: Box<Option<i32>>,
    /// The sysctl setting net.core.wmem_default. Must be between `212992` and `134217728`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "netCoreWmemDefault")]
    pub r#net_core_wmem_default: Box<Option<i32>>,
    /// The sysctl setting net.core.wmem_max. Must be between `212992` and `134217728`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "netCoreWmemMax")]
    pub r#net_core_wmem_max: Box<Option<i32>>,
    /// The sysctl setting net.ipv4.ip_local_port_range max value. Must be between `32768` and `65535`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "netIpv4IpLocalPortRangeMax")]
    pub r#net_ipv_4_ip_local_port_range_max: Box<Option<i32>>,
    /// The sysctl setting net.ipv4.ip_local_port_range min value. Must be between `1024` and `60999`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "netIpv4IpLocalPortRangeMin")]
    pub r#net_ipv_4_ip_local_port_range_min: Box<Option<i32>>,
    /// The sysctl setting net.ipv4.neigh.default.gc_thresh1. Must be between `128` and `80000`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "netIpv4NeighDefaultGcThresh1")]
    pub r#net_ipv_4_neigh_default_gc_thresh_1: Box<Option<i32>>,
    /// The sysctl setting net.ipv4.neigh.default.gc_thresh2. Must be between `512` and `90000`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "netIpv4NeighDefaultGcThresh2")]
    pub r#net_ipv_4_neigh_default_gc_thresh_2: Box<Option<i32>>,
    /// The sysctl setting net.ipv4.neigh.default.gc_thresh3. Must be between `1024` and `100000`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "netIpv4NeighDefaultGcThresh3")]
    pub r#net_ipv_4_neigh_default_gc_thresh_3: Box<Option<i32>>,
    /// The sysctl setting net.ipv4.tcp_fin_timeout. Must be between `5` and `120`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "netIpv4TcpFinTimeout")]
    pub r#net_ipv_4_tcp_fin_timeout: Box<Option<i32>>,
    /// The sysctl setting net.ipv4.tcp_keepalive_intvl. Must be between `10` and `90`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "netIpv4TcpKeepaliveIntvl")]
    pub r#net_ipv_4_tcp_keepalive_intvl: Box<Option<i32>>,
    /// The sysctl setting net.ipv4.tcp_keepalive_probes. Must be between `1` and `15`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "netIpv4TcpKeepaliveProbes")]
    pub r#net_ipv_4_tcp_keepalive_probes: Box<Option<i32>>,
    /// The sysctl setting net.ipv4.tcp_keepalive_time. Must be between `30` and `432000`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "netIpv4TcpKeepaliveTime")]
    pub r#net_ipv_4_tcp_keepalive_time: Box<Option<i32>>,
    /// The sysctl setting net.ipv4.tcp_max_syn_backlog. Must be between `128` and `3240000`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "netIpv4TcpMaxSynBacklog")]
    pub r#net_ipv_4_tcp_max_syn_backlog: Box<Option<i32>>,
    /// The sysctl setting net.ipv4.tcp_max_tw_buckets. Must be between `8000` and `1440000`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "netIpv4TcpMaxTwBuckets")]
    pub r#net_ipv_4_tcp_max_tw_buckets: Box<Option<i32>>,
    /// Is sysctl setting net.ipv4.tcp_tw_reuse enabled? Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "netIpv4TcpTwReuse")]
    pub r#net_ipv_4_tcp_tw_reuse: Box<Option<bool>>,
    /// The sysctl setting net.netfilter.nf_conntrack_buckets. Must be between `65536` and `524288`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "netNetfilterNfConntrackBuckets")]
    pub r#net_netfilter_nf_conntrack_buckets: Box<Option<i32>>,
    /// The sysctl setting net.netfilter.nf_conntrack_max. Must be between `131072` and `2097152`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "netNetfilterNfConntrackMax")]
    pub r#net_netfilter_nf_conntrack_max: Box<Option<i32>>,
    /// The sysctl setting vm.max_map_count. Must be between `65530` and `262144`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "vmMaxMapCount")]
    pub r#vm_max_map_count: Box<Option<i32>>,
    /// The sysctl setting vm.swappiness. Must be between `0` and `100`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "vmSwappiness")]
    pub r#vm_swappiness: Box<Option<i32>>,
    /// The sysctl setting vm.vfs_cache_pressure. Must be between `0` and `100`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "vmVfsCachePressure")]
    pub r#vm_vfs_cache_pressure: Box<Option<i32>>,
}
