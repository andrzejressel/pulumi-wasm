#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TargetMultiTarget {
    /// Required. The target_ids of this multiTarget.
    #[builder(into)]
    #[serde(rename = "targetIds")]
    pub r#target_ids: Box<Vec<String>>,
}
