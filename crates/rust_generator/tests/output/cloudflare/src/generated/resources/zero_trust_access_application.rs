/// Provides a Cloudflare Access Application resource. Access
/// Applications are used to restrict access to a whole application using an
/// authorisation gateway managed by Cloudflare.
///
/// > It's required that an `account_id` or `zone_id` is provided and in
///    most cases using either is fine. However, if you're using a scoped
///    access token, you must provide the argument that matches the token's
///    scope. For example, an access token that is scoped to the "example.com"
///    zone needs to use the `zone_id` argument.
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/zeroTrustAccessApplication:ZeroTrustAccessApplication example <account_id>/<application_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod zero_trust_access_application {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZeroTrustAccessApplicationArgs {
        /// The account identifier to target for the resource. Conflicts with `zone_id`.
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// When set to true, users can authenticate to this application using their WARP session. When set to false this application will always require direct IdP authentication. This setting always overrides the organization setting for WARP authentication.
        #[builder(into, default)]
        pub allow_authenticate_via_warp: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The identity providers selected for the application.
        #[builder(into, default)]
        pub allowed_idps: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The logo URL of the app launcher.
        #[builder(into, default)]
        pub app_launcher_logo_url: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Option to show/hide applications in App Launcher. Defaults to `true`.
        #[builder(into, default)]
        pub app_launcher_visible: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Option to skip identity provider selection if only one is configured in `allowed_idps`. Defaults to `false`.
        #[builder(into, default)]
        pub auto_redirect_to_identity: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The background color of the app launcher.
        #[builder(into, default)]
        pub bg_color: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// CORS configuration for the Access Application. See below for reference structure.
        #[builder(into, default)]
        pub cors_headers: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::ZeroTrustAccessApplicationCorsHeader>>,
        >,
        /// Option that returns a custom error message when a user is denied access to the application.
        #[builder(into, default)]
        pub custom_deny_message: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Option that redirects to a custom URL when a user is denied access to the application via identity based rules.
        #[builder(into, default)]
        pub custom_deny_url: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Option that redirects to a custom URL when a user is denied access to the application via non identity rules.
        #[builder(into, default)]
        pub custom_non_identity_deny_url: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The custom pages selected for the application.
        #[builder(into, default)]
        pub custom_pages: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The primary hostname and path that Access will secure. If the app is visible in the App Launcher dashboard, this is the domain that will be displayed.
        #[builder(into, default)]
        pub domain: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Option to provide increased security against compromised authorization tokens and CSRF attacks by requiring an additional "binding" cookie on requests. Defaults to `false`.
        #[builder(into, default)]
        pub enable_binding_cookie: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The footer links of the app launcher.
        #[builder(into, default)]
        pub footer_links: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::ZeroTrustAccessApplicationFooterLink>>,
        >,
        /// The background color of the header bar in the app launcher.
        #[builder(into, default)]
        pub header_bg_color: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Option to add the `HttpOnly` cookie flag to access tokens.
        #[builder(into, default)]
        pub http_only_cookie_attribute: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The landing page design of the app launcher.
        #[builder(into, default)]
        pub landing_page_design: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::ZeroTrustAccessApplicationLandingPageDesign>,
        >,
        /// Image URL for the logo shown in the app launcher dashboard.
        #[builder(into, default)]
        pub logo_url: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Friendly name of the Access Application.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Allows options preflight requests to bypass Access authentication and go directly to the origin. Cannot turn on if cors_headers is set. Defaults to `false`.
        #[builder(into, default)]
        pub options_preflight_bypass: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The policies associated with the application, in ascending order of precedence. Warning: Do not use this field while you still have this application ID referenced as `application_id` in any `cloudflare.AccessPolicy` resource, as it can result in an inconsistent state.
        #[builder(into, default)]
        pub policies: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// SaaS configuration for the Access Application.
        #[builder(into, default)]
        pub saas_app: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::ZeroTrustAccessApplicationSaasApp>,
        >,
        /// Defines the same-site cookie setting for access tokens. Available values: `none`, `lax`, `strict`.
        #[builder(into, default)]
        pub same_site_cookie_attribute: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Configuration for provisioning to this application via SCIM. This is currently in closed beta.
        #[builder(into, default)]
        pub scim_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::ZeroTrustAccessApplicationScimConfig>,
        >,
        /// List of domains that access will secure. Only present for self_hosted, vnc, and ssh applications. Always includes the value set as `domain`.
        #[builder(into, default)]
        pub self_hosted_domains: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Option to return a 401 status code in service authentication rules on failed requests. Defaults to `false`.
        #[builder(into, default)]
        pub service_auth401_redirect: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// How often a user will be forced to re-authorise. Must be in the format `48h` or `2h45m`. Defaults to `24h`.
        #[builder(into, default)]
        pub session_duration: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Option to skip the App Launcher landing page. Defaults to `false`.
        #[builder(into, default)]
        pub skip_app_launcher_login_page: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Option to skip the authorization interstitial when using the CLI. Defaults to `false`.
        #[builder(into, default)]
        pub skip_interstitial: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The itags associated with the application.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The payload for an infrastructure application which defines the port, protocol, and target attributes. Only applicable to Infrastructure Applications, in which case this field is required.
        #[builder(into, default)]
        pub target_criterias: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::ZeroTrustAccessApplicationTargetCriteria>>,
        >,
        /// The application type. Available values: `app_launcher`, `bookmark`, `biso`, `dash_sso`, `saas`, `self_hosted`, `ssh`, `vnc`, `warp`, `infrastructure`. Defaults to `self_hosted`.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The zone identifier to target for the resource. Conflicts with `account_id`.
        #[builder(into, default)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ZeroTrustAccessApplicationResult {
        /// The account identifier to target for the resource. Conflicts with `zone_id`.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// When set to true, users can authenticate to this application using their WARP session. When set to false this application will always require direct IdP authentication. This setting always overrides the organization setting for WARP authentication.
        pub allow_authenticate_via_warp: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The identity providers selected for the application.
        pub allowed_idps: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The logo URL of the app launcher.
        pub app_launcher_logo_url: pulumi_gestalt_rust::Output<Option<String>>,
        /// Option to show/hide applications in App Launcher. Defaults to `true`.
        pub app_launcher_visible: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Application Audience (AUD) Tag of the application.
        pub aud: pulumi_gestalt_rust::Output<String>,
        /// Option to skip identity provider selection if only one is configured in `allowed_idps`. Defaults to `false`.
        pub auto_redirect_to_identity: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The background color of the app launcher.
        pub bg_color: pulumi_gestalt_rust::Output<Option<String>>,
        /// CORS configuration for the Access Application. See below for reference structure.
        pub cors_headers: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::ZeroTrustAccessApplicationCorsHeader>>,
        >,
        /// Option that returns a custom error message when a user is denied access to the application.
        pub custom_deny_message: pulumi_gestalt_rust::Output<Option<String>>,
        /// Option that redirects to a custom URL when a user is denied access to the application via identity based rules.
        pub custom_deny_url: pulumi_gestalt_rust::Output<Option<String>>,
        /// Option that redirects to a custom URL when a user is denied access to the application via non identity rules.
        pub custom_non_identity_deny_url: pulumi_gestalt_rust::Output<Option<String>>,
        /// The custom pages selected for the application.
        pub custom_pages: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The primary hostname and path that Access will secure. If the app is visible in the App Launcher dashboard, this is the domain that will be displayed.
        pub domain: pulumi_gestalt_rust::Output<String>,
        /// Option to provide increased security against compromised authorization tokens and CSRF attacks by requiring an additional "binding" cookie on requests. Defaults to `false`.
        pub enable_binding_cookie: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The footer links of the app launcher.
        pub footer_links: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::ZeroTrustAccessApplicationFooterLink>>,
        >,
        /// The background color of the header bar in the app launcher.
        pub header_bg_color: pulumi_gestalt_rust::Output<Option<String>>,
        /// Option to add the `HttpOnly` cookie flag to access tokens.
        pub http_only_cookie_attribute: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The landing page design of the app launcher.
        pub landing_page_design: pulumi_gestalt_rust::Output<
            Option<super::types::ZeroTrustAccessApplicationLandingPageDesign>,
        >,
        /// Image URL for the logo shown in the app launcher dashboard.
        pub logo_url: pulumi_gestalt_rust::Output<Option<String>>,
        /// Friendly name of the Access Application.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Allows options preflight requests to bypass Access authentication and go directly to the origin. Cannot turn on if cors_headers is set. Defaults to `false`.
        pub options_preflight_bypass: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The policies associated with the application, in ascending order of precedence. Warning: Do not use this field while you still have this application ID referenced as `application_id` in any `cloudflare.AccessPolicy` resource, as it can result in an inconsistent state.
        pub policies: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// SaaS configuration for the Access Application.
        pub saas_app: pulumi_gestalt_rust::Output<
            Option<super::types::ZeroTrustAccessApplicationSaasApp>,
        >,
        /// Defines the same-site cookie setting for access tokens. Available values: `none`, `lax`, `strict`.
        pub same_site_cookie_attribute: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration for provisioning to this application via SCIM. This is currently in closed beta.
        pub scim_config: pulumi_gestalt_rust::Output<
            Option<super::types::ZeroTrustAccessApplicationScimConfig>,
        >,
        /// List of domains that access will secure. Only present for self_hosted, vnc, and ssh applications. Always includes the value set as `domain`.
        pub self_hosted_domains: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Option to return a 401 status code in service authentication rules on failed requests. Defaults to `false`.
        pub service_auth401_redirect: pulumi_gestalt_rust::Output<Option<bool>>,
        /// How often a user will be forced to re-authorise. Must be in the format `48h` or `2h45m`. Defaults to `24h`.
        pub session_duration: pulumi_gestalt_rust::Output<Option<String>>,
        /// Option to skip the App Launcher landing page. Defaults to `false`.
        pub skip_app_launcher_login_page: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Option to skip the authorization interstitial when using the CLI. Defaults to `false`.
        pub skip_interstitial: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The itags associated with the application.
        pub tags: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The payload for an infrastructure application which defines the port, protocol, and target attributes. Only applicable to Infrastructure Applications, in which case this field is required.
        pub target_criterias: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::ZeroTrustAccessApplicationTargetCriteria>>,
        >,
        /// The application type. Available values: `app_launcher`, `bookmark`, `biso`, `dash_sso`, `saas`, `self_hosted`, `ssh`, `vnc`, `warp`, `infrastructure`. Defaults to `self_hosted`.
        pub type_: pulumi_gestalt_rust::Output<Option<String>>,
        /// The zone identifier to target for the resource. Conflicts with `account_id`.
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ZeroTrustAccessApplicationArgs,
    ) -> ZeroTrustAccessApplicationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let allow_authenticate_via_warp_binding = args
            .allow_authenticate_via_warp
            .get_output(context);
        let allowed_idps_binding = args.allowed_idps.get_output(context);
        let app_launcher_logo_url_binding = args
            .app_launcher_logo_url
            .get_output(context);
        let app_launcher_visible_binding = args.app_launcher_visible.get_output(context);
        let auto_redirect_to_identity_binding = args
            .auto_redirect_to_identity
            .get_output(context);
        let bg_color_binding = args.bg_color.get_output(context);
        let cors_headers_binding = args.cors_headers.get_output(context);
        let custom_deny_message_binding = args.custom_deny_message.get_output(context);
        let custom_deny_url_binding = args.custom_deny_url.get_output(context);
        let custom_non_identity_deny_url_binding = args
            .custom_non_identity_deny_url
            .get_output(context);
        let custom_pages_binding = args.custom_pages.get_output(context);
        let domain_binding = args.domain.get_output(context);
        let enable_binding_cookie_binding = args
            .enable_binding_cookie
            .get_output(context);
        let footer_links_binding = args.footer_links.get_output(context);
        let header_bg_color_binding = args.header_bg_color.get_output(context);
        let http_only_cookie_attribute_binding = args
            .http_only_cookie_attribute
            .get_output(context);
        let landing_page_design_binding = args.landing_page_design.get_output(context);
        let logo_url_binding = args.logo_url.get_output(context);
        let name_binding = args.name.get_output(context);
        let options_preflight_bypass_binding = args
            .options_preflight_bypass
            .get_output(context);
        let policies_binding = args.policies.get_output(context);
        let saas_app_binding = args.saas_app.get_output(context);
        let same_site_cookie_attribute_binding = args
            .same_site_cookie_attribute
            .get_output(context);
        let scim_config_binding = args.scim_config.get_output(context);
        let self_hosted_domains_binding = args.self_hosted_domains.get_output(context);
        let service_auth401_redirect_binding = args
            .service_auth401_redirect
            .get_output(context);
        let session_duration_binding = args.session_duration.get_output(context);
        let skip_app_launcher_login_page_binding = args
            .skip_app_launcher_login_page
            .get_output(context);
        let skip_interstitial_binding = args.skip_interstitial.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let target_criterias_binding = args.target_criterias.get_output(context);
        let type__binding = args.type_.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustAccessApplication:ZeroTrustAccessApplication"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowAuthenticateViaWarp".into(),
                    value: &allow_authenticate_via_warp_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowedIdps".into(),
                    value: &allowed_idps_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appLauncherLogoUrl".into(),
                    value: &app_launcher_logo_url_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appLauncherVisible".into(),
                    value: &app_launcher_visible_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoRedirectToIdentity".into(),
                    value: &auto_redirect_to_identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bgColor".into(),
                    value: &bg_color_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "corsHeaders".into(),
                    value: &cors_headers_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customDenyMessage".into(),
                    value: &custom_deny_message_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customDenyUrl".into(),
                    value: &custom_deny_url_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customNonIdentityDenyUrl".into(),
                    value: &custom_non_identity_deny_url_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customPages".into(),
                    value: &custom_pages_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domain".into(),
                    value: &domain_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableBindingCookie".into(),
                    value: &enable_binding_cookie_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "footerLinks".into(),
                    value: &footer_links_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "headerBgColor".into(),
                    value: &header_bg_color_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "httpOnlyCookieAttribute".into(),
                    value: &http_only_cookie_attribute_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "landingPageDesign".into(),
                    value: &landing_page_design_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logoUrl".into(),
                    value: &logo_url_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "optionsPreflightBypass".into(),
                    value: &options_preflight_bypass_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policies".into(),
                    value: &policies_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "saasApp".into(),
                    value: &saas_app_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sameSiteCookieAttribute".into(),
                    value: &same_site_cookie_attribute_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scimConfig".into(),
                    value: &scim_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "selfHostedDomains".into(),
                    value: &self_hosted_domains_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceAuth401Redirect".into(),
                    value: &service_auth401_redirect_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sessionDuration".into(),
                    value: &session_duration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skipAppLauncherLoginPage".into(),
                    value: &skip_app_launcher_login_page_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skipInterstitial".into(),
                    value: &skip_interstitial_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetCriterias".into(),
                    value: &target_criterias_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ZeroTrustAccessApplicationResult {
            account_id: o.get_field("accountId"),
            allow_authenticate_via_warp: o.get_field("allowAuthenticateViaWarp"),
            allowed_idps: o.get_field("allowedIdps"),
            app_launcher_logo_url: o.get_field("appLauncherLogoUrl"),
            app_launcher_visible: o.get_field("appLauncherVisible"),
            aud: o.get_field("aud"),
            auto_redirect_to_identity: o.get_field("autoRedirectToIdentity"),
            bg_color: o.get_field("bgColor"),
            cors_headers: o.get_field("corsHeaders"),
            custom_deny_message: o.get_field("customDenyMessage"),
            custom_deny_url: o.get_field("customDenyUrl"),
            custom_non_identity_deny_url: o.get_field("customNonIdentityDenyUrl"),
            custom_pages: o.get_field("customPages"),
            domain: o.get_field("domain"),
            enable_binding_cookie: o.get_field("enableBindingCookie"),
            footer_links: o.get_field("footerLinks"),
            header_bg_color: o.get_field("headerBgColor"),
            http_only_cookie_attribute: o.get_field("httpOnlyCookieAttribute"),
            landing_page_design: o.get_field("landingPageDesign"),
            logo_url: o.get_field("logoUrl"),
            name: o.get_field("name"),
            options_preflight_bypass: o.get_field("optionsPreflightBypass"),
            policies: o.get_field("policies"),
            saas_app: o.get_field("saasApp"),
            same_site_cookie_attribute: o.get_field("sameSiteCookieAttribute"),
            scim_config: o.get_field("scimConfig"),
            self_hosted_domains: o.get_field("selfHostedDomains"),
            service_auth401_redirect: o.get_field("serviceAuth401Redirect"),
            session_duration: o.get_field("sessionDuration"),
            skip_app_launcher_login_page: o.get_field("skipAppLauncherLoginPage"),
            skip_interstitial: o.get_field("skipInterstitial"),
            tags: o.get_field("tags"),
            target_criterias: o.get_field("targetCriterias"),
            type_: o.get_field("type"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
