#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct JobLoadTimePartitioning {
    /// Number of milliseconds for which to keep the storage for a partition. A wrapper is used here because 0 is an invalid value.
    #[builder(into, default)]
    #[serde(rename = "expirationMs")]
    pub r#expiration_ms: Box<Option<String>>,
    /// If not set, the table is partitioned by pseudo column '_PARTITIONTIME'; if set, the table is partitioned by this field.
    /// The field must be a top-level TIMESTAMP or DATE field. Its mode must be NULLABLE or REQUIRED.
    /// A wrapper is used here because an empty string is an invalid value.
    #[builder(into, default)]
    #[serde(rename = "field")]
    pub r#field: Box<Option<String>>,
    /// The only type supported is DAY, which will generate one partition per day. Providing an empty string used to cause an error,
    /// but in OnePlatform the field will be treated as unset.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
