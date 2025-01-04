#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BlobInventoryPolicyRule {
    /// A `filter` block as defined above.
    #[builder(into, default)]
    #[serde(rename = "filter")]
    pub r#filter: Box<Option<super::super::types::storage::BlobInventoryPolicyRuleFilter>>,
    /// The format of the inventory files. Possible values are `Csv` and `Parquet`.
    #[builder(into)]
    #[serde(rename = "format")]
    pub r#format: Box<String>,
    /// The name which should be used for this Blob Inventory Policy Rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The inventory schedule applied by this rule. Possible values are `Daily` and `Weekly`.
    #[builder(into)]
    #[serde(rename = "schedule")]
    pub r#schedule: Box<String>,
    /// A list of fields to be included in the inventory. See the [Azure API reference](https://docs.microsoft.com/rest/api/storagerp/blob-inventory-policies/create-or-update#blobinventorypolicydefinition) for all the supported fields.
    #[builder(into)]
    #[serde(rename = "schemaFields")]
    pub r#schema_fields: Box<Vec<String>>,
    /// The scope of the inventory for this rule. Possible values are `Blob` and `Container`.
    #[builder(into)]
    #[serde(rename = "scope")]
    pub r#scope: Box<String>,
    /// The storage container name to store the blob inventory files for this rule.
    #[builder(into)]
    #[serde(rename = "storageContainerName")]
    pub r#storage_container_name: Box<String>,
}
