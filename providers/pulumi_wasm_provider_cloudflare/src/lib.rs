use bindings::exports::pulumi::cloudflare::access_application;
use bindings::exports::pulumi::cloudflare::access_ca_certificate;
use bindings::exports::pulumi::cloudflare::access_custom_page;
use bindings::exports::pulumi::cloudflare::access_group;
use bindings::exports::pulumi::cloudflare::access_identity_provider;
use bindings::exports::pulumi::cloudflare::access_keys_configuration;
use bindings::exports::pulumi::cloudflare::access_mutual_tls_certificate;
use bindings::exports::pulumi::cloudflare::access_mutual_tls_hostname_settings;
use bindings::exports::pulumi::cloudflare::access_organization;
use bindings::exports::pulumi::cloudflare::access_policy;
use bindings::exports::pulumi::cloudflare::access_rule;
use bindings::exports::pulumi::cloudflare::access_service_token;
use bindings::exports::pulumi::cloudflare::access_tag;
use bindings::exports::pulumi::cloudflare::account;
use bindings::exports::pulumi::cloudflare::account_member;
use bindings::exports::pulumi::cloudflare::address_map;
use bindings::exports::pulumi::cloudflare::api_shield;
use bindings::exports::pulumi::cloudflare::api_shield_operation;
use bindings::exports::pulumi::cloudflare::api_shield_operation_schema_validation_settings;
use bindings::exports::pulumi::cloudflare::api_shield_schema;
use bindings::exports::pulumi::cloudflare::api_shield_schema_validation_settings;
use bindings::exports::pulumi::cloudflare::api_token;
use bindings::exports::pulumi::cloudflare::argo;
use bindings::exports::pulumi::cloudflare::authenticated_origin_pulls;
use bindings::exports::pulumi::cloudflare::authenticated_origin_pulls_certificate;
use bindings::exports::pulumi::cloudflare::bot_management;
use bindings::exports::pulumi::cloudflare::byo_ip_prefix;
use bindings::exports::pulumi::cloudflare::certificate_pack;
use bindings::exports::pulumi::cloudflare::custom_hostname;
use bindings::exports::pulumi::cloudflare::custom_hostname_fallback_origin;
use bindings::exports::pulumi::cloudflare::custom_pages;
use bindings::exports::pulumi::cloudflare::custom_ssl;
use bindings::exports::pulumi::cloudflare::d1_database;
use bindings::exports::pulumi::cloudflare::device_dex_test;
use bindings::exports::pulumi::cloudflare::device_managed_networks;
use bindings::exports::pulumi::cloudflare::device_policy_certificates;
use bindings::exports::pulumi::cloudflare::device_posture_integration;
use bindings::exports::pulumi::cloudflare::device_posture_rule;
use bindings::exports::pulumi::cloudflare::device_settings_policy;
use bindings::exports::pulumi::cloudflare::dlp_profile;
use bindings::exports::pulumi::cloudflare::email_routing_address;
use bindings::exports::pulumi::cloudflare::email_routing_catch_all;
use bindings::exports::pulumi::cloudflare::email_routing_rule;
use bindings::exports::pulumi::cloudflare::email_routing_settings;
use bindings::exports::pulumi::cloudflare::fallback_domain;
use bindings::exports::pulumi::cloudflare::filter;
use bindings::exports::pulumi::cloudflare::firewall_rule;
use bindings::exports::pulumi::cloudflare::gre_tunnel;
use bindings::exports::pulumi::cloudflare::healthcheck;
use bindings::exports::pulumi::cloudflare::hostname_tls_setting;
use bindings::exports::pulumi::cloudflare::hostname_tls_setting_ciphers;
use bindings::exports::pulumi::cloudflare::hyperdrive_config;
use bindings::exports::pulumi::cloudflare::ipsec_tunnel;
use bindings::exports::pulumi::cloudflare::keyless_certificate;
use bindings::exports::pulumi::cloudflare::list;
use bindings::exports::pulumi::cloudflare::list_item;
use bindings::exports::pulumi::cloudflare::load_balancer;
use bindings::exports::pulumi::cloudflare::load_balancer_monitor;
use bindings::exports::pulumi::cloudflare::load_balancer_pool;
use bindings::exports::pulumi::cloudflare::logpull_retention;
use bindings::exports::pulumi::cloudflare::logpush_job;
use bindings::exports::pulumi::cloudflare::logpush_ownership_challenge;
use bindings::exports::pulumi::cloudflare::magic_firewall_ruleset;
use bindings::exports::pulumi::cloudflare::managed_headers;
use bindings::exports::pulumi::cloudflare::mtls_certificate;
use bindings::exports::pulumi::cloudflare::notification_policy;
use bindings::exports::pulumi::cloudflare::notification_policy_webhooks;
use bindings::exports::pulumi::cloudflare::observatory_scheduled_test;
use bindings::exports::pulumi::cloudflare::origin_ca_certificate;
use bindings::exports::pulumi::cloudflare::page_rule;
use bindings::exports::pulumi::cloudflare::pages_domain;
use bindings::exports::pulumi::cloudflare::pages_project;
use bindings::exports::pulumi::cloudflare::queue;
use bindings::exports::pulumi::cloudflare::r2_bucket;
use bindings::exports::pulumi::cloudflare::rate_limit;
use bindings::exports::pulumi::cloudflare::record;
use bindings::exports::pulumi::cloudflare::regional_hostname;
use bindings::exports::pulumi::cloudflare::regional_tiered_cache;
use bindings::exports::pulumi::cloudflare::ruleset;
use bindings::exports::pulumi::cloudflare::spectrum_application;
use bindings::exports::pulumi::cloudflare::split_tunnel;
use bindings::exports::pulumi::cloudflare::static_route;
use bindings::exports::pulumi::cloudflare::teams_account;
use bindings::exports::pulumi::cloudflare::teams_list;
use bindings::exports::pulumi::cloudflare::teams_location;
use bindings::exports::pulumi::cloudflare::teams_proxy_endpoint;
use bindings::exports::pulumi::cloudflare::teams_rule;
use bindings::exports::pulumi::cloudflare::tiered_cache;
use bindings::exports::pulumi::cloudflare::total_tls;
use bindings::exports::pulumi::cloudflare::tunnel;
use bindings::exports::pulumi::cloudflare::tunnel_config;
use bindings::exports::pulumi::cloudflare::tunnel_route;
use bindings::exports::pulumi::cloudflare::tunnel_virtual_network;
use bindings::exports::pulumi::cloudflare::turnstile_widget;
use bindings::exports::pulumi::cloudflare::url_normalization_settings;
use bindings::exports::pulumi::cloudflare::user_agent_blocking_rule;
use bindings::exports::pulumi::cloudflare::waiting_room;
use bindings::exports::pulumi::cloudflare::waiting_room_event;
use bindings::exports::pulumi::cloudflare::waiting_room_rules;
use bindings::exports::pulumi::cloudflare::waiting_room_settings;
use bindings::exports::pulumi::cloudflare::web3_hostname;
use bindings::exports::pulumi::cloudflare::web_analytics_rule;
use bindings::exports::pulumi::cloudflare::web_analytics_site;
use bindings::exports::pulumi::cloudflare::worker_cron_trigger;
use bindings::exports::pulumi::cloudflare::worker_domain;
use bindings::exports::pulumi::cloudflare::worker_route;
use bindings::exports::pulumi::cloudflare::worker_script;
use bindings::exports::pulumi::cloudflare::worker_secret;
use bindings::exports::pulumi::cloudflare::workers_for_platforms_namespace;
use bindings::exports::pulumi::cloudflare::workers_kv;
use bindings::exports::pulumi::cloudflare::workers_kv_namespace;
use bindings::exports::pulumi::cloudflare::zone;
use bindings::exports::pulumi::cloudflare::zone_cache_reserve;
use bindings::exports::pulumi::cloudflare::zone_cache_variants;
use bindings::exports::pulumi::cloudflare::zone_dnssec;
use bindings::exports::pulumi::cloudflare::zone_hold;
use bindings::exports::pulumi::cloudflare::zone_lockdown;
use bindings::exports::pulumi::cloudflare::zone_settings_override;
use std::collections::HashMap;

use crate::bindings::component::pulumi_wasm::register_interface::{
    register, ObjectField, RegisterResourceRequest, ResultField,
};
mod bindings;
bindings::export!(Component with_types_in bindings);

struct Component {}

