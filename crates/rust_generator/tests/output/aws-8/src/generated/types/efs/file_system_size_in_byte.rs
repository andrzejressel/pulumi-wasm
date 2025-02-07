#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FileSystemSizeInByte {
    /// The latest known metered size (in bytes) of data stored in the file system.
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<i32>>,
    /// The latest known metered size (in bytes) of data stored in the Infrequent Access storage class.
    #[builder(into, default)]
    #[serde(rename = "valueInIa")]
    pub r#value_in_ia: Box<Option<i32>>,
    /// The latest known metered size (in bytes) of data stored in the Standard storage class.
    #[builder(into, default)]
    #[serde(rename = "valueInStandard")]
    pub r#value_in_standard: Box<Option<i32>>,
}
