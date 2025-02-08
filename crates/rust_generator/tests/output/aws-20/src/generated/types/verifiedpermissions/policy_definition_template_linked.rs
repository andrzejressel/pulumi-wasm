#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PolicyDefinitionTemplateLinked {
    /// The ID of the template.
    #[builder(into)]
    #[serde(rename = "policyTemplateId")]
    pub r#policy_template_id: Box<String>,
    /// The principal of the template linked policy.
    #[builder(into, default)]
    #[serde(rename = "principal")]
    pub r#principal: Box<Option<super::super::types::verifiedpermissions::PolicyDefinitionTemplateLinkedPrincipal>>,
    /// The resource of the template linked policy.
    #[builder(into, default)]
    #[serde(rename = "resource")]
    pub r#resource: Box<Option<super::super::types::verifiedpermissions::PolicyDefinitionTemplateLinkedResource>>,
}
