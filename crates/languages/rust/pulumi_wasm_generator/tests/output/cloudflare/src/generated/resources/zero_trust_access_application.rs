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
pub mod zero_trust_access_application {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZeroTrustAccessApplicationArgs {
        /// The account identifier to target for the resource. Conflicts with `zone_id`.
        #[builder(into, default)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// When set to true, users can authenticate to this application using their WARP session. When set to false this application will always require direct IdP authentication. This setting always overrides the organization setting for WARP authentication.
        #[builder(into, default)]
        pub allow_authenticate_via_warp: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The identity providers selected for the application.
        #[builder(into, default)]
        pub allowed_idps: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The logo URL of the app launcher.
        #[builder(into, default)]
        pub app_launcher_logo_url: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Option to show/hide applications in App Launcher. Defaults to `true`.
        #[builder(into, default)]
        pub app_launcher_visible: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Option to skip identity provider selection if only one is configured in `allowed_idps`. Defaults to `false`.
        #[builder(into, default)]
        pub auto_redirect_to_identity: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The background color of the app launcher.
        #[builder(into, default)]
        pub bg_color: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// CORS configuration for the Access Application. See below for reference structure.
        #[builder(into, default)]
        pub cors_headers: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::types::ZeroTrustAccessApplicationCorsHeader>>,
        >,
        /// Option that returns a custom error message when a user is denied access to the application.
        #[builder(into, default)]
        pub custom_deny_message: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Option that redirects to a custom URL when a user is denied access to the application via identity based rules.
        #[builder(into, default)]
        pub custom_deny_url: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Option that redirects to a custom URL when a user is denied access to the application via non identity rules.
        #[builder(into, default)]
        pub custom_non_identity_deny_url: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// The custom pages selected for the application.
        #[builder(into, default)]
        pub custom_pages: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The primary hostname and path that Access will secure. If the app is visible in the App Launcher dashboard, this is the domain that will be displayed.
        #[builder(into, default)]
        pub domain: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Option to provide increased security against compromised authorization tokens and CSRF attacks by requiring an additional "binding" cookie on requests. Defaults to `false`.
        #[builder(into, default)]
        pub enable_binding_cookie: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The footer links of the app launcher.
        #[builder(into, default)]
        pub footer_links: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::types::ZeroTrustAccessApplicationFooterLink>>,
        >,
        /// The background color of the header bar in the app launcher.
        #[builder(into, default)]
        pub header_bg_color: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Option to add the `HttpOnly` cookie flag to access tokens.
        #[builder(into, default)]
        pub http_only_cookie_attribute: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The landing page design of the app launcher.
        #[builder(into, default)]
        pub landing_page_design: pulumi_wasm_rust::InputOrOutput<
            Option<super::types::ZeroTrustAccessApplicationLandingPageDesign>,
        >,
        /// Image URL for the logo shown in the app launcher dashboard.
        #[builder(into, default)]
        pub logo_url: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Friendly name of the Access Application.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Allows options preflight requests to bypass Access authentication and go directly to the origin. Cannot turn on if cors_headers is set. Defaults to `false`.
        #[builder(into, default)]
        pub options_preflight_bypass: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The policies associated with the application, in ascending order of precedence. Warning: Do not use this field while you still have this application ID referenced as `application_id` in any `cloudflare.AccessPolicy` resource, as it can result in an inconsistent state.
        #[builder(into, default)]
        pub policies: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// SaaS configuration for the Access Application.
        #[builder(into, default)]
        pub saas_app: pulumi_wasm_rust::InputOrOutput<
            Option<super::types::ZeroTrustAccessApplicationSaasApp>,
        >,
        /// Defines the same-site cookie setting for access tokens. Available values: `none`, `lax`, `strict`.
        #[builder(into, default)]
        pub same_site_cookie_attribute: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Configuration for provisioning to this application via SCIM. This is currently in closed beta.
        #[builder(into, default)]
        pub scim_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::types::ZeroTrustAccessApplicationScimConfig>,
        >,
        /// List of domains that access will secure. Only present for self_hosted, vnc, and ssh applications. Always includes the value set as `domain`.
        #[builder(into, default)]
        pub self_hosted_domains: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Option to return a 401 status code in service authentication rules on failed requests. Defaults to `false`.
        #[builder(into, default)]
        pub service_auth401_redirect: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// How often a user will be forced to re-authorise. Must be in the format `48h` or `2h45m`. Defaults to `24h`.
        #[builder(into, default)]
        pub session_duration: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Option to skip the App Launcher landing page. Defaults to `false`.
        #[builder(into, default)]
        pub skip_app_launcher_login_page: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Option to skip the authorization interstitial when using the CLI. Defaults to `false`.
        #[builder(into, default)]
        pub skip_interstitial: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The itags associated with the application.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The payload for an infrastructure application which defines the port, protocol, and target attributes. Only applicable to Infrastructure Applications, in which case this field is required.
        #[builder(into, default)]
        pub target_criterias: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::types::ZeroTrustAccessApplicationTargetCriteria>>,
        >,
        /// The application type. Available values: `app_launcher`, `bookmark`, `biso`, `dash_sso`, `saas`, `self_hosted`, `ssh`, `vnc`, `warp`, `infrastructure`. Defaults to `self_hosted`.
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The zone identifier to target for the resource. Conflicts with `account_id`.
        #[builder(into, default)]
        pub zone_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ZeroTrustAccessApplicationResult {
        /// The account identifier to target for the resource. Conflicts with `zone_id`.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// When set to true, users can authenticate to this application using their WARP session. When set to false this application will always require direct IdP authentication. This setting always overrides the organization setting for WARP authentication.
        pub allow_authenticate_via_warp: pulumi_wasm_rust::Output<Option<bool>>,
        /// The identity providers selected for the application.
        pub allowed_idps: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The logo URL of the app launcher.
        pub app_launcher_logo_url: pulumi_wasm_rust::Output<Option<String>>,
        /// Option to show/hide applications in App Launcher. Defaults to `true`.
        pub app_launcher_visible: pulumi_wasm_rust::Output<Option<bool>>,
        /// Application Audience (AUD) Tag of the application.
        pub aud: pulumi_wasm_rust::Output<String>,
        /// Option to skip identity provider selection if only one is configured in `allowed_idps`. Defaults to `false`.
        pub auto_redirect_to_identity: pulumi_wasm_rust::Output<Option<bool>>,
        /// The background color of the app launcher.
        pub bg_color: pulumi_wasm_rust::Output<Option<String>>,
        /// CORS configuration for the Access Application. See below for reference structure.
        pub cors_headers: pulumi_wasm_rust::Output<
            Option<Vec<super::types::ZeroTrustAccessApplicationCorsHeader>>,
        >,
        /// Option that returns a custom error message when a user is denied access to the application.
        pub custom_deny_message: pulumi_wasm_rust::Output<Option<String>>,
        /// Option that redirects to a custom URL when a user is denied access to the application via identity based rules.
        pub custom_deny_url: pulumi_wasm_rust::Output<Option<String>>,
        /// Option that redirects to a custom URL when a user is denied access to the application via non identity rules.
        pub custom_non_identity_deny_url: pulumi_wasm_rust::Output<Option<String>>,
        /// The custom pages selected for the application.
        pub custom_pages: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The primary hostname and path that Access will secure. If the app is visible in the App Launcher dashboard, this is the domain that will be displayed.
        pub domain: pulumi_wasm_rust::Output<String>,
        /// Option to provide increased security against compromised authorization tokens and CSRF attacks by requiring an additional "binding" cookie on requests. Defaults to `false`.
        pub enable_binding_cookie: pulumi_wasm_rust::Output<Option<bool>>,
        /// The footer links of the app launcher.
        pub footer_links: pulumi_wasm_rust::Output<
            Option<Vec<super::types::ZeroTrustAccessApplicationFooterLink>>,
        >,
        /// The background color of the header bar in the app launcher.
        pub header_bg_color: pulumi_wasm_rust::Output<Option<String>>,
        /// Option to add the `HttpOnly` cookie flag to access tokens.
        pub http_only_cookie_attribute: pulumi_wasm_rust::Output<Option<bool>>,
        /// The landing page design of the app launcher.
        pub landing_page_design: pulumi_wasm_rust::Output<
            Option<super::types::ZeroTrustAccessApplicationLandingPageDesign>,
        >,
        /// Image URL for the logo shown in the app launcher dashboard.
        pub logo_url: pulumi_wasm_rust::Output<Option<String>>,
        /// Friendly name of the Access Application.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Allows options preflight requests to bypass Access authentication and go directly to the origin. Cannot turn on if cors_headers is set. Defaults to `false`.
        pub options_preflight_bypass: pulumi_wasm_rust::Output<Option<bool>>,
        /// The policies associated with the application, in ascending order of precedence. Warning: Do not use this field while you still have this application ID referenced as `application_id` in any `cloudflare.AccessPolicy` resource, as it can result in an inconsistent state.
        pub policies: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// SaaS configuration for the Access Application.
        pub saas_app: pulumi_wasm_rust::Output<
            Option<super::types::ZeroTrustAccessApplicationSaasApp>,
        >,
        /// Defines the same-site cookie setting for access tokens. Available values: `none`, `lax`, `strict`.
        pub same_site_cookie_attribute: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration for provisioning to this application via SCIM. This is currently in closed beta.
        pub scim_config: pulumi_wasm_rust::Output<
            Option<super::types::ZeroTrustAccessApplicationScimConfig>,
        >,
        /// List of domains that access will secure. Only present for self_hosted, vnc, and ssh applications. Always includes the value set as `domain`.
        pub self_hosted_domains: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Option to return a 401 status code in service authentication rules on failed requests. Defaults to `false`.
        pub service_auth401_redirect: pulumi_wasm_rust::Output<Option<bool>>,
        /// How often a user will be forced to re-authorise. Must be in the format `48h` or `2h45m`. Defaults to `24h`.
        pub session_duration: pulumi_wasm_rust::Output<Option<String>>,
        /// Option to skip the App Launcher landing page. Defaults to `false`.
        pub skip_app_launcher_login_page: pulumi_wasm_rust::Output<Option<bool>>,
        /// Option to skip the authorization interstitial when using the CLI. Defaults to `false`.
        pub skip_interstitial: pulumi_wasm_rust::Output<Option<bool>>,
        /// The itags associated with the application.
        pub tags: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The payload for an infrastructure application which defines the port, protocol, and target attributes. Only applicable to Infrastructure Applications, in which case this field is required.
        pub target_criterias: pulumi_wasm_rust::Output<
            Option<Vec<super::types::ZeroTrustAccessApplicationTargetCriteria>>,
        >,
        /// The application type. Available values: `app_launcher`, `bookmark`, `biso`, `dash_sso`, `saas`, `self_hosted`, `ssh`, `vnc`, `warp`, `infrastructure`. Defaults to `self_hosted`.
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
        /// The zone identifier to target for the resource. Conflicts with `account_id`.
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ZeroTrustAccessApplicationArgs,
    ) -> ZeroTrustAccessApplicationResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let allow_authenticate_via_warp_binding = args
            .allow_authenticate_via_warp
            .get_output(context)
            .get_inner();
        let allowed_idps_binding = args.allowed_idps.get_output(context).get_inner();
        let app_launcher_logo_url_binding = args
            .app_launcher_logo_url
            .get_output(context)
            .get_inner();
        let app_launcher_visible_binding = args
            .app_launcher_visible
            .get_output(context)
            .get_inner();
        let auto_redirect_to_identity_binding = args
            .auto_redirect_to_identity
            .get_output(context)
            .get_inner();
        let bg_color_binding = args.bg_color.get_output(context).get_inner();
        let cors_headers_binding = args.cors_headers.get_output(context).get_inner();
        let custom_deny_message_binding = args
            .custom_deny_message
            .get_output(context)
            .get_inner();
        let custom_deny_url_binding = args
            .custom_deny_url
            .get_output(context)
            .get_inner();
        let custom_non_identity_deny_url_binding = args
            .custom_non_identity_deny_url
            .get_output(context)
            .get_inner();
        let custom_pages_binding = args.custom_pages.get_output(context).get_inner();
        let domain_binding = args.domain.get_output(context).get_inner();
        let enable_binding_cookie_binding = args
            .enable_binding_cookie
            .get_output(context)
            .get_inner();
        let footer_links_binding = args.footer_links.get_output(context).get_inner();
        let header_bg_color_binding = args
            .header_bg_color
            .get_output(context)
            .get_inner();
        let http_only_cookie_attribute_binding = args
            .http_only_cookie_attribute
            .get_output(context)
            .get_inner();
        let landing_page_design_binding = args
            .landing_page_design
            .get_output(context)
            .get_inner();
        let logo_url_binding = args.logo_url.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let options_preflight_bypass_binding = args
            .options_preflight_bypass
            .get_output(context)
            .get_inner();
        let policies_binding = args.policies.get_output(context).get_inner();
        let saas_app_binding = args.saas_app.get_output(context).get_inner();
        let same_site_cookie_attribute_binding = args
            .same_site_cookie_attribute
            .get_output(context)
            .get_inner();
        let scim_config_binding = args.scim_config.get_output(context).get_inner();
        let self_hosted_domains_binding = args
            .self_hosted_domains
            .get_output(context)
            .get_inner();
        let service_auth401_redirect_binding = args
            .service_auth401_redirect
            .get_output(context)
            .get_inner();
        let session_duration_binding = args
            .session_duration
            .get_output(context)
            .get_inner();
        let skip_app_launcher_login_page_binding = args
            .skip_app_launcher_login_page
            .get_output(context)
            .get_inner();
        let skip_interstitial_binding = args
            .skip_interstitial
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let target_criterias_binding = args
            .target_criterias
            .get_output(context)
            .get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let zone_id_binding = args.zone_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustAccessApplication:ZeroTrustAccessApplication"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "allowAuthenticateViaWarp".into(),
                    value: &allow_authenticate_via_warp_binding,
                },
                register_interface::ObjectField {
                    name: "allowedIdps".into(),
                    value: &allowed_idps_binding,
                },
                register_interface::ObjectField {
                    name: "appLauncherLogoUrl".into(),
                    value: &app_launcher_logo_url_binding,
                },
                register_interface::ObjectField {
                    name: "appLauncherVisible".into(),
                    value: &app_launcher_visible_binding,
                },
                register_interface::ObjectField {
                    name: "autoRedirectToIdentity".into(),
                    value: &auto_redirect_to_identity_binding,
                },
                register_interface::ObjectField {
                    name: "bgColor".into(),
                    value: &bg_color_binding,
                },
                register_interface::ObjectField {
                    name: "corsHeaders".into(),
                    value: &cors_headers_binding,
                },
                register_interface::ObjectField {
                    name: "customDenyMessage".into(),
                    value: &custom_deny_message_binding,
                },
                register_interface::ObjectField {
                    name: "customDenyUrl".into(),
                    value: &custom_deny_url_binding,
                },
                register_interface::ObjectField {
                    name: "customNonIdentityDenyUrl".into(),
                    value: &custom_non_identity_deny_url_binding,
                },
                register_interface::ObjectField {
                    name: "customPages".into(),
                    value: &custom_pages_binding,
                },
                register_interface::ObjectField {
                    name: "domain".into(),
                    value: &domain_binding,
                },
                register_interface::ObjectField {
                    name: "enableBindingCookie".into(),
                    value: &enable_binding_cookie_binding,
                },
                register_interface::ObjectField {
                    name: "footerLinks".into(),
                    value: &footer_links_binding,
                },
                register_interface::ObjectField {
                    name: "headerBgColor".into(),
                    value: &header_bg_color_binding,
                },
                register_interface::ObjectField {
                    name: "httpOnlyCookieAttribute".into(),
                    value: &http_only_cookie_attribute_binding,
                },
                register_interface::ObjectField {
                    name: "landingPageDesign".into(),
                    value: &landing_page_design_binding,
                },
                register_interface::ObjectField {
                    name: "logoUrl".into(),
                    value: &logo_url_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "optionsPreflightBypass".into(),
                    value: &options_preflight_bypass_binding,
                },
                register_interface::ObjectField {
                    name: "policies".into(),
                    value: &policies_binding,
                },
                register_interface::ObjectField {
                    name: "saasApp".into(),
                    value: &saas_app_binding,
                },
                register_interface::ObjectField {
                    name: "sameSiteCookieAttribute".into(),
                    value: &same_site_cookie_attribute_binding,
                },
                register_interface::ObjectField {
                    name: "scimConfig".into(),
                    value: &scim_config_binding,
                },
                register_interface::ObjectField {
                    name: "selfHostedDomains".into(),
                    value: &self_hosted_domains_binding,
                },
                register_interface::ObjectField {
                    name: "serviceAuth401Redirect".into(),
                    value: &service_auth401_redirect_binding,
                },
                register_interface::ObjectField {
                    name: "sessionDuration".into(),
                    value: &session_duration_binding,
                },
                register_interface::ObjectField {
                    name: "skipAppLauncherLoginPage".into(),
                    value: &skip_app_launcher_login_page_binding,
                },
                register_interface::ObjectField {
                    name: "skipInterstitial".into(),
                    value: &skip_interstitial_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "targetCriterias".into(),
                    value: &target_criterias_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ZeroTrustAccessApplicationResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            allow_authenticate_via_warp: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("allowAuthenticateViaWarp"),
            ),
            allowed_idps: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("allowedIdps"),
            ),
            app_launcher_logo_url: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("appLauncherLogoUrl"),
            ),
            app_launcher_visible: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("appLauncherVisible"),
            ),
            aud: pulumi_wasm_rust::__private::into_domain(o.extract_field("aud")),
            auto_redirect_to_identity: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("autoRedirectToIdentity"),
            ),
            bg_color: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("bgColor"),
            ),
            cors_headers: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("corsHeaders"),
            ),
            custom_deny_message: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customDenyMessage"),
            ),
            custom_deny_url: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customDenyUrl"),
            ),
            custom_non_identity_deny_url: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customNonIdentityDenyUrl"),
            ),
            custom_pages: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customPages"),
            ),
            domain: pulumi_wasm_rust::__private::into_domain(o.extract_field("domain")),
            enable_binding_cookie: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enableBindingCookie"),
            ),
            footer_links: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("footerLinks"),
            ),
            header_bg_color: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("headerBgColor"),
            ),
            http_only_cookie_attribute: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("httpOnlyCookieAttribute"),
            ),
            landing_page_design: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("landingPageDesign"),
            ),
            logo_url: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("logoUrl"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            options_preflight_bypass: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("optionsPreflightBypass"),
            ),
            policies: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("policies"),
            ),
            saas_app: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("saasApp"),
            ),
            same_site_cookie_attribute: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sameSiteCookieAttribute"),
            ),
            scim_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("scimConfig"),
            ),
            self_hosted_domains: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("selfHostedDomains"),
            ),
            service_auth401_redirect: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceAuth401Redirect"),
            ),
            session_duration: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sessionDuration"),
            ),
            skip_app_launcher_login_page: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("skipAppLauncherLoginPage"),
            ),
            skip_interstitial: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("skipInterstitial"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            target_criterias: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("targetCriterias"),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
            zone_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("zoneId")),
        }
    }
}
