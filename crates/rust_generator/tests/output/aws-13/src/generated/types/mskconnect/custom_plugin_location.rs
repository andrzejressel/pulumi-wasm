#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CustomPluginLocation {
    /// Information of the plugin file stored in Amazon S3. See `s3` Block for details..
    #[builder(into)]
    #[serde(rename = "s3")]
    pub r#s_3: Box<super::super::types::mskconnect::CustomPluginLocationS3>,
}
