#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FleetFleetInstanceSet {
    /// The IDs of the instances.
    #[builder(into, default)]
    #[serde(rename = "instanceIds")]
    pub r#instance_ids: Box<Option<Vec<String>>>,
    /// The instance type.
    #[builder(into, default)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: Box<Option<String>>,
    /// Indicates if the instance that was launched is a Spot Instance or On-Demand Instance.
    #[builder(into, default)]
    #[serde(rename = "lifecycle")]
    pub r#lifecycle: Box<Option<String>>,
    /// The value is `Windows` for Windows instances. Otherwise, the value is blank.
    #[builder(into, default)]
    #[serde(rename = "platform")]
    pub r#platform: Box<Option<String>>,
}
