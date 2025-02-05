pub mod get_enterprise_database {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEnterpriseDatabaseArgs {
        /// The resource ID of Redis Enterprise Cluster which hosts the Redis Enterprise Database instance.
        #[builder(into)]
        pub cluster_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Redis Enterprise Database.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetEnterpriseDatabaseResult {
        /// The Redis Enterprise Cluster ID that is hosting the Redis Enterprise Database.
        pub cluster_id: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The Linked Database Group Nickname for the Redis Enterprise Database instance.
        pub linked_database_group_nickname: pulumi_wasm_rust::Output<String>,
        /// The Linked Database list for the Redis Enterprise Database instance.
        pub linked_database_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// The Redis Enterprise Database name.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Primary Access Key for the Redis Enterprise Database instance.
        pub primary_access_key: pulumi_wasm_rust::Output<String>,
        /// The Secondary Access Key for the Redis Enterprise Database instance.
        pub secondary_access_key: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetEnterpriseDatabaseArgs,
    ) -> GetEnterpriseDatabaseResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cluster_id_binding = args.cluster_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:redis/getEnterpriseDatabase:getEnterpriseDatabase".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterId".into(),
                    value: &cluster_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetEnterpriseDatabaseResult {
            cluster_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clusterId"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            linked_database_group_nickname: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("linkedDatabaseGroupNickname"),
            ),
            linked_database_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("linkedDatabaseIds"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            primary_access_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryAccessKey"),
            ),
            secondary_access_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryAccessKey"),
            ),
        }
    }
}
