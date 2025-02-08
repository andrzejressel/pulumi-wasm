#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClassificationJobS3JobDefinitionScopingIncludesAnd {
    /// A property-based condition that defines a property, operator, and one or more values for including or excluding an object from the job. (documented below)
    #[builder(into, default)]
    #[serde(rename = "simpleScopeTerm")]
    pub r#simple_scope_term: Box<Option<super::super::types::macie2::ClassificationJobS3JobDefinitionScopingIncludesAndSimpleScopeTerm>>,
    /// A tag-based condition that defines the operator and tag keys or tag key and value pairs for including or excluding an object from the job. (documented below)
    #[builder(into, default)]
    #[serde(rename = "tagScopeTerm")]
    pub r#tag_scope_term: Box<Option<super::super::types::macie2::ClassificationJobS3JobDefinitionScopingIncludesAndTagScopeTerm>>,
}
