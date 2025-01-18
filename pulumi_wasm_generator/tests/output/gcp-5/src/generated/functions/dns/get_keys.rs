pub mod get_keys {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetKeysArgs {
        /// The name or id of the Cloud DNS managed zone.
        #[builder(into)]
        pub managed_zone: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs. If `project` is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetKeysResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// A list of Key-signing key (KSK) records. Structure is documented below. Additionally, the DS record is provided:
        pub key_signing_keys: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::dns::GetKeysKeySigningKey>,
        >,
        pub managed_zone: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        /// A list of Zone-signing key (ZSK) records. Structure is documented below.
        pub zone_signing_keys: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::dns::GetKeysZoneSigningKey>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetKeysArgs) -> GetKeysResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let managed_zone_binding = args.managed_zone.get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:dns/getKeys:getKeys".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "managedZone".into(),
                    value: &managed_zone_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "keySigningKeys".into(),
                },
                register_interface::ResultField {
                    name: "managedZone".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "zoneSigningKeys".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetKeysResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            key_signing_keys: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keySigningKeys").unwrap(),
            ),
            managed_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managedZone").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            zone_signing_keys: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneSigningKeys").unwrap(),
            ),
        }
    }
}
