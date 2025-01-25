pub mod get_network_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetworkPolicyArgs {
        /// Location of the resource.
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// Name of the resource.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetNetworkPolicyResult {
        pub create_time: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<String>,
        pub edge_services_cidr: pulumi_wasm_rust::Output<String>,
        pub external_ips: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::vmwareengine::GetNetworkPolicyExternalIp>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub internet_accesses: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::vmwareengine::GetNetworkPolicyInternetAccess>,
        >,
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub uid: pulumi_wasm_rust::Output<String>,
        pub update_time: pulumi_wasm_rust::Output<String>,
        pub vmware_engine_network: pulumi_wasm_rust::Output<String>,
        pub vmware_engine_network_canonical: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetNetworkPolicyArgs,
    ) -> GetNetworkPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:vmwareengine/getNetworkPolicy:getNetworkPolicy".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "edgeServicesCidr".into(),
                },
                register_interface::ResultField {
                    name: "externalIps".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "internetAccesses".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "uid".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
                register_interface::ResultField {
                    name: "vmwareEngineNetwork".into(),
                },
                register_interface::ResultField {
                    name: "vmwareEngineNetworkCanonical".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetNetworkPolicyResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            edge_services_cidr: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("edgeServicesCidr").unwrap(),
            ),
            external_ips: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("externalIps").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            internet_accesses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("internetAccesses").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uid").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
            vmware_engine_network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vmwareEngineNetwork").unwrap(),
            ),
            vmware_engine_network_canonical: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vmwareEngineNetworkCanonical").unwrap(),
            ),
        }
    }
}
