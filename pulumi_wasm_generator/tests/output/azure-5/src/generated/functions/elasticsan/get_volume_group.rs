pub mod get_volume_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVolumeGroupArgs {
        /// The Elastic SAN ID within which the Elastic SAN Volume Group exists.
        #[builder(into)]
        pub elastic_san_id: pulumi_wasm_rust::Output<String>,
        /// The name of the Elastic SAN Volume Group.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetVolumeGroupResult {
        pub elastic_san_id: pulumi_wasm_rust::Output<String>,
        /// The type of the key used to encrypt the data of the disk.
        pub encryption_type: pulumi_wasm_rust::Output<String>,
        /// An `encryption` block as defined below.
        pub encryptions: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::elasticsan::GetVolumeGroupEncryption>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identities: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::elasticsan::GetVolumeGroupIdentity>,
        >,
        pub name: pulumi_wasm_rust::Output<String>,
        /// One or more `network_rule` blocks as defined below.
        pub network_rules: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::elasticsan::GetVolumeGroupNetworkRule>,
        >,
        /// The type of the storage target.
        pub protocol_type: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetVolumeGroupArgs) -> GetVolumeGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let elastic_san_id_binding = args.elastic_san_id.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:elasticsan/getVolumeGroup:getVolumeGroup".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "elasticSanId".into(),
                    value: &elastic_san_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "elasticSanId".into(),
                },
                register_interface::ResultField {
                    name: "encryptionType".into(),
                },
                register_interface::ResultField {
                    name: "encryptions".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "identities".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkRules".into(),
                },
                register_interface::ResultField {
                    name: "protocolType".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetVolumeGroupResult {
            elastic_san_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("elasticSanId").unwrap(),
            ),
            encryption_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionType").unwrap(),
            ),
            encryptions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptions").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            identities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identities").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkRules").unwrap(),
            ),
            protocol_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protocolType").unwrap(),
            ),
        }
    }
}
