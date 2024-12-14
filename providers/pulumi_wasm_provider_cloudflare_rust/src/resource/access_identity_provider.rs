//! Provides a Cloudflare Access Identity Provider resource. Identity
//! Providers are used as an authentication or authorisation source
//! within Access.
//! 
//! > It's required that an `account_id` or `zone_id` is provided and in
//!    most cases using either is fine. However, if you're using a scoped
//!    access token, you must provide the argument that matches the token's
//!    scope. For example, an access token that is scoped to the "example.com"
//!    zone needs to use the `zone_id` argument.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let githubOauth = access_identity_provider::create(
//!         "githubOauth",
//!         AccessIdentityProviderArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .configs(
//!                 vec![
//!                     AccessIdentityProviderConfig::builder().clientId("example")
//!                     .clientSecret("secret_key").build_struct(),
//!                 ],
//!             )
//!             .name("GitHub OAuth")
//!             .type_("github")
//!             .build_struct(),
//!     );
//!     let jumpcloudSaml = access_identity_provider::create(
//!         "jumpcloudSaml",
//!         AccessIdentityProviderArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .configs(
//!                 vec![
//!                     AccessIdentityProviderConfig::builder().attributes(vec!["email",
//!                     "username",])
//!                     .idpPublicCert("MIIDpDCCAoygAwIBAgIGAV2ka+55MA0GCSqGSIb3DQEBCwUAMIGSMQswCQ...GF/Q2/MHadws97cZg\nuTnQyuOqPuHbnN83d/2l1NSYKCbHt24o")
//!                     .issuerUrl("jumpcloud").signRequest(false)
//!                     .ssoTargetUrl("https://sso.myexample.jumpcloud.com/saml2/cloudflareaccess")
//!                     .build_struct(),
//!                 ],
//!             )
//!             .name("JumpCloud SAML")
//!             .type_("saml")
//!             .build_struct(),
//!     );
//!     let okta = access_identity_provider::create(
//!         "okta",
//!         AccessIdentityProviderArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .configs(
//!                 vec![
//!                     AccessIdentityProviderConfig::builder().apiToken("okta_api_token")
//!                     .clientId("example").clientSecret("secret_key")
//!                     .oktaAccount("https://example.com").build_struct(),
//!                 ],
//!             )
//!             .name("Okta")
//!             .type_("okta")
//!             .build_struct(),
//!     );
//!     let pinLogin = access_identity_provider::create(
//!         "pinLogin",
//!         AccessIdentityProviderArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .name("PIN login")
//!             .type_("onetimepin")
//!             .build_struct(),
//!     );
//! }
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/accessIdentityProvider:AccessIdentityProvider example <account_id>/<identity_provider_id>
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct AccessIdentityProviderArgs {
    /// The account identifier to target for the resource. Conflicts with `zone_id`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Provider configuration from the [developer documentation](https://developers.cloudflare.com/access/configuring-identity-providers/).
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub configs: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessIdentityProviderConfig>>>,
    /// Friendly name of the Access Identity Provider configuration.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// Configuration for SCIM settings for a given IDP.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub scim_configs: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessIdentityProviderScimConfig>>>,
    /// The provider type to use. Available values: `azureAD`, `centrify`, `facebook`, `github`, `google`, `google-apps`, `linkedin`, `oidc`, `okta`, `onelogin`, `onetimepin`, `pingone`, `saml`, `yandex`.
    #[builder(into)]
    pub type_: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct AccessIdentityProviderResult {
    /// The account identifier to target for the resource. Conflicts with `zone_id`. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Provider configuration from the [developer documentation](https://developers.cloudflare.com/access/configuring-identity-providers/).
    pub configs: pulumi_wasm_rust::Output<Vec<crate::types::AccessIdentityProviderConfig>>,
    /// Friendly name of the Access Identity Provider configuration.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Configuration for SCIM settings for a given IDP.
    pub scim_configs: pulumi_wasm_rust::Output<Vec<crate::types::AccessIdentityProviderScimConfig>>,
    /// The provider type to use. Available values: `azureAD`, `centrify`, `facebook`, `github`, `google`, `google-apps`, `linkedin`, `oidc`, `okta`, `onelogin`, `onetimepin`, `pingone`, `saml`, `yandex`.
    pub type_: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: AccessIdentityProviderArgs) -> AccessIdentityProviderResult {

    let result = crate::bindings::pulumi::cloudflare::access_identity_provider::invoke(name, &crate::bindings::pulumi::cloudflare::access_identity_provider::Args {
        account_id: &args.account_id.get_inner(),
        configs: &args.configs.get_inner(),
        name: &args.name.get_inner(),
        scim_configs: &args.scim_configs.get_inner(),
        type_: &args.type_.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    AccessIdentityProviderResult {
        account_id: crate::into_domain(result.account_id),
        configs: crate::into_domain(result.configs),
        name: crate::into_domain(result.name),
        scim_configs: crate::into_domain(result.scim_configs),
        type_: crate::into_domain(result.type_),
        zone_id: crate::into_domain(result.zone_id),
    }
}
