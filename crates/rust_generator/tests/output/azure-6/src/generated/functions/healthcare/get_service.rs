pub mod get_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServiceArgs {
        /// The Azure Region where the Service is located.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Healthcare Service.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group in which the Healthcare Service exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetServiceResult {
        pub access_policy_object_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// An `authentication_configuration` block as defined below.
        pub authentication_configurations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::healthcare::GetServiceAuthenticationConfiguration,
            >,
        >,
        /// A `cors_configuration` block as defined below.
        pub cors_configurations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::healthcare::GetServiceCorsConfiguration>,
        >,
        /// The versionless Key Vault Key ID for CMK encryption of the backing database.
        pub cosmosdb_key_vault_key_versionless_id: pulumi_gestalt_rust::Output<String>,
        /// The provisioned throughput for the backing database.
        pub cosmosdb_throughput: pulumi_gestalt_rust::Output<i32>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The type of the service.
        pub kind: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Service is located.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetServiceArgs,
    ) -> GetServiceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:healthcare/getService:getService".into(),
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
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetServiceResult {
            access_policy_object_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accessPolicyObjectIds"),
            ),
            authentication_configurations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("authenticationConfigurations"),
            ),
            cors_configurations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("corsConfigurations"),
            ),
            cosmosdb_key_vault_key_versionless_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cosmosdbKeyVaultKeyVersionlessId"),
            ),
            cosmosdb_throughput: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cosmosdbThroughput"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            kind: pulumi_gestalt_rust::__private::into_domain(o.extract_field("kind")),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
