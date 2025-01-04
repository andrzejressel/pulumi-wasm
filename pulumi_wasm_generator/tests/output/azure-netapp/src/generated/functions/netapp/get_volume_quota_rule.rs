pub mod get_volume_quota_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVolumeQuotaRuleArgs {
        /// The name of this Volume Quota Rule.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The NetApp volume ID where the Volume Quota Rule is assigned to.
        #[builder(into)]
        pub volume_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetVolumeQuotaRuleResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the Volume Quota Rule exists.
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The quota size in kibibytes.
        pub quota_size_in_kib: pulumi_wasm_rust::Output<i32>,
        /// The quota Target.
        pub quota_target: pulumi_wasm_rust::Output<String>,
        /// The quota type.
        pub quota_type: pulumi_wasm_rust::Output<String>,
        pub volume_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetVolumeQuotaRuleArgs) -> GetVolumeQuotaRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let volume_id_binding = args.volume_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:netapp/getVolumeQuotaRule:getVolumeQuotaRule".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "volumeId".into(),
                    value: &volume_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "quotaSizeInKib".into(),
                },
                register_interface::ResultField {
                    name: "quotaTarget".into(),
                },
                register_interface::ResultField {
                    name: "quotaType".into(),
                },
                register_interface::ResultField {
                    name: "volumeId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetVolumeQuotaRuleResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            quota_size_in_kib: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("quotaSizeInKib").unwrap(),
            ),
            quota_target: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("quotaTarget").unwrap(),
            ),
            quota_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("quotaType").unwrap(),
            ),
            volume_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("volumeId").unwrap(),
            ),
        }
    }
}
