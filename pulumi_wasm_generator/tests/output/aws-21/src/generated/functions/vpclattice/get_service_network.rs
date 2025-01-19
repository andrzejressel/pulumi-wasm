pub mod get_service_network {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServiceNetworkArgs {
        /// Identifier of the service network.
        #[builder(into)]
        pub service_network_identifier: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetServiceNetworkResult {
        /// ARN of the Service Network.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Authentication type for the service network. Either `NONE` or `AWS_IAM`.
        pub auth_type: pulumi_wasm_rust::Output<String>,
        /// Date and time the service network was created.
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Date and time the service network was last updated.
        pub last_updated_at: pulumi_wasm_rust::Output<String>,
        /// Name of the service network.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Number of services associated with this service network.
        pub number_of_associated_services: pulumi_wasm_rust::Output<i32>,
        /// Number of VPCs associated with this service network.
        pub number_of_associated_vpcs: pulumi_wasm_rust::Output<i32>,
        pub service_network_identifier: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetServiceNetworkArgs) -> GetServiceNetworkResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let service_network_identifier_binding = args
            .service_network_identifier
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:vpclattice/getServiceNetwork:getServiceNetwork".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "serviceNetworkIdentifier".into(),
                    value: &service_network_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "authType".into(),
                },
                register_interface::ResultField {
                    name: "createdAt".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "lastUpdatedAt".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "numberOfAssociatedServices".into(),
                },
                register_interface::ResultField {
                    name: "numberOfAssociatedVpcs".into(),
                },
                register_interface::ResultField {
                    name: "serviceNetworkIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetServiceNetworkResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            auth_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authType").unwrap(),
            ),
            created_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdAt").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            last_updated_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastUpdatedAt").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            number_of_associated_services: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("numberOfAssociatedServices").unwrap(),
            ),
            number_of_associated_vpcs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("numberOfAssociatedVpcs").unwrap(),
            ),
            service_network_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceNetworkIdentifier").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
