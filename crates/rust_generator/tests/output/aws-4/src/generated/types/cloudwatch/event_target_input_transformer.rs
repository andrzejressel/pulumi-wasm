#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EventTargetInputTransformer {
    /// Key value pairs specified in the form of JSONPath (for example, time = $.time)
    /// * You can have as many as 100 key-value pairs.
    /// * You must use JSON dot notation, not bracket notation.
    /// * The keys can't start with "AWS".
    #[builder(into, default)]
    #[serde(rename = "inputPaths")]
    pub r#input_paths: Box<Option<std::collections::HashMap<String, String>>>,
    /// Template to customize data sent to the target. Must be valid JSON. To send a string value, the string value must include double quotes.
    #[builder(into)]
    #[serde(rename = "inputTemplate")]
    pub r#input_template: Box<String>,
}
