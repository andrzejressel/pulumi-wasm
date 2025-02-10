#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_service_bus_namespace {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServiceBusNamespaceArgs {
        /// Specifies the name of the ServiceBus Namespace.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Resource Group where the ServiceBus Namespace exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetServiceBusNamespaceResult {
        /// The capacity of the ServiceBus Namespace.
        pub capacity: pulumi_gestalt_rust::Output<i32>,
        /// The primary connection string for the authorization
        /// rule `RootManageSharedAccessKey`.
        pub default_primary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The primary access key for the authorization rule `RootManageSharedAccessKey`.
        pub default_primary_key: pulumi_gestalt_rust::Output<String>,
        /// The secondary connection string for the
        /// authorization rule `RootManageSharedAccessKey`.
        pub default_secondary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The secondary access key for the authorization rule `RootManageSharedAccessKey`.
        pub default_secondary_key: pulumi_gestalt_rust::Output<String>,
        /// The URL to access the ServiceBus Namespace.
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The location of the Resource Group in which the ServiceBus Namespace exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The messaging partitions of the ServiceBus Namespace.
        pub premium_messaging_partitions: pulumi_gestalt_rust::Output<i32>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The Tier used for the ServiceBus Namespace.
        pub sku: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetServiceBusNamespaceArgs,
    ) -> GetServiceBusNamespaceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:eventhub/getServiceBusNamespace:getServiceBusNamespace".into(),
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
        GetServiceBusNamespaceResult {
            capacity: o.get_field("capacity"),
            default_primary_connection_string: o
                .get_field("defaultPrimaryConnectionString"),
            default_primary_key: o.get_field("defaultPrimaryKey"),
            default_secondary_connection_string: o
                .get_field("defaultSecondaryConnectionString"),
            default_secondary_key: o.get_field("defaultSecondaryKey"),
            endpoint: o.get_field("endpoint"),
            id: o.get_field("id"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            premium_messaging_partitions: o.get_field("premiumMessagingPartitions"),
            resource_group_name: o.get_field("resourceGroupName"),
            sku: o.get_field("sku"),
            tags: o.get_field("tags"),
        }
    }
}
