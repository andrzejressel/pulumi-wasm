#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ReservationSpecificReservationInstanceProperties {
    /// Guest accelerator type and count.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "guestAccelerators")]
    pub r#guest_accelerators: Box<Option<Vec<super::super::types::compute::ReservationSpecificReservationInstancePropertiesGuestAccelerator>>>,
    /// The amount of local ssd to reserve with each instance. This
    /// reserves disks of type `local-ssd`.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "localSsds")]
    pub r#local_ssds: Box<Option<Vec<super::super::types::compute::ReservationSpecificReservationInstancePropertiesLocalSsd>>>,
    /// The name of the machine type to reserve.
    #[builder(into)]
    #[serde(rename = "machineType")]
    pub r#machine_type: Box<String>,
    /// The minimum CPU platform for the reservation. For example,
    /// `"Intel Skylake"`. See
    /// the CPU platform availability reference](https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform#availablezones)
    /// for information on available CPU platforms.
    #[builder(into, default)]
    #[serde(rename = "minCpuPlatform")]
    pub r#min_cpu_platform: Box<Option<String>>,
}
