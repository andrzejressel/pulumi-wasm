#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BareMetalClusterLoadBalancerBgpLbConfigLoadBalancerNodePoolConfigNodePoolConfigKubeletConfig {
    /// The maximum size of bursty pulls, temporarily allows pulls to burst to this
    /// number, while still not exceeding registry_pull_qps.
    /// The value must not be a negative number.
    /// Updating this field may impact scalability by changing the amount of
    /// traffic produced by image pulls.
    /// Defaults to 10.
    #[builder(into, default)]
    #[serde(rename = "registryBurst")]
    pub r#registry_burst: Box<Option<i32>>,
    /// The limit of registry pulls per second.
    /// Setting this value to 0 means no limit.
    /// Updating this field may impact scalability by changing the amount of
    /// traffic produced by image pulls.
    /// Defaults to 5.
    #[builder(into, default)]
    #[serde(rename = "registryPullQps")]
    pub r#registry_pull_qps: Box<Option<i32>>,
    /// Prevents the Kubelet from pulling multiple images at a time.
    /// We recommend *not* changing the default value on nodes that run docker
    /// daemon with version  < 1.9 or an Another Union File System (Aufs) storage
    /// backend. Issue https://github.com/kubernetes/kubernetes/issues/10959 has
    /// more details.
    #[builder(into, default)]
    #[serde(rename = "serializeImagePullsDisabled")]
    pub r#serialize_image_pulls_disabled: Box<Option<bool>>,
}
