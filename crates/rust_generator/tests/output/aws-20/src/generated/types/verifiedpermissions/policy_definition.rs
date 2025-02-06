#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PolicyDefinition {
    /// The static policy statement. See Static below.
    #[builder(into, default)]
    #[serde(rename = "static")]
    pub r#static_: Box<Option<super::super::types::verifiedpermissions::PolicyDefinitionStatic>>,
    /// The template linked policy. See Template Linked below.
    #[builder(into, default)]
    #[serde(rename = "templateLinked")]
    pub r#template_linked: Box<Option<super::super::types::verifiedpermissions::PolicyDefinitionTemplateLinked>>,
}
