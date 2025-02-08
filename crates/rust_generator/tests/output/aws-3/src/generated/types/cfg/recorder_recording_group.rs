#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RecorderRecordingGroup {
    /// Specifies whether AWS Config records configuration changes for every supported type of regional resource (which includes any new type that will become supported in the future). Conflicts with `resource_types`. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "allSupported")]
    pub r#all_supported: Box<Option<bool>>,
    /// An object that specifies how AWS Config excludes resource types from being recorded by the configuration recorder.To use this option, you must set the useOnly field of RecordingStrategy to `EXCLUSION_BY_RESOURCE_TYPES` Requires `all_supported = false`. Conflicts with `resource_types`.
    #[builder(into, default)]
    #[serde(rename = "exclusionByResourceTypes")]
    pub r#exclusion_by_resource_types: Box<Option<Vec<super::super::types::cfg::RecorderRecordingGroupExclusionByResourceType>>>,
    /// Specifies whether AWS Config includes all supported types of _global resources_ with the resources that it records. Requires `all_supported = true`. Conflicts with `resource_types`.
    #[builder(into, default)]
    #[serde(rename = "includeGlobalResourceTypes")]
    pub r#include_global_resource_types: Box<Option<bool>>,
    /// Recording Strategy. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "recordingStrategies")]
    pub r#recording_strategies: Box<Option<Vec<super::super::types::cfg::RecorderRecordingGroupRecordingStrategy>>>,
    /// A list that specifies the types of AWS resources for which AWS Config records configuration changes (for example, `AWS::EC2::Instance` or `AWS::CloudTrail::Trail`). See [relevant part of AWS Docs](http://docs.aws.amazon.com/config/latest/APIReference/API_ResourceIdentifier.html#config-Type-ResourceIdentifier-resourceType) for available types. In order to use this attribute, `all_supported` must be set to false.
    #[builder(into, default)]
    #[serde(rename = "resourceTypes")]
    pub r#resource_types: Box<Option<Vec<String>>>,
}
