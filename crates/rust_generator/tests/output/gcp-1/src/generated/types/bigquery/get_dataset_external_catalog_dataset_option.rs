#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDatasetExternalCatalogDatasetOption {
    /// The storage location URI for all tables in the dataset. Equivalent to hive metastore's
    /// database locationUri. Maximum length of 1024 characters.
    #[builder(into)]
    #[serde(rename = "defaultStorageLocationUri")]
    pub r#default_storage_location_uri: Box<String>,
    /// A map of key value pairs defining the parameters and properties of the open source schema.
    /// Maximum size of 2Mib.
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<std::collections::HashMap<String, String>>,
}
