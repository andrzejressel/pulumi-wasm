#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        args: GetServiceArgs,
    ) -> GetServiceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:healthcare/getService:getService".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetServiceResult {
            access_policy_object_ids: o.get_field("accessPolicyObjectIds"),
            authentication_configurations: o.get_field("authenticationConfigurations"),
            cors_configurations: o.get_field("corsConfigurations"),
            cosmosdb_key_vault_key_versionless_id: o
                .get_field("cosmosdbKeyVaultKeyVersionlessId"),
            cosmosdb_throughput: o.get_field("cosmosdbThroughput"),
            id: o.get_field("id"),
            kind: o.get_field("kind"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
        }
    }
}
