#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkerPoolWorkerConfig {
    /// Size of the disk attached to the worker, in GB. See [diskSizeGb](https://cloud.google.com/build/docs/private-pools/private-pool-config-file-schema#disksizegb). Specify a value of up to 1000. If `0` is specified, Cloud Build will use a standard disk size.
    #[builder(into, default)]
    #[serde(rename = "diskSizeGb")]
    pub r#disk_size_gb: Box<Option<i32>>,
    /// Machine type of a worker, such as `n1-standard-1`. See [machineType](https://cloud.google.com/build/docs/private-pools/private-pool-config-file-schema#machinetype). If left blank, Cloud Build will use `n1-standard-1`.
    #[builder(into, default)]
    #[serde(rename = "machineType")]
    pub r#machine_type: Box<Option<String>>,
    /// If true, workers are created without any public address, which prevents network egress to public IPs.
    #[builder(into, default)]
    #[serde(rename = "noExternalIp")]
    pub r#no_external_ip: Box<Option<bool>>,
}
