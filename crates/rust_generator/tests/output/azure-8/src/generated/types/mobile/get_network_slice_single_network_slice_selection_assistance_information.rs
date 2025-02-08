#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetNetworkSliceSingleNetworkSliceSelectionAssistanceInformation {
    /// Slice differentiator (SD).
    #[builder(into)]
    #[serde(rename = "sliceDifferentiator")]
    pub r#slice_differentiator: Box<String>,
    /// Slice/service type (SST).
    #[builder(into)]
    #[serde(rename = "sliceServiceType")]
    pub r#slice_service_type: Box<i32>,
}
