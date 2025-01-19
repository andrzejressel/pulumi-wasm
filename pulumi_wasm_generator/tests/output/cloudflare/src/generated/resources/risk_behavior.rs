/// The [Risk Behavior](https://developers.cloudflare.com/cloudflare-one/insights/risk-score/) resource allows you to configure Cloudflare Risk Behaviors for an account.
pub mod risk_behavior {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RiskBehaviorArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// Zero Trust risk behaviors configured on this account
        #[builder(into, default)]
        pub behaviors: pulumi_wasm_rust::Output<
            Option<Vec<super::types::RiskBehaviorBehavior>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RiskBehaviorResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// Zero Trust risk behaviors configured on this account
        pub behaviors: pulumi_wasm_rust::Output<
            Option<Vec<super::types::RiskBehaviorBehavior>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RiskBehaviorArgs) -> RiskBehaviorResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let behaviors_binding = args.behaviors.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/riskBehavior:RiskBehavior".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "behaviors".into(),
                    value: &behaviors_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "behaviors".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RiskBehaviorResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            behaviors: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("behaviors").unwrap(),
            ),
        }
    }
}
