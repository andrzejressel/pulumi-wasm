#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigCustomOutputProperty {
    /// Name of the property for the custom output.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// The CEL expression for the custom output. A resource property can be specified
    /// to return the value of the property or a text string enclosed in quotation marks.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "valueExpression")]
    pub r#value_expression: Box<Option<super::super::types::securitycenter::ManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigCustomOutputPropertyValueExpression>>,
}
