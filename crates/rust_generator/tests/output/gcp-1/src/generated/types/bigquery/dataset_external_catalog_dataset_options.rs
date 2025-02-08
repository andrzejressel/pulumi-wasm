#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DatasetExternalCatalogDatasetOptions {
    /// The storage location URI for all tables in the dataset. Equivalent to hive metastore's
    /// database locationUri. Maximum length of 1024 characters.
    #[builder(into, default)]
    #[serde(rename = "defaultStorageLocationUri")]
    pub r#default_storage_location_uri: Box<Option<String>>,
    /// A map of key value pairs defining the parameters and properties of the open source schema.
    /// Maximum size of 2Mib.
    #[builder(into, default)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Option<std::collections::HashMap<String, String>>>,
}
