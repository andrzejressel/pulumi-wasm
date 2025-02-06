#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectorCapacity {
    /// Information about the auto scaling parameters for the connector. See `autoscaling` Block for details.
    #[builder(into, default)]
    #[serde(rename = "autoscaling")]
    pub r#autoscaling: Box<Option<super::super::types::mskconnect::ConnectorCapacityAutoscaling>>,
    /// Details about a fixed capacity allocated to a connector. See `provisioned_capacity` Block for details.
    #[builder(into, default)]
    #[serde(rename = "provisionedCapacity")]
    pub r#provisioned_capacity: Box<Option<super::super::types::mskconnect::ConnectorCapacityProvisionedCapacity>>,
}
