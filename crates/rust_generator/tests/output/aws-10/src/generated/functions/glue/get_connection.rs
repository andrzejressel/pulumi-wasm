pub mod get_connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetConnectionArgs {
        /// Concatenation of the catalog ID and connection name. For example, if your account ID is
        /// `123456789123` and the connection name is `conn` then the ID is `123456789123:conn`.
        #[builder(into)]
        pub id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Tags assigned to the resource
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetConnectionArgs,
    ) -> GetConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let id_binding = args.id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:glue/getConnection:getConnection".into(),
            version: super::super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetConnectionResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            catalog_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("catalogId"),
            ),
            connection_properties: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("connectionProperties"),
            ),
            connection_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("connectionType"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            match_criterias: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("matchCriterias"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            physical_connection_requirements: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("physicalConnectionRequirements"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
