#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetPolicySetDefinitionPolicyDefinitionReference {
    /// The parameter values for the referenced policy rule. This field is a JSON object.
    #[builder(into)]
    #[serde(rename = "parameterValues")]
    pub r#parameter_values: Box<String>,
    /// The mapping of the parameter values for the referenced policy rule. The keys are the parameter names.
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<std::collections::HashMap<String, String>>,
    /// The ID of the policy definition or policy set definition that is included in this policy set definition.
    #[builder(into)]
    #[serde(rename = "policyDefinitionId")]
    pub r#policy_definition_id: Box<String>,
    /// The list of names of the policy definition groups that this policy definition reference belongs to.
    #[builder(into)]
    #[serde(rename = "policyGroupNames")]
    pub r#policy_group_names: Box<Vec<String>>,
    /// The unique ID within this policy set definition for this policy definition reference.
    #[builder(into)]
    #[serde(rename = "referenceId")]
    pub r#reference_id: Box<String>,
}
