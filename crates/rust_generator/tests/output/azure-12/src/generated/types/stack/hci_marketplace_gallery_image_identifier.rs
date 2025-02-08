#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct HciMarketplaceGalleryImageIdentifier {
    /// The offer of the Azure Stack HCI Marketplace Gallery Image. Changing this forces a new Azure Stack HCI Marketplace Gallery Image to be created.
    #[builder(into)]
    #[serde(rename = "offer")]
    pub r#offer: Box<String>,
    /// The publisher of the Azure Stack HCI Marketplace Gallery Image. Changing this forces a new Azure Stack HCI Marketplace Gallery Image to be created.
    #[builder(into)]
    #[serde(rename = "publisher")]
    pub r#publisher: Box<String>,
    /// The sku of the Azure Stack HCI Marketplace Gallery Image. Changing this forces a new Azure Stack HCI Marketplace Gallery Image to be created.
    #[builder(into)]
    #[serde(rename = "sku")]
    pub r#sku: Box<String>,
}
