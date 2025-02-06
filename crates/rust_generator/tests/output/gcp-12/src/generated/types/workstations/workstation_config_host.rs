#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkstationConfigHost {
    /// A runtime using a Compute Engine instance.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "gceInstance")]
    pub r#gce_instance: Box<Option<super::super::types::workstations::WorkstationConfigHostGceInstance>>,
}
