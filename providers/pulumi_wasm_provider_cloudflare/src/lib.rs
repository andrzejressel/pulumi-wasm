use crate::bindings::component::pulumi_wasm::register_interface::{
    register, ObjectField, RegisterResourceRequest, ResultField,
};
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

#[allow(clippy::all)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_unsafe)]
mod bindings;
bindings::export!(Component with_types_in bindings);

struct Component {}

impl access_application::Guest for Component {
    fn invoke(name: String, args: access_application::Args) -> access_application::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "allowAuthenticateViaWarp".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "allowedIdps".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "appLauncherLogoUrl".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "appLauncherVisible".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "aud".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "autoRedirectToIdentity".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "bgColor".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "corsHeaders".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 136, 175, 97, 108, 108, 111, 119,
                        65, 108, 108, 72, 101, 97, 100, 101, 114, 115, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 164, 66, 111, 111, 108, 175, 97, 108, 108, 111, 119, 65,
                        108, 108, 77, 101, 116, 104, 111, 100, 115, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 164, 66, 111, 111, 108, 175, 97, 108, 108, 111, 119, 65,
                        108, 108, 79, 114, 105, 103, 105, 110, 115, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 164, 66, 111, 111, 108, 176, 97, 108, 108, 111, 119, 67,
                        114, 101, 100, 101, 110, 116, 105, 97, 108, 115, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 174, 97, 108, 108, 111, 119,
                        101, 100, 72, 101, 97, 100, 101, 114, 115, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110,
                        103, 174, 97, 108, 108, 111, 119, 101, 100, 77, 101, 116, 104, 111, 100,
                        115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114,
                        97, 121, 166, 83, 116, 114, 105, 110, 103, 174, 97, 108, 108, 111, 119,
                        101, 100, 79, 114, 105, 103, 105, 110, 115, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105,
                        110, 103, 166, 109, 97, 120, 65, 103, 101, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "customDenyMessage".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "customDenyUrl".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "customNonIdentityDenyUrl".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "customPages".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "domain".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "enableBindingCookie".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "footerLinks".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 164, 110, 97, 109, 101, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                        163, 117, 114, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83,
                        116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "headerBgColor".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "httpOnlyCookieAttribute".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "landingPageDesign".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101,
                        99, 116, 133, 171, 98, 117, 116, 116, 111, 110, 67, 111, 108, 111, 114,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103, 175, 98, 117, 116, 116, 111, 110, 84, 101, 120, 116, 67, 111, 108,
                        111, 114, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114,
                        105, 110, 103, 168, 105, 109, 97, 103, 101, 85, 114, 108, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 167,
                        109, 101, 115, 115, 97, 103, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 166, 83, 116, 114, 105, 110, 103, 165, 116, 105, 116, 108, 101, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "logoUrl".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "saasApp".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101,
                        99, 116, 222, 0, 18, 174, 97, 112, 112, 76, 97, 117, 110, 99, 104, 101,
                        114, 85, 114, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83,
                        116, 114, 105, 110, 103, 168, 97, 117, 116, 104, 84, 121, 112, 101, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                        168, 99, 108, 105, 101, 110, 116, 73, 100, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 172, 99, 108, 105, 101,
                        110, 116, 83, 101, 99, 114, 101, 116, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 166, 83, 116, 114, 105, 110, 103, 178, 99, 111, 110, 115, 117,
                        109, 101, 114, 83, 101, 114, 118, 105, 99, 101, 85, 114, 108, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 176, 99,
                        117, 115, 116, 111, 109, 65, 116, 116, 114, 105, 98, 117, 116, 101, 115,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 133, 172, 102, 114, 105, 101,
                        110, 100, 108, 121, 78, 97, 109, 101, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 166, 83, 116, 114, 105, 110, 103, 164, 110, 97, 109, 101, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                        170, 110, 97, 109, 101, 70, 111, 114, 109, 97, 116, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 168, 114, 101,
                        113, 117, 105, 114, 101, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 164, 66, 111, 111, 108, 166, 115, 111, 117, 114, 99, 101, 129, 166,
                        79, 98, 106, 101, 99, 116, 129, 164, 110, 97, 109, 101, 166, 83, 116, 114,
                        105, 110, 103, 177, 100, 101, 102, 97, 117, 108, 116, 82, 101, 108, 97,
                        121, 83, 116, 97, 116, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        166, 83, 116, 114, 105, 110, 103, 170, 103, 114, 97, 110, 116, 84, 121,
                        112, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65,
                        114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 176, 103, 114, 111,
                        117, 112, 70, 105, 108, 116, 101, 114, 82, 101, 103, 101, 120, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 171,
                        105, 100, 112, 69, 110, 116, 105, 116, 121, 73, 100, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 172, 110, 97,
                        109, 101, 73, 100, 70, 111, 114, 109, 97, 116, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 182, 110, 97, 109, 101,
                        73, 100, 84, 114, 97, 110, 115, 102, 111, 114, 109, 74, 115, 111, 110, 97,
                        116, 97, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114,
                        105, 110, 103, 169, 112, 117, 98, 108, 105, 99, 75, 101, 121, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 172,
                        114, 101, 100, 105, 114, 101, 99, 116, 85, 114, 105, 115, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83,
                        116, 114, 105, 110, 103, 189, 115, 97, 109, 108, 65, 116, 116, 114, 105,
                        98, 117, 116, 101, 84, 114, 97, 110, 115, 102, 111, 114, 109, 74, 115, 111,
                        110, 97, 116, 97, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83,
                        116, 114, 105, 110, 103, 166, 115, 99, 111, 112, 101, 115, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83,
                        116, 114, 105, 110, 103, 170, 115, 112, 69, 110, 116, 105, 116, 121, 73,
                        100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105,
                        110, 103, 171, 115, 115, 111, 69, 110, 100, 112, 111, 105, 110, 116, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "sameSiteCookieAttribute".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "selfHostedDomains".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "serviceAuth401Redirect".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "sessionDuration".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "skipInterstitial".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "tags".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "type".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        access_application::Res {
            account_id: o.get_field("accountId", true),
            allow_authenticate_via_warp: o.get_field("allowAuthenticateViaWarp", false),
            allowed_idps: o.get_field("allowedIdps", false),
            app_launcher_logo_url: o.get_field("appLauncherLogoUrl", false),
            app_launcher_visible: o.get_field("appLauncherVisible", false),
            aud: o.get_field("aud", true),
            auto_redirect_to_identity: o.get_field("autoRedirectToIdentity", false),
            bg_color: o.get_field("bgColor", false),
            cors_headers: o.get_field("corsHeaders", false),
            custom_deny_message: o.get_field("customDenyMessage", false),
            custom_deny_url: o.get_field("customDenyUrl", false),
            custom_non_identity_deny_url: o.get_field("customNonIdentityDenyUrl", false),
            custom_pages: o.get_field("customPages", false),
            domain: o.get_field("domain", true),
            enable_binding_cookie: o.get_field("enableBindingCookie", false),
            footer_links: o.get_field("footerLinks", false),
            header_bg_color: o.get_field("headerBgColor", false),
            http_only_cookie_attribute: o.get_field("httpOnlyCookieAttribute", false),
            landing_page_design: o.get_field("landingPageDesign", false),
            logo_url: o.get_field("logoUrl", false),
            name: o.get_field("name", true),
            saas_app: o.get_field("saasApp", false),
            same_site_cookie_attribute: o.get_field("sameSiteCookieAttribute", false),
            self_hosted_domains: o.get_field("selfHostedDomains", false),
            service_auth401_redirect: o.get_field("serviceAuth401Redirect", false),
            session_duration: o.get_field("sessionDuration", false),
            skip_interstitial: o.get_field("skipInterstitial", false),
            tags: o.get_field("tags", false),
            type_: o.get_field("type", false),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl access_ca_certificate::Guest for Component {
    fn invoke(name: String, args: access_ca_certificate::Args) -> access_ca_certificate::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "applicationId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "aud".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "publicKey".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        access_ca_certificate::Res {
            account_id: o.get_field("accountId", true),
            application_id: o.get_field("applicationId", true),
            aud: o.get_field("aud", true),
            public_key: o.get_field("publicKey", true),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl access_custom_page::Guest for Component {
    fn invoke(name: String, args: access_custom_page::Args) -> access_custom_page::Res {
        wasm_common::setup_logger();
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
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "appCount".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "customHtml".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "type".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
            ],
        };

        let o = register(&request);

        access_custom_page::Res {
            account_id: o.get_field("accountId", false),
            app_count: o.get_field("appCount", false),
            custom_html: o.get_field("customHtml", false),
            name: o.get_field("name", true),
            type_: o.get_field("type", true),
            zone_id: o.get_field("zoneId", false),
        }
    }
}
impl access_group::Guest for Component {
    fn invoke(name: String, args: access_group::Args) -> access_group::Res {
        wasm_common::setup_logger();
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
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "excludes".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 222, 0, 21, 180, 97, 110, 121,
                        86, 97, 108, 105, 100, 83, 101, 114, 118, 105, 99, 101, 84, 111, 107, 101,
                        110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                        172, 97, 117, 116, 104, 67, 111, 110, 116, 101, 120, 116, 115, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129,
                        166, 79, 98, 106, 101, 99, 116, 131, 164, 97, 99, 73, 100, 166, 83, 116,
                        114, 105, 110, 103, 162, 105, 100, 166, 83, 116, 114, 105, 110, 103, 178,
                        105, 100, 101, 110, 116, 105, 116, 121, 80, 114, 111, 118, 105, 100, 101,
                        114, 73, 100, 166, 83, 116, 114, 105, 110, 103, 170, 97, 117, 116, 104, 77,
                        101, 116, 104, 111, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        166, 83, 116, 114, 105, 110, 103, 166, 97, 122, 117, 114, 101, 115, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121,
                        129, 166, 79, 98, 106, 101, 99, 116, 130, 178, 105, 100, 101, 110, 116,
                        105, 116, 121, 80, 114, 111, 118, 105, 100, 101, 114, 73, 100, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 163,
                        105, 100, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65,
                        114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 171, 99, 101, 114,
                        116, 105, 102, 105, 99, 97, 116, 101, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 164, 66, 111, 111, 108, 170, 99, 111, 109, 109, 111, 110, 78, 97,
                        109, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114,
                        105, 110, 103, 174, 100, 101, 118, 105, 99, 101, 80, 111, 115, 116, 117,
                        114, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65,
                        114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 172, 101, 109, 97,
                        105, 108, 68, 111, 109, 97, 105, 110, 115, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110,
                        103, 166, 101, 109, 97, 105, 108, 115, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110,
                        103, 168, 101, 118, 101, 114, 121, 111, 110, 101, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 178, 101, 120, 116, 101,
                        114, 110, 97, 108, 69, 118, 97, 108, 117, 97, 116, 105, 111, 110, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116,
                        130, 171, 101, 118, 97, 108, 117, 97, 116, 101, 85, 114, 108, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 167,
                        107, 101, 121, 115, 85, 114, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 166, 83, 116, 114, 105, 110, 103, 164, 103, 101, 111, 115, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166,
                        83, 116, 114, 105, 110, 103, 167, 103, 105, 116, 104, 117, 98, 115, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121,
                        129, 166, 79, 98, 106, 101, 99, 116, 131, 178, 105, 100, 101, 110, 116,
                        105, 116, 121, 80, 114, 111, 118, 105, 100, 101, 114, 73, 100, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 164,
                        110, 97, 109, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83,
                        116, 114, 105, 110, 103, 165, 116, 101, 97, 109, 115, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116,
                        114, 105, 110, 103, 166, 103, 114, 111, 117, 112, 115, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116,
                        114, 105, 110, 103, 167, 103, 115, 117, 105, 116, 101, 115, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166,
                        79, 98, 106, 101, 99, 116, 130, 166, 101, 109, 97, 105, 108, 115, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166,
                        83, 116, 114, 105, 110, 103, 178, 105, 100, 101, 110, 116, 105, 116, 121,
                        80, 114, 111, 118, 105, 100, 101, 114, 73, 100, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 167, 105, 112, 76,
                        105, 115, 116, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129,
                        165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 163, 105,
                        112, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114,
                        114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 172, 108, 111, 103, 105,
                        110, 77, 101, 116, 104, 111, 100, 115, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110,
                        103, 165, 111, 107, 116, 97, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116,
                        130, 178, 105, 100, 101, 110, 116, 105, 116, 121, 80, 114, 111, 118, 105,
                        100, 101, 114, 73, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166,
                        83, 116, 114, 105, 110, 103, 165, 110, 97, 109, 101, 115, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83,
                        116, 114, 105, 110, 103, 165, 115, 97, 109, 108, 115, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79,
                        98, 106, 101, 99, 116, 131, 173, 97, 116, 116, 114, 105, 98, 117, 116, 101,
                        78, 97, 109, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83,
                        116, 114, 105, 110, 103, 174, 97, 116, 116, 114, 105, 98, 117, 116, 101,
                        86, 97, 108, 117, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166,
                        83, 116, 114, 105, 110, 103, 178, 105, 100, 101, 110, 116, 105, 116, 121,
                        80, 114, 111, 118, 105, 100, 101, 114, 73, 100, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 173, 115, 101,
                        114, 118, 105, 99, 101, 84, 111, 107, 101, 110, 115, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116,
                        114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "includes".into(),
                    schema: vec![
                        129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 222,
                        0, 21, 180, 97, 110, 121, 86, 97, 108, 105, 100, 83, 101, 114, 118, 105,
                        99, 101, 84, 111, 107, 101, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 164, 66, 111, 111, 108, 172, 97, 117, 116, 104, 67, 111, 110, 116,
                        101, 120, 116, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129,
                        165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 131, 164,
                        97, 99, 73, 100, 166, 83, 116, 114, 105, 110, 103, 162, 105, 100, 166, 83,
                        116, 114, 105, 110, 103, 178, 105, 100, 101, 110, 116, 105, 116, 121, 80,
                        114, 111, 118, 105, 100, 101, 114, 73, 100, 166, 83, 116, 114, 105, 110,
                        103, 170, 97, 117, 116, 104, 77, 101, 116, 104, 111, 100, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 166, 97,
                        122, 117, 114, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 130,
                        178, 105, 100, 101, 110, 116, 105, 116, 121, 80, 114, 111, 118, 105, 100,
                        101, 114, 73, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83,
                        116, 114, 105, 110, 103, 163, 105, 100, 115, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105,
                        110, 103, 171, 99, 101, 114, 116, 105, 102, 105, 99, 97, 116, 101, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 170, 99,
                        111, 109, 109, 111, 110, 78, 97, 109, 101, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 174, 100, 101, 118, 105,
                        99, 101, 80, 111, 115, 116, 117, 114, 101, 115, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114,
                        105, 110, 103, 172, 101, 109, 97, 105, 108, 68, 111, 109, 97, 105, 110,
                        115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114,
                        97, 121, 166, 83, 116, 114, 105, 110, 103, 166, 101, 109, 97, 105, 108,
                        115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114,
                        97, 121, 166, 83, 116, 114, 105, 110, 103, 168, 101, 118, 101, 114, 121,
                        111, 110, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111,
                        111, 108, 178, 101, 120, 116, 101, 114, 110, 97, 108, 69, 118, 97, 108,
                        117, 97, 116, 105, 111, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        129, 166, 79, 98, 106, 101, 99, 116, 130, 171, 101, 118, 97, 108, 117, 97,
                        116, 101, 85, 114, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166,
                        83, 116, 114, 105, 110, 103, 167, 107, 101, 121, 115, 85, 114, 108, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                        164, 103, 101, 111, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 167,
                        103, 105, 116, 104, 117, 98, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116,
                        131, 178, 105, 100, 101, 110, 116, 105, 116, 121, 80, 114, 111, 118, 105,
                        100, 101, 114, 73, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166,
                        83, 116, 114, 105, 110, 103, 164, 110, 97, 109, 101, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 165, 116,
                        101, 97, 109, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165,
                        65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 166, 103, 114,
                        111, 117, 112, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129,
                        165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 167, 103,
                        115, 117, 105, 116, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116,
                        130, 166, 101, 109, 97, 105, 108, 115, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110,
                        103, 178, 105, 100, 101, 110, 116, 105, 116, 121, 80, 114, 111, 118, 105,
                        100, 101, 114, 73, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166,
                        83, 116, 114, 105, 110, 103, 167, 105, 112, 76, 105, 115, 116, 115, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121,
                        166, 83, 116, 114, 105, 110, 103, 163, 105, 112, 115, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116,
                        114, 105, 110, 103, 172, 108, 111, 103, 105, 110, 77, 101, 116, 104, 111,
                        100, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114,
                        114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 165, 111, 107, 116, 97,
                        115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114,
                        97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 178, 105, 100, 101, 110,
                        116, 105, 116, 121, 80, 114, 111, 118, 105, 100, 101, 114, 73, 100, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                        165, 110, 97, 109, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 165,
                        115, 97, 109, 108, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129,
                        165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 131, 173,
                        97, 116, 116, 114, 105, 98, 117, 116, 101, 78, 97, 109, 101, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 174, 97,
                        116, 116, 114, 105, 98, 117, 116, 101, 86, 97, 108, 117, 101, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 178,
                        105, 100, 101, 110, 116, 105, 116, 121, 80, 114, 111, 118, 105, 100, 101,
                        114, 73, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116,
                        114, 105, 110, 103, 173, 115, 101, 114, 118, 105, 99, 101, 84, 111, 107,
                        101, 110, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65,
                        114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "requires".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 222, 0, 21, 180, 97, 110, 121,
                        86, 97, 108, 105, 100, 83, 101, 114, 118, 105, 99, 101, 84, 111, 107, 101,
                        110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                        172, 97, 117, 116, 104, 67, 111, 110, 116, 101, 120, 116, 115, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129,
                        166, 79, 98, 106, 101, 99, 116, 131, 164, 97, 99, 73, 100, 166, 83, 116,
                        114, 105, 110, 103, 162, 105, 100, 166, 83, 116, 114, 105, 110, 103, 178,
                        105, 100, 101, 110, 116, 105, 116, 121, 80, 114, 111, 118, 105, 100, 101,
                        114, 73, 100, 166, 83, 116, 114, 105, 110, 103, 170, 97, 117, 116, 104, 77,
                        101, 116, 104, 111, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        166, 83, 116, 114, 105, 110, 103, 166, 97, 122, 117, 114, 101, 115, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121,
                        129, 166, 79, 98, 106, 101, 99, 116, 130, 178, 105, 100, 101, 110, 116,
                        105, 116, 121, 80, 114, 111, 118, 105, 100, 101, 114, 73, 100, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 163,
                        105, 100, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65,
                        114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 171, 99, 101, 114,
                        116, 105, 102, 105, 99, 97, 116, 101, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 164, 66, 111, 111, 108, 170, 99, 111, 109, 109, 111, 110, 78, 97,
                        109, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114,
                        105, 110, 103, 174, 100, 101, 118, 105, 99, 101, 80, 111, 115, 116, 117,
                        114, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65,
                        114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 172, 101, 109, 97,
                        105, 108, 68, 111, 109, 97, 105, 110, 115, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110,
                        103, 166, 101, 109, 97, 105, 108, 115, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110,
                        103, 168, 101, 118, 101, 114, 121, 111, 110, 101, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 178, 101, 120, 116, 101,
                        114, 110, 97, 108, 69, 118, 97, 108, 117, 97, 116, 105, 111, 110, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116,
                        130, 171, 101, 118, 97, 108, 117, 97, 116, 101, 85, 114, 108, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 167,
                        107, 101, 121, 115, 85, 114, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 166, 83, 116, 114, 105, 110, 103, 164, 103, 101, 111, 115, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166,
                        83, 116, 114, 105, 110, 103, 167, 103, 105, 116, 104, 117, 98, 115, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121,
                        129, 166, 79, 98, 106, 101, 99, 116, 131, 178, 105, 100, 101, 110, 116,
                        105, 116, 121, 80, 114, 111, 118, 105, 100, 101, 114, 73, 100, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 164,
                        110, 97, 109, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83,
                        116, 114, 105, 110, 103, 165, 116, 101, 97, 109, 115, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116,
                        114, 105, 110, 103, 166, 103, 114, 111, 117, 112, 115, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116,
                        114, 105, 110, 103, 167, 103, 115, 117, 105, 116, 101, 115, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166,
                        79, 98, 106, 101, 99, 116, 130, 166, 101, 109, 97, 105, 108, 115, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166,
                        83, 116, 114, 105, 110, 103, 178, 105, 100, 101, 110, 116, 105, 116, 121,
                        80, 114, 111, 118, 105, 100, 101, 114, 73, 100, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 167, 105, 112, 76,
                        105, 115, 116, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129,
                        165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 163, 105,
                        112, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114,
                        114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 172, 108, 111, 103, 105,
                        110, 77, 101, 116, 104, 111, 100, 115, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110,
                        103, 165, 111, 107, 116, 97, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116,
                        130, 178, 105, 100, 101, 110, 116, 105, 116, 121, 80, 114, 111, 118, 105,
                        100, 101, 114, 73, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166,
                        83, 116, 114, 105, 110, 103, 165, 110, 97, 109, 101, 115, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83,
                        116, 114, 105, 110, 103, 165, 115, 97, 109, 108, 115, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79,
                        98, 106, 101, 99, 116, 131, 173, 97, 116, 116, 114, 105, 98, 117, 116, 101,
                        78, 97, 109, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83,
                        116, 114, 105, 110, 103, 174, 97, 116, 116, 114, 105, 98, 117, 116, 101,
                        86, 97, 108, 117, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166,
                        83, 116, 114, 105, 110, 103, 178, 105, 100, 101, 110, 116, 105, 116, 121,
                        80, 114, 111, 118, 105, 100, 101, 114, 73, 100, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 173, 115, 101,
                        114, 118, 105, 99, 101, 84, 111, 107, 101, 110, 115, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116,
                        114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        access_group::Res {
            account_id: o.get_field("accountId", false),
            excludes: o.get_field("excludes", false),
            includes: o.get_field("includes", true),
            name: o.get_field("name", true),
            requires: o.get_field("requires", false),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl access_identity_provider::Guest for Component {
    fn invoke(name: String, args: access_identity_provider::Args) -> access_identity_provider::Res {
        wasm_common::setup_logger();
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
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "configs".into(),
                    schema: vec![
                        129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 222,
                        0, 27, 168, 97, 112, 105, 84, 111, 107, 101, 110, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 170, 97, 112, 112,
                        115, 68, 111, 109, 97, 105, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 166, 83, 116, 114, 105, 110, 103, 170, 97, 116, 116, 114, 105, 98,
                        117, 116, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129,
                        165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 167, 97, 117,
                        116, 104, 85, 114, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166,
                        83, 116, 114, 105, 110, 103, 181, 97, 117, 116, 104, 111, 114, 105, 122,
                        97, 116, 105, 111, 110, 83, 101, 114, 118, 101, 114, 73, 100, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 175, 99,
                        101, 110, 116, 114, 105, 102, 121, 65, 99, 99, 111, 117, 110, 116, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                        173, 99, 101, 110, 116, 114, 105, 102, 121, 65, 112, 112, 73, 100, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                        168, 99, 101, 114, 116, 115, 85, 114, 108, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 166, 99, 108, 97, 105, 109,
                        115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114,
                        97, 121, 166, 83, 116, 114, 105, 110, 103, 168, 99, 108, 105, 101, 110,
                        116, 73, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116,
                        114, 105, 110, 103, 172, 99, 108, 105, 101, 110, 116, 83, 101, 99, 114,
                        101, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114,
                        105, 110, 103, 184, 99, 111, 110, 100, 105, 116, 105, 111, 110, 97, 108,
                        65, 99, 99, 101, 115, 115, 69, 110, 97, 98, 108, 101, 100, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 171, 100, 105,
                        114, 101, 99, 116, 111, 114, 121, 73, 100, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 178, 101, 109, 97, 105,
                        108, 65, 116, 116, 114, 105, 98, 117, 116, 101, 78, 97, 109, 101, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 174,
                        101, 109, 97, 105, 108, 67, 108, 97, 105, 109, 78, 97, 109, 101, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 173,
                        105, 100, 112, 80, 117, 98, 108, 105, 99, 67, 101, 114, 116, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 169,
                        105, 115, 115, 117, 101, 114, 85, 114, 108, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 171, 111, 107, 116, 97,
                        65, 99, 99, 111, 117, 110, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 166, 83, 116, 114, 105, 110, 103, 175, 111, 110, 101, 108, 111, 103,
                        105, 110, 65, 99, 99, 111, 117, 110, 116, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 169, 112, 105, 110, 103,
                        69, 110, 118, 73, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166,
                        83, 116, 114, 105, 110, 103, 171, 112, 107, 99, 101, 69, 110, 97, 98, 108,
                        101, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111,
                        108, 171, 114, 101, 100, 105, 114, 101, 99, 116, 85, 114, 108, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 166,
                        115, 99, 111, 112, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 171,
                        115, 105, 103, 110, 82, 101, 113, 117, 101, 115, 116, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 172, 115, 115, 111, 84,
                        97, 114, 103, 101, 116, 85, 114, 108, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 166, 83, 116, 114, 105, 110, 103, 173, 115, 117, 112, 112, 111,
                        114, 116, 71, 114, 111, 117, 112, 115, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 164, 66, 111, 111, 108, 168, 116, 111, 107, 101, 110, 85, 114,
                        108, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105,
                        110, 103,
                    ],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "scimConfigs".into(),
                    schema: vec![
                        129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 133,
                        167, 101, 110, 97, 98, 108, 101, 100, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 164, 66, 111, 111, 108, 182, 103, 114, 111, 117, 112, 77, 101,
                        109, 98, 101, 114, 68, 101, 112, 114, 111, 118, 105, 115, 105, 111, 110,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 175,
                        115, 101, 97, 116, 68, 101, 112, 114, 111, 118, 105, 115, 105, 111, 110,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 166,
                        115, 101, 99, 114, 101, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        166, 83, 116, 114, 105, 110, 103, 175, 117, 115, 101, 114, 68, 101, 112,
                        114, 111, 118, 105, 115, 105, 111, 110, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "type".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
            ],
        };

        let o = register(&request);

        access_identity_provider::Res {
            account_id: o.get_field("accountId", false),
            configs: o.get_field("configs", true),
            name: o.get_field("name", true),
            scim_configs: o.get_field("scimConfigs", true),
            type_: o.get_field("type", true),
            zone_id: o.get_field("zoneId", false),
        }
    }
}
impl access_keys_configuration::Guest for Component {
    fn invoke(
        name: String,
        args: access_keys_configuration::Args,
    ) -> access_keys_configuration::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "keyRotationIntervalDays".into(),
                    schema: vec![163, 73, 110, 116],
                },
            ],
        };

        let o = register(&request);

        access_keys_configuration::Res {
            account_id: o.get_field("accountId", true),
            key_rotation_interval_days: o.get_field("keyRotationIntervalDays", true),
        }
    }
}
impl access_mutual_tls_certificate::Guest for Component {
    fn invoke(
        name: String,
        args: access_mutual_tls_certificate::Args,
    ) -> access_mutual_tls_certificate::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "associatedHostnames".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "certificate".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "fingerprint".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        access_mutual_tls_certificate::Res {
            account_id: o.get_field("accountId", true),
            associated_hostnames: o.get_field("associatedHostnames", false),
            certificate: o.get_field("certificate", false),
            fingerprint: o.get_field("fingerprint", true),
            name: o.get_field("name", true),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl access_mutual_tls_hostname_settings::Guest for Component {
    fn invoke(
        name: String,
        args: access_mutual_tls_hostname_settings::Args,
    ) -> access_mutual_tls_hostname_settings::Res {
        wasm_common::setup_logger();
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
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "settings".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 131, 172, 99, 104, 105, 110, 97,
                        78, 101, 116, 119, 111, 114, 107, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 164, 66, 111, 111, 108, 187, 99, 108, 105, 101, 110, 116, 67, 101,
                        114, 116, 105, 102, 105, 99, 97, 116, 101, 70, 111, 114, 119, 97, 114, 100,
                        105, 110, 103, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111,
                        111, 108, 168, 104, 111, 115, 116, 110, 97, 109, 101, 166, 83, 116, 114,
                        105, 110, 103,
                    ],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
            ],
        };

