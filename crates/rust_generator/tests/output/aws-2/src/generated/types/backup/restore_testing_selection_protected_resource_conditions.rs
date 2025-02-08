#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RestoreTestingSelectionProtectedResourceConditions {
    /// The list of string equals conditions for resource tags. Filters the values of your tagged resources for only those resources that you tagged with the same value. Also called "exact matching.". See the structure for details
    #[builder(into, default)]
    #[serde(rename = "stringEquals")]
    pub r#string_equals: Box<Option<Vec<super::super::types::backup::RestoreTestingSelectionProtectedResourceConditionsStringEqual>>>,
    /// The list of string not equals conditions for resource tags. Filters the values of your tagged resources for only those resources that you tagged that do not have the same value. Also called "negated matching.". See the structure for details
    #[builder(into, default)]
    #[serde(rename = "stringNotEquals")]
    pub r#string_not_equals: Box<Option<Vec<super::super::types::backup::RestoreTestingSelectionProtectedResourceConditionsStringNotEqual>>>,
}
