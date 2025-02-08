#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FirehoseDeliveryStreamExtendedS3ConfigurationDataFormatConversionConfigurationInputFormatConfigurationDeserializerOpenXJsonSerDe {
    /// When set to true, which is the default, Kinesis Data Firehose converts JSON keys to lowercase before deserializing them.
    #[builder(into, default)]
    #[serde(rename = "caseInsensitive")]
    pub r#case_insensitive: Box<Option<bool>>,
    /// A map of column names to JSON keys that aren't identical to the column names. This is useful when the JSON contains keys that are Hive keywords. For example, timestamp is a Hive keyword. If you have a JSON key named timestamp, set this parameter to `{ ts = "timestamp" }` to map this key to a column named ts.
    #[builder(into, default)]
    #[serde(rename = "columnToJsonKeyMappings")]
    pub r#column_to_json_key_mappings: Box<Option<std::collections::HashMap<String, String>>>,
    /// When set to `true`, specifies that the names of the keys include dots and that you want Kinesis Data Firehose to replace them with underscores. This is useful because Apache Hive does not allow dots in column names. For example, if the JSON contains a key whose name is "a.b", you can define the column name to be "a_b" when using this option. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "convertDotsInJsonKeysToUnderscores")]
    pub r#convert_dots_in_json_keys_to_underscores: Box<Option<bool>>,
}
