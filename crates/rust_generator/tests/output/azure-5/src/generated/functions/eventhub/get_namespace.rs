#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_namespace {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNamespaceArgs {
        /// The name of the EventHub Namespace.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Name of the Resource Group where the EventHub Namespace exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetNamespaceResult {
        /// Is Auto Inflate enabled for the EventHub Namespace?
        pub auto_inflate_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The Capacity / Throughput Units for a `Standard` SKU namespace.
        pub capacity: pulumi_gestalt_rust::Output<i32>,
        /// The ID of the EventHub Dedicated Cluster where this Namespace exists.
        pub dedicated_cluster_id: pulumi_gestalt_rust::Output<String>,
        /// The primary connection string for the authorization
        /// rule `RootManageSharedAccessKey`.
        pub default_primary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The alias of the primary connection string for the authorization
        /// rule `RootManageSharedAccessKey`.
        pub default_primary_connection_string_alias: pulumi_gestalt_rust::Output<String>,
        /// The primary access key for the authorization rule `RootManageSharedAccessKey`.
        pub default_primary_key: pulumi_gestalt_rust::Output<String>,
        /// The secondary connection string for the
        /// authorization rule `RootManageSharedAccessKey`.
        pub default_secondary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The alias of the secondary connection string for the
        /// authorization rule `RootManageSharedAccessKey`.
        pub default_secondary_connection_string_alias: pulumi_gestalt_rust::Output<
            String,
        >,
        /// The secondary access key for the authorization rule `RootManageSharedAccessKey`.
        pub default_secondary_key: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub kafka_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The Azure location where the EventHub Namespace exists
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the maximum number of throughput units when Auto Inflate is Enabled.
        pub maximum_throughput_units: pulumi_gestalt_rust::Output<i32>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Defines which tier to use.
        pub sku: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the EventHub Namespace.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetNamespaceArgs,
    ) -> GetNamespaceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:eventhub/getNamespace:getNamespace".into(),
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
        GetNamespaceResult {
            auto_inflate_enabled: o.get_field("autoInflateEnabled"),
            capacity: o.get_field("capacity"),
            dedicated_cluster_id: o.get_field("dedicatedClusterId"),
            default_primary_connection_string: o
                .get_field("defaultPrimaryConnectionString"),
            default_primary_connection_string_alias: o
                .get_field("defaultPrimaryConnectionStringAlias"),
            default_primary_key: o.get_field("defaultPrimaryKey"),
            default_secondary_connection_string: o
                .get_field("defaultSecondaryConnectionString"),
            default_secondary_connection_string_alias: o
                .get_field("defaultSecondaryConnectionStringAlias"),
            default_secondary_key: o.get_field("defaultSecondaryKey"),
            id: o.get_field("id"),
            kafka_enabled: o.get_field("kafkaEnabled"),
            location: o.get_field("location"),
            maximum_throughput_units: o.get_field("maximumThroughputUnits"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            sku: o.get_field("sku"),
            tags: o.get_field("tags"),
        }
    }
}
