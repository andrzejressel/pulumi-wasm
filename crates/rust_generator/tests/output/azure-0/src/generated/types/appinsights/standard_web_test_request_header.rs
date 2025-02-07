#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StandardWebTestRequestHeader {
    /// The name which should be used for a header in the request.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The value which should be used for a header in the request.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
