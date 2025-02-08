#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetMachineTypesMachineTypeAccelerator {
    /// Number of accelerator cards exposed to the guest.
    #[builder(into)]
    #[serde(rename = "guestAcceleratorCount")]
    pub r#guest_accelerator_count: Box<i32>,
    /// The accelerator type resource name, not a full URL, e.g. `nvidia-tesla-t4`.
    #[builder(into)]
    #[serde(rename = "guestAcceleratorType")]
    pub r#guest_accelerator_type: Box<String>,
}
