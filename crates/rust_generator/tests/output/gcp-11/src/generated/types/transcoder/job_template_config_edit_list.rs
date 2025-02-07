#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct JobTemplateConfigEditList {
    /// List of values identifying files that should be used in this atom.
    #[builder(into, default)]
    #[serde(rename = "inputs")]
    pub r#inputs: Box<Option<Vec<String>>>,
    /// A unique key for this atom.
    #[builder(into, default)]
    #[serde(rename = "key")]
    pub r#key: Box<Option<String>>,
    /// Start time in seconds for the atom, relative to the input file timeline.  The default is `0s`.
    #[builder(into, default)]
    #[serde(rename = "startTimeOffset")]
    pub r#start_time_offset: Box<Option<String>>,
}
