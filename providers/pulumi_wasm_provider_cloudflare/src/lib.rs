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

        access_application::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            allow_authenticate_via_warp: o
                .fields
                .iter()
                .find(|o| o.name == "allowAuthenticateViaWarp")
                .unwrap()
                .output
                .duplicate(),
            allowed_idps: o
                .fields
                .iter()
                .find(|o| o.name == "allowedIdps")
                .unwrap()
                .output
                .duplicate(),
            app_launcher_logo_url: o
                .fields
                .iter()
                .find(|o| o.name == "appLauncherLogoUrl")
                .unwrap()
                .output
                .duplicate(),
            app_launcher_visible: o
                .fields
                .iter()
                .find(|o| o.name == "appLauncherVisible")
                .unwrap()
                .output
                .duplicate(),
            aud: o
                .fields
                .iter()
                .find(|o| o.name == "aud")
                .unwrap()
                .output
                .duplicate(),
            auto_redirect_to_identity: o
                .fields
                .iter()
                .find(|o| o.name == "autoRedirectToIdentity")
                .unwrap()
                .output
                .duplicate(),
            bg_color: o
                .fields
                .iter()
                .find(|o| o.name == "bgColor")
                .unwrap()
                .output
                .duplicate(),
            cors_headers: o
                .fields
                .iter()
                .find(|o| o.name == "corsHeaders")
                .unwrap()
                .output
                .duplicate(),
            custom_deny_message: o
                .fields
                .iter()
                .find(|o| o.name == "customDenyMessage")
                .unwrap()
                .output
                .duplicate(),
            custom_deny_url: o
                .fields
                .iter()
                .find(|o| o.name == "customDenyUrl")
                .unwrap()
                .output
                .duplicate(),
            custom_non_identity_deny_url: o
                .fields
                .iter()
                .find(|o| o.name == "customNonIdentityDenyUrl")
                .unwrap()
                .output
                .duplicate(),
            custom_pages: o
                .fields
                .iter()
                .find(|o| o.name == "customPages")
                .unwrap()
                .output
                .duplicate(),
            domain: o
                .fields
                .iter()
                .find(|o| o.name == "domain")
                .unwrap()
                .output
                .duplicate(),
            enable_binding_cookie: o
                .fields
                .iter()
                .find(|o| o.name == "enableBindingCookie")
                .unwrap()
                .output
                .duplicate(),
            footer_links: o
                .fields
                .iter()
                .find(|o| o.name == "footerLinks")
                .unwrap()
                .output
                .duplicate(),
            header_bg_color: o
                .fields
                .iter()
                .find(|o| o.name == "headerBgColor")
                .unwrap()
                .output
                .duplicate(),
            http_only_cookie_attribute: o
                .fields
                .iter()
                .find(|o| o.name == "httpOnlyCookieAttribute")
                .unwrap()
                .output
                .duplicate(),
            landing_page_design: o
                .fields
                .iter()
                .find(|o| o.name == "landingPageDesign")
                .unwrap()
                .output
                .duplicate(),
            logo_url: o
                .fields
                .iter()
                .find(|o| o.name == "logoUrl")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            saas_app: o
                .fields
                .iter()
                .find(|o| o.name == "saasApp")
                .unwrap()
                .output
                .duplicate(),
            same_site_cookie_attribute: o
                .fields
                .iter()
                .find(|o| o.name == "sameSiteCookieAttribute")
                .unwrap()
                .output
                .duplicate(),
            self_hosted_domains: o
                .fields
                .iter()
                .find(|o| o.name == "selfHostedDomains")
                .unwrap()
                .output
                .duplicate(),
            service_auth401_redirect: o
                .fields
                .iter()
                .find(|o| o.name == "serviceAuth401Redirect")
                .unwrap()
                .output
                .duplicate(),
            session_duration: o
                .fields
                .iter()
                .find(|o| o.name == "sessionDuration")
                .unwrap()
                .output
                .duplicate(),
            skip_interstitial: o
                .fields
                .iter()
                .find(|o| o.name == "skipInterstitial")
                .unwrap()
                .output
                .duplicate(),
            tags: o
                .fields
                .iter()
                .find(|o| o.name == "tags")
                .unwrap()
                .output
                .duplicate(),
            type_: o
                .fields
                .iter()
                .find(|o| o.name == "type")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        access_ca_certificate::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            application_id: o
                .fields
                .iter()
                .find(|o| o.name == "applicationId")
                .unwrap()
                .output
                .duplicate(),
            aud: o
                .fields
                .iter()
                .find(|o| o.name == "aud")
                .unwrap()
                .output
                .duplicate(),
            public_key: o
                .fields
                .iter()
                .find(|o| o.name == "publicKey")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        access_custom_page::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            app_count: o
                .fields
                .iter()
                .find(|o| o.name == "appCount")
                .unwrap()
                .output
                .duplicate(),
            custom_html: o
                .fields
                .iter()
                .find(|o| o.name == "customHtml")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            type_: o
                .fields
                .iter()
                .find(|o| o.name == "type")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        access_group::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            excludes: o
                .fields
                .iter()
                .find(|o| o.name == "excludes")
                .unwrap()
                .output
                .duplicate(),
            includes: o
                .fields
                .iter()
                .find(|o| o.name == "includes")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            requires: o
                .fields
                .iter()
                .find(|o| o.name == "requires")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        access_identity_provider::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            configs: o
                .fields
                .iter()
                .find(|o| o.name == "configs")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            scim_configs: o
                .fields
                .iter()
                .find(|o| o.name == "scimConfigs")
                .unwrap()
                .output
                .duplicate(),
            type_: o
                .fields
                .iter()
                .find(|o| o.name == "type")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        access_keys_configuration::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            key_rotation_interval_days: o
                .fields
                .iter()
                .find(|o| o.name == "keyRotationIntervalDays")
                .unwrap()
                .output
                .duplicate(),
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

        access_mutual_tls_certificate::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            associated_hostnames: o
                .fields
                .iter()
                .find(|o| o.name == "associatedHostnames")
                .unwrap()
                .output
                .duplicate(),
            certificate: o
                .fields
                .iter()
                .find(|o| o.name == "certificate")
                .unwrap()
                .output
                .duplicate(),
            fingerprint: o
                .fields
                .iter()
                .find(|o| o.name == "fingerprint")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        access_mutual_tls_hostname_settings::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            settings: o
                .fields
                .iter()
                .find(|o| o.name == "settings")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        access_organization::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            allow_authenticate_via_warp: o
                .fields
                .iter()
                .find(|o| o.name == "allowAuthenticateViaWarp")
                .unwrap()
                .output
                .duplicate(),
            auth_domain: o
                .fields
                .iter()
                .find(|o| o.name == "authDomain")
                .unwrap()
                .output
                .duplicate(),
            auto_redirect_to_identity: o
                .fields
                .iter()
                .find(|o| o.name == "autoRedirectToIdentity")
                .unwrap()
                .output
                .duplicate(),
            custom_pages: o
                .fields
                .iter()
                .find(|o| o.name == "customPages")
                .unwrap()
                .output
                .duplicate(),
            is_ui_read_only: o
                .fields
                .iter()
                .find(|o| o.name == "isUiReadOnly")
                .unwrap()
                .output
                .duplicate(),
            login_designs: o
                .fields
                .iter()
                .find(|o| o.name == "loginDesigns")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            session_duration: o
                .fields
                .iter()
                .find(|o| o.name == "sessionDuration")
                .unwrap()
                .output
                .duplicate(),
            ui_read_only_toggle_reason: o
                .fields
                .iter()
                .find(|o| o.name == "uiReadOnlyToggleReason")
                .unwrap()
                .output
                .duplicate(),
            user_seat_expiration_inactive_time: o
                .fields
                .iter()
                .find(|o| o.name == "userSeatExpirationInactiveTime")
                .unwrap()
                .output
                .duplicate(),
            warp_auth_session_duration: o
                .fields
                .iter()
                .find(|o| o.name == "warpAuthSessionDuration")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        access_policy::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            application_id: o
                .fields
                .iter()
                .find(|o| o.name == "applicationId")
                .unwrap()
                .output
                .duplicate(),
            approval_groups: o
                .fields
                .iter()
                .find(|o| o.name == "approvalGroups")
                .unwrap()
                .output
                .duplicate(),
            approval_required: o
                .fields
                .iter()
                .find(|o| o.name == "approvalRequired")
                .unwrap()
                .output
                .duplicate(),
            decision: o
                .fields
                .iter()
                .find(|o| o.name == "decision")
                .unwrap()
                .output
                .duplicate(),
            excludes: o
                .fields
                .iter()
                .find(|o| o.name == "excludes")
                .unwrap()
                .output
                .duplicate(),
            includes: o
                .fields
                .iter()
                .find(|o| o.name == "includes")
                .unwrap()
                .output
                .duplicate(),
            isolation_required: o
                .fields
                .iter()
                .find(|o| o.name == "isolationRequired")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            precedence: o
                .fields
                .iter()
                .find(|o| o.name == "precedence")
                .unwrap()
                .output
                .duplicate(),
            purpose_justification_prompt: o
                .fields
                .iter()
                .find(|o| o.name == "purposeJustificationPrompt")
                .unwrap()
                .output
                .duplicate(),
            purpose_justification_required: o
                .fields
                .iter()
                .find(|o| o.name == "purposeJustificationRequired")
                .unwrap()
                .output
                .duplicate(),
            requires: o
                .fields
                .iter()
                .find(|o| o.name == "requires")
                .unwrap()
                .output
                .duplicate(),
            session_duration: o
                .fields
                .iter()
                .find(|o| o.name == "sessionDuration")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        access_rule::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            configuration: o
                .fields
                .iter()
                .find(|o| o.name == "configuration")
                .unwrap()
                .output
                .duplicate(),
            mode: o
                .fields
                .iter()
                .find(|o| o.name == "mode")
                .unwrap()
                .output
                .duplicate(),
            notes: o
                .fields
                .iter()
                .find(|o| o.name == "notes")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        access_service_token::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            client_id: o
                .fields
                .iter()
                .find(|o| o.name == "clientId")
                .unwrap()
                .output
                .duplicate(),
            client_secret: o
                .fields
                .iter()
                .find(|o| o.name == "clientSecret")
                .unwrap()
                .output
                .duplicate(),
            duration: o
                .fields
                .iter()
                .find(|o| o.name == "duration")
                .unwrap()
                .output
                .duplicate(),
            expires_at: o
                .fields
                .iter()
                .find(|o| o.name == "expiresAt")
                .unwrap()
                .output
                .duplicate(),
            min_days_for_renewal: o
                .fields
                .iter()
                .find(|o| o.name == "minDaysForRenewal")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        access_tag::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            app_count: o
                .fields
                .iter()
                .find(|o| o.name == "appCount")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        account::Res {
            enforce_twofactor: o
                .fields
                .iter()
                .find(|o| o.name == "enforceTwofactor")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            type_: o
                .fields
                .iter()
                .find(|o| o.name == "type")
                .unwrap()
                .output
                .duplicate(),
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

        account_member::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            email_address: o
                .fields
                .iter()
                .find(|o| o.name == "emailAddress")
                .unwrap()
                .output
                .duplicate(),
            role_ids: o
                .fields
                .iter()
                .find(|o| o.name == "roleIds")
                .unwrap()
                .output
                .duplicate(),
            status: o
                .fields
                .iter()
                .find(|o| o.name == "status")
                .unwrap()
                .output
                .duplicate(),
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

        address_map::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            can_delete: o
                .fields
                .iter()
                .find(|o| o.name == "canDelete")
                .unwrap()
                .output
                .duplicate(),
            can_modify_ips: o
                .fields
                .iter()
                .find(|o| o.name == "canModifyIps")
                .unwrap()
                .output
                .duplicate(),
            default_sni: o
                .fields
                .iter()
                .find(|o| o.name == "defaultSni")
                .unwrap()
                .output
                .duplicate(),
            description: o
                .fields
                .iter()
                .find(|o| o.name == "description")
                .unwrap()
                .output
                .duplicate(),
            enabled: o
                .fields
                .iter()
                .find(|o| o.name == "enabled")
                .unwrap()
                .output
                .duplicate(),
            ips: o
                .fields
                .iter()
                .find(|o| o.name == "ips")
                .unwrap()
                .output
                .duplicate(),
            memberships: o
                .fields
                .iter()
                .find(|o| o.name == "memberships")
                .unwrap()
                .output
                .duplicate(),
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

        api_shield::Res {
            auth_id_characteristics: o
                .fields
                .iter()
                .find(|o| o.name == "authIdCharacteristics")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        api_shield_operation::Res {
            endpoint: o
                .fields
                .iter()
                .find(|o| o.name == "endpoint")
                .unwrap()
                .output
                .duplicate(),
            host: o
                .fields
                .iter()
                .find(|o| o.name == "host")
                .unwrap()
                .output
                .duplicate(),
            method: o
                .fields
                .iter()
                .find(|o| o.name == "method")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        api_shield_operation_schema_validation_settings::Res {
            mitigation_action: o
                .fields
                .iter()
                .find(|o| o.name == "mitigationAction")
                .unwrap()
                .output
                .duplicate(),
            operation_id: o
                .fields
                .iter()
                .find(|o| o.name == "operationId")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        api_shield_schema::Res {
            kind: o
                .fields
                .iter()
                .find(|o| o.name == "kind")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            source: o
                .fields
                .iter()
                .find(|o| o.name == "source")
                .unwrap()
                .output
                .duplicate(),
            validation_enabled: o
                .fields
                .iter()
                .find(|o| o.name == "validationEnabled")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        api_shield_schema_validation_settings::Res {
            validation_default_mitigation_action: o
                .fields
                .iter()
                .find(|o| o.name == "validationDefaultMitigationAction")
                .unwrap()
                .output
                .duplicate(),
            validation_override_mitigation_action: o
                .fields
                .iter()
                .find(|o| o.name == "validationOverrideMitigationAction")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        api_token::Res {
            condition: o
                .fields
                .iter()
                .find(|o| o.name == "condition")
                .unwrap()
                .output
                .duplicate(),
            expires_on: o
                .fields
                .iter()
                .find(|o| o.name == "expiresOn")
                .unwrap()
                .output
                .duplicate(),
            issued_on: o
                .fields
                .iter()
                .find(|o| o.name == "issuedOn")
                .unwrap()
                .output
                .duplicate(),
            modified_on: o
                .fields
                .iter()
                .find(|o| o.name == "modifiedOn")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            not_before: o
                .fields
                .iter()
                .find(|o| o.name == "notBefore")
                .unwrap()
                .output
                .duplicate(),
            policies: o
                .fields
                .iter()
                .find(|o| o.name == "policies")
                .unwrap()
                .output
                .duplicate(),
            status: o
                .fields
                .iter()
                .find(|o| o.name == "status")
                .unwrap()
                .output
                .duplicate(),
            value: o
                .fields
                .iter()
                .find(|o| o.name == "value")
                .unwrap()
                .output
                .duplicate(),
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

        argo::Res {
            smart_routing: o
                .fields
                .iter()
                .find(|o| o.name == "smartRouting")
                .unwrap()
                .output
                .duplicate(),
            tiered_caching: o
                .fields
                .iter()
                .find(|o| o.name == "tieredCaching")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        authenticated_origin_pulls::Res {
            authenticated_origin_pulls_certificate: o
                .fields
                .iter()
                .find(|o| o.name == "authenticatedOriginPullsCertificate")
                .unwrap()
                .output
                .duplicate(),
            enabled: o
                .fields
                .iter()
                .find(|o| o.name == "enabled")
                .unwrap()
                .output
                .duplicate(),
            hostname: o
                .fields
                .iter()
                .find(|o| o.name == "hostname")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        authenticated_origin_pulls_certificate::Res {
            certificate: o
                .fields
                .iter()
                .find(|o| o.name == "certificate")
                .unwrap()
                .output
                .duplicate(),
            expires_on: o
                .fields
                .iter()
                .find(|o| o.name == "expiresOn")
                .unwrap()
                .output
                .duplicate(),
            issuer: o
                .fields
                .iter()
                .find(|o| o.name == "issuer")
                .unwrap()
                .output
                .duplicate(),
            private_key: o
                .fields
                .iter()
                .find(|o| o.name == "privateKey")
                .unwrap()
                .output
                .duplicate(),
            serial_number: o
                .fields
                .iter()
                .find(|o| o.name == "serialNumber")
                .unwrap()
                .output
                .duplicate(),
            signature: o
                .fields
                .iter()
                .find(|o| o.name == "signature")
                .unwrap()
                .output
                .duplicate(),
            status: o
                .fields
                .iter()
                .find(|o| o.name == "status")
                .unwrap()
                .output
                .duplicate(),
            type_: o
                .fields
                .iter()
                .find(|o| o.name == "type")
                .unwrap()
                .output
                .duplicate(),
            uploaded_on: o
                .fields
                .iter()
                .find(|o| o.name == "uploadedOn")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        bot_management::Res {
            auto_update_model: o
                .fields
                .iter()
                .find(|o| o.name == "autoUpdateModel")
                .unwrap()
                .output
                .duplicate(),
            enable_js: o
                .fields
                .iter()
                .find(|o| o.name == "enableJs")
                .unwrap()
                .output
                .duplicate(),
            fight_mode: o
                .fields
                .iter()
                .find(|o| o.name == "fightMode")
                .unwrap()
                .output
                .duplicate(),
            optimize_wordpress: o
                .fields
                .iter()
                .find(|o| o.name == "optimizeWordpress")
                .unwrap()
                .output
                .duplicate(),
            sbfm_definitely_automated: o
                .fields
                .iter()
                .find(|o| o.name == "sbfmDefinitelyAutomated")
                .unwrap()
                .output
                .duplicate(),
            sbfm_likely_automated: o
                .fields
                .iter()
                .find(|o| o.name == "sbfmLikelyAutomated")
                .unwrap()
                .output
                .duplicate(),
            sbfm_static_resource_protection: o
                .fields
                .iter()
                .find(|o| o.name == "sbfmStaticResourceProtection")
                .unwrap()
                .output
                .duplicate(),
            sbfm_verified_bots: o
                .fields
                .iter()
                .find(|o| o.name == "sbfmVerifiedBots")
                .unwrap()
                .output
                .duplicate(),
            suppress_session_score: o
                .fields
                .iter()
                .find(|o| o.name == "suppressSessionScore")
                .unwrap()
                .output
                .duplicate(),
            using_latest_model: o
                .fields
                .iter()
                .find(|o| o.name == "usingLatestModel")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        byo_ip_prefix::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            advertisement: o
                .fields
                .iter()
                .find(|o| o.name == "advertisement")
                .unwrap()
                .output
                .duplicate(),
            description: o
                .fields
                .iter()
                .find(|o| o.name == "description")
                .unwrap()
                .output
                .duplicate(),
            prefix_id: o
                .fields
                .iter()
                .find(|o| o.name == "prefixId")
                .unwrap()
                .output
                .duplicate(),
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

        certificate_pack::Res {
            certificate_authority: o
                .fields
                .iter()
                .find(|o| o.name == "certificateAuthority")
                .unwrap()
                .output
                .duplicate(),
            cloudflare_branding: o
                .fields
                .iter()
                .find(|o| o.name == "cloudflareBranding")
                .unwrap()
                .output
                .duplicate(),
            hosts: o
                .fields
                .iter()
                .find(|o| o.name == "hosts")
                .unwrap()
                .output
                .duplicate(),
            type_: o
                .fields
                .iter()
                .find(|o| o.name == "type")
                .unwrap()
                .output
                .duplicate(),
            validation_errors: o
                .fields
                .iter()
                .find(|o| o.name == "validationErrors")
                .unwrap()
                .output
                .duplicate(),
            validation_method: o
                .fields
                .iter()
                .find(|o| o.name == "validationMethod")
                .unwrap()
                .output
                .duplicate(),
            validation_records: o
                .fields
                .iter()
                .find(|o| o.name == "validationRecords")
                .unwrap()
                .output
                .duplicate(),
            validity_days: o
                .fields
                .iter()
                .find(|o| o.name == "validityDays")
                .unwrap()
                .output
                .duplicate(),
            wait_for_active_status: o
                .fields
                .iter()
                .find(|o| o.name == "waitForActiveStatus")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        custom_hostname::Res {
            custom_metadata: o
                .fields
                .iter()
                .find(|o| o.name == "customMetadata")
                .unwrap()
                .output
                .duplicate(),
            custom_origin_server: o
                .fields
                .iter()
                .find(|o| o.name == "customOriginServer")
                .unwrap()
                .output
                .duplicate(),
            custom_origin_sni: o
                .fields
                .iter()
                .find(|o| o.name == "customOriginSni")
                .unwrap()
                .output
                .duplicate(),
            hostname: o
                .fields
                .iter()
                .find(|o| o.name == "hostname")
                .unwrap()
                .output
                .duplicate(),
            ownership_verification: o
                .fields
                .iter()
                .find(|o| o.name == "ownershipVerification")
                .unwrap()
                .output
                .duplicate(),
            ownership_verification_http: o
                .fields
                .iter()
                .find(|o| o.name == "ownershipVerificationHttp")
                .unwrap()
                .output
                .duplicate(),
            ssls: o
                .fields
                .iter()
                .find(|o| o.name == "ssls")
                .unwrap()
                .output
                .duplicate(),
            status: o
                .fields
                .iter()
                .find(|o| o.name == "status")
                .unwrap()
                .output
                .duplicate(),
            wait_for_ssl_pending_validation: o
                .fields
                .iter()
                .find(|o| o.name == "waitForSslPendingValidation")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        custom_hostname_fallback_origin::Res {
            origin: o
                .fields
                .iter()
                .find(|o| o.name == "origin")
                .unwrap()
                .output
                .duplicate(),
            status: o
                .fields
                .iter()
                .find(|o| o.name == "status")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        custom_pages::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            state: o
                .fields
                .iter()
                .find(|o| o.name == "state")
                .unwrap()
                .output
                .duplicate(),
            type_: o
                .fields
                .iter()
                .find(|o| o.name == "type")
                .unwrap()
                .output
                .duplicate(),
            url: o
                .fields
                .iter()
                .find(|o| o.name == "url")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        custom_ssl::Res {
            custom_ssl_options: o
                .fields
                .iter()
                .find(|o| o.name == "customSslOptions")
                .unwrap()
                .output
                .duplicate(),
            custom_ssl_priorities: o
                .fields
                .iter()
                .find(|o| o.name == "customSslPriorities")
                .unwrap()
                .output
                .duplicate(),
            expires_on: o
                .fields
                .iter()
                .find(|o| o.name == "expiresOn")
                .unwrap()
                .output
                .duplicate(),
            hosts: o
                .fields
                .iter()
                .find(|o| o.name == "hosts")
                .unwrap()
                .output
                .duplicate(),
            issuer: o
                .fields
                .iter()
                .find(|o| o.name == "issuer")
                .unwrap()
                .output
                .duplicate(),
            modified_on: o
                .fields
                .iter()
                .find(|o| o.name == "modifiedOn")
                .unwrap()
                .output
                .duplicate(),
            priority: o
                .fields
                .iter()
                .find(|o| o.name == "priority")
                .unwrap()
                .output
                .duplicate(),
            signature: o
                .fields
                .iter()
                .find(|o| o.name == "signature")
                .unwrap()
                .output
                .duplicate(),
            status: o
                .fields
                .iter()
                .find(|o| o.name == "status")
                .unwrap()
                .output
                .duplicate(),
            uploaded_on: o
                .fields
                .iter()
                .find(|o| o.name == "uploadedOn")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        d1_database::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            version: o
                .fields
                .iter()
                .find(|o| o.name == "version")
                .unwrap()
                .output
                .duplicate(),
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

        device_dex_test::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            created: o
                .fields
                .iter()
                .find(|o| o.name == "created")
                .unwrap()
                .output
                .duplicate(),
            data: o
                .fields
                .iter()
                .find(|o| o.name == "data")
                .unwrap()
                .output
                .duplicate(),
            description: o
                .fields
                .iter()
                .find(|o| o.name == "description")
                .unwrap()
                .output
                .duplicate(),
            enabled: o
                .fields
                .iter()
                .find(|o| o.name == "enabled")
                .unwrap()
                .output
                .duplicate(),
            interval: o
                .fields
                .iter()
                .find(|o| o.name == "interval")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            updated: o
                .fields
                .iter()
                .find(|o| o.name == "updated")
                .unwrap()
                .output
                .duplicate(),
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

        device_managed_networks::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            config: o
                .fields
                .iter()
                .find(|o| o.name == "config")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            type_: o
                .fields
                .iter()
                .find(|o| o.name == "type")
                .unwrap()
                .output
                .duplicate(),
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

        device_policy_certificates::Res {
            enabled: o
                .fields
                .iter()
                .find(|o| o.name == "enabled")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        device_posture_integration::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            configs: o
                .fields
                .iter()
                .find(|o| o.name == "configs")
                .unwrap()
                .output
                .duplicate(),
            identifier: o
                .fields
                .iter()
                .find(|o| o.name == "identifier")
                .unwrap()
                .output
                .duplicate(),
            interval: o
                .fields
                .iter()
                .find(|o| o.name == "interval")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            type_: o
                .fields
                .iter()
                .find(|o| o.name == "type")
                .unwrap()
                .output
                .duplicate(),
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

        device_posture_rule::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            description: o
                .fields
                .iter()
                .find(|o| o.name == "description")
                .unwrap()
                .output
                .duplicate(),
            expiration: o
                .fields
                .iter()
                .find(|o| o.name == "expiration")
                .unwrap()
                .output
                .duplicate(),
            inputs: o
                .fields
                .iter()
                .find(|o| o.name == "inputs")
                .unwrap()
                .output
                .duplicate(),
            matches: o
                .fields
                .iter()
                .find(|o| o.name == "matches")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            schedule: o
                .fields
                .iter()
                .find(|o| o.name == "schedule")
                .unwrap()
                .output
                .duplicate(),
            type_: o
                .fields
                .iter()
                .find(|o| o.name == "type")
                .unwrap()
                .output
                .duplicate(),
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

        device_settings_policy::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            allow_mode_switch: o
                .fields
                .iter()
                .find(|o| o.name == "allowModeSwitch")
                .unwrap()
                .output
                .duplicate(),
            allow_updates: o
                .fields
                .iter()
                .find(|o| o.name == "allowUpdates")
                .unwrap()
                .output
                .duplicate(),
            allowed_to_leave: o
                .fields
                .iter()
                .find(|o| o.name == "allowedToLeave")
                .unwrap()
                .output
                .duplicate(),
            auto_connect: o
                .fields
                .iter()
                .find(|o| o.name == "autoConnect")
                .unwrap()
                .output
                .duplicate(),
            captive_portal: o
                .fields
                .iter()
                .find(|o| o.name == "captivePortal")
                .unwrap()
                .output
                .duplicate(),
            default: o
                .fields
                .iter()
                .find(|o| o.name == "default")
                .unwrap()
                .output
                .duplicate(),
            description: o
                .fields
                .iter()
                .find(|o| o.name == "description")
                .unwrap()
                .output
                .duplicate(),
            disable_auto_fallback: o
                .fields
                .iter()
                .find(|o| o.name == "disableAutoFallback")
                .unwrap()
                .output
                .duplicate(),
            enabled: o
                .fields
                .iter()
                .find(|o| o.name == "enabled")
                .unwrap()
                .output
                .duplicate(),
            exclude_office_ips: o
                .fields
                .iter()
                .find(|o| o.name == "excludeOfficeIps")
                .unwrap()
                .output
                .duplicate(),
            match_: o
                .fields
                .iter()
                .find(|o| o.name == "match")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            precedence: o
                .fields
                .iter()
                .find(|o| o.name == "precedence")
                .unwrap()
                .output
                .duplicate(),
            service_mode_v2_mode: o
                .fields
                .iter()
                .find(|o| o.name == "serviceModeV2Mode")
                .unwrap()
                .output
                .duplicate(),
            service_mode_v2_port: o
                .fields
                .iter()
                .find(|o| o.name == "serviceModeV2Port")
                .unwrap()
                .output
                .duplicate(),
            support_url: o
                .fields
                .iter()
                .find(|o| o.name == "supportUrl")
                .unwrap()
                .output
                .duplicate(),
            switch_locked: o
                .fields
                .iter()
                .find(|o| o.name == "switchLocked")
                .unwrap()
                .output
                .duplicate(),
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

        dlp_profile::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            allowed_match_count: o
                .fields
                .iter()
                .find(|o| o.name == "allowedMatchCount")
                .unwrap()
                .output
                .duplicate(),
            context_awareness: o
                .fields
                .iter()
                .find(|o| o.name == "contextAwareness")
                .unwrap()
                .output
                .duplicate(),
            description: o
                .fields
                .iter()
                .find(|o| o.name == "description")
                .unwrap()
                .output
                .duplicate(),
            entries: o
                .fields
                .iter()
                .find(|o| o.name == "entries")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            type_: o
                .fields
                .iter()
                .find(|o| o.name == "type")
                .unwrap()
                .output
                .duplicate(),
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

        email_routing_address::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            created: o
                .fields
                .iter()
                .find(|o| o.name == "created")
                .unwrap()
                .output
                .duplicate(),
            email: o
                .fields
                .iter()
                .find(|o| o.name == "email")
                .unwrap()
                .output
                .duplicate(),
            modified: o
                .fields
                .iter()
                .find(|o| o.name == "modified")
                .unwrap()
                .output
                .duplicate(),
            tag: o
                .fields
                .iter()
                .find(|o| o.name == "tag")
                .unwrap()
                .output
                .duplicate(),
            verified: o
                .fields
                .iter()
                .find(|o| o.name == "verified")
                .unwrap()
                .output
                .duplicate(),
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

        email_routing_catch_all::Res {
            actions: o
                .fields
                .iter()
                .find(|o| o.name == "actions")
                .unwrap()
                .output
                .duplicate(),
            enabled: o
                .fields
                .iter()
                .find(|o| o.name == "enabled")
                .unwrap()
                .output
                .duplicate(),
            matchers: o
                .fields
                .iter()
                .find(|o| o.name == "matchers")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            tag: o
                .fields
                .iter()
                .find(|o| o.name == "tag")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        email_routing_rule::Res {
            actions: o
                .fields
                .iter()
                .find(|o| o.name == "actions")
                .unwrap()
                .output
                .duplicate(),
            enabled: o
                .fields
                .iter()
                .find(|o| o.name == "enabled")
                .unwrap()
                .output
                .duplicate(),
            matchers: o
                .fields
                .iter()
                .find(|o| o.name == "matchers")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            priority: o
                .fields
                .iter()
                .find(|o| o.name == "priority")
                .unwrap()
                .output
                .duplicate(),
            tag: o
                .fields
                .iter()
                .find(|o| o.name == "tag")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        email_routing_settings::Res {
            created: o
                .fields
                .iter()
                .find(|o| o.name == "created")
                .unwrap()
                .output
                .duplicate(),
            enabled: o
                .fields
                .iter()
                .find(|o| o.name == "enabled")
                .unwrap()
                .output
                .duplicate(),
            modified: o
                .fields
                .iter()
                .find(|o| o.name == "modified")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            skip_wizard: o
                .fields
                .iter()
                .find(|o| o.name == "skipWizard")
                .unwrap()
                .output
                .duplicate(),
            status: o
                .fields
                .iter()
                .find(|o| o.name == "status")
                .unwrap()
                .output
                .duplicate(),
            tag: o
                .fields
                .iter()
                .find(|o| o.name == "tag")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        fallback_domain::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            domains: o
                .fields
                .iter()
                .find(|o| o.name == "domains")
                .unwrap()
                .output
                .duplicate(),
            policy_id: o
                .fields
                .iter()
                .find(|o| o.name == "policyId")
                .unwrap()
                .output
                .duplicate(),
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

        filter::Res {
            description: o
                .fields
                .iter()
                .find(|o| o.name == "description")
                .unwrap()
                .output
                .duplicate(),
            expression: o
                .fields
                .iter()
                .find(|o| o.name == "expression")
                .unwrap()
                .output
                .duplicate(),
            paused: o
                .fields
                .iter()
                .find(|o| o.name == "paused")
                .unwrap()
                .output
                .duplicate(),
            ref_: o
                .fields
                .iter()
                .find(|o| o.name == "ref")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        firewall_rule::Res {
            action: o
                .fields
                .iter()
                .find(|o| o.name == "action")
                .unwrap()
                .output
                .duplicate(),
            description: o
                .fields
                .iter()
                .find(|o| o.name == "description")
                .unwrap()
                .output
                .duplicate(),
            filter_id: o
                .fields
                .iter()
                .find(|o| o.name == "filterId")
                .unwrap()
                .output
                .duplicate(),
            paused: o
                .fields
                .iter()
                .find(|o| o.name == "paused")
                .unwrap()
                .output
                .duplicate(),
            priority: o
                .fields
                .iter()
                .find(|o| o.name == "priority")
                .unwrap()
                .output
                .duplicate(),
            products: o
                .fields
                .iter()
                .find(|o| o.name == "products")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        gre_tunnel::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            cloudflare_gre_endpoint: o
                .fields
                .iter()
                .find(|o| o.name == "cloudflareGreEndpoint")
                .unwrap()
                .output
                .duplicate(),
            customer_gre_endpoint: o
                .fields
                .iter()
                .find(|o| o.name == "customerGreEndpoint")
                .unwrap()
                .output
                .duplicate(),
            description: o
                .fields
                .iter()
                .find(|o| o.name == "description")
                .unwrap()
                .output
                .duplicate(),
            health_check_enabled: o
                .fields
                .iter()
                .find(|o| o.name == "healthCheckEnabled")
                .unwrap()
                .output
                .duplicate(),
            health_check_target: o
                .fields
                .iter()
                .find(|o| o.name == "healthCheckTarget")
                .unwrap()
                .output
                .duplicate(),
            health_check_type: o
                .fields
                .iter()
                .find(|o| o.name == "healthCheckType")
                .unwrap()
                .output
                .duplicate(),
            interface_address: o
                .fields
                .iter()
                .find(|o| o.name == "interfaceAddress")
                .unwrap()
                .output
                .duplicate(),
            mtu: o
                .fields
                .iter()
                .find(|o| o.name == "mtu")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            ttl: o
                .fields
                .iter()
                .find(|o| o.name == "ttl")
                .unwrap()
                .output
                .duplicate(),
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

        healthcheck::Res {
            address: o
                .fields
                .iter()
                .find(|o| o.name == "address")
                .unwrap()
                .output
                .duplicate(),
            allow_insecure: o
                .fields
                .iter()
                .find(|o| o.name == "allowInsecure")
                .unwrap()
                .output
                .duplicate(),
            check_regions: o
                .fields
                .iter()
                .find(|o| o.name == "checkRegions")
                .unwrap()
                .output
                .duplicate(),
            consecutive_fails: o
                .fields
                .iter()
                .find(|o| o.name == "consecutiveFails")
                .unwrap()
                .output
                .duplicate(),
            consecutive_successes: o
                .fields
                .iter()
                .find(|o| o.name == "consecutiveSuccesses")
                .unwrap()
                .output
                .duplicate(),
            created_on: o
                .fields
                .iter()
                .find(|o| o.name == "createdOn")
                .unwrap()
                .output
                .duplicate(),
            description: o
                .fields
                .iter()
                .find(|o| o.name == "description")
                .unwrap()
                .output
                .duplicate(),
            expected_body: o
                .fields
                .iter()
                .find(|o| o.name == "expectedBody")
                .unwrap()
                .output
                .duplicate(),
            expected_codes: o
                .fields
                .iter()
                .find(|o| o.name == "expectedCodes")
                .unwrap()
                .output
                .duplicate(),
            follow_redirects: o
                .fields
                .iter()
                .find(|o| o.name == "followRedirects")
                .unwrap()
                .output
                .duplicate(),
            headers: o
                .fields
                .iter()
                .find(|o| o.name == "headers")
                .unwrap()
                .output
                .duplicate(),
            interval: o
                .fields
                .iter()
                .find(|o| o.name == "interval")
                .unwrap()
                .output
                .duplicate(),
            method: o
                .fields
                .iter()
                .find(|o| o.name == "method")
                .unwrap()
                .output
                .duplicate(),
            modified_on: o
                .fields
                .iter()
                .find(|o| o.name == "modifiedOn")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            path: o
                .fields
                .iter()
                .find(|o| o.name == "path")
                .unwrap()
                .output
                .duplicate(),
            port: o
                .fields
                .iter()
                .find(|o| o.name == "port")
                .unwrap()
                .output
                .duplicate(),
            retries: o
                .fields
                .iter()
                .find(|o| o.name == "retries")
                .unwrap()
                .output
                .duplicate(),
            suspended: o
                .fields
                .iter()
                .find(|o| o.name == "suspended")
                .unwrap()
                .output
                .duplicate(),
            timeout: o
                .fields
                .iter()
                .find(|o| o.name == "timeout")
                .unwrap()
                .output
                .duplicate(),
            type_: o
                .fields
                .iter()
                .find(|o| o.name == "type")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        hostname_tls_setting::Res {
            created_at: o
                .fields
                .iter()
                .find(|o| o.name == "createdAt")
                .unwrap()
                .output
                .duplicate(),
            hostname: o
                .fields
                .iter()
                .find(|o| o.name == "hostname")
                .unwrap()
                .output
                .duplicate(),
            setting: o
                .fields
                .iter()
                .find(|o| o.name == "setting")
                .unwrap()
                .output
                .duplicate(),
            updated_at: o
                .fields
                .iter()
                .find(|o| o.name == "updatedAt")
                .unwrap()
                .output
                .duplicate(),
            value: o
                .fields
                .iter()
                .find(|o| o.name == "value")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        hostname_tls_setting_ciphers::Res {
            created_at: o
                .fields
                .iter()
                .find(|o| o.name == "createdAt")
                .unwrap()
                .output
                .duplicate(),
            hostname: o
                .fields
                .iter()
                .find(|o| o.name == "hostname")
                .unwrap()
                .output
                .duplicate(),
            ports: o
                .fields
                .iter()
                .find(|o| o.name == "ports")
                .unwrap()
                .output
                .duplicate(),
            updated_at: o
                .fields
                .iter()
                .find(|o| o.name == "updatedAt")
                .unwrap()
                .output
                .duplicate(),
            values: o
                .fields
                .iter()
                .find(|o| o.name == "values")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        hyperdrive_config::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            caching: o
                .fields
                .iter()
                .find(|o| o.name == "caching")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            origin: o
                .fields
                .iter()
                .find(|o| o.name == "origin")
                .unwrap()
                .output
                .duplicate(),
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

        ipsec_tunnel::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            allow_null_cipher: o
                .fields
                .iter()
                .find(|o| o.name == "allowNullCipher")
                .unwrap()
                .output
                .duplicate(),
            cloudflare_endpoint: o
                .fields
                .iter()
                .find(|o| o.name == "cloudflareEndpoint")
                .unwrap()
                .output
                .duplicate(),
            customer_endpoint: o
                .fields
                .iter()
                .find(|o| o.name == "customerEndpoint")
                .unwrap()
                .output
                .duplicate(),
            description: o
                .fields
                .iter()
                .find(|o| o.name == "description")
                .unwrap()
                .output
                .duplicate(),
            fqdn_id: o
                .fields
                .iter()
                .find(|o| o.name == "fqdnId")
                .unwrap()
                .output
                .duplicate(),
            health_check_direction: o
                .fields
                .iter()
                .find(|o| o.name == "healthCheckDirection")
                .unwrap()
                .output
                .duplicate(),
            health_check_enabled: o
                .fields
                .iter()
                .find(|o| o.name == "healthCheckEnabled")
                .unwrap()
                .output
                .duplicate(),
            health_check_rate: o
                .fields
                .iter()
                .find(|o| o.name == "healthCheckRate")
                .unwrap()
                .output
                .duplicate(),
            health_check_target: o
                .fields
                .iter()
                .find(|o| o.name == "healthCheckTarget")
                .unwrap()
                .output
                .duplicate(),
            health_check_type: o
                .fields
                .iter()
                .find(|o| o.name == "healthCheckType")
                .unwrap()
                .output
                .duplicate(),
            hex_id: o
                .fields
                .iter()
                .find(|o| o.name == "hexId")
                .unwrap()
                .output
                .duplicate(),
            interface_address: o
                .fields
                .iter()
                .find(|o| o.name == "interfaceAddress")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            psk: o
                .fields
                .iter()
                .find(|o| o.name == "psk")
                .unwrap()
                .output
                .duplicate(),
            remote_id: o
                .fields
                .iter()
                .find(|o| o.name == "remoteId")
                .unwrap()
                .output
                .duplicate(),
            user_id: o
                .fields
                .iter()
                .find(|o| o.name == "userId")
                .unwrap()
                .output
                .duplicate(),
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

        keyless_certificate::Res {
            bundle_method: o
                .fields
                .iter()
                .find(|o| o.name == "bundleMethod")
                .unwrap()
                .output
                .duplicate(),
            certificate: o
                .fields
                .iter()
                .find(|o| o.name == "certificate")
                .unwrap()
                .output
                .duplicate(),
            enabled: o
                .fields
                .iter()
                .find(|o| o.name == "enabled")
                .unwrap()
                .output
                .duplicate(),
            host: o
                .fields
                .iter()
                .find(|o| o.name == "host")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            port: o
                .fields
                .iter()
                .find(|o| o.name == "port")
                .unwrap()
                .output
                .duplicate(),
            status: o
                .fields
                .iter()
                .find(|o| o.name == "status")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        list::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            description: o
                .fields
                .iter()
                .find(|o| o.name == "description")
                .unwrap()
                .output
                .duplicate(),
            items: o
                .fields
                .iter()
                .find(|o| o.name == "items")
                .unwrap()
                .output
                .duplicate(),
            kind: o
                .fields
                .iter()
                .find(|o| o.name == "kind")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
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

        list_item::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            asn: o
                .fields
                .iter()
                .find(|o| o.name == "asn")
                .unwrap()
                .output
                .duplicate(),
            comment: o
                .fields
                .iter()
                .find(|o| o.name == "comment")
                .unwrap()
                .output
                .duplicate(),
            hostname: o
                .fields
                .iter()
                .find(|o| o.name == "hostname")
                .unwrap()
                .output
                .duplicate(),
            ip: o
                .fields
                .iter()
                .find(|o| o.name == "ip")
                .unwrap()
                .output
                .duplicate(),
            list_id: o
                .fields
                .iter()
                .find(|o| o.name == "listId")
                .unwrap()
                .output
                .duplicate(),
            redirect: o
                .fields
                .iter()
                .find(|o| o.name == "redirect")
                .unwrap()
                .output
                .duplicate(),
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

        load_balancer::Res {
            adaptive_routings: o
                .fields
                .iter()
                .find(|o| o.name == "adaptiveRoutings")
                .unwrap()
                .output
                .duplicate(),
            country_pools: o
                .fields
                .iter()
                .find(|o| o.name == "countryPools")
                .unwrap()
                .output
                .duplicate(),
            created_on: o
                .fields
                .iter()
                .find(|o| o.name == "createdOn")
                .unwrap()
                .output
                .duplicate(),
            default_pool_ids: o
                .fields
                .iter()
                .find(|o| o.name == "defaultPoolIds")
                .unwrap()
                .output
                .duplicate(),
            description: o
                .fields
                .iter()
                .find(|o| o.name == "description")
                .unwrap()
                .output
                .duplicate(),
            enabled: o
                .fields
                .iter()
                .find(|o| o.name == "enabled")
                .unwrap()
                .output
                .duplicate(),
            fallback_pool_id: o
                .fields
                .iter()
                .find(|o| o.name == "fallbackPoolId")
                .unwrap()
                .output
                .duplicate(),
            location_strategies: o
                .fields
                .iter()
                .find(|o| o.name == "locationStrategies")
                .unwrap()
                .output
                .duplicate(),
            modified_on: o
                .fields
                .iter()
                .find(|o| o.name == "modifiedOn")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            pop_pools: o
                .fields
                .iter()
                .find(|o| o.name == "popPools")
                .unwrap()
                .output
                .duplicate(),
            proxied: o
                .fields
                .iter()
                .find(|o| o.name == "proxied")
                .unwrap()
                .output
                .duplicate(),
            random_steerings: o
                .fields
                .iter()
                .find(|o| o.name == "randomSteerings")
                .unwrap()
                .output
                .duplicate(),
            region_pools: o
                .fields
                .iter()
                .find(|o| o.name == "regionPools")
                .unwrap()
                .output
                .duplicate(),
            rules: o
                .fields
                .iter()
                .find(|o| o.name == "rules")
                .unwrap()
                .output
                .duplicate(),
            session_affinity: o
                .fields
                .iter()
                .find(|o| o.name == "sessionAffinity")
                .unwrap()
                .output
                .duplicate(),
            session_affinity_attributes: o
                .fields
                .iter()
                .find(|o| o.name == "sessionAffinityAttributes")
                .unwrap()
                .output
                .duplicate(),
            session_affinity_ttl: o
                .fields
                .iter()
                .find(|o| o.name == "sessionAffinityTtl")
                .unwrap()
                .output
                .duplicate(),
            steering_policy: o
                .fields
                .iter()
                .find(|o| o.name == "steeringPolicy")
                .unwrap()
                .output
                .duplicate(),
            ttl: o
                .fields
                .iter()
                .find(|o| o.name == "ttl")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        load_balancer_monitor::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            allow_insecure: o
                .fields
                .iter()
                .find(|o| o.name == "allowInsecure")
                .unwrap()
                .output
                .duplicate(),
            consecutive_down: o
                .fields
                .iter()
                .find(|o| o.name == "consecutiveDown")
                .unwrap()
                .output
                .duplicate(),
            consecutive_up: o
                .fields
                .iter()
                .find(|o| o.name == "consecutiveUp")
                .unwrap()
                .output
                .duplicate(),
            created_on: o
                .fields
                .iter()
                .find(|o| o.name == "createdOn")
                .unwrap()
                .output
                .duplicate(),
            description: o
                .fields
                .iter()
                .find(|o| o.name == "description")
                .unwrap()
                .output
                .duplicate(),
            expected_body: o
                .fields
                .iter()
                .find(|o| o.name == "expectedBody")
                .unwrap()
                .output
                .duplicate(),
            expected_codes: o
                .fields
                .iter()
                .find(|o| o.name == "expectedCodes")
                .unwrap()
                .output
                .duplicate(),
            follow_redirects: o
                .fields
                .iter()
                .find(|o| o.name == "followRedirects")
                .unwrap()
                .output
                .duplicate(),
            headers: o
                .fields
                .iter()
                .find(|o| o.name == "headers")
                .unwrap()
                .output
                .duplicate(),
            interval: o
                .fields
                .iter()
                .find(|o| o.name == "interval")
                .unwrap()
                .output
                .duplicate(),
            method: o
                .fields
                .iter()
                .find(|o| o.name == "method")
                .unwrap()
                .output
                .duplicate(),
            modified_on: o
                .fields
                .iter()
                .find(|o| o.name == "modifiedOn")
                .unwrap()
                .output
                .duplicate(),
            path: o
                .fields
                .iter()
                .find(|o| o.name == "path")
                .unwrap()
                .output
                .duplicate(),
            port: o
                .fields
                .iter()
                .find(|o| o.name == "port")
                .unwrap()
                .output
                .duplicate(),
            probe_zone: o
                .fields
                .iter()
                .find(|o| o.name == "probeZone")
                .unwrap()
                .output
                .duplicate(),
            retries: o
                .fields
                .iter()
                .find(|o| o.name == "retries")
                .unwrap()
                .output
                .duplicate(),
            timeout: o
                .fields
                .iter()
                .find(|o| o.name == "timeout")
                .unwrap()
                .output
                .duplicate(),
            type_: o
                .fields
                .iter()
                .find(|o| o.name == "type")
                .unwrap()
                .output
                .duplicate(),
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

        load_balancer_pool::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            check_regions: o
                .fields
                .iter()
                .find(|o| o.name == "checkRegions")
                .unwrap()
                .output
                .duplicate(),
            created_on: o
                .fields
                .iter()
                .find(|o| o.name == "createdOn")
                .unwrap()
                .output
                .duplicate(),
            description: o
                .fields
                .iter()
                .find(|o| o.name == "description")
                .unwrap()
                .output
                .duplicate(),
            enabled: o
                .fields
                .iter()
                .find(|o| o.name == "enabled")
                .unwrap()
                .output
                .duplicate(),
            latitude: o
                .fields
                .iter()
                .find(|o| o.name == "latitude")
                .unwrap()
                .output
                .duplicate(),
            load_sheddings: o
                .fields
                .iter()
                .find(|o| o.name == "loadSheddings")
                .unwrap()
                .output
                .duplicate(),
            longitude: o
                .fields
                .iter()
                .find(|o| o.name == "longitude")
                .unwrap()
                .output
                .duplicate(),
            minimum_origins: o
                .fields
                .iter()
                .find(|o| o.name == "minimumOrigins")
                .unwrap()
                .output
                .duplicate(),
            modified_on: o
                .fields
                .iter()
                .find(|o| o.name == "modifiedOn")
                .unwrap()
                .output
                .duplicate(),
            monitor: o
                .fields
                .iter()
                .find(|o| o.name == "monitor")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            notification_email: o
                .fields
                .iter()
                .find(|o| o.name == "notificationEmail")
                .unwrap()
                .output
                .duplicate(),
            origin_steerings: o
                .fields
                .iter()
                .find(|o| o.name == "originSteerings")
                .unwrap()
                .output
                .duplicate(),
            origins: o
                .fields
                .iter()
                .find(|o| o.name == "origins")
                .unwrap()
                .output
                .duplicate(),
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

        logpull_retention::Res {
            enabled: o
                .fields
                .iter()
                .find(|o| o.name == "enabled")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        logpush_job::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            dataset: o
                .fields
                .iter()
                .find(|o| o.name == "dataset")
                .unwrap()
                .output
                .duplicate(),
            destination_conf: o
                .fields
                .iter()
                .find(|o| o.name == "destinationConf")
                .unwrap()
                .output
                .duplicate(),
            enabled: o
                .fields
                .iter()
                .find(|o| o.name == "enabled")
                .unwrap()
                .output
                .duplicate(),
            filter: o
                .fields
                .iter()
                .find(|o| o.name == "filter")
                .unwrap()
                .output
                .duplicate(),
            frequency: o
                .fields
                .iter()
                .find(|o| o.name == "frequency")
                .unwrap()
                .output
                .duplicate(),
            kind: o
                .fields
                .iter()
                .find(|o| o.name == "kind")
                .unwrap()
                .output
                .duplicate(),
            logpull_options: o
                .fields
                .iter()
                .find(|o| o.name == "logpullOptions")
                .unwrap()
                .output
                .duplicate(),
            max_upload_bytes: o
                .fields
                .iter()
                .find(|o| o.name == "maxUploadBytes")
                .unwrap()
                .output
                .duplicate(),
            max_upload_interval_seconds: o
                .fields
                .iter()
                .find(|o| o.name == "maxUploadIntervalSeconds")
                .unwrap()
                .output
                .duplicate(),
            max_upload_records: o
                .fields
                .iter()
                .find(|o| o.name == "maxUploadRecords")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            output_options: o
                .fields
                .iter()
                .find(|o| o.name == "outputOptions")
                .unwrap()
                .output
                .duplicate(),
            ownership_challenge: o
                .fields
                .iter()
                .find(|o| o.name == "ownershipChallenge")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        logpush_ownership_challenge::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            destination_conf: o
                .fields
                .iter()
                .find(|o| o.name == "destinationConf")
                .unwrap()
                .output
                .duplicate(),
            ownership_challenge_filename: o
                .fields
                .iter()
                .find(|o| o.name == "ownershipChallengeFilename")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        magic_firewall_ruleset::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            description: o
                .fields
                .iter()
                .find(|o| o.name == "description")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            rules: o
                .fields
                .iter()
                .find(|o| o.name == "rules")
                .unwrap()
                .output
                .duplicate(),
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

        managed_headers::Res {
            managed_request_headers: o
                .fields
                .iter()
                .find(|o| o.name == "managedRequestHeaders")
                .unwrap()
                .output
                .duplicate(),
            managed_response_headers: o
                .fields
                .iter()
                .find(|o| o.name == "managedResponseHeaders")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        mtls_certificate::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            ca: o
                .fields
                .iter()
                .find(|o| o.name == "ca")
                .unwrap()
                .output
                .duplicate(),
            certificates: o
                .fields
                .iter()
                .find(|o| o.name == "certificates")
                .unwrap()
                .output
                .duplicate(),
            expires_on: o
                .fields
                .iter()
                .find(|o| o.name == "expiresOn")
                .unwrap()
                .output
                .duplicate(),
            issuer: o
                .fields
                .iter()
                .find(|o| o.name == "issuer")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            private_key: o
                .fields
                .iter()
                .find(|o| o.name == "privateKey")
                .unwrap()
                .output
                .duplicate(),
            serial_number: o
                .fields
                .iter()
                .find(|o| o.name == "serialNumber")
                .unwrap()
                .output
                .duplicate(),
            signature: o
                .fields
                .iter()
                .find(|o| o.name == "signature")
                .unwrap()
                .output
                .duplicate(),
            uploaded_on: o
                .fields
                .iter()
                .find(|o| o.name == "uploadedOn")
                .unwrap()
                .output
                .duplicate(),
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

        notification_policy::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            alert_type: o
                .fields
                .iter()
                .find(|o| o.name == "alertType")
                .unwrap()
                .output
                .duplicate(),
            created: o
                .fields
                .iter()
                .find(|o| o.name == "created")
                .unwrap()
                .output
                .duplicate(),
            description: o
                .fields
                .iter()
                .find(|o| o.name == "description")
                .unwrap()
                .output
                .duplicate(),
            email_integrations: o
                .fields
                .iter()
                .find(|o| o.name == "emailIntegrations")
                .unwrap()
                .output
                .duplicate(),
            enabled: o
                .fields
                .iter()
                .find(|o| o.name == "enabled")
                .unwrap()
                .output
                .duplicate(),
            filters: o
                .fields
                .iter()
                .find(|o| o.name == "filters")
                .unwrap()
                .output
                .duplicate(),
            modified: o
                .fields
                .iter()
                .find(|o| o.name == "modified")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            pagerduty_integrations: o
                .fields
                .iter()
                .find(|o| o.name == "pagerdutyIntegrations")
                .unwrap()
                .output
                .duplicate(),
            webhooks_integrations: o
                .fields
                .iter()
                .find(|o| o.name == "webhooksIntegrations")
                .unwrap()
                .output
                .duplicate(),
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

        notification_policy_webhooks::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            created_at: o
                .fields
                .iter()
                .find(|o| o.name == "createdAt")
                .unwrap()
                .output
                .duplicate(),
            last_failure: o
                .fields
                .iter()
                .find(|o| o.name == "lastFailure")
                .unwrap()
                .output
                .duplicate(),
            last_success: o
                .fields
                .iter()
                .find(|o| o.name == "lastSuccess")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            secret: o
                .fields
                .iter()
                .find(|o| o.name == "secret")
                .unwrap()
                .output
                .duplicate(),
            type_: o
                .fields
                .iter()
                .find(|o| o.name == "type")
                .unwrap()
                .output
                .duplicate(),
            url: o
                .fields
                .iter()
                .find(|o| o.name == "url")
                .unwrap()
                .output
                .duplicate(),
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

        observatory_scheduled_test::Res {
            frequency: o
                .fields
                .iter()
                .find(|o| o.name == "frequency")
                .unwrap()
                .output
                .duplicate(),
            region: o
                .fields
                .iter()
                .find(|o| o.name == "region")
                .unwrap()
                .output
                .duplicate(),
            url: o
                .fields
                .iter()
                .find(|o| o.name == "url")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        origin_ca_certificate::Res {
            certificate: o
                .fields
                .iter()
                .find(|o| o.name == "certificate")
                .unwrap()
                .output
                .duplicate(),
            csr: o
                .fields
                .iter()
                .find(|o| o.name == "csr")
                .unwrap()
                .output
                .duplicate(),
            expires_on: o
                .fields
                .iter()
                .find(|o| o.name == "expiresOn")
                .unwrap()
                .output
                .duplicate(),
            hostnames: o
                .fields
                .iter()
                .find(|o| o.name == "hostnames")
                .unwrap()
                .output
                .duplicate(),
            min_days_for_renewal: o
                .fields
                .iter()
                .find(|o| o.name == "minDaysForRenewal")
                .unwrap()
                .output
                .duplicate(),
            request_type: o
                .fields
                .iter()
                .find(|o| o.name == "requestType")
                .unwrap()
                .output
                .duplicate(),
            requested_validity: o
                .fields
                .iter()
                .find(|o| o.name == "requestedValidity")
                .unwrap()
                .output
                .duplicate(),
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

        page_rule::Res {
            actions: o
                .fields
                .iter()
                .find(|o| o.name == "actions")
                .unwrap()
                .output
                .duplicate(),
            priority: o
                .fields
                .iter()
                .find(|o| o.name == "priority")
                .unwrap()
                .output
                .duplicate(),
            status: o
                .fields
                .iter()
                .find(|o| o.name == "status")
                .unwrap()
                .output
                .duplicate(),
            target: o
                .fields
                .iter()
                .find(|o| o.name == "target")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        pages_domain::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            domain: o
                .fields
                .iter()
                .find(|o| o.name == "domain")
                .unwrap()
                .output
                .duplicate(),
            project_name: o
                .fields
                .iter()
                .find(|o| o.name == "projectName")
                .unwrap()
                .output
                .duplicate(),
            status: o
                .fields
                .iter()
                .find(|o| o.name == "status")
                .unwrap()
                .output
                .duplicate(),
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

        pages_project::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            build_config: o
                .fields
                .iter()
                .find(|o| o.name == "buildConfig")
                .unwrap()
                .output
                .duplicate(),
            created_on: o
                .fields
                .iter()
                .find(|o| o.name == "createdOn")
                .unwrap()
                .output
                .duplicate(),
            deployment_configs: o
                .fields
                .iter()
                .find(|o| o.name == "deploymentConfigs")
                .unwrap()
                .output
                .duplicate(),
            domains: o
                .fields
                .iter()
                .find(|o| o.name == "domains")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            production_branch: o
                .fields
                .iter()
                .find(|o| o.name == "productionBranch")
                .unwrap()
                .output
                .duplicate(),
            source: o
                .fields
                .iter()
                .find(|o| o.name == "source")
                .unwrap()
                .output
                .duplicate(),
            subdomain: o
                .fields
                .iter()
                .find(|o| o.name == "subdomain")
                .unwrap()
                .output
                .duplicate(),
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

        queue::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
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

        r2_bucket::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            location: o
                .fields
                .iter()
                .find(|o| o.name == "location")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
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

        rate_limit::Res {
            action: o
                .fields
                .iter()
                .find(|o| o.name == "action")
                .unwrap()
                .output
                .duplicate(),
            bypass_url_patterns: o
                .fields
                .iter()
                .find(|o| o.name == "bypassUrlPatterns")
                .unwrap()
                .output
                .duplicate(),
            correlate: o
                .fields
                .iter()
                .find(|o| o.name == "correlate")
                .unwrap()
                .output
                .duplicate(),
            description: o
                .fields
                .iter()
                .find(|o| o.name == "description")
                .unwrap()
                .output
                .duplicate(),
            disabled: o
                .fields
                .iter()
                .find(|o| o.name == "disabled")
                .unwrap()
                .output
                .duplicate(),
            match_: o
                .fields
                .iter()
                .find(|o| o.name == "match")
                .unwrap()
                .output
                .duplicate(),
            period: o
                .fields
                .iter()
                .find(|o| o.name == "period")
                .unwrap()
                .output
                .duplicate(),
            threshold: o
                .fields
                .iter()
                .find(|o| o.name == "threshold")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        record::Res {
            allow_overwrite: o
                .fields
                .iter()
                .find(|o| o.name == "allowOverwrite")
                .unwrap()
                .output
                .duplicate(),
            comment: o
                .fields
                .iter()
                .find(|o| o.name == "comment")
                .unwrap()
                .output
                .duplicate(),
            created_on: o
                .fields
                .iter()
                .find(|o| o.name == "createdOn")
                .unwrap()
                .output
                .duplicate(),
            data: o
                .fields
                .iter()
                .find(|o| o.name == "data")
                .unwrap()
                .output
                .duplicate(),
            hostname: o
                .fields
                .iter()
                .find(|o| o.name == "hostname")
                .unwrap()
                .output
                .duplicate(),
            metadata: o
                .fields
                .iter()
                .find(|o| o.name == "metadata")
                .unwrap()
                .output
                .duplicate(),
            modified_on: o
                .fields
                .iter()
                .find(|o| o.name == "modifiedOn")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            priority: o
                .fields
                .iter()
                .find(|o| o.name == "priority")
                .unwrap()
                .output
                .duplicate(),
            proxiable: o
                .fields
                .iter()
                .find(|o| o.name == "proxiable")
                .unwrap()
                .output
                .duplicate(),
            proxied: o
                .fields
                .iter()
                .find(|o| o.name == "proxied")
                .unwrap()
                .output
                .duplicate(),
            tags: o
                .fields
                .iter()
                .find(|o| o.name == "tags")
                .unwrap()
                .output
                .duplicate(),
            ttl: o
                .fields
                .iter()
                .find(|o| o.name == "ttl")
                .unwrap()
                .output
                .duplicate(),
            type_: o
                .fields
                .iter()
                .find(|o| o.name == "type")
                .unwrap()
                .output
                .duplicate(),
            value: o
                .fields
                .iter()
                .find(|o| o.name == "value")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        regional_hostname::Res {
            created_on: o
                .fields
                .iter()
                .find(|o| o.name == "createdOn")
                .unwrap()
                .output
                .duplicate(),
            hostname: o
                .fields
                .iter()
                .find(|o| o.name == "hostname")
                .unwrap()
                .output
                .duplicate(),
            region_key: o
                .fields
                .iter()
                .find(|o| o.name == "regionKey")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        regional_tiered_cache::Res {
            value: o
                .fields
                .iter()
                .find(|o| o.name == "value")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        ruleset::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            description: o
                .fields
                .iter()
                .find(|o| o.name == "description")
                .unwrap()
                .output
                .duplicate(),
            kind: o
                .fields
                .iter()
                .find(|o| o.name == "kind")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            phase: o
                .fields
                .iter()
                .find(|o| o.name == "phase")
                .unwrap()
                .output
                .duplicate(),
            rules: o
                .fields
                .iter()
                .find(|o| o.name == "rules")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        spectrum_application::Res {
            argo_smart_routing: o
                .fields
                .iter()
                .find(|o| o.name == "argoSmartRouting")
                .unwrap()
                .output
                .duplicate(),
            dns: o
                .fields
                .iter()
                .find(|o| o.name == "dns")
                .unwrap()
                .output
                .duplicate(),
            edge_ips: o
                .fields
                .iter()
                .find(|o| o.name == "edgeIps")
                .unwrap()
                .output
                .duplicate(),
            ip_firewall: o
                .fields
                .iter()
                .find(|o| o.name == "ipFirewall")
                .unwrap()
                .output
                .duplicate(),
            origin_directs: o
                .fields
                .iter()
                .find(|o| o.name == "originDirects")
                .unwrap()
                .output
                .duplicate(),
            origin_dns: o
                .fields
                .iter()
                .find(|o| o.name == "originDns")
                .unwrap()
                .output
                .duplicate(),
            origin_port: o
                .fields
                .iter()
                .find(|o| o.name == "originPort")
                .unwrap()
                .output
                .duplicate(),
            origin_port_range: o
                .fields
                .iter()
                .find(|o| o.name == "originPortRange")
                .unwrap()
                .output
                .duplicate(),
            protocol: o
                .fields
                .iter()
                .find(|o| o.name == "protocol")
                .unwrap()
                .output
                .duplicate(),
            proxy_protocol: o
                .fields
                .iter()
                .find(|o| o.name == "proxyProtocol")
                .unwrap()
                .output
                .duplicate(),
            tls: o
                .fields
                .iter()
                .find(|o| o.name == "tls")
                .unwrap()
                .output
                .duplicate(),
            traffic_type: o
                .fields
                .iter()
                .find(|o| o.name == "trafficType")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        split_tunnel::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            mode: o
                .fields
                .iter()
                .find(|o| o.name == "mode")
                .unwrap()
                .output
                .duplicate(),
            policy_id: o
                .fields
                .iter()
                .find(|o| o.name == "policyId")
                .unwrap()
                .output
                .duplicate(),
            tunnels: o
                .fields
                .iter()
                .find(|o| o.name == "tunnels")
                .unwrap()
                .output
                .duplicate(),
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

        static_route::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            colo_names: o
                .fields
                .iter()
                .find(|o| o.name == "coloNames")
                .unwrap()
                .output
                .duplicate(),
            colo_regions: o
                .fields
                .iter()
                .find(|o| o.name == "coloRegions")
                .unwrap()
                .output
                .duplicate(),
            description: o
                .fields
                .iter()
                .find(|o| o.name == "description")
                .unwrap()
                .output
                .duplicate(),
            nexthop: o
                .fields
                .iter()
                .find(|o| o.name == "nexthop")
                .unwrap()
                .output
                .duplicate(),
            prefix: o
                .fields
                .iter()
                .find(|o| o.name == "prefix")
                .unwrap()
                .output
                .duplicate(),
            priority: o
                .fields
                .iter()
                .find(|o| o.name == "priority")
                .unwrap()
                .output
                .duplicate(),
            weight: o
                .fields
                .iter()
                .find(|o| o.name == "weight")
                .unwrap()
                .output
                .duplicate(),
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

        teams_account::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            activity_log_enabled: o
                .fields
                .iter()
                .find(|o| o.name == "activityLogEnabled")
                .unwrap()
                .output
                .duplicate(),
            antivirus: o
                .fields
                .iter()
                .find(|o| o.name == "antivirus")
                .unwrap()
                .output
                .duplicate(),
            block_page: o
                .fields
                .iter()
                .find(|o| o.name == "blockPage")
                .unwrap()
                .output
                .duplicate(),
            body_scanning: o
                .fields
                .iter()
                .find(|o| o.name == "bodyScanning")
                .unwrap()
                .output
                .duplicate(),
            extended_email_matching: o
                .fields
                .iter()
                .find(|o| o.name == "extendedEmailMatching")
                .unwrap()
                .output
                .duplicate(),
            fips: o
                .fields
                .iter()
                .find(|o| o.name == "fips")
                .unwrap()
                .output
                .duplicate(),
            logging: o
                .fields
                .iter()
                .find(|o| o.name == "logging")
                .unwrap()
                .output
                .duplicate(),
            non_identity_browser_isolation_enabled: o
                .fields
                .iter()
                .find(|o| o.name == "nonIdentityBrowserIsolationEnabled")
                .unwrap()
                .output
                .duplicate(),
            payload_log: o
                .fields
                .iter()
                .find(|o| o.name == "payloadLog")
                .unwrap()
                .output
                .duplicate(),
            protocol_detection_enabled: o
                .fields
                .iter()
                .find(|o| o.name == "protocolDetectionEnabled")
                .unwrap()
                .output
                .duplicate(),
            proxy: o
                .fields
                .iter()
                .find(|o| o.name == "proxy")
                .unwrap()
                .output
                .duplicate(),
            ssh_session_log: o
                .fields
                .iter()
                .find(|o| o.name == "sshSessionLog")
                .unwrap()
                .output
                .duplicate(),
            tls_decrypt_enabled: o
                .fields
                .iter()
                .find(|o| o.name == "tlsDecryptEnabled")
                .unwrap()
                .output
                .duplicate(),
            url_browser_isolation_enabled: o
                .fields
                .iter()
                .find(|o| o.name == "urlBrowserIsolationEnabled")
                .unwrap()
                .output
                .duplicate(),
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

        teams_list::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            description: o
                .fields
                .iter()
                .find(|o| o.name == "description")
                .unwrap()
                .output
                .duplicate(),
            items: o
                .fields
                .iter()
                .find(|o| o.name == "items")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            type_: o
                .fields
                .iter()
                .find(|o| o.name == "type")
                .unwrap()
                .output
                .duplicate(),
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

        teams_location::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            anonymized_logs_enabled: o
                .fields
                .iter()
                .find(|o| o.name == "anonymizedLogsEnabled")
                .unwrap()
                .output
                .duplicate(),
            client_default: o
                .fields
                .iter()
                .find(|o| o.name == "clientDefault")
                .unwrap()
                .output
                .duplicate(),
            doh_subdomain: o
                .fields
                .iter()
                .find(|o| o.name == "dohSubdomain")
                .unwrap()
                .output
                .duplicate(),
            ip: o
                .fields
                .iter()
                .find(|o| o.name == "ip")
                .unwrap()
                .output
                .duplicate(),
            ipv4_destination: o
                .fields
                .iter()
                .find(|o| o.name == "ipv4Destination")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            networks: o
                .fields
                .iter()
                .find(|o| o.name == "networks")
                .unwrap()
                .output
                .duplicate(),
            policy_ids: o
                .fields
                .iter()
                .find(|o| o.name == "policyIds")
                .unwrap()
                .output
                .duplicate(),
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

        teams_proxy_endpoint::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            ips: o
                .fields
                .iter()
                .find(|o| o.name == "ips")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            subdomain: o
                .fields
                .iter()
                .find(|o| o.name == "subdomain")
                .unwrap()
                .output
                .duplicate(),
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

        teams_rule::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            action: o
                .fields
                .iter()
                .find(|o| o.name == "action")
                .unwrap()
                .output
                .duplicate(),
            description: o
                .fields
                .iter()
                .find(|o| o.name == "description")
                .unwrap()
                .output
                .duplicate(),
            device_posture: o
                .fields
                .iter()
                .find(|o| o.name == "devicePosture")
                .unwrap()
                .output
                .duplicate(),
            enabled: o
                .fields
                .iter()
                .find(|o| o.name == "enabled")
                .unwrap()
                .output
                .duplicate(),
            filters: o
                .fields
                .iter()
                .find(|o| o.name == "filters")
                .unwrap()
                .output
                .duplicate(),
            identity: o
                .fields
                .iter()
                .find(|o| o.name == "identity")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            precedence: o
                .fields
                .iter()
                .find(|o| o.name == "precedence")
                .unwrap()
                .output
                .duplicate(),
            rule_settings: o
                .fields
                .iter()
                .find(|o| o.name == "ruleSettings")
                .unwrap()
                .output
                .duplicate(),
            traffic: o
                .fields
                .iter()
                .find(|o| o.name == "traffic")
                .unwrap()
                .output
                .duplicate(),
            version: o
                .fields
                .iter()
                .find(|o| o.name == "version")
                .unwrap()
                .output
                .duplicate(),
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

        tiered_cache::Res {
            cache_type: o
                .fields
                .iter()
                .find(|o| o.name == "cacheType")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        total_tls::Res {
            certificate_authority: o
                .fields
                .iter()
                .find(|o| o.name == "certificateAuthority")
                .unwrap()
                .output
                .duplicate(),
            enabled: o
                .fields
                .iter()
                .find(|o| o.name == "enabled")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        tunnel::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            cname: o
                .fields
                .iter()
                .find(|o| o.name == "cname")
                .unwrap()
                .output
                .duplicate(),
            config_src: o
                .fields
                .iter()
                .find(|o| o.name == "configSrc")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            secret: o
                .fields
                .iter()
                .find(|o| o.name == "secret")
                .unwrap()
                .output
                .duplicate(),
            tunnel_token: o
                .fields
                .iter()
                .find(|o| o.name == "tunnelToken")
                .unwrap()
                .output
                .duplicate(),
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

        tunnel_config::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            config: o
                .fields
                .iter()
                .find(|o| o.name == "config")
                .unwrap()
                .output
                .duplicate(),
            tunnel_id: o
                .fields
                .iter()
                .find(|o| o.name == "tunnelId")
                .unwrap()
                .output
                .duplicate(),
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

        tunnel_route::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            comment: o
                .fields
                .iter()
                .find(|o| o.name == "comment")
                .unwrap()
                .output
                .duplicate(),
            network: o
                .fields
                .iter()
                .find(|o| o.name == "network")
                .unwrap()
                .output
                .duplicate(),
            tunnel_id: o
                .fields
                .iter()
                .find(|o| o.name == "tunnelId")
                .unwrap()
                .output
                .duplicate(),
            virtual_network_id: o
                .fields
                .iter()
                .find(|o| o.name == "virtualNetworkId")
                .unwrap()
                .output
                .duplicate(),
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

        tunnel_virtual_network::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            comment: o
                .fields
                .iter()
                .find(|o| o.name == "comment")
                .unwrap()
                .output
                .duplicate(),
            is_default_network: o
                .fields
                .iter()
                .find(|o| o.name == "isDefaultNetwork")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
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

        turnstile_widget::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            bot_fight_mode: o
                .fields
                .iter()
                .find(|o| o.name == "botFightMode")
                .unwrap()
                .output
                .duplicate(),
            domains: o
                .fields
                .iter()
                .find(|o| o.name == "domains")
                .unwrap()
                .output
                .duplicate(),
            mode: o
                .fields
                .iter()
                .find(|o| o.name == "mode")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            offlabel: o
                .fields
                .iter()
                .find(|o| o.name == "offlabel")
                .unwrap()
                .output
                .duplicate(),
            region: o
                .fields
                .iter()
                .find(|o| o.name == "region")
                .unwrap()
                .output
                .duplicate(),
            secret: o
                .fields
                .iter()
                .find(|o| o.name == "secret")
                .unwrap()
                .output
                .duplicate(),
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

        url_normalization_settings::Res {
            scope: o
                .fields
                .iter()
                .find(|o| o.name == "scope")
                .unwrap()
                .output
                .duplicate(),
            type_: o
                .fields
                .iter()
                .find(|o| o.name == "type")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        user_agent_blocking_rule::Res {
            configuration: o
                .fields
                .iter()
                .find(|o| o.name == "configuration")
                .unwrap()
                .output
                .duplicate(),
            description: o
                .fields
                .iter()
                .find(|o| o.name == "description")
                .unwrap()
                .output
                .duplicate(),
            mode: o
                .fields
                .iter()
                .find(|o| o.name == "mode")
                .unwrap()
                .output
                .duplicate(),
            paused: o
                .fields
                .iter()
                .find(|o| o.name == "paused")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        waiting_room::Res {
            additional_routes: o
                .fields
                .iter()
                .find(|o| o.name == "additionalRoutes")
                .unwrap()
                .output
                .duplicate(),
            cookie_suffix: o
                .fields
                .iter()
                .find(|o| o.name == "cookieSuffix")
                .unwrap()
                .output
                .duplicate(),
            custom_page_html: o
                .fields
                .iter()
                .find(|o| o.name == "customPageHtml")
                .unwrap()
                .output
                .duplicate(),
            default_template_language: o
                .fields
                .iter()
                .find(|o| o.name == "defaultTemplateLanguage")
                .unwrap()
                .output
                .duplicate(),
            description: o
                .fields
                .iter()
                .find(|o| o.name == "description")
                .unwrap()
                .output
                .duplicate(),
            disable_session_renewal: o
                .fields
                .iter()
                .find(|o| o.name == "disableSessionRenewal")
                .unwrap()
                .output
                .duplicate(),
            host: o
                .fields
                .iter()
                .find(|o| o.name == "host")
                .unwrap()
                .output
                .duplicate(),
            json_response_enabled: o
                .fields
                .iter()
                .find(|o| o.name == "jsonResponseEnabled")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            new_users_per_minute: o
                .fields
                .iter()
                .find(|o| o.name == "newUsersPerMinute")
                .unwrap()
                .output
                .duplicate(),
            path: o
                .fields
                .iter()
                .find(|o| o.name == "path")
                .unwrap()
                .output
                .duplicate(),
            queue_all: o
                .fields
                .iter()
                .find(|o| o.name == "queueAll")
                .unwrap()
                .output
                .duplicate(),
            queueing_method: o
                .fields
                .iter()
                .find(|o| o.name == "queueingMethod")
                .unwrap()
                .output
                .duplicate(),
            queueing_status_code: o
                .fields
                .iter()
                .find(|o| o.name == "queueingStatusCode")
                .unwrap()
                .output
                .duplicate(),
            session_duration: o
                .fields
                .iter()
                .find(|o| o.name == "sessionDuration")
                .unwrap()
                .output
                .duplicate(),
            suspended: o
                .fields
                .iter()
                .find(|o| o.name == "suspended")
                .unwrap()
                .output
                .duplicate(),
            total_active_users: o
                .fields
                .iter()
                .find(|o| o.name == "totalActiveUsers")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        waiting_room_event::Res {
            created_on: o
                .fields
                .iter()
                .find(|o| o.name == "createdOn")
                .unwrap()
                .output
                .duplicate(),
            custom_page_html: o
                .fields
                .iter()
                .find(|o| o.name == "customPageHtml")
                .unwrap()
                .output
                .duplicate(),
            description: o
                .fields
                .iter()
                .find(|o| o.name == "description")
                .unwrap()
                .output
                .duplicate(),
            disable_session_renewal: o
                .fields
                .iter()
                .find(|o| o.name == "disableSessionRenewal")
                .unwrap()
                .output
                .duplicate(),
            event_end_time: o
                .fields
                .iter()
                .find(|o| o.name == "eventEndTime")
                .unwrap()
                .output
                .duplicate(),
            event_start_time: o
                .fields
                .iter()
                .find(|o| o.name == "eventStartTime")
                .unwrap()
                .output
                .duplicate(),
            modified_on: o
                .fields
                .iter()
                .find(|o| o.name == "modifiedOn")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            new_users_per_minute: o
                .fields
                .iter()
                .find(|o| o.name == "newUsersPerMinute")
                .unwrap()
                .output
                .duplicate(),
            prequeue_start_time: o
                .fields
                .iter()
                .find(|o| o.name == "prequeueStartTime")
                .unwrap()
                .output
                .duplicate(),
            queueing_method: o
                .fields
                .iter()
                .find(|o| o.name == "queueingMethod")
                .unwrap()
                .output
                .duplicate(),
            session_duration: o
                .fields
                .iter()
                .find(|o| o.name == "sessionDuration")
                .unwrap()
                .output
                .duplicate(),
            shuffle_at_event_start: o
                .fields
                .iter()
                .find(|o| o.name == "shuffleAtEventStart")
                .unwrap()
                .output
                .duplicate(),
            suspended: o
                .fields
                .iter()
                .find(|o| o.name == "suspended")
                .unwrap()
                .output
                .duplicate(),
            total_active_users: o
                .fields
                .iter()
                .find(|o| o.name == "totalActiveUsers")
                .unwrap()
                .output
                .duplicate(),
            waiting_room_id: o
                .fields
                .iter()
                .find(|o| o.name == "waitingRoomId")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        waiting_room_rules::Res {
            rules: o
                .fields
                .iter()
                .find(|o| o.name == "rules")
                .unwrap()
                .output
                .duplicate(),
            waiting_room_id: o
                .fields
                .iter()
                .find(|o| o.name == "waitingRoomId")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        waiting_room_settings::Res {
            search_engine_crawler_bypass: o
                .fields
                .iter()
                .find(|o| o.name == "searchEngineCrawlerBypass")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        web3_hostname::Res {
            created_on: o
                .fields
                .iter()
                .find(|o| o.name == "createdOn")
                .unwrap()
                .output
                .duplicate(),
            description: o
                .fields
                .iter()
                .find(|o| o.name == "description")
                .unwrap()
                .output
                .duplicate(),
            dnslink: o
                .fields
                .iter()
                .find(|o| o.name == "dnslink")
                .unwrap()
                .output
                .duplicate(),
            modified_on: o
                .fields
                .iter()
                .find(|o| o.name == "modifiedOn")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            status: o
                .fields
                .iter()
                .find(|o| o.name == "status")
                .unwrap()
                .output
                .duplicate(),
            target: o
                .fields
                .iter()
                .find(|o| o.name == "target")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        web_analytics_rule::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            host: o
                .fields
                .iter()
                .find(|o| o.name == "host")
                .unwrap()
                .output
                .duplicate(),
            inclusive: o
                .fields
                .iter()
                .find(|o| o.name == "inclusive")
                .unwrap()
                .output
                .duplicate(),
            is_paused: o
                .fields
                .iter()
                .find(|o| o.name == "isPaused")
                .unwrap()
                .output
                .duplicate(),
            paths: o
                .fields
                .iter()
                .find(|o| o.name == "paths")
                .unwrap()
                .output
                .duplicate(),
            ruleset_id: o
                .fields
                .iter()
                .find(|o| o.name == "rulesetId")
                .unwrap()
                .output
                .duplicate(),
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

        web_analytics_site::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            auto_install: o
                .fields
                .iter()
                .find(|o| o.name == "autoInstall")
                .unwrap()
                .output
                .duplicate(),
            host: o
                .fields
                .iter()
                .find(|o| o.name == "host")
                .unwrap()
                .output
                .duplicate(),
            ruleset_id: o
                .fields
                .iter()
                .find(|o| o.name == "rulesetId")
                .unwrap()
                .output
                .duplicate(),
            site_tag: o
                .fields
                .iter()
                .find(|o| o.name == "siteTag")
                .unwrap()
                .output
                .duplicate(),
            site_token: o
                .fields
                .iter()
                .find(|o| o.name == "siteToken")
                .unwrap()
                .output
                .duplicate(),
            snippet: o
                .fields
                .iter()
                .find(|o| o.name == "snippet")
                .unwrap()
                .output
                .duplicate(),
            zone_tag: o
                .fields
                .iter()
                .find(|o| o.name == "zoneTag")
                .unwrap()
                .output
                .duplicate(),
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

        worker_cron_trigger::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            schedules: o
                .fields
                .iter()
                .find(|o| o.name == "schedules")
                .unwrap()
                .output
                .duplicate(),
            script_name: o
                .fields
                .iter()
                .find(|o| o.name == "scriptName")
                .unwrap()
                .output
                .duplicate(),
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

        worker_domain::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            environment: o
                .fields
                .iter()
                .find(|o| o.name == "environment")
                .unwrap()
                .output
                .duplicate(),
            hostname: o
                .fields
                .iter()
                .find(|o| o.name == "hostname")
                .unwrap()
                .output
                .duplicate(),
            service: o
                .fields
                .iter()
                .find(|o| o.name == "service")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        worker_route::Res {
            pattern: o
                .fields
                .iter()
                .find(|o| o.name == "pattern")
                .unwrap()
                .output
                .duplicate(),
            script_name: o
                .fields
                .iter()
                .find(|o| o.name == "scriptName")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        worker_script::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            analytics_engine_bindings: o
                .fields
                .iter()
                .find(|o| o.name == "analyticsEngineBindings")
                .unwrap()
                .output
                .duplicate(),
            compatibility_date: o
                .fields
                .iter()
                .find(|o| o.name == "compatibilityDate")
                .unwrap()
                .output
                .duplicate(),
            compatibility_flags: o
                .fields
                .iter()
                .find(|o| o.name == "compatibilityFlags")
                .unwrap()
                .output
                .duplicate(),
            content: o
                .fields
                .iter()
                .find(|o| o.name == "content")
                .unwrap()
                .output
                .duplicate(),
            d1_database_bindings: o
                .fields
                .iter()
                .find(|o| o.name == "d1DatabaseBindings")
                .unwrap()
                .output
                .duplicate(),
            dispatch_namespace: o
                .fields
                .iter()
                .find(|o| o.name == "dispatchNamespace")
                .unwrap()
                .output
                .duplicate(),
            kv_namespace_bindings: o
                .fields
                .iter()
                .find(|o| o.name == "kvNamespaceBindings")
                .unwrap()
                .output
                .duplicate(),
            logpush: o
                .fields
                .iter()
                .find(|o| o.name == "logpush")
                .unwrap()
                .output
                .duplicate(),
            module: o
                .fields
                .iter()
                .find(|o| o.name == "module")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            placements: o
                .fields
                .iter()
                .find(|o| o.name == "placements")
                .unwrap()
                .output
                .duplicate(),
            plain_text_bindings: o
                .fields
                .iter()
                .find(|o| o.name == "plainTextBindings")
                .unwrap()
                .output
                .duplicate(),
            queue_bindings: o
                .fields
                .iter()
                .find(|o| o.name == "queueBindings")
                .unwrap()
                .output
                .duplicate(),
            r2_bucket_bindings: o
                .fields
                .iter()
                .find(|o| o.name == "r2BucketBindings")
                .unwrap()
                .output
                .duplicate(),
            secret_text_bindings: o
                .fields
                .iter()
                .find(|o| o.name == "secretTextBindings")
                .unwrap()
                .output
                .duplicate(),
            service_bindings: o
                .fields
                .iter()
                .find(|o| o.name == "serviceBindings")
                .unwrap()
                .output
                .duplicate(),
            tags: o
                .fields
                .iter()
                .find(|o| o.name == "tags")
                .unwrap()
                .output
                .duplicate(),
            webassembly_bindings: o
                .fields
                .iter()
                .find(|o| o.name == "webassemblyBindings")
                .unwrap()
                .output
                .duplicate(),
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

        worker_secret::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
            script_name: o
                .fields
                .iter()
                .find(|o| o.name == "scriptName")
                .unwrap()
                .output
                .duplicate(),
            secret_text: o
                .fields
                .iter()
                .find(|o| o.name == "secretText")
                .unwrap()
                .output
                .duplicate(),
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

        workers_for_platforms_namespace::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            name: o
                .fields
                .iter()
                .find(|o| o.name == "name")
                .unwrap()
                .output
                .duplicate(),
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

        workers_kv::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            key: o
                .fields
                .iter()
                .find(|o| o.name == "key")
                .unwrap()
                .output
                .duplicate(),
            namespace_id: o
                .fields
                .iter()
                .find(|o| o.name == "namespaceId")
                .unwrap()
                .output
                .duplicate(),
            value: o
                .fields
                .iter()
                .find(|o| o.name == "value")
                .unwrap()
                .output
                .duplicate(),
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

        workers_kv_namespace::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            title: o
                .fields
                .iter()
                .find(|o| o.name == "title")
                .unwrap()
                .output
                .duplicate(),
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

        zone::Res {
            account_id: o
                .fields
                .iter()
                .find(|o| o.name == "accountId")
                .unwrap()
                .output
                .duplicate(),
            jump_start: o
                .fields
                .iter()
                .find(|o| o.name == "jumpStart")
                .unwrap()
                .output
                .duplicate(),
            meta: o
                .fields
                .iter()
                .find(|o| o.name == "meta")
                .unwrap()
                .output
                .duplicate(),
            name_servers: o
                .fields
                .iter()
                .find(|o| o.name == "nameServers")
                .unwrap()
                .output
                .duplicate(),
            paused: o
                .fields
                .iter()
                .find(|o| o.name == "paused")
                .unwrap()
                .output
                .duplicate(),
            plan: o
                .fields
                .iter()
                .find(|o| o.name == "plan")
                .unwrap()
                .output
                .duplicate(),
            status: o
                .fields
                .iter()
                .find(|o| o.name == "status")
                .unwrap()
                .output
                .duplicate(),
            type_: o
                .fields
                .iter()
                .find(|o| o.name == "type")
                .unwrap()
                .output
                .duplicate(),
            vanity_name_servers: o
                .fields
                .iter()
                .find(|o| o.name == "vanityNameServers")
                .unwrap()
                .output
                .duplicate(),
            verification_key: o
                .fields
                .iter()
                .find(|o| o.name == "verificationKey")
                .unwrap()
                .output
                .duplicate(),
            zone: o
                .fields
                .iter()
                .find(|o| o.name == "zone")
                .unwrap()
                .output
                .duplicate(),
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

        zone_cache_reserve::Res {
            enabled: o
                .fields
                .iter()
                .find(|o| o.name == "enabled")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        zone_cache_variants::Res {
            avifs: o
                .fields
                .iter()
                .find(|o| o.name == "avifs")
                .unwrap()
                .output
                .duplicate(),
            bmps: o
                .fields
                .iter()
                .find(|o| o.name == "bmps")
                .unwrap()
                .output
                .duplicate(),
            gifs: o
                .fields
                .iter()
                .find(|o| o.name == "gifs")
                .unwrap()
                .output
                .duplicate(),
            jp2s: o
                .fields
                .iter()
                .find(|o| o.name == "jp2s")
                .unwrap()
                .output
                .duplicate(),
            jpegs: o
                .fields
                .iter()
                .find(|o| o.name == "jpegs")
                .unwrap()
                .output
                .duplicate(),
            jpg2s: o
                .fields
                .iter()
                .find(|o| o.name == "jpg2s")
                .unwrap()
                .output
                .duplicate(),
            jpgs: o
                .fields
                .iter()
                .find(|o| o.name == "jpgs")
                .unwrap()
                .output
                .duplicate(),
            pngs: o
                .fields
                .iter()
                .find(|o| o.name == "pngs")
                .unwrap()
                .output
                .duplicate(),
            tiffs: o
                .fields
                .iter()
                .find(|o| o.name == "tiffs")
                .unwrap()
                .output
                .duplicate(),
            tifs: o
                .fields
                .iter()
                .find(|o| o.name == "tifs")
                .unwrap()
                .output
                .duplicate(),
            webps: o
                .fields
                .iter()
                .find(|o| o.name == "webps")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        zone_dnssec::Res {
            algorithm: o
                .fields
                .iter()
                .find(|o| o.name == "algorithm")
                .unwrap()
                .output
                .duplicate(),
            digest: o
                .fields
                .iter()
                .find(|o| o.name == "digest")
                .unwrap()
                .output
                .duplicate(),
            digest_algorithm: o
                .fields
                .iter()
                .find(|o| o.name == "digestAlgorithm")
                .unwrap()
                .output
                .duplicate(),
            digest_type: o
                .fields
                .iter()
                .find(|o| o.name == "digestType")
                .unwrap()
                .output
                .duplicate(),
            ds: o
                .fields
                .iter()
                .find(|o| o.name == "ds")
                .unwrap()
                .output
                .duplicate(),
            flags: o
                .fields
                .iter()
                .find(|o| o.name == "flags")
                .unwrap()
                .output
                .duplicate(),
            key_tag: o
                .fields
                .iter()
                .find(|o| o.name == "keyTag")
                .unwrap()
                .output
                .duplicate(),
            key_type: o
                .fields
                .iter()
                .find(|o| o.name == "keyType")
                .unwrap()
                .output
                .duplicate(),
            modified_on: o
                .fields
                .iter()
                .find(|o| o.name == "modifiedOn")
                .unwrap()
                .output
                .duplicate(),
            public_key: o
                .fields
                .iter()
                .find(|o| o.name == "publicKey")
                .unwrap()
                .output
                .duplicate(),
            status: o
                .fields
                .iter()
                .find(|o| o.name == "status")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        zone_hold::Res {
            hold: o
                .fields
                .iter()
                .find(|o| o.name == "hold")
                .unwrap()
                .output
                .duplicate(),
            hold_after: o
                .fields
                .iter()
                .find(|o| o.name == "holdAfter")
                .unwrap()
                .output
                .duplicate(),
            include_subdomains: o
                .fields
                .iter()
                .find(|o| o.name == "includeSubdomains")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        zone_lockdown::Res {
            configurations: o
                .fields
                .iter()
                .find(|o| o.name == "configurations")
                .unwrap()
                .output
                .duplicate(),
            description: o
                .fields
                .iter()
                .find(|o| o.name == "description")
                .unwrap()
                .output
                .duplicate(),
            paused: o
                .fields
                .iter()
                .find(|o| o.name == "paused")
                .unwrap()
                .output
                .duplicate(),
            priority: o
                .fields
                .iter()
                .find(|o| o.name == "priority")
                .unwrap()
                .output
                .duplicate(),
            urls: o
                .fields
                .iter()
                .find(|o| o.name == "urls")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
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

        zone_settings_override::Res {
            initial_settings: o
                .fields
                .iter()
                .find(|o| o.name == "initialSettings")
                .unwrap()
                .output
                .duplicate(),
            initial_settings_read_at: o
                .fields
                .iter()
                .find(|o| o.name == "initialSettingsReadAt")
                .unwrap()
                .output
                .duplicate(),
            readonly_settings: o
                .fields
                .iter()
                .find(|o| o.name == "readonlySettings")
                .unwrap()
                .output
                .duplicate(),
            settings: o
                .fields
                .iter()
                .find(|o| o.name == "settings")
                .unwrap()
                .output
                .duplicate(),
            zone_id: o
                .fields
                .iter()
                .find(|o| o.name == "zoneId")
                .unwrap()
                .output
                .duplicate(),
            zone_status: o
                .fields
                .iter()
                .find(|o| o.name == "zoneStatus")
                .unwrap()
                .output
                .duplicate(),
            zone_type: o
                .fields
                .iter()
                .find(|o| o.name == "zoneType")
                .unwrap()
                .output
                .duplicate(),
        }
    }
}
