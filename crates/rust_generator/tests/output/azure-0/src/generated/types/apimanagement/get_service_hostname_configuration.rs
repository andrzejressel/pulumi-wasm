#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetServiceHostnameConfiguration {
    /// One or more `developer_portal` blocks as documented below.
    #[builder(into)]
    #[serde(rename = "developerPortals")]
    pub r#developer_portals: Box<Vec<super::super::types::apimanagement::GetServiceHostnameConfigurationDeveloperPortal>>,
    /// One or more `management` blocks as documented below.
    #[builder(into)]
    #[serde(rename = "managements")]
    pub r#managements: Box<Vec<super::super::types::apimanagement::GetServiceHostnameConfigurationManagement>>,
    /// One or more `portal` blocks as documented below.
    #[builder(into)]
    #[serde(rename = "portals")]
    pub r#portals: Box<Vec<super::super::types::apimanagement::GetServiceHostnameConfigurationPortal>>,
    /// One or more `proxy` blocks as documented below.
    #[builder(into)]
    #[serde(rename = "proxies")]
    pub r#proxies: Box<Vec<super::super::types::apimanagement::GetServiceHostnameConfigurationProxy>>,
    /// One or more `scm` blocks as documented below.
    #[builder(into)]
    #[serde(rename = "scms")]
    pub r#scms: Box<Vec<super::super::types::apimanagement::GetServiceHostnameConfigurationScm>>,
}
