#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetOsProfile {
    /// Specifies the host OS name of the Azure Arc machine.
    #[builder(into)]
    #[serde(rename = "computerName")]
    pub r#computer_name: Box<String>,
    /// A `linux` block as defined above.
    #[builder(into)]
    #[serde(rename = "linuxes")]
    pub r#linuxes: Box<Vec<super::super::types::arcmachine::GetOsProfileLinux>>,
    /// A `windows` block as defined below.
    #[builder(into)]
    #[serde(rename = "windows")]
    pub r#windows: Box<Vec<super::super::types::arcmachine::GetOsProfileWindow>>,
}
