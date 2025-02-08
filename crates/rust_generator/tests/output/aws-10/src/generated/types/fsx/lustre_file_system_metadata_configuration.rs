#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LustreFileSystemMetadataConfiguration {
    /// Amount of IOPS provisioned for metadata. This parameter should only be used when the mode is set to `USER_PROVISIONED`. Valid Values are `1500`,`3000`,`6000` and `12000` through `192000` in increments of `12000`.
    #[builder(into, default)]
    #[serde(rename = "iops")]
    pub r#iops: Box<Option<i32>>,
    /// Mode for the metadata configuration of the file system. Valid values are `AUTOMATIC`, and `USER_PROVISIONED`.
    /// 
    /// !> **WARNING:** Updating the value of `iops` from a higher to a lower value will force a recreation of the resource. Any data on the file system will be lost when recreating.
    #[builder(into, default)]
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
}
