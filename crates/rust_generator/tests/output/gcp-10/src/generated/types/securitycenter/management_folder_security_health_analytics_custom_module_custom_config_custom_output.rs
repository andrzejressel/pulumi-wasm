#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigCustomOutput {
    /// A list of custom output properties to add to the finding.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "properties")]
    pub r#properties: Box<Option<Vec<super::super::types::securitycenter::ManagementFolderSecurityHealthAnalyticsCustomModuleCustomConfigCustomOutputProperty>>>,
}
