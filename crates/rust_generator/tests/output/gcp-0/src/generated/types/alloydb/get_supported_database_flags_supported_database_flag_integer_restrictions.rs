#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetSupportedDatabaseFlagsSupportedDatabaseFlagIntegerRestrictions {
    /// The maximum value that can be specified, if applicable.
    #[builder(into)]
    #[serde(rename = "maxValue")]
    pub r#max_value: Box<String>,
    /// The minimum value that can be specified, if applicable.
    #[builder(into)]
    #[serde(rename = "minValue")]
    pub r#min_value: Box<String>,
}
