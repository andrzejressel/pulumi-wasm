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

pub fn create(name: &str, args: AccessOrganizationArgs) -> AccessOrganizationResult {
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
            user_seat_expiration_inactive_time: args.user_seat_expiration_inactive_time.get_inner(),
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
