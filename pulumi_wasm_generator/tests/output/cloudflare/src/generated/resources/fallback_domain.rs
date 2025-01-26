/// Provides a Cloudflare Fallback Domain resource. Fallback domains are
/// used to ignore DNS requests to a given list of domains. These DNS
/// requests will be passed back to other DNS servers configured on
/// existing network interfaces on the device.
///
/// ## Import
///
/// Fallback Domains for default device policies must use "default" as the policy ID.
///
/// ```sh
/// $ pulumi import cloudflare:index/fallbackDomain:FallbackDomain example <account_id>/<policy_id>
/// ```
///
pub mod fallback_domain {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FallbackDomainArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into)]
        pub domains: pulumi_wasm_rust::InputOrOutput<
            Vec<super::types::FallbackDomainDomain>,
        >,
        /// The settings policy for which to configure this fallback domain policy.
        #[builder(into, default)]
        pub policy_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct FallbackDomainResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_wasm_rust::Output<String>,
        pub domains: pulumi_wasm_rust::Output<Vec<super::types::FallbackDomainDomain>>,
        /// The settings policy for which to configure this fallback domain policy.
        pub policy_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: FallbackDomainArgs,
    ) -> FallbackDomainResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let domains_binding = args.domains.get_output(context).get_inner();
        let policy_id_binding = args.policy_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/fallbackDomain:FallbackDomain".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "domains".into(),
                    value: &domains_binding,
                },
                register_interface::ObjectField {
                    name: "policyId".into(),
                    value: &policy_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        FallbackDomainResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            domains: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("domains"),
            ),
            policy_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("policyId"),
            ),
        }
    }
}
