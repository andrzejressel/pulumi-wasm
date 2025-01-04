pub mod get_gateway_route {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetGatewayRouteArgs {
        /// Name of the service mesh in which the virtual gateway exists.
        #[builder(into)]
        pub mesh_name: pulumi_wasm_rust::Output<String>,
        /// AWS account ID of the service mesh's owner.
        #[builder(into, default)]
        pub mesh_owner: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the gateway route.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Map of tags.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the virtual gateway in which the route exists.
        #[builder(into)]
        pub virtual_gateway_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetGatewayRouteResult {
        /// ARN of the gateway route.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Creation date of the gateway route.
        pub created_date: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Last update date of the gateway route.
        pub last_updated_date: pulumi_wasm_rust::Output<String>,
        pub mesh_name: pulumi_wasm_rust::Output<String>,
        pub mesh_owner: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// Resource owner's AWS account ID.
        pub resource_owner: pulumi_wasm_rust::Output<String>,
        /// Gateway route specification. See the `aws.appmesh.GatewayRoute` resource for details.
        pub specs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::appmesh::GetGatewayRouteSpec>,
        >,
        /// Map of tags.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub virtual_gateway_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetGatewayRouteArgs) -> GetGatewayRouteResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let mesh_name_binding = args.mesh_name.get_inner();
        let mesh_owner_binding = args.mesh_owner.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let virtual_gateway_name_binding = args.virtual_gateway_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:appmesh/getGatewayRoute:getGatewayRoute".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "meshName".into(),
                    value: &mesh_name_binding,
                },
                register_interface::ObjectField {
                    name: "meshOwner".into(),
                    value: &mesh_owner_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "virtualGatewayName".into(),
                    value: &virtual_gateway_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "createdDate".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "lastUpdatedDate".into(),
                },
                register_interface::ResultField {
                    name: "meshName".into(),
                },
                register_interface::ResultField {
                    name: "meshOwner".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceOwner".into(),
                },
                register_interface::ResultField {
                    name: "specs".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "virtualGatewayName".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetGatewayRouteResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            created_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdDate").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            last_updated_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastUpdatedDate").unwrap(),
            ),
            mesh_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("meshName").unwrap(),
            ),
            mesh_owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("meshOwner").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceOwner").unwrap(),
            ),
            specs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("specs").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            virtual_gateway_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualGatewayName").unwrap(),
            ),
        }
    }
}
