#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionEnumTypeOptions {
    /// List of possible enum values.
    #[builder(into)]
    #[serde(rename = "possibleValues")]
    pub r#possible_values: Box<Vec<String>>,
    /// Make sure the enum property value provided in the document is in the possile value list during document creation. The validation check runs by default.
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "validationCheckDisabled")]
    pub r#validation_check_disabled: Box<Option<bool>>,
}
