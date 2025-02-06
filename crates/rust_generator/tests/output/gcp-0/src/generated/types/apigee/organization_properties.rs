#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct OrganizationProperties {
    /// List of all properties in the object.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "properties")]
    pub r#properties: Box<Option<Vec<super::super::types::apigee::OrganizationPropertiesProperty>>>,
}
