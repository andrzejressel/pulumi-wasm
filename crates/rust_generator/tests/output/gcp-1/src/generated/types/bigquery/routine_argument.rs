#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RoutineArgument {
    /// Defaults to FIXED_TYPE.
    /// Default value is `FIXED_TYPE`.
    /// Possible values are: `FIXED_TYPE`, `ANY_TYPE`.
    #[builder(into, default)]
    #[serde(rename = "argumentKind")]
    pub r#argument_kind: Box<Option<String>>,
    /// A JSON schema for the data type. Required unless argumentKind = ANY_TYPE.
    /// ~>**NOTE**: Because this field expects a JSON string, any changes to the string
    /// will create a diff, even if the JSON itself hasn't changed. If the API returns
    /// a different value for the same schema, e.g. it switched the order of values
    /// or replaced STRUCT field type with RECORD field type, we currently cannot
    /// suppress the recurring diff this causes. As a workaround, we recommend using
    /// the schema as returned by the API.
    #[builder(into, default)]
    #[serde(rename = "dataType")]
    pub r#data_type: Box<Option<String>>,
    /// Specifies whether the argument is input or output. Can be set for procedures only.
    /// Possible values are: `IN`, `OUT`, `INOUT`.
    #[builder(into, default)]
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
    /// The name of this argument. Can be absent for function return argument.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}
