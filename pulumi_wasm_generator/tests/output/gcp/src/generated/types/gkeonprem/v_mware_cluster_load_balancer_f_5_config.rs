#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VMwareClusterLoadBalancerF5Config {
    /// The load balancer's IP address.
    #[builder(into, default)]
    #[serde(rename = "address")]
    pub r#address: Box<Option<String>>,
    /// he preexisting partition to be used by the load balancer. T
    /// his partition is usually created for the admin cluster for example:
    /// 'my-f5-admin-partition'.
    #[builder(into, default)]
    #[serde(rename = "partition")]
    pub r#partition: Box<Option<String>>,
    /// The pool name. Only necessary, if using SNAT.
    #[builder(into, default)]
    #[serde(rename = "snatPool")]
    pub r#snat_pool: Box<Option<String>>,
}
