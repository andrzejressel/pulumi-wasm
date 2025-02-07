#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetCatalogTableStorageDescriptorSerDeInfo {
    /// Name of the table.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Map of initialization parameters for the SerDe, in key-value form.
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<std::collections::HashMap<String, String>>,
    /// Usually the class that implements the SerDe. An example is `org.apache.hadoop.hive.serde2.columnar.ColumnarSerDe`.
    #[builder(into)]
    #[serde(rename = "serializationLibrary")]
    pub r#serialization_library: Box<String>,
}
