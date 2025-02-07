/// Access Service Tokens are used for service-to-service communication
/// when an application is behind Cloudflare Access.
///
/// ## Import
///
/// If you are importing an Access Service Token you will not have the
///
/// client_secret available in the state for use. The client_secret is only
///
/// available once, at creation. In most cases, it is better to just create a new
///
/// resource should you need to reference it in other resources.
///
/// ```sh
/// $ pulumi import cloudflare:index/zeroTrustAccessServiceToken:ZeroTrustAccessServiceToken example <account_id>/<service_token_id>
/// ```
///
pub mod zero_trust_access_service_token {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZeroTrustAccessServiceTokenArgs {
        /// The account identifier to target for the resource. Conflicts with `zone_id`.
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Length of time the service token is valid for. Available values: `8760h`, `17520h`, `43800h`, `87600h`, `forever`.
        #[builder(into, default)]
        pub duration: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub min_days_for_renewal: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Friendly name of the token's intent.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The zone identifier to target for the resource. Conflicts with `account_id`.
        #[builder(into, default)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ZeroTrustAccessServiceTokenResult {
        /// The account identifier to target for the resource. Conflicts with `zone_id`.
        pub account_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Client ID associated with the Service Token. **Modifying this attribute will force creation of a new resource.**
        pub client_id: pulumi_gestalt_rust::Output<String>,
        /// A secret for interacting with Access protocols. **Modifying this attribute will force creation of a new resource.**
        pub client_secret: pulumi_gestalt_rust::Output<String>,
        /// Length of time the service token is valid for. Available values: `8760h`, `17520h`, `43800h`, `87600h`, `forever`.
        pub duration: pulumi_gestalt_rust::Output<String>,
        /// Date when the token expires.
        pub expires_at: pulumi_gestalt_rust::Output<String>,
        pub min_days_for_renewal: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Friendly name of the token's intent.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The zone identifier to target for the resource. Conflicts with `account_id`.
        pub zone_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ZeroTrustAccessServiceTokenArgs,
    ) -> ZeroTrustAccessServiceTokenResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let duration_binding = args.duration.get_output(context).get_inner();
        let min_days_for_renewal_binding = args
            .min_days_for_renewal
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let zone_id_binding = args.zone_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustAccessServiceToken:ZeroTrustAccessServiceToken"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "duration".into(),
                    value: &duration_binding,
                },
                register_interface::ObjectField {
                    name: "minDaysForRenewal".into(),
                    value: &min_days_for_renewal_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ZeroTrustAccessServiceTokenResult {
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            client_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clientId"),
            ),
            client_secret: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clientSecret"),
            ),
            duration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("duration"),
            ),
            expires_at: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("expiresAt"),
            ),
            min_days_for_renewal: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("minDaysForRenewal"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            zone_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zoneId"),
            ),
        }
    }
}
