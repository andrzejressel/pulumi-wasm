#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DatascanDataProfileSpecExcludeFields {
    /// Expected input is a list of fully qualified names of fields as in the schema.
    /// Only top-level field names for nested fields are supported.
    /// For instance, if 'x' is of nested field type, listing 'x' is supported but 'x.y.z' is not supported. Here 'y' and 'y.z' are nested fields of 'x'.
    #[builder(into, default)]
    #[serde(rename = "fieldNames")]
    pub r#field_names: Box<Option<Vec<String>>>,
}
