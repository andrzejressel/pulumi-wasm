use crate::bindings::component::pulumi_wasm::register_interface::{
    register, ObjectField, RegisterResourceRequest, ResultField,
};
use crate::bindings::exports::pulumi::cloudflare::access_organization;
use crate::Component;
use std::collections::HashMap;

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
