#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetSupportedDatabaseFlagsSupportedDatabaseFlagStringRestrictions {
    /// The list of allowed values, if bounded. This field will be empty if there is a unbounded number of allowed values.
    #[builder(into)]
    #[serde(rename = "allowedValues")]
    pub r#allowed_values: Box<Vec<String>>,
}
