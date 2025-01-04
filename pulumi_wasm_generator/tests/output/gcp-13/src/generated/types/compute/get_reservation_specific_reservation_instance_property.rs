#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetReservationSpecificReservationInstanceProperty {
    /// Guest accelerator type and count.
    #[builder(into)]
    #[serde(rename = "guestAccelerators")]
    pub r#guest_accelerators: Box<Vec<super::super::types::compute::GetReservationSpecificReservationInstancePropertyGuestAccelerator>>,
    /// The amount of local ssd to reserve with each instance. This
    /// reserves disks of type 'local-ssd'.
    #[builder(into)]
    #[serde(rename = "localSsds")]
    pub r#local_ssds: Box<Vec<super::super::types::compute::GetReservationSpecificReservationInstancePropertyLocalSsd>>,
    /// The name of the machine type to reserve.
    #[builder(into)]
    #[serde(rename = "machineType")]
    pub r#machine_type: Box<String>,
    /// The minimum CPU platform for the reservation. For example,
    /// '"Intel Skylake"'. See
    /// the CPU platform availability reference](https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform#availablezones)
    /// for information on available CPU platforms.
    #[builder(into)]
    #[serde(rename = "minCpuPlatform")]
    pub r#min_cpu_platform: Box<String>,
}
