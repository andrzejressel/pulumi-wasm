pub mod get_connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetConnectionArgs {
        /// Concatenation of the catalog ID and connection name. For example, if your account ID is
        /// `123456789123` and the connection name is `conn` then the ID is `123456789123:conn`.
        #[builder(into)]
        pub id: pulumi_wasm_rust::Output<String>,
        /// Tags assigned to the resource
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetConnectionResult {
        /// ARN of the Glue Connection.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Catalog ID of the Glue Connection.
        pub catalog_id: pulumi_wasm_rust::Output<String>,
        pub connection_properties: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Type of Glue Connection.
        pub connection_type: pulumi_wasm_rust::Output<String>,
        /// Description of the connection.
        pub description: pulumi_wasm_rust::Output<String>,
        pub id: pulumi_wasm_rust::Output<String>,
        /// A list of criteria that can be used in selecting this connection.
        pub match_criterias: pulumi_wasm_rust::Output<Vec<String>>,
        /// Name of the Glue Connection.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A map of physical connection requirements, such as VPC and SecurityGroup.
        pub physical_connection_requirements: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::glue::GetConnectionPhysicalConnectionRequirement,
            >,
        >,
        /// Tags assigned to the resource
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetConnectionArgs) -> GetConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let id_binding = args.id.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:glue/getConnection:getConnection".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
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
                    name: "catalogId".into(),
                },
                register_interface::ResultField {
                    name: "connectionProperties".into(),
                },
                register_interface::ResultField {
                    name: "connectionType".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "matchCriterias".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "physicalConnectionRequirements".into(),
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
        GetConnectionResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            catalog_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("catalogId").unwrap(),
            ),
            connection_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionProperties").unwrap(),
            ),
            connection_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionType").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            match_criterias: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("matchCriterias").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            physical_connection_requirements: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("physicalConnectionRequirements").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
