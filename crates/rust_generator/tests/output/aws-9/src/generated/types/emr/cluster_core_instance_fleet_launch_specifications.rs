#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClusterCoreInstanceFleetLaunchSpecifications {
    /// Configuration block for on demand instances launch specifications.
    #[builder(into, default)]
    #[serde(rename = "onDemandSpecifications")]
    pub r#on_demand_specifications: Box<Option<Vec<super::super::types::emr::ClusterCoreInstanceFleetLaunchSpecificationsOnDemandSpecification>>>,
    /// Configuration block for spot instances launch specifications.
    #[builder(into, default)]
    #[serde(rename = "spotSpecifications")]
    pub r#spot_specifications: Box<Option<Vec<super::super::types::emr::ClusterCoreInstanceFleetLaunchSpecificationsSpotSpecification>>>,
}
