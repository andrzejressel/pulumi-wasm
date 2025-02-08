/// A Zero Trust organization defines the user login experience.
#[allow(clippy::doc_lazy_continuation)]
pub mod zero_trust_access_organization {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZeroTrustAccessOrganizationArgs {
        /// The account identifier to target for the resource. Conflicts with `zone_id`.
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// When set to true, users can authenticate via WARP for any application in your organization. Application settings will take precedence over this value.
        #[builder(into, default)]
        pub allow_authenticate_via_warp: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The unique subdomain assigned to your Zero Trust organization.
        #[builder(into)]
        pub auth_domain: pulumi_gestalt_rust::InputOrOutput<String>,
        /// When set to true, users skip the identity provider selection step during login.
        #[builder(into, default)]
        pub auto_redirect_to_identity: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Custom pages for your Zero Trust organization.
        #[builder(into, default)]
        pub custom_pages: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::ZeroTrustAccessOrganizationCustomPage>>,
        >,
        /// When set to true, this will disable all editing of Access resources via the Zero Trust Dashboard.
        #[builder(into, default)]
        pub is_ui_read_only: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub login_designs: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::ZeroTrustAccessOrganizationLoginDesign>>,
        >,
        /// The name of your Zero Trust organization.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// How often a user will be forced to re-authorise. Must be in the format `48h` or `2h45m`.
        #[builder(into, default)]
        pub session_duration: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A description of the reason why the UI read only field is being toggled.
        #[builder(into, default)]
        pub ui_read_only_toggle_reason: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The amount of time a user seat is inactive before it expires. When the user seat exceeds the set time of inactivity, the user is removed as an active seat and no longer counts against your Teams seat count. Must be in the format `300ms` or `2h45m`.
        #[builder(into, default)]
        pub user_seat_expiration_inactive_time: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The amount of time that tokens issued for applications will be valid. Must be in the format 30m or 2h45m. Valid time units are: m, h.
        #[builder(into, default)]
        pub warp_auth_session_duration: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The zone identifier to target for the resource. Conflicts with `account_id`.
        #[builder(into, default)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ZeroTrustAccessOrganizationResult {
        /// The account identifier to target for the resource. Conflicts with `zone_id`.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// When set to true, users can authenticate via WARP for any application in your organization. Application settings will take precedence over this value.
        pub allow_authenticate_via_warp: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The unique subdomain assigned to your Zero Trust organization.
        pub auth_domain: pulumi_gestalt_rust::Output<String>,
        /// When set to true, users skip the identity provider selection step during login.
        pub auto_redirect_to_identity: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Custom pages for your Zero Trust organization.
        pub custom_pages: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::ZeroTrustAccessOrganizationCustomPage>>,
        >,
        /// When set to true, this will disable all editing of Access resources via the Zero Trust Dashboard.
        pub is_ui_read_only: pulumi_gestalt_rust::Output<Option<bool>>,
        pub login_designs: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::ZeroTrustAccessOrganizationLoginDesign>>,
        >,
        /// The name of your Zero Trust organization.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// How often a user will be forced to re-authorise. Must be in the format `48h` or `2h45m`.
        pub session_duration: pulumi_gestalt_rust::Output<Option<String>>,
        /// A description of the reason why the UI read only field is being toggled.
        pub ui_read_only_toggle_reason: pulumi_gestalt_rust::Output<Option<String>>,
        /// The amount of time a user seat is inactive before it expires. When the user seat exceeds the set time of inactivity, the user is removed as an active seat and no longer counts against your Teams seat count. Must be in the format `300ms` or `2h45m`.
        pub user_seat_expiration_inactive_time: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// The amount of time that tokens issued for applications will be valid. Must be in the format 30m or 2h45m. Valid time units are: m, h.
        pub warp_auth_session_duration: pulumi_gestalt_rust::Output<Option<String>>,
        /// The zone identifier to target for the resource. Conflicts with `account_id`.
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ZeroTrustAccessOrganizationArgs,
    ) -> ZeroTrustAccessOrganizationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let allow_authenticate_via_warp_binding = args
            .allow_authenticate_via_warp
            .get_output(context)
            .get_inner();
        let auth_domain_binding = args.auth_domain.get_output(context).get_inner();
        let auto_redirect_to_identity_binding = args
            .auto_redirect_to_identity
            .get_output(context)
            .get_inner();
        let custom_pages_binding = args.custom_pages.get_output(context).get_inner();
        let is_ui_read_only_binding = args
            .is_ui_read_only
            .get_output(context)
            .get_inner();
        let login_designs_binding = args.login_designs.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let session_duration_binding = args
            .session_duration
            .get_output(context)
            .get_inner();
        let ui_read_only_toggle_reason_binding = args
            .ui_read_only_toggle_reason
            .get_output(context)
            .get_inner();
        let user_seat_expiration_inactive_time_binding = args
            .user_seat_expiration_inactive_time
            .get_output(context)
            .get_inner();
        let warp_auth_session_duration_binding = args
            .warp_auth_session_duration
            .get_output(context)
            .get_inner();
        let zone_id_binding = args.zone_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustAccessOrganization:ZeroTrustAccessOrganization"
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        ZeroTrustAccessOrganizationResult {
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            allow_authenticate_via_warp: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("allowAuthenticateViaWarp"),
            ),
            auth_domain: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("authDomain"),
            ),
            auto_redirect_to_identity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoRedirectToIdentity"),
            ),
            custom_pages: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customPages"),
            ),
            is_ui_read_only: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("isUiReadOnly"),
            ),
            login_designs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("loginDesigns"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            session_duration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sessionDuration"),
            ),
            ui_read_only_toggle_reason: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("uiReadOnlyToggleReason"),
            ),
            user_seat_expiration_inactive_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userSeatExpirationInactiveTime"),
            ),
            warp_auth_session_duration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("warpAuthSessionDuration"),
            ),
            zone_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zoneId"),
            ),
        }
    }
}
