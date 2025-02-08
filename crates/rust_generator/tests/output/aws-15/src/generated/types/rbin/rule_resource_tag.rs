#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RuleResourceTag {
    /// The tag key.
    /// 
    /// The following argument is optional:
    #[builder(into)]
    #[serde(rename = "resourceTagKey")]
    pub r#resource_tag_key: Box<String>,
    /// The tag value.
    #[builder(into, default)]
    #[serde(rename = "resourceTagValue")]
    pub r#resource_tag_value: Box<Option<String>>,
}
