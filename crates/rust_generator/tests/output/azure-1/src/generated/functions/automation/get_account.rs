#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_account {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAccountArgs {
        /// The name of the Automation Account.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Resource Group where the Automation Account exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetAccountResult {
        /// The Endpoint for this Automation Account.
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// The URL of automation hybrid service which is used for hybrid worker on-boarding With this Automation Account.
        pub hybrid_service_url: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// (Optional) An `identity` block as defined below.
        pub identities: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::automation::GetAccountIdentity>,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Primary Access Key for the Automation Account.
        pub primary_key: pulumi_gestalt_rust::Output<String>,
        pub private_endpoint_connections: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::automation::GetAccountPrivateEndpointConnection,
            >,
        >,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The Secondary Access Key for the Automation Account.
        pub secondary_key: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAccountArgs,
    ) -> GetAccountResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:automation/getAccount:getAccount".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetAccountResult {
            endpoint: o.get_field("endpoint"),
            hybrid_service_url: o.get_field("hybridServiceUrl"),
            id: o.get_field("id"),
            identities: o.get_field("identities"),
            name: o.get_field("name"),
            primary_key: o.get_field("primaryKey"),
            private_endpoint_connections: o.get_field("privateEndpointConnections"),
            resource_group_name: o.get_field("resourceGroupName"),
            secondary_key: o.get_field("secondaryKey"),
        }
    }
}
