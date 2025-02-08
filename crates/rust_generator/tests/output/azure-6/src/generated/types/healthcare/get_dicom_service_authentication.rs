#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDicomServiceAuthentication {
    /// The intended audience to receive authentication tokens for the service. The default value is <https://dicom.azurehealthcareapis.azure.com>
    #[builder(into)]
    #[serde(rename = "audiences")]
    pub r#audiences: Box<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "authority")]
    pub r#authority: Box<String>,
}
