#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct OntapVolumeAggregateConfiguration {
    /// Used to specify the names of the aggregates on which the volume will be created. Each aggregate needs to be in the format aggrX where X is the number of the aggregate.
    #[builder(into, default)]
    #[serde(rename = "aggregates")]
    pub r#aggregates: Box<Option<Vec<String>>>,
    /// Used to explicitly set the number of constituents within the FlexGroup per storage aggregate. the default value is `8`.
    #[builder(into, default)]
    #[serde(rename = "constituentsPerAggregate")]
    pub r#constituents_per_aggregate: Box<Option<i32>>,
    /// The total amount of constituents for a `FLEXGROUP` volume. This would equal constituents_per_aggregate x aggregates.
    #[builder(into, default)]
    #[serde(rename = "totalConstituents")]
    pub r#total_constituents: Box<Option<i32>>,
}
