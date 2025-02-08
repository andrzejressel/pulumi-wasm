#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct InstanceAccessControlAttributesAttributeValue {
    /// The identity source to use when mapping a specified attribute to AWS SSO.
    #[builder(into)]
    #[serde(rename = "sources")]
    pub r#sources: Box<Vec<String>>,
}
