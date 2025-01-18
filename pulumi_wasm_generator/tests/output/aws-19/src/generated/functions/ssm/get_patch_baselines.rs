pub mod get_patch_baselines {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPatchBaselinesArgs {
        /// Only return baseline identities where `default_baseline` is `true`.
        #[builder(into, default)]
        pub default_baselines: pulumi_wasm_rust::Output<Option<bool>>,
        /// Key-value pairs used to filter the results. See `filter` below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ssm::GetPatchBaselinesFilter>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetPatchBaselinesResult {
        /// List of baseline identities. See `baseline_identities` below.
        pub baseline_identities: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ssm::GetPatchBaselinesBaselineIdentity>,
        >,
        pub default_baselines: pulumi_wasm_rust::Output<Option<bool>>,
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ssm::GetPatchBaselinesFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetPatchBaselinesArgs) -> GetPatchBaselinesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let default_baselines_binding = args.default_baselines.get_inner();
        let filters_binding = args.filters.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ssm/getPatchBaselines:getPatchBaselines".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "defaultBaselines".into(),
                    value: &default_baselines_binding,
                },
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "baselineIdentities".into(),
                },
                register_interface::ResultField {
                    name: "defaultBaselines".into(),
                },
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetPatchBaselinesResult {
            baseline_identities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("baselineIdentities").unwrap(),
            ),
            default_baselines: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultBaselines").unwrap(),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
        }
    }
}
