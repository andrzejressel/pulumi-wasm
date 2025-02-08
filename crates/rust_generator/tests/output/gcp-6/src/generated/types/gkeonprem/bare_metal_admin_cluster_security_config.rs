#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct BareMetalAdminClusterSecurityConfig {
    /// Configures user access to the Bare Metal User cluster.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "authorization")]
    pub r#authorization: Box<Option<super::super::types::gkeonprem::BareMetalAdminClusterSecurityConfigAuthorization>>,
}
