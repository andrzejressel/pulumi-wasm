#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ServiceOrderedPlacementStrategy {
    /// For the `spread` placement strategy, valid values are `instanceId` (or `host`,
    /// which has the same effect), or any platform or custom attribute that is applied to a container instance.
    /// For the `binpack` type, valid values are `memory` and `cpu`. For the `random` type, this attribute is not
    /// needed. For more information, see [Placement Strategy](https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_PlacementStrategy.html).
    /// 
    /// > **Note:** for `spread`, `host` and `instanceId` will be normalized, by AWS, to be `instanceId`. This means the statefile will show `instanceId` but your config will differ if you use `host`.
    #[builder(into, default)]
    #[serde(rename = "field")]
    pub r#field: Box<Option<String>>,
    /// Type of placement strategy. Must be one of: `binpack`, `random`, or `spread`
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
