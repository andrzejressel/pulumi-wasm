#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetNetworkSimPolicySlice {
    /// An array of `data_network` block as defined below.
    #[builder(into)]
    #[serde(rename = "dataNetworks")]
    pub r#data_networks: Box<Vec<super::super::types::mobile::GetNetworkSimPolicySliceDataNetwork>>,
    /// The ID of default data network to use if the UE does not explicitly specify it.
    #[builder(into)]
    #[serde(rename = "defaultDataNetworkId")]
    pub r#default_data_network_id: Box<String>,
    /// The ID of the slice that these settings apply to.
    #[builder(into)]
    #[serde(rename = "sliceId")]
    pub r#slice_id: Box<String>,
}
