#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct InstanceAccessControlAttributesAttribute {
    /// The name of the attribute associated with your identities in your identity source. This is used to map a specified attribute in your identity source with an attribute in AWS SSO.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// The value used for mapping a specified attribute to an identity source. See AccessControlAttributeValue
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Box<Vec<super::super::types::ssoadmin::InstanceAccessControlAttributesAttributeValue>>,
}
