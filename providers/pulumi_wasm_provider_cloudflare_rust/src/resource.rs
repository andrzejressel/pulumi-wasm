pub mod access_application {

    pub struct AccessApplicationArgs {
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        pub allow_authenticate_via_warp: pulumi_wasm_rust::Output<Option<bool>>,
        pub allowed_idps: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub app_launcher_logo_url: pulumi_wasm_rust::Output<Option<String>>,
        pub app_launcher_visible: pulumi_wasm_rust::Output<Option<bool>>,
        pub auto_redirect_to_identity: pulumi_wasm_rust::Output<Option<bool>>,
        pub bg_color: pulumi_wasm_rust::Output<Option<String>>,
        pub cors_headers:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessApplicationCorsHeader>>>,
        pub custom_deny_message: pulumi_wasm_rust::Output<Option<String>>,
        pub custom_deny_url: pulumi_wasm_rust::Output<Option<String>>,
        pub custom_non_identity_deny_url: pulumi_wasm_rust::Output<Option<String>>,
        pub custom_pages: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub domain: pulumi_wasm_rust::Output<Option<String>>,
        pub enable_binding_cookie: pulumi_wasm_rust::Output<Option<bool>>,
        pub footer_links:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessApplicationFooterLink>>>,
        pub header_bg_color: pulumi_wasm_rust::Output<Option<String>>,
        pub http_only_cookie_attribute: pulumi_wasm_rust::Output<Option<bool>>,
        pub landing_page_design:
            pulumi_wasm_rust::Output<Option<crate::types::AccessApplicationLandingPageDesign>>,
        pub logo_url: pulumi_wasm_rust::Output<Option<String>>,
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        pub saas_app: pulumi_wasm_rust::Output<Option<crate::types::AccessApplicationSaasApp>>,
        pub same_site_cookie_attribute: pulumi_wasm_rust::Output<Option<String>>,
        pub self_hosted_domains: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub service_auth401_redirect: pulumi_wasm_rust::Output<Option<bool>>,
        pub session_duration: pulumi_wasm_rust::Output<Option<String>>,
        pub skip_interstitial: pulumi_wasm_rust::Output<Option<bool>>,
        pub tags: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
        pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct AccessApplicationResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub allow_authenticate_via_warp: pulumi_wasm_rust::Output<Option<bool>>,
        pub allowed_idps: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub app_launcher_logo_url: pulumi_wasm_rust::Output<Option<String>>,
        pub app_launcher_visible: pulumi_wasm_rust::Output<Option<bool>>,
        pub aud: pulumi_wasm_rust::Output<String>,
        pub auto_redirect_to_identity: pulumi_wasm_rust::Output<Option<bool>>,
        pub bg_color: pulumi_wasm_rust::Output<Option<String>>,
        pub cors_headers:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessApplicationCorsHeader>>>,
        pub custom_deny_message: pulumi_wasm_rust::Output<Option<String>>,
        pub custom_deny_url: pulumi_wasm_rust::Output<Option<String>>,
        pub custom_non_identity_deny_url: pulumi_wasm_rust::Output<Option<String>>,
        pub custom_pages: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub domain: pulumi_wasm_rust::Output<String>,
        pub enable_binding_cookie: pulumi_wasm_rust::Output<Option<bool>>,
        pub footer_links:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessApplicationFooterLink>>>,
        pub header_bg_color: pulumi_wasm_rust::Output<Option<String>>,
        pub http_only_cookie_attribute: pulumi_wasm_rust::Output<Option<bool>>,
        pub landing_page_design:
            pulumi_wasm_rust::Output<Option<crate::types::AccessApplicationLandingPageDesign>>,
        pub logo_url: pulumi_wasm_rust::Output<Option<String>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub saas_app: pulumi_wasm_rust::Output<Option<crate::types::AccessApplicationSaasApp>>,
        pub same_site_cookie_attribute: pulumi_wasm_rust::Output<Option<String>>,
        pub self_hosted_domains: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub service_auth401_redirect: pulumi_wasm_rust::Output<Option<bool>>,
        pub session_duration: pulumi_wasm_rust::Output<Option<String>>,
        pub skip_interstitial: pulumi_wasm_rust::Output<Option<bool>>,
        pub tags: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn access_application(name: &str, args: AccessApplicationArgs) -> AccessApplicationResult {
        let result = crate::bindings::pulumi::cloudflare::access_application::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::access_application::Args {
                account_id: args.account_id.get_inner(),
                allow_authenticate_via_warp: args.allow_authenticate_via_warp.get_inner(),
                allowed_idps: args.allowed_idps.get_inner(),
                app_launcher_logo_url: args.app_launcher_logo_url.get_inner(),
                app_launcher_visible: args.app_launcher_visible.get_inner(),
                auto_redirect_to_identity: args.auto_redirect_to_identity.get_inner(),
                bg_color: args.bg_color.get_inner(),
                cors_headers: args.cors_headers.get_inner(),
                custom_deny_message: args.custom_deny_message.get_inner(),
                custom_deny_url: args.custom_deny_url.get_inner(),
                custom_non_identity_deny_url: args.custom_non_identity_deny_url.get_inner(),
                custom_pages: args.custom_pages.get_inner(),
                domain: args.domain.get_inner(),
                enable_binding_cookie: args.enable_binding_cookie.get_inner(),
                footer_links: args.footer_links.get_inner(),
                header_bg_color: args.header_bg_color.get_inner(),
                http_only_cookie_attribute: args.http_only_cookie_attribute.get_inner(),
                landing_page_design: args.landing_page_design.get_inner(),
                logo_url: args.logo_url.get_inner(),
                name: args.name.get_inner(),
                saas_app: args.saas_app.get_inner(),
                same_site_cookie_attribute: args.same_site_cookie_attribute.get_inner(),
                self_hosted_domains: args.self_hosted_domains.get_inner(),
                service_auth401_redirect: args.service_auth401_redirect.get_inner(),
                session_duration: args.session_duration.get_inner(),
                skip_interstitial: args.skip_interstitial.get_inner(),
                tags: args.tags.get_inner(),
                type_: args.type_.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        AccessApplicationResult {
            account_id: crate::into_domain(result.account_id),
            allow_authenticate_via_warp: crate::into_domain(result.allow_authenticate_via_warp),
            allowed_idps: crate::into_domain(result.allowed_idps),
            app_launcher_logo_url: crate::into_domain(result.app_launcher_logo_url),
            app_launcher_visible: crate::into_domain(result.app_launcher_visible),
            aud: crate::into_domain(result.aud),
            auto_redirect_to_identity: crate::into_domain(result.auto_redirect_to_identity),
            bg_color: crate::into_domain(result.bg_color),
            cors_headers: crate::into_domain(result.cors_headers),
            custom_deny_message: crate::into_domain(result.custom_deny_message),
            custom_deny_url: crate::into_domain(result.custom_deny_url),
            custom_non_identity_deny_url: crate::into_domain(result.custom_non_identity_deny_url),
            custom_pages: crate::into_domain(result.custom_pages),
            domain: crate::into_domain(result.domain),
            enable_binding_cookie: crate::into_domain(result.enable_binding_cookie),
            footer_links: crate::into_domain(result.footer_links),
            header_bg_color: crate::into_domain(result.header_bg_color),
            http_only_cookie_attribute: crate::into_domain(result.http_only_cookie_attribute),
            landing_page_design: crate::into_domain(result.landing_page_design),
            logo_url: crate::into_domain(result.logo_url),
            name: crate::into_domain(result.name),
            saas_app: crate::into_domain(result.saas_app),
            same_site_cookie_attribute: crate::into_domain(result.same_site_cookie_attribute),
            self_hosted_domains: crate::into_domain(result.self_hosted_domains),
            service_auth401_redirect: crate::into_domain(result.service_auth401_redirect),
            session_duration: crate::into_domain(result.session_duration),
            skip_interstitial: crate::into_domain(result.skip_interstitial),
            tags: crate::into_domain(result.tags),
            type_: crate::into_domain(result.type_),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod access_ca_certificate {

    pub struct AccessCaCertificateArgs {
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        pub application_id: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct AccessCaCertificateResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub application_id: pulumi_wasm_rust::Output<String>,
        pub aud: pulumi_wasm_rust::Output<String>,
        pub public_key: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn access_ca_certificate(
        name: &str,
        args: AccessCaCertificateArgs,
    ) -> AccessCaCertificateResult {
        let result = crate::bindings::pulumi::cloudflare::access_ca_certificate::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::access_ca_certificate::Args {
                account_id: args.account_id.get_inner(),
                application_id: args.application_id.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        AccessCaCertificateResult {
            account_id: crate::into_domain(result.account_id),
            application_id: crate::into_domain(result.application_id),
            aud: crate::into_domain(result.aud),
            public_key: crate::into_domain(result.public_key),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod access_custom_page {

    pub struct AccessCustomPageArgs {
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        pub app_count: pulumi_wasm_rust::Output<Option<i32>>,
        pub custom_html: pulumi_wasm_rust::Output<Option<String>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub type_: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct AccessCustomPageResult {
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        pub app_count: pulumi_wasm_rust::Output<Option<i32>>,
        pub custom_html: pulumi_wasm_rust::Output<Option<String>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub type_: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub fn access_custom_page(name: &str, args: AccessCustomPageArgs) -> AccessCustomPageResult {
        let result = crate::bindings::pulumi::cloudflare::access_custom_page::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::access_custom_page::Args {
                account_id: args.account_id.get_inner(),
                app_count: args.app_count.get_inner(),
                custom_html: args.custom_html.get_inner(),
                name: args.name.get_inner(),
                type_: args.type_.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        AccessCustomPageResult {
            account_id: crate::into_domain(result.account_id),
            app_count: crate::into_domain(result.app_count),
            custom_html: crate::into_domain(result.custom_html),
            name: crate::into_domain(result.name),
            type_: crate::into_domain(result.type_),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod access_group {

    pub struct AccessGroupArgs {
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        pub excludes: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessGroupExclude>>>,
        pub includes: pulumi_wasm_rust::Output<Vec<crate::types::AccessGroupInclude>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub requires: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessGroupRequire>>>,
        pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct AccessGroupResult {
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        pub excludes: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessGroupExclude>>>,
        pub includes: pulumi_wasm_rust::Output<Vec<crate::types::AccessGroupInclude>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub requires: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessGroupRequire>>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn access_group(name: &str, args: AccessGroupArgs) -> AccessGroupResult {
        let result = crate::bindings::pulumi::cloudflare::access_group::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::access_group::Args {
                account_id: args.account_id.get_inner(),
                excludes: args.excludes.get_inner(),
                includes: args.includes.get_inner(),
                name: args.name.get_inner(),
                requires: args.requires.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        AccessGroupResult {
            account_id: crate::into_domain(result.account_id),
            excludes: crate::into_domain(result.excludes),
            includes: crate::into_domain(result.includes),
            name: crate::into_domain(result.name),
            requires: crate::into_domain(result.requires),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod access_identity_provider {

    pub struct AccessIdentityProviderArgs {
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        pub configs:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessIdentityProviderConfig>>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub scim_configs:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessIdentityProviderScimConfig>>>,
        pub type_: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct AccessIdentityProviderResult {
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        pub configs: pulumi_wasm_rust::Output<Vec<crate::types::AccessIdentityProviderConfig>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub scim_configs:
            pulumi_wasm_rust::Output<Vec<crate::types::AccessIdentityProviderScimConfig>>,
        pub type_: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub fn access_identity_provider(
        name: &str,
        args: AccessIdentityProviderArgs,
    ) -> AccessIdentityProviderResult {
        let result = crate::bindings::pulumi::cloudflare::access_identity_provider::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::access_identity_provider::Args {
                account_id: args.account_id.get_inner(),
                configs: args.configs.get_inner(),
                name: args.name.get_inner(),
                scim_configs: args.scim_configs.get_inner(),
                type_: args.type_.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        AccessIdentityProviderResult {
            account_id: crate::into_domain(result.account_id),
            configs: crate::into_domain(result.configs),
            name: crate::into_domain(result.name),
            scim_configs: crate::into_domain(result.scim_configs),
            type_: crate::into_domain(result.type_),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod access_keys_configuration {

    pub struct AccessKeysConfigurationArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub key_rotation_interval_days: pulumi_wasm_rust::Output<Option<i32>>,
    }

    pub struct AccessKeysConfigurationResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub key_rotation_interval_days: pulumi_wasm_rust::Output<i32>,
    }

    pub fn access_keys_configuration(
        name: &str,
        args: AccessKeysConfigurationArgs,
    ) -> AccessKeysConfigurationResult {
        let result = crate::bindings::pulumi::cloudflare::access_keys_configuration::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::access_keys_configuration::Args {
                account_id: args.account_id.get_inner(),
                key_rotation_interval_days: args.key_rotation_interval_days.get_inner(),
            },
        );

        AccessKeysConfigurationResult {
            account_id: crate::into_domain(result.account_id),
            key_rotation_interval_days: crate::into_domain(result.key_rotation_interval_days),
        }
    }
}

pub mod access_mutual_tls_certificate {

    pub struct AccessMutualTlsCertificateArgs {
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        pub associated_hostnames: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub certificate: pulumi_wasm_rust::Output<Option<String>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct AccessMutualTlsCertificateResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub associated_hostnames: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub certificate: pulumi_wasm_rust::Output<Option<String>>,
        pub fingerprint: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn access_mutual_tls_certificate(
        name: &str,
        args: AccessMutualTlsCertificateArgs,
    ) -> AccessMutualTlsCertificateResult {
        let result = crate::bindings::pulumi::cloudflare::access_mutual_tls_certificate::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::access_mutual_tls_certificate::Args {
                account_id: args.account_id.get_inner(),
                associated_hostnames: args.associated_hostnames.get_inner(),
                certificate: args.certificate.get_inner(),
                name: args.name.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        AccessMutualTlsCertificateResult {
            account_id: crate::into_domain(result.account_id),
            associated_hostnames: crate::into_domain(result.associated_hostnames),
            certificate: crate::into_domain(result.certificate),
            fingerprint: crate::into_domain(result.fingerprint),
            name: crate::into_domain(result.name),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod access_mutual_tls_hostname_settings {

    pub struct AccessMutualTlsHostnameSettingsArgs {
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        pub settings: pulumi_wasm_rust::Output<
            Option<Vec<crate::types::AccessMutualTlsHostnameSettingsSetting>>,
        >,
        pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct AccessMutualTlsHostnameSettingsResult {
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        pub settings: pulumi_wasm_rust::Output<
            Option<Vec<crate::types::AccessMutualTlsHostnameSettingsSetting>>,
        >,
        pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub fn access_mutual_tls_hostname_settings(
        name: &str,
        args: AccessMutualTlsHostnameSettingsArgs,
    ) -> AccessMutualTlsHostnameSettingsResult {
        let result =
            crate::bindings::pulumi::cloudflare::access_mutual_tls_hostname_settings::invoke(
                name,
                &crate::bindings::pulumi::cloudflare::access_mutual_tls_hostname_settings::Args {
                    account_id: args.account_id.get_inner(),
                    settings: args.settings.get_inner(),
                    zone_id: args.zone_id.get_inner(),
                },
            );

        AccessMutualTlsHostnameSettingsResult {
            account_id: crate::into_domain(result.account_id),
            settings: crate::into_domain(result.settings),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod access_organization {

    pub struct AccessOrganizationArgs {
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        pub allow_authenticate_via_warp: pulumi_wasm_rust::Output<Option<bool>>,
        pub auth_domain: pulumi_wasm_rust::Output<String>,
        pub auto_redirect_to_identity: pulumi_wasm_rust::Output<Option<bool>>,
        pub custom_pages:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessOrganizationCustomPage>>>,
        pub is_ui_read_only: pulumi_wasm_rust::Output<Option<bool>>,
        pub login_designs:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessOrganizationLoginDesign>>>,
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        pub session_duration: pulumi_wasm_rust::Output<Option<String>>,
        pub ui_read_only_toggle_reason: pulumi_wasm_rust::Output<Option<String>>,
        pub user_seat_expiration_inactive_time: pulumi_wasm_rust::Output<Option<String>>,
        pub warp_auth_session_duration: pulumi_wasm_rust::Output<Option<String>>,
        pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct AccessOrganizationResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub allow_authenticate_via_warp: pulumi_wasm_rust::Output<Option<bool>>,
        pub auth_domain: pulumi_wasm_rust::Output<String>,
        pub auto_redirect_to_identity: pulumi_wasm_rust::Output<Option<bool>>,
        pub custom_pages:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessOrganizationCustomPage>>>,
        pub is_ui_read_only: pulumi_wasm_rust::Output<Option<bool>>,
        pub login_designs:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessOrganizationLoginDesign>>>,
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        pub session_duration: pulumi_wasm_rust::Output<Option<String>>,
        pub ui_read_only_toggle_reason: pulumi_wasm_rust::Output<Option<String>>,
        pub user_seat_expiration_inactive_time: pulumi_wasm_rust::Output<Option<String>>,
        pub warp_auth_session_duration: pulumi_wasm_rust::Output<Option<String>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn access_organization(
        name: &str,
        args: AccessOrganizationArgs,
    ) -> AccessOrganizationResult {
        let result = crate::bindings::pulumi::cloudflare::access_organization::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::access_organization::Args {
                account_id: args.account_id.get_inner(),
                allow_authenticate_via_warp: args.allow_authenticate_via_warp.get_inner(),
                auth_domain: args.auth_domain.get_inner(),
                auto_redirect_to_identity: args.auto_redirect_to_identity.get_inner(),
                custom_pages: args.custom_pages.get_inner(),
                is_ui_read_only: args.is_ui_read_only.get_inner(),
                login_designs: args.login_designs.get_inner(),
                name: args.name.get_inner(),
                session_duration: args.session_duration.get_inner(),
                ui_read_only_toggle_reason: args.ui_read_only_toggle_reason.get_inner(),
                user_seat_expiration_inactive_time: args
                    .user_seat_expiration_inactive_time
                    .get_inner(),
                warp_auth_session_duration: args.warp_auth_session_duration.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        AccessOrganizationResult {
            account_id: crate::into_domain(result.account_id),
            allow_authenticate_via_warp: crate::into_domain(result.allow_authenticate_via_warp),
            auth_domain: crate::into_domain(result.auth_domain),
            auto_redirect_to_identity: crate::into_domain(result.auto_redirect_to_identity),
            custom_pages: crate::into_domain(result.custom_pages),
            is_ui_read_only: crate::into_domain(result.is_ui_read_only),
            login_designs: crate::into_domain(result.login_designs),
            name: crate::into_domain(result.name),
            session_duration: crate::into_domain(result.session_duration),
            ui_read_only_toggle_reason: crate::into_domain(result.ui_read_only_toggle_reason),
            user_seat_expiration_inactive_time: crate::into_domain(
                result.user_seat_expiration_inactive_time,
            ),
            warp_auth_session_duration: crate::into_domain(result.warp_auth_session_duration),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod access_policy {

    pub struct AccessPolicyArgs {
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        pub application_id: pulumi_wasm_rust::Output<String>,
        pub approval_groups:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessPolicyApprovalGroup>>>,
        pub approval_required: pulumi_wasm_rust::Output<Option<bool>>,
        pub decision: pulumi_wasm_rust::Output<String>,
        pub excludes: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessPolicyExclude>>>,
        pub includes: pulumi_wasm_rust::Output<Vec<crate::types::AccessPolicyInclude>>,
        pub isolation_required: pulumi_wasm_rust::Output<Option<bool>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub precedence: pulumi_wasm_rust::Output<i32>,
        pub purpose_justification_prompt: pulumi_wasm_rust::Output<Option<String>>,
        pub purpose_justification_required: pulumi_wasm_rust::Output<Option<bool>>,
        pub requires: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessPolicyRequire>>>,
        pub session_duration: pulumi_wasm_rust::Output<Option<String>>,
        pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct AccessPolicyResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub application_id: pulumi_wasm_rust::Output<String>,
        pub approval_groups:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessPolicyApprovalGroup>>>,
        pub approval_required: pulumi_wasm_rust::Output<Option<bool>>,
        pub decision: pulumi_wasm_rust::Output<String>,
        pub excludes: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessPolicyExclude>>>,
        pub includes: pulumi_wasm_rust::Output<Vec<crate::types::AccessPolicyInclude>>,
        pub isolation_required: pulumi_wasm_rust::Output<Option<bool>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub precedence: pulumi_wasm_rust::Output<i32>,
        pub purpose_justification_prompt: pulumi_wasm_rust::Output<Option<String>>,
        pub purpose_justification_required: pulumi_wasm_rust::Output<Option<bool>>,
        pub requires: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessPolicyRequire>>>,
        pub session_duration: pulumi_wasm_rust::Output<Option<String>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn access_policy(name: &str, args: AccessPolicyArgs) -> AccessPolicyResult {
        let result = crate::bindings::pulumi::cloudflare::access_policy::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::access_policy::Args {
                account_id: args.account_id.get_inner(),
                application_id: args.application_id.get_inner(),
                approval_groups: args.approval_groups.get_inner(),
                approval_required: args.approval_required.get_inner(),
                decision: args.decision.get_inner(),
                excludes: args.excludes.get_inner(),
                includes: args.includes.get_inner(),
                isolation_required: args.isolation_required.get_inner(),
                name: args.name.get_inner(),
                precedence: args.precedence.get_inner(),
                purpose_justification_prompt: args.purpose_justification_prompt.get_inner(),
                purpose_justification_required: args.purpose_justification_required.get_inner(),
                requires: args.requires.get_inner(),
                session_duration: args.session_duration.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        AccessPolicyResult {
            account_id: crate::into_domain(result.account_id),
            application_id: crate::into_domain(result.application_id),
            approval_groups: crate::into_domain(result.approval_groups),
            approval_required: crate::into_domain(result.approval_required),
            decision: crate::into_domain(result.decision),
            excludes: crate::into_domain(result.excludes),
            includes: crate::into_domain(result.includes),
            isolation_required: crate::into_domain(result.isolation_required),
            name: crate::into_domain(result.name),
            precedence: crate::into_domain(result.precedence),
            purpose_justification_prompt: crate::into_domain(result.purpose_justification_prompt),
            purpose_justification_required: crate::into_domain(
                result.purpose_justification_required,
            ),
            requires: crate::into_domain(result.requires),
            session_duration: crate::into_domain(result.session_duration),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod access_rule {

    pub struct AccessRuleArgs {
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        pub configuration: pulumi_wasm_rust::Output<crate::types::AccessRuleConfiguration>,
        pub mode: pulumi_wasm_rust::Output<String>,
        pub notes: pulumi_wasm_rust::Output<Option<String>>,
        pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct AccessRuleResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub configuration: pulumi_wasm_rust::Output<crate::types::AccessRuleConfiguration>,
        pub mode: pulumi_wasm_rust::Output<String>,
        pub notes: pulumi_wasm_rust::Output<Option<String>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn access_rule(name: &str, args: AccessRuleArgs) -> AccessRuleResult {
        let result = crate::bindings::pulumi::cloudflare::access_rule::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::access_rule::Args {
                account_id: args.account_id.get_inner(),
                configuration: args.configuration.get_inner(),
                mode: args.mode.get_inner(),
                notes: args.notes.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        AccessRuleResult {
            account_id: crate::into_domain(result.account_id),
            configuration: crate::into_domain(result.configuration),
            mode: crate::into_domain(result.mode),
            notes: crate::into_domain(result.notes),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod access_service_token {

    pub struct AccessServiceTokenArgs {
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        pub duration: pulumi_wasm_rust::Output<Option<String>>,
        pub min_days_for_renewal: pulumi_wasm_rust::Output<Option<i32>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct AccessServiceTokenResult {
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        pub client_id: pulumi_wasm_rust::Output<String>,
        pub client_secret: pulumi_wasm_rust::Output<String>,
        pub duration: pulumi_wasm_rust::Output<String>,
        pub expires_at: pulumi_wasm_rust::Output<String>,
        pub min_days_for_renewal: pulumi_wasm_rust::Output<Option<i32>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub fn access_service_token(
        name: &str,
        args: AccessServiceTokenArgs,
    ) -> AccessServiceTokenResult {
        let result = crate::bindings::pulumi::cloudflare::access_service_token::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::access_service_token::Args {
                account_id: args.account_id.get_inner(),
                duration: args.duration.get_inner(),
                min_days_for_renewal: args.min_days_for_renewal.get_inner(),
                name: args.name.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        AccessServiceTokenResult {
            account_id: crate::into_domain(result.account_id),
            client_id: crate::into_domain(result.client_id),
            client_secret: crate::into_domain(result.client_secret),
            duration: crate::into_domain(result.duration),
            expires_at: crate::into_domain(result.expires_at),
            min_days_for_renewal: crate::into_domain(result.min_days_for_renewal),
            name: crate::into_domain(result.name),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod access_tag {

    pub struct AccessTagArgs {
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        pub app_count: pulumi_wasm_rust::Output<Option<i32>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct AccessTagResult {
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        pub app_count: pulumi_wasm_rust::Output<i32>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub fn access_tag(name: &str, args: AccessTagArgs) -> AccessTagResult {
        let result = crate::bindings::pulumi::cloudflare::access_tag::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::access_tag::Args {
                account_id: args.account_id.get_inner(),
                app_count: args.app_count.get_inner(),
                name: args.name.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        AccessTagResult {
            account_id: crate::into_domain(result.account_id),
            app_count: crate::into_domain(result.app_count),
            name: crate::into_domain(result.name),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod account {

    pub struct AccountArgs {
        pub enforce_twofactor: pulumi_wasm_rust::Output<Option<bool>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct AccountResult {
        pub enforce_twofactor: pulumi_wasm_rust::Output<Option<bool>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub fn account(name: &str, args: AccountArgs) -> AccountResult {
        let result = crate::bindings::pulumi::cloudflare::account::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::account::Args {
                enforce_twofactor: args.enforce_twofactor.get_inner(),
                name: args.name.get_inner(),
                type_: args.type_.get_inner(),
            },
        );

        AccountResult {
            enforce_twofactor: crate::into_domain(result.enforce_twofactor),
            name: crate::into_domain(result.name),
            type_: crate::into_domain(result.type_),
        }
    }
}

pub mod account_member {

    pub struct AccountMemberArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub email_address: pulumi_wasm_rust::Output<String>,
        pub role_ids: pulumi_wasm_rust::Output<Vec<String>>,
        pub status: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct AccountMemberResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub email_address: pulumi_wasm_rust::Output<String>,
        pub role_ids: pulumi_wasm_rust::Output<Vec<String>>,
        pub status: pulumi_wasm_rust::Output<String>,
    }

    pub fn account_member(name: &str, args: AccountMemberArgs) -> AccountMemberResult {
        let result = crate::bindings::pulumi::cloudflare::account_member::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::account_member::Args {
                account_id: args.account_id.get_inner(),
                email_address: args.email_address.get_inner(),
                role_ids: args.role_ids.get_inner(),
                status: args.status.get_inner(),
            },
        );

        AccountMemberResult {
            account_id: crate::into_domain(result.account_id),
            email_address: crate::into_domain(result.email_address),
            role_ids: crate::into_domain(result.role_ids),
            status: crate::into_domain(result.status),
        }
    }
}

pub mod address_map {

    pub struct AddressMapArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub default_sni: pulumi_wasm_rust::Output<Option<String>>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub enabled: pulumi_wasm_rust::Output<bool>,
        pub ips: pulumi_wasm_rust::Output<Option<Vec<crate::types::AddressMapIp>>>,
        pub memberships: pulumi_wasm_rust::Output<Option<Vec<crate::types::AddressMapMembership>>>,
    }

    pub struct AddressMapResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub can_delete: pulumi_wasm_rust::Output<bool>,
        pub can_modify_ips: pulumi_wasm_rust::Output<bool>,
        pub default_sni: pulumi_wasm_rust::Output<Option<String>>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub enabled: pulumi_wasm_rust::Output<bool>,
        pub ips: pulumi_wasm_rust::Output<Option<Vec<crate::types::AddressMapIp>>>,
        pub memberships: pulumi_wasm_rust::Output<Option<Vec<crate::types::AddressMapMembership>>>,
    }

    pub fn address_map(name: &str, args: AddressMapArgs) -> AddressMapResult {
        let result = crate::bindings::pulumi::cloudflare::address_map::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::address_map::Args {
                account_id: args.account_id.get_inner(),
                default_sni: args.default_sni.get_inner(),
                description: args.description.get_inner(),
                enabled: args.enabled.get_inner(),
                ips: args.ips.get_inner(),
                memberships: args.memberships.get_inner(),
            },
        );

        AddressMapResult {
            account_id: crate::into_domain(result.account_id),
            can_delete: crate::into_domain(result.can_delete),
            can_modify_ips: crate::into_domain(result.can_modify_ips),
            default_sni: crate::into_domain(result.default_sni),
            description: crate::into_domain(result.description),
            enabled: crate::into_domain(result.enabled),
            ips: crate::into_domain(result.ips),
            memberships: crate::into_domain(result.memberships),
        }
    }
}

pub mod api_shield {

    pub struct ApiShieldArgs {
        pub auth_id_characteristics:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::ApiShieldAuthIdCharacteristic>>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct ApiShieldResult {
        pub auth_id_characteristics:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::ApiShieldAuthIdCharacteristic>>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn api_shield(name: &str, args: ApiShieldArgs) -> ApiShieldResult {
        let result = crate::bindings::pulumi::cloudflare::api_shield::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::api_shield::Args {
                auth_id_characteristics: args.auth_id_characteristics.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        ApiShieldResult {
            auth_id_characteristics: crate::into_domain(result.auth_id_characteristics),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod api_shield_operation {

    pub struct ApiShieldOperationArgs {
        pub endpoint: pulumi_wasm_rust::Output<String>,
        pub host: pulumi_wasm_rust::Output<String>,
        pub method: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct ApiShieldOperationResult {
        pub endpoint: pulumi_wasm_rust::Output<String>,
        pub host: pulumi_wasm_rust::Output<String>,
        pub method: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn api_shield_operation(
        name: &str,
        args: ApiShieldOperationArgs,
    ) -> ApiShieldOperationResult {
        let result = crate::bindings::pulumi::cloudflare::api_shield_operation::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::api_shield_operation::Args {
                endpoint: args.endpoint.get_inner(),
                host: args.host.get_inner(),
                method: args.method.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        ApiShieldOperationResult {
            endpoint: crate::into_domain(result.endpoint),
            host: crate::into_domain(result.host),
            method: crate::into_domain(result.method),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod api_shield_operation_schema_validation_settings {

    pub struct ApiShieldOperationSchemaValidationSettingsArgs {
        pub mitigation_action: pulumi_wasm_rust::Output<Option<String>>,
        pub operation_id: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct ApiShieldOperationSchemaValidationSettingsResult {
        pub mitigation_action: pulumi_wasm_rust::Output<Option<String>>,
        pub operation_id: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn api_shield_operation_schema_validation_settings(
        name: &str,
        args: ApiShieldOperationSchemaValidationSettingsArgs,
    ) -> ApiShieldOperationSchemaValidationSettingsResult {
        let result = crate::bindings::pulumi::cloudflare::api_shield_operation_schema_validation_settings::invoke(name, &crate::bindings::pulumi::cloudflare::api_shield_operation_schema_validation_settings::Args {
            mitigation_action: args.mitigation_action.get_inner(),
            operation_id: args.operation_id.get_inner(),
            zone_id: args.zone_id.get_inner(),
        });

        ApiShieldOperationSchemaValidationSettingsResult {
            mitigation_action: crate::into_domain(result.mitigation_action),
            operation_id: crate::into_domain(result.operation_id),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod api_shield_schema {

    pub struct ApiShieldSchemaArgs {
        pub kind: pulumi_wasm_rust::Output<Option<String>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub source: pulumi_wasm_rust::Output<String>,
        pub validation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct ApiShieldSchemaResult {
        pub kind: pulumi_wasm_rust::Output<Option<String>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub source: pulumi_wasm_rust::Output<String>,
        pub validation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn api_shield_schema(name: &str, args: ApiShieldSchemaArgs) -> ApiShieldSchemaResult {
        let result = crate::bindings::pulumi::cloudflare::api_shield_schema::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::api_shield_schema::Args {
                kind: args.kind.get_inner(),
                name: args.name.get_inner(),
                source: args.source.get_inner(),
                validation_enabled: args.validation_enabled.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        ApiShieldSchemaResult {
            kind: crate::into_domain(result.kind),
            name: crate::into_domain(result.name),
            source: crate::into_domain(result.source),
            validation_enabled: crate::into_domain(result.validation_enabled),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod api_shield_schema_validation_settings {

    pub struct ApiShieldSchemaValidationSettingsArgs {
        pub validation_default_mitigation_action: pulumi_wasm_rust::Output<String>,
        pub validation_override_mitigation_action: pulumi_wasm_rust::Output<Option<String>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct ApiShieldSchemaValidationSettingsResult {
        pub validation_default_mitigation_action: pulumi_wasm_rust::Output<String>,
        pub validation_override_mitigation_action: pulumi_wasm_rust::Output<Option<String>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn api_shield_schema_validation_settings(
        name: &str,
        args: ApiShieldSchemaValidationSettingsArgs,
    ) -> ApiShieldSchemaValidationSettingsResult {
        let result =
            crate::bindings::pulumi::cloudflare::api_shield_schema_validation_settings::invoke(
                name,
                &crate::bindings::pulumi::cloudflare::api_shield_schema_validation_settings::Args {
                    validation_default_mitigation_action: args
                        .validation_default_mitigation_action
                        .get_inner(),
                    validation_override_mitigation_action: args
                        .validation_override_mitigation_action
                        .get_inner(),
                    zone_id: args.zone_id.get_inner(),
                },
            );

        ApiShieldSchemaValidationSettingsResult {
            validation_default_mitigation_action: crate::into_domain(
                result.validation_default_mitigation_action,
            ),
            validation_override_mitigation_action: crate::into_domain(
                result.validation_override_mitigation_action,
            ),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod api_token {

    pub struct ApiTokenArgs {
        pub condition: pulumi_wasm_rust::Output<Option<crate::types::ApiTokenCondition>>,
        pub expires_on: pulumi_wasm_rust::Output<Option<String>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub not_before: pulumi_wasm_rust::Output<Option<String>>,
        pub policies: pulumi_wasm_rust::Output<Vec<crate::types::ApiTokenPolicy>>,
    }

    pub struct ApiTokenResult {
        pub condition: pulumi_wasm_rust::Output<Option<crate::types::ApiTokenCondition>>,
        pub expires_on: pulumi_wasm_rust::Output<Option<String>>,
        pub issued_on: pulumi_wasm_rust::Output<String>,
        pub modified_on: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub not_before: pulumi_wasm_rust::Output<Option<String>>,
        pub policies: pulumi_wasm_rust::Output<Vec<crate::types::ApiTokenPolicy>>,
        pub status: pulumi_wasm_rust::Output<String>,
        pub value: pulumi_wasm_rust::Output<String>,
    }

    pub fn api_token(name: &str, args: ApiTokenArgs) -> ApiTokenResult {
        let result = crate::bindings::pulumi::cloudflare::api_token::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::api_token::Args {
                condition: args.condition.get_inner(),
                expires_on: args.expires_on.get_inner(),
                name: args.name.get_inner(),
                not_before: args.not_before.get_inner(),
                policies: args.policies.get_inner(),
            },
        );

        ApiTokenResult {
            condition: crate::into_domain(result.condition),
            expires_on: crate::into_domain(result.expires_on),
            issued_on: crate::into_domain(result.issued_on),
            modified_on: crate::into_domain(result.modified_on),
            name: crate::into_domain(result.name),
            not_before: crate::into_domain(result.not_before),
            policies: crate::into_domain(result.policies),
            status: crate::into_domain(result.status),
            value: crate::into_domain(result.value),
        }
    }
}

pub mod argo {

    pub struct ArgoArgs {
        pub smart_routing: pulumi_wasm_rust::Output<Option<String>>,
        pub tiered_caching: pulumi_wasm_rust::Output<Option<String>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct ArgoResult {
        pub smart_routing: pulumi_wasm_rust::Output<Option<String>>,
        pub tiered_caching: pulumi_wasm_rust::Output<Option<String>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn argo(name: &str, args: ArgoArgs) -> ArgoResult {
        let result = crate::bindings::pulumi::cloudflare::argo::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::argo::Args {
                smart_routing: args.smart_routing.get_inner(),
                tiered_caching: args.tiered_caching.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        ArgoResult {
            smart_routing: crate::into_domain(result.smart_routing),
            tiered_caching: crate::into_domain(result.tiered_caching),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod authenticated_origin_pulls {

    pub struct AuthenticatedOriginPullsArgs {
        pub authenticated_origin_pulls_certificate: pulumi_wasm_rust::Output<Option<String>>,
        pub enabled: pulumi_wasm_rust::Output<bool>,
        pub hostname: pulumi_wasm_rust::Output<Option<String>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct AuthenticatedOriginPullsResult {
        pub authenticated_origin_pulls_certificate: pulumi_wasm_rust::Output<Option<String>>,
        pub enabled: pulumi_wasm_rust::Output<bool>,
        pub hostname: pulumi_wasm_rust::Output<Option<String>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn authenticated_origin_pulls(
        name: &str,
        args: AuthenticatedOriginPullsArgs,
    ) -> AuthenticatedOriginPullsResult {
        let result = crate::bindings::pulumi::cloudflare::authenticated_origin_pulls::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::authenticated_origin_pulls::Args {
                authenticated_origin_pulls_certificate: args
                    .authenticated_origin_pulls_certificate
                    .get_inner(),
                enabled: args.enabled.get_inner(),
                hostname: args.hostname.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        AuthenticatedOriginPullsResult {
            authenticated_origin_pulls_certificate: crate::into_domain(
                result.authenticated_origin_pulls_certificate,
            ),
            enabled: crate::into_domain(result.enabled),
            hostname: crate::into_domain(result.hostname),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod authenticated_origin_pulls_certificate {

    pub struct AuthenticatedOriginPullsCertificateArgs {
        pub certificate: pulumi_wasm_rust::Output<String>,
        pub private_key: pulumi_wasm_rust::Output<String>,
        pub type_: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct AuthenticatedOriginPullsCertificateResult {
        pub certificate: pulumi_wasm_rust::Output<String>,
        pub expires_on: pulumi_wasm_rust::Output<String>,
        pub issuer: pulumi_wasm_rust::Output<String>,
        pub private_key: pulumi_wasm_rust::Output<String>,
        pub serial_number: pulumi_wasm_rust::Output<String>,
        pub signature: pulumi_wasm_rust::Output<String>,
        pub status: pulumi_wasm_rust::Output<String>,
        pub type_: pulumi_wasm_rust::Output<String>,
        pub uploaded_on: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn authenticated_origin_pulls_certificate(
        name: &str,
        args: AuthenticatedOriginPullsCertificateArgs,
    ) -> AuthenticatedOriginPullsCertificateResult {
        let result = crate::bindings::pulumi::cloudflare::authenticated_origin_pulls_certificate::invoke(name, &crate::bindings::pulumi::cloudflare::authenticated_origin_pulls_certificate::Args {
            certificate: args.certificate.get_inner(),
            private_key: args.private_key.get_inner(),
            type_: args.type_.get_inner(),
            zone_id: args.zone_id.get_inner(),
        });

        AuthenticatedOriginPullsCertificateResult {
            certificate: crate::into_domain(result.certificate),
            expires_on: crate::into_domain(result.expires_on),
            issuer: crate::into_domain(result.issuer),
            private_key: crate::into_domain(result.private_key),
            serial_number: crate::into_domain(result.serial_number),
            signature: crate::into_domain(result.signature),
            status: crate::into_domain(result.status),
            type_: crate::into_domain(result.type_),
            uploaded_on: crate::into_domain(result.uploaded_on),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod bot_management {

    pub struct BotManagementArgs {
        pub auto_update_model: pulumi_wasm_rust::Output<Option<bool>>,
        pub enable_js: pulumi_wasm_rust::Output<Option<bool>>,
        pub fight_mode: pulumi_wasm_rust::Output<Option<bool>>,
        pub optimize_wordpress: pulumi_wasm_rust::Output<Option<bool>>,
        pub sbfm_definitely_automated: pulumi_wasm_rust::Output<Option<String>>,
        pub sbfm_likely_automated: pulumi_wasm_rust::Output<Option<String>>,
        pub sbfm_static_resource_protection: pulumi_wasm_rust::Output<Option<bool>>,
        pub sbfm_verified_bots: pulumi_wasm_rust::Output<Option<String>>,
        pub suppress_session_score: pulumi_wasm_rust::Output<Option<bool>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct BotManagementResult {
        pub auto_update_model: pulumi_wasm_rust::Output<Option<bool>>,
        pub enable_js: pulumi_wasm_rust::Output<Option<bool>>,
        pub fight_mode: pulumi_wasm_rust::Output<Option<bool>>,
        pub optimize_wordpress: pulumi_wasm_rust::Output<Option<bool>>,
        pub sbfm_definitely_automated: pulumi_wasm_rust::Output<Option<String>>,
        pub sbfm_likely_automated: pulumi_wasm_rust::Output<Option<String>>,
        pub sbfm_static_resource_protection: pulumi_wasm_rust::Output<Option<bool>>,
        pub sbfm_verified_bots: pulumi_wasm_rust::Output<Option<String>>,
        pub suppress_session_score: pulumi_wasm_rust::Output<Option<bool>>,
        pub using_latest_model: pulumi_wasm_rust::Output<bool>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn bot_management(name: &str, args: BotManagementArgs) -> BotManagementResult {
        let result = crate::bindings::pulumi::cloudflare::bot_management::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::bot_management::Args {
                auto_update_model: args.auto_update_model.get_inner(),
                enable_js: args.enable_js.get_inner(),
                fight_mode: args.fight_mode.get_inner(),
                optimize_wordpress: args.optimize_wordpress.get_inner(),
                sbfm_definitely_automated: args.sbfm_definitely_automated.get_inner(),
                sbfm_likely_automated: args.sbfm_likely_automated.get_inner(),
                sbfm_static_resource_protection: args.sbfm_static_resource_protection.get_inner(),
                sbfm_verified_bots: args.sbfm_verified_bots.get_inner(),
                suppress_session_score: args.suppress_session_score.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        BotManagementResult {
            auto_update_model: crate::into_domain(result.auto_update_model),
            enable_js: crate::into_domain(result.enable_js),
            fight_mode: crate::into_domain(result.fight_mode),
            optimize_wordpress: crate::into_domain(result.optimize_wordpress),
            sbfm_definitely_automated: crate::into_domain(result.sbfm_definitely_automated),
            sbfm_likely_automated: crate::into_domain(result.sbfm_likely_automated),
            sbfm_static_resource_protection: crate::into_domain(
                result.sbfm_static_resource_protection,
            ),
            sbfm_verified_bots: crate::into_domain(result.sbfm_verified_bots),
            suppress_session_score: crate::into_domain(result.suppress_session_score),
            using_latest_model: crate::into_domain(result.using_latest_model),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod byo_ip_prefix {

    pub struct ByoIpPrefixArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub advertisement: pulumi_wasm_rust::Output<Option<String>>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub prefix_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct ByoIpPrefixResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub advertisement: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<String>,
        pub prefix_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn byo_ip_prefix(name: &str, args: ByoIpPrefixArgs) -> ByoIpPrefixResult {
        let result = crate::bindings::pulumi::cloudflare::byo_ip_prefix::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::byo_ip_prefix::Args {
                account_id: args.account_id.get_inner(),
                advertisement: args.advertisement.get_inner(),
                description: args.description.get_inner(),
                prefix_id: args.prefix_id.get_inner(),
            },
        );

        ByoIpPrefixResult {
            account_id: crate::into_domain(result.account_id),
            advertisement: crate::into_domain(result.advertisement),
            description: crate::into_domain(result.description),
            prefix_id: crate::into_domain(result.prefix_id),
        }
    }
}

pub mod certificate_pack {

    pub struct CertificatePackArgs {
        pub certificate_authority: pulumi_wasm_rust::Output<String>,
        pub cloudflare_branding: pulumi_wasm_rust::Output<Option<bool>>,
        pub hosts: pulumi_wasm_rust::Output<Vec<String>>,
        pub type_: pulumi_wasm_rust::Output<String>,
        pub validation_errors:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::CertificatePackValidationError>>>,
        pub validation_method: pulumi_wasm_rust::Output<String>,
        pub validation_records:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::CertificatePackValidationRecord>>>,
        pub validity_days: pulumi_wasm_rust::Output<i32>,
        pub wait_for_active_status: pulumi_wasm_rust::Output<Option<bool>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct CertificatePackResult {
        pub certificate_authority: pulumi_wasm_rust::Output<String>,
        pub cloudflare_branding: pulumi_wasm_rust::Output<Option<bool>>,
        pub hosts: pulumi_wasm_rust::Output<Vec<String>>,
        pub type_: pulumi_wasm_rust::Output<String>,
        pub validation_errors:
            pulumi_wasm_rust::Output<Vec<crate::types::CertificatePackValidationError>>,
        pub validation_method: pulumi_wasm_rust::Output<String>,
        pub validation_records:
            pulumi_wasm_rust::Output<Vec<crate::types::CertificatePackValidationRecord>>,
        pub validity_days: pulumi_wasm_rust::Output<i32>,
        pub wait_for_active_status: pulumi_wasm_rust::Output<Option<bool>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn certificate_pack(name: &str, args: CertificatePackArgs) -> CertificatePackResult {
        let result = crate::bindings::pulumi::cloudflare::certificate_pack::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::certificate_pack::Args {
                certificate_authority: args.certificate_authority.get_inner(),
                cloudflare_branding: args.cloudflare_branding.get_inner(),
                hosts: args.hosts.get_inner(),
                type_: args.type_.get_inner(),
                validation_errors: args.validation_errors.get_inner(),
                validation_method: args.validation_method.get_inner(),
                validation_records: args.validation_records.get_inner(),
                validity_days: args.validity_days.get_inner(),
                wait_for_active_status: args.wait_for_active_status.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        CertificatePackResult {
            certificate_authority: crate::into_domain(result.certificate_authority),
            cloudflare_branding: crate::into_domain(result.cloudflare_branding),
            hosts: crate::into_domain(result.hosts),
            type_: crate::into_domain(result.type_),
            validation_errors: crate::into_domain(result.validation_errors),
            validation_method: crate::into_domain(result.validation_method),
            validation_records: crate::into_domain(result.validation_records),
            validity_days: crate::into_domain(result.validity_days),
            wait_for_active_status: crate::into_domain(result.wait_for_active_status),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod custom_hostname {

    pub struct CustomHostnameArgs {
        pub custom_metadata:
            pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub custom_origin_server: pulumi_wasm_rust::Output<Option<String>>,
        pub custom_origin_sni: pulumi_wasm_rust::Output<Option<String>>,
        pub hostname: pulumi_wasm_rust::Output<String>,
        pub ssls: pulumi_wasm_rust::Output<Option<Vec<crate::types::CustomHostnameSsl>>>,
        pub wait_for_ssl_pending_validation: pulumi_wasm_rust::Output<Option<bool>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct CustomHostnameResult {
        pub custom_metadata:
            pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub custom_origin_server: pulumi_wasm_rust::Output<Option<String>>,
        pub custom_origin_sni: pulumi_wasm_rust::Output<Option<String>>,
        pub hostname: pulumi_wasm_rust::Output<String>,
        pub ownership_verification:
            pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub ownership_verification_http:
            pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub ssls: pulumi_wasm_rust::Output<Option<Vec<crate::types::CustomHostnameSsl>>>,
        pub status: pulumi_wasm_rust::Output<String>,
        pub wait_for_ssl_pending_validation: pulumi_wasm_rust::Output<Option<bool>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn custom_hostname(name: &str, args: CustomHostnameArgs) -> CustomHostnameResult {
        let result = crate::bindings::pulumi::cloudflare::custom_hostname::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::custom_hostname::Args {
                custom_metadata: args.custom_metadata.get_inner(),
                custom_origin_server: args.custom_origin_server.get_inner(),
                custom_origin_sni: args.custom_origin_sni.get_inner(),
                hostname: args.hostname.get_inner(),
                ssls: args.ssls.get_inner(),
                wait_for_ssl_pending_validation: args.wait_for_ssl_pending_validation.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        CustomHostnameResult {
            custom_metadata: crate::into_domain(result.custom_metadata),
            custom_origin_server: crate::into_domain(result.custom_origin_server),
            custom_origin_sni: crate::into_domain(result.custom_origin_sni),
            hostname: crate::into_domain(result.hostname),
            ownership_verification: crate::into_domain(result.ownership_verification),
            ownership_verification_http: crate::into_domain(result.ownership_verification_http),
            ssls: crate::into_domain(result.ssls),
            status: crate::into_domain(result.status),
            wait_for_ssl_pending_validation: crate::into_domain(
                result.wait_for_ssl_pending_validation,
            ),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod custom_hostname_fallback_origin {

    pub struct CustomHostnameFallbackOriginArgs {
        pub origin: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct CustomHostnameFallbackOriginResult {
        pub origin: pulumi_wasm_rust::Output<String>,
        pub status: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn custom_hostname_fallback_origin(
        name: &str,
        args: CustomHostnameFallbackOriginArgs,
    ) -> CustomHostnameFallbackOriginResult {
        let result = crate::bindings::pulumi::cloudflare::custom_hostname_fallback_origin::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::custom_hostname_fallback_origin::Args {
                origin: args.origin.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        CustomHostnameFallbackOriginResult {
            origin: crate::into_domain(result.origin),
            status: crate::into_domain(result.status),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod custom_pages {

    pub struct CustomPagesArgs {
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        pub state: pulumi_wasm_rust::Output<Option<String>>,
        pub type_: pulumi_wasm_rust::Output<String>,
        pub url: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct CustomPagesResult {
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        pub state: pulumi_wasm_rust::Output<Option<String>>,
        pub type_: pulumi_wasm_rust::Output<String>,
        pub url: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub fn custom_pages(name: &str, args: CustomPagesArgs) -> CustomPagesResult {
        let result = crate::bindings::pulumi::cloudflare::custom_pages::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::custom_pages::Args {
                account_id: args.account_id.get_inner(),
                state: args.state.get_inner(),
                type_: args.type_.get_inner(),
                url: args.url.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        CustomPagesResult {
            account_id: crate::into_domain(result.account_id),
            state: crate::into_domain(result.state),
            type_: crate::into_domain(result.type_),
            url: crate::into_domain(result.url),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod custom_ssl {

    pub struct CustomSslArgs {
        pub custom_ssl_options:
            pulumi_wasm_rust::Output<Option<crate::types::CustomSslCustomSslOptions>>,
        pub custom_ssl_priorities:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::CustomSslCustomSslPriority>>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct CustomSslResult {
        pub custom_ssl_options:
            pulumi_wasm_rust::Output<Option<crate::types::CustomSslCustomSslOptions>>,
        pub custom_ssl_priorities:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::CustomSslCustomSslPriority>>>,
        pub expires_on: pulumi_wasm_rust::Output<String>,
        pub hosts: pulumi_wasm_rust::Output<Vec<String>>,
        pub issuer: pulumi_wasm_rust::Output<String>,
        pub modified_on: pulumi_wasm_rust::Output<String>,
        pub priority: pulumi_wasm_rust::Output<i32>,
        pub signature: pulumi_wasm_rust::Output<String>,
        pub status: pulumi_wasm_rust::Output<String>,
        pub uploaded_on: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn custom_ssl(name: &str, args: CustomSslArgs) -> CustomSslResult {
        let result = crate::bindings::pulumi::cloudflare::custom_ssl::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::custom_ssl::Args {
                custom_ssl_options: args.custom_ssl_options.get_inner(),
                custom_ssl_priorities: args.custom_ssl_priorities.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        CustomSslResult {
            custom_ssl_options: crate::into_domain(result.custom_ssl_options),
            custom_ssl_priorities: crate::into_domain(result.custom_ssl_priorities),
            expires_on: crate::into_domain(result.expires_on),
            hosts: crate::into_domain(result.hosts),
            issuer: crate::into_domain(result.issuer),
            modified_on: crate::into_domain(result.modified_on),
            priority: crate::into_domain(result.priority),
            signature: crate::into_domain(result.signature),
            status: crate::into_domain(result.status),
            uploaded_on: crate::into_domain(result.uploaded_on),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod d1_database {

    pub struct D1DatabaseArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
    }

    pub struct D1DatabaseResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub version: pulumi_wasm_rust::Output<String>,
    }

    pub fn d_1_database(name: &str, args: D1DatabaseArgs) -> D1DatabaseResult {
        let result = crate::bindings::pulumi::cloudflare::d1_database::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::d1_database::Args {
                account_id: args.account_id.get_inner(),
                name: args.name.get_inner(),
            },
        );

        D1DatabaseResult {
            account_id: crate::into_domain(result.account_id),
            name: crate::into_domain(result.name),
            version: crate::into_domain(result.version),
        }
    }
}

pub mod device_dex_test {

    pub struct DeviceDexTestArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub data: pulumi_wasm_rust::Output<crate::types::DeviceDexTestData>,
        pub description: pulumi_wasm_rust::Output<String>,
        pub enabled: pulumi_wasm_rust::Output<bool>,
        pub interval: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
    }

    pub struct DeviceDexTestResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub created: pulumi_wasm_rust::Output<String>,
        pub data: pulumi_wasm_rust::Output<crate::types::DeviceDexTestData>,
        pub description: pulumi_wasm_rust::Output<String>,
        pub enabled: pulumi_wasm_rust::Output<bool>,
        pub interval: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub updated: pulumi_wasm_rust::Output<String>,
    }

    pub fn device_dex_test(name: &str, args: DeviceDexTestArgs) -> DeviceDexTestResult {
        let result = crate::bindings::pulumi::cloudflare::device_dex_test::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::device_dex_test::Args {
                account_id: args.account_id.get_inner(),
                data: args.data.get_inner(),
                description: args.description.get_inner(),
                enabled: args.enabled.get_inner(),
                interval: args.interval.get_inner(),
                name: args.name.get_inner(),
            },
        );

        DeviceDexTestResult {
            account_id: crate::into_domain(result.account_id),
            created: crate::into_domain(result.created),
            data: crate::into_domain(result.data),
            description: crate::into_domain(result.description),
            enabled: crate::into_domain(result.enabled),
            interval: crate::into_domain(result.interval),
            name: crate::into_domain(result.name),
            updated: crate::into_domain(result.updated),
        }
    }
}

pub mod device_managed_networks {

    pub struct DeviceManagedNetworksArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub config: pulumi_wasm_rust::Output<crate::types::DeviceManagedNetworksConfig>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub type_: pulumi_wasm_rust::Output<String>,
    }

    pub struct DeviceManagedNetworksResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub config: pulumi_wasm_rust::Output<crate::types::DeviceManagedNetworksConfig>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub type_: pulumi_wasm_rust::Output<String>,
    }

    pub fn device_managed_networks(
        name: &str,
        args: DeviceManagedNetworksArgs,
    ) -> DeviceManagedNetworksResult {
        let result = crate::bindings::pulumi::cloudflare::device_managed_networks::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::device_managed_networks::Args {
                account_id: args.account_id.get_inner(),
                config: args.config.get_inner(),
                name: args.name.get_inner(),
                type_: args.type_.get_inner(),
            },
        );

        DeviceManagedNetworksResult {
            account_id: crate::into_domain(result.account_id),
            config: crate::into_domain(result.config),
            name: crate::into_domain(result.name),
            type_: crate::into_domain(result.type_),
        }
    }
}

pub mod device_policy_certificates {

    pub struct DevicePolicyCertificatesArgs {
        pub enabled: pulumi_wasm_rust::Output<bool>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct DevicePolicyCertificatesResult {
        pub enabled: pulumi_wasm_rust::Output<bool>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn device_policy_certificates(
        name: &str,
        args: DevicePolicyCertificatesArgs,
    ) -> DevicePolicyCertificatesResult {
        let result = crate::bindings::pulumi::cloudflare::device_policy_certificates::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::device_policy_certificates::Args {
                enabled: args.enabled.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        DevicePolicyCertificatesResult {
            enabled: crate::into_domain(result.enabled),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod device_posture_integration {

    pub struct DevicePostureIntegrationArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub configs:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::DevicePostureIntegrationConfig>>>,
        pub identifier: pulumi_wasm_rust::Output<Option<String>>,
        pub interval: pulumi_wasm_rust::Output<Option<String>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub type_: pulumi_wasm_rust::Output<String>,
    }

    pub struct DevicePostureIntegrationResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub configs:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::DevicePostureIntegrationConfig>>>,
        pub identifier: pulumi_wasm_rust::Output<Option<String>>,
        pub interval: pulumi_wasm_rust::Output<Option<String>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub type_: pulumi_wasm_rust::Output<String>,
    }

    pub fn device_posture_integration(
        name: &str,
        args: DevicePostureIntegrationArgs,
    ) -> DevicePostureIntegrationResult {
        let result = crate::bindings::pulumi::cloudflare::device_posture_integration::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::device_posture_integration::Args {
                account_id: args.account_id.get_inner(),
                configs: args.configs.get_inner(),
                identifier: args.identifier.get_inner(),
                interval: args.interval.get_inner(),
                name: args.name.get_inner(),
                type_: args.type_.get_inner(),
            },
        );

        DevicePostureIntegrationResult {
            account_id: crate::into_domain(result.account_id),
            configs: crate::into_domain(result.configs),
            identifier: crate::into_domain(result.identifier),
            interval: crate::into_domain(result.interval),
            name: crate::into_domain(result.name),
            type_: crate::into_domain(result.type_),
        }
    }
}

pub mod device_posture_rule {

    pub struct DevicePostureRuleArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub expiration: pulumi_wasm_rust::Output<Option<String>>,
        pub inputs: pulumi_wasm_rust::Output<Option<Vec<crate::types::DevicePostureRuleInput>>>,
        pub matches: pulumi_wasm_rust::Output<Option<Vec<crate::types::DevicePostureRuleMatch>>>,
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        pub schedule: pulumi_wasm_rust::Output<Option<String>>,
        pub type_: pulumi_wasm_rust::Output<String>,
    }

    pub struct DevicePostureRuleResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub expiration: pulumi_wasm_rust::Output<Option<String>>,
        pub inputs: pulumi_wasm_rust::Output<Vec<crate::types::DevicePostureRuleInput>>,
        pub matches: pulumi_wasm_rust::Output<Option<Vec<crate::types::DevicePostureRuleMatch>>>,
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        pub schedule: pulumi_wasm_rust::Output<Option<String>>,
        pub type_: pulumi_wasm_rust::Output<String>,
    }

    pub fn device_posture_rule(name: &str, args: DevicePostureRuleArgs) -> DevicePostureRuleResult {
        let result = crate::bindings::pulumi::cloudflare::device_posture_rule::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::device_posture_rule::Args {
                account_id: args.account_id.get_inner(),
                description: args.description.get_inner(),
                expiration: args.expiration.get_inner(),
                inputs: args.inputs.get_inner(),
                matches: args.matches.get_inner(),
                name: args.name.get_inner(),
                schedule: args.schedule.get_inner(),
                type_: args.type_.get_inner(),
            },
        );

        DevicePostureRuleResult {
            account_id: crate::into_domain(result.account_id),
            description: crate::into_domain(result.description),
            expiration: crate::into_domain(result.expiration),
            inputs: crate::into_domain(result.inputs),
            matches: crate::into_domain(result.matches),
            name: crate::into_domain(result.name),
            schedule: crate::into_domain(result.schedule),
            type_: crate::into_domain(result.type_),
        }
    }
}

pub mod device_settings_policy {

    pub struct DeviceSettingsPolicyArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub allow_mode_switch: pulumi_wasm_rust::Output<Option<bool>>,
        pub allow_updates: pulumi_wasm_rust::Output<Option<bool>>,
        pub allowed_to_leave: pulumi_wasm_rust::Output<Option<bool>>,
        pub auto_connect: pulumi_wasm_rust::Output<Option<i32>>,
        pub captive_portal: pulumi_wasm_rust::Output<Option<i32>>,
        pub default: pulumi_wasm_rust::Output<Option<bool>>,
        pub description: pulumi_wasm_rust::Output<String>,
        pub disable_auto_fallback: pulumi_wasm_rust::Output<Option<bool>>,
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        pub exclude_office_ips: pulumi_wasm_rust::Output<Option<bool>>,
        pub match_: pulumi_wasm_rust::Output<Option<String>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub precedence: pulumi_wasm_rust::Output<Option<i32>>,
        pub service_mode_v2_mode: pulumi_wasm_rust::Output<Option<String>>,
        pub service_mode_v2_port: pulumi_wasm_rust::Output<Option<i32>>,
        pub support_url: pulumi_wasm_rust::Output<Option<String>>,
        pub switch_locked: pulumi_wasm_rust::Output<Option<bool>>,
    }

    pub struct DeviceSettingsPolicyResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub allow_mode_switch: pulumi_wasm_rust::Output<Option<bool>>,
        pub allow_updates: pulumi_wasm_rust::Output<Option<bool>>,
        pub allowed_to_leave: pulumi_wasm_rust::Output<Option<bool>>,
        pub auto_connect: pulumi_wasm_rust::Output<Option<i32>>,
        pub captive_portal: pulumi_wasm_rust::Output<Option<i32>>,
        pub default: pulumi_wasm_rust::Output<Option<bool>>,
        pub description: pulumi_wasm_rust::Output<String>,
        pub disable_auto_fallback: pulumi_wasm_rust::Output<Option<bool>>,
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        pub exclude_office_ips: pulumi_wasm_rust::Output<Option<bool>>,
        pub match_: pulumi_wasm_rust::Output<Option<String>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub precedence: pulumi_wasm_rust::Output<Option<i32>>,
        pub service_mode_v2_mode: pulumi_wasm_rust::Output<Option<String>>,
        pub service_mode_v2_port: pulumi_wasm_rust::Output<Option<i32>>,
        pub support_url: pulumi_wasm_rust::Output<Option<String>>,
        pub switch_locked: pulumi_wasm_rust::Output<Option<bool>>,
    }

    pub fn device_settings_policy(
        name: &str,
        args: DeviceSettingsPolicyArgs,
    ) -> DeviceSettingsPolicyResult {
        let result = crate::bindings::pulumi::cloudflare::device_settings_policy::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::device_settings_policy::Args {
                account_id: args.account_id.get_inner(),
                allow_mode_switch: args.allow_mode_switch.get_inner(),
                allow_updates: args.allow_updates.get_inner(),
                allowed_to_leave: args.allowed_to_leave.get_inner(),
                auto_connect: args.auto_connect.get_inner(),
                captive_portal: args.captive_portal.get_inner(),
                default: args.default.get_inner(),
                description: args.description.get_inner(),
                disable_auto_fallback: args.disable_auto_fallback.get_inner(),
                enabled: args.enabled.get_inner(),
                exclude_office_ips: args.exclude_office_ips.get_inner(),
                match_: args.match_.get_inner(),
                name: args.name.get_inner(),
                precedence: args.precedence.get_inner(),
                service_mode_v2_mode: args.service_mode_v2_mode.get_inner(),
                service_mode_v2_port: args.service_mode_v2_port.get_inner(),
                support_url: args.support_url.get_inner(),
                switch_locked: args.switch_locked.get_inner(),
            },
        );

        DeviceSettingsPolicyResult {
            account_id: crate::into_domain(result.account_id),
            allow_mode_switch: crate::into_domain(result.allow_mode_switch),
            allow_updates: crate::into_domain(result.allow_updates),
            allowed_to_leave: crate::into_domain(result.allowed_to_leave),
            auto_connect: crate::into_domain(result.auto_connect),
            captive_portal: crate::into_domain(result.captive_portal),
            default: crate::into_domain(result.default),
            description: crate::into_domain(result.description),
            disable_auto_fallback: crate::into_domain(result.disable_auto_fallback),
            enabled: crate::into_domain(result.enabled),
            exclude_office_ips: crate::into_domain(result.exclude_office_ips),
            match_: crate::into_domain(result.match_),
            name: crate::into_domain(result.name),
            precedence: crate::into_domain(result.precedence),
            service_mode_v2_mode: crate::into_domain(result.service_mode_v2_mode),
            service_mode_v2_port: crate::into_domain(result.service_mode_v2_port),
            support_url: crate::into_domain(result.support_url),
            switch_locked: crate::into_domain(result.switch_locked),
        }
    }
}

pub mod dlp_profile {

    pub struct DlpProfileArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub allowed_match_count: pulumi_wasm_rust::Output<i32>,
        pub context_awareness:
            pulumi_wasm_rust::Output<Option<crate::types::DlpProfileContextAwareness>>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub entries: pulumi_wasm_rust::Output<Vec<crate::types::DlpProfileEntry>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub type_: pulumi_wasm_rust::Output<String>,
    }

    pub struct DlpProfileResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub allowed_match_count: pulumi_wasm_rust::Output<i32>,
        pub context_awareness: pulumi_wasm_rust::Output<crate::types::DlpProfileContextAwareness>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub entries: pulumi_wasm_rust::Output<Vec<crate::types::DlpProfileEntry>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub type_: pulumi_wasm_rust::Output<String>,
    }

    pub fn dlp_profile(name: &str, args: DlpProfileArgs) -> DlpProfileResult {
        let result = crate::bindings::pulumi::cloudflare::dlp_profile::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::dlp_profile::Args {
                account_id: args.account_id.get_inner(),
                allowed_match_count: args.allowed_match_count.get_inner(),
                context_awareness: args.context_awareness.get_inner(),
                description: args.description.get_inner(),
                entries: args.entries.get_inner(),
                name: args.name.get_inner(),
                type_: args.type_.get_inner(),
            },
        );

        DlpProfileResult {
            account_id: crate::into_domain(result.account_id),
            allowed_match_count: crate::into_domain(result.allowed_match_count),
            context_awareness: crate::into_domain(result.context_awareness),
            description: crate::into_domain(result.description),
            entries: crate::into_domain(result.entries),
            name: crate::into_domain(result.name),
            type_: crate::into_domain(result.type_),
        }
    }
}

pub mod email_routing_address {

    pub struct EmailRoutingAddressArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub email: pulumi_wasm_rust::Output<String>,
    }

    pub struct EmailRoutingAddressResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub created: pulumi_wasm_rust::Output<String>,
        pub email: pulumi_wasm_rust::Output<String>,
        pub modified: pulumi_wasm_rust::Output<String>,
        pub tag: pulumi_wasm_rust::Output<String>,
        pub verified: pulumi_wasm_rust::Output<String>,
    }

    pub fn email_routing_address(
        name: &str,
        args: EmailRoutingAddressArgs,
    ) -> EmailRoutingAddressResult {
        let result = crate::bindings::pulumi::cloudflare::email_routing_address::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::email_routing_address::Args {
                account_id: args.account_id.get_inner(),
                email: args.email.get_inner(),
            },
        );

        EmailRoutingAddressResult {
            account_id: crate::into_domain(result.account_id),
            created: crate::into_domain(result.created),
            email: crate::into_domain(result.email),
            modified: crate::into_domain(result.modified),
            tag: crate::into_domain(result.tag),
            verified: crate::into_domain(result.verified),
        }
    }
}

pub mod email_routing_catch_all {

    pub struct EmailRoutingCatchAllArgs {
        pub actions: pulumi_wasm_rust::Output<Vec<crate::types::EmailRoutingCatchAllAction>>,
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        pub matchers: pulumi_wasm_rust::Output<Vec<crate::types::EmailRoutingCatchAllMatcher>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct EmailRoutingCatchAllResult {
        pub actions: pulumi_wasm_rust::Output<Vec<crate::types::EmailRoutingCatchAllAction>>,
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        pub matchers: pulumi_wasm_rust::Output<Vec<crate::types::EmailRoutingCatchAllMatcher>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub tag: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn email_routing_catch_all(
        name: &str,
        args: EmailRoutingCatchAllArgs,
    ) -> EmailRoutingCatchAllResult {
        let result = crate::bindings::pulumi::cloudflare::email_routing_catch_all::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::email_routing_catch_all::Args {
                actions: args.actions.get_inner(),
                enabled: args.enabled.get_inner(),
                matchers: args.matchers.get_inner(),
                name: args.name.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        EmailRoutingCatchAllResult {
            actions: crate::into_domain(result.actions),
            enabled: crate::into_domain(result.enabled),
            matchers: crate::into_domain(result.matchers),
            name: crate::into_domain(result.name),
            tag: crate::into_domain(result.tag),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod email_routing_rule {

    pub struct EmailRoutingRuleArgs {
        pub actions: pulumi_wasm_rust::Output<Option<Vec<crate::types::EmailRoutingRuleAction>>>,
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        pub matchers: pulumi_wasm_rust::Output<Option<Vec<crate::types::EmailRoutingRuleMatcher>>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub priority: pulumi_wasm_rust::Output<Option<i32>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct EmailRoutingRuleResult {
        pub actions: pulumi_wasm_rust::Output<Option<Vec<crate::types::EmailRoutingRuleAction>>>,
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        pub matchers: pulumi_wasm_rust::Output<Option<Vec<crate::types::EmailRoutingRuleMatcher>>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub priority: pulumi_wasm_rust::Output<i32>,
        pub tag: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn email_routing_rule(name: &str, args: EmailRoutingRuleArgs) -> EmailRoutingRuleResult {
        let result = crate::bindings::pulumi::cloudflare::email_routing_rule::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::email_routing_rule::Args {
                actions: args.actions.get_inner(),
                enabled: args.enabled.get_inner(),
                matchers: args.matchers.get_inner(),
                name: args.name.get_inner(),
                priority: args.priority.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        EmailRoutingRuleResult {
            actions: crate::into_domain(result.actions),
            enabled: crate::into_domain(result.enabled),
            matchers: crate::into_domain(result.matchers),
            name: crate::into_domain(result.name),
            priority: crate::into_domain(result.priority),
            tag: crate::into_domain(result.tag),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod email_routing_settings {

    pub struct EmailRoutingSettingsArgs {
        pub enabled: pulumi_wasm_rust::Output<bool>,
        pub skip_wizard: pulumi_wasm_rust::Output<Option<bool>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct EmailRoutingSettingsResult {
        pub created: pulumi_wasm_rust::Output<String>,
        pub enabled: pulumi_wasm_rust::Output<bool>,
        pub modified: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub skip_wizard: pulumi_wasm_rust::Output<bool>,
        pub status: pulumi_wasm_rust::Output<String>,
        pub tag: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn email_routing_settings(
        name: &str,
        args: EmailRoutingSettingsArgs,
    ) -> EmailRoutingSettingsResult {
        let result = crate::bindings::pulumi::cloudflare::email_routing_settings::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::email_routing_settings::Args {
                enabled: args.enabled.get_inner(),
                skip_wizard: args.skip_wizard.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        EmailRoutingSettingsResult {
            created: crate::into_domain(result.created),
            enabled: crate::into_domain(result.enabled),
            modified: crate::into_domain(result.modified),
            name: crate::into_domain(result.name),
            skip_wizard: crate::into_domain(result.skip_wizard),
            status: crate::into_domain(result.status),
            tag: crate::into_domain(result.tag),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod fallback_domain {

    pub struct FallbackDomainArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub domains: pulumi_wasm_rust::Output<Vec<crate::types::FallbackDomainDomain>>,
        pub policy_id: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct FallbackDomainResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub domains: pulumi_wasm_rust::Output<Vec<crate::types::FallbackDomainDomain>>,
        pub policy_id: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub fn fallback_domain(name: &str, args: FallbackDomainArgs) -> FallbackDomainResult {
        let result = crate::bindings::pulumi::cloudflare::fallback_domain::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::fallback_domain::Args {
                account_id: args.account_id.get_inner(),
                domains: args.domains.get_inner(),
                policy_id: args.policy_id.get_inner(),
            },
        );

        FallbackDomainResult {
            account_id: crate::into_domain(result.account_id),
            domains: crate::into_domain(result.domains),
            policy_id: crate::into_domain(result.policy_id),
        }
    }
}

pub mod filter {

    pub struct FilterArgs {
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub expression: pulumi_wasm_rust::Output<String>,
        pub paused: pulumi_wasm_rust::Output<Option<bool>>,
        pub ref_: pulumi_wasm_rust::Output<Option<String>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct FilterResult {
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub expression: pulumi_wasm_rust::Output<String>,
        pub paused: pulumi_wasm_rust::Output<Option<bool>>,
        pub ref_: pulumi_wasm_rust::Output<Option<String>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn filter(name: &str, args: FilterArgs) -> FilterResult {
        let result = crate::bindings::pulumi::cloudflare::filter::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::filter::Args {
                description: args.description.get_inner(),
                expression: args.expression.get_inner(),
                paused: args.paused.get_inner(),
                ref_: args.ref_.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        FilterResult {
            description: crate::into_domain(result.description),
            expression: crate::into_domain(result.expression),
            paused: crate::into_domain(result.paused),
            ref_: crate::into_domain(result.ref_),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod firewall_rule {

    pub struct FirewallRuleArgs {
        pub action: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub filter_id: pulumi_wasm_rust::Output<String>,
        pub paused: pulumi_wasm_rust::Output<Option<bool>>,
        pub priority: pulumi_wasm_rust::Output<Option<i32>>,
        pub products: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct FirewallRuleResult {
        pub action: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub filter_id: pulumi_wasm_rust::Output<String>,
        pub paused: pulumi_wasm_rust::Output<Option<bool>>,
        pub priority: pulumi_wasm_rust::Output<Option<i32>>,
        pub products: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn firewall_rule(name: &str, args: FirewallRuleArgs) -> FirewallRuleResult {
        let result = crate::bindings::pulumi::cloudflare::firewall_rule::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::firewall_rule::Args {
                action: args.action.get_inner(),
                description: args.description.get_inner(),
                filter_id: args.filter_id.get_inner(),
                paused: args.paused.get_inner(),
                priority: args.priority.get_inner(),
                products: args.products.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        FirewallRuleResult {
            action: crate::into_domain(result.action),
            description: crate::into_domain(result.description),
            filter_id: crate::into_domain(result.filter_id),
            paused: crate::into_domain(result.paused),
            priority: crate::into_domain(result.priority),
            products: crate::into_domain(result.products),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod gre_tunnel {

    pub struct GreTunnelArgs {
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        pub cloudflare_gre_endpoint: pulumi_wasm_rust::Output<String>,
        pub customer_gre_endpoint: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub health_check_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        pub health_check_target: pulumi_wasm_rust::Output<Option<String>>,
        pub health_check_type: pulumi_wasm_rust::Output<Option<String>>,
        pub interface_address: pulumi_wasm_rust::Output<String>,
        pub mtu: pulumi_wasm_rust::Output<Option<i32>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub ttl: pulumi_wasm_rust::Output<Option<i32>>,
    }

    pub struct GreTunnelResult {
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        pub cloudflare_gre_endpoint: pulumi_wasm_rust::Output<String>,
        pub customer_gre_endpoint: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub health_check_enabled: pulumi_wasm_rust::Output<bool>,
        pub health_check_target: pulumi_wasm_rust::Output<String>,
        pub health_check_type: pulumi_wasm_rust::Output<String>,
        pub interface_address: pulumi_wasm_rust::Output<String>,
        pub mtu: pulumi_wasm_rust::Output<i32>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub ttl: pulumi_wasm_rust::Output<i32>,
    }

    pub fn gre_tunnel(name: &str, args: GreTunnelArgs) -> GreTunnelResult {
        let result = crate::bindings::pulumi::cloudflare::gre_tunnel::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::gre_tunnel::Args {
                account_id: args.account_id.get_inner(),
                cloudflare_gre_endpoint: args.cloudflare_gre_endpoint.get_inner(),
                customer_gre_endpoint: args.customer_gre_endpoint.get_inner(),
                description: args.description.get_inner(),
                health_check_enabled: args.health_check_enabled.get_inner(),
                health_check_target: args.health_check_target.get_inner(),
                health_check_type: args.health_check_type.get_inner(),
                interface_address: args.interface_address.get_inner(),
                mtu: args.mtu.get_inner(),
                name: args.name.get_inner(),
                ttl: args.ttl.get_inner(),
            },
        );

        GreTunnelResult {
            account_id: crate::into_domain(result.account_id),
            cloudflare_gre_endpoint: crate::into_domain(result.cloudflare_gre_endpoint),
            customer_gre_endpoint: crate::into_domain(result.customer_gre_endpoint),
            description: crate::into_domain(result.description),
            health_check_enabled: crate::into_domain(result.health_check_enabled),
            health_check_target: crate::into_domain(result.health_check_target),
            health_check_type: crate::into_domain(result.health_check_type),
            interface_address: crate::into_domain(result.interface_address),
            mtu: crate::into_domain(result.mtu),
            name: crate::into_domain(result.name),
            ttl: crate::into_domain(result.ttl),
        }
    }
}

pub mod healthcheck {

    pub struct HealthcheckArgs {
        pub address: pulumi_wasm_rust::Output<String>,
        pub allow_insecure: pulumi_wasm_rust::Output<Option<bool>>,
        pub check_regions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub consecutive_fails: pulumi_wasm_rust::Output<Option<i32>>,
        pub consecutive_successes: pulumi_wasm_rust::Output<Option<i32>>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub expected_body: pulumi_wasm_rust::Output<Option<String>>,
        pub expected_codes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub follow_redirects: pulumi_wasm_rust::Output<Option<bool>>,
        pub headers: pulumi_wasm_rust::Output<Option<Vec<crate::types::HealthcheckHeader>>>,
        pub interval: pulumi_wasm_rust::Output<Option<i32>>,
        pub method: pulumi_wasm_rust::Output<Option<String>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub path: pulumi_wasm_rust::Output<Option<String>>,
        pub port: pulumi_wasm_rust::Output<Option<i32>>,
        pub retries: pulumi_wasm_rust::Output<Option<i32>>,
        pub suspended: pulumi_wasm_rust::Output<Option<bool>>,
        pub timeout: pulumi_wasm_rust::Output<Option<i32>>,
        pub type_: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct HealthcheckResult {
        pub address: pulumi_wasm_rust::Output<String>,
        pub allow_insecure: pulumi_wasm_rust::Output<Option<bool>>,
        pub check_regions: pulumi_wasm_rust::Output<Vec<String>>,
        pub consecutive_fails: pulumi_wasm_rust::Output<Option<i32>>,
        pub consecutive_successes: pulumi_wasm_rust::Output<Option<i32>>,
        pub created_on: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub expected_body: pulumi_wasm_rust::Output<Option<String>>,
        pub expected_codes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub follow_redirects: pulumi_wasm_rust::Output<Option<bool>>,
        pub headers: pulumi_wasm_rust::Output<Option<Vec<crate::types::HealthcheckHeader>>>,
        pub interval: pulumi_wasm_rust::Output<Option<i32>>,
        pub method: pulumi_wasm_rust::Output<String>,
        pub modified_on: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub path: pulumi_wasm_rust::Output<Option<String>>,
        pub port: pulumi_wasm_rust::Output<Option<i32>>,
        pub retries: pulumi_wasm_rust::Output<Option<i32>>,
        pub suspended: pulumi_wasm_rust::Output<Option<bool>>,
        pub timeout: pulumi_wasm_rust::Output<Option<i32>>,
        pub type_: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn healthcheck(name: &str, args: HealthcheckArgs) -> HealthcheckResult {
        let result = crate::bindings::pulumi::cloudflare::healthcheck::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::healthcheck::Args {
                address: args.address.get_inner(),
                allow_insecure: args.allow_insecure.get_inner(),
                check_regions: args.check_regions.get_inner(),
                consecutive_fails: args.consecutive_fails.get_inner(),
                consecutive_successes: args.consecutive_successes.get_inner(),
                description: args.description.get_inner(),
                expected_body: args.expected_body.get_inner(),
                expected_codes: args.expected_codes.get_inner(),
                follow_redirects: args.follow_redirects.get_inner(),
                headers: args.headers.get_inner(),
                interval: args.interval.get_inner(),
                method: args.method.get_inner(),
                name: args.name.get_inner(),
                path: args.path.get_inner(),
                port: args.port.get_inner(),
                retries: args.retries.get_inner(),
                suspended: args.suspended.get_inner(),
                timeout: args.timeout.get_inner(),
                type_: args.type_.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        HealthcheckResult {
            address: crate::into_domain(result.address),
            allow_insecure: crate::into_domain(result.allow_insecure),
            check_regions: crate::into_domain(result.check_regions),
            consecutive_fails: crate::into_domain(result.consecutive_fails),
            consecutive_successes: crate::into_domain(result.consecutive_successes),
            created_on: crate::into_domain(result.created_on),
            description: crate::into_domain(result.description),
            expected_body: crate::into_domain(result.expected_body),
            expected_codes: crate::into_domain(result.expected_codes),
            follow_redirects: crate::into_domain(result.follow_redirects),
            headers: crate::into_domain(result.headers),
            interval: crate::into_domain(result.interval),
            method: crate::into_domain(result.method),
            modified_on: crate::into_domain(result.modified_on),
            name: crate::into_domain(result.name),
            path: crate::into_domain(result.path),
            port: crate::into_domain(result.port),
            retries: crate::into_domain(result.retries),
            suspended: crate::into_domain(result.suspended),
            timeout: crate::into_domain(result.timeout),
            type_: crate::into_domain(result.type_),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod hostname_tls_setting {

    pub struct HostnameTlsSettingArgs {
        pub hostname: pulumi_wasm_rust::Output<String>,
        pub setting: pulumi_wasm_rust::Output<String>,
        pub value: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct HostnameTlsSettingResult {
        pub created_at: pulumi_wasm_rust::Output<String>,
        pub hostname: pulumi_wasm_rust::Output<String>,
        pub setting: pulumi_wasm_rust::Output<String>,
        pub updated_at: pulumi_wasm_rust::Output<String>,
        pub value: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn hostname_tls_setting(
        name: &str,
        args: HostnameTlsSettingArgs,
    ) -> HostnameTlsSettingResult {
        let result = crate::bindings::pulumi::cloudflare::hostname_tls_setting::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::hostname_tls_setting::Args {
                hostname: args.hostname.get_inner(),
                setting: args.setting.get_inner(),
                value: args.value.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        HostnameTlsSettingResult {
            created_at: crate::into_domain(result.created_at),
            hostname: crate::into_domain(result.hostname),
            setting: crate::into_domain(result.setting),
            updated_at: crate::into_domain(result.updated_at),
            value: crate::into_domain(result.value),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod hostname_tls_setting_ciphers {

    pub struct HostnameTlsSettingCiphersArgs {
        pub hostname: pulumi_wasm_rust::Output<String>,
        pub ports: pulumi_wasm_rust::Output<Option<Vec<i32>>>,
        pub values: pulumi_wasm_rust::Output<Vec<String>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct HostnameTlsSettingCiphersResult {
        pub created_at: pulumi_wasm_rust::Output<String>,
        pub hostname: pulumi_wasm_rust::Output<String>,
        pub ports: pulumi_wasm_rust::Output<Option<Vec<i32>>>,
        pub updated_at: pulumi_wasm_rust::Output<String>,
        pub values: pulumi_wasm_rust::Output<Vec<String>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn hostname_tls_setting_ciphers(
        name: &str,
        args: HostnameTlsSettingCiphersArgs,
    ) -> HostnameTlsSettingCiphersResult {
        let result = crate::bindings::pulumi::cloudflare::hostname_tls_setting_ciphers::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::hostname_tls_setting_ciphers::Args {
                hostname: args.hostname.get_inner(),
                ports: args.ports.get_inner(),
                values: args.values.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        HostnameTlsSettingCiphersResult {
            created_at: crate::into_domain(result.created_at),
            hostname: crate::into_domain(result.hostname),
            ports: crate::into_domain(result.ports),
            updated_at: crate::into_domain(result.updated_at),
            values: crate::into_domain(result.values),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod hyperdrive_config {

    pub struct HyperdriveConfigArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub caching: pulumi_wasm_rust::Output<Option<crate::types::HyperdriveConfigCaching>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub origin: pulumi_wasm_rust::Output<crate::types::HyperdriveConfigOrigin>,
    }

    pub struct HyperdriveConfigResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub caching: pulumi_wasm_rust::Output<crate::types::HyperdriveConfigCaching>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub origin: pulumi_wasm_rust::Output<crate::types::HyperdriveConfigOrigin>,
    }

    pub fn hyperdrive_config(name: &str, args: HyperdriveConfigArgs) -> HyperdriveConfigResult {
        let result = crate::bindings::pulumi::cloudflare::hyperdrive_config::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::hyperdrive_config::Args {
                account_id: args.account_id.get_inner(),
                caching: args.caching.get_inner(),
                name: args.name.get_inner(),
                origin: args.origin.get_inner(),
            },
        );

        HyperdriveConfigResult {
            account_id: crate::into_domain(result.account_id),
            caching: crate::into_domain(result.caching),
            name: crate::into_domain(result.name),
            origin: crate::into_domain(result.origin),
        }
    }
}

pub mod ipsec_tunnel {

    pub struct IpsecTunnelArgs {
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        pub allow_null_cipher: pulumi_wasm_rust::Output<Option<bool>>,
        pub cloudflare_endpoint: pulumi_wasm_rust::Output<String>,
        pub customer_endpoint: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub fqdn_id: pulumi_wasm_rust::Output<Option<String>>,
        pub health_check_direction: pulumi_wasm_rust::Output<Option<String>>,
        pub health_check_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        pub health_check_rate: pulumi_wasm_rust::Output<Option<String>>,
        pub health_check_target: pulumi_wasm_rust::Output<Option<String>>,
        pub health_check_type: pulumi_wasm_rust::Output<Option<String>>,
        pub hex_id: pulumi_wasm_rust::Output<Option<String>>,
        pub interface_address: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub psk: pulumi_wasm_rust::Output<Option<String>>,
        pub remote_id: pulumi_wasm_rust::Output<Option<String>>,
        pub user_id: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct IpsecTunnelResult {
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        pub allow_null_cipher: pulumi_wasm_rust::Output<Option<bool>>,
        pub cloudflare_endpoint: pulumi_wasm_rust::Output<String>,
        pub customer_endpoint: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub fqdn_id: pulumi_wasm_rust::Output<String>,
        pub health_check_direction: pulumi_wasm_rust::Output<String>,
        pub health_check_enabled: pulumi_wasm_rust::Output<bool>,
        pub health_check_rate: pulumi_wasm_rust::Output<String>,
        pub health_check_target: pulumi_wasm_rust::Output<String>,
        pub health_check_type: pulumi_wasm_rust::Output<String>,
        pub hex_id: pulumi_wasm_rust::Output<String>,
        pub interface_address: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub psk: pulumi_wasm_rust::Output<String>,
        pub remote_id: pulumi_wasm_rust::Output<String>,
        pub user_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn ipsec_tunnel(name: &str, args: IpsecTunnelArgs) -> IpsecTunnelResult {
        let result = crate::bindings::pulumi::cloudflare::ipsec_tunnel::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::ipsec_tunnel::Args {
                account_id: args.account_id.get_inner(),
                allow_null_cipher: args.allow_null_cipher.get_inner(),
                cloudflare_endpoint: args.cloudflare_endpoint.get_inner(),
                customer_endpoint: args.customer_endpoint.get_inner(),
                description: args.description.get_inner(),
                fqdn_id: args.fqdn_id.get_inner(),
                health_check_direction: args.health_check_direction.get_inner(),
                health_check_enabled: args.health_check_enabled.get_inner(),
                health_check_rate: args.health_check_rate.get_inner(),
                health_check_target: args.health_check_target.get_inner(),
                health_check_type: args.health_check_type.get_inner(),
                hex_id: args.hex_id.get_inner(),
                interface_address: args.interface_address.get_inner(),
                name: args.name.get_inner(),
                psk: args.psk.get_inner(),
                remote_id: args.remote_id.get_inner(),
                user_id: args.user_id.get_inner(),
            },
        );

        IpsecTunnelResult {
            account_id: crate::into_domain(result.account_id),
            allow_null_cipher: crate::into_domain(result.allow_null_cipher),
            cloudflare_endpoint: crate::into_domain(result.cloudflare_endpoint),
            customer_endpoint: crate::into_domain(result.customer_endpoint),
            description: crate::into_domain(result.description),
            fqdn_id: crate::into_domain(result.fqdn_id),
            health_check_direction: crate::into_domain(result.health_check_direction),
            health_check_enabled: crate::into_domain(result.health_check_enabled),
            health_check_rate: crate::into_domain(result.health_check_rate),
            health_check_target: crate::into_domain(result.health_check_target),
            health_check_type: crate::into_domain(result.health_check_type),
            hex_id: crate::into_domain(result.hex_id),
            interface_address: crate::into_domain(result.interface_address),
            name: crate::into_domain(result.name),
            psk: crate::into_domain(result.psk),
            remote_id: crate::into_domain(result.remote_id),
            user_id: crate::into_domain(result.user_id),
        }
    }
}

pub mod keyless_certificate {

    pub struct KeylessCertificateArgs {
        pub bundle_method: pulumi_wasm_rust::Output<Option<String>>,
        pub certificate: pulumi_wasm_rust::Output<String>,
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        pub host: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        pub port: pulumi_wasm_rust::Output<Option<i32>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct KeylessCertificateResult {
        pub bundle_method: pulumi_wasm_rust::Output<Option<String>>,
        pub certificate: pulumi_wasm_rust::Output<String>,
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        pub host: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        pub port: pulumi_wasm_rust::Output<Option<i32>>,
        pub status: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn keyless_certificate(
        name: &str,
        args: KeylessCertificateArgs,
    ) -> KeylessCertificateResult {
        let result = crate::bindings::pulumi::cloudflare::keyless_certificate::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::keyless_certificate::Args {
                bundle_method: args.bundle_method.get_inner(),
                certificate: args.certificate.get_inner(),
                enabled: args.enabled.get_inner(),
                host: args.host.get_inner(),
                name: args.name.get_inner(),
                port: args.port.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        KeylessCertificateResult {
            bundle_method: crate::into_domain(result.bundle_method),
            certificate: crate::into_domain(result.certificate),
            enabled: crate::into_domain(result.enabled),
            host: crate::into_domain(result.host),
            name: crate::into_domain(result.name),
            port: crate::into_domain(result.port),
            status: crate::into_domain(result.status),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod list {

    pub struct ListArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub items: pulumi_wasm_rust::Output<Option<Vec<crate::types::ListItem>>>,
        pub kind: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
    }

    pub struct ListResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub items: pulumi_wasm_rust::Output<Option<Vec<crate::types::ListItem>>>,
        pub kind: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
    }

    pub fn list(name: &str, args: ListArgs) -> ListResult {
        let result = crate::bindings::pulumi::cloudflare::list::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::list::Args {
                account_id: args.account_id.get_inner(),
                description: args.description.get_inner(),
                items: args.items.get_inner(),
                kind: args.kind.get_inner(),
                name: args.name.get_inner(),
            },
        );

        ListResult {
            account_id: crate::into_domain(result.account_id),
            description: crate::into_domain(result.description),
            items: crate::into_domain(result.items),
            kind: crate::into_domain(result.kind),
            name: crate::into_domain(result.name),
        }
    }
}

pub mod list_item {

    pub struct ListItemArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub asn: pulumi_wasm_rust::Output<Option<i32>>,
        pub comment: pulumi_wasm_rust::Output<Option<String>>,
        pub hostname: pulumi_wasm_rust::Output<Option<crate::types::ListItemHostname>>,
        pub ip: pulumi_wasm_rust::Output<Option<String>>,
        pub list_id: pulumi_wasm_rust::Output<String>,
        pub redirect: pulumi_wasm_rust::Output<Option<crate::types::ListItemRedirect>>,
    }

    pub struct ListItemResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub asn: pulumi_wasm_rust::Output<Option<i32>>,
        pub comment: pulumi_wasm_rust::Output<Option<String>>,
        pub hostname: pulumi_wasm_rust::Output<Option<crate::types::ListItemHostname>>,
        pub ip: pulumi_wasm_rust::Output<Option<String>>,
        pub list_id: pulumi_wasm_rust::Output<String>,
        pub redirect: pulumi_wasm_rust::Output<Option<crate::types::ListItemRedirect>>,
    }

    pub fn list_item(name: &str, args: ListItemArgs) -> ListItemResult {
        let result = crate::bindings::pulumi::cloudflare::list_item::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::list_item::Args {
                account_id: args.account_id.get_inner(),
                asn: args.asn.get_inner(),
                comment: args.comment.get_inner(),
                hostname: args.hostname.get_inner(),
                ip: args.ip.get_inner(),
                list_id: args.list_id.get_inner(),
                redirect: args.redirect.get_inner(),
            },
        );

        ListItemResult {
            account_id: crate::into_domain(result.account_id),
            asn: crate::into_domain(result.asn),
            comment: crate::into_domain(result.comment),
            hostname: crate::into_domain(result.hostname),
            ip: crate::into_domain(result.ip),
            list_id: crate::into_domain(result.list_id),
            redirect: crate::into_domain(result.redirect),
        }
    }
}

pub mod load_balancer {

    pub struct LoadBalancerArgs {
        pub adaptive_routings:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerAdaptiveRouting>>>,
        pub country_pools:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerCountryPool>>>,
        pub default_pool_ids: pulumi_wasm_rust::Output<Vec<String>>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        pub fallback_pool_id: pulumi_wasm_rust::Output<String>,
        pub location_strategies:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerLocationStrategy>>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub pop_pools: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerPopPool>>>,
        pub proxied: pulumi_wasm_rust::Output<Option<bool>>,
        pub random_steerings:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerRandomSteering>>>,
        pub region_pools:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerRegionPool>>>,
        pub rules: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerRule>>>,
        pub session_affinity: pulumi_wasm_rust::Output<Option<String>>,
        pub session_affinity_attributes: pulumi_wasm_rust::Output<
            Option<Vec<crate::types::LoadBalancerSessionAffinityAttribute>>,
        >,
        pub session_affinity_ttl: pulumi_wasm_rust::Output<Option<i32>>,
        pub steering_policy: pulumi_wasm_rust::Output<Option<String>>,
        pub ttl: pulumi_wasm_rust::Output<Option<i32>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct LoadBalancerResult {
        pub adaptive_routings:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerAdaptiveRouting>>>,
        pub country_pools:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerCountryPool>>>,
        pub created_on: pulumi_wasm_rust::Output<String>,
        pub default_pool_ids: pulumi_wasm_rust::Output<Vec<String>>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        pub fallback_pool_id: pulumi_wasm_rust::Output<String>,
        pub location_strategies:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerLocationStrategy>>>,
        pub modified_on: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub pop_pools: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerPopPool>>>,
        pub proxied: pulumi_wasm_rust::Output<Option<bool>>,
        pub random_steerings:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerRandomSteering>>>,
        pub region_pools:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerRegionPool>>>,
        pub rules: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerRule>>>,
        pub session_affinity: pulumi_wasm_rust::Output<Option<String>>,
        pub session_affinity_attributes: pulumi_wasm_rust::Output<
            Option<Vec<crate::types::LoadBalancerSessionAffinityAttribute>>,
        >,
        pub session_affinity_ttl: pulumi_wasm_rust::Output<Option<i32>>,
        pub steering_policy: pulumi_wasm_rust::Output<String>,
        pub ttl: pulumi_wasm_rust::Output<i32>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn load_balancer(name: &str, args: LoadBalancerArgs) -> LoadBalancerResult {
        let result = crate::bindings::pulumi::cloudflare::load_balancer::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::load_balancer::Args {
                adaptive_routings: args.adaptive_routings.get_inner(),
                country_pools: args.country_pools.get_inner(),
                default_pool_ids: args.default_pool_ids.get_inner(),
                description: args.description.get_inner(),
                enabled: args.enabled.get_inner(),
                fallback_pool_id: args.fallback_pool_id.get_inner(),
                location_strategies: args.location_strategies.get_inner(),
                name: args.name.get_inner(),
                pop_pools: args.pop_pools.get_inner(),
                proxied: args.proxied.get_inner(),
                random_steerings: args.random_steerings.get_inner(),
                region_pools: args.region_pools.get_inner(),
                rules: args.rules.get_inner(),
                session_affinity: args.session_affinity.get_inner(),
                session_affinity_attributes: args.session_affinity_attributes.get_inner(),
                session_affinity_ttl: args.session_affinity_ttl.get_inner(),
                steering_policy: args.steering_policy.get_inner(),
                ttl: args.ttl.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        LoadBalancerResult {
            adaptive_routings: crate::into_domain(result.adaptive_routings),
            country_pools: crate::into_domain(result.country_pools),
            created_on: crate::into_domain(result.created_on),
            default_pool_ids: crate::into_domain(result.default_pool_ids),
            description: crate::into_domain(result.description),
            enabled: crate::into_domain(result.enabled),
            fallback_pool_id: crate::into_domain(result.fallback_pool_id),
            location_strategies: crate::into_domain(result.location_strategies),
            modified_on: crate::into_domain(result.modified_on),
            name: crate::into_domain(result.name),
            pop_pools: crate::into_domain(result.pop_pools),
            proxied: crate::into_domain(result.proxied),
            random_steerings: crate::into_domain(result.random_steerings),
            region_pools: crate::into_domain(result.region_pools),
            rules: crate::into_domain(result.rules),
            session_affinity: crate::into_domain(result.session_affinity),
            session_affinity_attributes: crate::into_domain(result.session_affinity_attributes),
            session_affinity_ttl: crate::into_domain(result.session_affinity_ttl),
            steering_policy: crate::into_domain(result.steering_policy),
            ttl: crate::into_domain(result.ttl),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod load_balancer_monitor {

    pub struct LoadBalancerMonitorArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub allow_insecure: pulumi_wasm_rust::Output<Option<bool>>,
        pub consecutive_down: pulumi_wasm_rust::Output<Option<i32>>,
        pub consecutive_up: pulumi_wasm_rust::Output<Option<i32>>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub expected_body: pulumi_wasm_rust::Output<Option<String>>,
        pub expected_codes: pulumi_wasm_rust::Output<Option<String>>,
        pub follow_redirects: pulumi_wasm_rust::Output<Option<bool>>,
        pub headers: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerMonitorHeader>>>,
        pub interval: pulumi_wasm_rust::Output<Option<i32>>,
        pub method: pulumi_wasm_rust::Output<Option<String>>,
        pub path: pulumi_wasm_rust::Output<Option<String>>,
        pub port: pulumi_wasm_rust::Output<Option<i32>>,
        pub probe_zone: pulumi_wasm_rust::Output<Option<String>>,
        pub retries: pulumi_wasm_rust::Output<Option<i32>>,
        pub timeout: pulumi_wasm_rust::Output<Option<i32>>,
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct LoadBalancerMonitorResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub allow_insecure: pulumi_wasm_rust::Output<Option<bool>>,
        pub consecutive_down: pulumi_wasm_rust::Output<Option<i32>>,
        pub consecutive_up: pulumi_wasm_rust::Output<Option<i32>>,
        pub created_on: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub expected_body: pulumi_wasm_rust::Output<Option<String>>,
        pub expected_codes: pulumi_wasm_rust::Output<Option<String>>,
        pub follow_redirects: pulumi_wasm_rust::Output<Option<bool>>,
        pub headers: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerMonitorHeader>>>,
        pub interval: pulumi_wasm_rust::Output<Option<i32>>,
        pub method: pulumi_wasm_rust::Output<String>,
        pub modified_on: pulumi_wasm_rust::Output<String>,
        pub path: pulumi_wasm_rust::Output<String>,
        pub port: pulumi_wasm_rust::Output<Option<i32>>,
        pub probe_zone: pulumi_wasm_rust::Output<Option<String>>,
        pub retries: pulumi_wasm_rust::Output<Option<i32>>,
        pub timeout: pulumi_wasm_rust::Output<Option<i32>>,
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub fn load_balancer_monitor(
        name: &str,
        args: LoadBalancerMonitorArgs,
    ) -> LoadBalancerMonitorResult {
        let result = crate::bindings::pulumi::cloudflare::load_balancer_monitor::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::load_balancer_monitor::Args {
                account_id: args.account_id.get_inner(),
                allow_insecure: args.allow_insecure.get_inner(),
                consecutive_down: args.consecutive_down.get_inner(),
                consecutive_up: args.consecutive_up.get_inner(),
                description: args.description.get_inner(),
                expected_body: args.expected_body.get_inner(),
                expected_codes: args.expected_codes.get_inner(),
                follow_redirects: args.follow_redirects.get_inner(),
                headers: args.headers.get_inner(),
                interval: args.interval.get_inner(),
                method: args.method.get_inner(),
                path: args.path.get_inner(),
                port: args.port.get_inner(),
                probe_zone: args.probe_zone.get_inner(),
                retries: args.retries.get_inner(),
                timeout: args.timeout.get_inner(),
                type_: args.type_.get_inner(),
            },
        );

        LoadBalancerMonitorResult {
            account_id: crate::into_domain(result.account_id),
            allow_insecure: crate::into_domain(result.allow_insecure),
            consecutive_down: crate::into_domain(result.consecutive_down),
            consecutive_up: crate::into_domain(result.consecutive_up),
            created_on: crate::into_domain(result.created_on),
            description: crate::into_domain(result.description),
            expected_body: crate::into_domain(result.expected_body),
            expected_codes: crate::into_domain(result.expected_codes),
            follow_redirects: crate::into_domain(result.follow_redirects),
            headers: crate::into_domain(result.headers),
            interval: crate::into_domain(result.interval),
            method: crate::into_domain(result.method),
            modified_on: crate::into_domain(result.modified_on),
            path: crate::into_domain(result.path),
            port: crate::into_domain(result.port),
            probe_zone: crate::into_domain(result.probe_zone),
            retries: crate::into_domain(result.retries),
            timeout: crate::into_domain(result.timeout),
            type_: crate::into_domain(result.type_),
        }
    }
}

pub mod load_balancer_pool {

    pub struct LoadBalancerPoolArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub check_regions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        pub latitude: pulumi_wasm_rust::Output<Option<f64>>,
        pub load_sheddings:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerPoolLoadShedding>>>,
        pub longitude: pulumi_wasm_rust::Output<Option<f64>>,
        pub minimum_origins: pulumi_wasm_rust::Output<Option<i32>>,
        pub monitor: pulumi_wasm_rust::Output<Option<String>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub notification_email: pulumi_wasm_rust::Output<Option<String>>,
        pub origin_steerings:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerPoolOriginSteering>>>,
        pub origins: pulumi_wasm_rust::Output<Vec<crate::types::LoadBalancerPoolOrigin>>,
    }

    pub struct LoadBalancerPoolResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub check_regions: pulumi_wasm_rust::Output<Vec<String>>,
        pub created_on: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        pub latitude: pulumi_wasm_rust::Output<Option<f64>>,
        pub load_sheddings:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerPoolLoadShedding>>>,
        pub longitude: pulumi_wasm_rust::Output<Option<f64>>,
        pub minimum_origins: pulumi_wasm_rust::Output<Option<i32>>,
        pub modified_on: pulumi_wasm_rust::Output<String>,
        pub monitor: pulumi_wasm_rust::Output<Option<String>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub notification_email: pulumi_wasm_rust::Output<Option<String>>,
        pub origin_steerings:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerPoolOriginSteering>>>,
        pub origins: pulumi_wasm_rust::Output<Vec<crate::types::LoadBalancerPoolOrigin>>,
    }

    pub fn load_balancer_pool(name: &str, args: LoadBalancerPoolArgs) -> LoadBalancerPoolResult {
        let result = crate::bindings::pulumi::cloudflare::load_balancer_pool::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::load_balancer_pool::Args {
                account_id: args.account_id.get_inner(),
                check_regions: args.check_regions.get_inner(),
                description: args.description.get_inner(),
                enabled: args.enabled.get_inner(),
                latitude: args.latitude.get_inner(),
                load_sheddings: args.load_sheddings.get_inner(),
                longitude: args.longitude.get_inner(),
                minimum_origins: args.minimum_origins.get_inner(),
                monitor: args.monitor.get_inner(),
                name: args.name.get_inner(),
                notification_email: args.notification_email.get_inner(),
                origin_steerings: args.origin_steerings.get_inner(),
                origins: args.origins.get_inner(),
            },
        );

        LoadBalancerPoolResult {
            account_id: crate::into_domain(result.account_id),
            check_regions: crate::into_domain(result.check_regions),
            created_on: crate::into_domain(result.created_on),
            description: crate::into_domain(result.description),
            enabled: crate::into_domain(result.enabled),
            latitude: crate::into_domain(result.latitude),
            load_sheddings: crate::into_domain(result.load_sheddings),
            longitude: crate::into_domain(result.longitude),
            minimum_origins: crate::into_domain(result.minimum_origins),
            modified_on: crate::into_domain(result.modified_on),
            monitor: crate::into_domain(result.monitor),
            name: crate::into_domain(result.name),
            notification_email: crate::into_domain(result.notification_email),
            origin_steerings: crate::into_domain(result.origin_steerings),
            origins: crate::into_domain(result.origins),
        }
    }
}

pub mod logpull_retention {

    pub struct LogpullRetentionArgs {
        pub enabled: pulumi_wasm_rust::Output<bool>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct LogpullRetentionResult {
        pub enabled: pulumi_wasm_rust::Output<bool>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn logpull_retention(name: &str, args: LogpullRetentionArgs) -> LogpullRetentionResult {
        let result = crate::bindings::pulumi::cloudflare::logpull_retention::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::logpull_retention::Args {
                enabled: args.enabled.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        LogpullRetentionResult {
            enabled: crate::into_domain(result.enabled),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod logpush_job {

    pub struct LogpushJobArgs {
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        pub dataset: pulumi_wasm_rust::Output<String>,
        pub destination_conf: pulumi_wasm_rust::Output<String>,
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        pub filter: pulumi_wasm_rust::Output<Option<String>>,
        pub frequency: pulumi_wasm_rust::Output<Option<String>>,
        pub kind: pulumi_wasm_rust::Output<Option<String>>,
        pub logpull_options: pulumi_wasm_rust::Output<Option<String>>,
        pub max_upload_bytes: pulumi_wasm_rust::Output<Option<i32>>,
        pub max_upload_interval_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        pub max_upload_records: pulumi_wasm_rust::Output<Option<i32>>,
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        pub output_options: pulumi_wasm_rust::Output<Option<crate::types::LogpushJobOutputOptions>>,
        pub ownership_challenge: pulumi_wasm_rust::Output<Option<String>>,
        pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct LogpushJobResult {
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        pub dataset: pulumi_wasm_rust::Output<String>,
        pub destination_conf: pulumi_wasm_rust::Output<String>,
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        pub filter: pulumi_wasm_rust::Output<Option<String>>,
        pub frequency: pulumi_wasm_rust::Output<Option<String>>,
        pub kind: pulumi_wasm_rust::Output<Option<String>>,
        pub logpull_options: pulumi_wasm_rust::Output<Option<String>>,
        pub max_upload_bytes: pulumi_wasm_rust::Output<Option<i32>>,
        pub max_upload_interval_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        pub max_upload_records: pulumi_wasm_rust::Output<Option<i32>>,
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        pub output_options: pulumi_wasm_rust::Output<Option<crate::types::LogpushJobOutputOptions>>,
        pub ownership_challenge: pulumi_wasm_rust::Output<Option<String>>,
        pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub fn logpush_job(name: &str, args: LogpushJobArgs) -> LogpushJobResult {
        let result = crate::bindings::pulumi::cloudflare::logpush_job::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::logpush_job::Args {
                account_id: args.account_id.get_inner(),
                dataset: args.dataset.get_inner(),
                destination_conf: args.destination_conf.get_inner(),
                enabled: args.enabled.get_inner(),
                filter: args.filter.get_inner(),
                frequency: args.frequency.get_inner(),
                kind: args.kind.get_inner(),
                logpull_options: args.logpull_options.get_inner(),
                max_upload_bytes: args.max_upload_bytes.get_inner(),
                max_upload_interval_seconds: args.max_upload_interval_seconds.get_inner(),
                max_upload_records: args.max_upload_records.get_inner(),
                name: args.name.get_inner(),
                output_options: args.output_options.get_inner(),
                ownership_challenge: args.ownership_challenge.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        LogpushJobResult {
            account_id: crate::into_domain(result.account_id),
            dataset: crate::into_domain(result.dataset),
            destination_conf: crate::into_domain(result.destination_conf),
            enabled: crate::into_domain(result.enabled),
            filter: crate::into_domain(result.filter),
            frequency: crate::into_domain(result.frequency),
            kind: crate::into_domain(result.kind),
            logpull_options: crate::into_domain(result.logpull_options),
            max_upload_bytes: crate::into_domain(result.max_upload_bytes),
            max_upload_interval_seconds: crate::into_domain(result.max_upload_interval_seconds),
            max_upload_records: crate::into_domain(result.max_upload_records),
            name: crate::into_domain(result.name),
            output_options: crate::into_domain(result.output_options),
            ownership_challenge: crate::into_domain(result.ownership_challenge),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod logpush_ownership_challenge {

    pub struct LogpushOwnershipChallengeArgs {
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        pub destination_conf: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct LogpushOwnershipChallengeResult {
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        pub destination_conf: pulumi_wasm_rust::Output<String>,
        pub ownership_challenge_filename: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub fn logpush_ownership_challenge(
        name: &str,
        args: LogpushOwnershipChallengeArgs,
    ) -> LogpushOwnershipChallengeResult {
        let result = crate::bindings::pulumi::cloudflare::logpush_ownership_challenge::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::logpush_ownership_challenge::Args {
                account_id: args.account_id.get_inner(),
                destination_conf: args.destination_conf.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        LogpushOwnershipChallengeResult {
            account_id: crate::into_domain(result.account_id),
            destination_conf: crate::into_domain(result.destination_conf),
            ownership_challenge_filename: crate::into_domain(result.ownership_challenge_filename),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod magic_firewall_ruleset {

    pub struct MagicFirewallRulesetArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub rules: pulumi_wasm_rust::Output<Option<Vec<std::collections::HashMap<String, String>>>>,
    }

    pub struct MagicFirewallRulesetResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub rules: pulumi_wasm_rust::Output<Option<Vec<std::collections::HashMap<String, String>>>>,
    }

    pub fn magic_firewall_ruleset(
        name: &str,
        args: MagicFirewallRulesetArgs,
    ) -> MagicFirewallRulesetResult {
        let result = crate::bindings::pulumi::cloudflare::magic_firewall_ruleset::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::magic_firewall_ruleset::Args {
                account_id: args.account_id.get_inner(),
                description: args.description.get_inner(),
                name: args.name.get_inner(),
                rules: args.rules.get_inner(),
            },
        );

        MagicFirewallRulesetResult {
            account_id: crate::into_domain(result.account_id),
            description: crate::into_domain(result.description),
            name: crate::into_domain(result.name),
            rules: crate::into_domain(result.rules),
        }
    }
}

pub mod managed_headers {

    pub struct ManagedHeadersArgs {
        pub managed_request_headers:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::ManagedHeadersManagedRequestHeader>>>,
        pub managed_response_headers: pulumi_wasm_rust::Output<
            Option<Vec<crate::types::ManagedHeadersManagedResponseHeader>>,
        >,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct ManagedHeadersResult {
        pub managed_request_headers:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::ManagedHeadersManagedRequestHeader>>>,
        pub managed_response_headers: pulumi_wasm_rust::Output<
            Option<Vec<crate::types::ManagedHeadersManagedResponseHeader>>,
        >,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn managed_headers(name: &str, args: ManagedHeadersArgs) -> ManagedHeadersResult {
        let result = crate::bindings::pulumi::cloudflare::managed_headers::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::managed_headers::Args {
                managed_request_headers: args.managed_request_headers.get_inner(),
                managed_response_headers: args.managed_response_headers.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        ManagedHeadersResult {
            managed_request_headers: crate::into_domain(result.managed_request_headers),
            managed_response_headers: crate::into_domain(result.managed_response_headers),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod mtls_certificate {

    pub struct MtlsCertificateArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub ca: pulumi_wasm_rust::Output<bool>,
        pub certificates: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        pub private_key: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct MtlsCertificateResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub ca: pulumi_wasm_rust::Output<bool>,
        pub certificates: pulumi_wasm_rust::Output<String>,
        pub expires_on: pulumi_wasm_rust::Output<String>,
        pub issuer: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        pub private_key: pulumi_wasm_rust::Output<Option<String>>,
        pub serial_number: pulumi_wasm_rust::Output<String>,
        pub signature: pulumi_wasm_rust::Output<String>,
        pub uploaded_on: pulumi_wasm_rust::Output<String>,
    }

    pub fn mtls_certificate(name: &str, args: MtlsCertificateArgs) -> MtlsCertificateResult {
        let result = crate::bindings::pulumi::cloudflare::mtls_certificate::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::mtls_certificate::Args {
                account_id: args.account_id.get_inner(),
                ca: args.ca.get_inner(),
                certificates: args.certificates.get_inner(),
                name: args.name.get_inner(),
                private_key: args.private_key.get_inner(),
            },
        );

        MtlsCertificateResult {
            account_id: crate::into_domain(result.account_id),
            ca: crate::into_domain(result.ca),
            certificates: crate::into_domain(result.certificates),
            expires_on: crate::into_domain(result.expires_on),
            issuer: crate::into_domain(result.issuer),
            name: crate::into_domain(result.name),
            private_key: crate::into_domain(result.private_key),
            serial_number: crate::into_domain(result.serial_number),
            signature: crate::into_domain(result.signature),
            uploaded_on: crate::into_domain(result.uploaded_on),
        }
    }
}

pub mod notification_policy {

    pub struct NotificationPolicyArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub alert_type: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub email_integrations:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::NotificationPolicyEmailIntegration>>>,
        pub enabled: pulumi_wasm_rust::Output<bool>,
        pub filters: pulumi_wasm_rust::Output<Option<crate::types::NotificationPolicyFilters>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub pagerduty_integrations: pulumi_wasm_rust::Output<
            Option<Vec<crate::types::NotificationPolicyPagerdutyIntegration>>,
        >,
        pub webhooks_integrations: pulumi_wasm_rust::Output<
            Option<Vec<crate::types::NotificationPolicyWebhooksIntegration>>,
        >,
    }

    pub struct NotificationPolicyResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub alert_type: pulumi_wasm_rust::Output<String>,
        pub created: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub email_integrations:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::NotificationPolicyEmailIntegration>>>,
        pub enabled: pulumi_wasm_rust::Output<bool>,
        pub filters: pulumi_wasm_rust::Output<Option<crate::types::NotificationPolicyFilters>>,
        pub modified: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub pagerduty_integrations: pulumi_wasm_rust::Output<
            Option<Vec<crate::types::NotificationPolicyPagerdutyIntegration>>,
        >,
        pub webhooks_integrations: pulumi_wasm_rust::Output<
            Option<Vec<crate::types::NotificationPolicyWebhooksIntegration>>,
        >,
    }

    pub fn notification_policy(
        name: &str,
        args: NotificationPolicyArgs,
    ) -> NotificationPolicyResult {
        let result = crate::bindings::pulumi::cloudflare::notification_policy::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::notification_policy::Args {
                account_id: args.account_id.get_inner(),
                alert_type: args.alert_type.get_inner(),
                description: args.description.get_inner(),
                email_integrations: args.email_integrations.get_inner(),
                enabled: args.enabled.get_inner(),
                filters: args.filters.get_inner(),
                name: args.name.get_inner(),
                pagerduty_integrations: args.pagerduty_integrations.get_inner(),
                webhooks_integrations: args.webhooks_integrations.get_inner(),
            },
        );

        NotificationPolicyResult {
            account_id: crate::into_domain(result.account_id),
            alert_type: crate::into_domain(result.alert_type),
            created: crate::into_domain(result.created),
            description: crate::into_domain(result.description),
            email_integrations: crate::into_domain(result.email_integrations),
            enabled: crate::into_domain(result.enabled),
            filters: crate::into_domain(result.filters),
            modified: crate::into_domain(result.modified),
            name: crate::into_domain(result.name),
            pagerduty_integrations: crate::into_domain(result.pagerduty_integrations),
            webhooks_integrations: crate::into_domain(result.webhooks_integrations),
        }
    }
}

pub mod notification_policy_webhooks {

    pub struct NotificationPolicyWebhooksArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub secret: pulumi_wasm_rust::Output<Option<String>>,
        pub url: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct NotificationPolicyWebhooksResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub created_at: pulumi_wasm_rust::Output<String>,
        pub last_failure: pulumi_wasm_rust::Output<String>,
        pub last_success: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub secret: pulumi_wasm_rust::Output<Option<String>>,
        pub type_: pulumi_wasm_rust::Output<String>,
        pub url: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub fn notification_policy_webhooks(
        name: &str,
        args: NotificationPolicyWebhooksArgs,
    ) -> NotificationPolicyWebhooksResult {
        let result = crate::bindings::pulumi::cloudflare::notification_policy_webhooks::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::notification_policy_webhooks::Args {
                account_id: args.account_id.get_inner(),
                name: args.name.get_inner(),
                secret: args.secret.get_inner(),
                url: args.url.get_inner(),
            },
        );

        NotificationPolicyWebhooksResult {
            account_id: crate::into_domain(result.account_id),
            created_at: crate::into_domain(result.created_at),
            last_failure: crate::into_domain(result.last_failure),
            last_success: crate::into_domain(result.last_success),
            name: crate::into_domain(result.name),
            secret: crate::into_domain(result.secret),
            type_: crate::into_domain(result.type_),
            url: crate::into_domain(result.url),
        }
    }
}

pub mod observatory_scheduled_test {

    pub struct ObservatoryScheduledTestArgs {
        pub frequency: pulumi_wasm_rust::Output<String>,
        pub region: pulumi_wasm_rust::Output<String>,
        pub url: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct ObservatoryScheduledTestResult {
        pub frequency: pulumi_wasm_rust::Output<String>,
        pub region: pulumi_wasm_rust::Output<String>,
        pub url: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn observatory_scheduled_test(
        name: &str,
        args: ObservatoryScheduledTestArgs,
    ) -> ObservatoryScheduledTestResult {
        let result = crate::bindings::pulumi::cloudflare::observatory_scheduled_test::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::observatory_scheduled_test::Args {
                frequency: args.frequency.get_inner(),
                region: args.region.get_inner(),
                url: args.url.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        ObservatoryScheduledTestResult {
            frequency: crate::into_domain(result.frequency),
            region: crate::into_domain(result.region),
            url: crate::into_domain(result.url),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod origin_ca_certificate {

    pub struct OriginCaCertificateArgs {
        pub csr: pulumi_wasm_rust::Output<String>,
        pub hostnames: pulumi_wasm_rust::Output<Vec<String>>,
        pub min_days_for_renewal: pulumi_wasm_rust::Output<Option<i32>>,
        pub request_type: pulumi_wasm_rust::Output<String>,
        pub requested_validity: pulumi_wasm_rust::Output<Option<i32>>,
    }

    pub struct OriginCaCertificateResult {
        pub certificate: pulumi_wasm_rust::Output<String>,
        pub csr: pulumi_wasm_rust::Output<String>,
        pub expires_on: pulumi_wasm_rust::Output<String>,
        pub hostnames: pulumi_wasm_rust::Output<Vec<String>>,
        pub min_days_for_renewal: pulumi_wasm_rust::Output<Option<i32>>,
        pub request_type: pulumi_wasm_rust::Output<String>,
        pub requested_validity: pulumi_wasm_rust::Output<i32>,
    }

    pub fn origin_ca_certificate(
        name: &str,
        args: OriginCaCertificateArgs,
    ) -> OriginCaCertificateResult {
        let result = crate::bindings::pulumi::cloudflare::origin_ca_certificate::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::origin_ca_certificate::Args {
                csr: args.csr.get_inner(),
                hostnames: args.hostnames.get_inner(),
                min_days_for_renewal: args.min_days_for_renewal.get_inner(),
                request_type: args.request_type.get_inner(),
                requested_validity: args.requested_validity.get_inner(),
            },
        );

        OriginCaCertificateResult {
            certificate: crate::into_domain(result.certificate),
            csr: crate::into_domain(result.csr),
            expires_on: crate::into_domain(result.expires_on),
            hostnames: crate::into_domain(result.hostnames),
            min_days_for_renewal: crate::into_domain(result.min_days_for_renewal),
            request_type: crate::into_domain(result.request_type),
            requested_validity: crate::into_domain(result.requested_validity),
        }
    }
}

pub mod page_rule {

    pub struct PageRuleArgs {
        pub actions: pulumi_wasm_rust::Output<crate::types::PageRuleActions>,
        pub priority: pulumi_wasm_rust::Output<Option<i32>>,
        pub status: pulumi_wasm_rust::Output<Option<String>>,
        pub target: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct PageRuleResult {
        pub actions: pulumi_wasm_rust::Output<crate::types::PageRuleActions>,
        pub priority: pulumi_wasm_rust::Output<Option<i32>>,
        pub status: pulumi_wasm_rust::Output<Option<String>>,
        pub target: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn page_rule(name: &str, args: PageRuleArgs) -> PageRuleResult {
        let result = crate::bindings::pulumi::cloudflare::page_rule::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::page_rule::Args {
                actions: args.actions.get_inner(),
                priority: args.priority.get_inner(),
                status: args.status.get_inner(),
                target: args.target.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        PageRuleResult {
            actions: crate::into_domain(result.actions),
            priority: crate::into_domain(result.priority),
            status: crate::into_domain(result.status),
            target: crate::into_domain(result.target),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod pages_domain {

    pub struct PagesDomainArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub domain: pulumi_wasm_rust::Output<String>,
        pub project_name: pulumi_wasm_rust::Output<String>,
    }

    pub struct PagesDomainResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub domain: pulumi_wasm_rust::Output<String>,
        pub project_name: pulumi_wasm_rust::Output<String>,
        pub status: pulumi_wasm_rust::Output<String>,
    }

    pub fn pages_domain(name: &str, args: PagesDomainArgs) -> PagesDomainResult {
        let result = crate::bindings::pulumi::cloudflare::pages_domain::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::pages_domain::Args {
                account_id: args.account_id.get_inner(),
                domain: args.domain.get_inner(),
                project_name: args.project_name.get_inner(),
            },
        );

        PagesDomainResult {
            account_id: crate::into_domain(result.account_id),
            domain: crate::into_domain(result.domain),
            project_name: crate::into_domain(result.project_name),
            status: crate::into_domain(result.status),
        }
    }
}

pub mod pages_project {

    pub struct PagesProjectArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub build_config: pulumi_wasm_rust::Output<Option<crate::types::PagesProjectBuildConfig>>,
        pub deployment_configs:
            pulumi_wasm_rust::Output<Option<crate::types::PagesProjectDeploymentConfigs>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub production_branch: pulumi_wasm_rust::Output<String>,
        pub source: pulumi_wasm_rust::Output<Option<crate::types::PagesProjectSource>>,
    }

    pub struct PagesProjectResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub build_config: pulumi_wasm_rust::Output<Option<crate::types::PagesProjectBuildConfig>>,
        pub created_on: pulumi_wasm_rust::Output<String>,
        pub deployment_configs:
            pulumi_wasm_rust::Output<crate::types::PagesProjectDeploymentConfigs>,
        pub domains: pulumi_wasm_rust::Output<Vec<String>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub production_branch: pulumi_wasm_rust::Output<String>,
        pub source: pulumi_wasm_rust::Output<Option<crate::types::PagesProjectSource>>,
        pub subdomain: pulumi_wasm_rust::Output<String>,
    }

    pub fn pages_project(name: &str, args: PagesProjectArgs) -> PagesProjectResult {
        let result = crate::bindings::pulumi::cloudflare::pages_project::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::pages_project::Args {
                account_id: args.account_id.get_inner(),
                build_config: args.build_config.get_inner(),
                deployment_configs: args.deployment_configs.get_inner(),
                name: args.name.get_inner(),
                production_branch: args.production_branch.get_inner(),
                source: args.source.get_inner(),
            },
        );

        PagesProjectResult {
            account_id: crate::into_domain(result.account_id),
            build_config: crate::into_domain(result.build_config),
            created_on: crate::into_domain(result.created_on),
            deployment_configs: crate::into_domain(result.deployment_configs),
            domains: crate::into_domain(result.domains),
            name: crate::into_domain(result.name),
            production_branch: crate::into_domain(result.production_branch),
            source: crate::into_domain(result.source),
            subdomain: crate::into_domain(result.subdomain),
        }
    }
}

pub mod queue {

    pub struct QueueArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
    }

    pub struct QueueResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
    }

    pub fn queue(name: &str, args: QueueArgs) -> QueueResult {
        let result = crate::bindings::pulumi::cloudflare::queue::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::queue::Args {
                account_id: args.account_id.get_inner(),
                name: args.name.get_inner(),
            },
        );

        QueueResult {
            account_id: crate::into_domain(result.account_id),
            name: crate::into_domain(result.name),
        }
    }
}

pub mod r2_bucket {

    pub struct R2BucketArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        pub name: pulumi_wasm_rust::Output<String>,
    }

    pub struct R2BucketResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
    }

    pub fn r_2_bucket(name: &str, args: R2BucketArgs) -> R2BucketResult {
        let result = crate::bindings::pulumi::cloudflare::r2_bucket::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::r2_bucket::Args {
                account_id: args.account_id.get_inner(),
                location: args.location.get_inner(),
                name: args.name.get_inner(),
            },
        );

        R2BucketResult {
            account_id: crate::into_domain(result.account_id),
            location: crate::into_domain(result.location),
            name: crate::into_domain(result.name),
        }
    }
}

pub mod rate_limit {

    pub struct RateLimitArgs {
        pub action: pulumi_wasm_rust::Output<crate::types::RateLimitAction>,
        pub bypass_url_patterns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub correlate: pulumi_wasm_rust::Output<Option<crate::types::RateLimitCorrelate>>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub disabled: pulumi_wasm_rust::Output<Option<bool>>,
        pub match_: pulumi_wasm_rust::Output<Option<crate::types::RateLimitMatch>>,
        pub period: pulumi_wasm_rust::Output<i32>,
        pub threshold: pulumi_wasm_rust::Output<i32>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct RateLimitResult {
        pub action: pulumi_wasm_rust::Output<crate::types::RateLimitAction>,
        pub bypass_url_patterns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub correlate: pulumi_wasm_rust::Output<Option<crate::types::RateLimitCorrelate>>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub disabled: pulumi_wasm_rust::Output<Option<bool>>,
        pub match_: pulumi_wasm_rust::Output<crate::types::RateLimitMatch>,
        pub period: pulumi_wasm_rust::Output<i32>,
        pub threshold: pulumi_wasm_rust::Output<i32>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn rate_limit(name: &str, args: RateLimitArgs) -> RateLimitResult {
        let result = crate::bindings::pulumi::cloudflare::rate_limit::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::rate_limit::Args {
                action: args.action.get_inner(),
                bypass_url_patterns: args.bypass_url_patterns.get_inner(),
                correlate: args.correlate.get_inner(),
                description: args.description.get_inner(),
                disabled: args.disabled.get_inner(),
                match_: args.match_.get_inner(),
                period: args.period.get_inner(),
                threshold: args.threshold.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        RateLimitResult {
            action: crate::into_domain(result.action),
            bypass_url_patterns: crate::into_domain(result.bypass_url_patterns),
            correlate: crate::into_domain(result.correlate),
            description: crate::into_domain(result.description),
            disabled: crate::into_domain(result.disabled),
            match_: crate::into_domain(result.match_),
            period: crate::into_domain(result.period),
            threshold: crate::into_domain(result.threshold),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod record {

    pub struct RecordArgs {
        pub allow_overwrite: pulumi_wasm_rust::Output<Option<bool>>,
        pub comment: pulumi_wasm_rust::Output<Option<String>>,
        pub data: pulumi_wasm_rust::Output<Option<crate::types::RecordData>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub priority: pulumi_wasm_rust::Output<Option<i32>>,
        pub proxied: pulumi_wasm_rust::Output<Option<bool>>,
        pub tags: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub ttl: pulumi_wasm_rust::Output<Option<i32>>,
        pub type_: pulumi_wasm_rust::Output<String>,
        pub value: pulumi_wasm_rust::Output<Option<String>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct RecordResult {
        pub allow_overwrite: pulumi_wasm_rust::Output<Option<bool>>,
        pub comment: pulumi_wasm_rust::Output<Option<String>>,
        pub created_on: pulumi_wasm_rust::Output<String>,
        pub data: pulumi_wasm_rust::Output<Option<crate::types::RecordData>>,
        pub hostname: pulumi_wasm_rust::Output<String>,
        pub metadata: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub modified_on: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub priority: pulumi_wasm_rust::Output<Option<i32>>,
        pub proxiable: pulumi_wasm_rust::Output<bool>,
        pub proxied: pulumi_wasm_rust::Output<Option<bool>>,
        pub tags: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub ttl: pulumi_wasm_rust::Output<i32>,
        pub type_: pulumi_wasm_rust::Output<String>,
        pub value: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn record(name: &str, args: RecordArgs) -> RecordResult {
        let result = crate::bindings::pulumi::cloudflare::record::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::record::Args {
                allow_overwrite: args.allow_overwrite.get_inner(),
                comment: args.comment.get_inner(),
                data: args.data.get_inner(),
                name: args.name.get_inner(),
                priority: args.priority.get_inner(),
                proxied: args.proxied.get_inner(),
                tags: args.tags.get_inner(),
                ttl: args.ttl.get_inner(),
                type_: args.type_.get_inner(),
                value: args.value.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        RecordResult {
            allow_overwrite: crate::into_domain(result.allow_overwrite),
            comment: crate::into_domain(result.comment),
            created_on: crate::into_domain(result.created_on),
            data: crate::into_domain(result.data),
            hostname: crate::into_domain(result.hostname),
            metadata: crate::into_domain(result.metadata),
            modified_on: crate::into_domain(result.modified_on),
            name: crate::into_domain(result.name),
            priority: crate::into_domain(result.priority),
            proxiable: crate::into_domain(result.proxiable),
            proxied: crate::into_domain(result.proxied),
            tags: crate::into_domain(result.tags),
            ttl: crate::into_domain(result.ttl),
            type_: crate::into_domain(result.type_),
            value: crate::into_domain(result.value),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod regional_hostname {

    pub struct RegionalHostnameArgs {
        pub hostname: pulumi_wasm_rust::Output<String>,
        pub region_key: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct RegionalHostnameResult {
        pub created_on: pulumi_wasm_rust::Output<String>,
        pub hostname: pulumi_wasm_rust::Output<String>,
        pub region_key: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn regional_hostname(name: &str, args: RegionalHostnameArgs) -> RegionalHostnameResult {
        let result = crate::bindings::pulumi::cloudflare::regional_hostname::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::regional_hostname::Args {
                hostname: args.hostname.get_inner(),
                region_key: args.region_key.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        RegionalHostnameResult {
            created_on: crate::into_domain(result.created_on),
            hostname: crate::into_domain(result.hostname),
            region_key: crate::into_domain(result.region_key),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod regional_tiered_cache {

    pub struct RegionalTieredCacheArgs {
        pub value: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct RegionalTieredCacheResult {
        pub value: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn regional_tiered_cache(
        name: &str,
        args: RegionalTieredCacheArgs,
    ) -> RegionalTieredCacheResult {
        let result = crate::bindings::pulumi::cloudflare::regional_tiered_cache::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::regional_tiered_cache::Args {
                value: args.value.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        RegionalTieredCacheResult {
            value: crate::into_domain(result.value),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod ruleset {

    pub struct RulesetArgs {
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub kind: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub phase: pulumi_wasm_rust::Output<String>,
        pub rules: pulumi_wasm_rust::Output<Option<Vec<crate::types::RulesetRule>>>,
        pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct RulesetResult {
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        pub description: pulumi_wasm_rust::Output<String>,
        pub kind: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub phase: pulumi_wasm_rust::Output<String>,
        pub rules: pulumi_wasm_rust::Output<Option<Vec<crate::types::RulesetRule>>>,
        pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub fn ruleset(name: &str, args: RulesetArgs) -> RulesetResult {
        let result = crate::bindings::pulumi::cloudflare::ruleset::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::ruleset::Args {
                account_id: args.account_id.get_inner(),
                description: args.description.get_inner(),
                kind: args.kind.get_inner(),
                name: args.name.get_inner(),
                phase: args.phase.get_inner(),
                rules: args.rules.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        RulesetResult {
            account_id: crate::into_domain(result.account_id),
            description: crate::into_domain(result.description),
            kind: crate::into_domain(result.kind),
            name: crate::into_domain(result.name),
            phase: crate::into_domain(result.phase),
            rules: crate::into_domain(result.rules),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod spectrum_application {

    pub struct SpectrumApplicationArgs {
        pub argo_smart_routing: pulumi_wasm_rust::Output<Option<bool>>,
        pub dns: pulumi_wasm_rust::Output<crate::types::SpectrumApplicationDns>,
        pub edge_ips: pulumi_wasm_rust::Output<Option<crate::types::SpectrumApplicationEdgeIps>>,
        pub ip_firewall: pulumi_wasm_rust::Output<Option<bool>>,
        pub origin_directs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub origin_dns:
            pulumi_wasm_rust::Output<Option<crate::types::SpectrumApplicationOriginDns>>,
        pub origin_port: pulumi_wasm_rust::Output<Option<i32>>,
        pub origin_port_range:
            pulumi_wasm_rust::Output<Option<crate::types::SpectrumApplicationOriginPortRange>>,
        pub protocol: pulumi_wasm_rust::Output<String>,
        pub proxy_protocol: pulumi_wasm_rust::Output<Option<String>>,
        pub tls: pulumi_wasm_rust::Output<Option<String>>,
        pub traffic_type: pulumi_wasm_rust::Output<Option<String>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct SpectrumApplicationResult {
        pub argo_smart_routing: pulumi_wasm_rust::Output<bool>,
        pub dns: pulumi_wasm_rust::Output<crate::types::SpectrumApplicationDns>,
        pub edge_ips: pulumi_wasm_rust::Output<crate::types::SpectrumApplicationEdgeIps>,
        pub ip_firewall: pulumi_wasm_rust::Output<bool>,
        pub origin_directs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub origin_dns:
            pulumi_wasm_rust::Output<Option<crate::types::SpectrumApplicationOriginDns>>,
        pub origin_port: pulumi_wasm_rust::Output<Option<i32>>,
        pub origin_port_range:
            pulumi_wasm_rust::Output<Option<crate::types::SpectrumApplicationOriginPortRange>>,
        pub protocol: pulumi_wasm_rust::Output<String>,
        pub proxy_protocol: pulumi_wasm_rust::Output<String>,
        pub tls: pulumi_wasm_rust::Output<String>,
        pub traffic_type: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn spectrum_application(
        name: &str,
        args: SpectrumApplicationArgs,
    ) -> SpectrumApplicationResult {
        let result = crate::bindings::pulumi::cloudflare::spectrum_application::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::spectrum_application::Args {
                argo_smart_routing: args.argo_smart_routing.get_inner(),
                dns: args.dns.get_inner(),
                edge_ips: args.edge_ips.get_inner(),
                ip_firewall: args.ip_firewall.get_inner(),
                origin_directs: args.origin_directs.get_inner(),
                origin_dns: args.origin_dns.get_inner(),
                origin_port: args.origin_port.get_inner(),
                origin_port_range: args.origin_port_range.get_inner(),
                protocol: args.protocol.get_inner(),
                proxy_protocol: args.proxy_protocol.get_inner(),
                tls: args.tls.get_inner(),
                traffic_type: args.traffic_type.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        SpectrumApplicationResult {
            argo_smart_routing: crate::into_domain(result.argo_smart_routing),
            dns: crate::into_domain(result.dns),
            edge_ips: crate::into_domain(result.edge_ips),
            ip_firewall: crate::into_domain(result.ip_firewall),
            origin_directs: crate::into_domain(result.origin_directs),
            origin_dns: crate::into_domain(result.origin_dns),
            origin_port: crate::into_domain(result.origin_port),
            origin_port_range: crate::into_domain(result.origin_port_range),
            protocol: crate::into_domain(result.protocol),
            proxy_protocol: crate::into_domain(result.proxy_protocol),
            tls: crate::into_domain(result.tls),
            traffic_type: crate::into_domain(result.traffic_type),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod split_tunnel {

    pub struct SplitTunnelArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub mode: pulumi_wasm_rust::Output<String>,
        pub policy_id: pulumi_wasm_rust::Output<Option<String>>,
        pub tunnels: pulumi_wasm_rust::Output<Vec<crate::types::SplitTunnelTunnel>>,
    }

    pub struct SplitTunnelResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub mode: pulumi_wasm_rust::Output<String>,
        pub policy_id: pulumi_wasm_rust::Output<Option<String>>,
        pub tunnels: pulumi_wasm_rust::Output<Vec<crate::types::SplitTunnelTunnel>>,
    }

    pub fn split_tunnel(name: &str, args: SplitTunnelArgs) -> SplitTunnelResult {
        let result = crate::bindings::pulumi::cloudflare::split_tunnel::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::split_tunnel::Args {
                account_id: args.account_id.get_inner(),
                mode: args.mode.get_inner(),
                policy_id: args.policy_id.get_inner(),
                tunnels: args.tunnels.get_inner(),
            },
        );

        SplitTunnelResult {
            account_id: crate::into_domain(result.account_id),
            mode: crate::into_domain(result.mode),
            policy_id: crate::into_domain(result.policy_id),
            tunnels: crate::into_domain(result.tunnels),
        }
    }
}

pub mod static_route {

    pub struct StaticRouteArgs {
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        pub colo_names: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub colo_regions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub nexthop: pulumi_wasm_rust::Output<String>,
        pub prefix: pulumi_wasm_rust::Output<String>,
        pub priority: pulumi_wasm_rust::Output<i32>,
        pub weight: pulumi_wasm_rust::Output<Option<i32>>,
    }

    pub struct StaticRouteResult {
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        pub colo_names: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub colo_regions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub nexthop: pulumi_wasm_rust::Output<String>,
        pub prefix: pulumi_wasm_rust::Output<String>,
        pub priority: pulumi_wasm_rust::Output<i32>,
        pub weight: pulumi_wasm_rust::Output<Option<i32>>,
    }

    pub fn static_route(name: &str, args: StaticRouteArgs) -> StaticRouteResult {
        let result = crate::bindings::pulumi::cloudflare::static_route::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::static_route::Args {
                account_id: args.account_id.get_inner(),
                colo_names: args.colo_names.get_inner(),
                colo_regions: args.colo_regions.get_inner(),
                description: args.description.get_inner(),
                nexthop: args.nexthop.get_inner(),
                prefix: args.prefix.get_inner(),
                priority: args.priority.get_inner(),
                weight: args.weight.get_inner(),
            },
        );

        StaticRouteResult {
            account_id: crate::into_domain(result.account_id),
            colo_names: crate::into_domain(result.colo_names),
            colo_regions: crate::into_domain(result.colo_regions),
            description: crate::into_domain(result.description),
            nexthop: crate::into_domain(result.nexthop),
            prefix: crate::into_domain(result.prefix),
            priority: crate::into_domain(result.priority),
            weight: crate::into_domain(result.weight),
        }
    }
}

pub mod teams_account {

    pub struct TeamsAccountArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub activity_log_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        pub antivirus: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountAntivirus>>,
        pub block_page: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountBlockPage>>,
        pub body_scanning: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountBodyScanning>>,
        pub extended_email_matching:
            pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountExtendedEmailMatching>>,
        pub fips: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountFips>>,
        pub logging: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountLogging>>,
        pub non_identity_browser_isolation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        pub payload_log: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountPayloadLog>>,
        pub protocol_detection_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        pub proxy: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountProxy>>,
        pub ssh_session_log:
            pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountSshSessionLog>>,
        pub tls_decrypt_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        pub url_browser_isolation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    }

    pub struct TeamsAccountResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub activity_log_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        pub antivirus: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountAntivirus>>,
        pub block_page: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountBlockPage>>,
        pub body_scanning: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountBodyScanning>>,
        pub extended_email_matching:
            pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountExtendedEmailMatching>>,
        pub fips: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountFips>>,
        pub logging: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountLogging>>,
        pub non_identity_browser_isolation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        pub payload_log: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountPayloadLog>>,
        pub protocol_detection_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        pub proxy: pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountProxy>>,
        pub ssh_session_log:
            pulumi_wasm_rust::Output<Option<crate::types::TeamsAccountSshSessionLog>>,
        pub tls_decrypt_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        pub url_browser_isolation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    }

    pub fn teams_account(name: &str, args: TeamsAccountArgs) -> TeamsAccountResult {
        let result = crate::bindings::pulumi::cloudflare::teams_account::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::teams_account::Args {
                account_id: args.account_id.get_inner(),
                activity_log_enabled: args.activity_log_enabled.get_inner(),
                antivirus: args.antivirus.get_inner(),
                block_page: args.block_page.get_inner(),
                body_scanning: args.body_scanning.get_inner(),
                extended_email_matching: args.extended_email_matching.get_inner(),
                fips: args.fips.get_inner(),
                logging: args.logging.get_inner(),
                non_identity_browser_isolation_enabled: args
                    .non_identity_browser_isolation_enabled
                    .get_inner(),
                payload_log: args.payload_log.get_inner(),
                protocol_detection_enabled: args.protocol_detection_enabled.get_inner(),
                proxy: args.proxy.get_inner(),
                ssh_session_log: args.ssh_session_log.get_inner(),
                tls_decrypt_enabled: args.tls_decrypt_enabled.get_inner(),
                url_browser_isolation_enabled: args.url_browser_isolation_enabled.get_inner(),
            },
        );

        TeamsAccountResult {
            account_id: crate::into_domain(result.account_id),
            activity_log_enabled: crate::into_domain(result.activity_log_enabled),
            antivirus: crate::into_domain(result.antivirus),
            block_page: crate::into_domain(result.block_page),
            body_scanning: crate::into_domain(result.body_scanning),
            extended_email_matching: crate::into_domain(result.extended_email_matching),
            fips: crate::into_domain(result.fips),
            logging: crate::into_domain(result.logging),
            non_identity_browser_isolation_enabled: crate::into_domain(
                result.non_identity_browser_isolation_enabled,
            ),
            payload_log: crate::into_domain(result.payload_log),
            protocol_detection_enabled: crate::into_domain(result.protocol_detection_enabled),
            proxy: crate::into_domain(result.proxy),
            ssh_session_log: crate::into_domain(result.ssh_session_log),
            tls_decrypt_enabled: crate::into_domain(result.tls_decrypt_enabled),
            url_browser_isolation_enabled: crate::into_domain(result.url_browser_isolation_enabled),
        }
    }
}

pub mod teams_list {

    pub struct TeamsListArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub items: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub type_: pulumi_wasm_rust::Output<String>,
    }

    pub struct TeamsListResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub items: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub type_: pulumi_wasm_rust::Output<String>,
    }

    pub fn teams_list(name: &str, args: TeamsListArgs) -> TeamsListResult {
        let result = crate::bindings::pulumi::cloudflare::teams_list::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::teams_list::Args {
                account_id: args.account_id.get_inner(),
                description: args.description.get_inner(),
                items: args.items.get_inner(),
                name: args.name.get_inner(),
                type_: args.type_.get_inner(),
            },
        );

        TeamsListResult {
            account_id: crate::into_domain(result.account_id),
            description: crate::into_domain(result.description),
            items: crate::into_domain(result.items),
            name: crate::into_domain(result.name),
            type_: crate::into_domain(result.type_),
        }
    }
}

pub mod teams_location {

    pub struct TeamsLocationArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub client_default: pulumi_wasm_rust::Output<Option<bool>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub networks: pulumi_wasm_rust::Output<Option<Vec<crate::types::TeamsLocationNetwork>>>,
    }

    pub struct TeamsLocationResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub anonymized_logs_enabled: pulumi_wasm_rust::Output<bool>,
        pub client_default: pulumi_wasm_rust::Output<Option<bool>>,
        pub doh_subdomain: pulumi_wasm_rust::Output<String>,
        pub ip: pulumi_wasm_rust::Output<String>,
        pub ipv4_destination: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub networks: pulumi_wasm_rust::Output<Option<Vec<crate::types::TeamsLocationNetwork>>>,
        pub policy_ids: pulumi_wasm_rust::Output<Vec<String>>,
    }

    pub fn teams_location(name: &str, args: TeamsLocationArgs) -> TeamsLocationResult {
        let result = crate::bindings::pulumi::cloudflare::teams_location::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::teams_location::Args {
                account_id: args.account_id.get_inner(),
                client_default: args.client_default.get_inner(),
                name: args.name.get_inner(),
                networks: args.networks.get_inner(),
            },
        );

        TeamsLocationResult {
            account_id: crate::into_domain(result.account_id),
            anonymized_logs_enabled: crate::into_domain(result.anonymized_logs_enabled),
            client_default: crate::into_domain(result.client_default),
            doh_subdomain: crate::into_domain(result.doh_subdomain),
            ip: crate::into_domain(result.ip),
            ipv4_destination: crate::into_domain(result.ipv4_destination),
            name: crate::into_domain(result.name),
            networks: crate::into_domain(result.networks),
            policy_ids: crate::into_domain(result.policy_ids),
        }
    }
}

pub mod teams_proxy_endpoint {

    pub struct TeamsProxyEndpointArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub ips: pulumi_wasm_rust::Output<Vec<String>>,
        pub name: pulumi_wasm_rust::Output<String>,
    }

    pub struct TeamsProxyEndpointResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub ips: pulumi_wasm_rust::Output<Vec<String>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub subdomain: pulumi_wasm_rust::Output<String>,
    }

    pub fn teams_proxy_endpoint(
        name: &str,
        args: TeamsProxyEndpointArgs,
    ) -> TeamsProxyEndpointResult {
        let result = crate::bindings::pulumi::cloudflare::teams_proxy_endpoint::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::teams_proxy_endpoint::Args {
                account_id: args.account_id.get_inner(),
                ips: args.ips.get_inner(),
                name: args.name.get_inner(),
            },
        );

        TeamsProxyEndpointResult {
            account_id: crate::into_domain(result.account_id),
            ips: crate::into_domain(result.ips),
            name: crate::into_domain(result.name),
            subdomain: crate::into_domain(result.subdomain),
        }
    }
}

pub mod teams_rule {

    pub struct TeamsRuleArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub action: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<String>,
        pub device_posture: pulumi_wasm_rust::Output<Option<String>>,
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        pub filters: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub identity: pulumi_wasm_rust::Output<Option<String>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub precedence: pulumi_wasm_rust::Output<i32>,
        pub rule_settings: pulumi_wasm_rust::Output<Option<crate::types::TeamsRuleRuleSettings>>,
        pub traffic: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct TeamsRuleResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub action: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<String>,
        pub device_posture: pulumi_wasm_rust::Output<Option<String>>,
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        pub filters: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub identity: pulumi_wasm_rust::Output<Option<String>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub precedence: pulumi_wasm_rust::Output<i32>,
        pub rule_settings: pulumi_wasm_rust::Output<Option<crate::types::TeamsRuleRuleSettings>>,
        pub traffic: pulumi_wasm_rust::Output<Option<String>>,
        pub version: pulumi_wasm_rust::Output<i32>,
    }

    pub fn teams_rule(name: &str, args: TeamsRuleArgs) -> TeamsRuleResult {
        let result = crate::bindings::pulumi::cloudflare::teams_rule::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::teams_rule::Args {
                account_id: args.account_id.get_inner(),
                action: args.action.get_inner(),
                description: args.description.get_inner(),
                device_posture: args.device_posture.get_inner(),
                enabled: args.enabled.get_inner(),
                filters: args.filters.get_inner(),
                identity: args.identity.get_inner(),
                name: args.name.get_inner(),
                precedence: args.precedence.get_inner(),
                rule_settings: args.rule_settings.get_inner(),
                traffic: args.traffic.get_inner(),
            },
        );

        TeamsRuleResult {
            account_id: crate::into_domain(result.account_id),
            action: crate::into_domain(result.action),
            description: crate::into_domain(result.description),
            device_posture: crate::into_domain(result.device_posture),
            enabled: crate::into_domain(result.enabled),
            filters: crate::into_domain(result.filters),
            identity: crate::into_domain(result.identity),
            name: crate::into_domain(result.name),
            precedence: crate::into_domain(result.precedence),
            rule_settings: crate::into_domain(result.rule_settings),
            traffic: crate::into_domain(result.traffic),
            version: crate::into_domain(result.version),
        }
    }
}

pub mod tiered_cache {

    pub struct TieredCacheArgs {
        pub cache_type: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct TieredCacheResult {
        pub cache_type: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn tiered_cache(name: &str, args: TieredCacheArgs) -> TieredCacheResult {
        let result = crate::bindings::pulumi::cloudflare::tiered_cache::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::tiered_cache::Args {
                cache_type: args.cache_type.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        TieredCacheResult {
            cache_type: crate::into_domain(result.cache_type),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod total_tls {

    pub struct TotalTlsArgs {
        pub certificate_authority: pulumi_wasm_rust::Output<Option<String>>,
        pub enabled: pulumi_wasm_rust::Output<bool>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct TotalTlsResult {
        pub certificate_authority: pulumi_wasm_rust::Output<Option<String>>,
        pub enabled: pulumi_wasm_rust::Output<bool>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn total_tls(name: &str, args: TotalTlsArgs) -> TotalTlsResult {
        let result = crate::bindings::pulumi::cloudflare::total_tls::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::total_tls::Args {
                certificate_authority: args.certificate_authority.get_inner(),
                enabled: args.enabled.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        TotalTlsResult {
            certificate_authority: crate::into_domain(result.certificate_authority),
            enabled: crate::into_domain(result.enabled),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod tunnel {

    pub struct TunnelArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub config_src: pulumi_wasm_rust::Output<Option<String>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub secret: pulumi_wasm_rust::Output<String>,
    }

    pub struct TunnelResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub cname: pulumi_wasm_rust::Output<String>,
        pub config_src: pulumi_wasm_rust::Output<Option<String>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub secret: pulumi_wasm_rust::Output<String>,
        pub tunnel_token: pulumi_wasm_rust::Output<String>,
    }

    pub fn tunnel(name: &str, args: TunnelArgs) -> TunnelResult {
        let result = crate::bindings::pulumi::cloudflare::tunnel::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::tunnel::Args {
                account_id: args.account_id.get_inner(),
                config_src: args.config_src.get_inner(),
                name: args.name.get_inner(),
                secret: args.secret.get_inner(),
            },
        );

        TunnelResult {
            account_id: crate::into_domain(result.account_id),
            cname: crate::into_domain(result.cname),
            config_src: crate::into_domain(result.config_src),
            name: crate::into_domain(result.name),
            secret: crate::into_domain(result.secret),
            tunnel_token: crate::into_domain(result.tunnel_token),
        }
    }
}

pub mod tunnel_config {

    pub struct TunnelConfigArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub config: pulumi_wasm_rust::Output<crate::types::TunnelConfigConfig>,
        pub tunnel_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct TunnelConfigResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub config: pulumi_wasm_rust::Output<crate::types::TunnelConfigConfig>,
        pub tunnel_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn tunnel_config(name: &str, args: TunnelConfigArgs) -> TunnelConfigResult {
        let result = crate::bindings::pulumi::cloudflare::tunnel_config::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::tunnel_config::Args {
                account_id: args.account_id.get_inner(),
                config: args.config.get_inner(),
                tunnel_id: args.tunnel_id.get_inner(),
            },
        );

        TunnelConfigResult {
            account_id: crate::into_domain(result.account_id),
            config: crate::into_domain(result.config),
            tunnel_id: crate::into_domain(result.tunnel_id),
        }
    }
}

pub mod tunnel_route {

    pub struct TunnelRouteArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub comment: pulumi_wasm_rust::Output<Option<String>>,
        pub network: pulumi_wasm_rust::Output<String>,
        pub tunnel_id: pulumi_wasm_rust::Output<String>,
        pub virtual_network_id: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct TunnelRouteResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub comment: pulumi_wasm_rust::Output<Option<String>>,
        pub network: pulumi_wasm_rust::Output<String>,
        pub tunnel_id: pulumi_wasm_rust::Output<String>,
        pub virtual_network_id: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub fn tunnel_route(name: &str, args: TunnelRouteArgs) -> TunnelRouteResult {
        let result = crate::bindings::pulumi::cloudflare::tunnel_route::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::tunnel_route::Args {
                account_id: args.account_id.get_inner(),
                comment: args.comment.get_inner(),
                network: args.network.get_inner(),
                tunnel_id: args.tunnel_id.get_inner(),
                virtual_network_id: args.virtual_network_id.get_inner(),
            },
        );

        TunnelRouteResult {
            account_id: crate::into_domain(result.account_id),
            comment: crate::into_domain(result.comment),
            network: crate::into_domain(result.network),
            tunnel_id: crate::into_domain(result.tunnel_id),
            virtual_network_id: crate::into_domain(result.virtual_network_id),
        }
    }
}

pub mod tunnel_virtual_network {

    pub struct TunnelVirtualNetworkArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub comment: pulumi_wasm_rust::Output<Option<String>>,
        pub is_default_network: pulumi_wasm_rust::Output<Option<bool>>,
        pub name: pulumi_wasm_rust::Output<String>,
    }

    pub struct TunnelVirtualNetworkResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub comment: pulumi_wasm_rust::Output<Option<String>>,
        pub is_default_network: pulumi_wasm_rust::Output<Option<bool>>,
        pub name: pulumi_wasm_rust::Output<String>,
    }

    pub fn tunnel_virtual_network(
        name: &str,
        args: TunnelVirtualNetworkArgs,
    ) -> TunnelVirtualNetworkResult {
        let result = crate::bindings::pulumi::cloudflare::tunnel_virtual_network::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::tunnel_virtual_network::Args {
                account_id: args.account_id.get_inner(),
                comment: args.comment.get_inner(),
                is_default_network: args.is_default_network.get_inner(),
                name: args.name.get_inner(),
            },
        );

        TunnelVirtualNetworkResult {
            account_id: crate::into_domain(result.account_id),
            comment: crate::into_domain(result.comment),
            is_default_network: crate::into_domain(result.is_default_network),
            name: crate::into_domain(result.name),
        }
    }
}

pub mod turnstile_widget {

    pub struct TurnstileWidgetArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub bot_fight_mode: pulumi_wasm_rust::Output<Option<bool>>,
        pub domains: pulumi_wasm_rust::Output<Vec<String>>,
        pub mode: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub offlabel: pulumi_wasm_rust::Output<Option<bool>>,
        pub region: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct TurnstileWidgetResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub bot_fight_mode: pulumi_wasm_rust::Output<bool>,
        pub domains: pulumi_wasm_rust::Output<Vec<String>>,
        pub mode: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub offlabel: pulumi_wasm_rust::Output<bool>,
        pub region: pulumi_wasm_rust::Output<String>,
        pub secret: pulumi_wasm_rust::Output<String>,
    }

    pub fn turnstile_widget(name: &str, args: TurnstileWidgetArgs) -> TurnstileWidgetResult {
        let result = crate::bindings::pulumi::cloudflare::turnstile_widget::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::turnstile_widget::Args {
                account_id: args.account_id.get_inner(),
                bot_fight_mode: args.bot_fight_mode.get_inner(),
                domains: args.domains.get_inner(),
                mode: args.mode.get_inner(),
                name: args.name.get_inner(),
                offlabel: args.offlabel.get_inner(),
                region: args.region.get_inner(),
            },
        );

        TurnstileWidgetResult {
            account_id: crate::into_domain(result.account_id),
            bot_fight_mode: crate::into_domain(result.bot_fight_mode),
            domains: crate::into_domain(result.domains),
            mode: crate::into_domain(result.mode),
            name: crate::into_domain(result.name),
            offlabel: crate::into_domain(result.offlabel),
            region: crate::into_domain(result.region),
            secret: crate::into_domain(result.secret),
        }
    }
}

pub mod url_normalization_settings {

    pub struct UrlNormalizationSettingsArgs {
        pub scope: pulumi_wasm_rust::Output<String>,
        pub type_: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct UrlNormalizationSettingsResult {
        pub scope: pulumi_wasm_rust::Output<String>,
        pub type_: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn url_normalization_settings(
        name: &str,
        args: UrlNormalizationSettingsArgs,
    ) -> UrlNormalizationSettingsResult {
        let result = crate::bindings::pulumi::cloudflare::url_normalization_settings::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::url_normalization_settings::Args {
                scope: args.scope.get_inner(),
                type_: args.type_.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        UrlNormalizationSettingsResult {
            scope: crate::into_domain(result.scope),
            type_: crate::into_domain(result.type_),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod user_agent_blocking_rule {

    pub struct UserAgentBlockingRuleArgs {
        pub configuration:
            pulumi_wasm_rust::Output<crate::types::UserAgentBlockingRuleConfiguration>,
        pub description: pulumi_wasm_rust::Output<String>,
        pub mode: pulumi_wasm_rust::Output<String>,
        pub paused: pulumi_wasm_rust::Output<bool>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct UserAgentBlockingRuleResult {
        pub configuration:
            pulumi_wasm_rust::Output<crate::types::UserAgentBlockingRuleConfiguration>,
        pub description: pulumi_wasm_rust::Output<String>,
        pub mode: pulumi_wasm_rust::Output<String>,
        pub paused: pulumi_wasm_rust::Output<bool>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn user_agent_blocking_rule(
        name: &str,
        args: UserAgentBlockingRuleArgs,
    ) -> UserAgentBlockingRuleResult {
        let result = crate::bindings::pulumi::cloudflare::user_agent_blocking_rule::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::user_agent_blocking_rule::Args {
                configuration: args.configuration.get_inner(),
                description: args.description.get_inner(),
                mode: args.mode.get_inner(),
                paused: args.paused.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        UserAgentBlockingRuleResult {
            configuration: crate::into_domain(result.configuration),
            description: crate::into_domain(result.description),
            mode: crate::into_domain(result.mode),
            paused: crate::into_domain(result.paused),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod waiting_room {

    pub struct WaitingRoomArgs {
        pub additional_routes:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::WaitingRoomAdditionalRoute>>>,
        pub cookie_suffix: pulumi_wasm_rust::Output<Option<String>>,
        pub custom_page_html: pulumi_wasm_rust::Output<Option<String>>,
        pub default_template_language: pulumi_wasm_rust::Output<Option<String>>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub disable_session_renewal: pulumi_wasm_rust::Output<Option<bool>>,
        pub host: pulumi_wasm_rust::Output<String>,
        pub json_response_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub new_users_per_minute: pulumi_wasm_rust::Output<i32>,
        pub path: pulumi_wasm_rust::Output<Option<String>>,
        pub queue_all: pulumi_wasm_rust::Output<Option<bool>>,
        pub queueing_method: pulumi_wasm_rust::Output<Option<String>>,
        pub queueing_status_code: pulumi_wasm_rust::Output<Option<i32>>,
        pub session_duration: pulumi_wasm_rust::Output<Option<i32>>,
        pub suspended: pulumi_wasm_rust::Output<Option<bool>>,
        pub total_active_users: pulumi_wasm_rust::Output<i32>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct WaitingRoomResult {
        pub additional_routes:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::WaitingRoomAdditionalRoute>>>,
        pub cookie_suffix: pulumi_wasm_rust::Output<Option<String>>,
        pub custom_page_html: pulumi_wasm_rust::Output<Option<String>>,
        pub default_template_language: pulumi_wasm_rust::Output<Option<String>>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub disable_session_renewal: pulumi_wasm_rust::Output<Option<bool>>,
        pub host: pulumi_wasm_rust::Output<String>,
        pub json_response_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub new_users_per_minute: pulumi_wasm_rust::Output<i32>,
        pub path: pulumi_wasm_rust::Output<Option<String>>,
        pub queue_all: pulumi_wasm_rust::Output<Option<bool>>,
        pub queueing_method: pulumi_wasm_rust::Output<Option<String>>,
        pub queueing_status_code: pulumi_wasm_rust::Output<Option<i32>>,
        pub session_duration: pulumi_wasm_rust::Output<Option<i32>>,
        pub suspended: pulumi_wasm_rust::Output<Option<bool>>,
        pub total_active_users: pulumi_wasm_rust::Output<i32>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn waiting_room(name: &str, args: WaitingRoomArgs) -> WaitingRoomResult {
        let result = crate::bindings::pulumi::cloudflare::waiting_room::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::waiting_room::Args {
                additional_routes: args.additional_routes.get_inner(),
                cookie_suffix: args.cookie_suffix.get_inner(),
                custom_page_html: args.custom_page_html.get_inner(),
                default_template_language: args.default_template_language.get_inner(),
                description: args.description.get_inner(),
                disable_session_renewal: args.disable_session_renewal.get_inner(),
                host: args.host.get_inner(),
                json_response_enabled: args.json_response_enabled.get_inner(),
                name: args.name.get_inner(),
                new_users_per_minute: args.new_users_per_minute.get_inner(),
                path: args.path.get_inner(),
                queue_all: args.queue_all.get_inner(),
                queueing_method: args.queueing_method.get_inner(),
                queueing_status_code: args.queueing_status_code.get_inner(),
                session_duration: args.session_duration.get_inner(),
                suspended: args.suspended.get_inner(),
                total_active_users: args.total_active_users.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        WaitingRoomResult {
            additional_routes: crate::into_domain(result.additional_routes),
            cookie_suffix: crate::into_domain(result.cookie_suffix),
            custom_page_html: crate::into_domain(result.custom_page_html),
            default_template_language: crate::into_domain(result.default_template_language),
            description: crate::into_domain(result.description),
            disable_session_renewal: crate::into_domain(result.disable_session_renewal),
            host: crate::into_domain(result.host),
            json_response_enabled: crate::into_domain(result.json_response_enabled),
            name: crate::into_domain(result.name),
            new_users_per_minute: crate::into_domain(result.new_users_per_minute),
            path: crate::into_domain(result.path),
            queue_all: crate::into_domain(result.queue_all),
            queueing_method: crate::into_domain(result.queueing_method),
            queueing_status_code: crate::into_domain(result.queueing_status_code),
            session_duration: crate::into_domain(result.session_duration),
            suspended: crate::into_domain(result.suspended),
            total_active_users: crate::into_domain(result.total_active_users),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod waiting_room_event {

    pub struct WaitingRoomEventArgs {
        pub custom_page_html: pulumi_wasm_rust::Output<Option<String>>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub disable_session_renewal: pulumi_wasm_rust::Output<Option<bool>>,
        pub event_end_time: pulumi_wasm_rust::Output<String>,
        pub event_start_time: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub new_users_per_minute: pulumi_wasm_rust::Output<Option<i32>>,
        pub prequeue_start_time: pulumi_wasm_rust::Output<Option<String>>,
        pub queueing_method: pulumi_wasm_rust::Output<Option<String>>,
        pub session_duration: pulumi_wasm_rust::Output<Option<i32>>,
        pub shuffle_at_event_start: pulumi_wasm_rust::Output<Option<bool>>,
        pub suspended: pulumi_wasm_rust::Output<Option<bool>>,
        pub total_active_users: pulumi_wasm_rust::Output<Option<i32>>,
        pub waiting_room_id: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct WaitingRoomEventResult {
        pub created_on: pulumi_wasm_rust::Output<String>,
        pub custom_page_html: pulumi_wasm_rust::Output<Option<String>>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub disable_session_renewal: pulumi_wasm_rust::Output<Option<bool>>,
        pub event_end_time: pulumi_wasm_rust::Output<String>,
        pub event_start_time: pulumi_wasm_rust::Output<String>,
        pub modified_on: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub new_users_per_minute: pulumi_wasm_rust::Output<Option<i32>>,
        pub prequeue_start_time: pulumi_wasm_rust::Output<Option<String>>,
        pub queueing_method: pulumi_wasm_rust::Output<Option<String>>,
        pub session_duration: pulumi_wasm_rust::Output<Option<i32>>,
        pub shuffle_at_event_start: pulumi_wasm_rust::Output<Option<bool>>,
        pub suspended: pulumi_wasm_rust::Output<Option<bool>>,
        pub total_active_users: pulumi_wasm_rust::Output<Option<i32>>,
        pub waiting_room_id: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn waiting_room_event(name: &str, args: WaitingRoomEventArgs) -> WaitingRoomEventResult {
        let result = crate::bindings::pulumi::cloudflare::waiting_room_event::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::waiting_room_event::Args {
                custom_page_html: args.custom_page_html.get_inner(),
                description: args.description.get_inner(),
                disable_session_renewal: args.disable_session_renewal.get_inner(),
                event_end_time: args.event_end_time.get_inner(),
                event_start_time: args.event_start_time.get_inner(),
                name: args.name.get_inner(),
                new_users_per_minute: args.new_users_per_minute.get_inner(),
                prequeue_start_time: args.prequeue_start_time.get_inner(),
                queueing_method: args.queueing_method.get_inner(),
                session_duration: args.session_duration.get_inner(),
                shuffle_at_event_start: args.shuffle_at_event_start.get_inner(),
                suspended: args.suspended.get_inner(),
                total_active_users: args.total_active_users.get_inner(),
                waiting_room_id: args.waiting_room_id.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        WaitingRoomEventResult {
            created_on: crate::into_domain(result.created_on),
            custom_page_html: crate::into_domain(result.custom_page_html),
            description: crate::into_domain(result.description),
            disable_session_renewal: crate::into_domain(result.disable_session_renewal),
            event_end_time: crate::into_domain(result.event_end_time),
            event_start_time: crate::into_domain(result.event_start_time),
            modified_on: crate::into_domain(result.modified_on),
            name: crate::into_domain(result.name),
            new_users_per_minute: crate::into_domain(result.new_users_per_minute),
            prequeue_start_time: crate::into_domain(result.prequeue_start_time),
            queueing_method: crate::into_domain(result.queueing_method),
            session_duration: crate::into_domain(result.session_duration),
            shuffle_at_event_start: crate::into_domain(result.shuffle_at_event_start),
            suspended: crate::into_domain(result.suspended),
            total_active_users: crate::into_domain(result.total_active_users),
            waiting_room_id: crate::into_domain(result.waiting_room_id),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod waiting_room_rules {

    pub struct WaitingRoomRulesArgs {
        pub rules: pulumi_wasm_rust::Output<Option<Vec<crate::types::WaitingRoomRulesRule>>>,
        pub waiting_room_id: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct WaitingRoomRulesResult {
        pub rules: pulumi_wasm_rust::Output<Option<Vec<crate::types::WaitingRoomRulesRule>>>,
        pub waiting_room_id: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn waiting_room_rules(name: &str, args: WaitingRoomRulesArgs) -> WaitingRoomRulesResult {
        let result = crate::bindings::pulumi::cloudflare::waiting_room_rules::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::waiting_room_rules::Args {
                rules: args.rules.get_inner(),
                waiting_room_id: args.waiting_room_id.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        WaitingRoomRulesResult {
            rules: crate::into_domain(result.rules),
            waiting_room_id: crate::into_domain(result.waiting_room_id),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod waiting_room_settings {

    pub struct WaitingRoomSettingsArgs {
        pub search_engine_crawler_bypass: pulumi_wasm_rust::Output<Option<bool>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct WaitingRoomSettingsResult {
        pub search_engine_crawler_bypass: pulumi_wasm_rust::Output<Option<bool>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn waiting_room_settings(
        name: &str,
        args: WaitingRoomSettingsArgs,
    ) -> WaitingRoomSettingsResult {
        let result = crate::bindings::pulumi::cloudflare::waiting_room_settings::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::waiting_room_settings::Args {
                search_engine_crawler_bypass: args.search_engine_crawler_bypass.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        WaitingRoomSettingsResult {
            search_engine_crawler_bypass: crate::into_domain(result.search_engine_crawler_bypass),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod web3_hostname {

    pub struct Web3HostnameArgs {
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub dnslink: pulumi_wasm_rust::Output<Option<String>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub target: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct Web3HostnameResult {
        pub created_on: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub dnslink: pulumi_wasm_rust::Output<Option<String>>,
        pub modified_on: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub status: pulumi_wasm_rust::Output<String>,
        pub target: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn web_3_hostname(name: &str, args: Web3HostnameArgs) -> Web3HostnameResult {
        let result = crate::bindings::pulumi::cloudflare::web3_hostname::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::web3_hostname::Args {
                description: args.description.get_inner(),
                dnslink: args.dnslink.get_inner(),
                name: args.name.get_inner(),
                target: args.target.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        Web3HostnameResult {
            created_on: crate::into_domain(result.created_on),
            description: crate::into_domain(result.description),
            dnslink: crate::into_domain(result.dnslink),
            modified_on: crate::into_domain(result.modified_on),
            name: crate::into_domain(result.name),
            status: crate::into_domain(result.status),
            target: crate::into_domain(result.target),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod web_analytics_rule {

    pub struct WebAnalyticsRuleArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub host: pulumi_wasm_rust::Output<String>,
        pub inclusive: pulumi_wasm_rust::Output<bool>,
        pub is_paused: pulumi_wasm_rust::Output<bool>,
        pub paths: pulumi_wasm_rust::Output<Vec<String>>,
        pub ruleset_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct WebAnalyticsRuleResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub host: pulumi_wasm_rust::Output<String>,
        pub inclusive: pulumi_wasm_rust::Output<bool>,
        pub is_paused: pulumi_wasm_rust::Output<bool>,
        pub paths: pulumi_wasm_rust::Output<Vec<String>>,
        pub ruleset_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn web_analytics_rule(name: &str, args: WebAnalyticsRuleArgs) -> WebAnalyticsRuleResult {
        let result = crate::bindings::pulumi::cloudflare::web_analytics_rule::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::web_analytics_rule::Args {
                account_id: args.account_id.get_inner(),
                host: args.host.get_inner(),
                inclusive: args.inclusive.get_inner(),
                is_paused: args.is_paused.get_inner(),
                paths: args.paths.get_inner(),
                ruleset_id: args.ruleset_id.get_inner(),
            },
        );

        WebAnalyticsRuleResult {
            account_id: crate::into_domain(result.account_id),
            host: crate::into_domain(result.host),
            inclusive: crate::into_domain(result.inclusive),
            is_paused: crate::into_domain(result.is_paused),
            paths: crate::into_domain(result.paths),
            ruleset_id: crate::into_domain(result.ruleset_id),
        }
    }
}

pub mod web_analytics_site {

    pub struct WebAnalyticsSiteArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub auto_install: pulumi_wasm_rust::Output<bool>,
        pub host: pulumi_wasm_rust::Output<Option<String>>,
        pub zone_tag: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct WebAnalyticsSiteResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub auto_install: pulumi_wasm_rust::Output<bool>,
        pub host: pulumi_wasm_rust::Output<Option<String>>,
        pub ruleset_id: pulumi_wasm_rust::Output<String>,
        pub site_tag: pulumi_wasm_rust::Output<String>,
        pub site_token: pulumi_wasm_rust::Output<String>,
        pub snippet: pulumi_wasm_rust::Output<String>,
        pub zone_tag: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub fn web_analytics_site(name: &str, args: WebAnalyticsSiteArgs) -> WebAnalyticsSiteResult {
        let result = crate::bindings::pulumi::cloudflare::web_analytics_site::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::web_analytics_site::Args {
                account_id: args.account_id.get_inner(),
                auto_install: args.auto_install.get_inner(),
                host: args.host.get_inner(),
                zone_tag: args.zone_tag.get_inner(),
            },
        );

        WebAnalyticsSiteResult {
            account_id: crate::into_domain(result.account_id),
            auto_install: crate::into_domain(result.auto_install),
            host: crate::into_domain(result.host),
            ruleset_id: crate::into_domain(result.ruleset_id),
            site_tag: crate::into_domain(result.site_tag),
            site_token: crate::into_domain(result.site_token),
            snippet: crate::into_domain(result.snippet),
            zone_tag: crate::into_domain(result.zone_tag),
        }
    }
}

pub mod worker_cron_trigger {

    pub struct WorkerCronTriggerArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub schedules: pulumi_wasm_rust::Output<Vec<String>>,
        pub script_name: pulumi_wasm_rust::Output<String>,
    }

    pub struct WorkerCronTriggerResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub schedules: pulumi_wasm_rust::Output<Vec<String>>,
        pub script_name: pulumi_wasm_rust::Output<String>,
    }

    pub fn worker_cron_trigger(name: &str, args: WorkerCronTriggerArgs) -> WorkerCronTriggerResult {
        let result = crate::bindings::pulumi::cloudflare::worker_cron_trigger::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::worker_cron_trigger::Args {
                account_id: args.account_id.get_inner(),
                schedules: args.schedules.get_inner(),
                script_name: args.script_name.get_inner(),
            },
        );

        WorkerCronTriggerResult {
            account_id: crate::into_domain(result.account_id),
            schedules: crate::into_domain(result.schedules),
            script_name: crate::into_domain(result.script_name),
        }
    }
}

pub mod worker_domain {

    pub struct WorkerDomainArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub environment: pulumi_wasm_rust::Output<Option<String>>,
        pub hostname: pulumi_wasm_rust::Output<String>,
        pub service: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct WorkerDomainResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub environment: pulumi_wasm_rust::Output<Option<String>>,
        pub hostname: pulumi_wasm_rust::Output<String>,
        pub service: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn worker_domain(name: &str, args: WorkerDomainArgs) -> WorkerDomainResult {
        let result = crate::bindings::pulumi::cloudflare::worker_domain::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::worker_domain::Args {
                account_id: args.account_id.get_inner(),
                environment: args.environment.get_inner(),
                hostname: args.hostname.get_inner(),
                service: args.service.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        WorkerDomainResult {
            account_id: crate::into_domain(result.account_id),
            environment: crate::into_domain(result.environment),
            hostname: crate::into_domain(result.hostname),
            service: crate::into_domain(result.service),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod worker_route {

    pub struct WorkerRouteArgs {
        pub pattern: pulumi_wasm_rust::Output<String>,
        pub script_name: pulumi_wasm_rust::Output<Option<String>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct WorkerRouteResult {
        pub pattern: pulumi_wasm_rust::Output<String>,
        pub script_name: pulumi_wasm_rust::Output<Option<String>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn worker_route(name: &str, args: WorkerRouteArgs) -> WorkerRouteResult {
        let result = crate::bindings::pulumi::cloudflare::worker_route::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::worker_route::Args {
                pattern: args.pattern.get_inner(),
                script_name: args.script_name.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        WorkerRouteResult {
            pattern: crate::into_domain(result.pattern),
            script_name: crate::into_domain(result.script_name),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod worker_script {

    pub struct WorkerScriptArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub analytics_engine_bindings:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptAnalyticsEngineBinding>>>,
        pub compatibility_date: pulumi_wasm_rust::Output<Option<String>>,
        pub compatibility_flags: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub content: pulumi_wasm_rust::Output<String>,
        pub d1_database_bindings:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptD1DatabaseBinding>>>,
        pub dispatch_namespace: pulumi_wasm_rust::Output<Option<String>>,
        pub kv_namespace_bindings:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptKvNamespaceBinding>>>,
        pub logpush: pulumi_wasm_rust::Output<Option<bool>>,
        pub module: pulumi_wasm_rust::Output<Option<bool>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub placements: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptPlacement>>>,
        pub plain_text_bindings:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptPlainTextBinding>>>,
        pub queue_bindings:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptQueueBinding>>>,
        pub r2_bucket_bindings:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptR2BucketBinding>>>,
        pub secret_text_bindings:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptSecretTextBinding>>>,
        pub service_bindings:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptServiceBinding>>>,
        pub tags: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub webassembly_bindings:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptWebassemblyBinding>>>,
    }

    pub struct WorkerScriptResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub analytics_engine_bindings:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptAnalyticsEngineBinding>>>,
        pub compatibility_date: pulumi_wasm_rust::Output<Option<String>>,
        pub compatibility_flags: pulumi_wasm_rust::Output<Vec<String>>,
        pub content: pulumi_wasm_rust::Output<String>,
        pub d1_database_bindings:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptD1DatabaseBinding>>>,
        pub dispatch_namespace: pulumi_wasm_rust::Output<Option<String>>,
        pub kv_namespace_bindings:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptKvNamespaceBinding>>>,
        pub logpush: pulumi_wasm_rust::Output<Option<bool>>,
        pub module: pulumi_wasm_rust::Output<Option<bool>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub placements: pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptPlacement>>>,
        pub plain_text_bindings:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptPlainTextBinding>>>,
        pub queue_bindings:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptQueueBinding>>>,
        pub r2_bucket_bindings:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptR2BucketBinding>>>,
        pub secret_text_bindings:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptSecretTextBinding>>>,
        pub service_bindings:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptServiceBinding>>>,
        pub tags: pulumi_wasm_rust::Output<Vec<String>>,
        pub webassembly_bindings:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::WorkerScriptWebassemblyBinding>>>,
    }

    pub fn worker_script(name: &str, args: WorkerScriptArgs) -> WorkerScriptResult {
        let result = crate::bindings::pulumi::cloudflare::worker_script::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::worker_script::Args {
                account_id: args.account_id.get_inner(),
                analytics_engine_bindings: args.analytics_engine_bindings.get_inner(),
                compatibility_date: args.compatibility_date.get_inner(),
                compatibility_flags: args.compatibility_flags.get_inner(),
                content: args.content.get_inner(),
                d1_database_bindings: args.d1_database_bindings.get_inner(),
                dispatch_namespace: args.dispatch_namespace.get_inner(),
                kv_namespace_bindings: args.kv_namespace_bindings.get_inner(),
                logpush: args.logpush.get_inner(),
                module: args.module.get_inner(),
                name: args.name.get_inner(),
                placements: args.placements.get_inner(),
                plain_text_bindings: args.plain_text_bindings.get_inner(),
                queue_bindings: args.queue_bindings.get_inner(),
                r2_bucket_bindings: args.r2_bucket_bindings.get_inner(),
                secret_text_bindings: args.secret_text_bindings.get_inner(),
                service_bindings: args.service_bindings.get_inner(),
                tags: args.tags.get_inner(),
                webassembly_bindings: args.webassembly_bindings.get_inner(),
            },
        );

        WorkerScriptResult {
            account_id: crate::into_domain(result.account_id),
            analytics_engine_bindings: crate::into_domain(result.analytics_engine_bindings),
            compatibility_date: crate::into_domain(result.compatibility_date),
            compatibility_flags: crate::into_domain(result.compatibility_flags),
            content: crate::into_domain(result.content),
            d1_database_bindings: crate::into_domain(result.d1_database_bindings),
            dispatch_namespace: crate::into_domain(result.dispatch_namespace),
            kv_namespace_bindings: crate::into_domain(result.kv_namespace_bindings),
            logpush: crate::into_domain(result.logpush),
            module: crate::into_domain(result.module),
            name: crate::into_domain(result.name),
            placements: crate::into_domain(result.placements),
            plain_text_bindings: crate::into_domain(result.plain_text_bindings),
            queue_bindings: crate::into_domain(result.queue_bindings),
            r2_bucket_bindings: crate::into_domain(result.r2_bucket_bindings),
            secret_text_bindings: crate::into_domain(result.secret_text_bindings),
            service_bindings: crate::into_domain(result.service_bindings),
            tags: crate::into_domain(result.tags),
            webassembly_bindings: crate::into_domain(result.webassembly_bindings),
        }
    }
}

pub mod worker_secret {

    pub struct WorkerSecretArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub script_name: pulumi_wasm_rust::Output<String>,
        pub secret_text: pulumi_wasm_rust::Output<String>,
    }

    pub struct WorkerSecretResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub script_name: pulumi_wasm_rust::Output<String>,
        pub secret_text: pulumi_wasm_rust::Output<String>,
    }

    pub fn worker_secret(name: &str, args: WorkerSecretArgs) -> WorkerSecretResult {
        let result = crate::bindings::pulumi::cloudflare::worker_secret::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::worker_secret::Args {
                account_id: args.account_id.get_inner(),
                name: args.name.get_inner(),
                script_name: args.script_name.get_inner(),
                secret_text: args.secret_text.get_inner(),
            },
        );

        WorkerSecretResult {
            account_id: crate::into_domain(result.account_id),
            name: crate::into_domain(result.name),
            script_name: crate::into_domain(result.script_name),
            secret_text: crate::into_domain(result.secret_text),
        }
    }
}

pub mod workers_for_platforms_namespace {

    pub struct WorkersForPlatformsNamespaceArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
    }

    pub struct WorkersForPlatformsNamespaceResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
    }

    pub fn workers_for_platforms_namespace(
        name: &str,
        args: WorkersForPlatformsNamespaceArgs,
    ) -> WorkersForPlatformsNamespaceResult {
        let result = crate::bindings::pulumi::cloudflare::workers_for_platforms_namespace::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::workers_for_platforms_namespace::Args {
                account_id: args.account_id.get_inner(),
                name: args.name.get_inner(),
            },
        );

        WorkersForPlatformsNamespaceResult {
            account_id: crate::into_domain(result.account_id),
            name: crate::into_domain(result.name),
        }
    }
}

pub mod workers_kv {

    pub struct WorkersKvArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub key: pulumi_wasm_rust::Output<String>,
        pub namespace_id: pulumi_wasm_rust::Output<String>,
        pub value: pulumi_wasm_rust::Output<String>,
    }

    pub struct WorkersKvResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub key: pulumi_wasm_rust::Output<String>,
        pub namespace_id: pulumi_wasm_rust::Output<String>,
        pub value: pulumi_wasm_rust::Output<String>,
    }

    pub fn workers_kv(name: &str, args: WorkersKvArgs) -> WorkersKvResult {
        let result = crate::bindings::pulumi::cloudflare::workers_kv::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::workers_kv::Args {
                account_id: args.account_id.get_inner(),
                key: args.key.get_inner(),
                namespace_id: args.namespace_id.get_inner(),
                value: args.value.get_inner(),
            },
        );

        WorkersKvResult {
            account_id: crate::into_domain(result.account_id),
            key: crate::into_domain(result.key),
            namespace_id: crate::into_domain(result.namespace_id),
            value: crate::into_domain(result.value),
        }
    }
}

pub mod workers_kv_namespace {

    pub struct WorkersKvNamespaceArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub title: pulumi_wasm_rust::Output<String>,
    }

    pub struct WorkersKvNamespaceResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub title: pulumi_wasm_rust::Output<String>,
    }

    pub fn workers_kv_namespace(
        name: &str,
        args: WorkersKvNamespaceArgs,
    ) -> WorkersKvNamespaceResult {
        let result = crate::bindings::pulumi::cloudflare::workers_kv_namespace::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::workers_kv_namespace::Args {
                account_id: args.account_id.get_inner(),
                title: args.title.get_inner(),
            },
        );

        WorkersKvNamespaceResult {
            account_id: crate::into_domain(result.account_id),
            title: crate::into_domain(result.title),
        }
    }
}

pub mod zone {

    pub struct ZoneArgs {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub jump_start: pulumi_wasm_rust::Output<Option<bool>>,
        pub paused: pulumi_wasm_rust::Output<Option<bool>>,
        pub plan: pulumi_wasm_rust::Output<Option<String>>,
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
        pub zone: pulumi_wasm_rust::Output<String>,
    }

    pub struct ZoneResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub jump_start: pulumi_wasm_rust::Output<Option<bool>>,
        pub meta: pulumi_wasm_rust::Output<std::collections::HashMap<String, bool>>,
        pub name_servers: pulumi_wasm_rust::Output<Vec<String>>,
        pub paused: pulumi_wasm_rust::Output<Option<bool>>,
        pub plan: pulumi_wasm_rust::Output<String>,
        pub status: pulumi_wasm_rust::Output<String>,
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
        pub vanity_name_servers: pulumi_wasm_rust::Output<Vec<String>>,
        pub verification_key: pulumi_wasm_rust::Output<String>,
        pub zone: pulumi_wasm_rust::Output<String>,
    }

    pub fn zone(name: &str, args: ZoneArgs) -> ZoneResult {
        let result = crate::bindings::pulumi::cloudflare::zone::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::zone::Args {
                account_id: args.account_id.get_inner(),
                jump_start: args.jump_start.get_inner(),
                paused: args.paused.get_inner(),
                plan: args.plan.get_inner(),
                type_: args.type_.get_inner(),
                zone: args.zone.get_inner(),
            },
        );

        ZoneResult {
            account_id: crate::into_domain(result.account_id),
            jump_start: crate::into_domain(result.jump_start),
            meta: crate::into_domain(result.meta),
            name_servers: crate::into_domain(result.name_servers),
            paused: crate::into_domain(result.paused),
            plan: crate::into_domain(result.plan),
            status: crate::into_domain(result.status),
            type_: crate::into_domain(result.type_),
            vanity_name_servers: crate::into_domain(result.vanity_name_servers),
            verification_key: crate::into_domain(result.verification_key),
            zone: crate::into_domain(result.zone),
        }
    }
}

pub mod zone_cache_reserve {

    pub struct ZoneCacheReserveArgs {
        pub enabled: pulumi_wasm_rust::Output<bool>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct ZoneCacheReserveResult {
        pub enabled: pulumi_wasm_rust::Output<bool>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn zone_cache_reserve(name: &str, args: ZoneCacheReserveArgs) -> ZoneCacheReserveResult {
        let result = crate::bindings::pulumi::cloudflare::zone_cache_reserve::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::zone_cache_reserve::Args {
                enabled: args.enabled.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        ZoneCacheReserveResult {
            enabled: crate::into_domain(result.enabled),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod zone_cache_variants {

    pub struct ZoneCacheVariantsArgs {
        pub avifs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub bmps: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub gifs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub jp2s: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub jpegs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub jpg2s: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub jpgs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub pngs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub tiffs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub tifs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub webps: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct ZoneCacheVariantsResult {
        pub avifs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub bmps: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub gifs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub jp2s: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub jpegs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub jpg2s: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub jpgs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub pngs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub tiffs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub tifs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub webps: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn zone_cache_variants(name: &str, args: ZoneCacheVariantsArgs) -> ZoneCacheVariantsResult {
        let result = crate::bindings::pulumi::cloudflare::zone_cache_variants::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::zone_cache_variants::Args {
                avifs: args.avifs.get_inner(),
                bmps: args.bmps.get_inner(),
                gifs: args.gifs.get_inner(),
                jp2s: args.jp2s.get_inner(),
                jpegs: args.jpegs.get_inner(),
                jpg2s: args.jpg2s.get_inner(),
                jpgs: args.jpgs.get_inner(),
                pngs: args.pngs.get_inner(),
                tiffs: args.tiffs.get_inner(),
                tifs: args.tifs.get_inner(),
                webps: args.webps.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        ZoneCacheVariantsResult {
            avifs: crate::into_domain(result.avifs),
            bmps: crate::into_domain(result.bmps),
            gifs: crate::into_domain(result.gifs),
            jp2s: crate::into_domain(result.jp2s),
            jpegs: crate::into_domain(result.jpegs),
            jpg2s: crate::into_domain(result.jpg2s),
            jpgs: crate::into_domain(result.jpgs),
            pngs: crate::into_domain(result.pngs),
            tiffs: crate::into_domain(result.tiffs),
            tifs: crate::into_domain(result.tifs),
            webps: crate::into_domain(result.webps),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod zone_dnssec {

    pub struct ZoneDnssecArgs {
        pub modified_on: pulumi_wasm_rust::Output<Option<String>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct ZoneDnssecResult {
        pub algorithm: pulumi_wasm_rust::Output<String>,
        pub digest: pulumi_wasm_rust::Output<String>,
        pub digest_algorithm: pulumi_wasm_rust::Output<String>,
        pub digest_type: pulumi_wasm_rust::Output<String>,
        pub ds: pulumi_wasm_rust::Output<String>,
        pub flags: pulumi_wasm_rust::Output<i32>,
        pub key_tag: pulumi_wasm_rust::Output<i32>,
        pub key_type: pulumi_wasm_rust::Output<String>,
        pub modified_on: pulumi_wasm_rust::Output<String>,
        pub public_key: pulumi_wasm_rust::Output<String>,
        pub status: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn zone_dnssec(name: &str, args: ZoneDnssecArgs) -> ZoneDnssecResult {
        let result = crate::bindings::pulumi::cloudflare::zone_dnssec::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::zone_dnssec::Args {
                modified_on: args.modified_on.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        ZoneDnssecResult {
            algorithm: crate::into_domain(result.algorithm),
            digest: crate::into_domain(result.digest),
            digest_algorithm: crate::into_domain(result.digest_algorithm),
            digest_type: crate::into_domain(result.digest_type),
            ds: crate::into_domain(result.ds),
            flags: crate::into_domain(result.flags),
            key_tag: crate::into_domain(result.key_tag),
            key_type: crate::into_domain(result.key_type),
            modified_on: crate::into_domain(result.modified_on),
            public_key: crate::into_domain(result.public_key),
            status: crate::into_domain(result.status),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod zone_hold {

    pub struct ZoneHoldArgs {
        pub hold: pulumi_wasm_rust::Output<bool>,
        pub hold_after: pulumi_wasm_rust::Output<Option<String>>,
        pub include_subdomains: pulumi_wasm_rust::Output<Option<bool>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct ZoneHoldResult {
        pub hold: pulumi_wasm_rust::Output<bool>,
        pub hold_after: pulumi_wasm_rust::Output<String>,
        pub include_subdomains: pulumi_wasm_rust::Output<Option<bool>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn zone_hold(name: &str, args: ZoneHoldArgs) -> ZoneHoldResult {
        let result = crate::bindings::pulumi::cloudflare::zone_hold::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::zone_hold::Args {
                hold: args.hold.get_inner(),
                hold_after: args.hold_after.get_inner(),
                include_subdomains: args.include_subdomains.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        ZoneHoldResult {
            hold: crate::into_domain(result.hold),
            hold_after: crate::into_domain(result.hold_after),
            include_subdomains: crate::into_domain(result.include_subdomains),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod zone_lockdown {

    pub struct ZoneLockdownArgs {
        pub configurations: pulumi_wasm_rust::Output<Vec<crate::types::ZoneLockdownConfiguration>>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub paused: pulumi_wasm_rust::Output<Option<bool>>,
        pub priority: pulumi_wasm_rust::Output<Option<i32>>,
        pub urls: pulumi_wasm_rust::Output<Vec<String>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct ZoneLockdownResult {
        pub configurations: pulumi_wasm_rust::Output<Vec<crate::types::ZoneLockdownConfiguration>>,
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub paused: pulumi_wasm_rust::Output<Option<bool>>,
        pub priority: pulumi_wasm_rust::Output<Option<i32>>,
        pub urls: pulumi_wasm_rust::Output<Vec<String>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub fn zone_lockdown(name: &str, args: ZoneLockdownArgs) -> ZoneLockdownResult {
        let result = crate::bindings::pulumi::cloudflare::zone_lockdown::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::zone_lockdown::Args {
                configurations: args.configurations.get_inner(),
                description: args.description.get_inner(),
                paused: args.paused.get_inner(),
                priority: args.priority.get_inner(),
                urls: args.urls.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        ZoneLockdownResult {
            configurations: crate::into_domain(result.configurations),
            description: crate::into_domain(result.description),
            paused: crate::into_domain(result.paused),
            priority: crate::into_domain(result.priority),
            urls: crate::into_domain(result.urls),
            zone_id: crate::into_domain(result.zone_id),
        }
    }
}

pub mod zone_settings_override {

    pub struct ZoneSettingsOverrideArgs {
        pub settings: pulumi_wasm_rust::Output<Option<crate::types::ZoneSettingsOverrideSettings>>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }

    pub struct ZoneSettingsOverrideResult {
        pub initial_settings:
            pulumi_wasm_rust::Output<Vec<crate::types::ZoneSettingsOverrideInitialSetting>>,
        pub initial_settings_read_at: pulumi_wasm_rust::Output<String>,
        pub readonly_settings: pulumi_wasm_rust::Output<Vec<String>>,
        pub settings: pulumi_wasm_rust::Output<crate::types::ZoneSettingsOverrideSettings>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
        pub zone_status: pulumi_wasm_rust::Output<String>,
        pub zone_type: pulumi_wasm_rust::Output<String>,
    }

    pub fn zone_settings_override(
        name: &str,
        args: ZoneSettingsOverrideArgs,
    ) -> ZoneSettingsOverrideResult {
        let result = crate::bindings::pulumi::cloudflare::zone_settings_override::invoke(
            name,
            &crate::bindings::pulumi::cloudflare::zone_settings_override::Args {
                settings: args.settings.get_inner(),
                zone_id: args.zone_id.get_inner(),
            },
        );

        ZoneSettingsOverrideResult {
            initial_settings: crate::into_domain(result.initial_settings),
            initial_settings_read_at: crate::into_domain(result.initial_settings_read_at),
            readonly_settings: crate::into_domain(result.readonly_settings),
            settings: crate::into_domain(result.settings),
            zone_id: crate::into_domain(result.zone_id),
            zone_status: crate::into_domain(result.zone_status),
            zone_type: crate::into_domain(result.zone_type),
        }
    }
}
