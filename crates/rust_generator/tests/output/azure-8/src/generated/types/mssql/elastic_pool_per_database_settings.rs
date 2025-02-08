#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ElasticPoolPerDatabaseSettings {
    /// The maximum capacity any one database can consume.
    #[builder(into)]
    #[serde(rename = "maxCapacity")]
    pub r#max_capacity: Box<f64>,
    /// The minimum capacity all databases are guaranteed.
    #[builder(into)]
    #[serde(rename = "minCapacity")]
    pub r#min_capacity: Box<f64>,
}
