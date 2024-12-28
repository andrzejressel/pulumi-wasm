/// A Zero Trust organization defines the user login experience.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = access_organization::create(
///         "example",
///         AccessOrganizationArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .auth_domain("example.cloudflareaccess.com")
///             .auto_redirect_to_identity(false)
///             .is_ui_read_only(false)
///             .login_designs(
///                 vec![
///                     AccessOrganizationLoginDesign::builder().backgroundColor("#ffffff")
///                     .footerText("My footer text").headerText("My header text")
///                     .logoPath("https://example.com/logo.png").textColor("#000000")
///                     .build_struct(),
///                 ],
///             )
///             .name("example.cloudflareaccess.com")
///             .user_seat_expiration_inactive_time("720h")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/accessOrganization:AccessOrganization example <account_id>
/// ```
///
pub mod access_organization {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessOrganizationArgs {
        /// The account identifier to target for the resource. Conflicts with `zone_id`.
        #[builder(into, default)]
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// When set to true, users can authenticate via WARP for any application in your organization. Application settings will take precedence over this value.
        #[builder(into, default)]
        pub allow_authenticate_via_warp: pulumi_wasm_rust::Output<Option<bool>>,
        /// The unique subdomain assigned to your Zero Trust organization.
        #[builder(into)]
        pub auth_domain: pulumi_wasm_rust::Output<String>,
        /// When set to true, users skip the identity provider selection step during login.
        #[builder(into, default)]
        pub auto_redirect_to_identity: pulumi_wasm_rust::Output<Option<bool>>,
        /// Custom pages for your Zero Trust organization.
        #[builder(into, default)]
        pub custom_pages: pulumi_wasm_rust::Output<
            Option<Vec<super::types::AccessOrganizationCustomPage>>,
        >,
        /// When set to true, this will disable all editing of Access resources via the Zero Trust Dashboard.
        #[builder(into, default)]
        pub is_ui_read_only: pulumi_wasm_rust::Output<Option<bool>>,
        #[builder(into, default)]
        pub login_designs: pulumi_wasm_rust::Output<
            Option<Vec<super::types::AccessOrganizationLoginDesign>>,
        >,
        /// The name of your Zero Trust organization.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// How often a user will be forced to re-authorise. Must be in the format `48h` or `2h45m`.
        #[builder(into, default)]
        pub session_duration: pulumi_wasm_rust::Output<Option<String>>,
        /// A description of the reason why the UI read only field is being toggled.
        #[builder(into, default)]
        pub ui_read_only_toggle_reason: pulumi_wasm_rust::Output<Option<String>>,
        /// The amount of time a user seat is inactive before it expires. When the user seat exceeds the set time of inactivity, the user is removed as an active seat and no longer counts against your Teams seat count. Must be in the format `300ms` or `2h45m`.
        #[builder(into, default)]
        pub user_seat_expiration_inactive_time: pulumi_wasm_rust::Output<Option<String>>,
        /// The amount of time that tokens issued for applications will be valid. Must be in the format 30m or 2h45m. Valid time units are: m, h.
        #[builder(into, default)]
        pub warp_auth_session_duration: pulumi_wasm_rust::Output<Option<String>>,
        /// The zone identifier to target for the resource. Conflicts with `account_id`.
        #[builder(into, default)]
        pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AccessOrganizationResult {
        /// The account identifier to target for the resource. Conflicts with `zone_id`.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// When set to true, users can authenticate via WARP for any application in your organization. Application settings will take precedence over this value.
        pub allow_authenticate_via_warp: pulumi_wasm_rust::Output<Option<bool>>,
        /// The unique subdomain assigned to your Zero Trust organization.
        pub auth_domain: pulumi_wasm_rust::Output<String>,
        /// When set to true, users skip the identity provider selection step during login.
        pub auto_redirect_to_identity: pulumi_wasm_rust::Output<Option<bool>>,
        /// Custom pages for your Zero Trust organization.
        pub custom_pages: pulumi_wasm_rust::Output<
            Option<Vec<super::types::AccessOrganizationCustomPage>>,
        >,
        /// When set to true, this will disable all editing of Access resources via the Zero Trust Dashboard.
        pub is_ui_read_only: pulumi_wasm_rust::Output<Option<bool>>,
        pub login_designs: pulumi_wasm_rust::Output<
            Option<Vec<super::types::AccessOrganizationLoginDesign>>,
        >,
        /// The name of your Zero Trust organization.
        pub name: pulumi_wasm_rust::Output<String>,
        /// How often a user will be forced to re-authorise. Must be in the format `48h` or `2h45m`.
        pub session_duration: pulumi_wasm_rust::Output<Option<String>>,
        /// A description of the reason why the UI read only field is being toggled.
        pub ui_read_only_toggle_reason: pulumi_wasm_rust::Output<Option<String>>,
        /// The amount of time a user seat is inactive before it expires. When the user seat exceeds the set time of inactivity, the user is removed as an active seat and no longer counts against your Teams seat count. Must be in the format `300ms` or `2h45m`.
        pub user_seat_expiration_inactive_time: pulumi_wasm_rust::Output<Option<String>>,
        /// The amount of time that tokens issued for applications will be valid. Must be in the format 30m or 2h45m. Valid time units are: m, h.
        pub warp_auth_session_duration: pulumi_wasm_rust::Output<Option<String>>,
        /// The zone identifier to target for the resource. Conflicts with `account_id`.
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AccessOrganizationArgs) -> AccessOrganizationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let allow_authenticate_via_warp_binding = args
            .allow_authenticate_via_warp
            .get_inner();
        let auth_domain_binding = args.auth_domain.get_inner();
        let auto_redirect_to_identity_binding = args
            .auto_redirect_to_identity
            .get_inner();
        let custom_pages_binding = args.custom_pages.get_inner();
        let is_ui_read_only_binding = args.is_ui_read_only.get_inner();
        let login_designs_binding = args.login_designs.get_inner();
        let name_binding = args.name.get_inner();
        let session_duration_binding = args.session_duration.get_inner();
        let ui_read_only_toggle_reason_binding = args
            .ui_read_only_toggle_reason
            .get_inner();
        let user_seat_expiration_inactive_time_binding = args
            .user_seat_expiration_inactive_time
            .get_inner();
        let warp_auth_session_duration_binding = args
            .warp_auth_session_duration
            .get_inner();
        let zone_id_binding = args.zone_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/accessOrganization:AccessOrganization".into(),
            name: name.to_string(),
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
                    name: "authDomain".into(),
                    value: &auth_domain_binding,
                },
                register_interface::ObjectField {
                    name: "autoRedirectToIdentity".into(),
                    value: &auto_redirect_to_identity_binding,
                },
                register_interface::ObjectField {
                    name: "customPages".into(),
                    value: &custom_pages_binding,
                },
                register_interface::ObjectField {
                    name: "isUiReadOnly".into(),
                    value: &is_ui_read_only_binding,
                },
                register_interface::ObjectField {
                    name: "loginDesigns".into(),
                    value: &login_designs_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "sessionDuration".into(),
                    value: &session_duration_binding,
                },
                register_interface::ObjectField {
                    name: "uiReadOnlyToggleReason".into(),
                    value: &ui_read_only_toggle_reason_binding,
                },
                register_interface::ObjectField {
                    name: "userSeatExpirationInactiveTime".into(),
                    value: &user_seat_expiration_inactive_time_binding,
                },
                register_interface::ObjectField {
                    name: "warpAuthSessionDuration".into(),
                    value: &warp_auth_session_duration_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "allowAuthenticateViaWarp".into(),
                },
                register_interface::ResultField {
                    name: "authDomain".into(),
                },
                register_interface::ResultField {
                    name: "autoRedirectToIdentity".into(),
                },
                register_interface::ResultField {
                    name: "customPages".into(),
                },
                register_interface::ResultField {
                    name: "isUiReadOnly".into(),
                },
                register_interface::ResultField {
                    name: "loginDesigns".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "sessionDuration".into(),
                },
                register_interface::ResultField {
                    name: "uiReadOnlyToggleReason".into(),
                },
                register_interface::ResultField {
                    name: "userSeatExpirationInactiveTime".into(),
                },
                register_interface::ResultField {
                    name: "warpAuthSessionDuration".into(),
                },
                register_interface::ResultField {
                    name: "zoneId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AccessOrganizationResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            allow_authenticate_via_warp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowAuthenticateViaWarp").unwrap(),
            ),
            auth_domain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authDomain").unwrap(),
            ),
            auto_redirect_to_identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoRedirectToIdentity").unwrap(),
            ),
            custom_pages: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customPages").unwrap(),
            ),
            is_ui_read_only: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("isUiReadOnly").unwrap(),
            ),
            login_designs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loginDesigns").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            session_duration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sessionDuration").unwrap(),
            ),
            ui_read_only_toggle_reason: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uiReadOnlyToggleReason").unwrap(),
            ),
            user_seat_expiration_inactive_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userSeatExpirationInactiveTime").unwrap(),
            ),
            warp_auth_session_duration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("warpAuthSessionDuration").unwrap(),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneId").unwrap(),
            ),
        }
    }
}
