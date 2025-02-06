#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CxPageFormParameter {
    /// Hierarchical advanced settings for this parameter. The settings exposed at the lower level overrides the settings exposed at the higher level.
    /// Hierarchy: Agent->Flow->Page->Fulfillment/Parameter.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "advancedSettings")]
    pub r#advanced_settings: Box<Option<super::super::types::diagflow::CxPageFormParameterAdvancedSettings>>,
    /// The default value of an optional parameter. If the parameter is required, the default value will be ignored.
    #[builder(into, default)]
    #[serde(rename = "defaultValue")]
    pub r#default_value: Box<Option<String>>,
    /// The human-readable name of the parameter, unique within the form.
    #[builder(into, default)]
    #[serde(rename = "displayName")]
    pub r#display_name: Box<Option<String>>,
    /// The entity type of the parameter.
    /// Format: projects/-/locations/-/agents/-/entityTypes/<System Entity Type ID> for system entity types (for example, projects/-/locations/-/agents/-/entityTypes/sys.date), or projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/entityTypes/<Entity Type ID> for developer entity types.
    #[builder(into, default)]
    #[serde(rename = "entityType")]
    pub r#entity_type: Box<Option<String>>,
    /// Defines fill behavior for the parameter.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "fillBehavior")]
    pub r#fill_behavior: Box<Option<super::super::types::diagflow::CxPageFormParameterFillBehavior>>,
    /// Indicates whether the parameter represents a list of values.
    #[builder(into, default)]
    #[serde(rename = "isList")]
    pub r#is_list: Box<Option<bool>>,
    /// Indicates whether the parameter content should be redacted in log.
    /// If redaction is enabled, the parameter content will be replaced by parameter name during logging. Note: the parameter content is subject to redaction if either parameter level redaction or entity type level redaction is enabled.
    #[builder(into, default)]
    #[serde(rename = "redact")]
    pub r#redact: Box<Option<bool>>,
    /// Indicates whether the parameter is required. Optional parameters will not trigger prompts; however, they are filled if the user specifies them.
    /// Required parameters must be filled before form filling concludes.
    #[builder(into, default)]
    #[serde(rename = "required")]
    pub r#required: Box<Option<bool>>,
}
