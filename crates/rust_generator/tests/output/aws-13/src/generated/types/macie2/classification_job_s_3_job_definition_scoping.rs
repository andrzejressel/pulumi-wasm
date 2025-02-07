#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClassificationJobS3JobDefinitionScoping {
    /// The property- or tag-based conditions that determine which objects to exclude from the analysis. (documented below)
    #[builder(into, default)]
    #[serde(rename = "excludes")]
    pub r#excludes: Box<Option<super::super::types::macie2::ClassificationJobS3JobDefinitionScopingExcludes>>,
    /// The property- or tag-based conditions that determine which objects to include in the analysis. (documented below)
    #[builder(into, default)]
    #[serde(rename = "includes")]
    pub r#includes: Box<Option<super::super::types::macie2::ClassificationJobS3JobDefinitionScopingIncludes>>,
}
