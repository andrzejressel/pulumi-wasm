#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetTableMagneticStoreWritePropertyMagneticStoreRejectedDataLocation {
    /// Object containing the following attributes to describe the configuration of an s3 location to write error reports for records rejected.
    #[builder(into)]
    #[serde(rename = "s3Configurations")]
    pub r#s_3_configurations: Box<Vec<super::super::types::timestreamwrite::GetTableMagneticStoreWritePropertyMagneticStoreRejectedDataLocationS3Configuration>>,
}