        let o = register(&request);

        access_mutual_tls_hostname_settings::Res {
            account_id: o.get_field("accountId", false),
            settings: o.get_field("settings", false),
            zone_id: o.get_field("zoneId", false),
        }
    }
}
impl access_organization::Guest for Component {
    fn invoke(name: String, args: access_organization::Args) -> access_organization::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "allowAuthenticateViaWarp".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "authDomain".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "autoRedirectToIdentity".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "customPages".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 169, 102, 111, 114, 98, 105,
                        100, 100, 101, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83,
                        116, 114, 105, 110, 103, 174, 105, 100, 101, 110, 116, 105, 116, 121, 68,
                        101, 110, 105, 101, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "isUiReadOnly".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "loginDesigns".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 133, 175, 98, 97, 99, 107, 103,
                        114, 111, 117, 110, 100, 67, 111, 108, 111, 114, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 170, 102, 111,
                        111, 116, 101, 114, 84, 101, 120, 116, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 166, 83, 116, 114, 105, 110, 103, 170, 104, 101, 97, 100, 101,
                        114, 84, 101, 120, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166,
                        83, 116, 114, 105, 110, 103, 168, 108, 111, 103, 111, 80, 97, 116, 104,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103, 169, 116, 101, 120, 116, 67, 111, 108, 111, 114, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "sessionDuration".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "uiReadOnlyToggleReason".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "userSeatExpirationInactiveTime".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "warpAuthSessionDuration".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        access_organization::Res {
            account_id: o.get_field("accountId", true),
            allow_authenticate_via_warp: o.get_field("allowAuthenticateViaWarp", false),
            auth_domain: o.get_field("authDomain", true),
            auto_redirect_to_identity: o.get_field("autoRedirectToIdentity", false),
            custom_pages: o.get_field("customPages", false),
            is_ui_read_only: o.get_field("isUiReadOnly", false),
            login_designs: o.get_field("loginDesigns", false),
            name: o.get_field("name", false),
            session_duration: o.get_field("sessionDuration", false),
            ui_read_only_toggle_reason: o.get_field("uiReadOnlyToggleReason", false),
            user_seat_expiration_inactive_time: o
                .get_field("userSeatExpirationInactiveTime", false),
            warp_auth_session_duration: o.get_field("warpAuthSessionDuration", false),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl access_policy::Guest for Component {
    fn invoke(name: String, args: access_policy::Args) -> access_policy::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "applicationId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "approvalGroups".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 131, 175, 97, 112, 112, 114, 111,
                        118, 97, 108, 115, 78, 101, 101, 100, 101, 100, 163, 73, 110, 116, 174,
                        101, 109, 97, 105, 108, 65, 100, 100, 114, 101, 115, 115, 101, 115, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121,
                        166, 83, 116, 114, 105, 110, 103, 173, 101, 109, 97, 105, 108, 76, 105,
                        115, 116, 85, 117, 105, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "approvalRequired".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "decision".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "excludes".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 222, 0, 21, 180, 97, 110, 121,
                        86, 97, 108, 105, 100, 83, 101, 114, 118, 105, 99, 101, 84, 111, 107, 101,
                        110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                        172, 97, 117, 116, 104, 67, 111, 110, 116, 101, 120, 116, 115, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129,
                        166, 79, 98, 106, 101, 99, 116, 131, 164, 97, 99, 73, 100, 166, 83, 116,
                        114, 105, 110, 103, 162, 105, 100, 166, 83, 116, 114, 105, 110, 103, 178,
                        105, 100, 101, 110, 116, 105, 116, 121, 80, 114, 111, 118, 105, 100, 101,
                        114, 73, 100, 166, 83, 116, 114, 105, 110, 103, 170, 97, 117, 116, 104, 77,
                        101, 116, 104, 111, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        166, 83, 116, 114, 105, 110, 103, 166, 97, 122, 117, 114, 101, 115, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121,
                        129, 166, 79, 98, 106, 101, 99, 116, 130, 178, 105, 100, 101, 110, 116,
                        105, 116, 121, 80, 114, 111, 118, 105, 100, 101, 114, 73, 100, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 163,
                        105, 100, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65,
                        114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 171, 99, 101, 114,
                        116, 105, 102, 105, 99, 97, 116, 101, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 164, 66, 111, 111, 108, 170, 99, 111, 109, 109, 111, 110, 78, 97,
                        109, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114,
                        105, 110, 103, 174, 100, 101, 118, 105, 99, 101, 80, 111, 115, 116, 117,
                        114, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65,
                        114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 172, 101, 109, 97,
                        105, 108, 68, 111, 109, 97, 105, 110, 115, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110,
                        103, 166, 101, 109, 97, 105, 108, 115, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110,
                        103, 168, 101, 118, 101, 114, 121, 111, 110, 101, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 178, 101, 120, 116, 101,
                        114, 110, 97, 108, 69, 118, 97, 108, 117, 97, 116, 105, 111, 110, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116,
                        130, 171, 101, 118, 97, 108, 117, 97, 116, 101, 85, 114, 108, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 167,
                        107, 101, 121, 115, 85, 114, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 166, 83, 116, 114, 105, 110, 103, 164, 103, 101, 111, 115, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166,
                        83, 116, 114, 105, 110, 103, 167, 103, 105, 116, 104, 117, 98, 115, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121,
                        129, 166, 79, 98, 106, 101, 99, 116, 131, 178, 105, 100, 101, 110, 116,
                        105, 116, 121, 80, 114, 111, 118, 105, 100, 101, 114, 73, 100, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 164,
                        110, 97, 109, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83,
                        116, 114, 105, 110, 103, 165, 116, 101, 97, 109, 115, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116,
                        114, 105, 110, 103, 166, 103, 114, 111, 117, 112, 115, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116,
                        114, 105, 110, 103, 167, 103, 115, 117, 105, 116, 101, 115, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166,
                        79, 98, 106, 101, 99, 116, 130, 166, 101, 109, 97, 105, 108, 115, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166,
                        83, 116, 114, 105, 110, 103, 178, 105, 100, 101, 110, 116, 105, 116, 121,
                        80, 114, 111, 118, 105, 100, 101, 114, 73, 100, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 167, 105, 112, 76,
                        105, 115, 116, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129,
                        165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 163, 105,
                        112, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114,
                        114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 172, 108, 111, 103, 105,
                        110, 77, 101, 116, 104, 111, 100, 115, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110,
                        103, 165, 111, 107, 116, 97, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116,
                        130, 178, 105, 100, 101, 110, 116, 105, 116, 121, 80, 114, 111, 118, 105,
                        100, 101, 114, 73, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166,
                        83, 116, 114, 105, 110, 103, 165, 110, 97, 109, 101, 115, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83,
                        116, 114, 105, 110, 103, 165, 115, 97, 109, 108, 115, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79,
                        98, 106, 101, 99, 116, 131, 173, 97, 116, 116, 114, 105, 98, 117, 116, 101,
                        78, 97, 109, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83,
                        116, 114, 105, 110, 103, 174, 97, 116, 116, 114, 105, 98, 117, 116, 101,
                        86, 97, 108, 117, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166,
                        83, 116, 114, 105, 110, 103, 178, 105, 100, 101, 110, 116, 105, 116, 121,
                        80, 114, 111, 118, 105, 100, 101, 114, 73, 100, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 173, 115, 101,
                        114, 118, 105, 99, 101, 84, 111, 107, 101, 110, 115, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116,
                        114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "includes".into(),
                    schema: vec![
                        129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 222,
                        0, 21, 180, 97, 110, 121, 86, 97, 108, 105, 100, 83, 101, 114, 118, 105,
                        99, 101, 84, 111, 107, 101, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 164, 66, 111, 111, 108, 172, 97, 117, 116, 104, 67, 111, 110, 116,
                        101, 120, 116, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129,
                        165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 131, 164,
                        97, 99, 73, 100, 166, 83, 116, 114, 105, 110, 103, 162, 105, 100, 166, 83,
                        116, 114, 105, 110, 103, 178, 105, 100, 101, 110, 116, 105, 116, 121, 80,
                        114, 111, 118, 105, 100, 101, 114, 73, 100, 166, 83, 116, 114, 105, 110,
                        103, 170, 97, 117, 116, 104, 77, 101, 116, 104, 111, 100, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 166, 97,
                        122, 117, 114, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 130,
                        178, 105, 100, 101, 110, 116, 105, 116, 121, 80, 114, 111, 118, 105, 100,
                        101, 114, 73, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83,
                        116, 114, 105, 110, 103, 163, 105, 100, 115, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105,
                        110, 103, 171, 99, 101, 114, 116, 105, 102, 105, 99, 97, 116, 101, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 170, 99,
                        111, 109, 109, 111, 110, 78, 97, 109, 101, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 174, 100, 101, 118, 105,
                        99, 101, 80, 111, 115, 116, 117, 114, 101, 115, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114,
                        105, 110, 103, 172, 101, 109, 97, 105, 108, 68, 111, 109, 97, 105, 110,
                        115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114,
                        97, 121, 166, 83, 116, 114, 105, 110, 103, 166, 101, 109, 97, 105, 108,
                        115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114,
                        97, 121, 166, 83, 116, 114, 105, 110, 103, 168, 101, 118, 101, 114, 121,
                        111, 110, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111,
                        111, 108, 178, 101, 120, 116, 101, 114, 110, 97, 108, 69, 118, 97, 108,
                        117, 97, 116, 105, 111, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        129, 166, 79, 98, 106, 101, 99, 116, 130, 171, 101, 118, 97, 108, 117, 97,
                        116, 101, 85, 114, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166,
                        83, 116, 114, 105, 110, 103, 167, 107, 101, 121, 115, 85, 114, 108, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                        164, 103, 101, 111, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 167,
                        103, 105, 116, 104, 117, 98, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116,
                        131, 178, 105, 100, 101, 110, 116, 105, 116, 121, 80, 114, 111, 118, 105,
                        100, 101, 114, 73, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166,
                        83, 116, 114, 105, 110, 103, 164, 110, 97, 109, 101, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 165, 116,
                        101, 97, 109, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165,
                        65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 166, 103, 114,
                        111, 117, 112, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129,
                        165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 167, 103,
                        115, 117, 105, 116, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116,
                        130, 166, 101, 109, 97, 105, 108, 115, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110,
                        103, 178, 105, 100, 101, 110, 116, 105, 116, 121, 80, 114, 111, 118, 105,
                        100, 101, 114, 73, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166,
                        83, 116, 114, 105, 110, 103, 167, 105, 112, 76, 105, 115, 116, 115, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121,
                        166, 83, 116, 114, 105, 110, 103, 163, 105, 112, 115, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116,
                        114, 105, 110, 103, 172, 108, 111, 103, 105, 110, 77, 101, 116, 104, 111,
                        100, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114,
                        114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 165, 111, 107, 116, 97,
                        115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114,
                        97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 178, 105, 100, 101, 110,
                        116, 105, 116, 121, 80, 114, 111, 118, 105, 100, 101, 114, 73, 100, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                        165, 110, 97, 109, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 165,
                        115, 97, 109, 108, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129,
                        165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 131, 173,
                        97, 116, 116, 114, 105, 98, 117, 116, 101, 78, 97, 109, 101, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 174, 97,
                        116, 116, 114, 105, 98, 117, 116, 101, 86, 97, 108, 117, 101, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 178,
                        105, 100, 101, 110, 116, 105, 116, 121, 80, 114, 111, 118, 105, 100, 101,
                        114, 73, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116,
                        114, 105, 110, 103, 173, 115, 101, 114, 118, 105, 99, 101, 84, 111, 107,
                        101, 110, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65,
                        114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "isolationRequired".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "precedence".into(),
                    schema: vec![163, 73, 110, 116],
                },
                ResultField {
                    name: "purposeJustificationPrompt".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "purposeJustificationRequired".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "requires".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 222, 0, 21, 180, 97, 110, 121,
                        86, 97, 108, 105, 100, 83, 101, 114, 118, 105, 99, 101, 84, 111, 107, 101,
                        110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                        172, 97, 117, 116, 104, 67, 111, 110, 116, 101, 120, 116, 115, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129,
                        166, 79, 98, 106, 101, 99, 116, 131, 164, 97, 99, 73, 100, 166, 83, 116,
                        114, 105, 110, 103, 162, 105, 100, 166, 83, 116, 114, 105, 110, 103, 178,
                        105, 100, 101, 110, 116, 105, 116, 121, 80, 114, 111, 118, 105, 100, 101,
                        114, 73, 100, 166, 83, 116, 114, 105, 110, 103, 170, 97, 117, 116, 104, 77,
                        101, 116, 104, 111, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        166, 83, 116, 114, 105, 110, 103, 166, 97, 122, 117, 114, 101, 115, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121,
                        129, 166, 79, 98, 106, 101, 99, 116, 130, 178, 105, 100, 101, 110, 116,
                        105, 116, 121, 80, 114, 111, 118, 105, 100, 101, 114, 73, 100, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 163,
                        105, 100, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65,
                        114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 171, 99, 101, 114,
                        116, 105, 102, 105, 99, 97, 116, 101, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 164, 66, 111, 111, 108, 170, 99, 111, 109, 109, 111, 110, 78, 97,
                        109, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114,
                        105, 110, 103, 174, 100, 101, 118, 105, 99, 101, 80, 111, 115, 116, 117,
                        114, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65,
                        114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 172, 101, 109, 97,
                        105, 108, 68, 111, 109, 97, 105, 110, 115, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110,
                        103, 166, 101, 109, 97, 105, 108, 115, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110,
                        103, 168, 101, 118, 101, 114, 121, 111, 110, 101, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 178, 101, 120, 116, 101,
                        114, 110, 97, 108, 69, 118, 97, 108, 117, 97, 116, 105, 111, 110, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116,
                        130, 171, 101, 118, 97, 108, 117, 97, 116, 101, 85, 114, 108, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 167,
                        107, 101, 121, 115, 85, 114, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 166, 83, 116, 114, 105, 110, 103, 164, 103, 101, 111, 115, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166,
                        83, 116, 114, 105, 110, 103, 167, 103, 105, 116, 104, 117, 98, 115, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121,
                        129, 166, 79, 98, 106, 101, 99, 116, 131, 178, 105, 100, 101, 110, 116,
                        105, 116, 121, 80, 114, 111, 118, 105, 100, 101, 114, 73, 100, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 164,
                        110, 97, 109, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83,
                        116, 114, 105, 110, 103, 165, 116, 101, 97, 109, 115, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116,
                        114, 105, 110, 103, 166, 103, 114, 111, 117, 112, 115, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116,
                        114, 105, 110, 103, 167, 103, 115, 117, 105, 116, 101, 115, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166,
                        79, 98, 106, 101, 99, 116, 130, 166, 101, 109, 97, 105, 108, 115, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166,
                        83, 116, 114, 105, 110, 103, 178, 105, 100, 101, 110, 116, 105, 116, 121,
                        80, 114, 111, 118, 105, 100, 101, 114, 73, 100, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 167, 105, 112, 76,
                        105, 115, 116, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129,
                        165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 163, 105,
                        112, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114,
                        114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 172, 108, 111, 103, 105,
                        110, 77, 101, 116, 104, 111, 100, 115, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110,
                        103, 165, 111, 107, 116, 97, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116,
                        130, 178, 105, 100, 101, 110, 116, 105, 116, 121, 80, 114, 111, 118, 105,
                        100, 101, 114, 73, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166,
                        83, 116, 114, 105, 110, 103, 165, 110, 97, 109, 101, 115, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83,
                        116, 114, 105, 110, 103, 165, 115, 97, 109, 108, 115, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79,
                        98, 106, 101, 99, 116, 131, 173, 97, 116, 116, 114, 105, 98, 117, 116, 101,
                        78, 97, 109, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83,
                        116, 114, 105, 110, 103, 174, 97, 116, 116, 114, 105, 98, 117, 116, 101,
                        86, 97, 108, 117, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166,
                        83, 116, 114, 105, 110, 103, 178, 105, 100, 101, 110, 116, 105, 116, 121,
                        80, 114, 111, 118, 105, 100, 101, 114, 73, 100, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 173, 115, 101,
                        114, 118, 105, 99, 101, 84, 111, 107, 101, 110, 115, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116,
                        114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "sessionDuration".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        access_policy::Res {
            account_id: o.get_field("accountId", true),
            application_id: o.get_field("applicationId", true),
            approval_groups: o.get_field("approvalGroups", false),
            approval_required: o.get_field("approvalRequired", false),
            decision: o.get_field("decision", true),
            excludes: o.get_field("excludes", false),
            includes: o.get_field("includes", true),
            isolation_required: o.get_field("isolationRequired", false),
            name: o.get_field("name", true),
            precedence: o.get_field("precedence", true),
            purpose_justification_prompt: o.get_field("purposeJustificationPrompt", false),
            purpose_justification_required: o.get_field("purposeJustificationRequired", false),
            requires: o.get_field("requires", false),
            session_duration: o.get_field("sessionDuration", false),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl access_rule::Guest for Component {
    fn invoke(name: String, args: access_rule::Args) -> access_rule::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "configuration".into(),
                    schema: vec![
                        129, 166, 79, 98, 106, 101, 99, 116, 130, 166, 116, 97, 114, 103, 101, 116,
                        166, 83, 116, 114, 105, 110, 103, 165, 118, 97, 108, 117, 101, 166, 83,
                        116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "mode".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "notes".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        access_rule::Res {
            account_id: o.get_field("accountId", true),
            configuration: o.get_field("configuration", true),
            mode: o.get_field("mode", true),
            notes: o.get_field("notes", false),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl access_service_token::Guest for Component {
    fn invoke(name: String, args: access_service_token::Args) -> access_service_token::Res {
        wasm_common::setup_logger();
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
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "clientId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "clientSecret".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "duration".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "expiresAt".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "minDaysForRenewal".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
            ],
        };

        let o = register(&request);

        access_service_token::Res {
            account_id: o.get_field("accountId", false),
            client_id: o.get_field("clientId", true),
            client_secret: o.get_field("clientSecret", true),
            duration: o.get_field("duration", true),
            expires_at: o.get_field("expiresAt", true),
            min_days_for_renewal: o.get_field("minDaysForRenewal", false),
            name: o.get_field("name", true),
            zone_id: o.get_field("zoneId", false),
        }
    }
}
impl access_tag::Guest for Component {
    fn invoke(name: String, args: access_tag::Args) -> access_tag::Res {
        wasm_common::setup_logger();
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
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "appCount".into(),
                    schema: vec![163, 73, 110, 116],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
            ],
        };

        let o = register(&request);

        access_tag::Res {
            account_id: o.get_field("accountId", false),
            app_count: o.get_field("appCount", true),
            name: o.get_field("name", true),
            zone_id: o.get_field("zoneId", false),
        }
    }
}
impl account::Guest for Component {
    fn invoke(name: String, args: account::Args) -> account::Res {
        wasm_common::setup_logger();
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
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "type".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
            ],
        };

        let o = register(&request);

        account::Res {
            enforce_twofactor: o.get_field("enforceTwofactor", false),
            name: o.get_field("name", true),
            type_: o.get_field("type", false),
        }
    }
}
impl account_member::Guest for Component {
    fn invoke(name: String, args: account_member::Args) -> account_member::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "emailAddress".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "roleIds".into(),
                    schema: vec![
                        129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "status".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        account_member::Res {
            account_id: o.get_field("accountId", true),
            email_address: o.get_field("emailAddress", true),
            role_ids: o.get_field("roleIds", true),
            status: o.get_field("status", true),
        }
    }
}
impl address_map::Guest for Component {
    fn invoke(name: String, args: address_map::Args) -> address_map::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "canDelete".into(),
                    schema: vec![164, 66, 111, 111, 108],
                },
                ResultField {
                    name: "canModifyIps".into(),
                    schema: vec![164, 66, 111, 111, 108],
                },
                ResultField {
                    name: "defaultSni".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "description".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "enabled".into(),
                    schema: vec![164, 66, 111, 111, 108],
                },
                ResultField {
                    name: "ips".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 129, 162, 105, 112, 166, 83, 116,
                        114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "memberships".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 131, 169, 99, 97, 110, 68, 101,
                        108, 101, 116, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66,
                        111, 111, 108, 170, 105, 100, 101, 110, 116, 105, 102, 105, 101, 114, 166,
                        83, 116, 114, 105, 110, 103, 164, 107, 105, 110, 100, 166, 83, 116, 114,
                        105, 110, 103,
                    ],
                },
            ],
        };

        let o = register(&request);

        address_map::Res {
            account_id: o.get_field("accountId", true),
            can_delete: o.get_field("canDelete", true),
            can_modify_ips: o.get_field("canModifyIps", true),
            default_sni: o.get_field("defaultSni", false),
            description: o.get_field("description", false),
            enabled: o.get_field("enabled", true),
            ips: o.get_field("ips", false),
            memberships: o.get_field("memberships", false),
        }
    }
}
impl api_shield::Guest for Component {
    fn invoke(name: String, args: api_shield::Args) -> api_shield::Res {
        wasm_common::setup_logger();
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
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 164, 110, 97, 109, 101, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                        164, 116, 121, 112, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        api_shield::Res {
            auth_id_characteristics: o.get_field("authIdCharacteristics", false),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl api_shield_operation::Guest for Component {
    fn invoke(name: String, args: api_shield_operation::Args) -> api_shield_operation::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "host".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "method".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        api_shield_operation::Res {
            endpoint: o.get_field("endpoint", true),
            host: o.get_field("host", true),
            method: o.get_field("method", true),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl api_shield_operation_schema_validation_settings::Guest for Component {
    fn invoke(
        name: String,
        args: api_shield_operation_schema_validation_settings::Args,
    ) -> api_shield_operation_schema_validation_settings::Res {
        wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/apiShieldOperationSchemaValidationSettings:ApiShieldOperationSchemaValidationSettings".into(),
            name,
            object: vec![
                ObjectField { name: "mitigationAction".into(), value: args.mitigation_action },
                ObjectField { name: "operationId".into(), value: args.operation_id },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "mitigationAction".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "operationId".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "zoneId".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
            ],
        };

        let o = register(&request);

        api_shield_operation_schema_validation_settings::Res {
            mitigation_action: o.get_field("mitigationAction", false),
            operation_id: o.get_field("operationId", true),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl api_shield_schema::Guest for Component {
    fn invoke(name: String, args: api_shield_schema::Args) -> api_shield_schema::Res {
        wasm_common::setup_logger();
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
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "source".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "validationEnabled".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        api_shield_schema::Res {
            kind: o.get_field("kind", false),
            name: o.get_field("name", true),
            source: o.get_field("source", true),
            validation_enabled: o.get_field("validationEnabled", false),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl api_shield_schema_validation_settings::Guest for Component {
    fn invoke(
        name: String,
        args: api_shield_schema_validation_settings::Args,
    ) -> api_shield_schema_validation_settings::Res {
        wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "cloudflare:index/apiShieldSchemaValidationSettings:ApiShieldSchemaValidationSettings".into(),
            name,
            object: vec![
                ObjectField { name: "validationDefaultMitigationAction".into(), value: args.validation_default_mitigation_action },
                ObjectField { name: "validationOverrideMitigationAction".into(), value: args.validation_override_mitigation_action },
                ObjectField { name: "zoneId".into(), value: args.zone_id },
            ],
            results: vec![
                ResultField { name: "validationDefaultMitigationAction".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "validationOverrideMitigationAction".into(), schema: vec![129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "zoneId".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
            ],
        };

        let o = register(&request);

        api_shield_schema_validation_settings::Res {
            validation_default_mitigation_action: o
                .get_field("validationDefaultMitigationAction", true),
            validation_override_mitigation_action: o
                .get_field("validationOverrideMitigationAction", false),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl api_token::Guest for Component {
    fn invoke(name: String, args: api_token::Args) -> api_token::Res {
        wasm_common::setup_logger();
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
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101,
                        99, 116, 129, 169, 114, 101, 113, 117, 101, 115, 116, 73, 112, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116,
                        130, 163, 105, 110, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 166,
                        110, 111, 116, 73, 110, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "expiresOn".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "issuedOn".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "modifiedOn".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "notBefore".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "policies".into(),
                    schema: vec![
                        129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 131,
                        166, 101, 102, 102, 101, 99, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 166, 83, 116, 114, 105, 110, 103, 176, 112, 101, 114, 109, 105, 115,
                        115, 105, 111, 110, 71, 114, 111, 117, 112, 115, 129, 165, 65, 114, 114,
                        97, 121, 166, 83, 116, 114, 105, 110, 103, 169, 114, 101, 115, 111, 117,
                        114, 99, 101, 115, 129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112,
                        101, 79, 98, 106, 101, 99, 116, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "status".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "value".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        api_token::Res {
            condition: o.get_field("condition", false),
            expires_on: o.get_field("expiresOn", false),
            issued_on: o.get_field("issuedOn", true),
            modified_on: o.get_field("modifiedOn", true),
            name: o.get_field("name", true),
            not_before: o.get_field("notBefore", false),
            policies: o.get_field("policies", true),
            status: o.get_field("status", true),
            value: o.get_field("value", true),
        }
    }
}
impl argo::Guest for Component {
    fn invoke(name: String, args: argo::Args) -> argo::Res {
        wasm_common::setup_logger();
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
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "tieredCaching".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        argo::Res {
            smart_routing: o.get_field("smartRouting", false),
            tiered_caching: o.get_field("tieredCaching", false),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl authenticated_origin_pulls::Guest for Component {
    fn invoke(
        name: String,
        args: authenticated_origin_pulls::Args,
    ) -> authenticated_origin_pulls::Res {
        wasm_common::setup_logger();
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
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "enabled".into(),
                    schema: vec![164, 66, 111, 111, 108],
                },
                ResultField {
                    name: "hostname".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        authenticated_origin_pulls::Res {
            authenticated_origin_pulls_certificate: o
                .get_field("authenticatedOriginPullsCertificate", false),
            enabled: o.get_field("enabled", true),
            hostname: o.get_field("hostname", false),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl authenticated_origin_pulls_certificate::Guest for Component {
    fn invoke(
        name: String,
        args: authenticated_origin_pulls_certificate::Args,
    ) -> authenticated_origin_pulls_certificate::Res {
        wasm_common::setup_logger();
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
                ResultField { name: "certificate".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "expiresOn".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "issuer".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "privateKey".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "serialNumber".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "signature".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "status".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "type".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "uploadedOn".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
                ResultField { name: "zoneId".into(), schema: vec![166, 83, 116, 114, 105, 110, 103] },
            ],
        };

        let o = register(&request);

        authenticated_origin_pulls_certificate::Res {
            certificate: o.get_field("certificate", true),
            expires_on: o.get_field("expiresOn", true),
            issuer: o.get_field("issuer", true),
            private_key: o.get_field("privateKey", true),
            serial_number: o.get_field("serialNumber", true),
            signature: o.get_field("signature", true),
            status: o.get_field("status", true),
            type_: o.get_field("type", true),
            uploaded_on: o.get_field("uploadedOn", true),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl bot_management::Guest for Component {
    fn invoke(name: String, args: bot_management::Args) -> bot_management::Res {
        wasm_common::setup_logger();
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
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "enableJs".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "fightMode".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "optimizeWordpress".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "sbfmDefinitelyAutomated".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "sbfmLikelyAutomated".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "sbfmStaticResourceProtection".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "sbfmVerifiedBots".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "suppressSessionScore".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "usingLatestModel".into(),
                    schema: vec![164, 66, 111, 111, 108],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        bot_management::Res {
            auto_update_model: o.get_field("autoUpdateModel", false),
            enable_js: o.get_field("enableJs", false),
            fight_mode: o.get_field("fightMode", false),
            optimize_wordpress: o.get_field("optimizeWordpress", false),
            sbfm_definitely_automated: o.get_field("sbfmDefinitelyAutomated", false),
            sbfm_likely_automated: o.get_field("sbfmLikelyAutomated", false),
            sbfm_static_resource_protection: o.get_field("sbfmStaticResourceProtection", false),
            sbfm_verified_bots: o.get_field("sbfmVerifiedBots", false),
            suppress_session_score: o.get_field("suppressSessionScore", false),
            using_latest_model: o.get_field("usingLatestModel", true),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl byo_ip_prefix::Guest for Component {
    fn invoke(name: String, args: byo_ip_prefix::Args) -> byo_ip_prefix::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "advertisement".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "description".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "prefixId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        byo_ip_prefix::Res {
            account_id: o.get_field("accountId", true),
            advertisement: o.get_field("advertisement", true),
            description: o.get_field("description", true),
            prefix_id: o.get_field("prefixId", true),
        }
    }
}
impl certificate_pack::Guest for Component {
    fn invoke(name: String, args: certificate_pack::Args) -> certificate_pack::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "cloudflareBranding".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "hosts".into(),
                    schema: vec![
                        129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "type".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "validationErrors".into(),
                    schema: vec![
                        129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 129,
                        167, 109, 101, 115, 115, 97, 103, 101, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "validationMethod".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "validationRecords".into(),
                    schema: vec![
                        129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 135,
                        169, 99, 110, 97, 109, 101, 78, 97, 109, 101, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 171, 99, 110, 97, 109,
                        101, 84, 97, 114, 103, 101, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 166, 83, 116, 114, 105, 110, 103, 166, 101, 109, 97, 105, 108, 115,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 166, 83, 116, 114, 105, 110, 103, 168, 104, 116, 116, 112, 66, 111,
                        100, 121, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114,
                        105, 110, 103, 167, 104, 116, 116, 112, 85, 114, 108, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 167, 116,
                        120, 116, 78, 97, 109, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        166, 83, 116, 114, 105, 110, 103, 168, 116, 120, 116, 86, 97, 108, 117,
                        101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105,
                        110, 103,
                    ],
                },
                ResultField {
                    name: "validityDays".into(),
                    schema: vec![163, 73, 110, 116],
                },
                ResultField {
                    name: "waitForActiveStatus".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        certificate_pack::Res {
            certificate_authority: o.get_field("certificateAuthority", true),
            cloudflare_branding: o.get_field("cloudflareBranding", false),
            hosts: o.get_field("hosts", true),
            type_: o.get_field("type", true),
            validation_errors: o.get_field("validationErrors", true),
            validation_method: o.get_field("validationMethod", true),
            validation_records: o.get_field("validationRecords", true),
            validity_days: o.get_field("validityDays", true),
            wait_for_active_status: o.get_field("waitForActiveStatus", false),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl custom_hostname::Guest for Component {
    fn invoke(name: String, args: custom_hostname::Args) -> custom_hostname::Res {
        wasm_common::setup_logger();
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
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 176, 83, 105, 110, 103,
                        108, 101, 84, 121, 112, 101, 79, 98, 106, 101, 99, 116, 166, 83, 116, 114,
                        105, 110, 103,
                    ],
                },
                ResultField {
                    name: "customOriginServer".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "customOriginSni".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "hostname".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "ownershipVerification".into(),
                    schema: vec![
                        129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106, 101,
                        99, 116, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "ownershipVerificationHttp".into(),
                    schema: vec![
                        129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106, 101,
                        99, 116, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "ssls".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 139, 172, 98, 117, 110, 100, 108,
                        101, 77, 101, 116, 104, 111, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 166, 83, 116, 114, 105, 110, 103, 180, 99, 101, 114, 116, 105, 102,
                        105, 99, 97, 116, 101, 65, 117, 116, 104, 111, 114, 105, 116, 121, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                        177, 99, 117, 115, 116, 111, 109, 67, 101, 114, 116, 105, 102, 105, 99, 97,
                        116, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114,
                        105, 110, 103, 169, 99, 117, 115, 116, 111, 109, 75, 101, 121, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 166,
                        109, 101, 116, 104, 111, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 166, 83, 116, 114, 105, 110, 103, 168, 115, 101, 116, 116, 105, 110,
                        103, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114,
                        114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 133, 167, 99, 105, 112,
                        104, 101, 114, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129,
                        165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 170, 101, 97,
                        114, 108, 121, 72, 105, 110, 116, 115, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 166, 83, 116, 114, 105, 110, 103, 165, 104, 116, 116, 112, 50,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103, 173, 109, 105, 110, 84, 108, 115, 86, 101, 114, 115, 105, 111, 110,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103, 165, 116, 108, 115, 49, 51, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 166, 83, 116, 114, 105, 110, 103, 166, 115, 116, 97, 116, 117, 115,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103, 164, 116, 121, 112, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 166, 83, 116, 114, 105, 110, 103, 176, 118, 97, 108, 105, 100, 97,
                        116, 105, 111, 110, 69, 114, 114, 111, 114, 115, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98,
                        106, 101, 99, 116, 129, 167, 109, 101, 115, 115, 97, 103, 101, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 177,
                        118, 97, 108, 105, 100, 97, 116, 105, 111, 110, 82, 101, 99, 111, 114, 100,
                        115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114,
                        97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 135, 169, 99, 110, 97, 109,
                        101, 78, 97, 109, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166,
                        83, 116, 114, 105, 110, 103, 171, 99, 110, 97, 109, 101, 84, 97, 114, 103,
                        101, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114,
                        105, 110, 103, 166, 101, 109, 97, 105, 108, 115, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114,
                        105, 110, 103, 168, 104, 116, 116, 112, 66, 111, 100, 121, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 167,
                        104, 116, 116, 112, 85, 114, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 166, 83, 116, 114, 105, 110, 103, 167, 116, 120, 116, 78, 97, 109,
                        101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105,
                        110, 103, 168, 116, 120, 116, 86, 97, 108, 117, 101, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 168, 119,
                        105, 108, 100, 99, 97, 114, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "status".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "waitForSslPendingValidation".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        custom_hostname::Res {
            custom_metadata: o.get_field("customMetadata", false),
            custom_origin_server: o.get_field("customOriginServer", false),
            custom_origin_sni: o.get_field("customOriginSni", false),
            hostname: o.get_field("hostname", true),
            ownership_verification: o.get_field("ownershipVerification", true),
            ownership_verification_http: o.get_field("ownershipVerificationHttp", true),
            ssls: o.get_field("ssls", false),
            status: o.get_field("status", true),
            wait_for_ssl_pending_validation: o.get_field("waitForSslPendingValidation", false),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl custom_hostname_fallback_origin::Guest for Component {
    fn invoke(
        name: String,
        args: custom_hostname_fallback_origin::Args,
    ) -> custom_hostname_fallback_origin::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "status".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        custom_hostname_fallback_origin::Res {
            origin: o.get_field("origin", true),
            status: o.get_field("status", true),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl custom_pages::Guest for Component {
    fn invoke(name: String, args: custom_pages::Args) -> custom_pages::Res {
        wasm_common::setup_logger();
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
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "state".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "type".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "url".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
            ],
        };

        let o = register(&request);

        custom_pages::Res {
            account_id: o.get_field("accountId", false),
            state: o.get_field("state", false),
            type_: o.get_field("type", true),
            url: o.get_field("url", true),
            zone_id: o.get_field("zoneId", false),
        }
    }
}
impl custom_ssl::Guest for Component {
    fn invoke(name: String, args: custom_ssl::Args) -> custom_ssl::Res {
        wasm_common::setup_logger();
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
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101,
                        99, 116, 133, 172, 98, 117, 110, 100, 108, 101, 77, 101, 116, 104, 111,
                        100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105,
                        110, 103, 171, 99, 101, 114, 116, 105, 102, 105, 99, 97, 116, 101, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                        175, 103, 101, 111, 82, 101, 115, 116, 114, 105, 99, 116, 105, 111, 110,
                        115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105,
                        110, 103, 170, 112, 114, 105, 118, 97, 116, 101, 75, 101, 121, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 164,
                        116, 121, 112, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83,
                        116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "customSslPriorities".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 162, 105, 100, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 168,
                        112, 114, 105, 111, 114, 105, 116, 121, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "expiresOn".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "hosts".into(),
                    schema: vec![
                        129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "issuer".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "modifiedOn".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "priority".into(),
                    schema: vec![163, 73, 110, 116],
                },
                ResultField {
                    name: "signature".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "status".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "uploadedOn".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        custom_ssl::Res {
            custom_ssl_options: o.get_field("customSslOptions", false),
            custom_ssl_priorities: o.get_field("customSslPriorities", false),
            expires_on: o.get_field("expiresOn", true),
            hosts: o.get_field("hosts", true),
            issuer: o.get_field("issuer", true),
            modified_on: o.get_field("modifiedOn", true),
            priority: o.get_field("priority", true),
            signature: o.get_field("signature", true),
            status: o.get_field("status", true),
            uploaded_on: o.get_field("uploadedOn", true),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl d1_database::Guest for Component {
    fn invoke(name: String, args: d1_database::Args) -> d1_database::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "version".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        d1_database::Res {
            account_id: o.get_field("accountId", true),
            name: o.get_field("name", true),
            version: o.get_field("version", true),
        }
    }
}
impl device_dex_test::Guest for Component {
    fn invoke(name: String, args: device_dex_test::Args) -> device_dex_test::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "created".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "data".into(),
                    schema: vec![
                        129, 166, 79, 98, 106, 101, 99, 116, 131, 164, 104, 111, 115, 116, 166, 83,
                        116, 114, 105, 110, 103, 164, 107, 105, 110, 100, 166, 83, 116, 114, 105,
                        110, 103, 166, 109, 101, 116, 104, 111, 100, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "description".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "enabled".into(),
                    schema: vec![164, 66, 111, 111, 108],
                },
                ResultField {
                    name: "interval".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "updated".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        device_dex_test::Res {
            account_id: o.get_field("accountId", true),
            created: o.get_field("created", true),
            data: o.get_field("data", true),
            description: o.get_field("description", true),
            enabled: o.get_field("enabled", true),
            interval: o.get_field("interval", true),
            name: o.get_field("name", true),
            updated: o.get_field("updated", true),
        }
    }
}
impl device_managed_networks::Guest for Component {
    fn invoke(name: String, args: device_managed_networks::Args) -> device_managed_networks::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "config".into(),
                    schema: vec![
                        129, 166, 79, 98, 106, 101, 99, 116, 130, 166, 115, 104, 97, 50, 53, 54,
                        166, 83, 116, 114, 105, 110, 103, 171, 116, 108, 115, 83, 111, 99, 107, 97,
                        100, 100, 114, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "type".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        device_managed_networks::Res {
            account_id: o.get_field("accountId", true),
            config: o.get_field("config", true),
            name: o.get_field("name", true),
            type_: o.get_field("type", true),
        }
    }
}
impl device_policy_certificates::Guest for Component {
    fn invoke(
        name: String,
        args: device_policy_certificates::Args,
    ) -> device_policy_certificates::Res {
        wasm_common::setup_logger();
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
                    schema: vec![164, 66, 111, 111, 108],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        device_policy_certificates::Res {
            enabled: o.get_field("enabled", true),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl device_posture_integration::Guest for Component {
    fn invoke(
        name: String,
        args: device_posture_integration::Args,
    ) -> device_posture_integration::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "configs".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 136, 174, 97, 99, 99, 101, 115,
                        115, 67, 108, 105, 101, 110, 116, 73, 100, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 178, 97, 99, 99, 101, 115,
                        115, 67, 108, 105, 101, 110, 116, 83, 101, 99, 114, 101, 116, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 166, 97,
                        112, 105, 85, 114, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166,
                        83, 116, 114, 105, 110, 103, 167, 97, 117, 116, 104, 85, 114, 108, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                        168, 99, 108, 105, 101, 110, 116, 73, 100, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 169, 99, 108, 105, 101,
                        110, 116, 75, 101, 121, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166,
                        83, 116, 114, 105, 110, 103, 172, 99, 108, 105, 101, 110, 116, 83, 101, 99,
                        114, 101, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116,
                        114, 105, 110, 103, 170, 99, 117, 115, 116, 111, 109, 101, 114, 73, 100,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "identifier".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "interval".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "type".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        device_posture_integration::Res {
            account_id: o.get_field("accountId", true),
            configs: o.get_field("configs", false),
            identifier: o.get_field("identifier", false),
            interval: o.get_field("interval", false),
            name: o.get_field("name", true),
            type_: o.get_field("type", true),
        }
    }
}
impl device_posture_rule::Guest for Component {
    fn invoke(name: String, args: device_posture_rule::Args) -> device_posture_rule::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "description".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "expiration".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "inputs".into(),
                    schema: vec![
                        129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 222,
                        0, 33, 173, 97, 99, 116, 105, 118, 101, 84, 104, 114, 101, 97, 116, 115,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 173, 99,
                        101, 114, 116, 105, 102, 105, 99, 97, 116, 101, 73, 100, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 170, 99, 104,
                        101, 99, 107, 68, 105, 115, 107, 115, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110,
                        103, 162, 99, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83,
                        116, 114, 105, 110, 103, 176, 99, 111, 109, 112, 108, 105, 97, 110, 99,
                        101, 83, 116, 97, 116, 117, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 166, 83, 116, 114, 105, 110, 103, 172, 99, 111, 110, 110, 101, 99,
                        116, 105, 111, 110, 73, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        166, 83, 116, 114, 105, 110, 103, 173, 99, 111, 117, 110, 116, 79, 112,
                        101, 114, 97, 116, 111, 114, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        166, 83, 116, 114, 105, 110, 103, 166, 100, 111, 109, 97, 105, 110, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                        171, 101, 105, 100, 76, 97, 115, 116, 83, 101, 101, 110, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 167, 101,
                        110, 97, 98, 108, 101, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        164, 66, 111, 111, 108, 166, 101, 120, 105, 115, 116, 115, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 162, 105, 100,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103, 168, 105, 110, 102, 101, 99, 116, 101, 100, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 168, 105, 115, 65, 99, 116,
                        105, 118, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111,
                        111, 108, 170, 105, 115, 115, 117, 101, 67, 111, 117, 110, 116, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 168,
                        108, 97, 115, 116, 83, 101, 101, 110, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 166, 83, 116, 114, 105, 110, 103, 173, 110, 101, 116, 119, 111,
                        114, 107, 83, 116, 97, 116, 117, 115, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 166, 83, 116, 114, 105, 110, 103, 168, 111, 112, 101, 114, 97,
                        116, 111, 114, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116,
                        114, 105, 110, 103, 162, 111, 115, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 166, 83, 116, 114, 105, 110, 103, 172, 111, 115, 68, 105, 115,
                        116, 114, 111, 78, 97, 109, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 166, 83, 116, 114, 105, 110, 103, 176, 111, 115, 68, 105, 115, 116,
                        114, 111, 82, 101, 118, 105, 115, 105, 111, 110, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 167, 111, 118,
                        101, 114, 97, 108, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166,
                        83, 116, 114, 105, 110, 103, 164, 112, 97, 116, 104, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 170, 114,
                        101, 113, 117, 105, 114, 101, 65, 108, 108, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 164, 66, 111, 111, 108, 169, 114, 105, 115, 107, 76, 101,
                        118, 101, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116,
                        114, 105, 110, 103, 167, 114, 117, 110, 110, 105, 110, 103, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 172, 115, 101,
                        110, 115, 111, 114, 67, 111, 110, 102, 105, 103, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 166, 115, 104, 97,
                        50, 53, 54, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116,
                        114, 105, 110, 103, 165, 115, 116, 97, 116, 101, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 170, 116, 104,
                        117, 109, 98, 112, 114, 105, 110, 116, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 166, 83, 116, 114, 105, 110, 103, 170, 116, 111, 116, 97, 108,
                        83, 99, 111, 114, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163,
                        73, 110, 116, 167, 118, 101, 114, 115, 105, 111, 110, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 175, 118,
                        101, 114, 115, 105, 111, 110, 79, 112, 101, 114, 97, 116, 111, 114, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "matches".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 129, 168, 112, 108, 97, 116, 102,
                        111, 114, 109, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116,
                        114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "schedule".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "type".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        device_posture_rule::Res {
            account_id: o.get_field("accountId", true),
            description: o.get_field("description", false),
            expiration: o.get_field("expiration", false),
            inputs: o.get_field("inputs", true),
            matches: o.get_field("matches", false),
            name: o.get_field("name", false),
            schedule: o.get_field("schedule", false),
            type_: o.get_field("type", true),
        }
    }
}
impl device_settings_policy::Guest for Component {
    fn invoke(name: String, args: device_settings_policy::Args) -> device_settings_policy::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "allowModeSwitch".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "allowUpdates".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "allowedToLeave".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "autoConnect".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "captivePortal".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "default".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "description".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "disableAutoFallback".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "enabled".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "excludeOfficeIps".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "match".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "precedence".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "serviceModeV2Mode".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "serviceModeV2Port".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "supportUrl".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "switchLocked".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
            ],
        };

        let o = register(&request);

        device_settings_policy::Res {
            account_id: o.get_field("accountId", true),
            allow_mode_switch: o.get_field("allowModeSwitch", false),
            allow_updates: o.get_field("allowUpdates", false),
            allowed_to_leave: o.get_field("allowedToLeave", false),
            auto_connect: o.get_field("autoConnect", false),
            captive_portal: o.get_field("captivePortal", false),
            default: o.get_field("default", false),
            description: o.get_field("description", true),
            disable_auto_fallback: o.get_field("disableAutoFallback", false),
            enabled: o.get_field("enabled", false),
            exclude_office_ips: o.get_field("excludeOfficeIps", false),
            match_: o.get_field("match", false),
            name: o.get_field("name", true),
            precedence: o.get_field("precedence", false),
            service_mode_v2_mode: o.get_field("serviceModeV2Mode", false),
            service_mode_v2_port: o.get_field("serviceModeV2Port", false),
            support_url: o.get_field("supportUrl", false),
            switch_locked: o.get_field("switchLocked", false),
        }
    }
}
impl dlp_profile::Guest for Component {
    fn invoke(name: String, args: dlp_profile::Args) -> dlp_profile::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "allowedMatchCount".into(),
                    schema: vec![163, 73, 110, 116],
                },
                ResultField {
                    name: "contextAwareness".into(),
                    schema: vec![
                        129, 166, 79, 98, 106, 101, 99, 116, 130, 167, 101, 110, 97, 98, 108, 101,
                        100, 164, 66, 111, 111, 108, 164, 115, 107, 105, 112, 129, 166, 79, 98,
                        106, 101, 99, 116, 129, 165, 102, 105, 108, 101, 115, 164, 66, 111, 111,
                        108,
                    ],
                },
                ResultField {
                    name: "description".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "entries".into(),
                    schema: vec![
                        129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 132,
                        167, 101, 110, 97, 98, 108, 101, 100, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 164, 66, 111, 111, 108, 162, 105, 100, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 164, 110, 97, 109,
                        101, 166, 83, 116, 114, 105, 110, 103, 167, 112, 97, 116, 116, 101, 114,
                        110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106,
                        101, 99, 116, 130, 165, 114, 101, 103, 101, 120, 166, 83, 116, 114, 105,
                        110, 103, 170, 118, 97, 108, 105, 100, 97, 116, 105, 111, 110, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "type".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        dlp_profile::Res {
            account_id: o.get_field("accountId", true),
            allowed_match_count: o.get_field("allowedMatchCount", true),
            context_awareness: o.get_field("contextAwareness", true),
            description: o.get_field("description", false),
            entries: o.get_field("entries", true),
            name: o.get_field("name", true),
            type_: o.get_field("type", true),
        }
    }
}
impl email_routing_address::Guest for Component {
    fn invoke(name: String, args: email_routing_address::Args) -> email_routing_address::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "created".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "email".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "modified".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "tag".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "verified".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        email_routing_address::Res {
            account_id: o.get_field("accountId", true),
            created: o.get_field("created", true),
            email: o.get_field("email", true),
            modified: o.get_field("modified", true),
            tag: o.get_field("tag", true),
            verified: o.get_field("verified", true),
        }
    }
}
impl email_routing_catch_all::Guest for Component {
    fn invoke(name: String, args: email_routing_catch_all::Args) -> email_routing_catch_all::Res {
        wasm_common::setup_logger();
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
                    schema: vec![
                        129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 130,
                        164, 116, 121, 112, 101, 166, 83, 116, 114, 105, 110, 103, 166, 118, 97,
                        108, 117, 101, 115, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114,
                        105, 110, 103,
                    ],
                },
                ResultField {
                    name: "enabled".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "matchers".into(),
                    schema: vec![
                        129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 129,
                        164, 116, 121, 112, 101, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "tag".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        email_routing_catch_all::Res {
            actions: o.get_field("actions", true),
            enabled: o.get_field("enabled", false),
            matchers: o.get_field("matchers", true),
            name: o.get_field("name", true),
            tag: o.get_field("tag", true),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl email_routing_rule::Guest for Component {
    fn invoke(name: String, args: email_routing_rule::Args) -> email_routing_rule::Res {
        wasm_common::setup_logger();
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
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 164, 116, 121, 112, 101,
                        166, 83, 116, 114, 105, 110, 103, 166, 118, 97, 108, 117, 101, 115, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121,
                        166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "enabled".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "matchers".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 131, 165, 102, 105, 101, 108,
                        100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105,
                        110, 103, 164, 116, 121, 112, 101, 166, 83, 116, 114, 105, 110, 103, 165,
                        118, 97, 108, 117, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166,
                        83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "priority".into(),
                    schema: vec![163, 73, 110, 116],
                },
                ResultField {
                    name: "tag".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        email_routing_rule::Res {
            actions: o.get_field("actions", false),
            enabled: o.get_field("enabled", false),
            matchers: o.get_field("matchers", false),
            name: o.get_field("name", true),
            priority: o.get_field("priority", true),
            tag: o.get_field("tag", true),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl email_routing_settings::Guest for Component {
    fn invoke(name: String, args: email_routing_settings::Args) -> email_routing_settings::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "enabled".into(),
                    schema: vec![164, 66, 111, 111, 108],
                },
                ResultField {
                    name: "modified".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "skipWizard".into(),
                    schema: vec![164, 66, 111, 111, 108],
                },
                ResultField {
                    name: "status".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "tag".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        email_routing_settings::Res {
            created: o.get_field("created", true),
            enabled: o.get_field("enabled", true),
            modified: o.get_field("modified", true),
            name: o.get_field("name", true),
            skip_wizard: o.get_field("skipWizard", true),
            status: o.get_field("status", true),
            tag: o.get_field("tag", true),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl fallback_domain::Guest for Component {
    fn invoke(name: String, args: fallback_domain::Args) -> fallback_domain::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "domains".into(),
                    schema: vec![
                        129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 131,
                        171, 100, 101, 115, 99, 114, 105, 112, 116, 105, 111, 110, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 170,
                        100, 110, 115, 83, 101, 114, 118, 101, 114, 115, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114,
                        105, 110, 103, 166, 115, 117, 102, 102, 105, 120, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "policyId".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
            ],
        };

        let o = register(&request);

        fallback_domain::Res {
            account_id: o.get_field("accountId", true),
            domains: o.get_field("domains", true),
            policy_id: o.get_field("policyId", false),
        }
    }
}
impl filter::Guest for Component {
    fn invoke(name: String, args: filter::Args) -> filter::Res {
        wasm_common::setup_logger();
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
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "expression".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "paused".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "ref".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        filter::Res {
            description: o.get_field("description", false),
            expression: o.get_field("expression", true),
            paused: o.get_field("paused", false),
            ref_: o.get_field("ref", false),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl firewall_rule::Guest for Component {
    fn invoke(name: String, args: firewall_rule::Args) -> firewall_rule::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "description".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "filterId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "paused".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "priority".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "products".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        firewall_rule::Res {
            action: o.get_field("action", true),
            description: o.get_field("description", false),
            filter_id: o.get_field("filterId", true),
            paused: o.get_field("paused", false),
            priority: o.get_field("priority", false),
            products: o.get_field("products", false),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl gre_tunnel::Guest for Component {
    fn invoke(name: String, args: gre_tunnel::Args) -> gre_tunnel::Res {
        wasm_common::setup_logger();
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
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "cloudflareGreEndpoint".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "customerGreEndpoint".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "description".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "healthCheckEnabled".into(),
                    schema: vec![164, 66, 111, 111, 108],
                },
                ResultField {
                    name: "healthCheckTarget".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "healthCheckType".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "interfaceAddress".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "mtu".into(),
                    schema: vec![163, 73, 110, 116],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "ttl".into(),
                    schema: vec![163, 73, 110, 116],
                },
            ],
        };

        let o = register(&request);

        gre_tunnel::Res {
            account_id: o.get_field("accountId", false),
            cloudflare_gre_endpoint: o.get_field("cloudflareGreEndpoint", true),
            customer_gre_endpoint: o.get_field("customerGreEndpoint", true),
            description: o.get_field("description", false),
            health_check_enabled: o.get_field("healthCheckEnabled", true),
            health_check_target: o.get_field("healthCheckTarget", true),
            health_check_type: o.get_field("healthCheckType", true),
            interface_address: o.get_field("interfaceAddress", true),
            mtu: o.get_field("mtu", true),
            name: o.get_field("name", true),
            ttl: o.get_field("ttl", true),
        }
    }
}
impl healthcheck::Guest for Component {
    fn invoke(name: String, args: healthcheck::Args) -> healthcheck::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "allowInsecure".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "checkRegions".into(),
                    schema: vec![
                        129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "consecutiveFails".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "consecutiveSuccesses".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "createdOn".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "description".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "expectedBody".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "expectedCodes".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "followRedirects".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "headers".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 166, 104, 101, 97, 100, 101,
                        114, 166, 83, 116, 114, 105, 110, 103, 166, 118, 97, 108, 117, 101, 115,
                        129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "interval".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "method".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "modifiedOn".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "path".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "port".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "retries".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "suspended".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "timeout".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "type".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        healthcheck::Res {
            address: o.get_field("address", true),
            allow_insecure: o.get_field("allowInsecure", false),
            check_regions: o.get_field("checkRegions", true),
            consecutive_fails: o.get_field("consecutiveFails", false),
            consecutive_successes: o.get_field("consecutiveSuccesses", false),
            created_on: o.get_field("createdOn", true),
            description: o.get_field("description", false),
            expected_body: o.get_field("expectedBody", false),
            expected_codes: o.get_field("expectedCodes", false),
            follow_redirects: o.get_field("followRedirects", false),
            headers: o.get_field("headers", false),
            interval: o.get_field("interval", false),
            method: o.get_field("method", true),
            modified_on: o.get_field("modifiedOn", true),
            name: o.get_field("name", true),
            path: o.get_field("path", false),
            port: o.get_field("port", false),
            retries: o.get_field("retries", false),
            suspended: o.get_field("suspended", false),
            timeout: o.get_field("timeout", false),
            type_: o.get_field("type", true),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl hostname_tls_setting::Guest for Component {
    fn invoke(name: String, args: hostname_tls_setting::Args) -> hostname_tls_setting::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "hostname".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "setting".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "updatedAt".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "value".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        hostname_tls_setting::Res {
            created_at: o.get_field("createdAt", true),
            hostname: o.get_field("hostname", true),
            setting: o.get_field("setting", true),
            updated_at: o.get_field("updatedAt", true),
            value: o.get_field("value", true),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl hostname_tls_setting_ciphers::Guest for Component {
    fn invoke(
        name: String,
        args: hostname_tls_setting_ciphers::Args,
    ) -> hostname_tls_setting_ciphers::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "hostname".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "ports".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "updatedAt".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "values".into(),
                    schema: vec![
                        129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        hostname_tls_setting_ciphers::Res {
            created_at: o.get_field("createdAt", true),
            hostname: o.get_field("hostname", true),
            ports: o.get_field("ports", false),
            updated_at: o.get_field("updatedAt", true),
            values: o.get_field("values", true),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl hyperdrive_config::Guest for Component {
    fn invoke(name: String, args: hyperdrive_config::Args) -> hyperdrive_config::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "caching".into(),
                    schema: vec![
                        129, 166, 79, 98, 106, 101, 99, 116, 129, 168, 100, 105, 115, 97, 98, 108,
                        101, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111,
                        108,
                    ],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "origin".into(),
                    schema: vec![
                        129, 166, 79, 98, 106, 101, 99, 116, 134, 168, 100, 97, 116, 97, 98, 97,
                        115, 101, 166, 83, 116, 114, 105, 110, 103, 164, 104, 111, 115, 116, 166,
                        83, 116, 114, 105, 110, 103, 168, 112, 97, 115, 115, 119, 111, 114, 100,
                        166, 83, 116, 114, 105, 110, 103, 164, 112, 111, 114, 116, 163, 73, 110,
                        116, 166, 115, 99, 104, 101, 109, 101, 166, 83, 116, 114, 105, 110, 103,
                        164, 117, 115, 101, 114, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
            ],
        };

        let o = register(&request);

        hyperdrive_config::Res {
            account_id: o.get_field("accountId", true),
            caching: o.get_field("caching", true),
            name: o.get_field("name", true),
            origin: o.get_field("origin", true),
        }
    }
}
impl ipsec_tunnel::Guest for Component {
    fn invoke(name: String, args: ipsec_tunnel::Args) -> ipsec_tunnel::Res {
        wasm_common::setup_logger();
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
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "allowNullCipher".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "cloudflareEndpoint".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "customerEndpoint".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "description".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "fqdnId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "healthCheckDirection".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "healthCheckEnabled".into(),
                    schema: vec![164, 66, 111, 111, 108],
                },
                ResultField {
                    name: "healthCheckRate".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "healthCheckTarget".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "healthCheckType".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "hexId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "interfaceAddress".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "psk".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "remoteId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "userId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        ipsec_tunnel::Res {
            account_id: o.get_field("accountId", false),
            allow_null_cipher: o.get_field("allowNullCipher", false),
            cloudflare_endpoint: o.get_field("cloudflareEndpoint", true),
            customer_endpoint: o.get_field("customerEndpoint", true),
            description: o.get_field("description", false),
            fqdn_id: o.get_field("fqdnId", true),
            health_check_direction: o.get_field("healthCheckDirection", true),
            health_check_enabled: o.get_field("healthCheckEnabled", true),
            health_check_rate: o.get_field("healthCheckRate", true),
            health_check_target: o.get_field("healthCheckTarget", true),
            health_check_type: o.get_field("healthCheckType", true),
            hex_id: o.get_field("hexId", true),
            interface_address: o.get_field("interfaceAddress", true),
            name: o.get_field("name", true),
            psk: o.get_field("psk", true),
            remote_id: o.get_field("remoteId", true),
            user_id: o.get_field("userId", true),
        }
    }
}
impl keyless_certificate::Guest for Component {
    fn invoke(name: String, args: keyless_certificate::Args) -> keyless_certificate::Res {
        wasm_common::setup_logger();
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
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "certificate".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "enabled".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "host".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "port".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "status".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        keyless_certificate::Res {
            bundle_method: o.get_field("bundleMethod", false),
            certificate: o.get_field("certificate", true),
            enabled: o.get_field("enabled", false),
            host: o.get_field("host", true),
            name: o.get_field("name", false),
            port: o.get_field("port", false),
            status: o.get_field("status", true),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl list::Guest for Component {
    fn invoke(name: String, args: list::Args) -> list::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "description".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "items".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 167, 99, 111, 109, 109, 101,
                        110, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114,
                        105, 110, 103, 165, 118, 97, 108, 117, 101, 129, 166, 79, 98, 106, 101, 99,
                        116, 132, 163, 97, 115, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        163, 73, 110, 116, 169, 104, 111, 115, 116, 110, 97, 109, 101, 115, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121,
                        129, 166, 79, 98, 106, 101, 99, 116, 129, 171, 117, 114, 108, 72, 111, 115,
                        116, 110, 97, 109, 101, 166, 83, 116, 114, 105, 110, 103, 162, 105, 112,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103, 169, 114, 101, 100, 105, 114, 101, 99, 116, 115, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79,
                        98, 106, 101, 99, 116, 135, 177, 105, 110, 99, 108, 117, 100, 101, 83, 117,
                        98, 100, 111, 109, 97, 105, 110, 115, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 166, 83, 116, 114, 105, 110, 103, 178, 112, 114, 101, 115, 101,
                        114, 118, 101, 80, 97, 116, 104, 83, 117, 102, 102, 105, 120, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 179,
                        112, 114, 101, 115, 101, 114, 118, 101, 81, 117, 101, 114, 121, 83, 116,
                        114, 105, 110, 103, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83,
                        116, 114, 105, 110, 103, 169, 115, 111, 117, 114, 99, 101, 85, 114, 108,
                        166, 83, 116, 114, 105, 110, 103, 170, 115, 116, 97, 116, 117, 115, 67,
                        111, 100, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110,
                        116, 175, 115, 117, 98, 112, 97, 116, 104, 77, 97, 116, 99, 104, 105, 110,
                        103, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105,
                        110, 103, 169, 116, 97, 114, 103, 101, 116, 85, 114, 108, 166, 83, 116,
                        114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "kind".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        list::Res {
            account_id: o.get_field("accountId", true),
            description: o.get_field("description", false),
            items: o.get_field("items", false),
            kind: o.get_field("kind", true),
            name: o.get_field("name", true),
        }
    }
}
impl list_item::Guest for Component {
    fn invoke(name: String, args: list_item::Args) -> list_item::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "asn".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "comment".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "hostname".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101,
                        99, 116, 129, 171, 117, 114, 108, 72, 111, 115, 116, 110, 97, 109, 101,
                        166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "ip".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "listId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "redirect".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101,
                        99, 116, 135, 177, 105, 110, 99, 108, 117, 100, 101, 83, 117, 98, 100, 111,
                        109, 97, 105, 110, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164,
                        66, 111, 111, 108, 178, 112, 114, 101, 115, 101, 114, 118, 101, 80, 97,
                        116, 104, 83, 117, 102, 102, 105, 120, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 164, 66, 111, 111, 108, 179, 112, 114, 101, 115, 101, 114, 118,
                        101, 81, 117, 101, 114, 121, 83, 116, 114, 105, 110, 103, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 169, 115, 111,
                        117, 114, 99, 101, 85, 114, 108, 166, 83, 116, 114, 105, 110, 103, 170,
                        115, 116, 97, 116, 117, 115, 67, 111, 100, 101, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 163, 73, 110, 116, 175, 115, 117, 98, 112, 97, 116,
                        104, 77, 97, 116, 99, 104, 105, 110, 103, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 164, 66, 111, 111, 108, 169, 116, 97, 114, 103, 101, 116, 85,
                        114, 108, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
            ],
        };

        let o = register(&request);

        list_item::Res {
            account_id: o.get_field("accountId", true),
            asn: o.get_field("asn", false),
            comment: o.get_field("comment", false),
            hostname: o.get_field("hostname", false),
            ip: o.get_field("ip", false),
            list_id: o.get_field("listId", true),
            redirect: o.get_field("redirect", false),
        }
    }
}
impl load_balancer::Guest for Component {
    fn invoke(name: String, args: load_balancer::Args) -> load_balancer::Res {
        wasm_common::setup_logger();
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
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 129, 179, 102, 97, 105, 108, 111,
                        118, 101, 114, 65, 99, 114, 111, 115, 115, 80, 111, 111, 108, 115, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "countryPools".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 167, 99, 111, 117, 110, 116,
                        114, 121, 166, 83, 116, 114, 105, 110, 103, 167, 112, 111, 111, 108, 73,
                        100, 115, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "createdOn".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "defaultPoolIds".into(),
                    schema: vec![
                        129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "description".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "enabled".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "fallbackPoolId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "locationStrategies".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 164, 109, 111, 100, 101,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103, 169, 112, 114, 101, 102, 101, 114, 69, 99, 115, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "modifiedOn".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "popPools".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 167, 112, 111, 111, 108, 73,
                        100, 115, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110,
                        103, 163, 112, 111, 112, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "proxied".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "randomSteerings".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 173, 100, 101, 102, 97, 117,
                        108, 116, 87, 101, 105, 103, 104, 116, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 166, 68, 111, 117, 98, 108, 101, 171, 112, 111, 111, 108, 87,
                        101, 105, 103, 104, 116, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106,
                        101, 99, 116, 166, 68, 111, 117, 98, 108, 101,
                    ],
                },
                ResultField {
                    name: "regionPools".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 167, 112, 111, 111, 108, 73,
                        100, 115, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110,
                        103, 166, 114, 101, 103, 105, 111, 110, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "rules".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 135, 169, 99, 111, 110, 100, 105,
                        116, 105, 111, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83,
                        116, 114, 105, 110, 103, 168, 100, 105, 115, 97, 98, 108, 101, 100, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 173, 102,
                        105, 120, 101, 100, 82, 101, 115, 112, 111, 110, 115, 101, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 132,
                        171, 99, 111, 110, 116, 101, 110, 116, 84, 121, 112, 101, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 168,
                        108, 111, 99, 97, 116, 105, 111, 110, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 166, 83, 116, 114, 105, 110, 103, 171, 109, 101, 115, 115, 97,
                        103, 101, 66, 111, 100, 121, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        166, 83, 116, 114, 105, 110, 103, 170, 115, 116, 97, 116, 117, 115, 67,
                        111, 100, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110,
                        116, 164, 110, 97, 109, 101, 166, 83, 116, 114, 105, 110, 103, 169, 111,
                        118, 101, 114, 114, 105, 100, 101, 115, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101,
                        99, 116, 141, 176, 97, 100, 97, 112, 116, 105, 118, 101, 82, 111, 117, 116,
                        105, 110, 103, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129,
                        165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 129, 179,
                        102, 97, 105, 108, 111, 118, 101, 114, 65, 99, 114, 111, 115, 115, 80, 111,
                        111, 108, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111,
                        111, 108, 172, 99, 111, 117, 110, 116, 114, 121, 80, 111, 111, 108, 115,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 167, 99, 111, 117, 110, 116,
                        114, 121, 166, 83, 116, 114, 105, 110, 103, 167, 112, 111, 111, 108, 73,
                        100, 115, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110,
                        103, 172, 100, 101, 102, 97, 117, 108, 116, 80, 111, 111, 108, 115, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121,
                        166, 83, 116, 114, 105, 110, 103, 172, 102, 97, 108, 108, 98, 97, 99, 107,
                        80, 111, 111, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83,
                        116, 114, 105, 110, 103, 178, 108, 111, 99, 97, 116, 105, 111, 110, 83,
                        116, 114, 97, 116, 101, 103, 105, 101, 115, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106,
                        101, 99, 116, 130, 164, 109, 111, 100, 101, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 169, 112, 114, 101,
                        102, 101, 114, 69, 99, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        166, 83, 116, 114, 105, 110, 103, 168, 112, 111, 112, 80, 111, 111, 108,
                        115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114,
                        97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 167, 112, 111, 111, 108,
                        73, 100, 115, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110,
                        103, 163, 112, 111, 112, 166, 83, 116, 114, 105, 110, 103, 175, 114, 97,
                        110, 100, 111, 109, 83, 116, 101, 101, 114, 105, 110, 103, 115, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129,
                        166, 79, 98, 106, 101, 99, 116, 130, 173, 100, 101, 102, 97, 117, 108, 116,
                        87, 101, 105, 103, 104, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        166, 68, 111, 117, 98, 108, 101, 171, 112, 111, 111, 108, 87, 101, 105,
                        103, 104, 116, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129,
                        176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106, 101, 99,
                        116, 166, 68, 111, 117, 98, 108, 101, 171, 114, 101, 103, 105, 111, 110,
                        80, 111, 111, 108, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129,
                        165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 167,
                        112, 111, 111, 108, 73, 100, 115, 129, 165, 65, 114, 114, 97, 121, 166, 83,
                        116, 114, 105, 110, 103, 166, 114, 101, 103, 105, 111, 110, 166, 83, 116,
                        114, 105, 110, 103, 175, 115, 101, 115, 115, 105, 111, 110, 65, 102, 102,
                        105, 110, 105, 116, 121, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        166, 83, 116, 114, 105, 110, 103, 185, 115, 101, 115, 115, 105, 111, 110,
                        65, 102, 102, 105, 110, 105, 116, 121, 65, 116, 116, 114, 105, 98, 117,
                        116, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65,
                        114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 133, 167, 104, 101,
                        97, 100, 101, 114, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129,
                        165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 177, 114,
                        101, 113, 117, 105, 114, 101, 65, 108, 108, 72, 101, 97, 100, 101, 114,
                        115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                        168, 115, 97, 109, 101, 115, 105, 116, 101, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 166, 115, 101, 99, 117,
                        114, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114,
                        105, 110, 103, 180, 122, 101, 114, 111, 68, 111, 119, 110, 116, 105, 109,
                        101, 70, 97, 105, 108, 111, 118, 101, 114, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 178, 115, 101, 115, 115,
                        105, 111, 110, 65, 102, 102, 105, 110, 105, 116, 121, 84, 116, 108, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 174, 115, 116,
                        101, 101, 114, 105, 110, 103, 80, 111, 108, 105, 99, 121, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 163,
                        116, 116, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110,
                        116, 168, 112, 114, 105, 111, 114, 105, 116, 121, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 163, 73, 110, 116, 170, 116, 101, 114, 109, 105,
                        110, 97, 116, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164,
                        66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "sessionAffinity".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "sessionAffinityAttributes".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 134, 173, 100, 114, 97, 105, 110,
                        68, 117, 114, 97, 116, 105, 111, 110, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 163, 73, 110, 116, 167, 104, 101, 97, 100, 101, 114, 115, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121,
                        166, 83, 116, 114, 105, 110, 103, 177, 114, 101, 113, 117, 105, 114, 101,
                        65, 108, 108, 72, 101, 97, 100, 101, 114, 115, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 164, 66, 111, 111, 108, 168, 115, 97, 109, 101, 115, 105,
                        116, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114,
                        105, 110, 103, 166, 115, 101, 99, 117, 114, 101, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 180, 122, 101,
                        114, 111, 68, 111, 119, 110, 116, 105, 109, 101, 70, 97, 105, 108, 111,
                        118, 101, 114, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116,
                        114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "sessionAffinityTtl".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "steeringPolicy".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "ttl".into(),
                    schema: vec![163, 73, 110, 116],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        load_balancer::Res {
            adaptive_routings: o.get_field("adaptiveRoutings", false),
            country_pools: o.get_field("countryPools", false),
            created_on: o.get_field("createdOn", true),
            default_pool_ids: o.get_field("defaultPoolIds", true),
            description: o.get_field("description", false),
            enabled: o.get_field("enabled", false),
            fallback_pool_id: o.get_field("fallbackPoolId", true),
            location_strategies: o.get_field("locationStrategies", false),
            modified_on: o.get_field("modifiedOn", true),
            name: o.get_field("name", true),
            pop_pools: o.get_field("popPools", false),
            proxied: o.get_field("proxied", false),
            random_steerings: o.get_field("randomSteerings", false),
            region_pools: o.get_field("regionPools", false),
            rules: o.get_field("rules", false),
            session_affinity: o.get_field("sessionAffinity", false),
            session_affinity_attributes: o.get_field("sessionAffinityAttributes", false),
            session_affinity_ttl: o.get_field("sessionAffinityTtl", false),
            steering_policy: o.get_field("steeringPolicy", true),
            ttl: o.get_field("ttl", true),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl load_balancer_monitor::Guest for Component {
    fn invoke(name: String, args: load_balancer_monitor::Args) -> load_balancer_monitor::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "allowInsecure".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "consecutiveDown".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "consecutiveUp".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "createdOn".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "description".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "expectedBody".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "expectedCodes".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "followRedirects".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "headers".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 166, 104, 101, 97, 100, 101,
                        114, 166, 83, 116, 114, 105, 110, 103, 166, 118, 97, 108, 117, 101, 115,
                        129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "interval".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "method".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "modifiedOn".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "path".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "port".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "probeZone".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "retries".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "timeout".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "type".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
            ],
        };

        let o = register(&request);

        load_balancer_monitor::Res {
            account_id: o.get_field("accountId", true),
            allow_insecure: o.get_field("allowInsecure", false),
            consecutive_down: o.get_field("consecutiveDown", false),
            consecutive_up: o.get_field("consecutiveUp", false),
            created_on: o.get_field("createdOn", true),
            description: o.get_field("description", false),
            expected_body: o.get_field("expectedBody", false),
            expected_codes: o.get_field("expectedCodes", false),
            follow_redirects: o.get_field("followRedirects", false),
            headers: o.get_field("headers", false),
            interval: o.get_field("interval", false),
            method: o.get_field("method", true),
            modified_on: o.get_field("modifiedOn", true),
            path: o.get_field("path", true),
            port: o.get_field("port", false),
            probe_zone: o.get_field("probeZone", false),
            retries: o.get_field("retries", false),
            timeout: o.get_field("timeout", false),
            type_: o.get_field("type", false),
        }
    }
}
impl load_balancer_pool::Guest for Component {
    fn invoke(name: String, args: load_balancer_pool::Args) -> load_balancer_pool::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "checkRegions".into(),
                    schema: vec![
                        129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "createdOn".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "description".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "enabled".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "latitude".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 68, 111, 117, 98, 108,
                        101,
                    ],
                },
                ResultField {
                    name: "loadSheddings".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 132, 174, 100, 101, 102, 97, 117,
                        108, 116, 80, 101, 114, 99, 101, 110, 116, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 166, 68, 111, 117, 98, 108, 101, 173, 100, 101, 102, 97, 117,
                        108, 116, 80, 111, 108, 105, 99, 121, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 166, 83, 116, 114, 105, 110, 103, 174, 115, 101, 115, 115, 105,
                        111, 110, 80, 101, 114, 99, 101, 110, 116, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 166, 68, 111, 117, 98, 108, 101, 173, 115, 101, 115, 115,
                        105, 111, 110, 80, 111, 108, 105, 99, 121, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "longitude".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 68, 111, 117, 98, 108,
                        101,
                    ],
                },
                ResultField {
                    name: "minimumOrigins".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "modifiedOn".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "monitor".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "notificationEmail".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "originSteerings".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 129, 166, 112, 111, 108, 105, 99,
                        121, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105,
                        110, 103,
                    ],
                },
                ResultField {
                    name: "origins".into(),
                    schema: vec![
                        129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 133,
                        167, 97, 100, 100, 114, 101, 115, 115, 166, 83, 116, 114, 105, 110, 103,
                        167, 101, 110, 97, 98, 108, 101, 100, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 164, 66, 111, 111, 108, 167, 104, 101, 97, 100, 101, 114, 115,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 166, 104, 101, 97, 100, 101,
                        114, 166, 83, 116, 114, 105, 110, 103, 166, 118, 97, 108, 117, 101, 115,
                        129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 164,
                        110, 97, 109, 101, 166, 83, 116, 114, 105, 110, 103, 166, 119, 101, 105,
                        103, 104, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 68, 111,
                        117, 98, 108, 101,
                    ],
                },
            ],
        };

        let o = register(&request);

        load_balancer_pool::Res {
            account_id: o.get_field("accountId", true),
            check_regions: o.get_field("checkRegions", true),
            created_on: o.get_field("createdOn", true),
            description: o.get_field("description", false),
            enabled: o.get_field("enabled", false),
            latitude: o.get_field("latitude", false),
            load_sheddings: o.get_field("loadSheddings", false),
            longitude: o.get_field("longitude", false),
            minimum_origins: o.get_field("minimumOrigins", false),
            modified_on: o.get_field("modifiedOn", true),
            monitor: o.get_field("monitor", false),
            name: o.get_field("name", true),
            notification_email: o.get_field("notificationEmail", false),
            origin_steerings: o.get_field("originSteerings", false),
            origins: o.get_field("origins", true),
        }
    }
}
impl logpull_retention::Guest for Component {
    fn invoke(name: String, args: logpull_retention::Args) -> logpull_retention::Res {
        wasm_common::setup_logger();
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
                    schema: vec![164, 66, 111, 111, 108],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        logpull_retention::Res {
            enabled: o.get_field("enabled", true),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl logpush_job::Guest for Component {
    fn invoke(name: String, args: logpush_job::Args) -> logpush_job::Res {
        wasm_common::setup_logger();
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
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "dataset".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "destinationConf".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "enabled".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "filter".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "frequency".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "kind".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "logpullOptions".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "maxUploadBytes".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "maxUploadIntervalSeconds".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "maxUploadRecords".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "outputOptions".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101,
                        99, 116, 140, 171, 98, 97, 116, 99, 104, 80, 114, 101, 102, 105, 120, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                        171, 98, 97, 116, 99, 104, 83, 117, 102, 102, 105, 120, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 171, 99, 118,
                        101, 50, 48, 50, 49, 52, 52, 50, 56, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 164, 66, 111, 111, 108, 174, 102, 105, 101, 108, 100, 68, 101,
                        108, 105, 109, 105, 116, 101, 114, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 166, 83, 116, 114, 105, 110, 103, 170, 102, 105, 101, 108, 100,
                        78, 97, 109, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129,
                        165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 170, 111,
                        117, 116, 112, 117, 116, 84, 121, 112, 101, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 175, 114, 101, 99, 111,
                        114, 100, 68, 101, 108, 105, 109, 105, 116, 101, 114, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 172, 114,
                        101, 99, 111, 114, 100, 80, 114, 101, 102, 105, 120, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 172, 114,
                        101, 99, 111, 114, 100, 83, 117, 102, 102, 105, 120, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 174, 114,
                        101, 99, 111, 114, 100, 84, 101, 109, 112, 108, 97, 116, 101, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 170,
                        115, 97, 109, 112, 108, 101, 82, 97, 116, 101, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 166, 68, 111, 117, 98, 108, 101, 175, 116, 105, 109, 101,
                        115, 116, 97, 109, 112, 70, 111, 114, 109, 97, 116, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "ownershipChallenge".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
            ],
        };

        let o = register(&request);

        logpush_job::Res {
            account_id: o.get_field("accountId", false),
            dataset: o.get_field("dataset", true),
            destination_conf: o.get_field("destinationConf", true),
            enabled: o.get_field("enabled", false),
            filter: o.get_field("filter", false),
            frequency: o.get_field("frequency", false),
            kind: o.get_field("kind", false),
            logpull_options: o.get_field("logpullOptions", false),
            max_upload_bytes: o.get_field("maxUploadBytes", false),
            max_upload_interval_seconds: o.get_field("maxUploadIntervalSeconds", false),
            max_upload_records: o.get_field("maxUploadRecords", false),
            name: o.get_field("name", false),
            output_options: o.get_field("outputOptions", false),
            ownership_challenge: o.get_field("ownershipChallenge", false),
            zone_id: o.get_field("zoneId", false),
        }
    }
}
impl logpush_ownership_challenge::Guest for Component {
    fn invoke(
        name: String,
        args: logpush_ownership_challenge::Args,
    ) -> logpush_ownership_challenge::Res {
        wasm_common::setup_logger();
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
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "destinationConf".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "ownershipChallengeFilename".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
            ],
        };

        let o = register(&request);

        logpush_ownership_challenge::Res {
            account_id: o.get_field("accountId", false),
            destination_conf: o.get_field("destinationConf", true),
            ownership_challenge_filename: o.get_field("ownershipChallengeFilename", true),
            zone_id: o.get_field("zoneId", false),
        }
    }
}
impl magic_firewall_ruleset::Guest for Component {
    fn invoke(name: String, args: magic_firewall_ruleset::Args) -> magic_firewall_ruleset::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "description".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "rules".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106,
                        101, 99, 116, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
            ],
        };

        let o = register(&request);

        magic_firewall_ruleset::Res {
            account_id: o.get_field("accountId", true),
            description: o.get_field("description", false),
            name: o.get_field("name", true),
            rules: o.get_field("rules", false),
        }
    }
}
impl managed_headers::Guest for Component {
    fn invoke(name: String, args: managed_headers::Args) -> managed_headers::Res {
        wasm_common::setup_logger();
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
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 167, 101, 110, 97, 98, 108,
                        101, 100, 164, 66, 111, 111, 108, 162, 105, 100, 166, 83, 116, 114, 105,
                        110, 103,
                    ],
                },
                ResultField {
                    name: "managedResponseHeaders".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 167, 101, 110, 97, 98, 108,
                        101, 100, 164, 66, 111, 111, 108, 162, 105, 100, 166, 83, 116, 114, 105,
                        110, 103,
                    ],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        managed_headers::Res {
            managed_request_headers: o.get_field("managedRequestHeaders", false),
            managed_response_headers: o.get_field("managedResponseHeaders", false),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl mtls_certificate::Guest for Component {
    fn invoke(name: String, args: mtls_certificate::Args) -> mtls_certificate::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "ca".into(),
                    schema: vec![164, 66, 111, 111, 108],
                },
                ResultField {
                    name: "certificates".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "expiresOn".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "issuer".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "privateKey".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "serialNumber".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "signature".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "uploadedOn".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        mtls_certificate::Res {
            account_id: o.get_field("accountId", true),
            ca: o.get_field("ca", true),
            certificates: o.get_field("certificates", true),
            expires_on: o.get_field("expiresOn", true),
            issuer: o.get_field("issuer", true),
            name: o.get_field("name", false),
            private_key: o.get_field("privateKey", false),
            serial_number: o.get_field("serialNumber", true),
            signature: o.get_field("signature", true),
            uploaded_on: o.get_field("uploadedOn", true),
        }
    }
}
impl notification_policy::Guest for Component {
    fn invoke(name: String, args: notification_policy::Args) -> notification_policy::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "alertType".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "created".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "description".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "emailIntegrations".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 162, 105, 100, 166, 83, 116,
                        114, 105, 110, 103, 164, 110, 97, 109, 101, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "enabled".into(),
                    schema: vec![164, 66, 111, 111, 108],
                },
                ResultField {
                    name: "filters".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101,
                        99, 116, 222, 0, 32, 167, 97, 99, 116, 105, 111, 110, 115, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83,
                        116, 114, 105, 110, 103, 178, 97, 102, 102, 101, 99, 116, 101, 100, 67,
                        111, 109, 112, 111, 110, 101, 110, 116, 115, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105,
                        110, 103, 172, 97, 105, 114, 112, 111, 114, 116, 67, 111, 100, 101, 115,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 166, 83, 116, 114, 105, 110, 103, 183, 97, 108, 101, 114, 116, 84,
                        114, 105, 103, 103, 101, 114, 80, 114, 101, 102, 101, 114, 101, 110, 99,
                        101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114,
                        114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 168, 101, 110, 97, 98, 108,
                        101, 100, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65,
                        114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 172, 101, 110, 118,
                        105, 114, 111, 110, 109, 101, 110, 116, 115, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105,
                        110, 103, 172, 101, 118, 101, 110, 116, 83, 111, 117, 114, 99, 101, 115,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 166, 83, 116, 114, 105, 110, 103, 170, 101, 118, 101, 110, 116, 84,
                        121, 112, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129,
                        165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 166, 101,
                        118, 101, 110, 116, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 169,
                        103, 114, 111, 117, 112, 66, 105, 101, 115, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105,
                        110, 103, 174, 104, 101, 97, 108, 116, 104, 67, 104, 101, 99, 107, 73, 100,
                        115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114,
                        97, 121, 166, 83, 116, 114, 105, 110, 103, 175, 105, 110, 99, 105, 100,
                        101, 110, 116, 73, 109, 112, 97, 99, 116, 115, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105,
                        110, 103, 168, 105, 110, 112, 117, 116, 73, 100, 115, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116,
                        114, 105, 110, 103, 166, 108, 105, 109, 105, 116, 115, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116,
                        114, 105, 110, 103, 178, 109, 101, 103, 97, 98, 105, 116, 115, 80, 101,
                        114, 83, 101, 99, 111, 110, 100, 115, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110,
                        103, 170, 110, 101, 119, 72, 101, 97, 108, 116, 104, 115, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83,
                        116, 114, 105, 110, 103, 171, 110, 101, 119, 83, 116, 97, 116, 117, 115,
                        101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114,
                        114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 177, 112, 97, 99, 107, 101,
                        116, 115, 80, 101, 114, 83, 101, 99, 111, 110, 100, 115, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116,
                        114, 105, 110, 103, 167, 112, 111, 111, 108, 73, 100, 115, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83,
                        116, 114, 105, 110, 103, 168, 112, 114, 111, 100, 117, 99, 116, 115, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121,
                        166, 83, 116, 114, 105, 110, 103, 170, 112, 114, 111, 106, 101, 99, 116,
                        73, 100, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65,
                        114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 169, 112, 114, 111,
                        116, 111, 99, 111, 108, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 178,
                        114, 101, 113, 117, 101, 115, 116, 115, 80, 101, 114, 83, 101, 99, 111,
                        110, 100, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65,
                        114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 169, 115, 101, 108,
                        101, 99, 116, 111, 114, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 168,
                        115, 101, 114, 118, 105, 99, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110,
                        103, 164, 115, 108, 111, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103,
                        168, 115, 116, 97, 116, 117, 115, 101, 115, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105,
                        110, 103, 175, 116, 97, 114, 103, 101, 116, 72, 111, 115, 116, 110, 97,
                        109, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65,
                        114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 175, 116, 97, 114,
                        103, 101, 116, 90, 111, 110, 101, 78, 97, 109, 101, 115, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116,
                        114, 105, 110, 103, 169, 116, 117, 110, 110, 101, 108, 73, 100, 115, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121,
                        166, 83, 116, 114, 105, 110, 103, 166, 119, 104, 101, 114, 101, 115, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121,
                        166, 83, 116, 114, 105, 110, 103, 165, 122, 111, 110, 101, 115, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166,
                        83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "modified".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "pagerdutyIntegrations".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 162, 105, 100, 166, 83, 116,
                        114, 105, 110, 103, 164, 110, 97, 109, 101, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "webhooksIntegrations".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 162, 105, 100, 166, 83, 116,
                        114, 105, 110, 103, 164, 110, 97, 109, 101, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
            ],
        };

        let o = register(&request);

        notification_policy::Res {
            account_id: o.get_field("accountId", true),
            alert_type: o.get_field("alertType", true),
            created: o.get_field("created", true),
            description: o.get_field("description", false),
            email_integrations: o.get_field("emailIntegrations", false),
            enabled: o.get_field("enabled", true),
            filters: o.get_field("filters", false),
            modified: o.get_field("modified", true),
            name: o.get_field("name", true),
            pagerduty_integrations: o.get_field("pagerdutyIntegrations", false),
            webhooks_integrations: o.get_field("webhooksIntegrations", false),
        }
    }
}
impl notification_policy_webhooks::Guest for Component {
    fn invoke(
        name: String,
        args: notification_policy_webhooks::Args,
    ) -> notification_policy_webhooks::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "createdAt".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "lastFailure".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "lastSuccess".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "secret".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "type".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "url".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
            ],
        };

        let o = register(&request);

        notification_policy_webhooks::Res {
            account_id: o.get_field("accountId", true),
            created_at: o.get_field("createdAt", true),
            last_failure: o.get_field("lastFailure", true),
            last_success: o.get_field("lastSuccess", true),
            name: o.get_field("name", true),
            secret: o.get_field("secret", false),
            type_: o.get_field("type", true),
            url: o.get_field("url", false),
        }
    }
}
impl observatory_scheduled_test::Guest for Component {
    fn invoke(
        name: String,
        args: observatory_scheduled_test::Args,
    ) -> observatory_scheduled_test::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "region".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "url".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        observatory_scheduled_test::Res {
            frequency: o.get_field("frequency", true),
            region: o.get_field("region", true),
            url: o.get_field("url", true),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl origin_ca_certificate::Guest for Component {
    fn invoke(name: String, args: origin_ca_certificate::Args) -> origin_ca_certificate::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "csr".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "expiresOn".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "hostnames".into(),
                    schema: vec![
                        129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "minDaysForRenewal".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "requestType".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "requestedValidity".into(),
                    schema: vec![163, 73, 110, 116],
                },
            ],
        };

        let o = register(&request);

        origin_ca_certificate::Res {
            certificate: o.get_field("certificate", true),
            csr: o.get_field("csr", true),
            expires_on: o.get_field("expiresOn", true),
            hostnames: o.get_field("hostnames", true),
            min_days_for_renewal: o.get_field("minDaysForRenewal", false),
            request_type: o.get_field("requestType", true),
            requested_validity: o.get_field("requestedValidity", true),
        }
    }
}
impl page_rule::Guest for Component {
    fn invoke(name: String, args: page_rule::Args) -> page_rule::Res {
        wasm_common::setup_logger();
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
                    schema: vec![
                        129, 166, 79, 98, 106, 101, 99, 116, 222, 0, 37, 174, 97, 108, 119, 97,
                        121, 115, 85, 115, 101, 72, 116, 116, 112, 115, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 182, 97, 117, 116, 111, 109,
                        97, 116, 105, 99, 72, 116, 116, 112, 115, 82, 101, 119, 114, 105, 116, 101,
                        115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105,
                        110, 103, 175, 98, 114, 111, 119, 115, 101, 114, 67, 97, 99, 104, 101, 84,
                        116, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114,
                        105, 110, 103, 172, 98, 114, 111, 119, 115, 101, 114, 67, 104, 101, 99,
                        107, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105,
                        110, 103, 179, 98, 121, 112, 97, 115, 115, 67, 97, 99, 104, 101, 79, 110,
                        67, 111, 111, 107, 105, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        166, 83, 116, 114, 105, 110, 103, 177, 99, 97, 99, 104, 101, 66, 121, 68,
                        101, 118, 105, 99, 101, 84, 121, 112, 101, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 179, 99, 97, 99, 104, 101,
                        68, 101, 99, 101, 112, 116, 105, 111, 110, 65, 114, 109, 111, 114, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                        174, 99, 97, 99, 104, 101, 75, 101, 121, 70, 105, 101, 108, 100, 115, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99,
                        116, 133, 166, 99, 111, 111, 107, 105, 101, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 130, 174, 99, 104,
                        101, 99, 107, 80, 114, 101, 115, 101, 110, 99, 101, 115, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116,
                        114, 105, 110, 103, 168, 105, 110, 99, 108, 117, 100, 101, 115, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166,
                        83, 116, 114, 105, 110, 103, 166, 104, 101, 97, 100, 101, 114, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116,
                        131, 174, 99, 104, 101, 99, 107, 80, 114, 101, 115, 101, 110, 99, 101, 115,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 166, 83, 116, 114, 105, 110, 103, 168, 101, 120, 99, 108, 117, 100,
                        101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114,
                        114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 168, 105, 110, 99, 108,
                        117, 100, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129,
                        165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 164, 104,
                        111, 115, 116, 129, 166, 79, 98, 106, 101, 99, 116, 129, 168, 114, 101,
                        115, 111, 108, 118, 101, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 164, 66, 111, 111, 108, 171, 113, 117, 101, 114, 121, 83, 116, 114,
                        105, 110, 103, 129, 166, 79, 98, 106, 101, 99, 116, 131, 168, 101, 120, 99,
                        108, 117, 100, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 166,
                        105, 103, 110, 111, 114, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 164, 66, 111, 111, 108, 168, 105, 110, 99, 108, 117, 100, 101, 115,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 166, 83, 116, 114, 105, 110, 103, 164, 117, 115, 101, 114, 129, 166,
                        79, 98, 106, 101, 99, 116, 131, 170, 100, 101, 118, 105, 99, 101, 84, 121,
                        112, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111,
                        108, 163, 103, 101, 111, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        164, 66, 111, 111, 108, 164, 108, 97, 110, 103, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 170, 99, 97, 99, 104, 101,
                        76, 101, 118, 101, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166,
                        83, 116, 114, 105, 110, 103, 173, 99, 97, 99, 104, 101, 79, 110, 67, 111,
                        111, 107, 105, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83,
                        116, 114, 105, 110, 103, 178, 99, 97, 99, 104, 101, 84, 116, 108, 66, 121,
                        83, 116, 97, 116, 117, 115, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99,
                        116, 130, 165, 99, 111, 100, 101, 115, 166, 83, 116, 114, 105, 110, 103,
                        163, 116, 116, 108, 163, 73, 110, 116, 171, 100, 105, 115, 97, 98, 108,
                        101, 65, 112, 112, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164,
                        66, 111, 111, 108, 178, 100, 105, 115, 97, 98, 108, 101, 80, 101, 114, 102,
                        111, 114, 109, 97, 110, 99, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 164, 66, 111, 111, 108, 174, 100, 105, 115, 97, 98, 108, 101, 82, 97,
                        105, 108, 103, 117, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        164, 66, 111, 111, 108, 175, 100, 105, 115, 97, 98, 108, 101, 83, 101, 99,
                        117, 114, 105, 116, 121, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        164, 66, 111, 111, 108, 172, 100, 105, 115, 97, 98, 108, 101, 90, 97, 114,
                        97, 122, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111,
                        108, 172, 101, 100, 103, 101, 67, 97, 99, 104, 101, 84, 116, 108, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 176, 101, 109, 97,
                        105, 108, 79, 98, 102, 117, 115, 99, 97, 116, 105, 111, 110, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 180,
                        101, 120, 112, 108, 105, 99, 105, 116, 67, 97, 99, 104, 101, 67, 111, 110,
                        116, 114, 111, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83,
                        116, 114, 105, 110, 103, 173, 102, 111, 114, 119, 97, 114, 100, 105, 110,
                        103, 85, 114, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166,
                        79, 98, 106, 101, 99, 116, 130, 170, 115, 116, 97, 116, 117, 115, 67, 111,
                        100, 101, 163, 73, 110, 116, 163, 117, 114, 108, 166, 83, 116, 114, 105,
                        110, 103, 178, 104, 111, 115, 116, 72, 101, 97, 100, 101, 114, 79, 118,
                        101, 114, 114, 105, 100, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 166, 83, 116, 114, 105, 110, 103, 173, 105, 112, 71, 101, 111, 108,
                        111, 99, 97, 116, 105, 111, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 166, 83, 116, 114, 105, 110, 103, 168, 109, 105, 110, 105, 102, 105,
                        101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114,
                        114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 131, 163, 99, 115, 115,
                        166, 83, 116, 114, 105, 110, 103, 164, 104, 116, 109, 108, 166, 83, 116,
                        114, 105, 110, 103, 162, 106, 115, 166, 83, 116, 114, 105, 110, 103, 166,
                        109, 105, 114, 97, 103, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        166, 83, 116, 114, 105, 110, 103, 183, 111, 112, 112, 111, 114, 116, 117,
                        110, 105, 115, 116, 105, 99, 69, 110, 99, 114, 121, 112, 116, 105, 111,
                        110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105,
                        110, 103, 183, 111, 114, 105, 103, 105, 110, 69, 114, 114, 111, 114, 80,
                        97, 103, 101, 80, 97, 115, 115, 84, 104, 114, 117, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 166, 112, 111,
                        108, 105, 115, 104, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83,
                        116, 114, 105, 110, 103, 175, 114, 101, 115, 111, 108, 118, 101, 79, 118,
                        101, 114, 114, 105, 100, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 166, 83, 116, 114, 105, 110, 103, 177, 114, 101, 115, 112, 101, 99,
                        116, 83, 116, 114, 111, 110, 103, 69, 116, 97, 103, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 177, 114, 101,
                        115, 112, 111, 110, 115, 101, 66, 117, 102, 102, 101, 114, 105, 110, 103,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103, 172, 114, 111, 99, 107, 101, 116, 76, 111, 97, 100, 101, 114, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                        173, 115, 101, 99, 117, 114, 105, 116, 121, 76, 101, 118, 101, 108, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                        177, 115, 101, 114, 118, 101, 114, 83, 105, 100, 101, 69, 120, 99, 108,
                        117, 100, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116,
                        114, 105, 110, 103, 183, 115, 111, 114, 116, 81, 117, 101, 114, 121, 83,
                        116, 114, 105, 110, 103, 70, 111, 114, 67, 97, 99, 104, 101, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 163,
                        115, 115, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116,
                        114, 105, 110, 103, 178, 116, 114, 117, 101, 67, 108, 105, 101, 110, 116,
                        73, 112, 72, 101, 97, 100, 101, 114, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 166, 83, 116, 114, 105, 110, 103, 163, 119, 97, 102, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "priority".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "status".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "target".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        page_rule::Res {
            actions: o.get_field("actions", true),
            priority: o.get_field("priority", false),
            status: o.get_field("status", false),
            target: o.get_field("target", true),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl pages_domain::Guest for Component {
    fn invoke(name: String, args: pages_domain::Args) -> pages_domain::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "domain".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "projectName".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "status".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        pages_domain::Res {
            account_id: o.get_field("accountId", true),
            domain: o.get_field("domain", true),
            project_name: o.get_field("projectName", true),
            status: o.get_field("status", true),
        }
    }
}
impl pages_project::Guest for Component {
    fn invoke(name: String, args: pages_project::Args) -> pages_project::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "buildConfig".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101,
                        99, 116, 134, 172, 98, 117, 105, 108, 100, 67, 97, 99, 104, 105, 110, 103,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 172,
                        98, 117, 105, 108, 100, 67, 111, 109, 109, 97, 110, 100, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 174, 100,
                        101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 68, 105, 114, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 167,
                        114, 111, 111, 116, 68, 105, 114, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 166, 83, 116, 114, 105, 110, 103, 175, 119, 101, 98, 65, 110, 97, 108,
                        121, 116, 105, 99, 115, 84, 97, 103, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 166, 83, 116, 114, 105, 110, 103, 177, 119, 101, 98, 65, 110, 97,
                        108, 121, 116, 105, 99, 115, 84, 111, 107, 101, 110, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "createdOn".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "deploymentConfigs".into(),
                    schema: vec![
                        129, 166, 79, 98, 106, 101, 99, 116, 130, 167, 112, 114, 101, 118, 105,
                        101, 119, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98,
                        106, 101, 99, 116, 141, 217, 32, 97, 108, 119, 97, 121, 115, 85, 115, 101,
                        76, 97, 116, 101, 115, 116, 67, 111, 109, 112, 97, 116, 105, 98, 105, 108,
                        105, 116, 121, 68, 97, 116, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 164, 66, 111, 111, 108, 177, 99, 111, 109, 112, 97, 116, 105, 98, 105,
                        108, 105, 116, 121, 68, 97, 116, 101, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 166, 83, 116, 114, 105, 110, 103, 178, 99, 111, 109, 112, 97,
                        116, 105, 98, 105, 108, 105, 116, 121, 70, 108, 97, 103, 115, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83,
                        116, 114, 105, 110, 103, 171, 100, 49, 68, 97, 116, 97, 98, 97, 115, 101,
                        115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 176, 83, 105, 110,
                        103, 108, 101, 84, 121, 112, 101, 79, 98, 106, 101, 99, 116, 166, 83, 116,
                        114, 105, 110, 103, 183, 100, 117, 114, 97, 98, 108, 101, 79, 98, 106, 101,
                        99, 116, 78, 97, 109, 101, 115, 112, 97, 99, 101, 115, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 129, 176, 83, 105, 110, 103, 108, 101, 84, 121,
                        112, 101, 79, 98, 106, 101, 99, 116, 166, 83, 116, 114, 105, 110, 103, 180,
                        101, 110, 118, 105, 114, 111, 110, 109, 101, 110, 116, 86, 97, 114, 105,
                        97, 98, 108, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129,
                        176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106, 101, 99,
                        116, 166, 83, 116, 114, 105, 110, 103, 168, 102, 97, 105, 108, 79, 112,
                        101, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111,
                        108, 172, 107, 118, 78, 97, 109, 101, 115, 112, 97, 99, 101, 115, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 129, 176, 83, 105, 110, 103, 108, 101,
                        84, 121, 112, 101, 79, 98, 106, 101, 99, 116, 166, 83, 116, 114, 105, 110,
                        103, 169, 112, 108, 97, 99, 101, 109, 101, 110, 116, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 129, 164,
                        109, 111, 100, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83,
                        116, 114, 105, 110, 103, 169, 114, 50, 66, 117, 99, 107, 101, 116, 115,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 176, 83, 105, 110, 103,
                        108, 101, 84, 121, 112, 101, 79, 98, 106, 101, 99, 116, 166, 83, 116, 114,
                        105, 110, 103, 167, 115, 101, 99, 114, 101, 116, 115, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 129, 176, 83, 105, 110, 103, 108, 101, 84, 121,
                        112, 101, 79, 98, 106, 101, 99, 116, 166, 83, 116, 114, 105, 110, 103, 175,
                        115, 101, 114, 118, 105, 99, 101, 66, 105, 110, 100, 105, 110, 103, 115,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 131, 171, 101, 110, 118, 105,
                        114, 111, 110, 109, 101, 110, 116, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 166, 83, 116, 114, 105, 110, 103, 164, 110, 97, 109, 101, 166,
                        83, 116, 114, 105, 110, 103, 167, 115, 101, 114, 118, 105, 99, 101, 166,
                        83, 116, 114, 105, 110, 103, 170, 117, 115, 97, 103, 101, 77, 111, 100,
                        101, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114,
                        105, 110, 103, 170, 112, 114, 111, 100, 117, 99, 116, 105, 111, 110, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99,
                        116, 141, 217, 32, 97, 108, 119, 97, 121, 115, 85, 115, 101, 76, 97, 116,
                        101, 115, 116, 67, 111, 109, 112, 97, 116, 105, 98, 105, 108, 105, 116,
                        121, 68, 97, 116, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164,
                        66, 111, 111, 108, 177, 99, 111, 109, 112, 97, 116, 105, 98, 105, 108, 105,
                        116, 121, 68, 97, 116, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        166, 83, 116, 114, 105, 110, 103, 178, 99, 111, 109, 112, 97, 116, 105, 98,
                        105, 108, 105, 116, 121, 70, 108, 97, 103, 115, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114,
                        105, 110, 103, 171, 100, 49, 68, 97, 116, 97, 98, 97, 115, 101, 115, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 176, 83, 105, 110, 103, 108,
                        101, 84, 121, 112, 101, 79, 98, 106, 101, 99, 116, 166, 83, 116, 114, 105,
                        110, 103, 183, 100, 117, 114, 97, 98, 108, 101, 79, 98, 106, 101, 99, 116,
                        78, 97, 109, 101, 115, 112, 97, 99, 101, 115, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101,
                        79, 98, 106, 101, 99, 116, 166, 83, 116, 114, 105, 110, 103, 180, 101, 110,
                        118, 105, 114, 111, 110, 109, 101, 110, 116, 86, 97, 114, 105, 97, 98, 108,
                        101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 176, 83, 105,
                        110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106, 101, 99, 116, 166, 83,
                        116, 114, 105, 110, 103, 168, 102, 97, 105, 108, 79, 112, 101, 110, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 172, 107,
                        118, 78, 97, 109, 101, 115, 112, 97, 99, 101, 115, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112,
                        101, 79, 98, 106, 101, 99, 116, 166, 83, 116, 114, 105, 110, 103, 169, 112,
                        108, 97, 99, 101, 109, 101, 110, 116, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 129, 164, 109, 111, 100,
                        101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105,
                        110, 103, 169, 114, 50, 66, 117, 99, 107, 101, 116, 115, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 129, 176, 83, 105, 110, 103, 108, 101, 84, 121,
                        112, 101, 79, 98, 106, 101, 99, 116, 166, 83, 116, 114, 105, 110, 103, 167,
                        115, 101, 99, 114, 101, 116, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106,
                        101, 99, 116, 166, 83, 116, 114, 105, 110, 103, 175, 115, 101, 114, 118,
                        105, 99, 101, 66, 105, 110, 100, 105, 110, 103, 115, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79,
                        98, 106, 101, 99, 116, 131, 171, 101, 110, 118, 105, 114, 111, 110, 109,
                        101, 110, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116,
                        114, 105, 110, 103, 164, 110, 97, 109, 101, 166, 83, 116, 114, 105, 110,
                        103, 167, 115, 101, 114, 118, 105, 99, 101, 166, 83, 116, 114, 105, 110,
                        103, 170, 117, 115, 97, 103, 101, 77, 111, 100, 101, 108, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "domains".into(),
                    schema: vec![
                        129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "productionBranch".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "source".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101,
                        99, 116, 130, 166, 99, 111, 110, 102, 105, 103, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 137, 178, 100,
                        101, 112, 108, 111, 121, 109, 101, 110, 116, 115, 69, 110, 97, 98, 108,
                        101, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111,
                        108, 165, 111, 119, 110, 101, 114, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 166, 83, 116, 114, 105, 110, 103, 177, 112, 114, 67, 111, 109,
                        109, 101, 110, 116, 115, 69, 110, 97, 98, 108, 101, 100, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 181, 112, 114, 101,
                        118, 105, 101, 119, 66, 114, 97, 110, 99, 104, 69, 120, 99, 108, 117, 100,
                        101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114,
                        114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 181, 112, 114, 101, 118,
                        105, 101, 119, 66, 114, 97, 110, 99, 104, 73, 110, 99, 108, 117, 100, 101,
                        115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114,
                        97, 121, 166, 83, 116, 114, 105, 110, 103, 184, 112, 114, 101, 118, 105,
                        101, 119, 68, 101, 112, 108, 111, 121, 109, 101, 110, 116, 83, 101, 116,
                        116, 105, 110, 103, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83,
                        116, 114, 105, 110, 103, 176, 112, 114, 111, 100, 117, 99, 116, 105, 111,
                        110, 66, 114, 97, 110, 99, 104, 166, 83, 116, 114, 105, 110, 103, 187, 112,
                        114, 111, 100, 117, 99, 116, 105, 111, 110, 68, 101, 112, 108, 111, 121,
                        109, 101, 110, 116, 69, 110, 97, 98, 108, 101, 100, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 168, 114, 101, 112, 111, 78,
                        97, 109, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116,
                        114, 105, 110, 103, 164, 116, 121, 112, 101, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "subdomain".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        pages_project::Res {
            account_id: o.get_field("accountId", true),
            build_config: o.get_field("buildConfig", false),
            created_on: o.get_field("createdOn", true),
            deployment_configs: o.get_field("deploymentConfigs", true),
            domains: o.get_field("domains", true),
            name: o.get_field("name", true),
            production_branch: o.get_field("productionBranch", true),
            source: o.get_field("source", false),
            subdomain: o.get_field("subdomain", true),
        }
    }
}
impl queue::Guest for Component {
    fn invoke(name: String, args: queue::Args) -> queue::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        queue::Res {
            account_id: o.get_field("accountId", true),
            name: o.get_field("name", true),
        }
    }
}
impl r2_bucket::Guest for Component {
    fn invoke(name: String, args: r2_bucket::Args) -> r2_bucket::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "location".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        r2_bucket::Res {
            account_id: o.get_field("accountId", true),
            location: o.get_field("location", true),
            name: o.get_field("name", true),
        }
    }
}
impl rate_limit::Guest for Component {
    fn invoke(name: String, args: rate_limit::Args) -> rate_limit::Res {
        wasm_common::setup_logger();
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
                    schema: vec![
                        129, 166, 79, 98, 106, 101, 99, 116, 131, 164, 109, 111, 100, 101, 166, 83,
                        116, 114, 105, 110, 103, 168, 114, 101, 115, 112, 111, 110, 115, 101, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99,
                        116, 130, 164, 98, 111, 100, 121, 166, 83, 116, 114, 105, 110, 103, 171,
                        99, 111, 110, 116, 101, 110, 116, 84, 121, 112, 101, 166, 83, 116, 114,
                        105, 110, 103, 167, 116, 105, 109, 101, 111, 117, 116, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "bypassUrlPatterns".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "correlate".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101,
                        99, 116, 129, 162, 98, 121, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "description".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "disabled".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "match".into(),
                    schema: vec![
                        129, 166, 79, 98, 106, 101, 99, 116, 130, 167, 114, 101, 113, 117, 101,
                        115, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98,
                        106, 101, 99, 116, 131, 167, 109, 101, 116, 104, 111, 100, 115, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166,
                        83, 116, 114, 105, 110, 103, 167, 115, 99, 104, 101, 109, 101, 115, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121,
                        166, 83, 116, 114, 105, 110, 103, 170, 117, 114, 108, 80, 97, 116, 116,
                        101, 114, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116,
                        114, 105, 110, 103, 168, 114, 101, 115, 112, 111, 110, 115, 101, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116,
                        131, 167, 104, 101, 97, 100, 101, 114, 115, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 176, 83, 105, 110,
                        103, 108, 101, 84, 121, 112, 101, 79, 98, 106, 101, 99, 116, 166, 83, 116,
                        114, 105, 110, 103, 173, 111, 114, 105, 103, 105, 110, 84, 114, 97, 102,
                        102, 105, 99, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111,
                        111, 108, 168, 115, 116, 97, 116, 117, 115, 101, 115, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 163, 73, 110,
                        116,
                    ],
                },
                ResultField {
                    name: "period".into(),
                    schema: vec![163, 73, 110, 116],
                },
                ResultField {
                    name: "threshold".into(),
                    schema: vec![163, 73, 110, 116],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        rate_limit::Res {
            action: o.get_field("action", true),
            bypass_url_patterns: o.get_field("bypassUrlPatterns", false),
            correlate: o.get_field("correlate", false),
            description: o.get_field("description", false),
            disabled: o.get_field("disabled", false),
            match_: o.get_field("match", true),
            period: o.get_field("period", true),
            threshold: o.get_field("threshold", true),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl record::Guest for Component {
    fn invoke(name: String, args: record::Args) -> record::Res {
        wasm_common::setup_logger();
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
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "comment".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "createdOn".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "data".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101,
                        99, 116, 222, 0, 39, 169, 97, 108, 103, 111, 114, 105, 116, 104, 109, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 168, 97, 108,
                        116, 105, 116, 117, 100, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 166, 68, 111, 117, 98, 108, 101, 171, 99, 101, 114, 116, 105, 102,
                        105, 99, 97, 116, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166,
                        83, 116, 114, 105, 110, 103, 167, 99, 111, 110, 116, 101, 110, 116, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                        166, 100, 105, 103, 101, 115, 116, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 166, 83, 116, 114, 105, 110, 103, 170, 100, 105, 103, 101, 115,
                        116, 84, 121, 112, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163,
                        73, 110, 116, 171, 102, 105, 110, 103, 101, 114, 112, 114, 105, 110, 116,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103, 165, 102, 108, 97, 103, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 166, 83, 116, 114, 105, 110, 103, 166, 107, 101, 121, 84, 97, 103,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 170, 108,
                        97, 116, 68, 101, 103, 114, 101, 101, 115, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 163, 73, 110, 116, 172, 108, 97, 116, 68, 105, 114, 101, 99,
                        116, 105, 111, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83,
                        116, 114, 105, 110, 103, 170, 108, 97, 116, 77, 105, 110, 117, 116, 101,
                        115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 170,
                        108, 97, 116, 83, 101, 99, 111, 110, 100, 115, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 166, 68, 111, 117, 98, 108, 101, 171, 108, 111, 110, 103,
                        68, 101, 103, 114, 101, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 163, 73, 110, 116, 173, 108, 111, 110, 103, 68, 105, 114, 101, 99,
                        116, 105, 111, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83,
                        116, 114, 105, 110, 103, 171, 108, 111, 110, 103, 77, 105, 110, 117, 116,
                        101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116,
                        171, 108, 111, 110, 103, 83, 101, 99, 111, 110, 100, 115, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 166, 68, 111, 117, 98, 108, 101, 172, 109,
                        97, 116, 99, 104, 105, 110, 103, 84, 121, 112, 101, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 163, 73, 110, 116, 164, 110, 97, 109, 101, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 165,
                        111, 114, 100, 101, 114, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        163, 73, 110, 116, 164, 112, 111, 114, 116, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 163, 73, 110, 116, 173, 112, 114, 101, 99, 105, 115, 105,
                        111, 110, 72, 111, 114, 122, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        166, 68, 111, 117, 98, 108, 101, 173, 112, 114, 101, 99, 105, 115, 105,
                        111, 110, 86, 101, 114, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        166, 68, 111, 117, 98, 108, 101, 170, 112, 114, 101, 102, 101, 114, 101,
                        110, 99, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110,
                        116, 168, 112, 114, 105, 111, 114, 105, 116, 121, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 163, 73, 110, 116, 165, 112, 114, 111, 116, 111,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103, 168, 112, 114, 111, 116, 111, 99, 111, 108, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 163, 73, 110, 116, 169, 112, 117, 98, 108, 105, 99,
                        75, 101, 121, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116,
                        114, 105, 110, 103, 165, 114, 101, 103, 101, 120, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 171, 114, 101,
                        112, 108, 97, 99, 101, 109, 101, 110, 116, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 168, 115, 101, 108, 101,
                        99, 116, 111, 114, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73,
                        110, 116, 167, 115, 101, 114, 118, 105, 99, 101, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 164, 115, 105,
                        122, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 68, 111, 117,
                        98, 108, 101, 163, 116, 97, 103, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 166, 83, 116, 114, 105, 110, 103, 166, 116, 97, 114, 103, 101, 116,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103, 164, 116, 121, 112, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 163, 73, 110, 116, 165, 117, 115, 97, 103, 101, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 165, 118, 97, 108, 117, 101,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103, 166, 119, 101, 105, 103, 104, 116, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "hostname".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "metadata".into(),
                    schema: vec![
                        129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106, 101,
                        99, 116, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "modifiedOn".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "priority".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "proxiable".into(),
                    schema: vec![164, 66, 111, 111, 108],
                },
                ResultField {
                    name: "proxied".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "tags".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "ttl".into(),
                    schema: vec![163, 73, 110, 116],
                },
                ResultField {
                    name: "type".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "value".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        record::Res {
            allow_overwrite: o.get_field("allowOverwrite", false),
            comment: o.get_field("comment", false),
            created_on: o.get_field("createdOn", true),
            data: o.get_field("data", false),
            hostname: o.get_field("hostname", true),
            metadata: o.get_field("metadata", true),
            modified_on: o.get_field("modifiedOn", true),
            name: o.get_field("name", true),
            priority: o.get_field("priority", false),
            proxiable: o.get_field("proxiable", true),
            proxied: o.get_field("proxied", false),
            tags: o.get_field("tags", false),
            ttl: o.get_field("ttl", true),
            type_: o.get_field("type", true),
            value: o.get_field("value", true),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl regional_hostname::Guest for Component {
    fn invoke(name: String, args: regional_hostname::Args) -> regional_hostname::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "hostname".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "regionKey".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        regional_hostname::Res {
            created_on: o.get_field("createdOn", true),
            hostname: o.get_field("hostname", true),
            region_key: o.get_field("regionKey", true),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl regional_tiered_cache::Guest for Component {
    fn invoke(name: String, args: regional_tiered_cache::Args) -> regional_tiered_cache::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        regional_tiered_cache::Res {
            value: o.get_field("value", true),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl ruleset::Guest for Component {
    fn invoke(name: String, args: ruleset::Args) -> ruleset::Res {
        wasm_common::setup_logger();
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
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "description".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "kind".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "phase".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "rules".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 140, 166, 97, 99, 116, 105, 111,
                        110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105,
                        110, 103, 176, 97, 99, 116, 105, 111, 110, 80, 97, 114, 97, 109, 101, 116,
                        101, 114, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79,
                        98, 106, 101, 99, 116, 222, 0, 51, 184, 97, 100, 100, 105, 116, 105, 111,
                        110, 97, 108, 67, 97, 99, 104, 101, 97, 98, 108, 101, 80, 111, 114, 116,
                        115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114,
                        97, 121, 163, 73, 110, 116, 170, 97, 108, 103, 111, 114, 105, 116, 104,
                        109, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114,
                        114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 129, 164, 110, 97, 109,
                        101, 166, 83, 116, 114, 105, 110, 103, 182, 97, 117, 116, 111, 109, 97,
                        116, 105, 99, 72, 116, 116, 112, 115, 82, 101, 119, 114, 105, 116, 101,
                        115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                        172, 97, 117, 116, 111, 109, 105, 110, 105, 102, 105, 101, 115, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129,
                        166, 79, 98, 106, 101, 99, 116, 131, 163, 99, 115, 115, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 164, 104, 116, 109,
                        108, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                        162, 106, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111,
                        111, 108, 163, 98, 105, 99, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        164, 66, 111, 111, 108, 170, 98, 114, 111, 119, 115, 101, 114, 84, 116,
                        108, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106,
                        101, 99, 116, 130, 167, 100, 101, 102, 97, 117, 108, 116, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 164, 109, 111, 100,
                        101, 166, 83, 116, 114, 105, 110, 103, 165, 99, 97, 99, 104, 101, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 168, 99, 97,
                        99, 104, 101, 75, 101, 121, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        129, 166, 79, 98, 106, 101, 99, 116, 132, 177, 99, 97, 99, 104, 101, 66,
                        121, 68, 101, 118, 105, 99, 101, 84, 121, 112, 101, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 179, 99, 97, 99, 104, 101,
                        68, 101, 99, 101, 112, 116, 105, 111, 110, 65, 114, 109, 111, 114, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 169, 99,
                        117, 115, 116, 111, 109, 75, 101, 121, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 133, 166, 99, 111, 111, 107,
                        105, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98,
                        106, 101, 99, 116, 130, 174, 99, 104, 101, 99, 107, 80, 114, 101, 115, 101,
                        110, 99, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165,
                        65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 168, 105, 110, 99,
                        108, 117, 100, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 166,
                        104, 101, 97, 100, 101, 114, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        129, 166, 79, 98, 106, 101, 99, 116, 131, 174, 99, 104, 101, 99, 107, 80,
                        114, 101, 115, 101, 110, 99, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110,
                        103, 173, 101, 120, 99, 108, 117, 100, 101, 79, 114, 105, 103, 105, 110,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 168,
                        105, 110, 99, 108, 117, 100, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110,
                        103, 164, 104, 111, 115, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 129, 166, 79, 98, 106, 101, 99, 116, 129, 168, 114, 101, 115, 111,
                        108, 118, 101, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66,
                        111, 111, 108, 171, 113, 117, 101, 114, 121, 83, 116, 114, 105, 110, 103,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101,
                        99, 116, 130, 168, 101, 120, 99, 108, 117, 100, 101, 115, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83,
                        116, 114, 105, 110, 103, 168, 105, 110, 99, 108, 117, 100, 101, 115, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121,
                        166, 83, 116, 114, 105, 110, 103, 164, 117, 115, 101, 114, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 131,
                        170, 100, 101, 118, 105, 99, 101, 84, 121, 112, 101, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 163, 103, 101, 111,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 164,
                        108, 97, 110, 103, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66,
                        111, 111, 108, 183, 105, 103, 110, 111, 114, 101, 81, 117, 101, 114, 121,
                        83, 116, 114, 105, 110, 103, 115, 79, 114, 100, 101, 114, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 167, 99, 111, 110,
                        116, 101, 110, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83,
                        116, 114, 105, 110, 103, 171, 99, 111, 110, 116, 101, 110, 116, 84, 121,
                        112, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114,
                        105, 110, 103, 172, 99, 111, 111, 107, 105, 101, 70, 105, 101, 108, 100,
                        115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114,
                        97, 121, 166, 83, 116, 114, 105, 110, 103, 171, 100, 105, 115, 97, 98, 108,
                        101, 65, 112, 112, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164,
                        66, 111, 111, 108, 174, 100, 105, 115, 97, 98, 108, 101, 82, 97, 105, 108,
                        103, 117, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111,
                        111, 108, 172, 100, 105, 115, 97, 98, 108, 101, 90, 97, 114, 97, 122, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 167, 101,
                        100, 103, 101, 84, 116, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        129, 166, 79, 98, 106, 101, 99, 116, 131, 167, 100, 101, 102, 97, 117, 108,
                        116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 164,
                        109, 111, 100, 101, 166, 83, 116, 114, 105, 110, 103, 174, 115, 116, 97,
                        116, 117, 115, 67, 111, 100, 101, 84, 116, 108, 115, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79,
                        98, 106, 101, 99, 116, 131, 170, 115, 116, 97, 116, 117, 115, 67, 111, 100,
                        101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 176,
                        115, 116, 97, 116, 117, 115, 67, 111, 100, 101, 82, 97, 110, 103, 101, 115,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 164, 102, 114, 111, 109,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 162, 116,
                        111, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 165,
                        118, 97, 108, 117, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163,
                        73, 110, 116, 176, 101, 109, 97, 105, 108, 79, 98, 102, 117, 115, 99, 97,
                        116, 105, 111, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66,
                        111, 111, 108, 168, 102, 114, 111, 109, 76, 105, 115, 116, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 130,
                        163, 107, 101, 121, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83,
                        116, 114, 105, 110, 103, 164, 110, 97, 109, 101, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 169, 102, 114,
                        111, 109, 86, 97, 108, 117, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 129, 166, 79, 98, 106, 101, 99, 116, 131, 179, 112, 114, 101, 115,
                        101, 114, 118, 101, 81, 117, 101, 114, 121, 83, 116, 114, 105, 110, 103,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 170,
                        115, 116, 97, 116, 117, 115, 67, 111, 100, 101, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 163, 73, 110, 116, 169, 116, 97, 114, 103, 101, 116,
                        85, 114, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79,
                        98, 106, 101, 99, 116, 130, 170, 101, 120, 112, 114, 101, 115, 115, 105,
                        111, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114,
                        105, 110, 103, 165, 118, 97, 108, 117, 101, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 167, 104, 101, 97, 100,
                        101, 114, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65,
                        114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 132, 170, 101, 120,
                        112, 114, 101, 115, 115, 105, 111, 110, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 164, 110, 97, 109, 101,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103, 169, 111, 112, 101, 114, 97, 116, 105, 111, 110, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 165, 118, 97,
                        108, 117, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116,
                        114, 105, 110, 103, 170, 104, 111, 115, 116, 72, 101, 97, 100, 101, 114,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103, 177, 104, 111, 116, 108, 105, 110, 107, 80, 114, 111, 116, 101, 99,
                        116, 105, 111, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66,
                        111, 111, 108, 162, 105, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 166, 83, 116, 114, 105, 110, 103, 169, 105, 110, 99, 114, 101, 109,
                        101, 110, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110,
                        116, 171, 109, 97, 116, 99, 104, 101, 100, 68, 97, 116, 97, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 129,
                        169, 112, 117, 98, 108, 105, 99, 75, 101, 121, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 166, 109, 105, 114, 97,
                        103, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111,
                        108, 183, 111, 112, 112, 111, 114, 116, 117, 110, 105, 115, 116, 105, 99,
                        69, 110, 99, 114, 121, 112, 116, 105, 111, 110, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 166, 111, 114, 105, 103,
                        105, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98,
                        106, 101, 99, 116, 130, 164, 104, 111, 115, 116, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 164, 112, 111,
                        114, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116,
                        178, 111, 114, 105, 103, 105, 110, 67, 97, 99, 104, 101, 67, 111, 110, 116,
                        114, 111, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111,
                        111, 108, 183, 111, 114, 105, 103, 105, 110, 69, 114, 114, 111, 114, 80,
                        97, 103, 101, 80, 97, 115, 115, 116, 104, 114, 117, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 169, 111, 118, 101, 114,
                        114, 105, 100, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        129, 166, 79, 98, 106, 101, 99, 116, 133, 166, 97, 99, 116, 105, 111, 110,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103, 170, 99, 97, 116, 101, 103, 111, 114, 105, 101, 115, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166,
                        79, 98, 106, 101, 99, 116, 131, 166, 97, 99, 116, 105, 111, 110, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 168,
                        99, 97, 116, 101, 103, 111, 114, 121, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 166, 83, 116, 114, 105, 110, 103, 167, 101, 110, 97, 98, 108,
                        101, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111,
                        108, 167, 101, 110, 97, 98, 108, 101, 100, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 164, 66, 111, 111, 108, 165, 114, 117, 108, 101, 115, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121,
                        129, 166, 79, 98, 106, 101, 99, 116, 133, 166, 97, 99, 116, 105, 111, 110,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103, 167, 101, 110, 97, 98, 108, 101, 100, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 164, 66, 111, 111, 108, 162, 105, 100, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 174, 115, 99,
                        111, 114, 101, 84, 104, 114, 101, 115, 104, 111, 108, 100, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 176, 115, 101, 110,
                        115, 105, 116, 105, 118, 105, 116, 121, 76, 101, 118, 101, 108, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 176,
                        115, 101, 110, 115, 105, 116, 105, 118, 105, 116, 121, 76, 101, 118, 101,
                        108, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105,
                        110, 103, 166, 112, 104, 97, 115, 101, 115, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105,
                        110, 103, 166, 112, 111, 108, 105, 115, 104, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 168, 112, 114, 111,
                        100, 117, 99, 116, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129,
                        165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 171, 114,
                        101, 97, 100, 84, 105, 109, 101, 111, 117, 116, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 163, 73, 110, 116, 173, 114, 101, 113, 117, 101,
                        115, 116, 70, 105, 101, 108, 100, 115, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110,
                        103, 178, 114, 101, 115, 112, 101, 99, 116, 83, 116, 114, 111, 110, 103,
                        69, 116, 97, 103, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164,
                        66, 111, 111, 108, 174, 114, 101, 115, 112, 111, 110, 115, 101, 70, 105,
                        101, 108, 100, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129,
                        165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 169, 114,
                        101, 115, 112, 111, 110, 115, 101, 115, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101,
                        99, 116, 131, 167, 99, 111, 110, 116, 101, 110, 116, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 171, 99, 111,
                        110, 116, 101, 110, 116, 84, 121, 112, 101, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 170, 115, 116, 97, 116,
                        117, 115, 67, 111, 100, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        163, 73, 110, 116, 172, 114, 111, 99, 107, 101, 116, 76, 111, 97, 100, 101,
                        114, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                        165, 114, 117, 108, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106,
                        101, 99, 116, 166, 83, 116, 114, 105, 110, 103, 167, 114, 117, 108, 101,
                        115, 101, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116,
                        114, 105, 110, 103, 168, 114, 117, 108, 101, 115, 101, 116, 115, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166,
                        83, 116, 114, 105, 110, 103, 173, 115, 101, 99, 117, 114, 105, 116, 121,
                        76, 101, 118, 101, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166,
                        83, 116, 114, 105, 110, 103, 170, 115, 101, 114, 118, 101, 83, 116, 97,
                        108, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98,
                        106, 101, 99, 116, 129, 185, 100, 105, 115, 97, 98, 108, 101, 83, 116, 97,
                        108, 101, 87, 104, 105, 108, 101, 85, 112, 100, 97, 116, 105, 110, 103,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 178,
                        115, 101, 114, 118, 101, 114, 83, 105, 100, 101, 69, 120, 99, 108, 117,
                        100, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111,
                        111, 108, 163, 115, 110, 105, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 129, 166, 79, 98, 106, 101, 99, 116, 129, 165, 118, 97, 108, 117, 101,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103, 163, 115, 115, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        166, 83, 116, 114, 105, 110, 103, 170, 115, 116, 97, 116, 117, 115, 67,
                        111, 100, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110,
                        116, 163, 115, 120, 103, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        164, 66, 111, 111, 108, 163, 117, 114, 105, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 131, 166, 111, 114,
                        105, 103, 105, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66,
                        111, 111, 108, 164, 112, 97, 116, 104, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 130, 170, 101, 120, 112,
                        114, 101, 115, 115, 105, 111, 110, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 166, 83, 116, 114, 105, 110, 103, 165, 118, 97, 108, 117, 101,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103, 165, 113, 117, 101, 114, 121, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 130, 170, 101, 120, 112,
                        114, 101, 115, 115, 105, 111, 110, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 166, 83, 116, 114, 105, 110, 103, 165, 118, 97, 108, 117, 101,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103, 167, 118, 101, 114, 115, 105, 111, 110, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 171, 100, 101, 115, 99,
                        114, 105, 112, 116, 105, 111, 110, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 166, 83, 116, 114, 105, 110, 103, 167, 101, 110, 97, 98, 108,
                        101, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111,
                        108, 182, 101, 120, 112, 111, 115, 101, 100, 67, 114, 101, 100, 101, 110,
                        116, 105, 97, 108, 67, 104, 101, 99, 107, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 130, 178, 112, 97, 115,
                        115, 119, 111, 114, 100, 69, 120, 112, 114, 101, 115, 115, 105, 111, 110,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103, 178, 117, 115, 101, 114, 110, 97, 109, 101, 69, 120, 112, 114, 101,
                        115, 115, 105, 111, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        166, 83, 116, 114, 105, 110, 103, 170, 101, 120, 112, 114, 101, 115, 115,
                        105, 111, 110, 166, 83, 116, 114, 105, 110, 103, 162, 105, 100, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 171,
                        108, 97, 115, 116, 85, 112, 100, 97, 116, 101, 100, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 167, 108, 111,
                        103, 103, 105, 110, 103, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        129, 166, 79, 98, 106, 101, 99, 116, 129, 167, 101, 110, 97, 98, 108, 101,
                        100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                        169, 114, 97, 116, 101, 108, 105, 109, 105, 116, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 136, 175, 99,
                        104, 97, 114, 97, 99, 116, 101, 114, 105, 115, 116, 105, 99, 115, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166,
                        83, 116, 114, 105, 110, 103, 178, 99, 111, 117, 110, 116, 105, 110, 103,
                        69, 120, 112, 114, 101, 115, 115, 105, 111, 110, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 177, 109, 105,
                        116, 105, 103, 97, 116, 105, 111, 110, 84, 105, 109, 101, 111, 117, 116,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 166, 112,
                        101, 114, 105, 111, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        163, 73, 110, 116, 177, 114, 101, 113, 117, 101, 115, 116, 115, 80, 101,
                        114, 80, 101, 114, 105, 111, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 163, 73, 110, 116, 176, 114, 101, 113, 117, 101, 115, 116, 115, 84,
                        111, 79, 114, 105, 103, 105, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 164, 66, 111, 111, 108, 174, 115, 99, 111, 114, 101, 80, 101, 114, 80,
                        101, 114, 105, 111, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        163, 73, 110, 116, 183, 115, 99, 111, 114, 101, 82, 101, 115, 112, 111,
                        110, 115, 101, 72, 101, 97, 100, 101, 114, 78, 97, 109, 101, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 163,
                        114, 101, 102, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116,
                        114, 105, 110, 103, 167, 118, 101, 114, 115, 105, 111, 110, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
            ],
        };

        let o = register(&request);

        ruleset::Res {
            account_id: o.get_field("accountId", false),
            description: o.get_field("description", true),
            kind: o.get_field("kind", true),
            name: o.get_field("name", true),
            phase: o.get_field("phase", true),
            rules: o.get_field("rules", false),
            zone_id: o.get_field("zoneId", false),
        }
    }
}
impl spectrum_application::Guest for Component {
    fn invoke(name: String, args: spectrum_application::Args) -> spectrum_application::Res {
        wasm_common::setup_logger();
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
                    schema: vec![164, 66, 111, 111, 108],
                },
                ResultField {
                    name: "dns".into(),
                    schema: vec![
                        129, 166, 79, 98, 106, 101, 99, 116, 130, 164, 110, 97, 109, 101, 166, 83,
                        116, 114, 105, 110, 103, 164, 116, 121, 112, 101, 166, 83, 116, 114, 105,
                        110, 103,
                    ],
                },
                ResultField {
                    name: "edgeIps".into(),
                    schema: vec![
                        129, 166, 79, 98, 106, 101, 99, 116, 131, 172, 99, 111, 110, 110, 101, 99,
                        116, 105, 118, 105, 116, 121, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 166, 83, 116, 114, 105, 110, 103, 163, 105, 112, 115, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83,
                        116, 114, 105, 110, 103, 164, 116, 121, 112, 101, 166, 83, 116, 114, 105,
                        110, 103,
                    ],
                },
                ResultField {
                    name: "ipFirewall".into(),
                    schema: vec![164, 66, 111, 111, 108],
                },
                ResultField {
                    name: "originDirects".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "originDns".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101,
                        99, 116, 129, 164, 110, 97, 109, 101, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "originPort".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "originPortRange".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101,
                        99, 116, 130, 163, 101, 110, 100, 163, 73, 110, 116, 165, 115, 116, 97,
                        114, 116, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "protocol".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "proxyProtocol".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "tls".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "trafficType".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        spectrum_application::Res {
            argo_smart_routing: o.get_field("argoSmartRouting", true),
            dns: o.get_field("dns", true),
            edge_ips: o.get_field("edgeIps", true),
            ip_firewall: o.get_field("ipFirewall", true),
            origin_directs: o.get_field("originDirects", false),
            origin_dns: o.get_field("originDns", false),
            origin_port: o.get_field("originPort", false),
            origin_port_range: o.get_field("originPortRange", false),
            protocol: o.get_field("protocol", true),
            proxy_protocol: o.get_field("proxyProtocol", true),
            tls: o.get_field("tls", true),
            traffic_type: o.get_field("trafficType", true),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl split_tunnel::Guest for Component {
    fn invoke(name: String, args: split_tunnel::Args) -> split_tunnel::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "mode".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "policyId".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "tunnels".into(),
                    schema: vec![
                        129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 131,
                        167, 97, 100, 100, 114, 101, 115, 115, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 166, 83, 116, 114, 105, 110, 103, 171, 100, 101, 115, 99, 114,
                        105, 112, 116, 105, 111, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 166, 83, 116, 114, 105, 110, 103, 164, 104, 111, 115, 116, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
            ],
        };

        let o = register(&request);

        split_tunnel::Res {
            account_id: o.get_field("accountId", true),
            mode: o.get_field("mode", true),
            policy_id: o.get_field("policyId", false),
            tunnels: o.get_field("tunnels", true),
        }
    }
}
impl static_route::Guest for Component {
    fn invoke(name: String, args: static_route::Args) -> static_route::Res {
        wasm_common::setup_logger();
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
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "coloNames".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "coloRegions".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "description".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "nexthop".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "prefix".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "priority".into(),
                    schema: vec![163, 73, 110, 116],
                },
                ResultField {
                    name: "weight".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116,
                    ],
                },
            ],
        };

        let o = register(&request);

        static_route::Res {
            account_id: o.get_field("accountId", false),
            colo_names: o.get_field("coloNames", false),
            colo_regions: o.get_field("coloRegions", false),
            description: o.get_field("description", false),
            nexthop: o.get_field("nexthop", true),
            prefix: o.get_field("prefix", true),
            priority: o.get_field("priority", true),
            weight: o.get_field("weight", false),
        }
    }
}
impl teams_account::Guest for Component {
    fn invoke(name: String, args: teams_account::Args) -> teams_account::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "activityLogEnabled".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "antivirus".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101,
                        99, 116, 132, 180, 101, 110, 97, 98, 108, 101, 100, 68, 111, 119, 110, 108,
                        111, 97, 100, 80, 104, 97, 115, 101, 164, 66, 111, 111, 108, 178, 101, 110,
                        97, 98, 108, 101, 100, 85, 112, 108, 111, 97, 100, 80, 104, 97, 115, 101,
                        164, 66, 111, 111, 108, 170, 102, 97, 105, 108, 67, 108, 111, 115, 101,
                        100, 164, 66, 111, 111, 108, 180, 110, 111, 116, 105, 102, 105, 99, 97,
                        116, 105, 111, 110, 83, 101, 116, 116, 105, 110, 103, 115, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 131,
                        167, 101, 110, 97, 98, 108, 101, 100, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 164, 66, 111, 111, 108, 167, 109, 101, 115, 115, 97, 103, 101,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103, 170, 115, 117, 112, 112, 111, 114, 116, 85, 114, 108, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "blockPage".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101,
                        99, 116, 136, 175, 98, 97, 99, 107, 103, 114, 111, 117, 110, 100, 67, 111,
                        108, 111, 114, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116,
                        114, 105, 110, 103, 167, 101, 110, 97, 98, 108, 101, 100, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 170, 102, 111,
                        111, 116, 101, 114, 84, 101, 120, 116, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 166, 83, 116, 114, 105, 110, 103, 170, 104, 101, 97, 100, 101,
                        114, 84, 101, 120, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166,
                        83, 116, 114, 105, 110, 103, 168, 108, 111, 103, 111, 80, 97, 116, 104,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103, 173, 109, 97, 105, 108, 116, 111, 65, 100, 100, 114, 101, 115, 115,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103, 173, 109, 97, 105, 108, 116, 111, 83, 117, 98, 106, 101, 99, 116, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                        164, 110, 97, 109, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166,
                        83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "bodyScanning".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101,
                        99, 116, 129, 174, 105, 110, 115, 112, 101, 99, 116, 105, 111, 110, 77,
                        111, 100, 101, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "extendedEmailMatching".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101,
                        99, 116, 129, 167, 101, 110, 97, 98, 108, 101, 100, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "fips".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101,
                        99, 116, 129, 163, 116, 108, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "logging".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101,
                        99, 116, 130, 169, 114, 101, 100, 97, 99, 116, 80, 105, 105, 164, 66, 111,
                        111, 108, 178, 115, 101, 116, 116, 105, 110, 103, 115, 66, 121, 82, 117,
                        108, 101, 84, 121, 112, 101, 129, 166, 79, 98, 106, 101, 99, 116, 131, 163,
                        100, 110, 115, 129, 166, 79, 98, 106, 101, 99, 116, 130, 166, 108, 111,
                        103, 65, 108, 108, 164, 66, 111, 111, 108, 169, 108, 111, 103, 66, 108,
                        111, 99, 107, 115, 164, 66, 111, 111, 108, 164, 104, 116, 116, 112, 129,
                        166, 79, 98, 106, 101, 99, 116, 130, 166, 108, 111, 103, 65, 108, 108, 164,
                        66, 111, 111, 108, 169, 108, 111, 103, 66, 108, 111, 99, 107, 115, 164, 66,
                        111, 111, 108, 162, 108, 52, 129, 166, 79, 98, 106, 101, 99, 116, 130, 166,
                        108, 111, 103, 65, 108, 108, 164, 66, 111, 111, 108, 169, 108, 111, 103,
                        66, 108, 111, 99, 107, 115, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "nonIdentityBrowserIsolationEnabled".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "payloadLog".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101,
                        99, 116, 129, 169, 112, 117, 98, 108, 105, 99, 75, 101, 121, 166, 83, 116,
                        114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "protocolDetectionEnabled".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "proxy".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101,
                        99, 116, 131, 166, 114, 111, 111, 116, 67, 97, 164, 66, 111, 111, 108, 163,
                        116, 99, 112, 164, 66, 111, 111, 108, 163, 117, 100, 112, 164, 66, 111,
                        111, 108,
                    ],
                },
                ResultField {
                    name: "sshSessionLog".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101,
                        99, 116, 129, 169, 112, 117, 98, 108, 105, 99, 75, 101, 121, 166, 83, 116,
                        114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "tlsDecryptEnabled".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "urlBrowserIsolationEnabled".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
            ],
        };

        let o = register(&request);

        teams_account::Res {
            account_id: o.get_field("accountId", true),
            activity_log_enabled: o.get_field("activityLogEnabled", false),
            antivirus: o.get_field("antivirus", false),
            block_page: o.get_field("blockPage", false),
            body_scanning: o.get_field("bodyScanning", false),
            extended_email_matching: o.get_field("extendedEmailMatching", false),
            fips: o.get_field("fips", false),
            logging: o.get_field("logging", false),
            non_identity_browser_isolation_enabled: o
                .get_field("nonIdentityBrowserIsolationEnabled", false),
            payload_log: o.get_field("payloadLog", false),
            protocol_detection_enabled: o.get_field("protocolDetectionEnabled", false),
            proxy: o.get_field("proxy", false),
            ssh_session_log: o.get_field("sshSessionLog", false),
            tls_decrypt_enabled: o.get_field("tlsDecryptEnabled", false),
            url_browser_isolation_enabled: o.get_field("urlBrowserIsolationEnabled", false),
        }
    }
}
impl teams_list::Guest for Component {
    fn invoke(name: String, args: teams_list::Args) -> teams_list::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "description".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "items".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "type".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        teams_list::Res {
            account_id: o.get_field("accountId", true),
            description: o.get_field("description", false),
            items: o.get_field("items", false),
            name: o.get_field("name", true),
            type_: o.get_field("type", true),
        }
    }
}
impl teams_location::Guest for Component {
    fn invoke(name: String, args: teams_location::Args) -> teams_location::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "anonymizedLogsEnabled".into(),
                    schema: vec![164, 66, 111, 111, 108],
                },
                ResultField {
                    name: "clientDefault".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "dohSubdomain".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "ip".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "ipv4Destination".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "networks".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 162, 105, 100, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 167,
                        110, 101, 116, 119, 111, 114, 107, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "policyIds".into(),
                    schema: vec![
                        129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
            ],
        };

        let o = register(&request);

        teams_location::Res {
            account_id: o.get_field("accountId", true),
            anonymized_logs_enabled: o.get_field("anonymizedLogsEnabled", true),
            client_default: o.get_field("clientDefault", false),
            doh_subdomain: o.get_field("dohSubdomain", true),
            ip: o.get_field("ip", true),
            ipv4_destination: o.get_field("ipv4Destination", true),
            name: o.get_field("name", true),
            networks: o.get_field("networks", false),
            policy_ids: o.get_field("policyIds", true),
        }
    }
}
impl teams_proxy_endpoint::Guest for Component {
    fn invoke(name: String, args: teams_proxy_endpoint::Args) -> teams_proxy_endpoint::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "ips".into(),
                    schema: vec![
                        129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "subdomain".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        teams_proxy_endpoint::Res {
            account_id: o.get_field("accountId", true),
            ips: o.get_field("ips", true),
            name: o.get_field("name", true),
            subdomain: o.get_field("subdomain", true),
        }
    }
}
impl teams_rule::Guest for Component {
    fn invoke(name: String, args: teams_rule::Args) -> teams_rule::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "action".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "description".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "devicePosture".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "enabled".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "filters".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "identity".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "precedence".into(),
                    schema: vec![163, 73, 110, 116],
                },
                ResultField {
                    name: "ruleSettings".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101,
                        99, 116, 222, 0, 17, 170, 97, 100, 100, 72, 101, 97, 100, 101, 114, 115,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 176, 83, 105, 110, 103,
                        108, 101, 84, 121, 112, 101, 79, 98, 106, 101, 99, 116, 166, 83, 116, 114,
                        105, 110, 103, 176, 97, 108, 108, 111, 119, 67, 104, 105, 108, 100, 66,
                        121, 112, 97, 115, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164,
                        66, 111, 111, 108, 168, 97, 117, 100, 105, 116, 83, 115, 104, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 129,
                        174, 99, 111, 109, 109, 97, 110, 100, 76, 111, 103, 103, 105, 110, 103,
                        164, 66, 111, 111, 108, 177, 98, 105, 115, 111, 65, 100, 109, 105, 110, 67,
                        111, 110, 116, 114, 111, 108, 115, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 133, 176, 100, 105, 115, 97,
                        98, 108, 101, 67, 111, 112, 121, 80, 97, 115, 116, 101, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 175, 100, 105, 115, 97,
                        98, 108, 101, 68, 111, 119, 110, 108, 111, 97, 100, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 175, 100, 105, 115, 97, 98,
                        108, 101, 75, 101, 121, 98, 111, 97, 114, 100, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 164, 66, 111, 111, 108, 175, 100, 105, 115, 97, 98, 108,
                        101, 80, 114, 105, 110, 116, 105, 110, 103, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 164, 66, 111, 111, 108, 173, 100, 105, 115, 97, 98, 108,
                        101, 85, 112, 108, 111, 97, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 164, 66, 111, 111, 108, 176, 98, 108, 111, 99, 107, 80, 97, 103, 101,
                        69, 110, 97, 98, 108, 101, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 164, 66, 111, 111, 108, 175, 98, 108, 111, 99, 107, 80, 97, 103, 101,
                        82, 101, 97, 115, 111, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        166, 83, 116, 114, 105, 110, 103, 176, 98, 121, 112, 97, 115, 115, 80, 97,
                        114, 101, 110, 116, 82, 117, 108, 101, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 164, 66, 111, 111, 108, 172, 99, 104, 101, 99, 107, 83, 101, 115,
                        115, 105, 111, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129,
                        166, 79, 98, 106, 101, 99, 116, 130, 168, 100, 117, 114, 97, 116, 105, 111,
                        110, 166, 83, 116, 114, 105, 110, 103, 167, 101, 110, 102, 111, 114, 99,
                        101, 164, 66, 111, 111, 108, 166, 101, 103, 114, 101, 115, 115, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116,
                        131, 164, 105, 112, 118, 52, 166, 83, 116, 114, 105, 110, 103, 172, 105,
                        112, 118, 52, 70, 97, 108, 108, 98, 97, 99, 107, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 164, 105, 112,
                        118, 54, 166, 83, 116, 114, 105, 110, 103, 191, 105, 110, 115, 101, 99,
                        117, 114, 101, 68, 105, 115, 97, 98, 108, 101, 68, 110, 115, 115, 101, 99,
                        86, 97, 108, 105, 100, 97, 116, 105, 111, 110, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 164, 66, 111, 111, 108, 172, 105, 112, 67, 97, 116, 101,
                        103, 111, 114, 105, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 164, 66, 111, 111, 108, 170, 108, 52, 111, 118, 101, 114, 114, 105,
                        100, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98,
                        106, 101, 99, 116, 130, 162, 105, 112, 166, 83, 116, 114, 105, 110, 103,
                        164, 112, 111, 114, 116, 163, 73, 110, 116, 180, 110, 111, 116, 105, 102,
                        105, 99, 97, 116, 105, 111, 110, 83, 101, 116, 116, 105, 110, 103, 115,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101,
                        99, 116, 131, 167, 101, 110, 97, 98, 108, 101, 100, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 167, 109, 101, 115, 115, 97,
                        103, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114,
                        105, 110, 103, 170, 115, 117, 112, 112, 111, 114, 116, 85, 114, 108, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                        172, 111, 118, 101, 114, 114, 105, 100, 101, 72, 111, 115, 116, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 171,
                        111, 118, 101, 114, 114, 105, 100, 101, 73, 112, 115, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116,
                        114, 105, 110, 103, 170, 112, 97, 121, 108, 111, 97, 100, 76, 111, 103,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101,
                        99, 116, 129, 167, 101, 110, 97, 98, 108, 101, 100, 164, 66, 111, 111, 108,
                        173, 117, 110, 116, 114, 117, 115, 116, 101, 100, 67, 101, 114, 116, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99,
                        116, 129, 166, 97, 99, 116, 105, 111, 110, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "traffic".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "version".into(),
                    schema: vec![163, 73, 110, 116],
                },
            ],
        };

        let o = register(&request);

        teams_rule::Res {
            account_id: o.get_field("accountId", true),
            action: o.get_field("action", true),
            description: o.get_field("description", true),
            device_posture: o.get_field("devicePosture", false),
            enabled: o.get_field("enabled", false),
            filters: o.get_field("filters", false),
            identity: o.get_field("identity", false),
            name: o.get_field("name", true),
            precedence: o.get_field("precedence", true),
            rule_settings: o.get_field("ruleSettings", false),
            traffic: o.get_field("traffic", false),
            version: o.get_field("version", true),
        }
    }
}
impl tiered_cache::Guest for Component {
    fn invoke(name: String, args: tiered_cache::Args) -> tiered_cache::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        tiered_cache::Res {
            cache_type: o.get_field("cacheType", true),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl total_tls::Guest for Component {
    fn invoke(name: String, args: total_tls::Args) -> total_tls::Res {
        wasm_common::setup_logger();
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
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "enabled".into(),
                    schema: vec![164, 66, 111, 111, 108],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        total_tls::Res {
            certificate_authority: o.get_field("certificateAuthority", false),
            enabled: o.get_field("enabled", true),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl tunnel::Guest for Component {
    fn invoke(name: String, args: tunnel::Args) -> tunnel::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "cname".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "configSrc".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "secret".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "tunnelToken".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        tunnel::Res {
            account_id: o.get_field("accountId", true),
            cname: o.get_field("cname", true),
            config_src: o.get_field("configSrc", false),
            name: o.get_field("name", true),
            secret: o.get_field("secret", true),
            tunnel_token: o.get_field("tunnelToken", true),
        }
    }
}
impl tunnel_config::Guest for Component {
    fn invoke(name: String, args: tunnel_config::Args) -> tunnel_config::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "config".into(),
                    schema: vec![
                        129, 166, 79, 98, 106, 101, 99, 116, 131, 172, 105, 110, 103, 114, 101,
                        115, 115, 82, 117, 108, 101, 115, 129, 165, 65, 114, 114, 97, 121, 129,
                        166, 79, 98, 106, 101, 99, 116, 132, 168, 104, 111, 115, 116, 110, 97, 109,
                        101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105,
                        110, 103, 173, 111, 114, 105, 103, 105, 110, 82, 101, 113, 117, 101, 115,
                        116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106,
                        101, 99, 116, 222, 0, 18, 166, 97, 99, 99, 101, 115, 115, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 131,
                        167, 97, 117, 100, 84, 97, 103, 115, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110,
                        103, 168, 114, 101, 113, 117, 105, 114, 101, 100, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 168, 116, 101, 97, 109, 78,
                        97, 109, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116,
                        114, 105, 110, 103, 171, 98, 97, 115, 116, 105, 111, 110, 77, 111, 100,
                        101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                        166, 99, 97, 80, 111, 111, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 166, 83, 116, 114, 105, 110, 103, 174, 99, 111, 110, 110, 101, 99,
                        116, 84, 105, 109, 101, 111, 117, 116, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 166, 83, 116, 114, 105, 110, 103, 182, 100, 105, 115, 97, 98,
                        108, 101, 67, 104, 117, 110, 107, 101, 100, 69, 110, 99, 111, 100, 105,
                        110, 103, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111,
                        108, 171, 104, 116, 116, 112, 50, 79, 114, 105, 103, 105, 110, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 174, 104, 116,
                        116, 112, 72, 111, 115, 116, 72, 101, 97, 100, 101, 114, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 167, 105,
                        112, 82, 117, 108, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 131,
                        165, 97, 108, 108, 111, 119, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        164, 66, 111, 111, 108, 165, 112, 111, 114, 116, 115, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 163, 73, 110,
                        116, 166, 112, 114, 101, 102, 105, 120, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 180, 107, 101, 101, 112,
                        65, 108, 105, 118, 101, 67, 111, 110, 110, 101, 99, 116, 105, 111, 110,
                        115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 176,
                        107, 101, 101, 112, 65, 108, 105, 118, 101, 84, 105, 109, 101, 111, 117,
                        116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105,
                        110, 103, 175, 110, 111, 72, 97, 112, 112, 121, 69, 121, 101, 98, 97, 108,
                        108, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111,
                        108, 171, 110, 111, 84, 108, 115, 86, 101, 114, 105, 102, 121, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 176, 111, 114,
                        105, 103, 105, 110, 83, 101, 114, 118, 101, 114, 78, 97, 109, 101, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                        172, 112, 114, 111, 120, 121, 65, 100, 100, 114, 101, 115, 115, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 169,
                        112, 114, 111, 120, 121, 80, 111, 114, 116, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 163, 73, 110, 116, 169, 112, 114, 111, 120, 121, 84, 121,
                        112, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114,
                        105, 110, 103, 172, 116, 99, 112, 75, 101, 101, 112, 65, 108, 105, 118,
                        101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105,
                        110, 103, 170, 116, 108, 115, 84, 105, 109, 101, 111, 117, 116, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 164,
                        112, 97, 116, 104, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83,
                        116, 114, 105, 110, 103, 167, 115, 101, 114, 118, 105, 99, 101, 166, 83,
                        116, 114, 105, 110, 103, 173, 111, 114, 105, 103, 105, 110, 82, 101, 113,
                        117, 101, 115, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129,
                        166, 79, 98, 106, 101, 99, 116, 222, 0, 18, 166, 97, 99, 99, 101, 115, 115,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101,
                        99, 116, 131, 167, 97, 117, 100, 84, 97, 103, 115, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114,
                        105, 110, 103, 168, 114, 101, 113, 117, 105, 114, 101, 100, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 168, 116, 101, 97,
                        109, 78, 97, 109, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166,
                        83, 116, 114, 105, 110, 103, 171, 98, 97, 115, 116, 105, 111, 110, 77, 111,
                        100, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111,
                        108, 166, 99, 97, 80, 111, 111, 108, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 166, 83, 116, 114, 105, 110, 103, 174, 99, 111, 110, 110, 101,
                        99, 116, 84, 105, 109, 101, 111, 117, 116, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 182, 100, 105, 115, 97, 98,
                        108, 101, 67, 104, 117, 110, 107, 101, 100, 69, 110, 99, 111, 100, 105,
                        110, 103, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111,
                        108, 171, 104, 116, 116, 112, 50, 79, 114, 105, 103, 105, 110, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 174, 104, 116,
                        116, 112, 72, 111, 115, 116, 72, 101, 97, 100, 101, 114, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 167, 105,
                        112, 82, 117, 108, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 131,
                        165, 97, 108, 108, 111, 119, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        164, 66, 111, 111, 108, 165, 112, 111, 114, 116, 115, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 163, 73, 110,
                        116, 166, 112, 114, 101, 102, 105, 120, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 180, 107, 101, 101, 112,
                        65, 108, 105, 118, 101, 67, 111, 110, 110, 101, 99, 116, 105, 111, 110,
                        115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 176,
                        107, 101, 101, 112, 65, 108, 105, 118, 101, 84, 105, 109, 101, 111, 117,
                        116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105,
                        110, 103, 175, 110, 111, 72, 97, 112, 112, 121, 69, 121, 101, 98, 97, 108,
                        108, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111,
                        108, 171, 110, 111, 84, 108, 115, 86, 101, 114, 105, 102, 121, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 176, 111, 114,
                        105, 103, 105, 110, 83, 101, 114, 118, 101, 114, 78, 97, 109, 101, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                        172, 112, 114, 111, 120, 121, 65, 100, 100, 114, 101, 115, 115, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 169,
                        112, 114, 111, 120, 121, 80, 111, 114, 116, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 163, 73, 110, 116, 169, 112, 114, 111, 120, 121, 84, 121,
                        112, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114,
                        105, 110, 103, 172, 116, 99, 112, 75, 101, 101, 112, 65, 108, 105, 118,
                        101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105,
                        110, 103, 170, 116, 108, 115, 84, 105, 109, 101, 111, 117, 116, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 171,
                        119, 97, 114, 112, 82, 111, 117, 116, 105, 110, 103, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 129, 167,
                        101, 110, 97, 98, 108, 101, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "tunnelId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        tunnel_config::Res {
            account_id: o.get_field("accountId", true),
            config: o.get_field("config", true),
            tunnel_id: o.get_field("tunnelId", true),
        }
    }
}
impl tunnel_route::Guest for Component {
    fn invoke(name: String, args: tunnel_route::Args) -> tunnel_route::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "comment".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "network".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "tunnelId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "virtualNetworkId".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
            ],
        };

        let o = register(&request);

        tunnel_route::Res {
            account_id: o.get_field("accountId", true),
            comment: o.get_field("comment", false),
            network: o.get_field("network", true),
            tunnel_id: o.get_field("tunnelId", true),
            virtual_network_id: o.get_field("virtualNetworkId", false),
        }
    }
}
impl tunnel_virtual_network::Guest for Component {
    fn invoke(name: String, args: tunnel_virtual_network::Args) -> tunnel_virtual_network::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "comment".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "isDefaultNetwork".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        tunnel_virtual_network::Res {
            account_id: o.get_field("accountId", true),
            comment: o.get_field("comment", false),
            is_default_network: o.get_field("isDefaultNetwork", false),
            name: o.get_field("name", true),
        }
    }
}
impl turnstile_widget::Guest for Component {
    fn invoke(name: String, args: turnstile_widget::Args) -> turnstile_widget::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "botFightMode".into(),
                    schema: vec![164, 66, 111, 111, 108],
                },
                ResultField {
                    name: "domains".into(),
                    schema: vec![
                        129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "mode".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "offlabel".into(),
                    schema: vec![164, 66, 111, 111, 108],
                },
                ResultField {
                    name: "region".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "secret".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        turnstile_widget::Res {
            account_id: o.get_field("accountId", true),
            bot_fight_mode: o.get_field("botFightMode", true),
            domains: o.get_field("domains", true),
            mode: o.get_field("mode", true),
            name: o.get_field("name", true),
            offlabel: o.get_field("offlabel", true),
            region: o.get_field("region", true),
            secret: o.get_field("secret", true),
        }
    }
}
impl url_normalization_settings::Guest for Component {
    fn invoke(
        name: String,
        args: url_normalization_settings::Args,
    ) -> url_normalization_settings::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "type".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        url_normalization_settings::Res {
            scope: o.get_field("scope", true),
            type_: o.get_field("type", true),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl user_agent_blocking_rule::Guest for Component {
    fn invoke(name: String, args: user_agent_blocking_rule::Args) -> user_agent_blocking_rule::Res {
        wasm_common::setup_logger();
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
                    schema: vec![
                        129, 166, 79, 98, 106, 101, 99, 116, 130, 166, 116, 97, 114, 103, 101, 116,
                        166, 83, 116, 114, 105, 110, 103, 165, 118, 97, 108, 117, 101, 166, 83,
                        116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "description".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "mode".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "paused".into(),
                    schema: vec![164, 66, 111, 111, 108],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        user_agent_blocking_rule::Res {
            configuration: o.get_field("configuration", true),
            description: o.get_field("description", true),
            mode: o.get_field("mode", true),
            paused: o.get_field("paused", true),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl waiting_room::Guest for Component {
    fn invoke(name: String, args: waiting_room::Args) -> waiting_room::Res {
        wasm_common::setup_logger();
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
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 164, 104, 111, 115, 116,
                        166, 83, 116, 114, 105, 110, 103, 164, 112, 97, 116, 104, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "cookieSuffix".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "customPageHtml".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "defaultTemplateLanguage".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "description".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "disableSessionRenewal".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "host".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "jsonResponseEnabled".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "newUsersPerMinute".into(),
                    schema: vec![163, 73, 110, 116],
                },
                ResultField {
                    name: "path".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "queueAll".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "queueingMethod".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "queueingStatusCode".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "sessionDuration".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "suspended".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "totalActiveUsers".into(),
                    schema: vec![163, 73, 110, 116],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        waiting_room::Res {
            additional_routes: o.get_field("additionalRoutes", false),
            cookie_suffix: o.get_field("cookieSuffix", false),
            custom_page_html: o.get_field("customPageHtml", false),
            default_template_language: o.get_field("defaultTemplateLanguage", false),
            description: o.get_field("description", false),
            disable_session_renewal: o.get_field("disableSessionRenewal", false),
            host: o.get_field("host", true),
            json_response_enabled: o.get_field("jsonResponseEnabled", false),
            name: o.get_field("name", true),
            new_users_per_minute: o.get_field("newUsersPerMinute", true),
            path: o.get_field("path", false),
            queue_all: o.get_field("queueAll", false),
            queueing_method: o.get_field("queueingMethod", false),
            queueing_status_code: o.get_field("queueingStatusCode", false),
            session_duration: o.get_field("sessionDuration", false),
            suspended: o.get_field("suspended", false),
            total_active_users: o.get_field("totalActiveUsers", true),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl waiting_room_event::Guest for Component {
    fn invoke(name: String, args: waiting_room_event::Args) -> waiting_room_event::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "customPageHtml".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "description".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "disableSessionRenewal".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "eventEndTime".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "eventStartTime".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "modifiedOn".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "newUsersPerMinute".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "prequeueStartTime".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "queueingMethod".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "sessionDuration".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "shuffleAtEventStart".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "suspended".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "totalActiveUsers".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "waitingRoomId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        waiting_room_event::Res {
            created_on: o.get_field("createdOn", true),
            custom_page_html: o.get_field("customPageHtml", false),
            description: o.get_field("description", false),
            disable_session_renewal: o.get_field("disableSessionRenewal", false),
            event_end_time: o.get_field("eventEndTime", true),
            event_start_time: o.get_field("eventStartTime", true),
            modified_on: o.get_field("modifiedOn", true),
            name: o.get_field("name", true),
            new_users_per_minute: o.get_field("newUsersPerMinute", false),
            prequeue_start_time: o.get_field("prequeueStartTime", false),
            queueing_method: o.get_field("queueingMethod", false),
            session_duration: o.get_field("sessionDuration", false),
            shuffle_at_event_start: o.get_field("shuffleAtEventStart", false),
            suspended: o.get_field("suspended", false),
            total_active_users: o.get_field("totalActiveUsers", false),
            waiting_room_id: o.get_field("waitingRoomId", true),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl waiting_room_rules::Guest for Component {
    fn invoke(name: String, args: waiting_room_rules::Args) -> waiting_room_rules::Res {
        wasm_common::setup_logger();
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
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 134, 166, 97, 99, 116, 105, 111,
                        110, 166, 83, 116, 114, 105, 110, 103, 171, 100, 101, 115, 99, 114, 105,
                        112, 116, 105, 111, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        166, 83, 116, 114, 105, 110, 103, 170, 101, 120, 112, 114, 101, 115, 115,
                        105, 111, 110, 166, 83, 116, 114, 105, 110, 103, 162, 105, 100, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 166,
                        115, 116, 97, 116, 117, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        166, 83, 116, 114, 105, 110, 103, 167, 118, 101, 114, 115, 105, 111, 110,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "waitingRoomId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        waiting_room_rules::Res {
            rules: o.get_field("rules", false),
            waiting_room_id: o.get_field("waitingRoomId", true),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl waiting_room_settings::Guest for Component {
    fn invoke(name: String, args: waiting_room_settings::Args) -> waiting_room_settings::Res {
        wasm_common::setup_logger();
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
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        waiting_room_settings::Res {
            search_engine_crawler_bypass: o.get_field("searchEngineCrawlerBypass", false),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl web3_hostname::Guest for Component {
    fn invoke(name: String, args: web3_hostname::Args) -> web3_hostname::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "description".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "dnslink".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "modifiedOn".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "status".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "target".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        web3_hostname::Res {
            created_on: o.get_field("createdOn", true),
            description: o.get_field("description", false),
            dnslink: o.get_field("dnslink", false),
            modified_on: o.get_field("modifiedOn", true),
            name: o.get_field("name", true),
            status: o.get_field("status", true),
            target: o.get_field("target", true),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl web_analytics_rule::Guest for Component {
    fn invoke(name: String, args: web_analytics_rule::Args) -> web_analytics_rule::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "host".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "inclusive".into(),
                    schema: vec![164, 66, 111, 111, 108],
                },
                ResultField {
                    name: "isPaused".into(),
                    schema: vec![164, 66, 111, 111, 108],
                },
                ResultField {
                    name: "paths".into(),
                    schema: vec![
                        129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "rulesetId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        web_analytics_rule::Res {
            account_id: o.get_field("accountId", true),
            host: o.get_field("host", true),
            inclusive: o.get_field("inclusive", true),
            is_paused: o.get_field("isPaused", true),
            paths: o.get_field("paths", true),
            ruleset_id: o.get_field("rulesetId", true),
        }
    }
}
impl web_analytics_site::Guest for Component {
    fn invoke(name: String, args: web_analytics_site::Args) -> web_analytics_site::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "autoInstall".into(),
                    schema: vec![164, 66, 111, 111, 108],
                },
                ResultField {
                    name: "host".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "rulesetId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "siteTag".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "siteToken".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "snippet".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "zoneTag".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
            ],
        };

        let o = register(&request);

        web_analytics_site::Res {
            account_id: o.get_field("accountId", true),
            auto_install: o.get_field("autoInstall", true),
            host: o.get_field("host", false),
            ruleset_id: o.get_field("rulesetId", true),
            site_tag: o.get_field("siteTag", true),
            site_token: o.get_field("siteToken", true),
            snippet: o.get_field("snippet", true),
            zone_tag: o.get_field("zoneTag", false),
        }
    }
}
impl worker_cron_trigger::Guest for Component {
    fn invoke(name: String, args: worker_cron_trigger::Args) -> worker_cron_trigger::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "schedules".into(),
                    schema: vec![
                        129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "scriptName".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        worker_cron_trigger::Res {
            account_id: o.get_field("accountId", true),
            schedules: o.get_field("schedules", true),
            script_name: o.get_field("scriptName", true),
        }
    }
}
impl worker_domain::Guest for Component {
    fn invoke(name: String, args: worker_domain::Args) -> worker_domain::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "environment".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "hostname".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "service".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        worker_domain::Res {
            account_id: o.get_field("accountId", true),
            environment: o.get_field("environment", false),
            hostname: o.get_field("hostname", true),
            service: o.get_field("service", true),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl worker_route::Guest for Component {
    fn invoke(name: String, args: worker_route::Args) -> worker_route::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "scriptName".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        worker_route::Res {
            pattern: o.get_field("pattern", true),
            script_name: o.get_field("scriptName", false),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl worker_script::Guest for Component {
    fn invoke(name: String, args: worker_script::Args) -> worker_script::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "analyticsEngineBindings".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 167, 100, 97, 116, 97, 115,
                        101, 116, 166, 83, 116, 114, 105, 110, 103, 164, 110, 97, 109, 101, 166,
                        83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "compatibilityDate".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "compatibilityFlags".into(),
                    schema: vec![
                        129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "content".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "d1DatabaseBindings".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 170, 100, 97, 116, 97, 98,
                        97, 115, 101, 73, 100, 166, 83, 116, 114, 105, 110, 103, 164, 110, 97, 109,
                        101, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "dispatchNamespace".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "kvNamespaceBindings".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 164, 110, 97, 109, 101, 166,
                        83, 116, 114, 105, 110, 103, 171, 110, 97, 109, 101, 115, 112, 97, 99, 101,
                        73, 100, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "logpush".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "module".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "placements".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 129, 164, 109, 111, 100, 101,
                        166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "plainTextBindings".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 164, 110, 97, 109, 101, 166,
                        83, 116, 114, 105, 110, 103, 164, 116, 101, 120, 116, 166, 83, 116, 114,
                        105, 110, 103,
                    ],
                },
                ResultField {
                    name: "queueBindings".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 167, 98, 105, 110, 100, 105,
                        110, 103, 166, 83, 116, 114, 105, 110, 103, 165, 113, 117, 101, 117, 101,
                        166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "r2BucketBindings".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 170, 98, 117, 99, 107, 101,
                        116, 78, 97, 109, 101, 166, 83, 116, 114, 105, 110, 103, 164, 110, 97, 109,
                        101, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "secretTextBindings".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 164, 110, 97, 109, 101, 166,
                        83, 116, 114, 105, 110, 103, 164, 116, 101, 120, 116, 166, 83, 116, 114,
                        105, 110, 103,
                    ],
                },
                ResultField {
                    name: "serviceBindings".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 131, 171, 101, 110, 118, 105,
                        114, 111, 110, 109, 101, 110, 116, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 166, 83, 116, 114, 105, 110, 103, 164, 110, 97, 109, 101, 166,
                        83, 116, 114, 105, 110, 103, 167, 115, 101, 114, 118, 105, 99, 101, 166,
                        83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "tags".into(),
                    schema: vec![
                        129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "webassemblyBindings".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 129, 166, 79, 98, 106, 101, 99, 116, 130, 166, 109, 111, 100, 117,
                        108, 101, 166, 83, 116, 114, 105, 110, 103, 164, 110, 97, 109, 101, 166,
                        83, 116, 114, 105, 110, 103,
                    ],
                },
            ],
        };

        let o = register(&request);

        worker_script::Res {
            account_id: o.get_field("accountId", true),
            analytics_engine_bindings: o.get_field("analyticsEngineBindings", false),
            compatibility_date: o.get_field("compatibilityDate", false),
            compatibility_flags: o.get_field("compatibilityFlags", true),
            content: o.get_field("content", true),
            d1_database_bindings: o.get_field("d1DatabaseBindings", false),
            dispatch_namespace: o.get_field("dispatchNamespace", false),
            kv_namespace_bindings: o.get_field("kvNamespaceBindings", false),
            logpush: o.get_field("logpush", false),
            module: o.get_field("module", false),
            name: o.get_field("name", true),
            placements: o.get_field("placements", false),
            plain_text_bindings: o.get_field("plainTextBindings", false),
            queue_bindings: o.get_field("queueBindings", false),
            r2_bucket_bindings: o.get_field("r2BucketBindings", false),
            secret_text_bindings: o.get_field("secretTextBindings", false),
            service_bindings: o.get_field("serviceBindings", false),
            tags: o.get_field("tags", true),
            webassembly_bindings: o.get_field("webassemblyBindings", false),
        }
    }
}
impl worker_secret::Guest for Component {
    fn invoke(name: String, args: worker_secret::Args) -> worker_secret::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "scriptName".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "secretText".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        worker_secret::Res {
            account_id: o.get_field("accountId", true),
            name: o.get_field("name", true),
            script_name: o.get_field("scriptName", true),
            secret_text: o.get_field("secretText", true),
        }
    }
}
impl workers_for_platforms_namespace::Guest for Component {
    fn invoke(
        name: String,
        args: workers_for_platforms_namespace::Args,
    ) -> workers_for_platforms_namespace::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "name".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        workers_for_platforms_namespace::Res {
            account_id: o.get_field("accountId", true),
            name: o.get_field("name", true),
        }
    }
}
impl workers_kv::Guest for Component {
    fn invoke(name: String, args: workers_kv::Args) -> workers_kv::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "key".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "namespaceId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "value".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        workers_kv::Res {
            account_id: o.get_field("accountId", true),
            key: o.get_field("key", true),
            namespace_id: o.get_field("namespaceId", true),
            value: o.get_field("value", true),
        }
    }
}
impl workers_kv_namespace::Guest for Component {
    fn invoke(name: String, args: workers_kv_namespace::Args) -> workers_kv_namespace::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "title".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        workers_kv_namespace::Res {
            account_id: o.get_field("accountId", true),
            title: o.get_field("title", true),
        }
    }
}
impl zone::Guest for Component {
    fn invoke(name: String, args: zone::Args) -> zone::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "jumpStart".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "meta".into(),
                    schema: vec![
                        129, 176, 83, 105, 110, 103, 108, 101, 84, 121, 112, 101, 79, 98, 106, 101,
                        99, 116, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "nameServers".into(),
                    schema: vec![
                        129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "paused".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "plan".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "status".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "type".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "vanityNameServers".into(),
                    schema: vec![
                        129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "verificationKey".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "zone".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        zone::Res {
            account_id: o.get_field("accountId", true),
            jump_start: o.get_field("jumpStart", false),
            meta: o.get_field("meta", true),
            name_servers: o.get_field("nameServers", true),
            paused: o.get_field("paused", false),
            plan: o.get_field("plan", true),
            status: o.get_field("status", true),
            type_: o.get_field("type", false),
            vanity_name_servers: o.get_field("vanityNameServers", true),
            verification_key: o.get_field("verificationKey", true),
            zone: o.get_field("zone", true),
        }
    }
}
impl zone_cache_reserve::Guest for Component {
    fn invoke(name: String, args: zone_cache_reserve::Args) -> zone_cache_reserve::Res {
        wasm_common::setup_logger();
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
                    schema: vec![164, 66, 111, 111, 108],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        zone_cache_reserve::Res {
            enabled: o.get_field("enabled", true),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl zone_cache_variants::Guest for Component {
    fn invoke(name: String, args: zone_cache_variants::Args) -> zone_cache_variants::Res {
        wasm_common::setup_logger();
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
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "bmps".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "gifs".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "jp2s".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "jpegs".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "jpg2s".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "jpgs".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "pngs".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "tiffs".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "tifs".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "webps".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97,
                        121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        zone_cache_variants::Res {
            avifs: o.get_field("avifs", false),
            bmps: o.get_field("bmps", false),
            gifs: o.get_field("gifs", false),
            jp2s: o.get_field("jp2s", false),
            jpegs: o.get_field("jpegs", false),
            jpg2s: o.get_field("jpg2s", false),
            jpgs: o.get_field("jpgs", false),
            pngs: o.get_field("pngs", false),
            tiffs: o.get_field("tiffs", false),
            tifs: o.get_field("tifs", false),
            webps: o.get_field("webps", false),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl zone_dnssec::Guest for Component {
    fn invoke(name: String, args: zone_dnssec::Args) -> zone_dnssec::Res {
        wasm_common::setup_logger();
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
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "digest".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "digestAlgorithm".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "digestType".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "ds".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "flags".into(),
                    schema: vec![163, 73, 110, 116],
                },
                ResultField {
                    name: "keyTag".into(),
                    schema: vec![163, 73, 110, 116],
                },
                ResultField {
                    name: "keyType".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "modifiedOn".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "publicKey".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "status".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        zone_dnssec::Res {
            algorithm: o.get_field("algorithm", true),
            digest: o.get_field("digest", true),
            digest_algorithm: o.get_field("digestAlgorithm", true),
            digest_type: o.get_field("digestType", true),
            ds: o.get_field("ds", true),
            flags: o.get_field("flags", true),
            key_tag: o.get_field("keyTag", true),
            key_type: o.get_field("keyType", true),
            modified_on: o.get_field("modifiedOn", true),
            public_key: o.get_field("publicKey", true),
            status: o.get_field("status", true),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl zone_hold::Guest for Component {
    fn invoke(name: String, args: zone_hold::Args) -> zone_hold::Res {
        wasm_common::setup_logger();
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
                    schema: vec![164, 66, 111, 111, 108],
                },
                ResultField {
                    name: "holdAfter".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "includeSubdomains".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        zone_hold::Res {
            hold: o.get_field("hold", true),
            hold_after: o.get_field("holdAfter", true),
            include_subdomains: o.get_field("includeSubdomains", false),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl zone_lockdown::Guest for Component {
    fn invoke(name: String, args: zone_lockdown::Args) -> zone_lockdown::Res {
        wasm_common::setup_logger();
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
                    schema: vec![
                        129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 130,
                        166, 116, 97, 114, 103, 101, 116, 166, 83, 116, 114, 105, 110, 103, 165,
                        118, 97, 108, 117, 101, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "description".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103,
                    ],
                },
                ResultField {
                    name: "paused".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108,
                    ],
                },
                ResultField {
                    name: "priority".into(),
                    schema: vec![
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116,
                    ],
                },
                ResultField {
                    name: "urls".into(),
                    schema: vec![
                        129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        zone_lockdown::Res {
            configurations: o.get_field("configurations", true),
            description: o.get_field("description", false),
            paused: o.get_field("paused", false),
            priority: o.get_field("priority", false),
            urls: o.get_field("urls", true),
            zone_id: o.get_field("zoneId", true),
        }
    }
}
impl zone_settings_override::Guest for Component {
    fn invoke(name: String, args: zone_settings_override::Args) -> zone_settings_override::Res {
        wasm_common::setup_logger();
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
                    schema: vec![
                        129, 165, 65, 114, 114, 97, 121, 129, 166, 79, 98, 106, 101, 99, 116, 222,
                        0, 56, 172, 97, 108, 119, 97, 121, 115, 79, 110, 108, 105, 110, 101, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                        174, 97, 108, 119, 97, 121, 115, 85, 115, 101, 72, 116, 116, 112, 115, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                        182, 97, 117, 116, 111, 109, 97, 116, 105, 99, 72, 116, 116, 112, 115, 82,
                        101, 119, 114, 105, 116, 101, 115, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 166, 83, 116, 114, 105, 110, 103, 169, 98, 105, 110, 97, 114,
                        121, 65, 115, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83,
                        116, 114, 105, 110, 103, 166, 98, 114, 111, 116, 108, 105, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 175, 98,
                        114, 111, 119, 115, 101, 114, 67, 97, 99, 104, 101, 84, 116, 108, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 172, 98, 114, 111,
                        119, 115, 101, 114, 67, 104, 101, 99, 107, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 170, 99, 97, 99, 104, 101,
                        76, 101, 118, 101, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166,
                        83, 116, 114, 105, 110, 103, 172, 99, 104, 97, 108, 108, 101, 110, 103,
                        101, 84, 116, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73,
                        110, 116, 167, 99, 105, 112, 104, 101, 114, 115, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114,
                        105, 110, 103, 175, 99, 110, 97, 109, 101, 70, 108, 97, 116, 116, 101, 110,
                        105, 110, 103, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116,
                        114, 105, 110, 103, 175, 100, 101, 118, 101, 108, 111, 112, 109, 101, 110,
                        116, 77, 111, 100, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166,
                        83, 116, 114, 105, 110, 103, 170, 101, 97, 114, 108, 121, 72, 105, 110,
                        116, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114,
                        105, 110, 103, 176, 101, 109, 97, 105, 108, 79, 98, 102, 117, 115, 99, 97,
                        116, 105, 111, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83,
                        116, 114, 105, 110, 103, 182, 102, 105, 108, 116, 101, 114, 76, 111, 103,
                        115, 84, 111, 67, 108, 111, 117, 100, 102, 108, 97, 114, 101, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 165,
                        102, 111, 110, 116, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        166, 83, 116, 114, 105, 110, 103, 176, 104, 50, 80, 114, 105, 111, 114,
                        105, 116, 105, 122, 97, 116, 105, 111, 110, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 177, 104, 111, 116,
                        108, 105, 110, 107, 80, 114, 111, 116, 101, 99, 116, 105, 111, 110, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                        165, 104, 116, 116, 112, 50, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        166, 83, 116, 114, 105, 110, 103, 165, 104, 116, 116, 112, 51, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 173,
                        105, 109, 97, 103, 101, 82, 101, 115, 105, 122, 105, 110, 103, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 173,
                        105, 112, 71, 101, 111, 108, 111, 99, 97, 116, 105, 111, 110, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 164,
                        105, 112, 118, 54, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83,
                        116, 114, 105, 110, 103, 175, 108, 111, 103, 84, 111, 67, 108, 111, 117,
                        100, 102, 108, 97, 114, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        166, 83, 116, 114, 105, 110, 103, 169, 109, 97, 120, 85, 112, 108, 111, 97,
                        100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 173,
                        109, 105, 110, 84, 108, 115, 86, 101, 114, 115, 105, 111, 110, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 166,
                        109, 105, 110, 105, 102, 121, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 129, 166, 79, 98, 106, 101, 99, 116, 131, 163, 99, 115, 115, 166, 83,
                        116, 114, 105, 110, 103, 164, 104, 116, 109, 108, 166, 83, 116, 114, 105,
                        110, 103, 162, 106, 115, 166, 83, 116, 114, 105, 110, 103, 166, 109, 105,
                        114, 97, 103, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83,
                        116, 114, 105, 110, 103, 174, 109, 111, 98, 105, 108, 101, 82, 101, 100,
                        105, 114, 101, 99, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129,
                        166, 79, 98, 106, 101, 99, 116, 131, 175, 109, 111, 98, 105, 108, 101, 83,
                        117, 98, 100, 111, 109, 97, 105, 110, 166, 83, 116, 114, 105, 110, 103,
                        166, 115, 116, 97, 116, 117, 115, 166, 83, 116, 114, 105, 110, 103, 168,
                        115, 116, 114, 105, 112, 85, 114, 105, 164, 66, 111, 111, 108, 183, 111,
                        112, 112, 111, 114, 116, 117, 110, 105, 115, 116, 105, 99, 69, 110, 99,
                        114, 121, 112, 116, 105, 111, 110, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 166, 83, 116, 114, 105, 110, 103, 178, 111, 112, 112, 111, 114,
                        116, 117, 110, 105, 115, 116, 105, 99, 79, 110, 105, 111, 110, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 174,
                        111, 114, 97, 110, 103, 101, 84, 111, 79, 114, 97, 110, 103, 101, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 183,
                        111, 114, 105, 103, 105, 110, 69, 114, 114, 111, 114, 80, 97, 103, 101, 80,
                        97, 115, 115, 84, 104, 114, 117, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 166, 83, 116, 114, 105, 110, 103, 180, 111, 114, 105, 103, 105, 110,
                        77, 97, 120, 72, 116, 116, 112, 86, 101, 114, 115, 105, 111, 110, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 166,
                        112, 111, 108, 105, 115, 104, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 166, 83, 116, 114, 105, 110, 103, 175, 112, 114, 101, 102, 101, 116,
                        99, 104, 80, 114, 101, 108, 111, 97, 100, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 171, 112, 114, 105, 118,
                        97, 99, 121, 80, 97, 115, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 166, 83, 116, 114, 105, 110, 103, 176, 112, 114, 111, 120, 121, 82,
                        101, 97, 100, 84, 105, 109, 101, 111, 117, 116, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 170, 112, 115,
                        101, 117, 100, 111, 73, 112, 118, 52, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 166, 83, 116, 114, 105, 110, 103, 177, 114, 101, 115, 112, 111,
                        110, 115, 101, 66, 117, 102, 102, 101, 114, 105, 110, 103, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 172,
                        114, 111, 99, 107, 101, 116, 76, 111, 97, 100, 101, 114, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 174, 115,
                        101, 99, 117, 114, 105, 116, 121, 72, 101, 97, 100, 101, 114, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 133,
                        167, 101, 110, 97, 98, 108, 101, 100, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 164, 66, 111, 111, 108, 177, 105, 110, 99, 108, 117, 100, 101,
                        83, 117, 98, 100, 111, 109, 97, 105, 110, 115, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 164, 66, 111, 111, 108, 166, 109, 97, 120, 65, 103, 101,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 167, 110,
                        111, 115, 110, 105, 102, 102, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 164, 66, 111, 111, 108, 167, 112, 114, 101, 108, 111, 97, 100, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 173, 115,
                        101, 99, 117, 114, 105, 116, 121, 76, 101, 118, 101, 108, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 177,
                        115, 101, 114, 118, 101, 114, 83, 105, 100, 101, 69, 120, 99, 108, 117,
                        100, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114,
                        105, 110, 103, 183, 115, 111, 114, 116, 81, 117, 101, 114, 121, 83, 116,
                        114, 105, 110, 103, 70, 111, 114, 67, 97, 99, 104, 101, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 163, 115,
                        115, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114,
                        105, 110, 103, 169, 116, 108, 115, 49, 50, 79, 110, 108, 121, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 165,
                        116, 108, 115, 49, 51, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166,
                        83, 116, 114, 105, 110, 103, 173, 116, 108, 115, 67, 108, 105, 101, 110,
                        116, 65, 117, 116, 104, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166,
                        83, 116, 114, 105, 110, 103, 178, 116, 114, 117, 101, 67, 108, 105, 101,
                        110, 116, 73, 112, 72, 101, 97, 100, 101, 114, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 172, 117, 110, 105,
                        118, 101, 114, 115, 97, 108, 83, 115, 108, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 169, 118, 105, 115, 105,
                        116, 111, 114, 73, 112, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166,
                        83, 116, 114, 105, 110, 103, 163, 119, 97, 102, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 164, 119, 101, 98,
                        112, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105,
                        110, 103, 170, 119, 101, 98, 115, 111, 99, 107, 101, 116, 115, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 167,
                        122, 101, 114, 111, 82, 116, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "initialSettingsReadAt".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "readonlySettings".into(),
                    schema: vec![
                        129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "settings".into(),
                    schema: vec![
                        129, 166, 79, 98, 106, 101, 99, 116, 222, 0, 56, 172, 97, 108, 119, 97,
                        121, 115, 79, 110, 108, 105, 110, 101, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 166, 83, 116, 114, 105, 110, 103, 174, 97, 108, 119, 97, 121,
                        115, 85, 115, 101, 72, 116, 116, 112, 115, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 182, 97, 117, 116, 111,
                        109, 97, 116, 105, 99, 72, 116, 116, 112, 115, 82, 101, 119, 114, 105, 116,
                        101, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114,
                        105, 110, 103, 169, 98, 105, 110, 97, 114, 121, 65, 115, 116, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 166, 98,
                        114, 111, 116, 108, 105, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        166, 83, 116, 114, 105, 110, 103, 175, 98, 114, 111, 119, 115, 101, 114,
                        67, 97, 99, 104, 101, 84, 116, 108, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 163, 73, 110, 116, 172, 98, 114, 111, 119, 115, 101, 114, 67,
                        104, 101, 99, 107, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83,
                        116, 114, 105, 110, 103, 170, 99, 97, 99, 104, 101, 76, 101, 118, 101, 108,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103, 172, 99, 104, 97, 108, 108, 101, 110, 103, 101, 84, 116, 108, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 167, 99, 105,
                        112, 104, 101, 114, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        129, 165, 65, 114, 114, 97, 121, 166, 83, 116, 114, 105, 110, 103, 175, 99,
                        110, 97, 109, 101, 70, 108, 97, 116, 116, 101, 110, 105, 110, 103, 129,
                        168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103,
                        175, 100, 101, 118, 101, 108, 111, 112, 109, 101, 110, 116, 77, 111, 100,
                        101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105,
                        110, 103, 170, 101, 97, 114, 108, 121, 72, 105, 110, 116, 115, 129, 168,
                        78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 176,
                        101, 109, 97, 105, 108, 79, 98, 102, 117, 115, 99, 97, 116, 105, 111, 110,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103, 182, 102, 105, 108, 116, 101, 114, 76, 111, 103, 115, 84, 111, 67,
                        108, 111, 117, 100, 102, 108, 97, 114, 101, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 165, 102, 111, 110,
                        116, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114,
                        105, 110, 103, 176, 104, 50, 80, 114, 105, 111, 114, 105, 116, 105, 122,
                        97, 116, 105, 111, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166,
                        83, 116, 114, 105, 110, 103, 177, 104, 111, 116, 108, 105, 110, 107, 80,
                        114, 111, 116, 101, 99, 116, 105, 111, 110, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 165, 104, 116, 116,
                        112, 50, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114,
                        105, 110, 103, 165, 104, 116, 116, 112, 51, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 173, 105, 109, 97, 103,
                        101, 82, 101, 115, 105, 122, 105, 110, 103, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 173, 105, 112, 71, 101,
                        111, 108, 111, 99, 97, 116, 105, 111, 110, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 164, 105, 112, 118, 54,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103, 175, 108, 111, 103, 84, 111, 67, 108, 111, 117, 100, 102, 108, 97,
                        114, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114,
                        105, 110, 103, 169, 109, 97, 120, 85, 112, 108, 111, 97, 100, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 173, 109, 105, 110, 84,
                        108, 115, 86, 101, 114, 115, 105, 111, 110, 129, 168, 78, 117, 108, 108,
                        97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 166, 109, 105, 110,
                        105, 102, 121, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79,
                        98, 106, 101, 99, 116, 131, 163, 99, 115, 115, 166, 83, 116, 114, 105, 110,
                        103, 164, 104, 116, 109, 108, 166, 83, 116, 114, 105, 110, 103, 162, 106,
                        115, 166, 83, 116, 114, 105, 110, 103, 166, 109, 105, 114, 97, 103, 101,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103, 174, 109, 111, 98, 105, 108, 101, 82, 101, 100, 105, 114, 101, 99,
                        116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 129, 166, 79, 98, 106,
                        101, 99, 116, 131, 175, 109, 111, 98, 105, 108, 101, 83, 117, 98, 100, 111,
                        109, 97, 105, 110, 166, 83, 116, 114, 105, 110, 103, 166, 115, 116, 97,
                        116, 117, 115, 166, 83, 116, 114, 105, 110, 103, 168, 115, 116, 114, 105,
                        112, 85, 114, 105, 164, 66, 111, 111, 108, 183, 111, 112, 112, 111, 114,
                        116, 117, 110, 105, 115, 116, 105, 99, 69, 110, 99, 114, 121, 112, 116,
                        105, 111, 110, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116,
                        114, 105, 110, 103, 178, 111, 112, 112, 111, 114, 116, 117, 110, 105, 115,
                        116, 105, 99, 79, 110, 105, 111, 110, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 166, 83, 116, 114, 105, 110, 103, 174, 111, 114, 97, 110, 103,
                        101, 84, 111, 79, 114, 97, 110, 103, 101, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 183, 111, 114, 105, 103,
                        105, 110, 69, 114, 114, 111, 114, 80, 97, 103, 101, 80, 97, 115, 115, 84,
                        104, 114, 117, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116,
                        114, 105, 110, 103, 180, 111, 114, 105, 103, 105, 110, 77, 97, 120, 72,
                        116, 116, 112, 86, 101, 114, 115, 105, 111, 110, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 166, 112, 111,
                        108, 105, 115, 104, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83,
                        116, 114, 105, 110, 103, 175, 112, 114, 101, 102, 101, 116, 99, 104, 80,
                        114, 101, 108, 111, 97, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        166, 83, 116, 114, 105, 110, 103, 171, 112, 114, 105, 118, 97, 99, 121, 80,
                        97, 115, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116,
                        114, 105, 110, 103, 176, 112, 114, 111, 120, 121, 82, 101, 97, 100, 84,
                        105, 109, 101, 111, 117, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 166, 83, 116, 114, 105, 110, 103, 170, 112, 115, 101, 117, 100, 111,
                        73, 112, 118, 52, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83,
                        116, 114, 105, 110, 103, 177, 114, 101, 115, 112, 111, 110, 115, 101, 66,
                        117, 102, 102, 101, 114, 105, 110, 103, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 172, 114, 111, 99, 107,
                        101, 116, 76, 111, 97, 100, 101, 114, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 166, 83, 116, 114, 105, 110, 103, 174, 115, 101, 99, 117, 114,
                        105, 116, 121, 72, 101, 97, 100, 101, 114, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 129, 166, 79, 98, 106, 101, 99, 116, 133, 167, 101, 110, 97,
                        98, 108, 101, 100, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66,
                        111, 111, 108, 177, 105, 110, 99, 108, 117, 100, 101, 83, 117, 98, 100,
                        111, 109, 97, 105, 110, 115, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101,
                        164, 66, 111, 111, 108, 166, 109, 97, 120, 65, 103, 101, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 163, 73, 110, 116, 167, 110, 111, 115, 110,
                        105, 102, 102, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 164, 66, 111,
                        111, 108, 167, 112, 114, 101, 108, 111, 97, 100, 129, 168, 78, 117, 108,
                        108, 97, 98, 108, 101, 164, 66, 111, 111, 108, 173, 115, 101, 99, 117, 114,
                        105, 116, 121, 76, 101, 118, 101, 108, 129, 168, 78, 117, 108, 108, 97, 98,
                        108, 101, 166, 83, 116, 114, 105, 110, 103, 177, 115, 101, 114, 118, 101,
                        114, 83, 105, 100, 101, 69, 120, 99, 108, 117, 100, 101, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 183, 115,
                        111, 114, 116, 81, 117, 101, 114, 121, 83, 116, 114, 105, 110, 103, 70,
                        111, 114, 67, 97, 99, 104, 101, 129, 168, 78, 117, 108, 108, 97, 98, 108,
                        101, 166, 83, 116, 114, 105, 110, 103, 163, 115, 115, 108, 129, 168, 78,
                        117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 169,
                        116, 108, 115, 49, 50, 79, 110, 108, 121, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 165, 116, 108, 115, 49, 51,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103, 173, 116, 108, 115, 67, 108, 105, 101, 110, 116, 65, 117, 116, 104,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103, 178, 116, 114, 117, 101, 67, 108, 105, 101, 110, 116, 73, 112, 72,
                        101, 97, 100, 101, 114, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166,
                        83, 116, 114, 105, 110, 103, 172, 117, 110, 105, 118, 101, 114, 115, 97,
                        108, 83, 115, 108, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83,
                        116, 114, 105, 110, 103, 169, 118, 105, 115, 105, 116, 111, 114, 73, 112,
                        129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110,
                        103, 163, 119, 97, 102, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166,
                        83, 116, 114, 105, 110, 103, 164, 119, 101, 98, 112, 129, 168, 78, 117,
                        108, 108, 97, 98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 170, 119,
                        101, 98, 115, 111, 99, 107, 101, 116, 115, 129, 168, 78, 117, 108, 108, 97,
                        98, 108, 101, 166, 83, 116, 114, 105, 110, 103, 167, 122, 101, 114, 111,
                        82, 116, 116, 129, 168, 78, 117, 108, 108, 97, 98, 108, 101, 166, 83, 116,
                        114, 105, 110, 103,
                    ],
                },
                ResultField {
                    name: "zoneId".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "zoneStatus".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
                ResultField {
                    name: "zoneType".into(),
                    schema: vec![166, 83, 116, 114, 105, 110, 103],
                },
            ],
        };

        let o = register(&request);

        zone_settings_override::Res {
            initial_settings: o.get_field("initialSettings", true),
            initial_settings_read_at: o.get_field("initialSettingsReadAt", true),
            readonly_settings: o.get_field("readonlySettings", true),
            settings: o.get_field("settings", true),
            zone_id: o.get_field("zoneId", true),
            zone_status: o.get_field("zoneStatus", true),
            zone_type: o.get_field("zoneType", true),
        }
    }
}
