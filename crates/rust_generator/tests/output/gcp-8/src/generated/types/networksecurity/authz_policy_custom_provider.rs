#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AuthzPolicyCustomProvider {
    /// Delegate authorization decision to user authored Service Extension. Only one of cloudIap or authzExtension can be specified.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "authzExtension")]
    pub r#authz_extension: Box<Option<super::super::types::networksecurity::AuthzPolicyCustomProviderAuthzExtension>>,
    /// Delegates authorization decisions to Cloud IAP. Applicable only for managed load balancers. Enabling Cloud IAP at the AuthzPolicy level is not compatible with Cloud IAP settings in the BackendService. Enabling IAP in both places will result in request failure. Ensure that IAP is enabled in either the AuthzPolicy or the BackendService but not in both places.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "cloudIap")]
    pub r#cloud_iap: Box<Option<super::super::types::networksecurity::AuthzPolicyCustomProviderCloudIap>>,
}
