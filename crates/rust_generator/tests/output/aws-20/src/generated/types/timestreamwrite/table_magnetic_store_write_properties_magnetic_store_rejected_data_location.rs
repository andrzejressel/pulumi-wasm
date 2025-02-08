#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct TableMagneticStoreWritePropertiesMagneticStoreRejectedDataLocation {
    /// Configuration of an S3 location to write error reports for records rejected, asynchronously, during magnetic store writes. See S3 Configuration below for more details.
    #[builder(into, default)]
    #[serde(rename = "s3Configuration")]
    pub r#s_3_configuration: Box<Option<super::super::types::timestreamwrite::TableMagneticStoreWritePropertiesMagneticStoreRejectedDataLocationS3Configuration>>,
}
