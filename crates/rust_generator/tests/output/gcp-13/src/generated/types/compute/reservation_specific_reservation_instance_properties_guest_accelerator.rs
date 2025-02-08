#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ReservationSpecificReservationInstancePropertiesGuestAccelerator {
    /// The number of the guest accelerator cards exposed to
    /// this instance.
    #[builder(into)]
    #[serde(rename = "acceleratorCount")]
    pub r#accelerator_count: Box<i32>,
    /// The full or partial URL of the accelerator type to
    /// attach to this instance. For example:
    /// `projects/my-project/zones/us-central1-c/acceleratorTypes/nvidia-tesla-p100`
    /// If you are creating an instance template, specify only the accelerator name.
    #[builder(into)]
    #[serde(rename = "acceleratorType")]
    pub r#accelerator_type: Box<String>,
}
