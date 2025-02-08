#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DomainMappingStatus {
    /// (Output)
    /// Array of observed DomainMappingConditions, indicating the current state
    /// of the DomainMapping.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "conditions")]
    pub r#conditions: Box<Option<Vec<super::super::types::cloudrun::DomainMappingStatusCondition>>>,
    /// (Output)
    /// The name of the route that the mapping currently points to.
    #[builder(into, default)]
    #[serde(rename = "mappedRouteName")]
    pub r#mapped_route_name: Box<Option<String>>,
    /// (Output)
    /// ObservedGeneration is the 'Generation' of the DomainMapping that
    /// was last processed by the controller.
    #[builder(into, default)]
    #[serde(rename = "observedGeneration")]
    pub r#observed_generation: Box<Option<i32>>,
    /// The resource records required to configure this domain mapping. These
    /// records must be added to the domain's DNS configuration in order to
    /// serve the application via this domain mapping.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "resourceRecords")]
    pub r#resource_records: Box<Option<Vec<super::super::types::cloudrun::DomainMappingStatusResourceRecord>>>,
}
