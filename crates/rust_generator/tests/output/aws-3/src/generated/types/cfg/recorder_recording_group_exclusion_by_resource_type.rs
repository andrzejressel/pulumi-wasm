#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RecorderRecordingGroupExclusionByResourceType {
    /// A list that specifies the types of AWS resources for which AWS Config excludes records configuration changes. See [relevant part of AWS Docs](http://docs.aws.amazon.com/config/latest/APIReference/API_ResourceIdentifier.html#config-Type-ResourceIdentifier-resourceType) for available types.
    #[builder(into, default)]
    #[serde(rename = "resourceTypes")]
    pub r#resource_types: Box<Option<Vec<String>>>,
}
