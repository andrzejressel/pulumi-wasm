#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FindingsFilterFindingCriteria {
    /// A condition that specifies the property, operator, and one or more values to use to filter the results.  (documented below)
    #[builder(into, default)]
    #[serde(rename = "criterions")]
    pub r#criterions: Box<Option<Vec<super::super::types::macie::FindingsFilterFindingCriteriaCriterion>>>,
}
