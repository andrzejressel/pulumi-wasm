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
                account_id: &crate::clone::<Option<String>>(args.account_id),
                allow_authenticate_via_warp: &crate::clone::<Option<bool>>(
                    args.allow_authenticate_via_warp,
                ),
                allowed_idps: &crate::clone::<Option<Vec<String>>>(args.allowed_idps),
                app_launcher_logo_url: &crate::clone::<Option<String>>(args.app_launcher_logo_url),
                app_launcher_visible: &crate::clone::<Option<bool>>(args.app_launcher_visible),
                auto_redirect_to_identity: &crate::clone::<Option<bool>>(
                    args.auto_redirect_to_identity,
                ),
                bg_color: &crate::clone::<Option<String>>(args.bg_color),
                cors_headers: &crate::clone::<Option<Vec<crate::types::AccessApplicationCorsHeader>>>(
                    args.cors_headers,
                ),
                custom_deny_message: &crate::clone::<Option<String>>(args.custom_deny_message),
                custom_deny_url: &crate::clone::<Option<String>>(args.custom_deny_url),
                custom_non_identity_deny_url: &crate::clone::<Option<String>>(
                    args.custom_non_identity_deny_url,
                ),
                custom_pages: &crate::clone::<Option<Vec<String>>>(args.custom_pages),
                domain: &crate::clone::<Option<String>>(args.domain),
                enable_binding_cookie: &crate::clone::<Option<bool>>(args.enable_binding_cookie),
                footer_links: &crate::clone::<Option<Vec<crate::types::AccessApplicationFooterLink>>>(
                    args.footer_links,
                ),
                header_bg_color: &crate::clone::<Option<String>>(args.header_bg_color),
                http_only_cookie_attribute: &crate::clone::<Option<bool>>(
                    args.http_only_cookie_attribute,
                ),
                landing_page_design: &crate::clone::<
                    Option<crate::types::AccessApplicationLandingPageDesign>,
                >(args.landing_page_design),
                logo_url: &crate::clone::<Option<String>>(args.logo_url),
                name: &crate::clone::<Option<String>>(args.name),
                saas_app: &crate::clone::<Option<crate::types::AccessApplicationSaasApp>>(
                    args.saas_app,
                ),
                same_site_cookie_attribute: &crate::clone::<Option<String>>(
                    args.same_site_cookie_attribute,
                ),
                self_hosted_domains: &crate::clone::<Option<Vec<String>>>(args.self_hosted_domains),
                service_auth401_redirect: &crate::clone::<Option<bool>>(
                    args.service_auth401_redirect,
                ),
                session_duration: &crate::clone::<Option<String>>(args.session_duration),
                skip_interstitial: &crate::clone::<Option<bool>>(args.skip_interstitial),
                tags: &crate::clone::<Option<Vec<String>>>(args.tags),
                type_: &crate::clone::<Option<String>>(args.type_),
                zone_id: &crate::clone::<Option<String>>(args.zone_id),
            },
        );

        AccessApplicationResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            allow_authenticate_via_warp: crate::random_to_domain_mapper::<Option<bool>>(
                result.allow_authenticate_via_warp,
            ),
            allowed_idps: crate::random_to_domain_mapper::<Option<Vec<String>>>(
                result.allowed_idps,
            ),
            app_launcher_logo_url: crate::random_to_domain_mapper::<Option<String>>(
                result.app_launcher_logo_url,
            ),
            app_launcher_visible: crate::random_to_domain_mapper::<Option<bool>>(
                result.app_launcher_visible,
            ),
            aud: crate::random_to_domain_mapper::<String>(result.aud),
            auto_redirect_to_identity: crate::random_to_domain_mapper::<Option<bool>>(
                result.auto_redirect_to_identity,
            ),
            bg_color: crate::random_to_domain_mapper::<Option<String>>(result.bg_color),
            cors_headers: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::AccessApplicationCorsHeader>>,
            >(result.cors_headers),
            custom_deny_message: crate::random_to_domain_mapper::<Option<String>>(
                result.custom_deny_message,
            ),
            custom_deny_url: crate::random_to_domain_mapper::<Option<String>>(
                result.custom_deny_url,
            ),
            custom_non_identity_deny_url: crate::random_to_domain_mapper::<Option<String>>(
                result.custom_non_identity_deny_url,
            ),
            custom_pages: crate::random_to_domain_mapper::<Option<Vec<String>>>(
                result.custom_pages,
            ),
            domain: crate::random_to_domain_mapper::<String>(result.domain),
            enable_binding_cookie: crate::random_to_domain_mapper::<Option<bool>>(
                result.enable_binding_cookie,
            ),
            footer_links: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::AccessApplicationFooterLink>>,
            >(result.footer_links),
            header_bg_color: crate::random_to_domain_mapper::<Option<String>>(
                result.header_bg_color,
            ),
            http_only_cookie_attribute: crate::random_to_domain_mapper::<Option<bool>>(
                result.http_only_cookie_attribute,
            ),
            landing_page_design: crate::random_to_domain_mapper::<
                Option<crate::types::AccessApplicationLandingPageDesign>,
            >(result.landing_page_design),
            logo_url: crate::random_to_domain_mapper::<Option<String>>(result.logo_url),
            name: crate::random_to_domain_mapper::<String>(result.name),
            saas_app: crate::random_to_domain_mapper::<
                Option<crate::types::AccessApplicationSaasApp>,
            >(result.saas_app),
            same_site_cookie_attribute: crate::random_to_domain_mapper::<Option<String>>(
                result.same_site_cookie_attribute,
            ),
            self_hosted_domains: crate::random_to_domain_mapper::<Option<Vec<String>>>(
                result.self_hosted_domains,
            ),
            service_auth401_redirect: crate::random_to_domain_mapper::<Option<bool>>(
                result.service_auth401_redirect,
            ),
            session_duration: crate::random_to_domain_mapper::<Option<String>>(
                result.session_duration,
            ),
            skip_interstitial: crate::random_to_domain_mapper::<Option<bool>>(
                result.skip_interstitial,
            ),
            tags: crate::random_to_domain_mapper::<Option<Vec<String>>>(result.tags),
            type_: crate::random_to_domain_mapper::<Option<String>>(result.type_),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                account_id: &crate::clone::<Option<String>>(args.account_id),
                application_id: &crate::clone::<String>(args.application_id),
                zone_id: &crate::clone::<Option<String>>(args.zone_id),
            },
        );

        AccessCaCertificateResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            application_id: crate::random_to_domain_mapper::<String>(result.application_id),
            aud: crate::random_to_domain_mapper::<String>(result.aud),
            public_key: crate::random_to_domain_mapper::<String>(result.public_key),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                account_id: &crate::clone::<Option<String>>(args.account_id),
                app_count: &crate::clone::<Option<i32>>(args.app_count),
                custom_html: &crate::clone::<Option<String>>(args.custom_html),
                name: &crate::clone::<String>(args.name),
                type_: &crate::clone::<String>(args.type_),
                zone_id: &crate::clone::<Option<String>>(args.zone_id),
            },
        );

        AccessCustomPageResult {
            account_id: crate::random_to_domain_mapper::<Option<String>>(result.account_id),
            app_count: crate::random_to_domain_mapper::<Option<i32>>(result.app_count),
            custom_html: crate::random_to_domain_mapper::<Option<String>>(result.custom_html),
            name: crate::random_to_domain_mapper::<String>(result.name),
            type_: crate::random_to_domain_mapper::<String>(result.type_),
            zone_id: crate::random_to_domain_mapper::<Option<String>>(result.zone_id),
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
                account_id: &crate::clone::<Option<String>>(args.account_id),
                excludes: &crate::clone::<Option<Vec<crate::types::AccessGroupExclude>>>(
                    args.excludes,
                ),
                includes: &crate::clone::<Vec<crate::types::AccessGroupInclude>>(args.includes),
                name: &crate::clone::<String>(args.name),
                requires: &crate::clone::<Option<Vec<crate::types::AccessGroupRequire>>>(
                    args.requires,
                ),
                zone_id: &crate::clone::<Option<String>>(args.zone_id),
            },
        );

        AccessGroupResult {
            account_id: crate::random_to_domain_mapper::<Option<String>>(result.account_id),
            excludes: crate::random_to_domain_mapper::<Option<Vec<crate::types::AccessGroupExclude>>>(
                result.excludes,
            ),
            includes: crate::random_to_domain_mapper::<Vec<crate::types::AccessGroupInclude>>(
                result.includes,
            ),
            name: crate::random_to_domain_mapper::<String>(result.name),
            requires: crate::random_to_domain_mapper::<Option<Vec<crate::types::AccessGroupRequire>>>(
                result.requires,
            ),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                account_id: &crate::clone::<Option<String>>(args.account_id),
                configs: &crate::clone::<Option<Vec<crate::types::AccessIdentityProviderConfig>>>(
                    args.configs,
                ),
                name: &crate::clone::<String>(args.name),
                scim_configs: &crate::clone::<
                    Option<Vec<crate::types::AccessIdentityProviderScimConfig>>,
                >(args.scim_configs),
                type_: &crate::clone::<String>(args.type_),
                zone_id: &crate::clone::<Option<String>>(args.zone_id),
            },
        );

        AccessIdentityProviderResult {
            account_id: crate::random_to_domain_mapper::<Option<String>>(result.account_id),
            configs: crate::random_to_domain_mapper::<
                Vec<crate::types::AccessIdentityProviderConfig>,
            >(result.configs),
            name: crate::random_to_domain_mapper::<String>(result.name),
            scim_configs: crate::random_to_domain_mapper::<
                Vec<crate::types::AccessIdentityProviderScimConfig>,
            >(result.scim_configs),
            type_: crate::random_to_domain_mapper::<String>(result.type_),
            zone_id: crate::random_to_domain_mapper::<Option<String>>(result.zone_id),
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
                account_id: &crate::clone::<String>(args.account_id),
                key_rotation_interval_days: &crate::clone::<Option<i32>>(
                    args.key_rotation_interval_days,
                ),
            },
        );

        AccessKeysConfigurationResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            key_rotation_interval_days: crate::random_to_domain_mapper::<i32>(
                result.key_rotation_interval_days,
            ),
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
                account_id: &crate::clone::<Option<String>>(args.account_id),
                associated_hostnames: &crate::clone::<Option<Vec<String>>>(
                    args.associated_hostnames,
                ),
                certificate: &crate::clone::<Option<String>>(args.certificate),
                name: &crate::clone::<String>(args.name),
                zone_id: &crate::clone::<Option<String>>(args.zone_id),
            },
        );

        AccessMutualTlsCertificateResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            associated_hostnames: crate::random_to_domain_mapper::<Option<Vec<String>>>(
                result.associated_hostnames,
            ),
            certificate: crate::random_to_domain_mapper::<Option<String>>(result.certificate),
            fingerprint: crate::random_to_domain_mapper::<String>(result.fingerprint),
            name: crate::random_to_domain_mapper::<String>(result.name),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                    account_id: &crate::clone::<Option<String>>(args.account_id),
                    settings: &crate::clone::<
                        Option<Vec<crate::types::AccessMutualTlsHostnameSettingsSetting>>,
                    >(args.settings),
                    zone_id: &crate::clone::<Option<String>>(args.zone_id),
                },
            );

        AccessMutualTlsHostnameSettingsResult {
            account_id: crate::random_to_domain_mapper::<Option<String>>(result.account_id),
            settings: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::AccessMutualTlsHostnameSettingsSetting>>,
            >(result.settings),
            zone_id: crate::random_to_domain_mapper::<Option<String>>(result.zone_id),
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
                account_id: &crate::clone::<Option<String>>(args.account_id),
                allow_authenticate_via_warp: &crate::clone::<Option<bool>>(
                    args.allow_authenticate_via_warp,
                ),
                auth_domain: &crate::clone::<String>(args.auth_domain),
                auto_redirect_to_identity: &crate::clone::<Option<bool>>(
                    args.auto_redirect_to_identity,
                ),
                custom_pages: &crate::clone::<
                    Option<Vec<crate::types::AccessOrganizationCustomPage>>,
                >(args.custom_pages),
                is_ui_read_only: &crate::clone::<Option<bool>>(args.is_ui_read_only),
                login_designs: &crate::clone::<
                    Option<Vec<crate::types::AccessOrganizationLoginDesign>>,
                >(args.login_designs),
                name: &crate::clone::<Option<String>>(args.name),
                session_duration: &crate::clone::<Option<String>>(args.session_duration),
                ui_read_only_toggle_reason: &crate::clone::<Option<String>>(
                    args.ui_read_only_toggle_reason,
                ),
                user_seat_expiration_inactive_time: &crate::clone::<Option<String>>(
                    args.user_seat_expiration_inactive_time,
                ),
                warp_auth_session_duration: &crate::clone::<Option<String>>(
                    args.warp_auth_session_duration,
                ),
                zone_id: &crate::clone::<Option<String>>(args.zone_id),
            },
        );

        AccessOrganizationResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            allow_authenticate_via_warp: crate::random_to_domain_mapper::<Option<bool>>(
                result.allow_authenticate_via_warp,
            ),
            auth_domain: crate::random_to_domain_mapper::<String>(result.auth_domain),
            auto_redirect_to_identity: crate::random_to_domain_mapper::<Option<bool>>(
                result.auto_redirect_to_identity,
            ),
            custom_pages: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::AccessOrganizationCustomPage>>,
            >(result.custom_pages),
            is_ui_read_only: crate::random_to_domain_mapper::<Option<bool>>(result.is_ui_read_only),
            login_designs: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::AccessOrganizationLoginDesign>>,
            >(result.login_designs),
            name: crate::random_to_domain_mapper::<Option<String>>(result.name),
            session_duration: crate::random_to_domain_mapper::<Option<String>>(
                result.session_duration,
            ),
            ui_read_only_toggle_reason: crate::random_to_domain_mapper::<Option<String>>(
                result.ui_read_only_toggle_reason,
            ),
            user_seat_expiration_inactive_time: crate::random_to_domain_mapper::<Option<String>>(
                result.user_seat_expiration_inactive_time,
            ),
            warp_auth_session_duration: crate::random_to_domain_mapper::<Option<String>>(
                result.warp_auth_session_duration,
            ),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                account_id: &crate::clone::<Option<String>>(args.account_id),
                application_id: &crate::clone::<String>(args.application_id),
                approval_groups: &crate::clone::<
                    Option<Vec<crate::types::AccessPolicyApprovalGroup>>,
                >(args.approval_groups),
                approval_required: &crate::clone::<Option<bool>>(args.approval_required),
                decision: &crate::clone::<String>(args.decision),
                excludes: &crate::clone::<Option<Vec<crate::types::AccessPolicyExclude>>>(
                    args.excludes,
                ),
                includes: &crate::clone::<Vec<crate::types::AccessPolicyInclude>>(args.includes),
                isolation_required: &crate::clone::<Option<bool>>(args.isolation_required),
                name: &crate::clone::<String>(args.name),
                precedence: &crate::clone::<i32>(args.precedence),
                purpose_justification_prompt: &crate::clone::<Option<String>>(
                    args.purpose_justification_prompt,
                ),
                purpose_justification_required: &crate::clone::<Option<bool>>(
                    args.purpose_justification_required,
                ),
                requires: &crate::clone::<Option<Vec<crate::types::AccessPolicyRequire>>>(
                    args.requires,
                ),
                session_duration: &crate::clone::<Option<String>>(args.session_duration),
                zone_id: &crate::clone::<Option<String>>(args.zone_id),
            },
        );

        AccessPolicyResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            application_id: crate::random_to_domain_mapper::<String>(result.application_id),
            approval_groups: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::AccessPolicyApprovalGroup>>,
            >(result.approval_groups),
            approval_required: crate::random_to_domain_mapper::<Option<bool>>(
                result.approval_required,
            ),
            decision: crate::random_to_domain_mapper::<String>(result.decision),
            excludes: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::AccessPolicyExclude>>,
            >(result.excludes),
            includes: crate::random_to_domain_mapper::<Vec<crate::types::AccessPolicyInclude>>(
                result.includes,
            ),
            isolation_required: crate::random_to_domain_mapper::<Option<bool>>(
                result.isolation_required,
            ),
            name: crate::random_to_domain_mapper::<String>(result.name),
            precedence: crate::random_to_domain_mapper::<i32>(result.precedence),
            purpose_justification_prompt: crate::random_to_domain_mapper::<Option<String>>(
                result.purpose_justification_prompt,
            ),
            purpose_justification_required: crate::random_to_domain_mapper::<Option<bool>>(
                result.purpose_justification_required,
            ),
            requires: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::AccessPolicyRequire>>,
            >(result.requires),
            session_duration: crate::random_to_domain_mapper::<Option<String>>(
                result.session_duration,
            ),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                account_id: &crate::clone::<Option<String>>(args.account_id),
                configuration: &crate::clone::<crate::types::AccessRuleConfiguration>(
                    args.configuration,
                ),
                mode: &crate::clone::<String>(args.mode),
                notes: &crate::clone::<Option<String>>(args.notes),
                zone_id: &crate::clone::<Option<String>>(args.zone_id),
            },
        );

        AccessRuleResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            configuration: crate::random_to_domain_mapper::<crate::types::AccessRuleConfiguration>(
                result.configuration,
            ),
            mode: crate::random_to_domain_mapper::<String>(result.mode),
            notes: crate::random_to_domain_mapper::<Option<String>>(result.notes),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                account_id: &crate::clone::<Option<String>>(args.account_id),
                duration: &crate::clone::<Option<String>>(args.duration),
                min_days_for_renewal: &crate::clone::<Option<i32>>(args.min_days_for_renewal),
                name: &crate::clone::<String>(args.name),
                zone_id: &crate::clone::<Option<String>>(args.zone_id),
            },
        );

        AccessServiceTokenResult {
            account_id: crate::random_to_domain_mapper::<Option<String>>(result.account_id),
            client_id: crate::random_to_domain_mapper::<String>(result.client_id),
            client_secret: crate::random_to_domain_mapper::<String>(result.client_secret),
            duration: crate::random_to_domain_mapper::<String>(result.duration),
            expires_at: crate::random_to_domain_mapper::<String>(result.expires_at),
            min_days_for_renewal: crate::random_to_domain_mapper::<Option<i32>>(
                result.min_days_for_renewal,
            ),
            name: crate::random_to_domain_mapper::<String>(result.name),
            zone_id: crate::random_to_domain_mapper::<Option<String>>(result.zone_id),
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
                account_id: &crate::clone::<Option<String>>(args.account_id),
                app_count: &crate::clone::<Option<i32>>(args.app_count),
                name: &crate::clone::<String>(args.name),
                zone_id: &crate::clone::<Option<String>>(args.zone_id),
            },
        );

        AccessTagResult {
            account_id: crate::random_to_domain_mapper::<Option<String>>(result.account_id),
            app_count: crate::random_to_domain_mapper::<i32>(result.app_count),
            name: crate::random_to_domain_mapper::<String>(result.name),
            zone_id: crate::random_to_domain_mapper::<Option<String>>(result.zone_id),
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
                enforce_twofactor: &crate::clone::<Option<bool>>(args.enforce_twofactor),
                name: &crate::clone::<String>(args.name),
                type_: &crate::clone::<Option<String>>(args.type_),
            },
        );

        AccountResult {
            enforce_twofactor: crate::random_to_domain_mapper::<Option<bool>>(
                result.enforce_twofactor,
            ),
            name: crate::random_to_domain_mapper::<String>(result.name),
            type_: crate::random_to_domain_mapper::<Option<String>>(result.type_),
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
                account_id: &crate::clone::<String>(args.account_id),
                email_address: &crate::clone::<String>(args.email_address),
                role_ids: &crate::clone::<Vec<String>>(args.role_ids),
                status: &crate::clone::<Option<String>>(args.status),
            },
        );

        AccountMemberResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            email_address: crate::random_to_domain_mapper::<String>(result.email_address),
            role_ids: crate::random_to_domain_mapper::<Vec<String>>(result.role_ids),
            status: crate::random_to_domain_mapper::<String>(result.status),
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
                account_id: &crate::clone::<String>(args.account_id),
                default_sni: &crate::clone::<Option<String>>(args.default_sni),
                description: &crate::clone::<Option<String>>(args.description),
                enabled: &crate::clone::<bool>(args.enabled),
                ips: &crate::clone::<Option<Vec<crate::types::AddressMapIp>>>(args.ips),
                memberships: &crate::clone::<Option<Vec<crate::types::AddressMapMembership>>>(
                    args.memberships,
                ),
            },
        );

        AddressMapResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            can_delete: crate::random_to_domain_mapper::<bool>(result.can_delete),
            can_modify_ips: crate::random_to_domain_mapper::<bool>(result.can_modify_ips),
            default_sni: crate::random_to_domain_mapper::<Option<String>>(result.default_sni),
            description: crate::random_to_domain_mapper::<Option<String>>(result.description),
            enabled: crate::random_to_domain_mapper::<bool>(result.enabled),
            ips: crate::random_to_domain_mapper::<Option<Vec<crate::types::AddressMapIp>>>(
                result.ips,
            ),
            memberships: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::AddressMapMembership>>,
            >(result.memberships),
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
                auth_id_characteristics: &crate::clone::<
                    Option<Vec<crate::types::ApiShieldAuthIdCharacteristic>>,
                >(args.auth_id_characteristics),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        ApiShieldResult {
            auth_id_characteristics: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::ApiShieldAuthIdCharacteristic>>,
            >(result.auth_id_characteristics),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                endpoint: &crate::clone::<String>(args.endpoint),
                host: &crate::clone::<String>(args.host),
                method: &crate::clone::<String>(args.method),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        ApiShieldOperationResult {
            endpoint: crate::random_to_domain_mapper::<String>(result.endpoint),
            host: crate::random_to_domain_mapper::<String>(result.host),
            method: crate::random_to_domain_mapper::<String>(result.method),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
            mitigation_action: &crate::clone::<Option<String>>(args.mitigation_action),
            operation_id: &crate::clone::<String>(args.operation_id),
            zone_id: &crate::clone::<String>(args.zone_id),
        });

        ApiShieldOperationSchemaValidationSettingsResult {
            mitigation_action: crate::random_to_domain_mapper::<Option<String>>(
                result.mitigation_action,
            ),
            operation_id: crate::random_to_domain_mapper::<String>(result.operation_id),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                kind: &crate::clone::<Option<String>>(args.kind),
                name: &crate::clone::<String>(args.name),
                source: &crate::clone::<String>(args.source),
                validation_enabled: &crate::clone::<Option<bool>>(args.validation_enabled),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        ApiShieldSchemaResult {
            kind: crate::random_to_domain_mapper::<Option<String>>(result.kind),
            name: crate::random_to_domain_mapper::<String>(result.name),
            source: crate::random_to_domain_mapper::<String>(result.source),
            validation_enabled: crate::random_to_domain_mapper::<Option<bool>>(
                result.validation_enabled,
            ),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                    validation_default_mitigation_action: &crate::clone::<String>(
                        args.validation_default_mitigation_action,
                    ),
                    validation_override_mitigation_action: &crate::clone::<Option<String>>(
                        args.validation_override_mitigation_action,
                    ),
                    zone_id: &crate::clone::<String>(args.zone_id),
                },
            );

        ApiShieldSchemaValidationSettingsResult {
            validation_default_mitigation_action: crate::random_to_domain_mapper::<String>(
                result.validation_default_mitigation_action,
            ),
            validation_override_mitigation_action: crate::random_to_domain_mapper::<Option<String>>(
                result.validation_override_mitigation_action,
            ),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                condition: &crate::clone::<Option<crate::types::ApiTokenCondition>>(args.condition),
                expires_on: &crate::clone::<Option<String>>(args.expires_on),
                name: &crate::clone::<String>(args.name),
                not_before: &crate::clone::<Option<String>>(args.not_before),
                policies: &crate::clone::<Vec<crate::types::ApiTokenPolicy>>(args.policies),
            },
        );

        ApiTokenResult {
            condition: crate::random_to_domain_mapper::<Option<crate::types::ApiTokenCondition>>(
                result.condition,
            ),
            expires_on: crate::random_to_domain_mapper::<Option<String>>(result.expires_on),
            issued_on: crate::random_to_domain_mapper::<String>(result.issued_on),
            modified_on: crate::random_to_domain_mapper::<String>(result.modified_on),
            name: crate::random_to_domain_mapper::<String>(result.name),
            not_before: crate::random_to_domain_mapper::<Option<String>>(result.not_before),
            policies: crate::random_to_domain_mapper::<Vec<crate::types::ApiTokenPolicy>>(
                result.policies,
            ),
            status: crate::random_to_domain_mapper::<String>(result.status),
            value: crate::random_to_domain_mapper::<String>(result.value),
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
                smart_routing: &crate::clone::<Option<String>>(args.smart_routing),
                tiered_caching: &crate::clone::<Option<String>>(args.tiered_caching),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        ArgoResult {
            smart_routing: crate::random_to_domain_mapper::<Option<String>>(result.smart_routing),
            tiered_caching: crate::random_to_domain_mapper::<Option<String>>(result.tiered_caching),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                authenticated_origin_pulls_certificate: &crate::clone::<Option<String>>(
                    args.authenticated_origin_pulls_certificate,
                ),
                enabled: &crate::clone::<bool>(args.enabled),
                hostname: &crate::clone::<Option<String>>(args.hostname),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        AuthenticatedOriginPullsResult {
            authenticated_origin_pulls_certificate: crate::random_to_domain_mapper::<Option<String>>(
                result.authenticated_origin_pulls_certificate,
            ),
            enabled: crate::random_to_domain_mapper::<bool>(result.enabled),
            hostname: crate::random_to_domain_mapper::<Option<String>>(result.hostname),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
            certificate: &crate::clone::<String>(args.certificate),
            private_key: &crate::clone::<String>(args.private_key),
            type_: &crate::clone::<String>(args.type_),
            zone_id: &crate::clone::<String>(args.zone_id),
        });

        AuthenticatedOriginPullsCertificateResult {
            certificate: crate::random_to_domain_mapper::<String>(result.certificate),
            expires_on: crate::random_to_domain_mapper::<String>(result.expires_on),
            issuer: crate::random_to_domain_mapper::<String>(result.issuer),
            private_key: crate::random_to_domain_mapper::<String>(result.private_key),
            serial_number: crate::random_to_domain_mapper::<String>(result.serial_number),
            signature: crate::random_to_domain_mapper::<String>(result.signature),
            status: crate::random_to_domain_mapper::<String>(result.status),
            type_: crate::random_to_domain_mapper::<String>(result.type_),
            uploaded_on: crate::random_to_domain_mapper::<String>(result.uploaded_on),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                auto_update_model: &crate::clone::<Option<bool>>(args.auto_update_model),
                enable_js: &crate::clone::<Option<bool>>(args.enable_js),
                fight_mode: &crate::clone::<Option<bool>>(args.fight_mode),
                optimize_wordpress: &crate::clone::<Option<bool>>(args.optimize_wordpress),
                sbfm_definitely_automated: &crate::clone::<Option<String>>(
                    args.sbfm_definitely_automated,
                ),
                sbfm_likely_automated: &crate::clone::<Option<String>>(args.sbfm_likely_automated),
                sbfm_static_resource_protection: &crate::clone::<Option<bool>>(
                    args.sbfm_static_resource_protection,
                ),
                sbfm_verified_bots: &crate::clone::<Option<String>>(args.sbfm_verified_bots),
                suppress_session_score: &crate::clone::<Option<bool>>(args.suppress_session_score),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        BotManagementResult {
            auto_update_model: crate::random_to_domain_mapper::<Option<bool>>(
                result.auto_update_model,
            ),
            enable_js: crate::random_to_domain_mapper::<Option<bool>>(result.enable_js),
            fight_mode: crate::random_to_domain_mapper::<Option<bool>>(result.fight_mode),
            optimize_wordpress: crate::random_to_domain_mapper::<Option<bool>>(
                result.optimize_wordpress,
            ),
            sbfm_definitely_automated: crate::random_to_domain_mapper::<Option<String>>(
                result.sbfm_definitely_automated,
            ),
            sbfm_likely_automated: crate::random_to_domain_mapper::<Option<String>>(
                result.sbfm_likely_automated,
            ),
            sbfm_static_resource_protection: crate::random_to_domain_mapper::<Option<bool>>(
                result.sbfm_static_resource_protection,
            ),
            sbfm_verified_bots: crate::random_to_domain_mapper::<Option<String>>(
                result.sbfm_verified_bots,
            ),
            suppress_session_score: crate::random_to_domain_mapper::<Option<bool>>(
                result.suppress_session_score,
            ),
            using_latest_model: crate::random_to_domain_mapper::<bool>(result.using_latest_model),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                account_id: &crate::clone::<String>(args.account_id),
                advertisement: &crate::clone::<Option<String>>(args.advertisement),
                description: &crate::clone::<Option<String>>(args.description),
                prefix_id: &crate::clone::<String>(args.prefix_id),
            },
        );

        ByoIpPrefixResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            advertisement: crate::random_to_domain_mapper::<String>(result.advertisement),
            description: crate::random_to_domain_mapper::<String>(result.description),
            prefix_id: crate::random_to_domain_mapper::<String>(result.prefix_id),
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
                certificate_authority: &crate::clone::<String>(args.certificate_authority),
                cloudflare_branding: &crate::clone::<Option<bool>>(args.cloudflare_branding),
                hosts: &crate::clone::<Vec<String>>(args.hosts),
                type_: &crate::clone::<String>(args.type_),
                validation_errors: &crate::clone::<
                    Option<Vec<crate::types::CertificatePackValidationError>>,
                >(args.validation_errors),
                validation_method: &crate::clone::<String>(args.validation_method),
                validation_records: &crate::clone::<
                    Option<Vec<crate::types::CertificatePackValidationRecord>>,
                >(args.validation_records),
                validity_days: &crate::clone::<i32>(args.validity_days),
                wait_for_active_status: &crate::clone::<Option<bool>>(args.wait_for_active_status),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        CertificatePackResult {
            certificate_authority: crate::random_to_domain_mapper::<String>(
                result.certificate_authority,
            ),
            cloudflare_branding: crate::random_to_domain_mapper::<Option<bool>>(
                result.cloudflare_branding,
            ),
            hosts: crate::random_to_domain_mapper::<Vec<String>>(result.hosts),
            type_: crate::random_to_domain_mapper::<String>(result.type_),
            validation_errors: crate::random_to_domain_mapper::<
                Vec<crate::types::CertificatePackValidationError>,
            >(result.validation_errors),
            validation_method: crate::random_to_domain_mapper::<String>(result.validation_method),
            validation_records: crate::random_to_domain_mapper::<
                Vec<crate::types::CertificatePackValidationRecord>,
            >(result.validation_records),
            validity_days: crate::random_to_domain_mapper::<i32>(result.validity_days),
            wait_for_active_status: crate::random_to_domain_mapper::<Option<bool>>(
                result.wait_for_active_status,
            ),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                custom_metadata: &crate::clone::<Option<std::collections::HashMap<String, String>>>(
                    args.custom_metadata,
                ),
                custom_origin_server: &crate::clone::<Option<String>>(args.custom_origin_server),
                custom_origin_sni: &crate::clone::<Option<String>>(args.custom_origin_sni),
                hostname: &crate::clone::<String>(args.hostname),
                ssls: &crate::clone::<Option<Vec<crate::types::CustomHostnameSsl>>>(args.ssls),
                wait_for_ssl_pending_validation: &crate::clone::<Option<bool>>(
                    args.wait_for_ssl_pending_validation,
                ),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        CustomHostnameResult {
            custom_metadata: crate::random_to_domain_mapper::<
                Option<std::collections::HashMap<String, String>>,
            >(result.custom_metadata),
            custom_origin_server: crate::random_to_domain_mapper::<Option<String>>(
                result.custom_origin_server,
            ),
            custom_origin_sni: crate::random_to_domain_mapper::<Option<String>>(
                result.custom_origin_sni,
            ),
            hostname: crate::random_to_domain_mapper::<String>(result.hostname),
            ownership_verification: crate::random_to_domain_mapper::<
                std::collections::HashMap<String, String>,
            >(result.ownership_verification),
            ownership_verification_http: crate::random_to_domain_mapper::<
                std::collections::HashMap<String, String>,
            >(result.ownership_verification_http),
            ssls: crate::random_to_domain_mapper::<Option<Vec<crate::types::CustomHostnameSsl>>>(
                result.ssls,
            ),
            status: crate::random_to_domain_mapper::<String>(result.status),
            wait_for_ssl_pending_validation: crate::random_to_domain_mapper::<Option<bool>>(
                result.wait_for_ssl_pending_validation,
            ),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                origin: &crate::clone::<String>(args.origin),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        CustomHostnameFallbackOriginResult {
            origin: crate::random_to_domain_mapper::<String>(result.origin),
            status: crate::random_to_domain_mapper::<String>(result.status),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                account_id: &crate::clone::<Option<String>>(args.account_id),
                state: &crate::clone::<Option<String>>(args.state),
                type_: &crate::clone::<String>(args.type_),
                url: &crate::clone::<String>(args.url),
                zone_id: &crate::clone::<Option<String>>(args.zone_id),
            },
        );

        CustomPagesResult {
            account_id: crate::random_to_domain_mapper::<Option<String>>(result.account_id),
            state: crate::random_to_domain_mapper::<Option<String>>(result.state),
            type_: crate::random_to_domain_mapper::<String>(result.type_),
            url: crate::random_to_domain_mapper::<String>(result.url),
            zone_id: crate::random_to_domain_mapper::<Option<String>>(result.zone_id),
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
                custom_ssl_options: &crate::clone::<Option<crate::types::CustomSslCustomSslOptions>>(
                    args.custom_ssl_options,
                ),
                custom_ssl_priorities: &crate::clone::<
                    Option<Vec<crate::types::CustomSslCustomSslPriority>>,
                >(args.custom_ssl_priorities),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        CustomSslResult {
            custom_ssl_options: crate::random_to_domain_mapper::<
                Option<crate::types::CustomSslCustomSslOptions>,
            >(result.custom_ssl_options),
            custom_ssl_priorities: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::CustomSslCustomSslPriority>>,
            >(result.custom_ssl_priorities),
            expires_on: crate::random_to_domain_mapper::<String>(result.expires_on),
            hosts: crate::random_to_domain_mapper::<Vec<String>>(result.hosts),
            issuer: crate::random_to_domain_mapper::<String>(result.issuer),
            modified_on: crate::random_to_domain_mapper::<String>(result.modified_on),
            priority: crate::random_to_domain_mapper::<i32>(result.priority),
            signature: crate::random_to_domain_mapper::<String>(result.signature),
            status: crate::random_to_domain_mapper::<String>(result.status),
            uploaded_on: crate::random_to_domain_mapper::<String>(result.uploaded_on),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                account_id: &crate::clone::<String>(args.account_id),
                name: &crate::clone::<String>(args.name),
            },
        );

        D1DatabaseResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            name: crate::random_to_domain_mapper::<String>(result.name),
            version: crate::random_to_domain_mapper::<String>(result.version),
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
                account_id: &crate::clone::<String>(args.account_id),
                data: &crate::clone::<crate::types::DeviceDexTestData>(args.data),
                description: &crate::clone::<String>(args.description),
                enabled: &crate::clone::<bool>(args.enabled),
                interval: &crate::clone::<String>(args.interval),
                name: &crate::clone::<String>(args.name),
            },
        );

        DeviceDexTestResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            created: crate::random_to_domain_mapper::<String>(result.created),
            data: crate::random_to_domain_mapper::<crate::types::DeviceDexTestData>(result.data),
            description: crate::random_to_domain_mapper::<String>(result.description),
            enabled: crate::random_to_domain_mapper::<bool>(result.enabled),
            interval: crate::random_to_domain_mapper::<String>(result.interval),
            name: crate::random_to_domain_mapper::<String>(result.name),
            updated: crate::random_to_domain_mapper::<String>(result.updated),
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
                account_id: &crate::clone::<String>(args.account_id),
                config: &crate::clone::<crate::types::DeviceManagedNetworksConfig>(args.config),
                name: &crate::clone::<String>(args.name),
                type_: &crate::clone::<String>(args.type_),
            },
        );

        DeviceManagedNetworksResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            config: crate::random_to_domain_mapper::<crate::types::DeviceManagedNetworksConfig>(
                result.config,
            ),
            name: crate::random_to_domain_mapper::<String>(result.name),
            type_: crate::random_to_domain_mapper::<String>(result.type_),
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
                enabled: &crate::clone::<bool>(args.enabled),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        DevicePolicyCertificatesResult {
            enabled: crate::random_to_domain_mapper::<bool>(result.enabled),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                account_id: &crate::clone::<String>(args.account_id),
                configs: &crate::clone::<Option<Vec<crate::types::DevicePostureIntegrationConfig>>>(
                    args.configs,
                ),
                identifier: &crate::clone::<Option<String>>(args.identifier),
                interval: &crate::clone::<Option<String>>(args.interval),
                name: &crate::clone::<String>(args.name),
                type_: &crate::clone::<String>(args.type_),
            },
        );

        DevicePostureIntegrationResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            configs: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::DevicePostureIntegrationConfig>>,
            >(result.configs),
            identifier: crate::random_to_domain_mapper::<Option<String>>(result.identifier),
            interval: crate::random_to_domain_mapper::<Option<String>>(result.interval),
            name: crate::random_to_domain_mapper::<String>(result.name),
            type_: crate::random_to_domain_mapper::<String>(result.type_),
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
                account_id: &crate::clone::<String>(args.account_id),
                description: &crate::clone::<Option<String>>(args.description),
                expiration: &crate::clone::<Option<String>>(args.expiration),
                inputs: &crate::clone::<Option<Vec<crate::types::DevicePostureRuleInput>>>(
                    args.inputs,
                ),
                matches: &crate::clone::<Option<Vec<crate::types::DevicePostureRuleMatch>>>(
                    args.matches,
                ),
                name: &crate::clone::<Option<String>>(args.name),
                schedule: &crate::clone::<Option<String>>(args.schedule),
                type_: &crate::clone::<String>(args.type_),
            },
        );

        DevicePostureRuleResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            description: crate::random_to_domain_mapper::<Option<String>>(result.description),
            expiration: crate::random_to_domain_mapper::<Option<String>>(result.expiration),
            inputs: crate::random_to_domain_mapper::<Vec<crate::types::DevicePostureRuleInput>>(
                result.inputs,
            ),
            matches: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::DevicePostureRuleMatch>>,
            >(result.matches),
            name: crate::random_to_domain_mapper::<Option<String>>(result.name),
            schedule: crate::random_to_domain_mapper::<Option<String>>(result.schedule),
            type_: crate::random_to_domain_mapper::<String>(result.type_),
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
                account_id: &crate::clone::<String>(args.account_id),
                allow_mode_switch: &crate::clone::<Option<bool>>(args.allow_mode_switch),
                allow_updates: &crate::clone::<Option<bool>>(args.allow_updates),
                allowed_to_leave: &crate::clone::<Option<bool>>(args.allowed_to_leave),
                auto_connect: &crate::clone::<Option<i32>>(args.auto_connect),
                captive_portal: &crate::clone::<Option<i32>>(args.captive_portal),
                default: &crate::clone::<Option<bool>>(args.default),
                description: &crate::clone::<String>(args.description),
                disable_auto_fallback: &crate::clone::<Option<bool>>(args.disable_auto_fallback),
                enabled: &crate::clone::<Option<bool>>(args.enabled),
                exclude_office_ips: &crate::clone::<Option<bool>>(args.exclude_office_ips),
                match_: &crate::clone::<Option<String>>(args.match_),
                name: &crate::clone::<String>(args.name),
                precedence: &crate::clone::<Option<i32>>(args.precedence),
                service_mode_v2_mode: &crate::clone::<Option<String>>(args.service_mode_v2_mode),
                service_mode_v2_port: &crate::clone::<Option<i32>>(args.service_mode_v2_port),
                support_url: &crate::clone::<Option<String>>(args.support_url),
                switch_locked: &crate::clone::<Option<bool>>(args.switch_locked),
            },
        );

        DeviceSettingsPolicyResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            allow_mode_switch: crate::random_to_domain_mapper::<Option<bool>>(
                result.allow_mode_switch,
            ),
            allow_updates: crate::random_to_domain_mapper::<Option<bool>>(result.allow_updates),
            allowed_to_leave: crate::random_to_domain_mapper::<Option<bool>>(
                result.allowed_to_leave,
            ),
            auto_connect: crate::random_to_domain_mapper::<Option<i32>>(result.auto_connect),
            captive_portal: crate::random_to_domain_mapper::<Option<i32>>(result.captive_portal),
            default: crate::random_to_domain_mapper::<Option<bool>>(result.default),
            description: crate::random_to_domain_mapper::<String>(result.description),
            disable_auto_fallback: crate::random_to_domain_mapper::<Option<bool>>(
                result.disable_auto_fallback,
            ),
            enabled: crate::random_to_domain_mapper::<Option<bool>>(result.enabled),
            exclude_office_ips: crate::random_to_domain_mapper::<Option<bool>>(
                result.exclude_office_ips,
            ),
            match_: crate::random_to_domain_mapper::<Option<String>>(result.match_),
            name: crate::random_to_domain_mapper::<String>(result.name),
            precedence: crate::random_to_domain_mapper::<Option<i32>>(result.precedence),
            service_mode_v2_mode: crate::random_to_domain_mapper::<Option<String>>(
                result.service_mode_v2_mode,
            ),
            service_mode_v2_port: crate::random_to_domain_mapper::<Option<i32>>(
                result.service_mode_v2_port,
            ),
            support_url: crate::random_to_domain_mapper::<Option<String>>(result.support_url),
            switch_locked: crate::random_to_domain_mapper::<Option<bool>>(result.switch_locked),
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
                account_id: &crate::clone::<String>(args.account_id),
                allowed_match_count: &crate::clone::<i32>(args.allowed_match_count),
                context_awareness: &crate::clone::<Option<crate::types::DlpProfileContextAwareness>>(
                    args.context_awareness,
                ),
                description: &crate::clone::<Option<String>>(args.description),
                entries: &crate::clone::<Vec<crate::types::DlpProfileEntry>>(args.entries),
                name: &crate::clone::<String>(args.name),
                type_: &crate::clone::<String>(args.type_),
            },
        );

        DlpProfileResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            allowed_match_count: crate::random_to_domain_mapper::<i32>(result.allowed_match_count),
            context_awareness: crate::random_to_domain_mapper::<
                crate::types::DlpProfileContextAwareness,
            >(result.context_awareness),
            description: crate::random_to_domain_mapper::<Option<String>>(result.description),
            entries: crate::random_to_domain_mapper::<Vec<crate::types::DlpProfileEntry>>(
                result.entries,
            ),
            name: crate::random_to_domain_mapper::<String>(result.name),
            type_: crate::random_to_domain_mapper::<String>(result.type_),
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
                account_id: &crate::clone::<String>(args.account_id),
                email: &crate::clone::<String>(args.email),
            },
        );

        EmailRoutingAddressResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            created: crate::random_to_domain_mapper::<String>(result.created),
            email: crate::random_to_domain_mapper::<String>(result.email),
            modified: crate::random_to_domain_mapper::<String>(result.modified),
            tag: crate::random_to_domain_mapper::<String>(result.tag),
            verified: crate::random_to_domain_mapper::<String>(result.verified),
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
                actions: &crate::clone::<Vec<crate::types::EmailRoutingCatchAllAction>>(
                    args.actions,
                ),
                enabled: &crate::clone::<Option<bool>>(args.enabled),
                matchers: &crate::clone::<Vec<crate::types::EmailRoutingCatchAllMatcher>>(
                    args.matchers,
                ),
                name: &crate::clone::<String>(args.name),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        EmailRoutingCatchAllResult {
            actions: crate::random_to_domain_mapper::<Vec<crate::types::EmailRoutingCatchAllAction>>(
                result.actions,
            ),
            enabled: crate::random_to_domain_mapper::<Option<bool>>(result.enabled),
            matchers: crate::random_to_domain_mapper::<
                Vec<crate::types::EmailRoutingCatchAllMatcher>,
            >(result.matchers),
            name: crate::random_to_domain_mapper::<String>(result.name),
            tag: crate::random_to_domain_mapper::<String>(result.tag),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                actions: &crate::clone::<Option<Vec<crate::types::EmailRoutingRuleAction>>>(
                    args.actions,
                ),
                enabled: &crate::clone::<Option<bool>>(args.enabled),
                matchers: &crate::clone::<Option<Vec<crate::types::EmailRoutingRuleMatcher>>>(
                    args.matchers,
                ),
                name: &crate::clone::<String>(args.name),
                priority: &crate::clone::<Option<i32>>(args.priority),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        EmailRoutingRuleResult {
            actions: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::EmailRoutingRuleAction>>,
            >(result.actions),
            enabled: crate::random_to_domain_mapper::<Option<bool>>(result.enabled),
            matchers: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::EmailRoutingRuleMatcher>>,
            >(result.matchers),
            name: crate::random_to_domain_mapper::<String>(result.name),
            priority: crate::random_to_domain_mapper::<i32>(result.priority),
            tag: crate::random_to_domain_mapper::<String>(result.tag),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                enabled: &crate::clone::<bool>(args.enabled),
                skip_wizard: &crate::clone::<Option<bool>>(args.skip_wizard),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        EmailRoutingSettingsResult {
            created: crate::random_to_domain_mapper::<String>(result.created),
            enabled: crate::random_to_domain_mapper::<bool>(result.enabled),
            modified: crate::random_to_domain_mapper::<String>(result.modified),
            name: crate::random_to_domain_mapper::<String>(result.name),
            skip_wizard: crate::random_to_domain_mapper::<bool>(result.skip_wizard),
            status: crate::random_to_domain_mapper::<String>(result.status),
            tag: crate::random_to_domain_mapper::<String>(result.tag),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                account_id: &crate::clone::<String>(args.account_id),
                domains: &crate::clone::<Vec<crate::types::FallbackDomainDomain>>(args.domains),
                policy_id: &crate::clone::<Option<String>>(args.policy_id),
            },
        );

        FallbackDomainResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            domains: crate::random_to_domain_mapper::<Vec<crate::types::FallbackDomainDomain>>(
                result.domains,
            ),
            policy_id: crate::random_to_domain_mapper::<Option<String>>(result.policy_id),
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
                description: &crate::clone::<Option<String>>(args.description),
                expression: &crate::clone::<String>(args.expression),
                paused: &crate::clone::<Option<bool>>(args.paused),
                ref_: &crate::clone::<Option<String>>(args.ref_),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        FilterResult {
            description: crate::random_to_domain_mapper::<Option<String>>(result.description),
            expression: crate::random_to_domain_mapper::<String>(result.expression),
            paused: crate::random_to_domain_mapper::<Option<bool>>(result.paused),
            ref_: crate::random_to_domain_mapper::<Option<String>>(result.ref_),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                action: &crate::clone::<String>(args.action),
                description: &crate::clone::<Option<String>>(args.description),
                filter_id: &crate::clone::<String>(args.filter_id),
                paused: &crate::clone::<Option<bool>>(args.paused),
                priority: &crate::clone::<Option<i32>>(args.priority),
                products: &crate::clone::<Option<Vec<String>>>(args.products),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        FirewallRuleResult {
            action: crate::random_to_domain_mapper::<String>(result.action),
            description: crate::random_to_domain_mapper::<Option<String>>(result.description),
            filter_id: crate::random_to_domain_mapper::<String>(result.filter_id),
            paused: crate::random_to_domain_mapper::<Option<bool>>(result.paused),
            priority: crate::random_to_domain_mapper::<Option<i32>>(result.priority),
            products: crate::random_to_domain_mapper::<Option<Vec<String>>>(result.products),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                account_id: &crate::clone::<Option<String>>(args.account_id),
                cloudflare_gre_endpoint: &crate::clone::<String>(args.cloudflare_gre_endpoint),
                customer_gre_endpoint: &crate::clone::<String>(args.customer_gre_endpoint),
                description: &crate::clone::<Option<String>>(args.description),
                health_check_enabled: &crate::clone::<Option<bool>>(args.health_check_enabled),
                health_check_target: &crate::clone::<Option<String>>(args.health_check_target),
                health_check_type: &crate::clone::<Option<String>>(args.health_check_type),
                interface_address: &crate::clone::<String>(args.interface_address),
                mtu: &crate::clone::<Option<i32>>(args.mtu),
                name: &crate::clone::<String>(args.name),
                ttl: &crate::clone::<Option<i32>>(args.ttl),
            },
        );

        GreTunnelResult {
            account_id: crate::random_to_domain_mapper::<Option<String>>(result.account_id),
            cloudflare_gre_endpoint: crate::random_to_domain_mapper::<String>(
                result.cloudflare_gre_endpoint,
            ),
            customer_gre_endpoint: crate::random_to_domain_mapper::<String>(
                result.customer_gre_endpoint,
            ),
            description: crate::random_to_domain_mapper::<Option<String>>(result.description),
            health_check_enabled: crate::random_to_domain_mapper::<bool>(
                result.health_check_enabled,
            ),
            health_check_target: crate::random_to_domain_mapper::<String>(
                result.health_check_target,
            ),
            health_check_type: crate::random_to_domain_mapper::<String>(result.health_check_type),
            interface_address: crate::random_to_domain_mapper::<String>(result.interface_address),
            mtu: crate::random_to_domain_mapper::<i32>(result.mtu),
            name: crate::random_to_domain_mapper::<String>(result.name),
            ttl: crate::random_to_domain_mapper::<i32>(result.ttl),
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
                address: &crate::clone::<String>(args.address),
                allow_insecure: &crate::clone::<Option<bool>>(args.allow_insecure),
                check_regions: &crate::clone::<Option<Vec<String>>>(args.check_regions),
                consecutive_fails: &crate::clone::<Option<i32>>(args.consecutive_fails),
                consecutive_successes: &crate::clone::<Option<i32>>(args.consecutive_successes),
                description: &crate::clone::<Option<String>>(args.description),
                expected_body: &crate::clone::<Option<String>>(args.expected_body),
                expected_codes: &crate::clone::<Option<Vec<String>>>(args.expected_codes),
                follow_redirects: &crate::clone::<Option<bool>>(args.follow_redirects),
                headers: &crate::clone::<Option<Vec<crate::types::HealthcheckHeader>>>(
                    args.headers,
                ),
                interval: &crate::clone::<Option<i32>>(args.interval),
                method: &crate::clone::<Option<String>>(args.method),
                name: &crate::clone::<String>(args.name),
                path: &crate::clone::<Option<String>>(args.path),
                port: &crate::clone::<Option<i32>>(args.port),
                retries: &crate::clone::<Option<i32>>(args.retries),
                suspended: &crate::clone::<Option<bool>>(args.suspended),
                timeout: &crate::clone::<Option<i32>>(args.timeout),
                type_: &crate::clone::<String>(args.type_),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        HealthcheckResult {
            address: crate::random_to_domain_mapper::<String>(result.address),
            allow_insecure: crate::random_to_domain_mapper::<Option<bool>>(result.allow_insecure),
            check_regions: crate::random_to_domain_mapper::<Vec<String>>(result.check_regions),
            consecutive_fails: crate::random_to_domain_mapper::<Option<i32>>(
                result.consecutive_fails,
            ),
            consecutive_successes: crate::random_to_domain_mapper::<Option<i32>>(
                result.consecutive_successes,
            ),
            created_on: crate::random_to_domain_mapper::<String>(result.created_on),
            description: crate::random_to_domain_mapper::<Option<String>>(result.description),
            expected_body: crate::random_to_domain_mapper::<Option<String>>(result.expected_body),
            expected_codes: crate::random_to_domain_mapper::<Option<Vec<String>>>(
                result.expected_codes,
            ),
            follow_redirects: crate::random_to_domain_mapper::<Option<bool>>(
                result.follow_redirects,
            ),
            headers: crate::random_to_domain_mapper::<Option<Vec<crate::types::HealthcheckHeader>>>(
                result.headers,
            ),
            interval: crate::random_to_domain_mapper::<Option<i32>>(result.interval),
            method: crate::random_to_domain_mapper::<String>(result.method),
            modified_on: crate::random_to_domain_mapper::<String>(result.modified_on),
            name: crate::random_to_domain_mapper::<String>(result.name),
            path: crate::random_to_domain_mapper::<Option<String>>(result.path),
            port: crate::random_to_domain_mapper::<Option<i32>>(result.port),
            retries: crate::random_to_domain_mapper::<Option<i32>>(result.retries),
            suspended: crate::random_to_domain_mapper::<Option<bool>>(result.suspended),
            timeout: crate::random_to_domain_mapper::<Option<i32>>(result.timeout),
            type_: crate::random_to_domain_mapper::<String>(result.type_),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                hostname: &crate::clone::<String>(args.hostname),
                setting: &crate::clone::<String>(args.setting),
                value: &crate::clone::<String>(args.value),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        HostnameTlsSettingResult {
            created_at: crate::random_to_domain_mapper::<String>(result.created_at),
            hostname: crate::random_to_domain_mapper::<String>(result.hostname),
            setting: crate::random_to_domain_mapper::<String>(result.setting),
            updated_at: crate::random_to_domain_mapper::<String>(result.updated_at),
            value: crate::random_to_domain_mapper::<String>(result.value),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                hostname: &crate::clone::<String>(args.hostname),
                ports: &crate::clone::<Option<Vec<i32>>>(args.ports),
                values: &crate::clone::<Vec<String>>(args.values),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        HostnameTlsSettingCiphersResult {
            created_at: crate::random_to_domain_mapper::<String>(result.created_at),
            hostname: crate::random_to_domain_mapper::<String>(result.hostname),
            ports: crate::random_to_domain_mapper::<Option<Vec<i32>>>(result.ports),
            updated_at: crate::random_to_domain_mapper::<String>(result.updated_at),
            values: crate::random_to_domain_mapper::<Vec<String>>(result.values),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                account_id: &crate::clone::<String>(args.account_id),
                caching: &crate::clone::<Option<crate::types::HyperdriveConfigCaching>>(
                    args.caching,
                ),
                name: &crate::clone::<String>(args.name),
                origin: &crate::clone::<crate::types::HyperdriveConfigOrigin>(args.origin),
            },
        );

        HyperdriveConfigResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            caching: crate::random_to_domain_mapper::<crate::types::HyperdriveConfigCaching>(
                result.caching,
            ),
            name: crate::random_to_domain_mapper::<String>(result.name),
            origin: crate::random_to_domain_mapper::<crate::types::HyperdriveConfigOrigin>(
                result.origin,
            ),
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
                account_id: &crate::clone::<Option<String>>(args.account_id),
                allow_null_cipher: &crate::clone::<Option<bool>>(args.allow_null_cipher),
                cloudflare_endpoint: &crate::clone::<String>(args.cloudflare_endpoint),
                customer_endpoint: &crate::clone::<String>(args.customer_endpoint),
                description: &crate::clone::<Option<String>>(args.description),
                fqdn_id: &crate::clone::<Option<String>>(args.fqdn_id),
                health_check_direction: &crate::clone::<Option<String>>(
                    args.health_check_direction,
                ),
                health_check_enabled: &crate::clone::<Option<bool>>(args.health_check_enabled),
                health_check_rate: &crate::clone::<Option<String>>(args.health_check_rate),
                health_check_target: &crate::clone::<Option<String>>(args.health_check_target),
                health_check_type: &crate::clone::<Option<String>>(args.health_check_type),
                hex_id: &crate::clone::<Option<String>>(args.hex_id),
                interface_address: &crate::clone::<String>(args.interface_address),
                name: &crate::clone::<String>(args.name),
                psk: &crate::clone::<Option<String>>(args.psk),
                remote_id: &crate::clone::<Option<String>>(args.remote_id),
                user_id: &crate::clone::<Option<String>>(args.user_id),
            },
        );

        IpsecTunnelResult {
            account_id: crate::random_to_domain_mapper::<Option<String>>(result.account_id),
            allow_null_cipher: crate::random_to_domain_mapper::<Option<bool>>(
                result.allow_null_cipher,
            ),
            cloudflare_endpoint: crate::random_to_domain_mapper::<String>(
                result.cloudflare_endpoint,
            ),
            customer_endpoint: crate::random_to_domain_mapper::<String>(result.customer_endpoint),
            description: crate::random_to_domain_mapper::<Option<String>>(result.description),
            fqdn_id: crate::random_to_domain_mapper::<String>(result.fqdn_id),
            health_check_direction: crate::random_to_domain_mapper::<String>(
                result.health_check_direction,
            ),
            health_check_enabled: crate::random_to_domain_mapper::<bool>(
                result.health_check_enabled,
            ),
            health_check_rate: crate::random_to_domain_mapper::<String>(result.health_check_rate),
            health_check_target: crate::random_to_domain_mapper::<String>(
                result.health_check_target,
            ),
            health_check_type: crate::random_to_domain_mapper::<String>(result.health_check_type),
            hex_id: crate::random_to_domain_mapper::<String>(result.hex_id),
            interface_address: crate::random_to_domain_mapper::<String>(result.interface_address),
            name: crate::random_to_domain_mapper::<String>(result.name),
            psk: crate::random_to_domain_mapper::<String>(result.psk),
            remote_id: crate::random_to_domain_mapper::<String>(result.remote_id),
            user_id: crate::random_to_domain_mapper::<String>(result.user_id),
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
                bundle_method: &crate::clone::<Option<String>>(args.bundle_method),
                certificate: &crate::clone::<String>(args.certificate),
                enabled: &crate::clone::<Option<bool>>(args.enabled),
                host: &crate::clone::<String>(args.host),
                name: &crate::clone::<Option<String>>(args.name),
                port: &crate::clone::<Option<i32>>(args.port),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        KeylessCertificateResult {
            bundle_method: crate::random_to_domain_mapper::<Option<String>>(result.bundle_method),
            certificate: crate::random_to_domain_mapper::<String>(result.certificate),
            enabled: crate::random_to_domain_mapper::<Option<bool>>(result.enabled),
            host: crate::random_to_domain_mapper::<String>(result.host),
            name: crate::random_to_domain_mapper::<Option<String>>(result.name),
            port: crate::random_to_domain_mapper::<Option<i32>>(result.port),
            status: crate::random_to_domain_mapper::<String>(result.status),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                account_id: &crate::clone::<String>(args.account_id),
                description: &crate::clone::<Option<String>>(args.description),
                items: &crate::clone::<Option<Vec<crate::types::ListItem>>>(args.items),
                kind: &crate::clone::<String>(args.kind),
                name: &crate::clone::<String>(args.name),
            },
        );

        ListResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            description: crate::random_to_domain_mapper::<Option<String>>(result.description),
            items: crate::random_to_domain_mapper::<Option<Vec<crate::types::ListItem>>>(
                result.items,
            ),
            kind: crate::random_to_domain_mapper::<String>(result.kind),
            name: crate::random_to_domain_mapper::<String>(result.name),
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
                account_id: &crate::clone::<String>(args.account_id),
                asn: &crate::clone::<Option<i32>>(args.asn),
                comment: &crate::clone::<Option<String>>(args.comment),
                hostname: &crate::clone::<Option<crate::types::ListItemHostname>>(args.hostname),
                ip: &crate::clone::<Option<String>>(args.ip),
                list_id: &crate::clone::<String>(args.list_id),
                redirect: &crate::clone::<Option<crate::types::ListItemRedirect>>(args.redirect),
            },
        );

        ListItemResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            asn: crate::random_to_domain_mapper::<Option<i32>>(result.asn),
            comment: crate::random_to_domain_mapper::<Option<String>>(result.comment),
            hostname: crate::random_to_domain_mapper::<Option<crate::types::ListItemHostname>>(
                result.hostname,
            ),
            ip: crate::random_to_domain_mapper::<Option<String>>(result.ip),
            list_id: crate::random_to_domain_mapper::<String>(result.list_id),
            redirect: crate::random_to_domain_mapper::<Option<crate::types::ListItemRedirect>>(
                result.redirect,
            ),
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
                adaptive_routings: &crate::clone::<
                    Option<Vec<crate::types::LoadBalancerAdaptiveRouting>>,
                >(args.adaptive_routings),
                country_pools: &crate::clone::<Option<Vec<crate::types::LoadBalancerCountryPool>>>(
                    args.country_pools,
                ),
                default_pool_ids: &crate::clone::<Vec<String>>(args.default_pool_ids),
                description: &crate::clone::<Option<String>>(args.description),
                enabled: &crate::clone::<Option<bool>>(args.enabled),
                fallback_pool_id: &crate::clone::<String>(args.fallback_pool_id),
                location_strategies: &crate::clone::<
                    Option<Vec<crate::types::LoadBalancerLocationStrategy>>,
                >(args.location_strategies),
                name: &crate::clone::<String>(args.name),
                pop_pools: &crate::clone::<Option<Vec<crate::types::LoadBalancerPopPool>>>(
                    args.pop_pools,
                ),
                proxied: &crate::clone::<Option<bool>>(args.proxied),
                random_steerings: &crate::clone::<
                    Option<Vec<crate::types::LoadBalancerRandomSteering>>,
                >(args.random_steerings),
                region_pools: &crate::clone::<Option<Vec<crate::types::LoadBalancerRegionPool>>>(
                    args.region_pools,
                ),
                rules: &crate::clone::<Option<Vec<crate::types::LoadBalancerRule>>>(args.rules),
                session_affinity: &crate::clone::<Option<String>>(args.session_affinity),
                session_affinity_attributes: &crate::clone::<
                    Option<Vec<crate::types::LoadBalancerSessionAffinityAttribute>>,
                >(args.session_affinity_attributes),
                session_affinity_ttl: &crate::clone::<Option<i32>>(args.session_affinity_ttl),
                steering_policy: &crate::clone::<Option<String>>(args.steering_policy),
                ttl: &crate::clone::<Option<i32>>(args.ttl),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        LoadBalancerResult {
            adaptive_routings: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::LoadBalancerAdaptiveRouting>>,
            >(result.adaptive_routings),
            country_pools: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::LoadBalancerCountryPool>>,
            >(result.country_pools),
            created_on: crate::random_to_domain_mapper::<String>(result.created_on),
            default_pool_ids: crate::random_to_domain_mapper::<Vec<String>>(
                result.default_pool_ids,
            ),
            description: crate::random_to_domain_mapper::<Option<String>>(result.description),
            enabled: crate::random_to_domain_mapper::<Option<bool>>(result.enabled),
            fallback_pool_id: crate::random_to_domain_mapper::<String>(result.fallback_pool_id),
            location_strategies: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::LoadBalancerLocationStrategy>>,
            >(result.location_strategies),
            modified_on: crate::random_to_domain_mapper::<String>(result.modified_on),
            name: crate::random_to_domain_mapper::<String>(result.name),
            pop_pools: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::LoadBalancerPopPool>>,
            >(result.pop_pools),
            proxied: crate::random_to_domain_mapper::<Option<bool>>(result.proxied),
            random_steerings: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::LoadBalancerRandomSteering>>,
            >(result.random_steerings),
            region_pools: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::LoadBalancerRegionPool>>,
            >(result.region_pools),
            rules: crate::random_to_domain_mapper::<Option<Vec<crate::types::LoadBalancerRule>>>(
                result.rules,
            ),
            session_affinity: crate::random_to_domain_mapper::<Option<String>>(
                result.session_affinity,
            ),
            session_affinity_attributes: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::LoadBalancerSessionAffinityAttribute>>,
            >(result.session_affinity_attributes),
            session_affinity_ttl: crate::random_to_domain_mapper::<Option<i32>>(
                result.session_affinity_ttl,
            ),
            steering_policy: crate::random_to_domain_mapper::<String>(result.steering_policy),
            ttl: crate::random_to_domain_mapper::<i32>(result.ttl),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                account_id: &crate::clone::<String>(args.account_id),
                allow_insecure: &crate::clone::<Option<bool>>(args.allow_insecure),
                consecutive_down: &crate::clone::<Option<i32>>(args.consecutive_down),
                consecutive_up: &crate::clone::<Option<i32>>(args.consecutive_up),
                description: &crate::clone::<Option<String>>(args.description),
                expected_body: &crate::clone::<Option<String>>(args.expected_body),
                expected_codes: &crate::clone::<Option<String>>(args.expected_codes),
                follow_redirects: &crate::clone::<Option<bool>>(args.follow_redirects),
                headers: &crate::clone::<Option<Vec<crate::types::LoadBalancerMonitorHeader>>>(
                    args.headers,
                ),
                interval: &crate::clone::<Option<i32>>(args.interval),
                method: &crate::clone::<Option<String>>(args.method),
                path: &crate::clone::<Option<String>>(args.path),
                port: &crate::clone::<Option<i32>>(args.port),
                probe_zone: &crate::clone::<Option<String>>(args.probe_zone),
                retries: &crate::clone::<Option<i32>>(args.retries),
                timeout: &crate::clone::<Option<i32>>(args.timeout),
                type_: &crate::clone::<Option<String>>(args.type_),
            },
        );

        LoadBalancerMonitorResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            allow_insecure: crate::random_to_domain_mapper::<Option<bool>>(result.allow_insecure),
            consecutive_down: crate::random_to_domain_mapper::<Option<i32>>(
                result.consecutive_down,
            ),
            consecutive_up: crate::random_to_domain_mapper::<Option<i32>>(result.consecutive_up),
            created_on: crate::random_to_domain_mapper::<String>(result.created_on),
            description: crate::random_to_domain_mapper::<Option<String>>(result.description),
            expected_body: crate::random_to_domain_mapper::<Option<String>>(result.expected_body),
            expected_codes: crate::random_to_domain_mapper::<Option<String>>(result.expected_codes),
            follow_redirects: crate::random_to_domain_mapper::<Option<bool>>(
                result.follow_redirects,
            ),
            headers: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::LoadBalancerMonitorHeader>>,
            >(result.headers),
            interval: crate::random_to_domain_mapper::<Option<i32>>(result.interval),
            method: crate::random_to_domain_mapper::<String>(result.method),
            modified_on: crate::random_to_domain_mapper::<String>(result.modified_on),
            path: crate::random_to_domain_mapper::<String>(result.path),
            port: crate::random_to_domain_mapper::<Option<i32>>(result.port),
            probe_zone: crate::random_to_domain_mapper::<Option<String>>(result.probe_zone),
            retries: crate::random_to_domain_mapper::<Option<i32>>(result.retries),
            timeout: crate::random_to_domain_mapper::<Option<i32>>(result.timeout),
            type_: crate::random_to_domain_mapper::<Option<String>>(result.type_),
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
                account_id: &crate::clone::<String>(args.account_id),
                check_regions: &crate::clone::<Option<Vec<String>>>(args.check_regions),
                description: &crate::clone::<Option<String>>(args.description),
                enabled: &crate::clone::<Option<bool>>(args.enabled),
                latitude: &crate::clone::<Option<f64>>(args.latitude),
                load_sheddings: &crate::clone::<
                    Option<Vec<crate::types::LoadBalancerPoolLoadShedding>>,
                >(args.load_sheddings),
                longitude: &crate::clone::<Option<f64>>(args.longitude),
                minimum_origins: &crate::clone::<Option<i32>>(args.minimum_origins),
                monitor: &crate::clone::<Option<String>>(args.monitor),
                name: &crate::clone::<String>(args.name),
                notification_email: &crate::clone::<Option<String>>(args.notification_email),
                origin_steerings: &crate::clone::<
                    Option<Vec<crate::types::LoadBalancerPoolOriginSteering>>,
                >(args.origin_steerings),
                origins: &crate::clone::<Vec<crate::types::LoadBalancerPoolOrigin>>(args.origins),
            },
        );

        LoadBalancerPoolResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            check_regions: crate::random_to_domain_mapper::<Vec<String>>(result.check_regions),
            created_on: crate::random_to_domain_mapper::<String>(result.created_on),
            description: crate::random_to_domain_mapper::<Option<String>>(result.description),
            enabled: crate::random_to_domain_mapper::<Option<bool>>(result.enabled),
            latitude: crate::random_to_domain_mapper::<Option<f64>>(result.latitude),
            load_sheddings: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::LoadBalancerPoolLoadShedding>>,
            >(result.load_sheddings),
            longitude: crate::random_to_domain_mapper::<Option<f64>>(result.longitude),
            minimum_origins: crate::random_to_domain_mapper::<Option<i32>>(result.minimum_origins),
            modified_on: crate::random_to_domain_mapper::<String>(result.modified_on),
            monitor: crate::random_to_domain_mapper::<Option<String>>(result.monitor),
            name: crate::random_to_domain_mapper::<String>(result.name),
            notification_email: crate::random_to_domain_mapper::<Option<String>>(
                result.notification_email,
            ),
            origin_steerings: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::LoadBalancerPoolOriginSteering>>,
            >(result.origin_steerings),
            origins: crate::random_to_domain_mapper::<Vec<crate::types::LoadBalancerPoolOrigin>>(
                result.origins,
            ),
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
                enabled: &crate::clone::<bool>(args.enabled),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        LogpullRetentionResult {
            enabled: crate::random_to_domain_mapper::<bool>(result.enabled),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                account_id: &crate::clone::<Option<String>>(args.account_id),
                dataset: &crate::clone::<String>(args.dataset),
                destination_conf: &crate::clone::<String>(args.destination_conf),
                enabled: &crate::clone::<Option<bool>>(args.enabled),
                filter: &crate::clone::<Option<String>>(args.filter),
                frequency: &crate::clone::<Option<String>>(args.frequency),
                kind: &crate::clone::<Option<String>>(args.kind),
                logpull_options: &crate::clone::<Option<String>>(args.logpull_options),
                max_upload_bytes: &crate::clone::<Option<i32>>(args.max_upload_bytes),
                max_upload_interval_seconds: &crate::clone::<Option<i32>>(
                    args.max_upload_interval_seconds,
                ),
                max_upload_records: &crate::clone::<Option<i32>>(args.max_upload_records),
                name: &crate::clone::<Option<String>>(args.name),
                output_options: &crate::clone::<Option<crate::types::LogpushJobOutputOptions>>(
                    args.output_options,
                ),
                ownership_challenge: &crate::clone::<Option<String>>(args.ownership_challenge),
                zone_id: &crate::clone::<Option<String>>(args.zone_id),
            },
        );

        LogpushJobResult {
            account_id: crate::random_to_domain_mapper::<Option<String>>(result.account_id),
            dataset: crate::random_to_domain_mapper::<String>(result.dataset),
            destination_conf: crate::random_to_domain_mapper::<String>(result.destination_conf),
            enabled: crate::random_to_domain_mapper::<Option<bool>>(result.enabled),
            filter: crate::random_to_domain_mapper::<Option<String>>(result.filter),
            frequency: crate::random_to_domain_mapper::<Option<String>>(result.frequency),
            kind: crate::random_to_domain_mapper::<Option<String>>(result.kind),
            logpull_options: crate::random_to_domain_mapper::<Option<String>>(
                result.logpull_options,
            ),
            max_upload_bytes: crate::random_to_domain_mapper::<Option<i32>>(
                result.max_upload_bytes,
            ),
            max_upload_interval_seconds: crate::random_to_domain_mapper::<Option<i32>>(
                result.max_upload_interval_seconds,
            ),
            max_upload_records: crate::random_to_domain_mapper::<Option<i32>>(
                result.max_upload_records,
            ),
            name: crate::random_to_domain_mapper::<Option<String>>(result.name),
            output_options: crate::random_to_domain_mapper::<
                Option<crate::types::LogpushJobOutputOptions>,
            >(result.output_options),
            ownership_challenge: crate::random_to_domain_mapper::<Option<String>>(
                result.ownership_challenge,
            ),
            zone_id: crate::random_to_domain_mapper::<Option<String>>(result.zone_id),
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
                account_id: &crate::clone::<Option<String>>(args.account_id),
                destination_conf: &crate::clone::<String>(args.destination_conf),
                zone_id: &crate::clone::<Option<String>>(args.zone_id),
            },
        );

        LogpushOwnershipChallengeResult {
            account_id: crate::random_to_domain_mapper::<Option<String>>(result.account_id),
            destination_conf: crate::random_to_domain_mapper::<String>(result.destination_conf),
            ownership_challenge_filename: crate::random_to_domain_mapper::<String>(
                result.ownership_challenge_filename,
            ),
            zone_id: crate::random_to_domain_mapper::<Option<String>>(result.zone_id),
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
                account_id: &crate::clone::<String>(args.account_id),
                description: &crate::clone::<Option<String>>(args.description),
                name: &crate::clone::<String>(args.name),
                rules: &crate::clone::<Option<Vec<std::collections::HashMap<String, String>>>>(
                    args.rules,
                ),
            },
        );

        MagicFirewallRulesetResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            description: crate::random_to_domain_mapper::<Option<String>>(result.description),
            name: crate::random_to_domain_mapper::<String>(result.name),
            rules: crate::random_to_domain_mapper::<
                Option<Vec<std::collections::HashMap<String, String>>>,
            >(result.rules),
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
                managed_request_headers: &crate::clone::<
                    Option<Vec<crate::types::ManagedHeadersManagedRequestHeader>>,
                >(args.managed_request_headers),
                managed_response_headers: &crate::clone::<
                    Option<Vec<crate::types::ManagedHeadersManagedResponseHeader>>,
                >(args.managed_response_headers),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        ManagedHeadersResult {
            managed_request_headers: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::ManagedHeadersManagedRequestHeader>>,
            >(result.managed_request_headers),
            managed_response_headers: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::ManagedHeadersManagedResponseHeader>>,
            >(result.managed_response_headers),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                account_id: &crate::clone::<String>(args.account_id),
                ca: &crate::clone::<bool>(args.ca),
                certificates: &crate::clone::<String>(args.certificates),
                name: &crate::clone::<Option<String>>(args.name),
                private_key: &crate::clone::<Option<String>>(args.private_key),
            },
        );

        MtlsCertificateResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            ca: crate::random_to_domain_mapper::<bool>(result.ca),
            certificates: crate::random_to_domain_mapper::<String>(result.certificates),
            expires_on: crate::random_to_domain_mapper::<String>(result.expires_on),
            issuer: crate::random_to_domain_mapper::<String>(result.issuer),
            name: crate::random_to_domain_mapper::<Option<String>>(result.name),
            private_key: crate::random_to_domain_mapper::<Option<String>>(result.private_key),
            serial_number: crate::random_to_domain_mapper::<String>(result.serial_number),
            signature: crate::random_to_domain_mapper::<String>(result.signature),
            uploaded_on: crate::random_to_domain_mapper::<String>(result.uploaded_on),
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
                account_id: &crate::clone::<String>(args.account_id),
                alert_type: &crate::clone::<String>(args.alert_type),
                description: &crate::clone::<Option<String>>(args.description),
                email_integrations: &crate::clone::<
                    Option<Vec<crate::types::NotificationPolicyEmailIntegration>>,
                >(args.email_integrations),
                enabled: &crate::clone::<bool>(args.enabled),
                filters: &crate::clone::<Option<crate::types::NotificationPolicyFilters>>(
                    args.filters,
                ),
                name: &crate::clone::<String>(args.name),
                pagerduty_integrations: &crate::clone::<
                    Option<Vec<crate::types::NotificationPolicyPagerdutyIntegration>>,
                >(args.pagerduty_integrations),
                webhooks_integrations: &crate::clone::<
                    Option<Vec<crate::types::NotificationPolicyWebhooksIntegration>>,
                >(args.webhooks_integrations),
            },
        );

        NotificationPolicyResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            alert_type: crate::random_to_domain_mapper::<String>(result.alert_type),
            created: crate::random_to_domain_mapper::<String>(result.created),
            description: crate::random_to_domain_mapper::<Option<String>>(result.description),
            email_integrations: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::NotificationPolicyEmailIntegration>>,
            >(result.email_integrations),
            enabled: crate::random_to_domain_mapper::<bool>(result.enabled),
            filters: crate::random_to_domain_mapper::<
                Option<crate::types::NotificationPolicyFilters>,
            >(result.filters),
            modified: crate::random_to_domain_mapper::<String>(result.modified),
            name: crate::random_to_domain_mapper::<String>(result.name),
            pagerduty_integrations: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::NotificationPolicyPagerdutyIntegration>>,
            >(result.pagerduty_integrations),
            webhooks_integrations: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::NotificationPolicyWebhooksIntegration>>,
            >(result.webhooks_integrations),
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
                account_id: &crate::clone::<String>(args.account_id),
                name: &crate::clone::<String>(args.name),
                secret: &crate::clone::<Option<String>>(args.secret),
                url: &crate::clone::<Option<String>>(args.url),
            },
        );

        NotificationPolicyWebhooksResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            created_at: crate::random_to_domain_mapper::<String>(result.created_at),
            last_failure: crate::random_to_domain_mapper::<String>(result.last_failure),
            last_success: crate::random_to_domain_mapper::<String>(result.last_success),
            name: crate::random_to_domain_mapper::<String>(result.name),
            secret: crate::random_to_domain_mapper::<Option<String>>(result.secret),
            type_: crate::random_to_domain_mapper::<String>(result.type_),
            url: crate::random_to_domain_mapper::<Option<String>>(result.url),
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
                frequency: &crate::clone::<String>(args.frequency),
                region: &crate::clone::<String>(args.region),
                url: &crate::clone::<String>(args.url),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        ObservatoryScheduledTestResult {
            frequency: crate::random_to_domain_mapper::<String>(result.frequency),
            region: crate::random_to_domain_mapper::<String>(result.region),
            url: crate::random_to_domain_mapper::<String>(result.url),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                csr: &crate::clone::<String>(args.csr),
                hostnames: &crate::clone::<Vec<String>>(args.hostnames),
                min_days_for_renewal: &crate::clone::<Option<i32>>(args.min_days_for_renewal),
                request_type: &crate::clone::<String>(args.request_type),
                requested_validity: &crate::clone::<Option<i32>>(args.requested_validity),
            },
        );

        OriginCaCertificateResult {
            certificate: crate::random_to_domain_mapper::<String>(result.certificate),
            csr: crate::random_to_domain_mapper::<String>(result.csr),
            expires_on: crate::random_to_domain_mapper::<String>(result.expires_on),
            hostnames: crate::random_to_domain_mapper::<Vec<String>>(result.hostnames),
            min_days_for_renewal: crate::random_to_domain_mapper::<Option<i32>>(
                result.min_days_for_renewal,
            ),
            request_type: crate::random_to_domain_mapper::<String>(result.request_type),
            requested_validity: crate::random_to_domain_mapper::<i32>(result.requested_validity),
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
                actions: &crate::clone::<crate::types::PageRuleActions>(args.actions),
                priority: &crate::clone::<Option<i32>>(args.priority),
                status: &crate::clone::<Option<String>>(args.status),
                target: &crate::clone::<String>(args.target),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        PageRuleResult {
            actions: crate::random_to_domain_mapper::<crate::types::PageRuleActions>(
                result.actions,
            ),
            priority: crate::random_to_domain_mapper::<Option<i32>>(result.priority),
            status: crate::random_to_domain_mapper::<Option<String>>(result.status),
            target: crate::random_to_domain_mapper::<String>(result.target),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                account_id: &crate::clone::<String>(args.account_id),
                domain: &crate::clone::<String>(args.domain),
                project_name: &crate::clone::<String>(args.project_name),
            },
        );

        PagesDomainResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            domain: crate::random_to_domain_mapper::<String>(result.domain),
            project_name: crate::random_to_domain_mapper::<String>(result.project_name),
            status: crate::random_to_domain_mapper::<String>(result.status),
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
                account_id: &crate::clone::<String>(args.account_id),
                build_config: &crate::clone::<Option<crate::types::PagesProjectBuildConfig>>(
                    args.build_config,
                ),
                deployment_configs: &crate::clone::<
                    Option<crate::types::PagesProjectDeploymentConfigs>,
                >(args.deployment_configs),
                name: &crate::clone::<String>(args.name),
                production_branch: &crate::clone::<String>(args.production_branch),
                source: &crate::clone::<Option<crate::types::PagesProjectSource>>(args.source),
            },
        );

        PagesProjectResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            build_config: crate::random_to_domain_mapper::<
                Option<crate::types::PagesProjectBuildConfig>,
            >(result.build_config),
            created_on: crate::random_to_domain_mapper::<String>(result.created_on),
            deployment_configs: crate::random_to_domain_mapper::<
                crate::types::PagesProjectDeploymentConfigs,
            >(result.deployment_configs),
            domains: crate::random_to_domain_mapper::<Vec<String>>(result.domains),
            name: crate::random_to_domain_mapper::<String>(result.name),
            production_branch: crate::random_to_domain_mapper::<String>(result.production_branch),
            source: crate::random_to_domain_mapper::<Option<crate::types::PagesProjectSource>>(
                result.source,
            ),
            subdomain: crate::random_to_domain_mapper::<String>(result.subdomain),
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
                account_id: &crate::clone::<String>(args.account_id),
                name: &crate::clone::<String>(args.name),
            },
        );

        QueueResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            name: crate::random_to_domain_mapper::<String>(result.name),
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
                account_id: &crate::clone::<String>(args.account_id),
                location: &crate::clone::<Option<String>>(args.location),
                name: &crate::clone::<String>(args.name),
            },
        );

        R2BucketResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            location: crate::random_to_domain_mapper::<String>(result.location),
            name: crate::random_to_domain_mapper::<String>(result.name),
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
                action: &crate::clone::<crate::types::RateLimitAction>(args.action),
                bypass_url_patterns: &crate::clone::<Option<Vec<String>>>(args.bypass_url_patterns),
                correlate: &crate::clone::<Option<crate::types::RateLimitCorrelate>>(
                    args.correlate,
                ),
                description: &crate::clone::<Option<String>>(args.description),
                disabled: &crate::clone::<Option<bool>>(args.disabled),
                match_: &crate::clone::<Option<crate::types::RateLimitMatch>>(args.match_),
                period: &crate::clone::<i32>(args.period),
                threshold: &crate::clone::<i32>(args.threshold),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        RateLimitResult {
            action: crate::random_to_domain_mapper::<crate::types::RateLimitAction>(result.action),
            bypass_url_patterns: crate::random_to_domain_mapper::<Option<Vec<String>>>(
                result.bypass_url_patterns,
            ),
            correlate: crate::random_to_domain_mapper::<Option<crate::types::RateLimitCorrelate>>(
                result.correlate,
            ),
            description: crate::random_to_domain_mapper::<Option<String>>(result.description),
            disabled: crate::random_to_domain_mapper::<Option<bool>>(result.disabled),
            match_: crate::random_to_domain_mapper::<crate::types::RateLimitMatch>(result.match_),
            period: crate::random_to_domain_mapper::<i32>(result.period),
            threshold: crate::random_to_domain_mapper::<i32>(result.threshold),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                allow_overwrite: &crate::clone::<Option<bool>>(args.allow_overwrite),
                comment: &crate::clone::<Option<String>>(args.comment),
                data: &crate::clone::<Option<crate::types::RecordData>>(args.data),
                name: &crate::clone::<String>(args.name),
                priority: &crate::clone::<Option<i32>>(args.priority),
                proxied: &crate::clone::<Option<bool>>(args.proxied),
                tags: &crate::clone::<Option<Vec<String>>>(args.tags),
                ttl: &crate::clone::<Option<i32>>(args.ttl),
                type_: &crate::clone::<String>(args.type_),
                value: &crate::clone::<Option<String>>(args.value),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        RecordResult {
            allow_overwrite: crate::random_to_domain_mapper::<Option<bool>>(result.allow_overwrite),
            comment: crate::random_to_domain_mapper::<Option<String>>(result.comment),
            created_on: crate::random_to_domain_mapper::<String>(result.created_on),
            data: crate::random_to_domain_mapper::<Option<crate::types::RecordData>>(result.data),
            hostname: crate::random_to_domain_mapper::<String>(result.hostname),
            metadata: crate::random_to_domain_mapper::<std::collections::HashMap<String, String>>(
                result.metadata,
            ),
            modified_on: crate::random_to_domain_mapper::<String>(result.modified_on),
            name: crate::random_to_domain_mapper::<String>(result.name),
            priority: crate::random_to_domain_mapper::<Option<i32>>(result.priority),
            proxiable: crate::random_to_domain_mapper::<bool>(result.proxiable),
            proxied: crate::random_to_domain_mapper::<Option<bool>>(result.proxied),
            tags: crate::random_to_domain_mapper::<Option<Vec<String>>>(result.tags),
            ttl: crate::random_to_domain_mapper::<i32>(result.ttl),
            type_: crate::random_to_domain_mapper::<String>(result.type_),
            value: crate::random_to_domain_mapper::<String>(result.value),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                hostname: &crate::clone::<String>(args.hostname),
                region_key: &crate::clone::<String>(args.region_key),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        RegionalHostnameResult {
            created_on: crate::random_to_domain_mapper::<String>(result.created_on),
            hostname: crate::random_to_domain_mapper::<String>(result.hostname),
            region_key: crate::random_to_domain_mapper::<String>(result.region_key),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                value: &crate::clone::<String>(args.value),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        RegionalTieredCacheResult {
            value: crate::random_to_domain_mapper::<String>(result.value),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                account_id: &crate::clone::<Option<String>>(args.account_id),
                description: &crate::clone::<Option<String>>(args.description),
                kind: &crate::clone::<String>(args.kind),
                name: &crate::clone::<String>(args.name),
                phase: &crate::clone::<String>(args.phase),
                rules: &crate::clone::<Option<Vec<crate::types::RulesetRule>>>(args.rules),
                zone_id: &crate::clone::<Option<String>>(args.zone_id),
            },
        );

        RulesetResult {
            account_id: crate::random_to_domain_mapper::<Option<String>>(result.account_id),
            description: crate::random_to_domain_mapper::<String>(result.description),
            kind: crate::random_to_domain_mapper::<String>(result.kind),
            name: crate::random_to_domain_mapper::<String>(result.name),
            phase: crate::random_to_domain_mapper::<String>(result.phase),
            rules: crate::random_to_domain_mapper::<Option<Vec<crate::types::RulesetRule>>>(
                result.rules,
            ),
            zone_id: crate::random_to_domain_mapper::<Option<String>>(result.zone_id),
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
                argo_smart_routing: &crate::clone::<Option<bool>>(args.argo_smart_routing),
                dns: &crate::clone::<crate::types::SpectrumApplicationDns>(args.dns),
                edge_ips: &crate::clone::<Option<crate::types::SpectrumApplicationEdgeIps>>(
                    args.edge_ips,
                ),
                ip_firewall: &crate::clone::<Option<bool>>(args.ip_firewall),
                origin_directs: &crate::clone::<Option<Vec<String>>>(args.origin_directs),
                origin_dns: &crate::clone::<Option<crate::types::SpectrumApplicationOriginDns>>(
                    args.origin_dns,
                ),
                origin_port: &crate::clone::<Option<i32>>(args.origin_port),
                origin_port_range: &crate::clone::<
                    Option<crate::types::SpectrumApplicationOriginPortRange>,
                >(args.origin_port_range),
                protocol: &crate::clone::<String>(args.protocol),
                proxy_protocol: &crate::clone::<Option<String>>(args.proxy_protocol),
                tls: &crate::clone::<Option<String>>(args.tls),
                traffic_type: &crate::clone::<Option<String>>(args.traffic_type),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        SpectrumApplicationResult {
            argo_smart_routing: crate::random_to_domain_mapper::<bool>(result.argo_smart_routing),
            dns: crate::random_to_domain_mapper::<crate::types::SpectrumApplicationDns>(result.dns),
            edge_ips: crate::random_to_domain_mapper::<crate::types::SpectrumApplicationEdgeIps>(
                result.edge_ips,
            ),
            ip_firewall: crate::random_to_domain_mapper::<bool>(result.ip_firewall),
            origin_directs: crate::random_to_domain_mapper::<Option<Vec<String>>>(
                result.origin_directs,
            ),
            origin_dns: crate::random_to_domain_mapper::<
                Option<crate::types::SpectrumApplicationOriginDns>,
            >(result.origin_dns),
            origin_port: crate::random_to_domain_mapper::<Option<i32>>(result.origin_port),
            origin_port_range: crate::random_to_domain_mapper::<
                Option<crate::types::SpectrumApplicationOriginPortRange>,
            >(result.origin_port_range),
            protocol: crate::random_to_domain_mapper::<String>(result.protocol),
            proxy_protocol: crate::random_to_domain_mapper::<String>(result.proxy_protocol),
            tls: crate::random_to_domain_mapper::<String>(result.tls),
            traffic_type: crate::random_to_domain_mapper::<String>(result.traffic_type),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                account_id: &crate::clone::<String>(args.account_id),
                mode: &crate::clone::<String>(args.mode),
                policy_id: &crate::clone::<Option<String>>(args.policy_id),
                tunnels: &crate::clone::<Vec<crate::types::SplitTunnelTunnel>>(args.tunnels),
            },
        );

        SplitTunnelResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            mode: crate::random_to_domain_mapper::<String>(result.mode),
            policy_id: crate::random_to_domain_mapper::<Option<String>>(result.policy_id),
            tunnels: crate::random_to_domain_mapper::<Vec<crate::types::SplitTunnelTunnel>>(
                result.tunnels,
            ),
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
                account_id: &crate::clone::<Option<String>>(args.account_id),
                colo_names: &crate::clone::<Option<Vec<String>>>(args.colo_names),
                colo_regions: &crate::clone::<Option<Vec<String>>>(args.colo_regions),
                description: &crate::clone::<Option<String>>(args.description),
                nexthop: &crate::clone::<String>(args.nexthop),
                prefix: &crate::clone::<String>(args.prefix),
                priority: &crate::clone::<i32>(args.priority),
                weight: &crate::clone::<Option<i32>>(args.weight),
            },
        );

        StaticRouteResult {
            account_id: crate::random_to_domain_mapper::<Option<String>>(result.account_id),
            colo_names: crate::random_to_domain_mapper::<Option<Vec<String>>>(result.colo_names),
            colo_regions: crate::random_to_domain_mapper::<Option<Vec<String>>>(
                result.colo_regions,
            ),
            description: crate::random_to_domain_mapper::<Option<String>>(result.description),
            nexthop: crate::random_to_domain_mapper::<String>(result.nexthop),
            prefix: crate::random_to_domain_mapper::<String>(result.prefix),
            priority: crate::random_to_domain_mapper::<i32>(result.priority),
            weight: crate::random_to_domain_mapper::<Option<i32>>(result.weight),
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
                account_id: &crate::clone::<String>(args.account_id),
                activity_log_enabled: &crate::clone::<Option<bool>>(args.activity_log_enabled),
                antivirus: &crate::clone::<Option<crate::types::TeamsAccountAntivirus>>(
                    args.antivirus,
                ),
                block_page: &crate::clone::<Option<crate::types::TeamsAccountBlockPage>>(
                    args.block_page,
                ),
                body_scanning: &crate::clone::<Option<crate::types::TeamsAccountBodyScanning>>(
                    args.body_scanning,
                ),
                extended_email_matching: &crate::clone::<
                    Option<crate::types::TeamsAccountExtendedEmailMatching>,
                >(args.extended_email_matching),
                fips: &crate::clone::<Option<crate::types::TeamsAccountFips>>(args.fips),
                logging: &crate::clone::<Option<crate::types::TeamsAccountLogging>>(args.logging),
                non_identity_browser_isolation_enabled: &crate::clone::<Option<bool>>(
                    args.non_identity_browser_isolation_enabled,
                ),
                payload_log: &crate::clone::<Option<crate::types::TeamsAccountPayloadLog>>(
                    args.payload_log,
                ),
                protocol_detection_enabled: &crate::clone::<Option<bool>>(
                    args.protocol_detection_enabled,
                ),
                proxy: &crate::clone::<Option<crate::types::TeamsAccountProxy>>(args.proxy),
                ssh_session_log: &crate::clone::<Option<crate::types::TeamsAccountSshSessionLog>>(
                    args.ssh_session_log,
                ),
                tls_decrypt_enabled: &crate::clone::<Option<bool>>(args.tls_decrypt_enabled),
                url_browser_isolation_enabled: &crate::clone::<Option<bool>>(
                    args.url_browser_isolation_enabled,
                ),
            },
        );

        TeamsAccountResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            activity_log_enabled: crate::random_to_domain_mapper::<Option<bool>>(
                result.activity_log_enabled,
            ),
            antivirus: crate::random_to_domain_mapper::<Option<crate::types::TeamsAccountAntivirus>>(
                result.antivirus,
            ),
            block_page: crate::random_to_domain_mapper::<Option<crate::types::TeamsAccountBlockPage>>(
                result.block_page,
            ),
            body_scanning: crate::random_to_domain_mapper::<
                Option<crate::types::TeamsAccountBodyScanning>,
            >(result.body_scanning),
            extended_email_matching: crate::random_to_domain_mapper::<
                Option<crate::types::TeamsAccountExtendedEmailMatching>,
            >(result.extended_email_matching),
            fips: crate::random_to_domain_mapper::<Option<crate::types::TeamsAccountFips>>(
                result.fips,
            ),
            logging: crate::random_to_domain_mapper::<Option<crate::types::TeamsAccountLogging>>(
                result.logging,
            ),
            non_identity_browser_isolation_enabled: crate::random_to_domain_mapper::<Option<bool>>(
                result.non_identity_browser_isolation_enabled,
            ),
            payload_log: crate::random_to_domain_mapper::<
                Option<crate::types::TeamsAccountPayloadLog>,
            >(result.payload_log),
            protocol_detection_enabled: crate::random_to_domain_mapper::<Option<bool>>(
                result.protocol_detection_enabled,
            ),
            proxy: crate::random_to_domain_mapper::<Option<crate::types::TeamsAccountProxy>>(
                result.proxy,
            ),
            ssh_session_log: crate::random_to_domain_mapper::<
                Option<crate::types::TeamsAccountSshSessionLog>,
            >(result.ssh_session_log),
            tls_decrypt_enabled: crate::random_to_domain_mapper::<Option<bool>>(
                result.tls_decrypt_enabled,
            ),
            url_browser_isolation_enabled: crate::random_to_domain_mapper::<Option<bool>>(
                result.url_browser_isolation_enabled,
            ),
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
                account_id: &crate::clone::<String>(args.account_id),
                description: &crate::clone::<Option<String>>(args.description),
                items: &crate::clone::<Option<Vec<String>>>(args.items),
                name: &crate::clone::<String>(args.name),
                type_: &crate::clone::<String>(args.type_),
            },
        );

        TeamsListResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            description: crate::random_to_domain_mapper::<Option<String>>(result.description),
            items: crate::random_to_domain_mapper::<Option<Vec<String>>>(result.items),
            name: crate::random_to_domain_mapper::<String>(result.name),
            type_: crate::random_to_domain_mapper::<String>(result.type_),
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
                account_id: &crate::clone::<String>(args.account_id),
                client_default: &crate::clone::<Option<bool>>(args.client_default),
                name: &crate::clone::<String>(args.name),
                networks: &crate::clone::<Option<Vec<crate::types::TeamsLocationNetwork>>>(
                    args.networks,
                ),
            },
        );

        TeamsLocationResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            anonymized_logs_enabled: crate::random_to_domain_mapper::<bool>(
                result.anonymized_logs_enabled,
            ),
            client_default: crate::random_to_domain_mapper::<Option<bool>>(result.client_default),
            doh_subdomain: crate::random_to_domain_mapper::<String>(result.doh_subdomain),
            ip: crate::random_to_domain_mapper::<String>(result.ip),
            ipv4_destination: crate::random_to_domain_mapper::<String>(result.ipv4_destination),
            name: crate::random_to_domain_mapper::<String>(result.name),
            networks: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::TeamsLocationNetwork>>,
            >(result.networks),
            policy_ids: crate::random_to_domain_mapper::<Vec<String>>(result.policy_ids),
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
                account_id: &crate::clone::<String>(args.account_id),
                ips: &crate::clone::<Vec<String>>(args.ips),
                name: &crate::clone::<String>(args.name),
            },
        );

        TeamsProxyEndpointResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            ips: crate::random_to_domain_mapper::<Vec<String>>(result.ips),
            name: crate::random_to_domain_mapper::<String>(result.name),
            subdomain: crate::random_to_domain_mapper::<String>(result.subdomain),
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
                account_id: &crate::clone::<String>(args.account_id),
                action: &crate::clone::<String>(args.action),
                description: &crate::clone::<String>(args.description),
                device_posture: &crate::clone::<Option<String>>(args.device_posture),
                enabled: &crate::clone::<Option<bool>>(args.enabled),
                filters: &crate::clone::<Option<Vec<String>>>(args.filters),
                identity: &crate::clone::<Option<String>>(args.identity),
                name: &crate::clone::<String>(args.name),
                precedence: &crate::clone::<i32>(args.precedence),
                rule_settings: &crate::clone::<Option<crate::types::TeamsRuleRuleSettings>>(
                    args.rule_settings,
                ),
                traffic: &crate::clone::<Option<String>>(args.traffic),
            },
        );

        TeamsRuleResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            action: crate::random_to_domain_mapper::<String>(result.action),
            description: crate::random_to_domain_mapper::<String>(result.description),
            device_posture: crate::random_to_domain_mapper::<Option<String>>(result.device_posture),
            enabled: crate::random_to_domain_mapper::<Option<bool>>(result.enabled),
            filters: crate::random_to_domain_mapper::<Option<Vec<String>>>(result.filters),
            identity: crate::random_to_domain_mapper::<Option<String>>(result.identity),
            name: crate::random_to_domain_mapper::<String>(result.name),
            precedence: crate::random_to_domain_mapper::<i32>(result.precedence),
            rule_settings: crate::random_to_domain_mapper::<
                Option<crate::types::TeamsRuleRuleSettings>,
            >(result.rule_settings),
            traffic: crate::random_to_domain_mapper::<Option<String>>(result.traffic),
            version: crate::random_to_domain_mapper::<i32>(result.version),
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
                cache_type: &crate::clone::<String>(args.cache_type),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        TieredCacheResult {
            cache_type: crate::random_to_domain_mapper::<String>(result.cache_type),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                certificate_authority: &crate::clone::<Option<String>>(args.certificate_authority),
                enabled: &crate::clone::<bool>(args.enabled),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        TotalTlsResult {
            certificate_authority: crate::random_to_domain_mapper::<Option<String>>(
                result.certificate_authority,
            ),
            enabled: crate::random_to_domain_mapper::<bool>(result.enabled),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                account_id: &crate::clone::<String>(args.account_id),
                config_src: &crate::clone::<Option<String>>(args.config_src),
                name: &crate::clone::<String>(args.name),
                secret: &crate::clone::<String>(args.secret),
            },
        );

        TunnelResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            cname: crate::random_to_domain_mapper::<String>(result.cname),
            config_src: crate::random_to_domain_mapper::<Option<String>>(result.config_src),
            name: crate::random_to_domain_mapper::<String>(result.name),
            secret: crate::random_to_domain_mapper::<String>(result.secret),
            tunnel_token: crate::random_to_domain_mapper::<String>(result.tunnel_token),
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
                account_id: &crate::clone::<String>(args.account_id),
                config: &crate::clone::<crate::types::TunnelConfigConfig>(args.config),
                tunnel_id: &crate::clone::<String>(args.tunnel_id),
            },
        );

        TunnelConfigResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            config: crate::random_to_domain_mapper::<crate::types::TunnelConfigConfig>(
                result.config,
            ),
            tunnel_id: crate::random_to_domain_mapper::<String>(result.tunnel_id),
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
                account_id: &crate::clone::<String>(args.account_id),
                comment: &crate::clone::<Option<String>>(args.comment),
                network: &crate::clone::<String>(args.network),
                tunnel_id: &crate::clone::<String>(args.tunnel_id),
                virtual_network_id: &crate::clone::<Option<String>>(args.virtual_network_id),
            },
        );

        TunnelRouteResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            comment: crate::random_to_domain_mapper::<Option<String>>(result.comment),
            network: crate::random_to_domain_mapper::<String>(result.network),
            tunnel_id: crate::random_to_domain_mapper::<String>(result.tunnel_id),
            virtual_network_id: crate::random_to_domain_mapper::<Option<String>>(
                result.virtual_network_id,
            ),
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
                account_id: &crate::clone::<String>(args.account_id),
                comment: &crate::clone::<Option<String>>(args.comment),
                is_default_network: &crate::clone::<Option<bool>>(args.is_default_network),
                name: &crate::clone::<String>(args.name),
            },
        );

        TunnelVirtualNetworkResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            comment: crate::random_to_domain_mapper::<Option<String>>(result.comment),
            is_default_network: crate::random_to_domain_mapper::<Option<bool>>(
                result.is_default_network,
            ),
            name: crate::random_to_domain_mapper::<String>(result.name),
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
                account_id: &crate::clone::<String>(args.account_id),
                bot_fight_mode: &crate::clone::<Option<bool>>(args.bot_fight_mode),
                domains: &crate::clone::<Vec<String>>(args.domains),
                mode: &crate::clone::<String>(args.mode),
                name: &crate::clone::<String>(args.name),
                offlabel: &crate::clone::<Option<bool>>(args.offlabel),
                region: &crate::clone::<Option<String>>(args.region),
            },
        );

        TurnstileWidgetResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            bot_fight_mode: crate::random_to_domain_mapper::<bool>(result.bot_fight_mode),
            domains: crate::random_to_domain_mapper::<Vec<String>>(result.domains),
            mode: crate::random_to_domain_mapper::<String>(result.mode),
            name: crate::random_to_domain_mapper::<String>(result.name),
            offlabel: crate::random_to_domain_mapper::<bool>(result.offlabel),
            region: crate::random_to_domain_mapper::<String>(result.region),
            secret: crate::random_to_domain_mapper::<String>(result.secret),
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
                scope: &crate::clone::<String>(args.scope),
                type_: &crate::clone::<String>(args.type_),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        UrlNormalizationSettingsResult {
            scope: crate::random_to_domain_mapper::<String>(result.scope),
            type_: crate::random_to_domain_mapper::<String>(result.type_),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                configuration: &crate::clone::<crate::types::UserAgentBlockingRuleConfiguration>(
                    args.configuration,
                ),
                description: &crate::clone::<String>(args.description),
                mode: &crate::clone::<String>(args.mode),
                paused: &crate::clone::<bool>(args.paused),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        UserAgentBlockingRuleResult {
            configuration: crate::random_to_domain_mapper::<
                crate::types::UserAgentBlockingRuleConfiguration,
            >(result.configuration),
            description: crate::random_to_domain_mapper::<String>(result.description),
            mode: crate::random_to_domain_mapper::<String>(result.mode),
            paused: crate::random_to_domain_mapper::<bool>(result.paused),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                additional_routes: &crate::clone::<
                    Option<Vec<crate::types::WaitingRoomAdditionalRoute>>,
                >(args.additional_routes),
                cookie_suffix: &crate::clone::<Option<String>>(args.cookie_suffix),
                custom_page_html: &crate::clone::<Option<String>>(args.custom_page_html),
                default_template_language: &crate::clone::<Option<String>>(
                    args.default_template_language,
                ),
                description: &crate::clone::<Option<String>>(args.description),
                disable_session_renewal: &crate::clone::<Option<bool>>(
                    args.disable_session_renewal,
                ),
                host: &crate::clone::<String>(args.host),
                json_response_enabled: &crate::clone::<Option<bool>>(args.json_response_enabled),
                name: &crate::clone::<String>(args.name),
                new_users_per_minute: &crate::clone::<i32>(args.new_users_per_minute),
                path: &crate::clone::<Option<String>>(args.path),
                queue_all: &crate::clone::<Option<bool>>(args.queue_all),
                queueing_method: &crate::clone::<Option<String>>(args.queueing_method),
                queueing_status_code: &crate::clone::<Option<i32>>(args.queueing_status_code),
                session_duration: &crate::clone::<Option<i32>>(args.session_duration),
                suspended: &crate::clone::<Option<bool>>(args.suspended),
                total_active_users: &crate::clone::<i32>(args.total_active_users),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        WaitingRoomResult {
            additional_routes: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::WaitingRoomAdditionalRoute>>,
            >(result.additional_routes),
            cookie_suffix: crate::random_to_domain_mapper::<Option<String>>(result.cookie_suffix),
            custom_page_html: crate::random_to_domain_mapper::<Option<String>>(
                result.custom_page_html,
            ),
            default_template_language: crate::random_to_domain_mapper::<Option<String>>(
                result.default_template_language,
            ),
            description: crate::random_to_domain_mapper::<Option<String>>(result.description),
            disable_session_renewal: crate::random_to_domain_mapper::<Option<bool>>(
                result.disable_session_renewal,
            ),
            host: crate::random_to_domain_mapper::<String>(result.host),
            json_response_enabled: crate::random_to_domain_mapper::<Option<bool>>(
                result.json_response_enabled,
            ),
            name: crate::random_to_domain_mapper::<String>(result.name),
            new_users_per_minute: crate::random_to_domain_mapper::<i32>(
                result.new_users_per_minute,
            ),
            path: crate::random_to_domain_mapper::<Option<String>>(result.path),
            queue_all: crate::random_to_domain_mapper::<Option<bool>>(result.queue_all),
            queueing_method: crate::random_to_domain_mapper::<Option<String>>(
                result.queueing_method,
            ),
            queueing_status_code: crate::random_to_domain_mapper::<Option<i32>>(
                result.queueing_status_code,
            ),
            session_duration: crate::random_to_domain_mapper::<Option<i32>>(
                result.session_duration,
            ),
            suspended: crate::random_to_domain_mapper::<Option<bool>>(result.suspended),
            total_active_users: crate::random_to_domain_mapper::<i32>(result.total_active_users),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                custom_page_html: &crate::clone::<Option<String>>(args.custom_page_html),
                description: &crate::clone::<Option<String>>(args.description),
                disable_session_renewal: &crate::clone::<Option<bool>>(
                    args.disable_session_renewal,
                ),
                event_end_time: &crate::clone::<String>(args.event_end_time),
                event_start_time: &crate::clone::<String>(args.event_start_time),
                name: &crate::clone::<String>(args.name),
                new_users_per_minute: &crate::clone::<Option<i32>>(args.new_users_per_minute),
                prequeue_start_time: &crate::clone::<Option<String>>(args.prequeue_start_time),
                queueing_method: &crate::clone::<Option<String>>(args.queueing_method),
                session_duration: &crate::clone::<Option<i32>>(args.session_duration),
                shuffle_at_event_start: &crate::clone::<Option<bool>>(args.shuffle_at_event_start),
                suspended: &crate::clone::<Option<bool>>(args.suspended),
                total_active_users: &crate::clone::<Option<i32>>(args.total_active_users),
                waiting_room_id: &crate::clone::<String>(args.waiting_room_id),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        WaitingRoomEventResult {
            created_on: crate::random_to_domain_mapper::<String>(result.created_on),
            custom_page_html: crate::random_to_domain_mapper::<Option<String>>(
                result.custom_page_html,
            ),
            description: crate::random_to_domain_mapper::<Option<String>>(result.description),
            disable_session_renewal: crate::random_to_domain_mapper::<Option<bool>>(
                result.disable_session_renewal,
            ),
            event_end_time: crate::random_to_domain_mapper::<String>(result.event_end_time),
            event_start_time: crate::random_to_domain_mapper::<String>(result.event_start_time),
            modified_on: crate::random_to_domain_mapper::<String>(result.modified_on),
            name: crate::random_to_domain_mapper::<String>(result.name),
            new_users_per_minute: crate::random_to_domain_mapper::<Option<i32>>(
                result.new_users_per_minute,
            ),
            prequeue_start_time: crate::random_to_domain_mapper::<Option<String>>(
                result.prequeue_start_time,
            ),
            queueing_method: crate::random_to_domain_mapper::<Option<String>>(
                result.queueing_method,
            ),
            session_duration: crate::random_to_domain_mapper::<Option<i32>>(
                result.session_duration,
            ),
            shuffle_at_event_start: crate::random_to_domain_mapper::<Option<bool>>(
                result.shuffle_at_event_start,
            ),
            suspended: crate::random_to_domain_mapper::<Option<bool>>(result.suspended),
            total_active_users: crate::random_to_domain_mapper::<Option<i32>>(
                result.total_active_users,
            ),
            waiting_room_id: crate::random_to_domain_mapper::<String>(result.waiting_room_id),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                rules: &crate::clone::<Option<Vec<crate::types::WaitingRoomRulesRule>>>(args.rules),
                waiting_room_id: &crate::clone::<String>(args.waiting_room_id),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        WaitingRoomRulesResult {
            rules: crate::random_to_domain_mapper::<Option<Vec<crate::types::WaitingRoomRulesRule>>>(
                result.rules,
            ),
            waiting_room_id: crate::random_to_domain_mapper::<String>(result.waiting_room_id),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                search_engine_crawler_bypass: &crate::clone::<Option<bool>>(
                    args.search_engine_crawler_bypass,
                ),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        WaitingRoomSettingsResult {
            search_engine_crawler_bypass: crate::random_to_domain_mapper::<Option<bool>>(
                result.search_engine_crawler_bypass,
            ),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                description: &crate::clone::<Option<String>>(args.description),
                dnslink: &crate::clone::<Option<String>>(args.dnslink),
                name: &crate::clone::<String>(args.name),
                target: &crate::clone::<String>(args.target),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        Web3HostnameResult {
            created_on: crate::random_to_domain_mapper::<String>(result.created_on),
            description: crate::random_to_domain_mapper::<Option<String>>(result.description),
            dnslink: crate::random_to_domain_mapper::<Option<String>>(result.dnslink),
            modified_on: crate::random_to_domain_mapper::<String>(result.modified_on),
            name: crate::random_to_domain_mapper::<String>(result.name),
            status: crate::random_to_domain_mapper::<String>(result.status),
            target: crate::random_to_domain_mapper::<String>(result.target),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                account_id: &crate::clone::<String>(args.account_id),
                host: &crate::clone::<String>(args.host),
                inclusive: &crate::clone::<bool>(args.inclusive),
                is_paused: &crate::clone::<bool>(args.is_paused),
                paths: &crate::clone::<Vec<String>>(args.paths),
                ruleset_id: &crate::clone::<String>(args.ruleset_id),
            },
        );

        WebAnalyticsRuleResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            host: crate::random_to_domain_mapper::<String>(result.host),
            inclusive: crate::random_to_domain_mapper::<bool>(result.inclusive),
            is_paused: crate::random_to_domain_mapper::<bool>(result.is_paused),
            paths: crate::random_to_domain_mapper::<Vec<String>>(result.paths),
            ruleset_id: crate::random_to_domain_mapper::<String>(result.ruleset_id),
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
                account_id: &crate::clone::<String>(args.account_id),
                auto_install: &crate::clone::<bool>(args.auto_install),
                host: &crate::clone::<Option<String>>(args.host),
                zone_tag: &crate::clone::<Option<String>>(args.zone_tag),
            },
        );

        WebAnalyticsSiteResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            auto_install: crate::random_to_domain_mapper::<bool>(result.auto_install),
            host: crate::random_to_domain_mapper::<Option<String>>(result.host),
            ruleset_id: crate::random_to_domain_mapper::<String>(result.ruleset_id),
            site_tag: crate::random_to_domain_mapper::<String>(result.site_tag),
            site_token: crate::random_to_domain_mapper::<String>(result.site_token),
            snippet: crate::random_to_domain_mapper::<String>(result.snippet),
            zone_tag: crate::random_to_domain_mapper::<Option<String>>(result.zone_tag),
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
                account_id: &crate::clone::<String>(args.account_id),
                schedules: &crate::clone::<Vec<String>>(args.schedules),
                script_name: &crate::clone::<String>(args.script_name),
            },
        );

        WorkerCronTriggerResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            schedules: crate::random_to_domain_mapper::<Vec<String>>(result.schedules),
            script_name: crate::random_to_domain_mapper::<String>(result.script_name),
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
                account_id: &crate::clone::<String>(args.account_id),
                environment: &crate::clone::<Option<String>>(args.environment),
                hostname: &crate::clone::<String>(args.hostname),
                service: &crate::clone::<String>(args.service),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        WorkerDomainResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            environment: crate::random_to_domain_mapper::<Option<String>>(result.environment),
            hostname: crate::random_to_domain_mapper::<String>(result.hostname),
            service: crate::random_to_domain_mapper::<String>(result.service),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                pattern: &crate::clone::<String>(args.pattern),
                script_name: &crate::clone::<Option<String>>(args.script_name),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        WorkerRouteResult {
            pattern: crate::random_to_domain_mapper::<String>(result.pattern),
            script_name: crate::random_to_domain_mapper::<Option<String>>(result.script_name),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                account_id: &crate::clone::<String>(args.account_id),
                analytics_engine_bindings: &crate::clone::<
                    Option<Vec<crate::types::WorkerScriptAnalyticsEngineBinding>>,
                >(args.analytics_engine_bindings),
                compatibility_date: &crate::clone::<Option<String>>(args.compatibility_date),
                compatibility_flags: &crate::clone::<Option<Vec<String>>>(args.compatibility_flags),
                content: &crate::clone::<String>(args.content),
                d1_database_bindings: &crate::clone::<
                    Option<Vec<crate::types::WorkerScriptD1DatabaseBinding>>,
                >(args.d1_database_bindings),
                dispatch_namespace: &crate::clone::<Option<String>>(args.dispatch_namespace),
                kv_namespace_bindings: &crate::clone::<
                    Option<Vec<crate::types::WorkerScriptKvNamespaceBinding>>,
                >(args.kv_namespace_bindings),
                logpush: &crate::clone::<Option<bool>>(args.logpush),
                module: &crate::clone::<Option<bool>>(args.module),
                name: &crate::clone::<String>(args.name),
                placements: &crate::clone::<Option<Vec<crate::types::WorkerScriptPlacement>>>(
                    args.placements,
                ),
                plain_text_bindings: &crate::clone::<
                    Option<Vec<crate::types::WorkerScriptPlainTextBinding>>,
                >(args.plain_text_bindings),
                queue_bindings: &crate::clone::<Option<Vec<crate::types::WorkerScriptQueueBinding>>>(
                    args.queue_bindings,
                ),
                r2_bucket_bindings: &crate::clone::<
                    Option<Vec<crate::types::WorkerScriptR2BucketBinding>>,
                >(args.r2_bucket_bindings),
                secret_text_bindings: &crate::clone::<
                    Option<Vec<crate::types::WorkerScriptSecretTextBinding>>,
                >(args.secret_text_bindings),
                service_bindings: &crate::clone::<
                    Option<Vec<crate::types::WorkerScriptServiceBinding>>,
                >(args.service_bindings),
                tags: &crate::clone::<Option<Vec<String>>>(args.tags),
                webassembly_bindings: &crate::clone::<
                    Option<Vec<crate::types::WorkerScriptWebassemblyBinding>>,
                >(args.webassembly_bindings),
            },
        );

        WorkerScriptResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            analytics_engine_bindings: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::WorkerScriptAnalyticsEngineBinding>>,
            >(result.analytics_engine_bindings),
            compatibility_date: crate::random_to_domain_mapper::<Option<String>>(
                result.compatibility_date,
            ),
            compatibility_flags: crate::random_to_domain_mapper::<Vec<String>>(
                result.compatibility_flags,
            ),
            content: crate::random_to_domain_mapper::<String>(result.content),
            d1_database_bindings: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::WorkerScriptD1DatabaseBinding>>,
            >(result.d1_database_bindings),
            dispatch_namespace: crate::random_to_domain_mapper::<Option<String>>(
                result.dispatch_namespace,
            ),
            kv_namespace_bindings: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::WorkerScriptKvNamespaceBinding>>,
            >(result.kv_namespace_bindings),
            logpush: crate::random_to_domain_mapper::<Option<bool>>(result.logpush),
            module: crate::random_to_domain_mapper::<Option<bool>>(result.module),
            name: crate::random_to_domain_mapper::<String>(result.name),
            placements: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::WorkerScriptPlacement>>,
            >(result.placements),
            plain_text_bindings: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::WorkerScriptPlainTextBinding>>,
            >(result.plain_text_bindings),
            queue_bindings: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::WorkerScriptQueueBinding>>,
            >(result.queue_bindings),
            r2_bucket_bindings: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::WorkerScriptR2BucketBinding>>,
            >(result.r2_bucket_bindings),
            secret_text_bindings: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::WorkerScriptSecretTextBinding>>,
            >(result.secret_text_bindings),
            service_bindings: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::WorkerScriptServiceBinding>>,
            >(result.service_bindings),
            tags: crate::random_to_domain_mapper::<Vec<String>>(result.tags),
            webassembly_bindings: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::WorkerScriptWebassemblyBinding>>,
            >(result.webassembly_bindings),
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
                account_id: &crate::clone::<String>(args.account_id),
                name: &crate::clone::<String>(args.name),
                script_name: &crate::clone::<String>(args.script_name),
                secret_text: &crate::clone::<String>(args.secret_text),
            },
        );

        WorkerSecretResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            name: crate::random_to_domain_mapper::<String>(result.name),
            script_name: crate::random_to_domain_mapper::<String>(result.script_name),
            secret_text: crate::random_to_domain_mapper::<String>(result.secret_text),
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
                account_id: &crate::clone::<String>(args.account_id),
                name: &crate::clone::<String>(args.name),
            },
        );

        WorkersForPlatformsNamespaceResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            name: crate::random_to_domain_mapper::<String>(result.name),
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
                account_id: &crate::clone::<String>(args.account_id),
                key: &crate::clone::<String>(args.key),
                namespace_id: &crate::clone::<String>(args.namespace_id),
                value: &crate::clone::<String>(args.value),
            },
        );

        WorkersKvResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            key: crate::random_to_domain_mapper::<String>(result.key),
            namespace_id: crate::random_to_domain_mapper::<String>(result.namespace_id),
            value: crate::random_to_domain_mapper::<String>(result.value),
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
                account_id: &crate::clone::<String>(args.account_id),
                title: &crate::clone::<String>(args.title),
            },
        );

        WorkersKvNamespaceResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            title: crate::random_to_domain_mapper::<String>(result.title),
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
                account_id: &crate::clone::<String>(args.account_id),
                jump_start: &crate::clone::<Option<bool>>(args.jump_start),
                paused: &crate::clone::<Option<bool>>(args.paused),
                plan: &crate::clone::<Option<String>>(args.plan),
                type_: &crate::clone::<Option<String>>(args.type_),
                zone: &crate::clone::<String>(args.zone),
            },
        );

        ZoneResult {
            account_id: crate::random_to_domain_mapper::<String>(result.account_id),
            jump_start: crate::random_to_domain_mapper::<Option<bool>>(result.jump_start),
            meta: crate::random_to_domain_mapper::<std::collections::HashMap<String, bool>>(
                result.meta,
            ),
            name_servers: crate::random_to_domain_mapper::<Vec<String>>(result.name_servers),
            paused: crate::random_to_domain_mapper::<Option<bool>>(result.paused),
            plan: crate::random_to_domain_mapper::<String>(result.plan),
            status: crate::random_to_domain_mapper::<String>(result.status),
            type_: crate::random_to_domain_mapper::<Option<String>>(result.type_),
            vanity_name_servers: crate::random_to_domain_mapper::<Vec<String>>(
                result.vanity_name_servers,
            ),
            verification_key: crate::random_to_domain_mapper::<String>(result.verification_key),
            zone: crate::random_to_domain_mapper::<String>(result.zone),
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
                enabled: &crate::clone::<bool>(args.enabled),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        ZoneCacheReserveResult {
            enabled: crate::random_to_domain_mapper::<bool>(result.enabled),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                avifs: &crate::clone::<Option<Vec<String>>>(args.avifs),
                bmps: &crate::clone::<Option<Vec<String>>>(args.bmps),
                gifs: &crate::clone::<Option<Vec<String>>>(args.gifs),
                jp2s: &crate::clone::<Option<Vec<String>>>(args.jp2s),
                jpegs: &crate::clone::<Option<Vec<String>>>(args.jpegs),
                jpg2s: &crate::clone::<Option<Vec<String>>>(args.jpg2s),
                jpgs: &crate::clone::<Option<Vec<String>>>(args.jpgs),
                pngs: &crate::clone::<Option<Vec<String>>>(args.pngs),
                tiffs: &crate::clone::<Option<Vec<String>>>(args.tiffs),
                tifs: &crate::clone::<Option<Vec<String>>>(args.tifs),
                webps: &crate::clone::<Option<Vec<String>>>(args.webps),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        ZoneCacheVariantsResult {
            avifs: crate::random_to_domain_mapper::<Option<Vec<String>>>(result.avifs),
            bmps: crate::random_to_domain_mapper::<Option<Vec<String>>>(result.bmps),
            gifs: crate::random_to_domain_mapper::<Option<Vec<String>>>(result.gifs),
            jp2s: crate::random_to_domain_mapper::<Option<Vec<String>>>(result.jp2s),
            jpegs: crate::random_to_domain_mapper::<Option<Vec<String>>>(result.jpegs),
            jpg2s: crate::random_to_domain_mapper::<Option<Vec<String>>>(result.jpg2s),
            jpgs: crate::random_to_domain_mapper::<Option<Vec<String>>>(result.jpgs),
            pngs: crate::random_to_domain_mapper::<Option<Vec<String>>>(result.pngs),
            tiffs: crate::random_to_domain_mapper::<Option<Vec<String>>>(result.tiffs),
            tifs: crate::random_to_domain_mapper::<Option<Vec<String>>>(result.tifs),
            webps: crate::random_to_domain_mapper::<Option<Vec<String>>>(result.webps),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                modified_on: &crate::clone::<Option<String>>(args.modified_on),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        ZoneDnssecResult {
            algorithm: crate::random_to_domain_mapper::<String>(result.algorithm),
            digest: crate::random_to_domain_mapper::<String>(result.digest),
            digest_algorithm: crate::random_to_domain_mapper::<String>(result.digest_algorithm),
            digest_type: crate::random_to_domain_mapper::<String>(result.digest_type),
            ds: crate::random_to_domain_mapper::<String>(result.ds),
            flags: crate::random_to_domain_mapper::<i32>(result.flags),
            key_tag: crate::random_to_domain_mapper::<i32>(result.key_tag),
            key_type: crate::random_to_domain_mapper::<String>(result.key_type),
            modified_on: crate::random_to_domain_mapper::<String>(result.modified_on),
            public_key: crate::random_to_domain_mapper::<String>(result.public_key),
            status: crate::random_to_domain_mapper::<String>(result.status),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                hold: &crate::clone::<bool>(args.hold),
                hold_after: &crate::clone::<Option<String>>(args.hold_after),
                include_subdomains: &crate::clone::<Option<bool>>(args.include_subdomains),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        ZoneHoldResult {
            hold: crate::random_to_domain_mapper::<bool>(result.hold),
            hold_after: crate::random_to_domain_mapper::<String>(result.hold_after),
            include_subdomains: crate::random_to_domain_mapper::<Option<bool>>(
                result.include_subdomains,
            ),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                configurations: &crate::clone::<Vec<crate::types::ZoneLockdownConfiguration>>(
                    args.configurations,
                ),
                description: &crate::clone::<Option<String>>(args.description),
                paused: &crate::clone::<Option<bool>>(args.paused),
                priority: &crate::clone::<Option<i32>>(args.priority),
                urls: &crate::clone::<Vec<String>>(args.urls),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        ZoneLockdownResult {
            configurations: crate::random_to_domain_mapper::<
                Vec<crate::types::ZoneLockdownConfiguration>,
            >(result.configurations),
            description: crate::random_to_domain_mapper::<Option<String>>(result.description),
            paused: crate::random_to_domain_mapper::<Option<bool>>(result.paused),
            priority: crate::random_to_domain_mapper::<Option<i32>>(result.priority),
            urls: crate::random_to_domain_mapper::<Vec<String>>(result.urls),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
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
                settings: &crate::clone::<Option<crate::types::ZoneSettingsOverrideSettings>>(
                    args.settings,
                ),
                zone_id: &crate::clone::<String>(args.zone_id),
            },
        );

        ZoneSettingsOverrideResult {
            initial_settings: crate::random_to_domain_mapper::<
                Vec<crate::types::ZoneSettingsOverrideInitialSetting>,
            >(result.initial_settings),
            initial_settings_read_at: crate::random_to_domain_mapper::<String>(
                result.initial_settings_read_at,
            ),
            readonly_settings: crate::random_to_domain_mapper::<Vec<String>>(
                result.readonly_settings,
            ),
            settings: crate::random_to_domain_mapper::<crate::types::ZoneSettingsOverrideSettings>(
                result.settings,
            ),
            zone_id: crate::random_to_domain_mapper::<String>(result.zone_id),
            zone_status: crate::random_to_domain_mapper::<String>(result.zone_status),
            zone_type: crate::random_to_domain_mapper::<String>(result.zone_type),
        }
    }
}