impl access_application::Guest for Component {
    fn invoke(name: String, args: access_application::Args) -> access_application::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/accessApplication:AccessApplication".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "allowAuthenticateViaWarp".into(),
                    value: args.allow_authenticate_via_warp,
                },
                ObjectField {
                    name: "allowedIdps".into(),
                    value: args.allowed_idps,
                },
                ObjectField {
                    name: "appLauncherLogoUrl".into(),
                    value: args.app_launcher_logo_url,
                },
                ObjectField {
                    name: "appLauncherVisible".into(),
                    value: args.app_launcher_visible,
                },
                ObjectField {
                    name: "autoRedirectToIdentity".into(),
                    value: args.auto_redirect_to_identity,
                },
                ObjectField {
                    name: "bgColor".into(),
                    value: args.bg_color,
                },
                ObjectField {
                    name: "corsHeaders".into(),
                    value: args.cors_headers,
                },
                ObjectField {
                    name: "customDenyMessage".into(),
                    value: args.custom_deny_message,
                },
                ObjectField {
                    name: "customDenyUrl".into(),
                    value: args.custom_deny_url,
                },
                ObjectField {
                    name: "customNonIdentityDenyUrl".into(),
                    value: args.custom_non_identity_deny_url,
                },
                ObjectField {
                    name: "customPages".into(),
                    value: args.custom_pages,
                },
                ObjectField {
                    name: "domain".into(),
                    value: args.domain,
                },
                ObjectField {
                    name: "enableBindingCookie".into(),
                    value: args.enable_binding_cookie,
                },
                ObjectField {
                    name: "footerLinks".into(),
                    value: args.footer_links,
                },
                ObjectField {
                    name: "headerBgColor".into(),
                    value: args.header_bg_color,
                },
                ObjectField {
                    name: "httpOnlyCookieAttribute".into(),
                    value: args.http_only_cookie_attribute,
                },
                ObjectField {
                    name: "landingPageDesign".into(),
                    value: args.landing_page_design,
                },
                ObjectField {
                    name: "logoUrl".into(),
                    value: args.logo_url,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "saasApp".into(),
                    value: args.saas_app,
                },
                ObjectField {
                    name: "sameSiteCookieAttribute".into(),
                    value: args.same_site_cookie_attribute,
                },
                ObjectField {
                    name: "selfHostedDomains".into(),
                    value: args.self_hosted_domains,
                },
                ObjectField {
                    name: "serviceAuth401Redirect".into(),
                    value: args.service_auth401_redirect,
                },
                ObjectField {
                    name: "sessionDuration".into(),
                    value: args.session_duration,
                },
                ObjectField {
                    name: "skipInterstitial".into(),
                    value: args.skip_interstitial,
                },
                ObjectField {
                    name: "tags".into(),
                    value: args.tags,
                },
                ObjectField {
                    name: "type".into(),
                    value: args.type_,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "allowAuthenticateViaWarp".into(),
                },
                ResultField {
                    name: "allowedIdps".into(),
                },
                ResultField {
                    name: "appLauncherLogoUrl".into(),
                },
                ResultField {
                    name: "appLauncherVisible".into(),
                },
                ResultField { name: "aud".into() },
                ResultField {
                    name: "autoRedirectToIdentity".into(),
                },
                ResultField {
                    name: "bgColor".into(),
                },
                ResultField {
                    name: "corsHeaders".into(),
                },
                ResultField {
                    name: "customDenyMessage".into(),
                },
                ResultField {
                    name: "customDenyUrl".into(),
                },
                ResultField {
                    name: "customNonIdentityDenyUrl".into(),
                },
                ResultField {
                    name: "customPages".into(),
                },
                ResultField {
                    name: "domain".into(),
                },
                ResultField {
                    name: "enableBindingCookie".into(),
                },
                ResultField {
                    name: "footerLinks".into(),
                },
                ResultField {
                    name: "headerBgColor".into(),
                },
                ResultField {
                    name: "httpOnlyCookieAttribute".into(),
                },
                ResultField {
                    name: "landingPageDesign".into(),
                },
                ResultField {
                    name: "logoUrl".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "saasApp".into(),
                },
                ResultField {
                    name: "sameSiteCookieAttribute".into(),
                },
                ResultField {
                    name: "selfHostedDomains".into(),
                },
                ResultField {
                    name: "serviceAuth401Redirect".into(),
                },
                ResultField {
                    name: "sessionDuration".into(),
                },
                ResultField {
                    name: "skipInterstitial".into(),
                },
                ResultField {
                    name: "tags".into(),
                },
                ResultField {
                    name: "type".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        access_application::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            allow_authenticate_via_warp: hashmap.remove("allowAuthenticateViaWarp").unwrap(),
            allowed_idps: hashmap.remove("allowedIdps").unwrap(),
            app_launcher_logo_url: hashmap.remove("appLauncherLogoUrl").unwrap(),
            app_launcher_visible: hashmap.remove("appLauncherVisible").unwrap(),
            aud: hashmap.remove("aud").unwrap(),
            auto_redirect_to_identity: hashmap.remove("autoRedirectToIdentity").unwrap(),
            bg_color: hashmap.remove("bgColor").unwrap(),
            cors_headers: hashmap.remove("corsHeaders").unwrap(),
            custom_deny_message: hashmap.remove("customDenyMessage").unwrap(),
            custom_deny_url: hashmap.remove("customDenyUrl").unwrap(),
            custom_non_identity_deny_url: hashmap.remove("customNonIdentityDenyUrl").unwrap(),
            custom_pages: hashmap.remove("customPages").unwrap(),
            domain: hashmap.remove("domain").unwrap(),
            enable_binding_cookie: hashmap.remove("enableBindingCookie").unwrap(),
            footer_links: hashmap.remove("footerLinks").unwrap(),
            header_bg_color: hashmap.remove("headerBgColor").unwrap(),
            http_only_cookie_attribute: hashmap.remove("httpOnlyCookieAttribute").unwrap(),
            landing_page_design: hashmap.remove("landingPageDesign").unwrap(),
            logo_url: hashmap.remove("logoUrl").unwrap(),
            name: hashmap.remove("name").unwrap(),
            saas_app: hashmap.remove("saasApp").unwrap(),
            same_site_cookie_attribute: hashmap.remove("sameSiteCookieAttribute").unwrap(),
            self_hosted_domains: hashmap.remove("selfHostedDomains").unwrap(),
            service_auth401_redirect: hashmap.remove("serviceAuth401Redirect").unwrap(),
            session_duration: hashmap.remove("sessionDuration").unwrap(),
            skip_interstitial: hashmap.remove("skipInterstitial").unwrap(),
            tags: hashmap.remove("tags").unwrap(),
            type_: hashmap.remove("type").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl access_ca_certificate::Guest for Component {
    fn invoke(name: String, args: access_ca_certificate::Args) -> access_ca_certificate::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/accessCaCertificate:AccessCaCertificate".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "applicationId".into(),
                    value: args.application_id,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "applicationId".into(),
                },
                ResultField { name: "aud".into() },
                ResultField {
                    name: "publicKey".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        access_ca_certificate::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            application_id: hashmap.remove("applicationId").unwrap(),
            aud: hashmap.remove("aud").unwrap(),
            public_key: hashmap.remove("publicKey").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl access_custom_page::Guest for Component {
    fn invoke(name: String, args: access_custom_page::Args) -> access_custom_page::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/accessCustomPage:AccessCustomPage".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "appCount".into(),
                    value: args.app_count,
                },
                ObjectField {
                    name: "customHtml".into(),
                    value: args.custom_html,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "type".into(),
                    value: args.type_,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "appCount".into(),
                },
                ResultField {
                    name: "customHtml".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "type".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        access_custom_page::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            app_count: hashmap.remove("appCount").unwrap(),
            custom_html: hashmap.remove("customHtml").unwrap(),
            name: hashmap.remove("name").unwrap(),
            type_: hashmap.remove("type").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl access_group::Guest for Component {
    fn invoke(name: String, args: access_group::Args) -> access_group::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/accessGroup:AccessGroup".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "excludes".into(),
                    value: args.excludes,
                },
                ObjectField {
                    name: "includes".into(),
                    value: args.includes,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "requires".into(),
                    value: args.requires,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "excludes".into(),
                },
                ResultField {
                    name: "includes".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "requires".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        access_group::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            excludes: hashmap.remove("excludes").unwrap(),
            includes: hashmap.remove("includes").unwrap(),
            name: hashmap.remove("name").unwrap(),
            requires: hashmap.remove("requires").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl access_identity_provider::Guest for Component {
    fn invoke(name: String, args: access_identity_provider::Args) -> access_identity_provider::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/accessIdentityProvider:AccessIdentityProvider".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "configs".into(),
                    value: args.configs,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "scimConfigs".into(),
                    value: args.scim_configs,
                },
                ObjectField {
                    name: "type".into(),
                    value: args.type_,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "configs".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "scimConfigs".into(),
                },
                ResultField {
                    name: "type".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        access_identity_provider::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            configs: hashmap.remove("configs").unwrap(),
            name: hashmap.remove("name").unwrap(),
            scim_configs: hashmap.remove("scimConfigs").unwrap(),
            type_: hashmap.remove("type").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl access_keys_configuration::Guest for Component {
    fn invoke(
        name: String,
        args: access_keys_configuration::Args,
    ) -> access_keys_configuration::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/accessKeysConfiguration:AccessKeysConfiguration".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "keyRotationIntervalDays".into(),
                    value: args.key_rotation_interval_days,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "keyRotationIntervalDays".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        access_keys_configuration::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            key_rotation_interval_days: hashmap.remove("keyRotationIntervalDays").unwrap(),
        }
    }
}
impl access_mutual_tls_certificate::Guest for Component {
    fn invoke(
        name: String,
        args: access_mutual_tls_certificate::Args,
    ) -> access_mutual_tls_certificate::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/accessMutualTlsCertificate:AccessMutualTlsCertificate".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "associatedHostnames".into(),
                    value: args.associated_hostnames,
                },
                ObjectField {
                    name: "certificate".into(),
                    value: args.certificate,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "associatedHostnames".into(),
                },
                ResultField {
                    name: "certificate".into(),
                },
                ResultField {
                    name: "fingerprint".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        access_mutual_tls_certificate::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            associated_hostnames: hashmap.remove("associatedHostnames").unwrap(),
            certificate: hashmap.remove("certificate").unwrap(),
            fingerprint: hashmap.remove("fingerprint").unwrap(),
            name: hashmap.remove("name").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl access_mutual_tls_hostname_settings::Guest for Component {
    fn invoke(
        name: String,
        args: access_mutual_tls_hostname_settings::Args,
    ) -> access_mutual_tls_hostname_settings::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_:
                "cloudflare:index/accessMutualTlsHostnameSettings:AccessMutualTlsHostnameSettings"
                    .into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "settings".into(),
                    value: args.settings,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "settings".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        access_mutual_tls_hostname_settings::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            settings: hashmap.remove("settings").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl access_organization::Guest for Component {
    fn invoke(name: String, args: access_organization::Args) -> access_organization::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/accessOrganization:AccessOrganization".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "allowAuthenticateViaWarp".into(),
                    value: args.allow_authenticate_via_warp,
                },
                ObjectField {
                    name: "authDomain".into(),
                    value: args.auth_domain,
                },
                ObjectField {
                    name: "autoRedirectToIdentity".into(),
                    value: args.auto_redirect_to_identity,
                },
                ObjectField {
                    name: "customPages".into(),
                    value: args.custom_pages,
                },
                ObjectField {
                    name: "isUiReadOnly".into(),
                    value: args.is_ui_read_only,
                },
                ObjectField {
                    name: "loginDesigns".into(),
                    value: args.login_designs,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "sessionDuration".into(),
                    value: args.session_duration,
                },
                ObjectField {
                    name: "uiReadOnlyToggleReason".into(),
                    value: args.ui_read_only_toggle_reason,
                },
                ObjectField {
                    name: "userSeatExpirationInactiveTime".into(),
                    value: args.user_seat_expiration_inactive_time,
                },
                ObjectField {
                    name: "warpAuthSessionDuration".into(),
                    value: args.warp_auth_session_duration,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "allowAuthenticateViaWarp".into(),
                },
                ResultField {
                    name: "authDomain".into(),
                },
                ResultField {
                    name: "autoRedirectToIdentity".into(),
                },
                ResultField {
                    name: "customPages".into(),
                },
                ResultField {
                    name: "isUiReadOnly".into(),
                },
                ResultField {
                    name: "loginDesigns".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "sessionDuration".into(),
                },
                ResultField {
                    name: "uiReadOnlyToggleReason".into(),
                },
                ResultField {
                    name: "userSeatExpirationInactiveTime".into(),
                },
                ResultField {
                    name: "warpAuthSessionDuration".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        access_organization::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            allow_authenticate_via_warp: hashmap.remove("allowAuthenticateViaWarp").unwrap(),
            auth_domain: hashmap.remove("authDomain").unwrap(),
            auto_redirect_to_identity: hashmap.remove("autoRedirectToIdentity").unwrap(),
            custom_pages: hashmap.remove("customPages").unwrap(),
            is_ui_read_only: hashmap.remove("isUiReadOnly").unwrap(),
            login_designs: hashmap.remove("loginDesigns").unwrap(),
            name: hashmap.remove("name").unwrap(),
            session_duration: hashmap.remove("sessionDuration").unwrap(),
            ui_read_only_toggle_reason: hashmap.remove("uiReadOnlyToggleReason").unwrap(),
            user_seat_expiration_inactive_time: hashmap
                .remove("userSeatExpirationInactiveTime")
                .unwrap(),
            warp_auth_session_duration: hashmap.remove("warpAuthSessionDuration").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl access_policy::Guest for Component {
    fn invoke(name: String, args: access_policy::Args) -> access_policy::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/accessPolicy:AccessPolicy".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "applicationId".into(),
                    value: args.application_id,
                },
                ObjectField {
                    name: "approvalGroups".into(),
                    value: args.approval_groups,
                },
                ObjectField {
                    name: "approvalRequired".into(),
                    value: args.approval_required,
                },
                ObjectField {
                    name: "decision".into(),
                    value: args.decision,
                },
                ObjectField {
                    name: "excludes".into(),
                    value: args.excludes,
                },
                ObjectField {
                    name: "includes".into(),
                    value: args.includes,
                },
                ObjectField {
                    name: "isolationRequired".into(),
                    value: args.isolation_required,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "precedence".into(),
                    value: args.precedence,
                },
                ObjectField {
                    name: "purposeJustificationPrompt".into(),
                    value: args.purpose_justification_prompt,
                },
                ObjectField {
                    name: "purposeJustificationRequired".into(),
                    value: args.purpose_justification_required,
                },
                ObjectField {
                    name: "requires".into(),
                    value: args.requires,
                },
                ObjectField {
                    name: "sessionDuration".into(),
                    value: args.session_duration,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "applicationId".into(),
                },
                ResultField {
                    name: "approvalGroups".into(),
                },
                ResultField {
                    name: "approvalRequired".into(),
                },
                ResultField {
                    name: "decision".into(),
                },
                ResultField {
                    name: "excludes".into(),
                },
                ResultField {
                    name: "includes".into(),
                },
                ResultField {
                    name: "isolationRequired".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "precedence".into(),
                },
                ResultField {
                    name: "purposeJustificationPrompt".into(),
                },
                ResultField {
                    name: "purposeJustificationRequired".into(),
                },
                ResultField {
                    name: "requires".into(),
                },
                ResultField {
                    name: "sessionDuration".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        access_policy::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            application_id: hashmap.remove("applicationId").unwrap(),
            approval_groups: hashmap.remove("approvalGroups").unwrap(),
            approval_required: hashmap.remove("approvalRequired").unwrap(),
            decision: hashmap.remove("decision").unwrap(),
            excludes: hashmap.remove("excludes").unwrap(),
            includes: hashmap.remove("includes").unwrap(),
            isolation_required: hashmap.remove("isolationRequired").unwrap(),
            name: hashmap.remove("name").unwrap(),
            precedence: hashmap.remove("precedence").unwrap(),
            purpose_justification_prompt: hashmap.remove("purposeJustificationPrompt").unwrap(),
            purpose_justification_required: hashmap.remove("purposeJustificationRequired").unwrap(),
            requires: hashmap.remove("requires").unwrap(),
            session_duration: hashmap.remove("sessionDuration").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl access_rule::Guest for Component {
    fn invoke(name: String, args: access_rule::Args) -> access_rule::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/accessRule:AccessRule".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "configuration".into(),
                    value: args.configuration,
                },
                ObjectField {
                    name: "mode".into(),
                    value: args.mode,
                },
                ObjectField {
                    name: "notes".into(),
                    value: args.notes,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "configuration".into(),
                },
                ResultField {
                    name: "mode".into(),
                },
                ResultField {
                    name: "notes".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        access_rule::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            configuration: hashmap.remove("configuration").unwrap(),
            mode: hashmap.remove("mode").unwrap(),
            notes: hashmap.remove("notes").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl access_service_token::Guest for Component {
    fn invoke(name: String, args: access_service_token::Args) -> access_service_token::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/accessServiceToken:AccessServiceToken".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "duration".into(),
                    value: args.duration,
                },
                ObjectField {
                    name: "minDaysForRenewal".into(),
                    value: args.min_days_for_renewal,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "clientId".into(),
                },
                ResultField {
                    name: "clientSecret".into(),
                },
                ResultField {
                    name: "duration".into(),
                },
                ResultField {
                    name: "expiresAt".into(),
                },
                ResultField {
                    name: "minDaysForRenewal".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        access_service_token::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            client_id: hashmap.remove("clientId").unwrap(),
            client_secret: hashmap.remove("clientSecret").unwrap(),
            duration: hashmap.remove("duration").unwrap(),
            expires_at: hashmap.remove("expiresAt").unwrap(),
            min_days_for_renewal: hashmap.remove("minDaysForRenewal").unwrap(),
            name: hashmap.remove("name").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl access_tag::Guest for Component {
    fn invoke(name: String, args: access_tag::Args) -> access_tag::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/accessTag:AccessTag".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "appCount".into(),
                    value: args.app_count,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "appCount".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        access_tag::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            app_count: hashmap.remove("appCount").unwrap(),
            name: hashmap.remove("name").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl account::Guest for Component {
    fn invoke(name: String, args: account::Args) -> account::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/account:Account".into(),
            name,
            object: vec![
                ObjectField {
                    name: "enforceTwofactor".into(),
                    value: args.enforce_twofactor,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "type".into(),
                    value: args.type_,
                },
            ],
            results: vec![
                ResultField {
                    name: "enforceTwofactor".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "type".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        account::Res {
            enforce_twofactor: hashmap.remove("enforceTwofactor").unwrap(),
            name: hashmap.remove("name").unwrap(),
            type_: hashmap.remove("type").unwrap(),
        }
    }
}
impl account_member::Guest for Component {
    fn invoke(name: String, args: account_member::Args) -> account_member::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/accountMember:AccountMember".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "emailAddress".into(),
                    value: args.email_address,
                },
                ObjectField {
                    name: "roleIds".into(),
                    value: args.role_ids,
                },
                ObjectField {
                    name: "status".into(),
                    value: args.status,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "emailAddress".into(),
                },
                ResultField {
                    name: "roleIds".into(),
                },
                ResultField {
                    name: "status".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        account_member::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            email_address: hashmap.remove("emailAddress").unwrap(),
            role_ids: hashmap.remove("roleIds").unwrap(),
            status: hashmap.remove("status").unwrap(),
        }
    }
}
impl address_map::Guest for Component {
    fn invoke(name: String, args: address_map::Args) -> address_map::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/addressMap:AddressMap".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "defaultSni".into(),
                    value: args.default_sni,
                },
                ObjectField {
                    name: "description".into(),
                    value: args.description,
                },
                ObjectField {
                    name: "enabled".into(),
                    value: args.enabled,
                },
                ObjectField {
                    name: "ips".into(),
                    value: args.ips,
                },
                ObjectField {
                    name: "memberships".into(),
                    value: args.memberships,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "canDelete".into(),
                },
                ResultField {
                    name: "canModifyIps".into(),
                },
                ResultField {
                    name: "defaultSni".into(),
                },
                ResultField {
                    name: "description".into(),
                },
                ResultField {
                    name: "enabled".into(),
                },
                ResultField { name: "ips".into() },
                ResultField {
                    name: "memberships".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        address_map::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            can_delete: hashmap.remove("canDelete").unwrap(),
            can_modify_ips: hashmap.remove("canModifyIps").unwrap(),
            default_sni: hashmap.remove("defaultSni").unwrap(),
            description: hashmap.remove("description").unwrap(),
            enabled: hashmap.remove("enabled").unwrap(),
            ips: hashmap.remove("ips").unwrap(),
            memberships: hashmap.remove("memberships").unwrap(),
        }
    }
}
impl api_shield::Guest for Component {
    fn invoke(name: String, args: api_shield::Args) -> api_shield::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/apiShield:ApiShield".into(),
            name,
            object: vec![
                ObjectField {
                    name: "authIdCharacteristics".into(),
                    value: args.auth_id_characteristics,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "authIdCharacteristics".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        api_shield::Res {
            auth_id_characteristics: hashmap.remove("authIdCharacteristics").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl api_shield_operation::Guest for Component {
    fn invoke(name: String, args: api_shield_operation::Args) -> api_shield_operation::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/apiShieldOperation:ApiShieldOperation".into(),
            name,
            object: vec![
                ObjectField {
                    name: "endpoint".into(),
                    value: args.endpoint,
                },
                ObjectField {
                    name: "host".into(),
                    value: args.host,
                },
                ObjectField {
                    name: "method".into(),
                    value: args.method,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "endpoint".into(),
                },
                ResultField {
                    name: "host".into(),
                },
                ResultField {
                    name: "method".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        api_shield_operation::Res {
            endpoint: hashmap.remove("endpoint").unwrap(),
            host: hashmap.remove("host").unwrap(),
            method: hashmap.remove("method").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl api_shield_operation_schema_validation_settings::Guest for Component {
    fn invoke(
        name: String,
        args: api_shield_operation_schema_validation_settings::Args,
    ) -> api_shield_operation_schema_validation_settings::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/apiShieldOperationSchemaValidationSettings:ApiShieldOperationSchemaValidationSettings".into(),
            name,
            object: vec![
                ObjectField { name: "mitigationAction".into(), value: args.mitigation_action },
                ObjectField { name: "operationId".into(), value: args.operation_id },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "mitigationAction".into() },
                ResultField { name: "operationId".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        api_shield_operation_schema_validation_settings::Res {
            mitigation_action: hashmap.remove("mitigationAction").unwrap(),
            operation_id: hashmap.remove("operationId").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl api_shield_schema::Guest for Component {
    fn invoke(name: String, args: api_shield_schema::Args) -> api_shield_schema::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/apiShieldSchema:ApiShieldSchema".into(),
            name,
            object: vec![
                ObjectField {
                    name: "kind".into(),
                    value: args.kind,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "source".into(),
                    value: args.source,
                },
                ObjectField {
                    name: "validationEnabled".into(),
                    value: args.validation_enabled,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "kind".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "source".into(),
                },
                ResultField {
                    name: "validationEnabled".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        api_shield_schema::Res {
            kind: hashmap.remove("kind").unwrap(),
            name: hashmap.remove("name").unwrap(),
            source: hashmap.remove("source").unwrap(),
            validation_enabled: hashmap.remove("validationEnabled").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl api_shield_schema_validation_settings::Guest for Component {
    fn invoke(
        name: String,
        args: api_shield_schema_validation_settings::Args,
    ) -> api_shield_schema_validation_settings::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/apiShieldSchemaValidationSettings:ApiShieldSchemaValidationSettings".into(),
            name,
            object: vec![
                ObjectField { name: "validationDefaultMitigationAction".into(), value: args.validation_default_mitigation_action },
                ObjectField { name: "validationOverrideMitigationAction".into(), value: args.validation_override_mitigation_action },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "validationDefaultMitigationAction".into() },
                ResultField { name: "validationOverrideMitigationAction".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        api_shield_schema_validation_settings::Res {
            validation_default_mitigation_action: hashmap
                .remove("validationDefaultMitigationAction")
                .unwrap(),
            validation_override_mitigation_action: hashmap
                .remove("validationOverrideMitigationAction")
                .unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl api_token::Guest for Component {
    fn invoke(name: String, args: api_token::Args) -> api_token::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/apiToken:ApiToken".into(),
            name,
            object: vec![
                ObjectField {
                    name: "condition".into(),
                    value: args.condition,
                },
                ObjectField {
                    name: "expiresOn".into(),
                    value: args.expires_on,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "notBefore".into(),
                    value: args.not_before,
                },
                ObjectField {
                    name: "policies".into(),
                    value: args.policies,
                },
            ],
            results: vec![
                ResultField {
                    name: "condition".into(),
                },
                ResultField {
                    name: "expiresOn".into(),
                },
                ResultField {
                    name: "issuedOn".into(),
                },
                ResultField {
                    name: "modifiedOn".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "notBefore".into(),
                },
                ResultField {
                    name: "policies".into(),
                },
                ResultField {
                    name: "status".into(),
                },
                ResultField {
                    name: "value".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        api_token::Res {
            condition: hashmap.remove("condition").unwrap(),
            expires_on: hashmap.remove("expiresOn").unwrap(),
            issued_on: hashmap.remove("issuedOn").unwrap(),
            modified_on: hashmap.remove("modifiedOn").unwrap(),
            name: hashmap.remove("name").unwrap(),
            not_before: hashmap.remove("notBefore").unwrap(),
            policies: hashmap.remove("policies").unwrap(),
            status: hashmap.remove("status").unwrap(),
            value: hashmap.remove("value").unwrap(),
        }
    }
}
impl argo::Guest for Component {
    fn invoke(name: String, args: argo::Args) -> argo::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/argo:Argo".into(),
            name,
            object: vec![
                ObjectField {
                    name: "smartRouting".into(),
                    value: args.smart_routing,
                },
                ObjectField {
                    name: "tieredCaching".into(),
                    value: args.tiered_caching,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "smartRouting".into(),
                },
                ResultField {
                    name: "tieredCaching".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        argo::Res {
            smart_routing: hashmap.remove("smartRouting").unwrap(),
            tiered_caching: hashmap.remove("tieredCaching").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl authenticated_origin_pulls::Guest for Component {
    fn invoke(
        name: String,
        args: authenticated_origin_pulls::Args,
    ) -> authenticated_origin_pulls::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/authenticatedOriginPulls:AuthenticatedOriginPulls".into(),
            name,
            object: vec![
                ObjectField {
                    name: "authenticatedOriginPullsCertificate".into(),
                    value: args.authenticated_origin_pulls_certificate,
                },
                ObjectField {
                    name: "enabled".into(),
                    value: args.enabled,
                },
                ObjectField {
                    name: "hostname".into(),
                    value: args.hostname,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "authenticatedOriginPullsCertificate".into(),
                },
                ResultField {
                    name: "enabled".into(),
                },
                ResultField {
                    name: "hostname".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        authenticated_origin_pulls::Res {
            authenticated_origin_pulls_certificate: hashmap
                .remove("authenticatedOriginPullsCertificate")
                .unwrap(),
            enabled: hashmap.remove("enabled").unwrap(),
            hostname: hashmap.remove("hostname").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl authenticated_origin_pulls_certificate::Guest for Component {
    fn invoke(
        name: String,
        args: authenticated_origin_pulls_certificate::Args,
    ) -> authenticated_origin_pulls_certificate::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/authenticatedOriginPullsCertificate:AuthenticatedOriginPullsCertificate".into(),
            name,
            object: vec![
                ObjectField { name: "certificate".into(), value: args.certificate },
                ObjectField { name: "privateKey".into(), value: args.private_key },
                ObjectField { name: "type".into(), value: args.type_ },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "certificate".into() },
                ResultField { name: "expiresOn".into() },
                ResultField { name: "issuer".into() },
                ResultField { name: "privateKey".into() },
                ResultField { name: "serialNumber".into() },
                ResultField { name: "signature".into() },
                ResultField { name: "status".into() },
                ResultField { name: "type".into() },
                ResultField { name: "uploadedOn".into() },
                ResultField { name: "zoneId".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        authenticated_origin_pulls_certificate::Res {
            certificate: hashmap.remove("certificate").unwrap(),
            expires_on: hashmap.remove("expiresOn").unwrap(),
            issuer: hashmap.remove("issuer").unwrap(),
            private_key: hashmap.remove("privateKey").unwrap(),
            serial_number: hashmap.remove("serialNumber").unwrap(),
            signature: hashmap.remove("signature").unwrap(),
            status: hashmap.remove("status").unwrap(),
            type_: hashmap.remove("type").unwrap(),
            uploaded_on: hashmap.remove("uploadedOn").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl bot_management::Guest for Component {
    fn invoke(name: String, args: bot_management::Args) -> bot_management::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/botManagement:BotManagement".into(),
            name,
            object: vec![
                ObjectField {
                    name: "autoUpdateModel".into(),
                    value: args.auto_update_model,
                },
                ObjectField {
                    name: "enableJs".into(),
                    value: args.enable_js,
                },
                ObjectField {
                    name: "fightMode".into(),
                    value: args.fight_mode,
                },
                ObjectField {
                    name: "optimizeWordpress".into(),
                    value: args.optimize_wordpress,
                },
                ObjectField {
                    name: "sbfmDefinitelyAutomated".into(),
                    value: args.sbfm_definitely_automated,
                },
                ObjectField {
                    name: "sbfmLikelyAutomated".into(),
                    value: args.sbfm_likely_automated,
                },
                ObjectField {
                    name: "sbfmStaticResourceProtection".into(),
                    value: args.sbfm_static_resource_protection,
                },
                ObjectField {
                    name: "sbfmVerifiedBots".into(),
                    value: args.sbfm_verified_bots,
                },
                ObjectField {
                    name: "suppressSessionScore".into(),
                    value: args.suppress_session_score,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "autoUpdateModel".into(),
                },
                ResultField {
                    name: "enableJs".into(),
                },
                ResultField {
                    name: "fightMode".into(),
                },
                ResultField {
                    name: "optimizeWordpress".into(),
                },
                ResultField {
                    name: "sbfmDefinitelyAutomated".into(),
                },
                ResultField {
                    name: "sbfmLikelyAutomated".into(),
                },
                ResultField {
                    name: "sbfmStaticResourceProtection".into(),
                },
                ResultField {
                    name: "sbfmVerifiedBots".into(),
                },
                ResultField {
                    name: "suppressSessionScore".into(),
                },
                ResultField {
                    name: "usingLatestModel".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        bot_management::Res {
            auto_update_model: hashmap.remove("autoUpdateModel").unwrap(),
            enable_js: hashmap.remove("enableJs").unwrap(),
            fight_mode: hashmap.remove("fightMode").unwrap(),
            optimize_wordpress: hashmap.remove("optimizeWordpress").unwrap(),
            sbfm_definitely_automated: hashmap.remove("sbfmDefinitelyAutomated").unwrap(),
            sbfm_likely_automated: hashmap.remove("sbfmLikelyAutomated").unwrap(),
            sbfm_static_resource_protection: hashmap
                .remove("sbfmStaticResourceProtection")
                .unwrap(),
            sbfm_verified_bots: hashmap.remove("sbfmVerifiedBots").unwrap(),
            suppress_session_score: hashmap.remove("suppressSessionScore").unwrap(),
            using_latest_model: hashmap.remove("usingLatestModel").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl byo_ip_prefix::Guest for Component {
    fn invoke(name: String, args: byo_ip_prefix::Args) -> byo_ip_prefix::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/byoIpPrefix:ByoIpPrefix".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "advertisement".into(),
                    value: args.advertisement,
                },
                ObjectField {
                    name: "description".into(),
                    value: args.description,
                },
                ObjectField {
                    name: "prefixId".into(),
                    value: args.prefix_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "advertisement".into(),
                },
                ResultField {
                    name: "description".into(),
                },
                ResultField {
                    name: "prefixId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        byo_ip_prefix::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            advertisement: hashmap.remove("advertisement").unwrap(),
            description: hashmap.remove("description").unwrap(),
            prefix_id: hashmap.remove("prefixId").unwrap(),
        }
    }
}
impl certificate_pack::Guest for Component {
    fn invoke(name: String, args: certificate_pack::Args) -> certificate_pack::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/certificatePack:CertificatePack".into(),
            name,
            object: vec![
                ObjectField {
                    name: "certificateAuthority".into(),
                    value: args.certificate_authority,
                },
                ObjectField {
                    name: "cloudflareBranding".into(),
                    value: args.cloudflare_branding,
                },
                ObjectField {
                    name: "hosts".into(),
                    value: args.hosts,
                },
                ObjectField {
                    name: "type".into(),
                    value: args.type_,
                },
                ObjectField {
                    name: "validationErrors".into(),
                    value: args.validation_errors,
                },
                ObjectField {
                    name: "validationMethod".into(),
                    value: args.validation_method,
                },
                ObjectField {
                    name: "validationRecords".into(),
                    value: args.validation_records,
                },
                ObjectField {
                    name: "validityDays".into(),
                    value: args.validity_days,
                },
                ObjectField {
                    name: "waitForActiveStatus".into(),
                    value: args.wait_for_active_status,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "certificateAuthority".into(),
                },
                ResultField {
                    name: "cloudflareBranding".into(),
                },
                ResultField {
                    name: "hosts".into(),
                },
                ResultField {
                    name: "type".into(),
                },
                ResultField {
                    name: "validationErrors".into(),
                },
                ResultField {
                    name: "validationMethod".into(),
                },
                ResultField {
                    name: "validationRecords".into(),
                },
                ResultField {
                    name: "validityDays".into(),
                },
                ResultField {
                    name: "waitForActiveStatus".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        certificate_pack::Res {
            certificate_authority: hashmap.remove("certificateAuthority").unwrap(),
            cloudflare_branding: hashmap.remove("cloudflareBranding").unwrap(),
            hosts: hashmap.remove("hosts").unwrap(),
            type_: hashmap.remove("type").unwrap(),
            validation_errors: hashmap.remove("validationErrors").unwrap(),
            validation_method: hashmap.remove("validationMethod").unwrap(),
            validation_records: hashmap.remove("validationRecords").unwrap(),
            validity_days: hashmap.remove("validityDays").unwrap(),
            wait_for_active_status: hashmap.remove("waitForActiveStatus").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl custom_hostname::Guest for Component {
    fn invoke(name: String, args: custom_hostname::Args) -> custom_hostname::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/customHostname:CustomHostname".into(),
            name,
            object: vec![
                ObjectField {
                    name: "customMetadata".into(),
                    value: args.custom_metadata,
                },
                ObjectField {
                    name: "customOriginServer".into(),
                    value: args.custom_origin_server,
                },
                ObjectField {
                    name: "customOriginSni".into(),
                    value: args.custom_origin_sni,
                },
                ObjectField {
                    name: "hostname".into(),
                    value: args.hostname,
                },
                ObjectField {
                    name: "ssls".into(),
                    value: args.ssls,
                },
                ObjectField {
                    name: "waitForSslPendingValidation".into(),
                    value: args.wait_for_ssl_pending_validation,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "customMetadata".into(),
                },
                ResultField {
                    name: "customOriginServer".into(),
                },
                ResultField {
                    name: "customOriginSni".into(),
                },
                ResultField {
                    name: "hostname".into(),
                },
                ResultField {
                    name: "ownershipVerification".into(),
                },
                ResultField {
                    name: "ownershipVerificationHttp".into(),
                },
                ResultField {
                    name: "ssls".into(),
                },
                ResultField {
                    name: "status".into(),
                },
                ResultField {
                    name: "waitForSslPendingValidation".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        custom_hostname::Res {
            custom_metadata: hashmap.remove("customMetadata").unwrap(),
            custom_origin_server: hashmap.remove("customOriginServer").unwrap(),
            custom_origin_sni: hashmap.remove("customOriginSni").unwrap(),
            hostname: hashmap.remove("hostname").unwrap(),
            ownership_verification: hashmap.remove("ownershipVerification").unwrap(),
            ownership_verification_http: hashmap.remove("ownershipVerificationHttp").unwrap(),
            ssls: hashmap.remove("ssls").unwrap(),
            status: hashmap.remove("status").unwrap(),
            wait_for_ssl_pending_validation: hashmap.remove("waitForSslPendingValidation").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl custom_hostname_fallback_origin::Guest for Component {
    fn invoke(
        name: String,
        args: custom_hostname_fallback_origin::Args,
    ) -> custom_hostname_fallback_origin::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/customHostnameFallbackOrigin:CustomHostnameFallbackOrigin"
                .into(),
            name,
            object: vec![
                ObjectField {
                    name: "origin".into(),
                    value: args.origin,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "origin".into(),
                },
                ResultField {
                    name: "status".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        custom_hostname_fallback_origin::Res {
            origin: hashmap.remove("origin").unwrap(),
            status: hashmap.remove("status").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl custom_pages::Guest for Component {
    fn invoke(name: String, args: custom_pages::Args) -> custom_pages::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/customPages:CustomPages".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "state".into(),
                    value: args.state,
                },
                ObjectField {
                    name: "type".into(),
                    value: args.type_,
                },
                ObjectField {
                    name: "url".into(),
                    value: args.url,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "state".into(),
                },
                ResultField {
                    name: "type".into(),
                },
                ResultField { name: "url".into() },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        custom_pages::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            state: hashmap.remove("state").unwrap(),
            type_: hashmap.remove("type").unwrap(),
            url: hashmap.remove("url").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl custom_ssl::Guest for Component {
    fn invoke(name: String, args: custom_ssl::Args) -> custom_ssl::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/customSsl:CustomSsl".into(),
            name,
            object: vec![
                ObjectField {
                    name: "customSslOptions".into(),
                    value: args.custom_ssl_options,
                },
                ObjectField {
                    name: "customSslPriorities".into(),
                    value: args.custom_ssl_priorities,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "customSslOptions".into(),
                },
                ResultField {
                    name: "customSslPriorities".into(),
                },
                ResultField {
                    name: "expiresOn".into(),
                },
                ResultField {
                    name: "hosts".into(),
                },
                ResultField {
                    name: "issuer".into(),
                },
                ResultField {
                    name: "modifiedOn".into(),
                },
                ResultField {
                    name: "priority".into(),
                },
                ResultField {
                    name: "signature".into(),
                },
                ResultField {
                    name: "status".into(),
                },
                ResultField {
                    name: "uploadedOn".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        custom_ssl::Res {
            custom_ssl_options: hashmap.remove("customSslOptions").unwrap(),
            custom_ssl_priorities: hashmap.remove("customSslPriorities").unwrap(),
            expires_on: hashmap.remove("expiresOn").unwrap(),
            hosts: hashmap.remove("hosts").unwrap(),
            issuer: hashmap.remove("issuer").unwrap(),
            modified_on: hashmap.remove("modifiedOn").unwrap(),
            priority: hashmap.remove("priority").unwrap(),
            signature: hashmap.remove("signature").unwrap(),
            status: hashmap.remove("status").unwrap(),
            uploaded_on: hashmap.remove("uploadedOn").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl d1_database::Guest for Component {
    fn invoke(name: String, args: d1_database::Args) -> d1_database::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/d1Database:D1Database".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "version".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        d1_database::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            name: hashmap.remove("name").unwrap(),
            version: hashmap.remove("version").unwrap(),
        }
    }
}
impl device_dex_test::Guest for Component {
    fn invoke(name: String, args: device_dex_test::Args) -> device_dex_test::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/deviceDexTest:DeviceDexTest".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "data".into(),
                    value: args.data,
                },
                ObjectField {
                    name: "description".into(),
                    value: args.description,
                },
                ObjectField {
                    name: "enabled".into(),
                    value: args.enabled,
                },
                ObjectField {
                    name: "interval".into(),
                    value: args.interval,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "created".into(),
                },
                ResultField {
                    name: "data".into(),
                },
                ResultField {
                    name: "description".into(),
                },
                ResultField {
                    name: "enabled".into(),
                },
                ResultField {
                    name: "interval".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "updated".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        device_dex_test::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            created: hashmap.remove("created").unwrap(),
            data: hashmap.remove("data").unwrap(),
            description: hashmap.remove("description").unwrap(),
            enabled: hashmap.remove("enabled").unwrap(),
            interval: hashmap.remove("interval").unwrap(),
            name: hashmap.remove("name").unwrap(),
            updated: hashmap.remove("updated").unwrap(),
        }
    }
}
impl device_managed_networks::Guest for Component {
    fn invoke(name: String, args: device_managed_networks::Args) -> device_managed_networks::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/deviceManagedNetworks:DeviceManagedNetworks".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "config".into(),
                    value: args.config,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "type".into(),
                    value: args.type_,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "config".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "type".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        device_managed_networks::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            config: hashmap.remove("config").unwrap(),
            name: hashmap.remove("name").unwrap(),
            type_: hashmap.remove("type").unwrap(),
        }
    }
}
impl device_policy_certificates::Guest for Component {
    fn invoke(
        name: String,
        args: device_policy_certificates::Args,
    ) -> device_policy_certificates::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/devicePolicyCertificates:DevicePolicyCertificates".into(),
            name,
            object: vec![
                ObjectField {
                    name: "enabled".into(),
                    value: args.enabled,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "enabled".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        device_policy_certificates::Res {
            enabled: hashmap.remove("enabled").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl device_posture_integration::Guest for Component {
    fn invoke(
        name: String,
        args: device_posture_integration::Args,
    ) -> device_posture_integration::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/devicePostureIntegration:DevicePostureIntegration".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "configs".into(),
                    value: args.configs,
                },
                ObjectField {
                    name: "identifier".into(),
                    value: args.identifier,
                },
                ObjectField {
                    name: "interval".into(),
                    value: args.interval,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "type".into(),
                    value: args.type_,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "configs".into(),
                },
                ResultField {
                    name: "identifier".into(),
                },
                ResultField {
                    name: "interval".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "type".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        device_posture_integration::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            configs: hashmap.remove("configs").unwrap(),
            identifier: hashmap.remove("identifier").unwrap(),
            interval: hashmap.remove("interval").unwrap(),
            name: hashmap.remove("name").unwrap(),
            type_: hashmap.remove("type").unwrap(),
        }
    }
}
impl device_posture_rule::Guest for Component {
    fn invoke(name: String, args: device_posture_rule::Args) -> device_posture_rule::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/devicePostureRule:DevicePostureRule".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "description".into(),
                    value: args.description,
                },
                ObjectField {
                    name: "expiration".into(),
                    value: args.expiration,
                },
                ObjectField {
                    name: "inputs".into(),
                    value: args.inputs,
                },
                ObjectField {
                    name: "matches".into(),
                    value: args.matches,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "schedule".into(),
                    value: args.schedule,
                },
                ObjectField {
                    name: "type".into(),
                    value: args.type_,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "description".into(),
                },
                ResultField {
                    name: "expiration".into(),
                },
                ResultField {
                    name: "inputs".into(),
                },
                ResultField {
                    name: "matches".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "schedule".into(),
                },
                ResultField {
                    name: "type".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        device_posture_rule::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            description: hashmap.remove("description").unwrap(),
            expiration: hashmap.remove("expiration").unwrap(),
            inputs: hashmap.remove("inputs").unwrap(),
            matches: hashmap.remove("matches").unwrap(),
            name: hashmap.remove("name").unwrap(),
            schedule: hashmap.remove("schedule").unwrap(),
            type_: hashmap.remove("type").unwrap(),
        }
    }
}
impl device_settings_policy::Guest for Component {
    fn invoke(name: String, args: device_settings_policy::Args) -> device_settings_policy::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/deviceSettingsPolicy:DeviceSettingsPolicy".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "allowModeSwitch".into(),
                    value: args.allow_mode_switch,
                },
                ObjectField {
                    name: "allowUpdates".into(),
                    value: args.allow_updates,
                },
                ObjectField {
                    name: "allowedToLeave".into(),
                    value: args.allowed_to_leave,
                },
                ObjectField {
                    name: "autoConnect".into(),
                    value: args.auto_connect,
                },
                ObjectField {
                    name: "captivePortal".into(),
                    value: args.captive_portal,
                },
                ObjectField {
                    name: "default".into(),
                    value: args.default,
                },
                ObjectField {
                    name: "description".into(),
                    value: args.description,
                },
                ObjectField {
                    name: "disableAutoFallback".into(),
                    value: args.disable_auto_fallback,
                },
                ObjectField {
                    name: "enabled".into(),
                    value: args.enabled,
                },
                ObjectField {
                    name: "excludeOfficeIps".into(),
                    value: args.exclude_office_ips,
                },
                ObjectField {
                    name: "match".into(),
                    value: args.match_,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "precedence".into(),
                    value: args.precedence,
                },
                ObjectField {
                    name: "serviceModeV2Mode".into(),
                    value: args.service_mode_v2_mode,
                },
                ObjectField {
                    name: "serviceModeV2Port".into(),
                    value: args.service_mode_v2_port,
                },
                ObjectField {
                    name: "supportUrl".into(),
                    value: args.support_url,
                },
                ObjectField {
                    name: "switchLocked".into(),
                    value: args.switch_locked,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "allowModeSwitch".into(),
                },
                ResultField {
                    name: "allowUpdates".into(),
                },
                ResultField {
                    name: "allowedToLeave".into(),
                },
                ResultField {
                    name: "autoConnect".into(),
                },
                ResultField {
                    name: "captivePortal".into(),
                },
                ResultField {
                    name: "default".into(),
                },
                ResultField {
                    name: "description".into(),
                },
                ResultField {
                    name: "disableAutoFallback".into(),
                },
                ResultField {
                    name: "enabled".into(),
                },
                ResultField {
                    name: "excludeOfficeIps".into(),
                },
                ResultField {
                    name: "match".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "precedence".into(),
                },
                ResultField {
                    name: "serviceModeV2Mode".into(),
                },
                ResultField {
                    name: "serviceModeV2Port".into(),
                },
                ResultField {
                    name: "supportUrl".into(),
                },
                ResultField {
                    name: "switchLocked".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        device_settings_policy::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            allow_mode_switch: hashmap.remove("allowModeSwitch").unwrap(),
            allow_updates: hashmap.remove("allowUpdates").unwrap(),
            allowed_to_leave: hashmap.remove("allowedToLeave").unwrap(),
            auto_connect: hashmap.remove("autoConnect").unwrap(),
            captive_portal: hashmap.remove("captivePortal").unwrap(),
            default: hashmap.remove("default").unwrap(),
            description: hashmap.remove("description").unwrap(),
            disable_auto_fallback: hashmap.remove("disableAutoFallback").unwrap(),
            enabled: hashmap.remove("enabled").unwrap(),
            exclude_office_ips: hashmap.remove("excludeOfficeIps").unwrap(),
            match_: hashmap.remove("match").unwrap(),
            name: hashmap.remove("name").unwrap(),
            precedence: hashmap.remove("precedence").unwrap(),
            service_mode_v2_mode: hashmap.remove("serviceModeV2Mode").unwrap(),
            service_mode_v2_port: hashmap.remove("serviceModeV2Port").unwrap(),
            support_url: hashmap.remove("supportUrl").unwrap(),
            switch_locked: hashmap.remove("switchLocked").unwrap(),
        }
    }
}
impl dlp_profile::Guest for Component {
    fn invoke(name: String, args: dlp_profile::Args) -> dlp_profile::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/dlpProfile:DlpProfile".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "allowedMatchCount".into(),
                    value: args.allowed_match_count,
                },
                ObjectField {
                    name: "contextAwareness".into(),
                    value: args.context_awareness,
                },
                ObjectField {
                    name: "description".into(),
                    value: args.description,
                },
                ObjectField {
                    name: "entries".into(),
                    value: args.entries,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "type".into(),
                    value: args.type_,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "allowedMatchCount".into(),
                },
                ResultField {
                    name: "contextAwareness".into(),
                },
                ResultField {
                    name: "description".into(),
                },
                ResultField {
                    name: "entries".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "type".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        dlp_profile::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            allowed_match_count: hashmap.remove("allowedMatchCount").unwrap(),
            context_awareness: hashmap.remove("contextAwareness").unwrap(),
            description: hashmap.remove("description").unwrap(),
            entries: hashmap.remove("entries").unwrap(),
            name: hashmap.remove("name").unwrap(),
            type_: hashmap.remove("type").unwrap(),
        }
    }
}
impl email_routing_address::Guest for Component {
    fn invoke(name: String, args: email_routing_address::Args) -> email_routing_address::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/emailRoutingAddress:EmailRoutingAddress".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "email".into(),
                    value: args.email,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "created".into(),
                },
                ResultField {
                    name: "email".into(),
                },
                ResultField {
                    name: "modified".into(),
                },
                ResultField { name: "tag".into() },
                ResultField {
                    name: "verified".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        email_routing_address::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            created: hashmap.remove("created").unwrap(),
            email: hashmap.remove("email").unwrap(),
            modified: hashmap.remove("modified").unwrap(),
            tag: hashmap.remove("tag").unwrap(),
            verified: hashmap.remove("verified").unwrap(),
        }
    }
}
impl email_routing_catch_all::Guest for Component {
    fn invoke(name: String, args: email_routing_catch_all::Args) -> email_routing_catch_all::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/emailRoutingCatchAll:EmailRoutingCatchAll".into(),
            name,
            object: vec![
                ObjectField {
                    name: "actions".into(),
                    value: args.actions,
                },
                ObjectField {
                    name: "enabled".into(),
                    value: args.enabled,
                },
                ObjectField {
                    name: "matchers".into(),
                    value: args.matchers,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "actions".into(),
                },
                ResultField {
                    name: "enabled".into(),
                },
                ResultField {
                    name: "matchers".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField { name: "tag".into() },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        email_routing_catch_all::Res {
            actions: hashmap.remove("actions").unwrap(),
            enabled: hashmap.remove("enabled").unwrap(),
            matchers: hashmap.remove("matchers").unwrap(),
            name: hashmap.remove("name").unwrap(),
            tag: hashmap.remove("tag").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl email_routing_rule::Guest for Component {
    fn invoke(name: String, args: email_routing_rule::Args) -> email_routing_rule::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/emailRoutingRule:EmailRoutingRule".into(),
            name,
            object: vec![
                ObjectField {
                    name: "actions".into(),
                    value: args.actions,
                },
                ObjectField {
                    name: "enabled".into(),
                    value: args.enabled,
                },
                ObjectField {
                    name: "matchers".into(),
                    value: args.matchers,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "priority".into(),
                    value: args.priority,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "actions".into(),
                },
                ResultField {
                    name: "enabled".into(),
                },
                ResultField {
                    name: "matchers".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "priority".into(),
                },
                ResultField { name: "tag".into() },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        email_routing_rule::Res {
            actions: hashmap.remove("actions").unwrap(),
            enabled: hashmap.remove("enabled").unwrap(),
            matchers: hashmap.remove("matchers").unwrap(),
            name: hashmap.remove("name").unwrap(),
            priority: hashmap.remove("priority").unwrap(),
            tag: hashmap.remove("tag").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl email_routing_settings::Guest for Component {
    fn invoke(name: String, args: email_routing_settings::Args) -> email_routing_settings::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/emailRoutingSettings:EmailRoutingSettings".into(),
            name,
            object: vec![
                ObjectField {
                    name: "enabled".into(),
                    value: args.enabled,
                },
                ObjectField {
                    name: "skipWizard".into(),
                    value: args.skip_wizard,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "created".into(),
                },
                ResultField {
                    name: "enabled".into(),
                },
                ResultField {
                    name: "modified".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "skipWizard".into(),
                },
                ResultField {
                    name: "status".into(),
                },
                ResultField { name: "tag".into() },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        email_routing_settings::Res {
            created: hashmap.remove("created").unwrap(),
            enabled: hashmap.remove("enabled").unwrap(),
            modified: hashmap.remove("modified").unwrap(),
            name: hashmap.remove("name").unwrap(),
            skip_wizard: hashmap.remove("skipWizard").unwrap(),
            status: hashmap.remove("status").unwrap(),
            tag: hashmap.remove("tag").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl fallback_domain::Guest for Component {
    fn invoke(name: String, args: fallback_domain::Args) -> fallback_domain::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/fallbackDomain:FallbackDomain".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "domains".into(),
                    value: args.domains,
                },
                ObjectField {
                    name: "policyId".into(),
                    value: args.policy_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "domains".into(),
                },
                ResultField {
                    name: "policyId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        fallback_domain::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            domains: hashmap.remove("domains").unwrap(),
            policy_id: hashmap.remove("policyId").unwrap(),
        }
    }
}
impl filter::Guest for Component {
    fn invoke(name: String, args: filter::Args) -> filter::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/filter:Filter".into(),
            name,
            object: vec![
                ObjectField {
                    name: "description".into(),
                    value: args.description,
                },
                ObjectField {
                    name: "expression".into(),
                    value: args.expression,
                },
                ObjectField {
                    name: "paused".into(),
                    value: args.paused,
                },
                ObjectField {
                    name: "ref".into(),
                    value: args.ref_,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "description".into(),
                },
                ResultField {
                    name: "expression".into(),
                },
                ResultField {
                    name: "paused".into(),
                },
                ResultField { name: "ref".into() },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        filter::Res {
            description: hashmap.remove("description").unwrap(),
            expression: hashmap.remove("expression").unwrap(),
            paused: hashmap.remove("paused").unwrap(),
            ref_: hashmap.remove("ref").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl firewall_rule::Guest for Component {
    fn invoke(name: String, args: firewall_rule::Args) -> firewall_rule::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/firewallRule:FirewallRule".into(),
            name,
            object: vec![
                ObjectField {
                    name: "action".into(),
                    value: args.action,
                },
                ObjectField {
                    name: "description".into(),
                    value: args.description,
                },
                ObjectField {
                    name: "filterId".into(),
                    value: args.filter_id,
                },
                ObjectField {
                    name: "paused".into(),
                    value: args.paused,
                },
                ObjectField {
                    name: "priority".into(),
                    value: args.priority,
                },
                ObjectField {
                    name: "products".into(),
                    value: args.products,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "action".into(),
                },
                ResultField {
                    name: "description".into(),
                },
                ResultField {
                    name: "filterId".into(),
                },
                ResultField {
                    name: "paused".into(),
                },
                ResultField {
                    name: "priority".into(),
                },
                ResultField {
                    name: "products".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        firewall_rule::Res {
            action: hashmap.remove("action").unwrap(),
            description: hashmap.remove("description").unwrap(),
            filter_id: hashmap.remove("filterId").unwrap(),
            paused: hashmap.remove("paused").unwrap(),
            priority: hashmap.remove("priority").unwrap(),
            products: hashmap.remove("products").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl gre_tunnel::Guest for Component {
    fn invoke(name: String, args: gre_tunnel::Args) -> gre_tunnel::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/greTunnel:GreTunnel".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "cloudflareGreEndpoint".into(),
                    value: args.cloudflare_gre_endpoint,
                },
                ObjectField {
                    name: "customerGreEndpoint".into(),
                    value: args.customer_gre_endpoint,
                },
                ObjectField {
                    name: "description".into(),
                    value: args.description,
                },
                ObjectField {
                    name: "healthCheckEnabled".into(),
                    value: args.health_check_enabled,
                },
                ObjectField {
                    name: "healthCheckTarget".into(),
                    value: args.health_check_target,
                },
                ObjectField {
                    name: "healthCheckType".into(),
                    value: args.health_check_type,
                },
                ObjectField {
                    name: "interfaceAddress".into(),
                    value: args.interface_address,
                },
                ObjectField {
                    name: "mtu".into(),
                    value: args.mtu,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "ttl".into(),
                    value: args.ttl,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "cloudflareGreEndpoint".into(),
                },
                ResultField {
                    name: "customerGreEndpoint".into(),
                },
                ResultField {
                    name: "description".into(),
                },
                ResultField {
                    name: "healthCheckEnabled".into(),
                },
                ResultField {
                    name: "healthCheckTarget".into(),
                },
                ResultField {
                    name: "healthCheckType".into(),
                },
                ResultField {
                    name: "interfaceAddress".into(),
                },
                ResultField { name: "mtu".into() },
                ResultField {
                    name: "name".into(),
                },
                ResultField { name: "ttl".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        gre_tunnel::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            cloudflare_gre_endpoint: hashmap.remove("cloudflareGreEndpoint").unwrap(),
            customer_gre_endpoint: hashmap.remove("customerGreEndpoint").unwrap(),
            description: hashmap.remove("description").unwrap(),
            health_check_enabled: hashmap.remove("healthCheckEnabled").unwrap(),
            health_check_target: hashmap.remove("healthCheckTarget").unwrap(),
            health_check_type: hashmap.remove("healthCheckType").unwrap(),
            interface_address: hashmap.remove("interfaceAddress").unwrap(),
            mtu: hashmap.remove("mtu").unwrap(),
            name: hashmap.remove("name").unwrap(),
            ttl: hashmap.remove("ttl").unwrap(),
        }
    }
}
impl healthcheck::Guest for Component {
    fn invoke(name: String, args: healthcheck::Args) -> healthcheck::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/healthcheck:Healthcheck".into(),
            name,
            object: vec![
                ObjectField {
                    name: "address".into(),
                    value: args.address,
                },
                ObjectField {
                    name: "allowInsecure".into(),
                    value: args.allow_insecure,
                },
                ObjectField {
                    name: "checkRegions".into(),
                    value: args.check_regions,
                },
                ObjectField {
                    name: "consecutiveFails".into(),
                    value: args.consecutive_fails,
                },
                ObjectField {
                    name: "consecutiveSuccesses".into(),
                    value: args.consecutive_successes,
                },
                ObjectField {
                    name: "description".into(),
                    value: args.description,
                },
                ObjectField {
                    name: "expectedBody".into(),
                    value: args.expected_body,
                },
                ObjectField {
                    name: "expectedCodes".into(),
                    value: args.expected_codes,
                },
                ObjectField {
                    name: "followRedirects".into(),
                    value: args.follow_redirects,
                },
                ObjectField {
                    name: "headers".into(),
                    value: args.headers,
                },
                ObjectField {
                    name: "interval".into(),
                    value: args.interval,
                },
                ObjectField {
                    name: "method".into(),
                    value: args.method,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "path".into(),
                    value: args.path,
                },
                ObjectField {
                    name: "port".into(),
                    value: args.port,
                },
                ObjectField {
                    name: "retries".into(),
                    value: args.retries,
                },
                ObjectField {
                    name: "suspended".into(),
                    value: args.suspended,
                },
                ObjectField {
                    name: "timeout".into(),
                    value: args.timeout,
                },
                ObjectField {
                    name: "type".into(),
                    value: args.type_,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "address".into(),
                },
                ResultField {
                    name: "allowInsecure".into(),
                },
                ResultField {
                    name: "checkRegions".into(),
                },
                ResultField {
                    name: "consecutiveFails".into(),
                },
                ResultField {
                    name: "consecutiveSuccesses".into(),
                },
                ResultField {
                    name: "createdOn".into(),
                },
                ResultField {
                    name: "description".into(),
                },
                ResultField {
                    name: "expectedBody".into(),
                },
                ResultField {
                    name: "expectedCodes".into(),
                },
                ResultField {
                    name: "followRedirects".into(),
                },
                ResultField {
                    name: "headers".into(),
                },
                ResultField {
                    name: "interval".into(),
                },
                ResultField {
                    name: "method".into(),
                },
                ResultField {
                    name: "modifiedOn".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "path".into(),
                },
                ResultField {
                    name: "port".into(),
                },
                ResultField {
                    name: "retries".into(),
                },
                ResultField {
                    name: "suspended".into(),
                },
                ResultField {
                    name: "timeout".into(),
                },
                ResultField {
                    name: "type".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        healthcheck::Res {
            address: hashmap.remove("address").unwrap(),
            allow_insecure: hashmap.remove("allowInsecure").unwrap(),
            check_regions: hashmap.remove("checkRegions").unwrap(),
            consecutive_fails: hashmap.remove("consecutiveFails").unwrap(),
            consecutive_successes: hashmap.remove("consecutiveSuccesses").unwrap(),
            created_on: hashmap.remove("createdOn").unwrap(),
            description: hashmap.remove("description").unwrap(),
            expected_body: hashmap.remove("expectedBody").unwrap(),
            expected_codes: hashmap.remove("expectedCodes").unwrap(),
            follow_redirects: hashmap.remove("followRedirects").unwrap(),
            headers: hashmap.remove("headers").unwrap(),
            interval: hashmap.remove("interval").unwrap(),
            method: hashmap.remove("method").unwrap(),
            modified_on: hashmap.remove("modifiedOn").unwrap(),
            name: hashmap.remove("name").unwrap(),
            path: hashmap.remove("path").unwrap(),
            port: hashmap.remove("port").unwrap(),
            retries: hashmap.remove("retries").unwrap(),
            suspended: hashmap.remove("suspended").unwrap(),
            timeout: hashmap.remove("timeout").unwrap(),
            type_: hashmap.remove("type").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl hostname_tls_setting::Guest for Component {
    fn invoke(name: String, args: hostname_tls_setting::Args) -> hostname_tls_setting::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/hostnameTlsSetting:HostnameTlsSetting".into(),
            name,
            object: vec![
                ObjectField {
                    name: "hostname".into(),
                    value: args.hostname,
                },
                ObjectField {
                    name: "setting".into(),
                    value: args.setting,
                },
                ObjectField {
                    name: "value".into(),
                    value: args.value,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "createdAt".into(),
                },
                ResultField {
                    name: "hostname".into(),
                },
                ResultField {
                    name: "setting".into(),
                },
                ResultField {
                    name: "updatedAt".into(),
                },
                ResultField {
                    name: "value".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        hostname_tls_setting::Res {
            created_at: hashmap.remove("createdAt").unwrap(),
            hostname: hashmap.remove("hostname").unwrap(),
            setting: hashmap.remove("setting").unwrap(),
            updated_at: hashmap.remove("updatedAt").unwrap(),
            value: hashmap.remove("value").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl hostname_tls_setting_ciphers::Guest for Component {
    fn invoke(
        name: String,
        args: hostname_tls_setting_ciphers::Args,
    ) -> hostname_tls_setting_ciphers::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/hostnameTlsSettingCiphers:HostnameTlsSettingCiphers".into(),
            name,
            object: vec![
                ObjectField {
                    name: "hostname".into(),
                    value: args.hostname,
                },
                ObjectField {
                    name: "ports".into(),
                    value: args.ports,
                },
                ObjectField {
                    name: "values".into(),
                    value: args.values,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "createdAt".into(),
                },
                ResultField {
                    name: "hostname".into(),
                },
                ResultField {
                    name: "ports".into(),
                },
                ResultField {
                    name: "updatedAt".into(),
                },
                ResultField {
                    name: "values".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        hostname_tls_setting_ciphers::Res {
            created_at: hashmap.remove("createdAt").unwrap(),
            hostname: hashmap.remove("hostname").unwrap(),
            ports: hashmap.remove("ports").unwrap(),
            updated_at: hashmap.remove("updatedAt").unwrap(),
            values: hashmap.remove("values").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl hyperdrive_config::Guest for Component {
    fn invoke(name: String, args: hyperdrive_config::Args) -> hyperdrive_config::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/hyperdriveConfig:HyperdriveConfig".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "caching".into(),
                    value: args.caching,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "origin".into(),
                    value: args.origin,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "caching".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "origin".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        hyperdrive_config::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            caching: hashmap.remove("caching").unwrap(),
            name: hashmap.remove("name").unwrap(),
            origin: hashmap.remove("origin").unwrap(),
        }
    }
}
impl ipsec_tunnel::Guest for Component {
    fn invoke(name: String, args: ipsec_tunnel::Args) -> ipsec_tunnel::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/ipsecTunnel:IpsecTunnel".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "allowNullCipher".into(),
                    value: args.allow_null_cipher,
                },
                ObjectField {
                    name: "cloudflareEndpoint".into(),
                    value: args.cloudflare_endpoint,
                },
                ObjectField {
                    name: "customerEndpoint".into(),
                    value: args.customer_endpoint,
                },
                ObjectField {
                    name: "description".into(),
                    value: args.description,
                },
                ObjectField {
                    name: "fqdnId".into(),
                    value: args.fqdn_id,
                },
                ObjectField {
                    name: "healthCheckDirection".into(),
                    value: args.health_check_direction,
                },
                ObjectField {
                    name: "healthCheckEnabled".into(),
                    value: args.health_check_enabled,
                },
                ObjectField {
                    name: "healthCheckRate".into(),
                    value: args.health_check_rate,
                },
                ObjectField {
                    name: "healthCheckTarget".into(),
                    value: args.health_check_target,
                },
                ObjectField {
                    name: "healthCheckType".into(),
                    value: args.health_check_type,
                },
                ObjectField {
                    name: "hexId".into(),
                    value: args.hex_id,
                },
                ObjectField {
                    name: "interfaceAddress".into(),
                    value: args.interface_address,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "psk".into(),
                    value: args.psk,
                },
                ObjectField {
                    name: "remoteId".into(),
                    value: args.remote_id,
                },
                ObjectField {
                    name: "userId".into(),
                    value: args.user_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "allowNullCipher".into(),
                },
                ResultField {
                    name: "cloudflareEndpoint".into(),
                },
                ResultField {
                    name: "customerEndpoint".into(),
                },
                ResultField {
                    name: "description".into(),
                },
                ResultField {
                    name: "fqdnId".into(),
                },
                ResultField {
                    name: "healthCheckDirection".into(),
                },
                ResultField {
                    name: "healthCheckEnabled".into(),
                },
                ResultField {
                    name: "healthCheckRate".into(),
                },
                ResultField {
                    name: "healthCheckTarget".into(),
                },
                ResultField {
                    name: "healthCheckType".into(),
                },
                ResultField {
                    name: "hexId".into(),
                },
                ResultField {
                    name: "interfaceAddress".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField { name: "psk".into() },
                ResultField {
                    name: "remoteId".into(),
                },
                ResultField {
                    name: "userId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        ipsec_tunnel::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            allow_null_cipher: hashmap.remove("allowNullCipher").unwrap(),
            cloudflare_endpoint: hashmap.remove("cloudflareEndpoint").unwrap(),
            customer_endpoint: hashmap.remove("customerEndpoint").unwrap(),
            description: hashmap.remove("description").unwrap(),
            fqdn_id: hashmap.remove("fqdnId").unwrap(),
            health_check_direction: hashmap.remove("healthCheckDirection").unwrap(),
            health_check_enabled: hashmap.remove("healthCheckEnabled").unwrap(),
            health_check_rate: hashmap.remove("healthCheckRate").unwrap(),
            health_check_target: hashmap.remove("healthCheckTarget").unwrap(),
            health_check_type: hashmap.remove("healthCheckType").unwrap(),
            hex_id: hashmap.remove("hexId").unwrap(),
            interface_address: hashmap.remove("interfaceAddress").unwrap(),
            name: hashmap.remove("name").unwrap(),
            psk: hashmap.remove("psk").unwrap(),
            remote_id: hashmap.remove("remoteId").unwrap(),
            user_id: hashmap.remove("userId").unwrap(),
        }
    }
}
impl keyless_certificate::Guest for Component {
    fn invoke(name: String, args: keyless_certificate::Args) -> keyless_certificate::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/keylessCertificate:KeylessCertificate".into(),
            name,
            object: vec![
                ObjectField {
                    name: "bundleMethod".into(),
                    value: args.bundle_method,
                },
                ObjectField {
                    name: "certificate".into(),
                    value: args.certificate,
                },
                ObjectField {
                    name: "enabled".into(),
                    value: args.enabled,
                },
                ObjectField {
                    name: "host".into(),
                    value: args.host,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "port".into(),
                    value: args.port,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "bundleMethod".into(),
                },
                ResultField {
                    name: "certificate".into(),
                },
                ResultField {
                    name: "enabled".into(),
                },
                ResultField {
                    name: "host".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "port".into(),
                },
                ResultField {
                    name: "status".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        keyless_certificate::Res {
            bundle_method: hashmap.remove("bundleMethod").unwrap(),
            certificate: hashmap.remove("certificate").unwrap(),
            enabled: hashmap.remove("enabled").unwrap(),
            host: hashmap.remove("host").unwrap(),
            name: hashmap.remove("name").unwrap(),
            port: hashmap.remove("port").unwrap(),
            status: hashmap.remove("status").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl list::Guest for Component {
    fn invoke(name: String, args: list::Args) -> list::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/list:List".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "description".into(),
                    value: args.description,
                },
                ObjectField {
                    name: "items".into(),
                    value: args.items,
                },
                ObjectField {
                    name: "kind".into(),
                    value: args.kind,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "description".into(),
                },
                ResultField {
                    name: "items".into(),
                },
                ResultField {
                    name: "kind".into(),
                },
                ResultField {
                    name: "name".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        list::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            description: hashmap.remove("description").unwrap(),
            items: hashmap.remove("items").unwrap(),
            kind: hashmap.remove("kind").unwrap(),
            name: hashmap.remove("name").unwrap(),
        }
    }
}
impl list_item::Guest for Component {
    fn invoke(name: String, args: list_item::Args) -> list_item::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/listItem:ListItem".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "asn".into(),
                    value: args.asn,
                },
                ObjectField {
                    name: "comment".into(),
                    value: args.comment,
                },
                ObjectField {
                    name: "hostname".into(),
                    value: args.hostname,
                },
                ObjectField {
                    name: "ip".into(),
                    value: args.ip,
                },
                ObjectField {
                    name: "listId".into(),
                    value: args.list_id,
                },
                ObjectField {
                    name: "redirect".into(),
                    value: args.redirect,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField { name: "asn".into() },
                ResultField {
                    name: "comment".into(),
                },
                ResultField {
                    name: "hostname".into(),
                },
                ResultField { name: "ip".into() },
                ResultField {
                    name: "listId".into(),
                },
                ResultField {
                    name: "redirect".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        list_item::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            asn: hashmap.remove("asn").unwrap(),
            comment: hashmap.remove("comment").unwrap(),
            hostname: hashmap.remove("hostname").unwrap(),
            ip: hashmap.remove("ip").unwrap(),
            list_id: hashmap.remove("listId").unwrap(),
            redirect: hashmap.remove("redirect").unwrap(),
        }
    }
}
impl load_balancer::Guest for Component {
    fn invoke(name: String, args: load_balancer::Args) -> load_balancer::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/loadBalancer:LoadBalancer".into(),
            name,
            object: vec![
                ObjectField {
                    name: "adaptiveRoutings".into(),
                    value: args.adaptive_routings,
                },
                ObjectField {
                    name: "countryPools".into(),
                    value: args.country_pools,
                },
                ObjectField {
                    name: "defaultPoolIds".into(),
                    value: args.default_pool_ids,
                },
                ObjectField {
                    name: "description".into(),
                    value: args.description,
                },
                ObjectField {
                    name: "enabled".into(),
                    value: args.enabled,
                },
                ObjectField {
                    name: "fallbackPoolId".into(),
                    value: args.fallback_pool_id,
                },
                ObjectField {
                    name: "locationStrategies".into(),
                    value: args.location_strategies,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "popPools".into(),
                    value: args.pop_pools,
                },
                ObjectField {
                    name: "proxied".into(),
                    value: args.proxied,
                },
                ObjectField {
                    name: "randomSteerings".into(),
                    value: args.random_steerings,
                },
                ObjectField {
                    name: "regionPools".into(),
                    value: args.region_pools,
                },
                ObjectField {
                    name: "rules".into(),
                    value: args.rules,
                },
                ObjectField {
                    name: "sessionAffinity".into(),
                    value: args.session_affinity,
                },
                ObjectField {
                    name: "sessionAffinityAttributes".into(),
                    value: args.session_affinity_attributes,
                },
                ObjectField {
                    name: "sessionAffinityTtl".into(),
                    value: args.session_affinity_ttl,
                },
                ObjectField {
                    name: "steeringPolicy".into(),
                    value: args.steering_policy,
                },
                ObjectField {
                    name: "ttl".into(),
                    value: args.ttl,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "adaptiveRoutings".into(),
                },
                ResultField {
                    name: "countryPools".into(),
                },
                ResultField {
                    name: "createdOn".into(),
                },
                ResultField {
                    name: "defaultPoolIds".into(),
                },
                ResultField {
                    name: "description".into(),
                },
                ResultField {
                    name: "enabled".into(),
                },
                ResultField {
                    name: "fallbackPoolId".into(),
                },
                ResultField {
                    name: "locationStrategies".into(),
                },
                ResultField {
                    name: "modifiedOn".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "popPools".into(),
                },
                ResultField {
                    name: "proxied".into(),
                },
                ResultField {
                    name: "randomSteerings".into(),
                },
                ResultField {
                    name: "regionPools".into(),
                },
                ResultField {
                    name: "rules".into(),
                },
                ResultField {
                    name: "sessionAffinity".into(),
                },
                ResultField {
                    name: "sessionAffinityAttributes".into(),
                },
                ResultField {
                    name: "sessionAffinityTtl".into(),
                },
                ResultField {
                    name: "steeringPolicy".into(),
                },
                ResultField { name: "ttl".into() },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        load_balancer::Res {
            adaptive_routings: hashmap.remove("adaptiveRoutings").unwrap(),
            country_pools: hashmap.remove("countryPools").unwrap(),
            created_on: hashmap.remove("createdOn").unwrap(),
            default_pool_ids: hashmap.remove("defaultPoolIds").unwrap(),
            description: hashmap.remove("description").unwrap(),
            enabled: hashmap.remove("enabled").unwrap(),
            fallback_pool_id: hashmap.remove("fallbackPoolId").unwrap(),
            location_strategies: hashmap.remove("locationStrategies").unwrap(),
            modified_on: hashmap.remove("modifiedOn").unwrap(),
            name: hashmap.remove("name").unwrap(),
            pop_pools: hashmap.remove("popPools").unwrap(),
            proxied: hashmap.remove("proxied").unwrap(),
            random_steerings: hashmap.remove("randomSteerings").unwrap(),
            region_pools: hashmap.remove("regionPools").unwrap(),
            rules: hashmap.remove("rules").unwrap(),
            session_affinity: hashmap.remove("sessionAffinity").unwrap(),
            session_affinity_attributes: hashmap.remove("sessionAffinityAttributes").unwrap(),
            session_affinity_ttl: hashmap.remove("sessionAffinityTtl").unwrap(),
            steering_policy: hashmap.remove("steeringPolicy").unwrap(),
            ttl: hashmap.remove("ttl").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl load_balancer_monitor::Guest for Component {
    fn invoke(name: String, args: load_balancer_monitor::Args) -> load_balancer_monitor::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/loadBalancerMonitor:LoadBalancerMonitor".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "allowInsecure".into(),
                    value: args.allow_insecure,
                },
                ObjectField {
                    name: "consecutiveDown".into(),
                    value: args.consecutive_down,
                },
                ObjectField {
                    name: "consecutiveUp".into(),
                    value: args.consecutive_up,
                },
                ObjectField {
                    name: "description".into(),
                    value: args.description,
                },
                ObjectField {
                    name: "expectedBody".into(),
                    value: args.expected_body,
                },
                ObjectField {
                    name: "expectedCodes".into(),
                    value: args.expected_codes,
                },
                ObjectField {
                    name: "followRedirects".into(),
                    value: args.follow_redirects,
                },
                ObjectField {
                    name: "headers".into(),
                    value: args.headers,
                },
                ObjectField {
                    name: "interval".into(),
                    value: args.interval,
                },
                ObjectField {
                    name: "method".into(),
                    value: args.method,
                },
                ObjectField {
                    name: "path".into(),
                    value: args.path,
                },
                ObjectField {
                    name: "port".into(),
                    value: args.port,
                },
                ObjectField {
                    name: "probeZone".into(),
                    value: args.probe_zone,
                },
                ObjectField {
                    name: "retries".into(),
                    value: args.retries,
                },
                ObjectField {
                    name: "timeout".into(),
                    value: args.timeout,
                },
                ObjectField {
                    name: "type".into(),
                    value: args.type_,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "allowInsecure".into(),
                },
                ResultField {
                    name: "consecutiveDown".into(),
                },
                ResultField {
                    name: "consecutiveUp".into(),
                },
                ResultField {
                    name: "createdOn".into(),
                },
                ResultField {
                    name: "description".into(),
                },
                ResultField {
                    name: "expectedBody".into(),
                },
                ResultField {
                    name: "expectedCodes".into(),
                },
                ResultField {
                    name: "followRedirects".into(),
                },
                ResultField {
                    name: "headers".into(),
                },
                ResultField {
                    name: "interval".into(),
                },
                ResultField {
                    name: "method".into(),
                },
                ResultField {
                    name: "modifiedOn".into(),
                },
                ResultField {
                    name: "path".into(),
                },
                ResultField {
                    name: "port".into(),
                },
                ResultField {
                    name: "probeZone".into(),
                },
                ResultField {
                    name: "retries".into(),
                },
                ResultField {
                    name: "timeout".into(),
                },
                ResultField {
                    name: "type".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        load_balancer_monitor::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            allow_insecure: hashmap.remove("allowInsecure").unwrap(),
            consecutive_down: hashmap.remove("consecutiveDown").unwrap(),
            consecutive_up: hashmap.remove("consecutiveUp").unwrap(),
            created_on: hashmap.remove("createdOn").unwrap(),
            description: hashmap.remove("description").unwrap(),
            expected_body: hashmap.remove("expectedBody").unwrap(),
            expected_codes: hashmap.remove("expectedCodes").unwrap(),
            follow_redirects: hashmap.remove("followRedirects").unwrap(),
            headers: hashmap.remove("headers").unwrap(),
            interval: hashmap.remove("interval").unwrap(),
            method: hashmap.remove("method").unwrap(),
            modified_on: hashmap.remove("modifiedOn").unwrap(),
            path: hashmap.remove("path").unwrap(),
            port: hashmap.remove("port").unwrap(),
            probe_zone: hashmap.remove("probeZone").unwrap(),
            retries: hashmap.remove("retries").unwrap(),
            timeout: hashmap.remove("timeout").unwrap(),
            type_: hashmap.remove("type").unwrap(),
        }
    }
}
impl load_balancer_pool::Guest for Component {
    fn invoke(name: String, args: load_balancer_pool::Args) -> load_balancer_pool::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/loadBalancerPool:LoadBalancerPool".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "checkRegions".into(),
                    value: args.check_regions,
                },
                ObjectField {
                    name: "description".into(),
                    value: args.description,
                },
                ObjectField {
                    name: "enabled".into(),
                    value: args.enabled,
                },
                ObjectField {
                    name: "latitude".into(),
                    value: args.latitude,
                },
                ObjectField {
                    name: "loadSheddings".into(),
                    value: args.load_sheddings,
                },
                ObjectField {
                    name: "longitude".into(),
                    value: args.longitude,
                },
                ObjectField {
                    name: "minimumOrigins".into(),
                    value: args.minimum_origins,
                },
                ObjectField {
                    name: "monitor".into(),
                    value: args.monitor,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "notificationEmail".into(),
                    value: args.notification_email,
                },
                ObjectField {
                    name: "originSteerings".into(),
                    value: args.origin_steerings,
                },
                ObjectField {
                    name: "origins".into(),
                    value: args.origins,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "checkRegions".into(),
                },
                ResultField {
                    name: "createdOn".into(),
                },
                ResultField {
                    name: "description".into(),
                },
                ResultField {
                    name: "enabled".into(),
                },
                ResultField {
                    name: "latitude".into(),
                },
                ResultField {
                    name: "loadSheddings".into(),
                },
                ResultField {
                    name: "longitude".into(),
                },
                ResultField {
                    name: "minimumOrigins".into(),
                },
                ResultField {
                    name: "modifiedOn".into(),
                },
                ResultField {
                    name: "monitor".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "notificationEmail".into(),
                },
                ResultField {
                    name: "originSteerings".into(),
                },
                ResultField {
                    name: "origins".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        load_balancer_pool::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            check_regions: hashmap.remove("checkRegions").unwrap(),
            created_on: hashmap.remove("createdOn").unwrap(),
            description: hashmap.remove("description").unwrap(),
            enabled: hashmap.remove("enabled").unwrap(),
            latitude: hashmap.remove("latitude").unwrap(),
            load_sheddings: hashmap.remove("loadSheddings").unwrap(),
            longitude: hashmap.remove("longitude").unwrap(),
            minimum_origins: hashmap.remove("minimumOrigins").unwrap(),
            modified_on: hashmap.remove("modifiedOn").unwrap(),
            monitor: hashmap.remove("monitor").unwrap(),
            name: hashmap.remove("name").unwrap(),
            notification_email: hashmap.remove("notificationEmail").unwrap(),
            origin_steerings: hashmap.remove("originSteerings").unwrap(),
            origins: hashmap.remove("origins").unwrap(),
        }
    }
}
impl logpull_retention::Guest for Component {
    fn invoke(name: String, args: logpull_retention::Args) -> logpull_retention::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/logpullRetention:LogpullRetention".into(),
            name,
            object: vec![
                ObjectField {
                    name: "enabled".into(),
                    value: args.enabled,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "enabled".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        logpull_retention::Res {
            enabled: hashmap.remove("enabled").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl logpush_job::Guest for Component {
    fn invoke(name: String, args: logpush_job::Args) -> logpush_job::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/logpushJob:LogpushJob".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "dataset".into(),
                    value: args.dataset,
                },
                ObjectField {
                    name: "destinationConf".into(),
                    value: args.destination_conf,
                },
                ObjectField {
                    name: "enabled".into(),
                    value: args.enabled,
                },
                ObjectField {
                    name: "filter".into(),
                    value: args.filter,
                },
                ObjectField {
                    name: "frequency".into(),
                    value: args.frequency,
                },
                ObjectField {
                    name: "kind".into(),
                    value: args.kind,
                },
                ObjectField {
                    name: "logpullOptions".into(),
                    value: args.logpull_options,
                },
                ObjectField {
                    name: "maxUploadBytes".into(),
                    value: args.max_upload_bytes,
                },
                ObjectField {
                    name: "maxUploadIntervalSeconds".into(),
                    value: args.max_upload_interval_seconds,
                },
                ObjectField {
                    name: "maxUploadRecords".into(),
                    value: args.max_upload_records,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "outputOptions".into(),
                    value: args.output_options,
                },
                ObjectField {
                    name: "ownershipChallenge".into(),
                    value: args.ownership_challenge,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "dataset".into(),
                },
                ResultField {
                    name: "destinationConf".into(),
                },
                ResultField {
                    name: "enabled".into(),
                },
                ResultField {
                    name: "filter".into(),
                },
                ResultField {
                    name: "frequency".into(),
                },
                ResultField {
                    name: "kind".into(),
                },
                ResultField {
                    name: "logpullOptions".into(),
                },
                ResultField {
                    name: "maxUploadBytes".into(),
                },
                ResultField {
                    name: "maxUploadIntervalSeconds".into(),
                },
                ResultField {
                    name: "maxUploadRecords".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "outputOptions".into(),
                },
                ResultField {
                    name: "ownershipChallenge".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        logpush_job::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            dataset: hashmap.remove("dataset").unwrap(),
            destination_conf: hashmap.remove("destinationConf").unwrap(),
            enabled: hashmap.remove("enabled").unwrap(),
            filter: hashmap.remove("filter").unwrap(),
            frequency: hashmap.remove("frequency").unwrap(),
            kind: hashmap.remove("kind").unwrap(),
            logpull_options: hashmap.remove("logpullOptions").unwrap(),
            max_upload_bytes: hashmap.remove("maxUploadBytes").unwrap(),
            max_upload_interval_seconds: hashmap.remove("maxUploadIntervalSeconds").unwrap(),
            max_upload_records: hashmap.remove("maxUploadRecords").unwrap(),
            name: hashmap.remove("name").unwrap(),
            output_options: hashmap.remove("outputOptions").unwrap(),
            ownership_challenge: hashmap.remove("ownershipChallenge").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl logpush_ownership_challenge::Guest for Component {
    fn invoke(
        name: String,
        args: logpush_ownership_challenge::Args,
    ) -> logpush_ownership_challenge::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/logpushOwnershipChallenge:LogpushOwnershipChallenge".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "destinationConf".into(),
                    value: args.destination_conf,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "destinationConf".into(),
                },
                ResultField {
                    name: "ownershipChallengeFilename".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        logpush_ownership_challenge::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            destination_conf: hashmap.remove("destinationConf").unwrap(),
            ownership_challenge_filename: hashmap.remove("ownershipChallengeFilename").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl magic_firewall_ruleset::Guest for Component {
    fn invoke(name: String, args: magic_firewall_ruleset::Args) -> magic_firewall_ruleset::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/magicFirewallRuleset:MagicFirewallRuleset".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "description".into(),
                    value: args.description,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "rules".into(),
                    value: args.rules,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "description".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "rules".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        magic_firewall_ruleset::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            description: hashmap.remove("description").unwrap(),
            name: hashmap.remove("name").unwrap(),
            rules: hashmap.remove("rules").unwrap(),
        }
    }
}
impl managed_headers::Guest for Component {
    fn invoke(name: String, args: managed_headers::Args) -> managed_headers::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/managedHeaders:ManagedHeaders".into(),
            name,
            object: vec![
                ObjectField {
                    name: "managedRequestHeaders".into(),
                    value: args.managed_request_headers,
                },
                ObjectField {
                    name: "managedResponseHeaders".into(),
                    value: args.managed_response_headers,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "managedRequestHeaders".into(),
                },
                ResultField {
                    name: "managedResponseHeaders".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        managed_headers::Res {
            managed_request_headers: hashmap.remove("managedRequestHeaders").unwrap(),
            managed_response_headers: hashmap.remove("managedResponseHeaders").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl mtls_certificate::Guest for Component {
    fn invoke(name: String, args: mtls_certificate::Args) -> mtls_certificate::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/mtlsCertificate:MtlsCertificate".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "ca".into(),
                    value: args.ca,
                },
                ObjectField {
                    name: "certificates".into(),
                    value: args.certificates,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "privateKey".into(),
                    value: args.private_key,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField { name: "ca".into() },
                ResultField {
                    name: "certificates".into(),
                },
                ResultField {
                    name: "expiresOn".into(),
                },
                ResultField {
                    name: "issuer".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "privateKey".into(),
                },
                ResultField {
                    name: "serialNumber".into(),
                },
                ResultField {
                    name: "signature".into(),
                },
                ResultField {
                    name: "uploadedOn".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        mtls_certificate::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            ca: hashmap.remove("ca").unwrap(),
            certificates: hashmap.remove("certificates").unwrap(),
            expires_on: hashmap.remove("expiresOn").unwrap(),
            issuer: hashmap.remove("issuer").unwrap(),
            name: hashmap.remove("name").unwrap(),
            private_key: hashmap.remove("privateKey").unwrap(),
            serial_number: hashmap.remove("serialNumber").unwrap(),
            signature: hashmap.remove("signature").unwrap(),
            uploaded_on: hashmap.remove("uploadedOn").unwrap(),
        }
    }
}
impl notification_policy::Guest for Component {
    fn invoke(name: String, args: notification_policy::Args) -> notification_policy::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/notificationPolicy:NotificationPolicy".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "alertType".into(),
                    value: args.alert_type,
                },
                ObjectField {
                    name: "description".into(),
                    value: args.description,
                },
                ObjectField {
                    name: "emailIntegrations".into(),
                    value: args.email_integrations,
                },
                ObjectField {
                    name: "enabled".into(),
                    value: args.enabled,
                },
                ObjectField {
                    name: "filters".into(),
                    value: args.filters,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "pagerdutyIntegrations".into(),
                    value: args.pagerduty_integrations,
                },
                ObjectField {
                    name: "webhooksIntegrations".into(),
                    value: args.webhooks_integrations,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "alertType".into(),
                },
                ResultField {
                    name: "created".into(),
                },
                ResultField {
                    name: "description".into(),
                },
                ResultField {
                    name: "emailIntegrations".into(),
                },
                ResultField {
                    name: "enabled".into(),
                },
                ResultField {
                    name: "filters".into(),
                },
                ResultField {
                    name: "modified".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "pagerdutyIntegrations".into(),
                },
                ResultField {
                    name: "webhooksIntegrations".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        notification_policy::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            alert_type: hashmap.remove("alertType").unwrap(),
            created: hashmap.remove("created").unwrap(),
            description: hashmap.remove("description").unwrap(),
            email_integrations: hashmap.remove("emailIntegrations").unwrap(),
            enabled: hashmap.remove("enabled").unwrap(),
            filters: hashmap.remove("filters").unwrap(),
            modified: hashmap.remove("modified").unwrap(),
            name: hashmap.remove("name").unwrap(),
            pagerduty_integrations: hashmap.remove("pagerdutyIntegrations").unwrap(),
            webhooks_integrations: hashmap.remove("webhooksIntegrations").unwrap(),
        }
    }
}
impl notification_policy_webhooks::Guest for Component {
    fn invoke(
        name: String,
        args: notification_policy_webhooks::Args,
    ) -> notification_policy_webhooks::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/notificationPolicyWebhooks:NotificationPolicyWebhooks".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "secret".into(),
                    value: args.secret,
                },
                ObjectField {
                    name: "url".into(),
                    value: args.url,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "createdAt".into(),
                },
                ResultField {
                    name: "lastFailure".into(),
                },
                ResultField {
                    name: "lastSuccess".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "secret".into(),
                },
                ResultField {
                    name: "type".into(),
                },
                ResultField { name: "url".into() },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        notification_policy_webhooks::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            created_at: hashmap.remove("createdAt").unwrap(),
            last_failure: hashmap.remove("lastFailure").unwrap(),
            last_success: hashmap.remove("lastSuccess").unwrap(),
            name: hashmap.remove("name").unwrap(),
            secret: hashmap.remove("secret").unwrap(),
            type_: hashmap.remove("type").unwrap(),
            url: hashmap.remove("url").unwrap(),
        }
    }
}
impl observatory_scheduled_test::Guest for Component {
    fn invoke(
        name: String,
        args: observatory_scheduled_test::Args,
    ) -> observatory_scheduled_test::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/observatoryScheduledTest:ObservatoryScheduledTest".into(),
            name,
            object: vec![
                ObjectField {
                    name: "frequency".into(),
                    value: args.frequency,
                },
                ObjectField {
                    name: "region".into(),
                    value: args.region,
                },
                ObjectField {
                    name: "url".into(),
                    value: args.url,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "frequency".into(),
                },
                ResultField {
                    name: "region".into(),
                },
                ResultField { name: "url".into() },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        observatory_scheduled_test::Res {
            frequency: hashmap.remove("frequency").unwrap(),
            region: hashmap.remove("region").unwrap(),
            url: hashmap.remove("url").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl origin_ca_certificate::Guest for Component {
    fn invoke(name: String, args: origin_ca_certificate::Args) -> origin_ca_certificate::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/originCaCertificate:OriginCaCertificate".into(),
            name,
            object: vec![
                ObjectField {
                    name: "csr".into(),
                    value: args.csr,
                },
                ObjectField {
                    name: "hostnames".into(),
                    value: args.hostnames,
                },
                ObjectField {
                    name: "minDaysForRenewal".into(),
                    value: args.min_days_for_renewal,
                },
                ObjectField {
                    name: "requestType".into(),
                    value: args.request_type,
                },
                ObjectField {
                    name: "requestedValidity".into(),
                    value: args.requested_validity,
                },
            ],
            results: vec![
                ResultField {
                    name: "certificate".into(),
                },
                ResultField { name: "csr".into() },
                ResultField {
                    name: "expiresOn".into(),
                },
                ResultField {
                    name: "hostnames".into(),
                },
                ResultField {
                    name: "minDaysForRenewal".into(),
                },
                ResultField {
                    name: "requestType".into(),
                },
                ResultField {
                    name: "requestedValidity".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        origin_ca_certificate::Res {
            certificate: hashmap.remove("certificate").unwrap(),
            csr: hashmap.remove("csr").unwrap(),
            expires_on: hashmap.remove("expiresOn").unwrap(),
            hostnames: hashmap.remove("hostnames").unwrap(),
            min_days_for_renewal: hashmap.remove("minDaysForRenewal").unwrap(),
            request_type: hashmap.remove("requestType").unwrap(),
            requested_validity: hashmap.remove("requestedValidity").unwrap(),
        }
    }
}
impl page_rule::Guest for Component {
    fn invoke(name: String, args: page_rule::Args) -> page_rule::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/pageRule:PageRule".into(),
            name,
            object: vec![
                ObjectField {
                    name: "actions".into(),
                    value: args.actions,
                },
                ObjectField {
                    name: "priority".into(),
                    value: args.priority,
                },
                ObjectField {
                    name: "status".into(),
                    value: args.status,
                },
                ObjectField {
                    name: "target".into(),
                    value: args.target,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "actions".into(),
                },
                ResultField {
                    name: "priority".into(),
                },
                ResultField {
                    name: "status".into(),
                },
                ResultField {
                    name: "target".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        page_rule::Res {
            actions: hashmap.remove("actions").unwrap(),
            priority: hashmap.remove("priority").unwrap(),
            status: hashmap.remove("status").unwrap(),
            target: hashmap.remove("target").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl pages_domain::Guest for Component {
    fn invoke(name: String, args: pages_domain::Args) -> pages_domain::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/pagesDomain:PagesDomain".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "domain".into(),
                    value: args.domain,
                },
                ObjectField {
                    name: "projectName".into(),
                    value: args.project_name,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "domain".into(),
                },
                ResultField {
                    name: "projectName".into(),
                },
                ResultField {
                    name: "status".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        pages_domain::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            domain: hashmap.remove("domain").unwrap(),
            project_name: hashmap.remove("projectName").unwrap(),
            status: hashmap.remove("status").unwrap(),
        }
    }
}
impl pages_project::Guest for Component {
    fn invoke(name: String, args: pages_project::Args) -> pages_project::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/pagesProject:PagesProject".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "buildConfig".into(),
                    value: args.build_config,
                },
                ObjectField {
                    name: "deploymentConfigs".into(),
                    value: args.deployment_configs,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "productionBranch".into(),
                    value: args.production_branch,
                },
                ObjectField {
                    name: "source".into(),
                    value: args.source,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "buildConfig".into(),
                },
                ResultField {
                    name: "createdOn".into(),
                },
                ResultField {
                    name: "deploymentConfigs".into(),
                },
                ResultField {
                    name: "domains".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "productionBranch".into(),
                },
                ResultField {
                    name: "source".into(),
                },
                ResultField {
                    name: "subdomain".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        pages_project::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            build_config: hashmap.remove("buildConfig").unwrap(),
            created_on: hashmap.remove("createdOn").unwrap(),
            deployment_configs: hashmap.remove("deploymentConfigs").unwrap(),
            domains: hashmap.remove("domains").unwrap(),
            name: hashmap.remove("name").unwrap(),
            production_branch: hashmap.remove("productionBranch").unwrap(),
            source: hashmap.remove("source").unwrap(),
            subdomain: hashmap.remove("subdomain").unwrap(),
        }
    }
}
impl queue::Guest for Component {
    fn invoke(name: String, args: queue::Args) -> queue::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/queue:Queue".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "name".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        queue::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            name: hashmap.remove("name").unwrap(),
        }
    }
}
impl r2_bucket::Guest for Component {
    fn invoke(name: String, args: r2_bucket::Args) -> r2_bucket::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/r2Bucket:R2Bucket".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "location".into(),
                    value: args.location,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "location".into(),
                },
                ResultField {
                    name: "name".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        r2_bucket::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            location: hashmap.remove("location").unwrap(),
            name: hashmap.remove("name").unwrap(),
        }
    }
}
impl rate_limit::Guest for Component {
    fn invoke(name: String, args: rate_limit::Args) -> rate_limit::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/rateLimit:RateLimit".into(),
            name,
            object: vec![
                ObjectField {
                    name: "action".into(),
                    value: args.action,
                },
                ObjectField {
                    name: "bypassUrlPatterns".into(),
                    value: args.bypass_url_patterns,
                },
                ObjectField {
                    name: "correlate".into(),
                    value: args.correlate,
                },
                ObjectField {
                    name: "description".into(),
                    value: args.description,
                },
                ObjectField {
                    name: "disabled".into(),
                    value: args.disabled,
                },
                ObjectField {
                    name: "match".into(),
                    value: args.match_,
                },
                ObjectField {
                    name: "period".into(),
                    value: args.period,
                },
                ObjectField {
                    name: "threshold".into(),
                    value: args.threshold,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "action".into(),
                },
                ResultField {
                    name: "bypassUrlPatterns".into(),
                },
                ResultField {
                    name: "correlate".into(),
                },
                ResultField {
                    name: "description".into(),
                },
                ResultField {
                    name: "disabled".into(),
                },
                ResultField {
                    name: "match".into(),
                },
                ResultField {
                    name: "period".into(),
                },
                ResultField {
                    name: "threshold".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        rate_limit::Res {
            action: hashmap.remove("action").unwrap(),
            bypass_url_patterns: hashmap.remove("bypassUrlPatterns").unwrap(),
            correlate: hashmap.remove("correlate").unwrap(),
            description: hashmap.remove("description").unwrap(),
            disabled: hashmap.remove("disabled").unwrap(),
            match_: hashmap.remove("match").unwrap(),
            period: hashmap.remove("period").unwrap(),
            threshold: hashmap.remove("threshold").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl record::Guest for Component {
    fn invoke(name: String, args: record::Args) -> record::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/record:Record".into(),
            name,
            object: vec![
                ObjectField {
                    name: "allowOverwrite".into(),
                    value: args.allow_overwrite,
                },
                ObjectField {
                    name: "comment".into(),
                    value: args.comment,
                },
                ObjectField {
                    name: "data".into(),
                    value: args.data,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "priority".into(),
                    value: args.priority,
                },
                ObjectField {
                    name: "proxied".into(),
                    value: args.proxied,
                },
                ObjectField {
                    name: "tags".into(),
                    value: args.tags,
                },
                ObjectField {
                    name: "ttl".into(),
                    value: args.ttl,
                },
                ObjectField {
                    name: "type".into(),
                    value: args.type_,
                },
                ObjectField {
                    name: "value".into(),
                    value: args.value,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "allowOverwrite".into(),
                },
                ResultField {
                    name: "comment".into(),
                },
                ResultField {
                    name: "createdOn".into(),
                },
                ResultField {
                    name: "data".into(),
                },
                ResultField {
                    name: "hostname".into(),
                },
                ResultField {
                    name: "metadata".into(),
                },
                ResultField {
                    name: "modifiedOn".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "priority".into(),
                },
                ResultField {
                    name: "proxiable".into(),
                },
                ResultField {
                    name: "proxied".into(),
                },
                ResultField {
                    name: "tags".into(),
                },
                ResultField { name: "ttl".into() },
                ResultField {
                    name: "type".into(),
                },
                ResultField {
                    name: "value".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        record::Res {
            allow_overwrite: hashmap.remove("allowOverwrite").unwrap(),
            comment: hashmap.remove("comment").unwrap(),
            created_on: hashmap.remove("createdOn").unwrap(),
            data: hashmap.remove("data").unwrap(),
            hostname: hashmap.remove("hostname").unwrap(),
            metadata: hashmap.remove("metadata").unwrap(),
            modified_on: hashmap.remove("modifiedOn").unwrap(),
            name: hashmap.remove("name").unwrap(),
            priority: hashmap.remove("priority").unwrap(),
            proxiable: hashmap.remove("proxiable").unwrap(),
            proxied: hashmap.remove("proxied").unwrap(),
            tags: hashmap.remove("tags").unwrap(),
            ttl: hashmap.remove("ttl").unwrap(),
            type_: hashmap.remove("type").unwrap(),
            value: hashmap.remove("value").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl regional_hostname::Guest for Component {
    fn invoke(name: String, args: regional_hostname::Args) -> regional_hostname::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/regionalHostname:RegionalHostname".into(),
            name,
            object: vec![
                ObjectField {
                    name: "hostname".into(),
                    value: args.hostname,
                },
                ObjectField {
                    name: "regionKey".into(),
                    value: args.region_key,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "createdOn".into(),
                },
                ResultField {
                    name: "hostname".into(),
                },
                ResultField {
                    name: "regionKey".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        regional_hostname::Res {
            created_on: hashmap.remove("createdOn").unwrap(),
            hostname: hashmap.remove("hostname").unwrap(),
            region_key: hashmap.remove("regionKey").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl regional_tiered_cache::Guest for Component {
    fn invoke(name: String, args: regional_tiered_cache::Args) -> regional_tiered_cache::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/regionalTieredCache:RegionalTieredCache".into(),
            name,
            object: vec![
                ObjectField {
                    name: "value".into(),
                    value: args.value,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "value".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        regional_tiered_cache::Res {
            value: hashmap.remove("value").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl ruleset::Guest for Component {
    fn invoke(name: String, args: ruleset::Args) -> ruleset::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/ruleset:Ruleset".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "description".into(),
                    value: args.description,
                },
                ObjectField {
                    name: "kind".into(),
                    value: args.kind,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "phase".into(),
                    value: args.phase,
                },
                ObjectField {
                    name: "rules".into(),
                    value: args.rules,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "description".into(),
                },
                ResultField {
                    name: "kind".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "phase".into(),
                },
                ResultField {
                    name: "rules".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        ruleset::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            description: hashmap.remove("description").unwrap(),
            kind: hashmap.remove("kind").unwrap(),
            name: hashmap.remove("name").unwrap(),
            phase: hashmap.remove("phase").unwrap(),
            rules: hashmap.remove("rules").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl spectrum_application::Guest for Component {
    fn invoke(name: String, args: spectrum_application::Args) -> spectrum_application::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/spectrumApplication:SpectrumApplication".into(),
            name,
            object: vec![
                ObjectField {
                    name: "argoSmartRouting".into(),
                    value: args.argo_smart_routing,
                },
                ObjectField {
                    name: "dns".into(),
                    value: args.dns,
                },
                ObjectField {
                    name: "edgeIps".into(),
                    value: args.edge_ips,
                },
                ObjectField {
                    name: "ipFirewall".into(),
                    value: args.ip_firewall,
                },
                ObjectField {
                    name: "originDirects".into(),
                    value: args.origin_directs,
                },
                ObjectField {
                    name: "originDns".into(),
                    value: args.origin_dns,
                },
                ObjectField {
                    name: "originPort".into(),
                    value: args.origin_port,
                },
                ObjectField {
                    name: "originPortRange".into(),
                    value: args.origin_port_range,
                },
                ObjectField {
                    name: "protocol".into(),
                    value: args.protocol,
                },
                ObjectField {
                    name: "proxyProtocol".into(),
                    value: args.proxy_protocol,
                },
                ObjectField {
                    name: "tls".into(),
                    value: args.tls,
                },
                ObjectField {
                    name: "trafficType".into(),
                    value: args.traffic_type,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "argoSmartRouting".into(),
                },
                ResultField { name: "dns".into() },
                ResultField {
                    name: "edgeIps".into(),
                },
                ResultField {
                    name: "ipFirewall".into(),
                },
                ResultField {
                    name: "originDirects".into(),
                },
                ResultField {
                    name: "originDns".into(),
                },
                ResultField {
                    name: "originPort".into(),
                },
                ResultField {
                    name: "originPortRange".into(),
                },
                ResultField {
                    name: "protocol".into(),
                },
                ResultField {
                    name: "proxyProtocol".into(),
                },
                ResultField { name: "tls".into() },
                ResultField {
                    name: "trafficType".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        spectrum_application::Res {
            argo_smart_routing: hashmap.remove("argoSmartRouting").unwrap(),
            dns: hashmap.remove("dns").unwrap(),
            edge_ips: hashmap.remove("edgeIps").unwrap(),
            ip_firewall: hashmap.remove("ipFirewall").unwrap(),
            origin_directs: hashmap.remove("originDirects").unwrap(),
            origin_dns: hashmap.remove("originDns").unwrap(),
            origin_port: hashmap.remove("originPort").unwrap(),
            origin_port_range: hashmap.remove("originPortRange").unwrap(),
            protocol: hashmap.remove("protocol").unwrap(),
            proxy_protocol: hashmap.remove("proxyProtocol").unwrap(),
            tls: hashmap.remove("tls").unwrap(),
            traffic_type: hashmap.remove("trafficType").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl split_tunnel::Guest for Component {
    fn invoke(name: String, args: split_tunnel::Args) -> split_tunnel::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/splitTunnel:SplitTunnel".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "mode".into(),
                    value: args.mode,
                },
                ObjectField {
                    name: "policyId".into(),
                    value: args.policy_id,
                },
                ObjectField {
                    name: "tunnels".into(),
                    value: args.tunnels,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "mode".into(),
                },
                ResultField {
                    name: "policyId".into(),
                },
                ResultField {
                    name: "tunnels".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        split_tunnel::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            mode: hashmap.remove("mode").unwrap(),
            policy_id: hashmap.remove("policyId").unwrap(),
            tunnels: hashmap.remove("tunnels").unwrap(),
        }
    }
}
impl static_route::Guest for Component {
    fn invoke(name: String, args: static_route::Args) -> static_route::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/staticRoute:StaticRoute".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "coloNames".into(),
                    value: args.colo_names,
                },
                ObjectField {
                    name: "coloRegions".into(),
                    value: args.colo_regions,
                },
                ObjectField {
                    name: "description".into(),
                    value: args.description,
                },
                ObjectField {
                    name: "nexthop".into(),
                    value: args.nexthop,
                },
                ObjectField {
                    name: "prefix".into(),
                    value: args.prefix,
                },
                ObjectField {
                    name: "priority".into(),
                    value: args.priority,
                },
                ObjectField {
                    name: "weight".into(),
                    value: args.weight,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "coloNames".into(),
                },
                ResultField {
                    name: "coloRegions".into(),
                },
                ResultField {
                    name: "description".into(),
                },
                ResultField {
                    name: "nexthop".into(),
                },
                ResultField {
                    name: "prefix".into(),
                },
                ResultField {
                    name: "priority".into(),
                },
                ResultField {
                    name: "weight".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        static_route::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            colo_names: hashmap.remove("coloNames").unwrap(),
            colo_regions: hashmap.remove("coloRegions").unwrap(),
            description: hashmap.remove("description").unwrap(),
            nexthop: hashmap.remove("nexthop").unwrap(),
            prefix: hashmap.remove("prefix").unwrap(),
            priority: hashmap.remove("priority").unwrap(),
            weight: hashmap.remove("weight").unwrap(),
        }
    }
}
impl teams_account::Guest for Component {
    fn invoke(name: String, args: teams_account::Args) -> teams_account::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/teamsAccount:TeamsAccount".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "activityLogEnabled".into(),
                    value: args.activity_log_enabled,
                },
                ObjectField {
                    name: "antivirus".into(),
                    value: args.antivirus,
                },
                ObjectField {
                    name: "blockPage".into(),
                    value: args.block_page,
                },
                ObjectField {
                    name: "bodyScanning".into(),
                    value: args.body_scanning,
                },
                ObjectField {
                    name: "extendedEmailMatching".into(),
                    value: args.extended_email_matching,
                },
                ObjectField {
                    name: "fips".into(),
                    value: args.fips,
                },
                ObjectField {
                    name: "logging".into(),
                    value: args.logging,
                },
                ObjectField {
                    name: "nonIdentityBrowserIsolationEnabled".into(),
                    value: args.non_identity_browser_isolation_enabled,
                },
                ObjectField {
                    name: "payloadLog".into(),
                    value: args.payload_log,
                },
                ObjectField {
                    name: "protocolDetectionEnabled".into(),
                    value: args.protocol_detection_enabled,
                },
                ObjectField {
                    name: "proxy".into(),
                    value: args.proxy,
                },
                ObjectField {
                    name: "sshSessionLog".into(),
                    value: args.ssh_session_log,
                },
                ObjectField {
                    name: "tlsDecryptEnabled".into(),
                    value: args.tls_decrypt_enabled,
                },
                ObjectField {
                    name: "urlBrowserIsolationEnabled".into(),
                    value: args.url_browser_isolation_enabled,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "activityLogEnabled".into(),
                },
                ResultField {
                    name: "antivirus".into(),
                },
                ResultField {
                    name: "blockPage".into(),
                },
                ResultField {
                    name: "bodyScanning".into(),
                },
                ResultField {
                    name: "extendedEmailMatching".into(),
                },
                ResultField {
                    name: "fips".into(),
                },
                ResultField {
                    name: "logging".into(),
                },
                ResultField {
                    name: "nonIdentityBrowserIsolationEnabled".into(),
                },
                ResultField {
                    name: "payloadLog".into(),
                },
                ResultField {
                    name: "protocolDetectionEnabled".into(),
                },
                ResultField {
                    name: "proxy".into(),
                },
                ResultField {
                    name: "sshSessionLog".into(),
                },
                ResultField {
                    name: "tlsDecryptEnabled".into(),
                },
                ResultField {
                    name: "urlBrowserIsolationEnabled".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        teams_account::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            activity_log_enabled: hashmap.remove("activityLogEnabled").unwrap(),
            antivirus: hashmap.remove("antivirus").unwrap(),
            block_page: hashmap.remove("blockPage").unwrap(),
            body_scanning: hashmap.remove("bodyScanning").unwrap(),
            extended_email_matching: hashmap.remove("extendedEmailMatching").unwrap(),
            fips: hashmap.remove("fips").unwrap(),
            logging: hashmap.remove("logging").unwrap(),
            non_identity_browser_isolation_enabled: hashmap
                .remove("nonIdentityBrowserIsolationEnabled")
                .unwrap(),
            payload_log: hashmap.remove("payloadLog").unwrap(),
            protocol_detection_enabled: hashmap.remove("protocolDetectionEnabled").unwrap(),
            proxy: hashmap.remove("proxy").unwrap(),
            ssh_session_log: hashmap.remove("sshSessionLog").unwrap(),
            tls_decrypt_enabled: hashmap.remove("tlsDecryptEnabled").unwrap(),
            url_browser_isolation_enabled: hashmap.remove("urlBrowserIsolationEnabled").unwrap(),
        }
    }
}
impl teams_list::Guest for Component {
    fn invoke(name: String, args: teams_list::Args) -> teams_list::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/teamsList:TeamsList".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "description".into(),
                    value: args.description,
                },
                ObjectField {
                    name: "items".into(),
                    value: args.items,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "type".into(),
                    value: args.type_,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "description".into(),
                },
                ResultField {
                    name: "items".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "type".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        teams_list::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            description: hashmap.remove("description").unwrap(),
            items: hashmap.remove("items").unwrap(),
            name: hashmap.remove("name").unwrap(),
            type_: hashmap.remove("type").unwrap(),
        }
    }
}
impl teams_location::Guest for Component {
    fn invoke(name: String, args: teams_location::Args) -> teams_location::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/teamsLocation:TeamsLocation".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "clientDefault".into(),
                    value: args.client_default,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "networks".into(),
                    value: args.networks,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "anonymizedLogsEnabled".into(),
                },
                ResultField {
                    name: "clientDefault".into(),
                },
                ResultField {
                    name: "dohSubdomain".into(),
                },
                ResultField { name: "ip".into() },
                ResultField {
                    name: "ipv4Destination".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "networks".into(),
                },
                ResultField {
                    name: "policyIds".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        teams_location::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            anonymized_logs_enabled: hashmap.remove("anonymizedLogsEnabled").unwrap(),
            client_default: hashmap.remove("clientDefault").unwrap(),
            doh_subdomain: hashmap.remove("dohSubdomain").unwrap(),
            ip: hashmap.remove("ip").unwrap(),
            ipv4_destination: hashmap.remove("ipv4Destination").unwrap(),
            name: hashmap.remove("name").unwrap(),
            networks: hashmap.remove("networks").unwrap(),
            policy_ids: hashmap.remove("policyIds").unwrap(),
        }
    }
}
impl teams_proxy_endpoint::Guest for Component {
    fn invoke(name: String, args: teams_proxy_endpoint::Args) -> teams_proxy_endpoint::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/teamsProxyEndpoint:TeamsProxyEndpoint".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "ips".into(),
                    value: args.ips,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField { name: "ips".into() },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "subdomain".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        teams_proxy_endpoint::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            ips: hashmap.remove("ips").unwrap(),
            name: hashmap.remove("name").unwrap(),
            subdomain: hashmap.remove("subdomain").unwrap(),
        }
    }
}
impl teams_rule::Guest for Component {
    fn invoke(name: String, args: teams_rule::Args) -> teams_rule::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/teamsRule:TeamsRule".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "action".into(),
                    value: args.action,
                },
                ObjectField {
                    name: "description".into(),
                    value: args.description,
                },
                ObjectField {
                    name: "devicePosture".into(),
                    value: args.device_posture,
                },
                ObjectField {
                    name: "enabled".into(),
                    value: args.enabled,
                },
                ObjectField {
                    name: "filters".into(),
                    value: args.filters,
                },
                ObjectField {
                    name: "identity".into(),
                    value: args.identity,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "precedence".into(),
                    value: args.precedence,
                },
                ObjectField {
                    name: "ruleSettings".into(),
                    value: args.rule_settings,
                },
                ObjectField {
                    name: "traffic".into(),
                    value: args.traffic,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "action".into(),
                },
                ResultField {
                    name: "description".into(),
                },
                ResultField {
                    name: "devicePosture".into(),
                },
                ResultField {
                    name: "enabled".into(),
                },
                ResultField {
                    name: "filters".into(),
                },
                ResultField {
                    name: "identity".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "precedence".into(),
                },
                ResultField {
                    name: "ruleSettings".into(),
                },
                ResultField {
                    name: "traffic".into(),
                },
                ResultField {
                    name: "version".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        teams_rule::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            action: hashmap.remove("action").unwrap(),
            description: hashmap.remove("description").unwrap(),
            device_posture: hashmap.remove("devicePosture").unwrap(),
            enabled: hashmap.remove("enabled").unwrap(),
            filters: hashmap.remove("filters").unwrap(),
            identity: hashmap.remove("identity").unwrap(),
            name: hashmap.remove("name").unwrap(),
            precedence: hashmap.remove("precedence").unwrap(),
            rule_settings: hashmap.remove("ruleSettings").unwrap(),
            traffic: hashmap.remove("traffic").unwrap(),
            version: hashmap.remove("version").unwrap(),
        }
    }
}
impl tiered_cache::Guest for Component {
    fn invoke(name: String, args: tiered_cache::Args) -> tiered_cache::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/tieredCache:TieredCache".into(),
            name,
            object: vec![
                ObjectField {
                    name: "cacheType".into(),
                    value: args.cache_type,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "cacheType".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        tiered_cache::Res {
            cache_type: hashmap.remove("cacheType").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl total_tls::Guest for Component {
    fn invoke(name: String, args: total_tls::Args) -> total_tls::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/totalTls:TotalTls".into(),
            name,
            object: vec![
                ObjectField {
                    name: "certificateAuthority".into(),
                    value: args.certificate_authority,
                },
                ObjectField {
                    name: "enabled".into(),
                    value: args.enabled,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "certificateAuthority".into(),
                },
                ResultField {
                    name: "enabled".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        total_tls::Res {
            certificate_authority: hashmap.remove("certificateAuthority").unwrap(),
            enabled: hashmap.remove("enabled").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl tunnel::Guest for Component {
    fn invoke(name: String, args: tunnel::Args) -> tunnel::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/tunnel:Tunnel".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "configSrc".into(),
                    value: args.config_src,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "secret".into(),
                    value: args.secret,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "cname".into(),
                },
                ResultField {
                    name: "configSrc".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "secret".into(),
                },
                ResultField {
                    name: "tunnelToken".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        tunnel::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            cname: hashmap.remove("cname").unwrap(),
            config_src: hashmap.remove("configSrc").unwrap(),
            name: hashmap.remove("name").unwrap(),
            secret: hashmap.remove("secret").unwrap(),
            tunnel_token: hashmap.remove("tunnelToken").unwrap(),
        }
    }
}
impl tunnel_config::Guest for Component {
    fn invoke(name: String, args: tunnel_config::Args) -> tunnel_config::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/tunnelConfig:TunnelConfig".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "config".into(),
                    value: args.config,
                },
                ObjectField {
                    name: "tunnelId".into(),
                    value: args.tunnel_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "config".into(),
                },
                ResultField {
                    name: "tunnelId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        tunnel_config::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            config: hashmap.remove("config").unwrap(),
            tunnel_id: hashmap.remove("tunnelId").unwrap(),
        }
    }
}
impl tunnel_route::Guest for Component {
    fn invoke(name: String, args: tunnel_route::Args) -> tunnel_route::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/tunnelRoute:TunnelRoute".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "comment".into(),
                    value: args.comment,
                },
                ObjectField {
                    name: "network".into(),
                    value: args.network,
                },
                ObjectField {
                    name: "tunnelId".into(),
                    value: args.tunnel_id,
                },
                ObjectField {
                    name: "virtualNetworkId".into(),
                    value: args.virtual_network_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "comment".into(),
                },
                ResultField {
                    name: "network".into(),
                },
                ResultField {
                    name: "tunnelId".into(),
                },
                ResultField {
                    name: "virtualNetworkId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        tunnel_route::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            comment: hashmap.remove("comment").unwrap(),
            network: hashmap.remove("network").unwrap(),
            tunnel_id: hashmap.remove("tunnelId").unwrap(),
            virtual_network_id: hashmap.remove("virtualNetworkId").unwrap(),
        }
    }
}
impl tunnel_virtual_network::Guest for Component {
    fn invoke(name: String, args: tunnel_virtual_network::Args) -> tunnel_virtual_network::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/tunnelVirtualNetwork:TunnelVirtualNetwork".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "comment".into(),
                    value: args.comment,
                },
                ObjectField {
                    name: "isDefaultNetwork".into(),
                    value: args.is_default_network,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "comment".into(),
                },
                ResultField {
                    name: "isDefaultNetwork".into(),
                },
                ResultField {
                    name: "name".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        tunnel_virtual_network::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            comment: hashmap.remove("comment").unwrap(),
            is_default_network: hashmap.remove("isDefaultNetwork").unwrap(),
            name: hashmap.remove("name").unwrap(),
        }
    }
}
impl turnstile_widget::Guest for Component {
    fn invoke(name: String, args: turnstile_widget::Args) -> turnstile_widget::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/turnstileWidget:TurnstileWidget".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "botFightMode".into(),
                    value: args.bot_fight_mode,
                },
                ObjectField {
                    name: "domains".into(),
                    value: args.domains,
                },
                ObjectField {
                    name: "mode".into(),
                    value: args.mode,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "offlabel".into(),
                    value: args.offlabel,
                },
                ObjectField {
                    name: "region".into(),
                    value: args.region,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "botFightMode".into(),
                },
                ResultField {
                    name: "domains".into(),
                },
                ResultField {
                    name: "mode".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "offlabel".into(),
                },
                ResultField {
                    name: "region".into(),
                },
                ResultField {
                    name: "secret".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        turnstile_widget::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            bot_fight_mode: hashmap.remove("botFightMode").unwrap(),
            domains: hashmap.remove("domains").unwrap(),
            mode: hashmap.remove("mode").unwrap(),
            name: hashmap.remove("name").unwrap(),
            offlabel: hashmap.remove("offlabel").unwrap(),
            region: hashmap.remove("region").unwrap(),
            secret: hashmap.remove("secret").unwrap(),
        }
    }
}
impl url_normalization_settings::Guest for Component {
    fn invoke(
        name: String,
        args: url_normalization_settings::Args,
    ) -> url_normalization_settings::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/urlNormalizationSettings:UrlNormalizationSettings".into(),
            name,
            object: vec![
                ObjectField {
                    name: "scope".into(),
                    value: args.scope,
                },
                ObjectField {
                    name: "type".into(),
                    value: args.type_,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "scope".into(),
                },
                ResultField {
                    name: "type".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        url_normalization_settings::Res {
            scope: hashmap.remove("scope").unwrap(),
            type_: hashmap.remove("type").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl user_agent_blocking_rule::Guest for Component {
    fn invoke(name: String, args: user_agent_blocking_rule::Args) -> user_agent_blocking_rule::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/userAgentBlockingRule:UserAgentBlockingRule".into(),
            name,
            object: vec![
                ObjectField {
                    name: "configuration".into(),
                    value: args.configuration,
                },
                ObjectField {
                    name: "description".into(),
                    value: args.description,
                },
                ObjectField {
                    name: "mode".into(),
                    value: args.mode,
                },
                ObjectField {
                    name: "paused".into(),
                    value: args.paused,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "configuration".into(),
                },
                ResultField {
                    name: "description".into(),
                },
                ResultField {
                    name: "mode".into(),
                },
                ResultField {
                    name: "paused".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        user_agent_blocking_rule::Res {
            configuration: hashmap.remove("configuration").unwrap(),
            description: hashmap.remove("description").unwrap(),
            mode: hashmap.remove("mode").unwrap(),
            paused: hashmap.remove("paused").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl waiting_room::Guest for Component {
    fn invoke(name: String, args: waiting_room::Args) -> waiting_room::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/waitingRoom:WaitingRoom".into(),
            name,
            object: vec![
                ObjectField {
                    name: "additionalRoutes".into(),
                    value: args.additional_routes,
                },
                ObjectField {
                    name: "cookieSuffix".into(),
                    value: args.cookie_suffix,
                },
                ObjectField {
                    name: "customPageHtml".into(),
                    value: args.custom_page_html,
                },
                ObjectField {
                    name: "defaultTemplateLanguage".into(),
                    value: args.default_template_language,
                },
                ObjectField {
                    name: "description".into(),
                    value: args.description,
                },
                ObjectField {
                    name: "disableSessionRenewal".into(),
                    value: args.disable_session_renewal,
                },
                ObjectField {
                    name: "host".into(),
                    value: args.host,
                },
                ObjectField {
                    name: "jsonResponseEnabled".into(),
                    value: args.json_response_enabled,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "newUsersPerMinute".into(),
                    value: args.new_users_per_minute,
                },
                ObjectField {
                    name: "path".into(),
                    value: args.path,
                },
                ObjectField {
                    name: "queueAll".into(),
                    value: args.queue_all,
                },
                ObjectField {
                    name: "queueingMethod".into(),
                    value: args.queueing_method,
                },
                ObjectField {
                    name: "queueingStatusCode".into(),
                    value: args.queueing_status_code,
                },
                ObjectField {
                    name: "sessionDuration".into(),
                    value: args.session_duration,
                },
                ObjectField {
                    name: "suspended".into(),
                    value: args.suspended,
                },
                ObjectField {
                    name: "totalActiveUsers".into(),
                    value: args.total_active_users,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "additionalRoutes".into(),
                },
                ResultField {
                    name: "cookieSuffix".into(),
                },
                ResultField {
                    name: "customPageHtml".into(),
                },
                ResultField {
                    name: "defaultTemplateLanguage".into(),
                },
                ResultField {
                    name: "description".into(),
                },
                ResultField {
                    name: "disableSessionRenewal".into(),
                },
                ResultField {
                    name: "host".into(),
                },
                ResultField {
                    name: "jsonResponseEnabled".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "newUsersPerMinute".into(),
                },
                ResultField {
                    name: "path".into(),
                },
                ResultField {
                    name: "queueAll".into(),
                },
                ResultField {
                    name: "queueingMethod".into(),
                },
                ResultField {
                    name: "queueingStatusCode".into(),
                },
                ResultField {
                    name: "sessionDuration".into(),
                },
                ResultField {
                    name: "suspended".into(),
                },
                ResultField {
                    name: "totalActiveUsers".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        waiting_room::Res {
            additional_routes: hashmap.remove("additionalRoutes").unwrap(),
            cookie_suffix: hashmap.remove("cookieSuffix").unwrap(),
            custom_page_html: hashmap.remove("customPageHtml").unwrap(),
            default_template_language: hashmap.remove("defaultTemplateLanguage").unwrap(),
            description: hashmap.remove("description").unwrap(),
            disable_session_renewal: hashmap.remove("disableSessionRenewal").unwrap(),
            host: hashmap.remove("host").unwrap(),
            json_response_enabled: hashmap.remove("jsonResponseEnabled").unwrap(),
            name: hashmap.remove("name").unwrap(),
            new_users_per_minute: hashmap.remove("newUsersPerMinute").unwrap(),
            path: hashmap.remove("path").unwrap(),
            queue_all: hashmap.remove("queueAll").unwrap(),
            queueing_method: hashmap.remove("queueingMethod").unwrap(),
            queueing_status_code: hashmap.remove("queueingStatusCode").unwrap(),
            session_duration: hashmap.remove("sessionDuration").unwrap(),
            suspended: hashmap.remove("suspended").unwrap(),
            total_active_users: hashmap.remove("totalActiveUsers").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl waiting_room_event::Guest for Component {
    fn invoke(name: String, args: waiting_room_event::Args) -> waiting_room_event::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/waitingRoomEvent:WaitingRoomEvent".into(),
            name,
            object: vec![
                ObjectField {
                    name: "customPageHtml".into(),
                    value: args.custom_page_html,
                },
                ObjectField {
                    name: "description".into(),
                    value: args.description,
                },
                ObjectField {
                    name: "disableSessionRenewal".into(),
                    value: args.disable_session_renewal,
                },
                ObjectField {
                    name: "eventEndTime".into(),
                    value: args.event_end_time,
                },
                ObjectField {
                    name: "eventStartTime".into(),
                    value: args.event_start_time,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "newUsersPerMinute".into(),
                    value: args.new_users_per_minute,
                },
                ObjectField {
                    name: "prequeueStartTime".into(),
                    value: args.prequeue_start_time,
                },
                ObjectField {
                    name: "queueingMethod".into(),
                    value: args.queueing_method,
                },
                ObjectField {
                    name: "sessionDuration".into(),
                    value: args.session_duration,
                },
                ObjectField {
                    name: "shuffleAtEventStart".into(),
                    value: args.shuffle_at_event_start,
                },
                ObjectField {
                    name: "suspended".into(),
                    value: args.suspended,
                },
                ObjectField {
                    name: "totalActiveUsers".into(),
                    value: args.total_active_users,
                },
                ObjectField {
                    name: "waitingRoomId".into(),
                    value: args.waiting_room_id,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "createdOn".into(),
                },
                ResultField {
                    name: "customPageHtml".into(),
                },
                ResultField {
                    name: "description".into(),
                },
                ResultField {
                    name: "disableSessionRenewal".into(),
                },
                ResultField {
                    name: "eventEndTime".into(),
                },
                ResultField {
                    name: "eventStartTime".into(),
                },
                ResultField {
                    name: "modifiedOn".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "newUsersPerMinute".into(),
                },
                ResultField {
                    name: "prequeueStartTime".into(),
                },
                ResultField {
                    name: "queueingMethod".into(),
                },
                ResultField {
                    name: "sessionDuration".into(),
                },
                ResultField {
                    name: "shuffleAtEventStart".into(),
                },
                ResultField {
                    name: "suspended".into(),
                },
                ResultField {
                    name: "totalActiveUsers".into(),
                },
                ResultField {
                    name: "waitingRoomId".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        waiting_room_event::Res {
            created_on: hashmap.remove("createdOn").unwrap(),
            custom_page_html: hashmap.remove("customPageHtml").unwrap(),
            description: hashmap.remove("description").unwrap(),
            disable_session_renewal: hashmap.remove("disableSessionRenewal").unwrap(),
            event_end_time: hashmap.remove("eventEndTime").unwrap(),
            event_start_time: hashmap.remove("eventStartTime").unwrap(),
            modified_on: hashmap.remove("modifiedOn").unwrap(),
            name: hashmap.remove("name").unwrap(),
            new_users_per_minute: hashmap.remove("newUsersPerMinute").unwrap(),
            prequeue_start_time: hashmap.remove("prequeueStartTime").unwrap(),
            queueing_method: hashmap.remove("queueingMethod").unwrap(),
            session_duration: hashmap.remove("sessionDuration").unwrap(),
            shuffle_at_event_start: hashmap.remove("shuffleAtEventStart").unwrap(),
            suspended: hashmap.remove("suspended").unwrap(),
            total_active_users: hashmap.remove("totalActiveUsers").unwrap(),
            waiting_room_id: hashmap.remove("waitingRoomId").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl waiting_room_rules::Guest for Component {
    fn invoke(name: String, args: waiting_room_rules::Args) -> waiting_room_rules::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/waitingRoomRules:WaitingRoomRules".into(),
            name,
            object: vec![
                ObjectField {
                    name: "rules".into(),
                    value: args.rules,
                },
                ObjectField {
                    name: "waitingRoomId".into(),
                    value: args.waiting_room_id,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "rules".into(),
                },
                ResultField {
                    name: "waitingRoomId".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        waiting_room_rules::Res {
            rules: hashmap.remove("rules").unwrap(),
            waiting_room_id: hashmap.remove("waitingRoomId").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl waiting_room_settings::Guest for Component {
    fn invoke(name: String, args: waiting_room_settings::Args) -> waiting_room_settings::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/waitingRoomSettings:WaitingRoomSettings".into(),
            name,
            object: vec![
                ObjectField {
                    name: "searchEngineCrawlerBypass".into(),
                    value: args.search_engine_crawler_bypass,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "searchEngineCrawlerBypass".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        waiting_room_settings::Res {
            search_engine_crawler_bypass: hashmap.remove("searchEngineCrawlerBypass").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl web3_hostname::Guest for Component {
    fn invoke(name: String, args: web3_hostname::Args) -> web3_hostname::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/web3Hostname:Web3Hostname".into(),
            name,
            object: vec![
                ObjectField {
                    name: "description".into(),
                    value: args.description,
                },
                ObjectField {
                    name: "dnslink".into(),
                    value: args.dnslink,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "target".into(),
                    value: args.target,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "createdOn".into(),
                },
                ResultField {
                    name: "description".into(),
                },
                ResultField {
                    name: "dnslink".into(),
                },
                ResultField {
                    name: "modifiedOn".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "status".into(),
                },
                ResultField {
                    name: "target".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        web3_hostname::Res {
            created_on: hashmap.remove("createdOn").unwrap(),
            description: hashmap.remove("description").unwrap(),
            dnslink: hashmap.remove("dnslink").unwrap(),
            modified_on: hashmap.remove("modifiedOn").unwrap(),
            name: hashmap.remove("name").unwrap(),
            status: hashmap.remove("status").unwrap(),
            target: hashmap.remove("target").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl web_analytics_rule::Guest for Component {
    fn invoke(name: String, args: web_analytics_rule::Args) -> web_analytics_rule::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/webAnalyticsRule:WebAnalyticsRule".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "host".into(),
                    value: args.host,
                },
                ObjectField {
                    name: "inclusive".into(),
                    value: args.inclusive,
                },
                ObjectField {
                    name: "isPaused".into(),
                    value: args.is_paused,
                },
                ObjectField {
                    name: "paths".into(),
                    value: args.paths,
                },
                ObjectField {
                    name: "rulesetId".into(),
                    value: args.ruleset_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "host".into(),
                },
                ResultField {
                    name: "inclusive".into(),
                },
                ResultField {
                    name: "isPaused".into(),
                },
                ResultField {
                    name: "paths".into(),
                },
                ResultField {
                    name: "rulesetId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        web_analytics_rule::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            host: hashmap.remove("host").unwrap(),
            inclusive: hashmap.remove("inclusive").unwrap(),
            is_paused: hashmap.remove("isPaused").unwrap(),
            paths: hashmap.remove("paths").unwrap(),
            ruleset_id: hashmap.remove("rulesetId").unwrap(),
        }
    }
}
impl web_analytics_site::Guest for Component {
    fn invoke(name: String, args: web_analytics_site::Args) -> web_analytics_site::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/webAnalyticsSite:WebAnalyticsSite".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "autoInstall".into(),
                    value: args.auto_install,
                },
                ObjectField {
                    name: "host".into(),
                    value: args.host,
                },
                ObjectField {
                    name: "zoneTag".into(),
                    value: args.zone_tag,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "autoInstall".into(),
                },
                ResultField {
                    name: "host".into(),
                },
                ResultField {
                    name: "rulesetId".into(),
                },
                ResultField {
                    name: "siteTag".into(),
                },
                ResultField {
                    name: "siteToken".into(),
                },
                ResultField {
                    name: "snippet".into(),
                },
                ResultField {
                    name: "zoneTag".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        web_analytics_site::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            auto_install: hashmap.remove("autoInstall").unwrap(),
            host: hashmap.remove("host").unwrap(),
            ruleset_id: hashmap.remove("rulesetId").unwrap(),
            site_tag: hashmap.remove("siteTag").unwrap(),
            site_token: hashmap.remove("siteToken").unwrap(),
            snippet: hashmap.remove("snippet").unwrap(),
            zone_tag: hashmap.remove("zoneTag").unwrap(),
        }
    }
}
impl worker_cron_trigger::Guest for Component {
    fn invoke(name: String, args: worker_cron_trigger::Args) -> worker_cron_trigger::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/workerCronTrigger:WorkerCronTrigger".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "schedules".into(),
                    value: args.schedules,
                },
                ObjectField {
                    name: "scriptName".into(),
                    value: args.script_name,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "schedules".into(),
                },
                ResultField {
                    name: "scriptName".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        worker_cron_trigger::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            schedules: hashmap.remove("schedules").unwrap(),
            script_name: hashmap.remove("scriptName").unwrap(),
        }
    }
}
impl worker_domain::Guest for Component {
    fn invoke(name: String, args: worker_domain::Args) -> worker_domain::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/workerDomain:WorkerDomain".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "environment".into(),
                    value: args.environment,
                },
                ObjectField {
                    name: "hostname".into(),
                    value: args.hostname,
                },
                ObjectField {
                    name: "service".into(),
                    value: args.service,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "environment".into(),
                },
                ResultField {
                    name: "hostname".into(),
                },
                ResultField {
                    name: "service".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        worker_domain::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            environment: hashmap.remove("environment").unwrap(),
            hostname: hashmap.remove("hostname").unwrap(),
            service: hashmap.remove("service").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl worker_route::Guest for Component {
    fn invoke(name: String, args: worker_route::Args) -> worker_route::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/workerRoute:WorkerRoute".into(),
            name,
            object: vec![
                ObjectField {
                    name: "pattern".into(),
                    value: args.pattern,
                },
                ObjectField {
                    name: "scriptName".into(),
                    value: args.script_name,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "pattern".into(),
                },
                ResultField {
                    name: "scriptName".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        worker_route::Res {
            pattern: hashmap.remove("pattern").unwrap(),
            script_name: hashmap.remove("scriptName").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl worker_script::Guest for Component {
    fn invoke(name: String, args: worker_script::Args) -> worker_script::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/workerScript:WorkerScript".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "analyticsEngineBindings".into(),
                    value: args.analytics_engine_bindings,
                },
                ObjectField {
                    name: "compatibilityDate".into(),
                    value: args.compatibility_date,
                },
                ObjectField {
                    name: "compatibilityFlags".into(),
                    value: args.compatibility_flags,
                },
                ObjectField {
                    name: "content".into(),
                    value: args.content,
                },
                ObjectField {
                    name: "d1DatabaseBindings".into(),
                    value: args.d1_database_bindings,
                },
                ObjectField {
                    name: "dispatchNamespace".into(),
                    value: args.dispatch_namespace,
                },
                ObjectField {
                    name: "kvNamespaceBindings".into(),
                    value: args.kv_namespace_bindings,
                },
                ObjectField {
                    name: "logpush".into(),
                    value: args.logpush,
                },
                ObjectField {
                    name: "module".into(),
                    value: args.module,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "placements".into(),
                    value: args.placements,
                },
                ObjectField {
                    name: "plainTextBindings".into(),
                    value: args.plain_text_bindings,
                },
                ObjectField {
                    name: "queueBindings".into(),
                    value: args.queue_bindings,
                },
                ObjectField {
                    name: "r2BucketBindings".into(),
                    value: args.r2_bucket_bindings,
                },
                ObjectField {
                    name: "secretTextBindings".into(),
                    value: args.secret_text_bindings,
                },
                ObjectField {
                    name: "serviceBindings".into(),
                    value: args.service_bindings,
                },
                ObjectField {
                    name: "tags".into(),
                    value: args.tags,
                },
                ObjectField {
                    name: "webassemblyBindings".into(),
                    value: args.webassembly_bindings,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "analyticsEngineBindings".into(),
                },
                ResultField {
                    name: "compatibilityDate".into(),
                },
                ResultField {
                    name: "compatibilityFlags".into(),
                },
                ResultField {
                    name: "content".into(),
                },
                ResultField {
                    name: "d1DatabaseBindings".into(),
                },
                ResultField {
                    name: "dispatchNamespace".into(),
                },
                ResultField {
                    name: "kvNamespaceBindings".into(),
                },
                ResultField {
                    name: "logpush".into(),
                },
                ResultField {
                    name: "module".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "placements".into(),
                },
                ResultField {
                    name: "plainTextBindings".into(),
                },
                ResultField {
                    name: "queueBindings".into(),
                },
                ResultField {
                    name: "r2BucketBindings".into(),
                },
                ResultField {
                    name: "secretTextBindings".into(),
                },
                ResultField {
                    name: "serviceBindings".into(),
                },
                ResultField {
                    name: "tags".into(),
                },
                ResultField {
                    name: "webassemblyBindings".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        worker_script::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            analytics_engine_bindings: hashmap.remove("analyticsEngineBindings").unwrap(),
            compatibility_date: hashmap.remove("compatibilityDate").unwrap(),
            compatibility_flags: hashmap.remove("compatibilityFlags").unwrap(),
            content: hashmap.remove("content").unwrap(),
            d1_database_bindings: hashmap.remove("d1DatabaseBindings").unwrap(),
            dispatch_namespace: hashmap.remove("dispatchNamespace").unwrap(),
            kv_namespace_bindings: hashmap.remove("kvNamespaceBindings").unwrap(),
            logpush: hashmap.remove("logpush").unwrap(),
            module: hashmap.remove("module").unwrap(),
            name: hashmap.remove("name").unwrap(),
            placements: hashmap.remove("placements").unwrap(),
            plain_text_bindings: hashmap.remove("plainTextBindings").unwrap(),
            queue_bindings: hashmap.remove("queueBindings").unwrap(),
            r2_bucket_bindings: hashmap.remove("r2BucketBindings").unwrap(),
            secret_text_bindings: hashmap.remove("secretTextBindings").unwrap(),
            service_bindings: hashmap.remove("serviceBindings").unwrap(),
            tags: hashmap.remove("tags").unwrap(),
            webassembly_bindings: hashmap.remove("webassemblyBindings").unwrap(),
        }
    }
}
impl worker_secret::Guest for Component {
    fn invoke(name: String, args: worker_secret::Args) -> worker_secret::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/workerSecret:WorkerSecret".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "scriptName".into(),
                    value: args.script_name,
                },
                ObjectField {
                    name: "secretText".into(),
                    value: args.secret_text,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "scriptName".into(),
                },
                ResultField {
                    name: "secretText".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        worker_secret::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            name: hashmap.remove("name").unwrap(),
            script_name: hashmap.remove("scriptName").unwrap(),
            secret_text: hashmap.remove("secretText").unwrap(),
        }
    }
}
impl workers_for_platforms_namespace::Guest for Component {
    fn invoke(
        name: String,
        args: workers_for_platforms_namespace::Args,
    ) -> workers_for_platforms_namespace::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/workersForPlatformsNamespace:WorkersForPlatformsNamespace"
                .into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "name".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        workers_for_platforms_namespace::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            name: hashmap.remove("name").unwrap(),
        }
    }
}
impl workers_kv::Guest for Component {
    fn invoke(name: String, args: workers_kv::Args) -> workers_kv::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/workersKv:WorkersKv".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "key".into(),
                    value: args.key,
                },
                ObjectField {
                    name: "namespaceId".into(),
                    value: args.namespace_id,
                },
                ObjectField {
                    name: "value".into(),
                    value: args.value,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField { name: "key".into() },
                ResultField {
                    name: "namespaceId".into(),
                },
                ResultField {
                    name: "value".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        workers_kv::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            key: hashmap.remove("key").unwrap(),
            namespace_id: hashmap.remove("namespaceId").unwrap(),
            value: hashmap.remove("value").unwrap(),
        }
    }
}
impl workers_kv_namespace::Guest for Component {
    fn invoke(name: String, args: workers_kv_namespace::Args) -> workers_kv_namespace::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/workersKvNamespace:WorkersKvNamespace".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "title".into(),
                    value: args.title,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "title".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        workers_kv_namespace::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            title: hashmap.remove("title").unwrap(),
        }
    }
}
impl zone::Guest for Component {
    fn invoke(name: String, args: zone::Args) -> zone::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/zone:Zone".into(),
            name,
            object: vec![
                ObjectField {
                    name: "accountId".into(),
                    value: args.account_id,
                },
                ObjectField {
                    name: "jumpStart".into(),
                    value: args.jump_start,
                },
                ObjectField {
                    name: "paused".into(),
                    value: args.paused,
                },
                ObjectField {
                    name: "plan".into(),
                    value: args.plan,
                },
                ObjectField {
                    name: "type".into(),
                    value: args.type_,
                },
                ObjectField {
                    name: "zone".into(),
                    value: args.zone,
                },
            ],
            results: vec![
                ResultField {
                    name: "accountId".into(),
                },
                ResultField {
                    name: "jumpStart".into(),
                },
                ResultField {
                    name: "meta".into(),
                },
                ResultField {
                    name: "nameServers".into(),
                },
                ResultField {
                    name: "paused".into(),
                },
                ResultField {
                    name: "plan".into(),
                },
                ResultField {
                    name: "status".into(),
                },
                ResultField {
                    name: "type".into(),
                },
                ResultField {
                    name: "vanityNameServers".into(),
                },
                ResultField {
                    name: "verificationKey".into(),
                },
                ResultField {
                    name: "zone".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        zone::Res {
            account_id: hashmap.remove("accountId").unwrap(),
            jump_start: hashmap.remove("jumpStart").unwrap(),
            meta: hashmap.remove("meta").unwrap(),
            name_servers: hashmap.remove("nameServers").unwrap(),
            paused: hashmap.remove("paused").unwrap(),
            plan: hashmap.remove("plan").unwrap(),
            status: hashmap.remove("status").unwrap(),
            type_: hashmap.remove("type").unwrap(),
            vanity_name_servers: hashmap.remove("vanityNameServers").unwrap(),
            verification_key: hashmap.remove("verificationKey").unwrap(),
            zone: hashmap.remove("zone").unwrap(),
        }
    }
}
impl zone_cache_reserve::Guest for Component {
    fn invoke(name: String, args: zone_cache_reserve::Args) -> zone_cache_reserve::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/zoneCacheReserve:ZoneCacheReserve".into(),
            name,
            object: vec![
                ObjectField {
                    name: "enabled".into(),
                    value: args.enabled,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "enabled".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        zone_cache_reserve::Res {
            enabled: hashmap.remove("enabled").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl zone_cache_variants::Guest for Component {
    fn invoke(name: String, args: zone_cache_variants::Args) -> zone_cache_variants::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/zoneCacheVariants:ZoneCacheVariants".into(),
            name,
            object: vec![
                ObjectField {
                    name: "avifs".into(),
                    value: args.avifs,
                },
                ObjectField {
                    name: "bmps".into(),
                    value: args.bmps,
                },
                ObjectField {
                    name: "gifs".into(),
                    value: args.gifs,
                },
                ObjectField {
                    name: "jp2s".into(),
                    value: args.jp2s,
                },
                ObjectField {
                    name: "jpegs".into(),
                    value: args.jpegs,
                },
                ObjectField {
                    name: "jpg2s".into(),
                    value: args.jpg2s,
                },
                ObjectField {
                    name: "jpgs".into(),
                    value: args.jpgs,
                },
                ObjectField {
                    name: "pngs".into(),
                    value: args.pngs,
                },
                ObjectField {
                    name: "tiffs".into(),
                    value: args.tiffs,
                },
                ObjectField {
                    name: "tifs".into(),
                    value: args.tifs,
                },
                ObjectField {
                    name: "webps".into(),
                    value: args.webps,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "avifs".into(),
                },
                ResultField {
                    name: "bmps".into(),
                },
                ResultField {
                    name: "gifs".into(),
                },
                ResultField {
                    name: "jp2s".into(),
                },
                ResultField {
                    name: "jpegs".into(),
                },
                ResultField {
                    name: "jpg2s".into(),
                },
                ResultField {
                    name: "jpgs".into(),
                },
                ResultField {
                    name: "pngs".into(),
                },
                ResultField {
                    name: "tiffs".into(),
                },
                ResultField {
                    name: "tifs".into(),
                },
                ResultField {
                    name: "webps".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        zone_cache_variants::Res {
            avifs: hashmap.remove("avifs").unwrap(),
            bmps: hashmap.remove("bmps").unwrap(),
            gifs: hashmap.remove("gifs").unwrap(),
            jp2s: hashmap.remove("jp2s").unwrap(),
            jpegs: hashmap.remove("jpegs").unwrap(),
            jpg2s: hashmap.remove("jpg2s").unwrap(),
            jpgs: hashmap.remove("jpgs").unwrap(),
            pngs: hashmap.remove("pngs").unwrap(),
            tiffs: hashmap.remove("tiffs").unwrap(),
            tifs: hashmap.remove("tifs").unwrap(),
            webps: hashmap.remove("webps").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl zone_dnssec::Guest for Component {
    fn invoke(name: String, args: zone_dnssec::Args) -> zone_dnssec::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/zoneDnssec:ZoneDnssec".into(),
            name,
            object: vec![
                ObjectField {
                    name: "modifiedOn".into(),
                    value: args.modified_on,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "algorithm".into(),
                },
                ResultField {
                    name: "digest".into(),
                },
                ResultField {
                    name: "digestAlgorithm".into(),
                },
                ResultField {
                    name: "digestType".into(),
                },
                ResultField { name: "ds".into() },
                ResultField {
                    name: "flags".into(),
                },
                ResultField {
                    name: "keyTag".into(),
                },
                ResultField {
                    name: "keyType".into(),
                },
                ResultField {
                    name: "modifiedOn".into(),
                },
                ResultField {
                    name: "publicKey".into(),
                },
                ResultField {
                    name: "status".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        zone_dnssec::Res {
            algorithm: hashmap.remove("algorithm").unwrap(),
            digest: hashmap.remove("digest").unwrap(),
            digest_algorithm: hashmap.remove("digestAlgorithm").unwrap(),
            digest_type: hashmap.remove("digestType").unwrap(),
            ds: hashmap.remove("ds").unwrap(),
            flags: hashmap.remove("flags").unwrap(),
            key_tag: hashmap.remove("keyTag").unwrap(),
            key_type: hashmap.remove("keyType").unwrap(),
            modified_on: hashmap.remove("modifiedOn").unwrap(),
            public_key: hashmap.remove("publicKey").unwrap(),
            status: hashmap.remove("status").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl zone_hold::Guest for Component {
    fn invoke(name: String, args: zone_hold::Args) -> zone_hold::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/zoneHold:ZoneHold".into(),
            name,
            object: vec![
                ObjectField {
                    name: "hold".into(),
                    value: args.hold,
                },
                ObjectField {
                    name: "holdAfter".into(),
                    value: args.hold_after,
                },
                ObjectField {
                    name: "includeSubdomains".into(),
                    value: args.include_subdomains,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "hold".into(),
                },
                ResultField {
                    name: "holdAfter".into(),
                },
                ResultField {
                    name: "includeSubdomains".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        zone_hold::Res {
            hold: hashmap.remove("hold").unwrap(),
            hold_after: hashmap.remove("holdAfter").unwrap(),
            include_subdomains: hashmap.remove("includeSubdomains").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl zone_lockdown::Guest for Component {
    fn invoke(name: String, args: zone_lockdown::Args) -> zone_lockdown::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/zoneLockdown:ZoneLockdown".into(),
            name,
            object: vec![
                ObjectField {
                    name: "configurations".into(),
                    value: args.configurations,
                },
                ObjectField {
                    name: "description".into(),
                    value: args.description,
                },
                ObjectField {
                    name: "paused".into(),
                    value: args.paused,
                },
                ObjectField {
                    name: "priority".into(),
                    value: args.priority,
                },
                ObjectField {
                    name: "urls".into(),
                    value: args.urls,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "configurations".into(),
                },
                ResultField {
                    name: "description".into(),
                },
                ResultField {
                    name: "paused".into(),
                },
                ResultField {
                    name: "priority".into(),
                },
                ResultField {
                    name: "urls".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        zone_lockdown::Res {
            configurations: hashmap.remove("configurations").unwrap(),
            description: hashmap.remove("description").unwrap(),
            paused: hashmap.remove("paused").unwrap(),
            priority: hashmap.remove("priority").unwrap(),
            urls: hashmap.remove("urls").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
        }
    }
}
impl zone_settings_override::Guest for Component {
    fn invoke(name: String, args: zone_settings_override::Args) -> zone_settings_override::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/zoneSettingsOverride:ZoneSettingsOverride".into(),
            name,
            object: vec![
                ObjectField {
                    name: "settings".into(),
                    value: args.settings,
                },
                ObjectField {
                    name: "zoneId".into(),
                    value: args.zone_id,
                },
            ],
            results: vec![
                ResultField {
                    name: "initialSettings".into(),
                },
                ResultField {
                    name: "initialSettingsReadAt".into(),
                },
                ResultField {
                    name: "readonlySettings".into(),
                },
                ResultField {
                    name: "settings".into(),
                },
                ResultField {
                    name: "zoneId".into(),
                },
                ResultField {
                    name: "zoneStatus".into(),
                },
                ResultField {
                    name: "zoneType".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        zone_settings_override::Res {
            initial_settings: hashmap.remove("initialSettings").unwrap(),
            initial_settings_read_at: hashmap.remove("initialSettingsReadAt").unwrap(),
            readonly_settings: hashmap.remove("readonlySettings").unwrap(),
            settings: hashmap.remove("settings").unwrap(),
            zone_id: hashmap.remove("zoneId").unwrap(),
            zone_status: hashmap.remove("zoneStatus").unwrap(),
            zone_type: hashmap.remove("zoneType").unwrap(),
        }
    }
}
