/// Manages a Purview Account.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleAccount:
///     type: azure:purview:Account
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       identity:
///         type: SystemAssigned
/// ```
///
/// ## Import
///
/// Purview Accounts can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:purview/account:Account example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Purview/accounts/account1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod account {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountArgs {
        /// An `identity` block as defined below.
        #[builder(into)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::purview::AccountIdentity,
        >,
        /// The Azure Region where the Purview Account should exist. Changing this forces a new Purview Account to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for the new Resource Group where Purview Account creates the managed resources. Changing this forces a new Purview Account to be created.
        ///
        /// > **Note:** `managed_resource_group_name` must be a new Resource Group
        #[builder(into, default)]
        pub managed_resource_group_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The name which should be used for this Purview Account. Changing this forces a new Purview Account to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Should the Purview Account be visible to the public network? Defaults to `true`.
        #[builder(into, default)]
        pub public_network_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the Resource Group where the Purview Account should exist. Changing this forces a new Purview Account to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Purview Account.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AccountResult {
        /// Atlas Kafka endpoint primary connection string.
        pub atlas_kafka_endpoint_primary_connection_string: pulumi_gestalt_rust::Output<
            String,
        >,
        /// Atlas Kafka endpoint secondary connection string.
        pub atlas_kafka_endpoint_secondary_connection_string: pulumi_gestalt_rust::Output<
            String,
        >,
        /// Catalog endpoint.
        pub catalog_endpoint: pulumi_gestalt_rust::Output<String>,
        /// Guardian endpoint.
        pub guardian_endpoint: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            super::super::types::purview::AccountIdentity,
        >,
        /// The Azure Region where the Purview Account should exist. Changing this forces a new Purview Account to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for the new Resource Group where Purview Account creates the managed resources. Changing this forces a new Purview Account to be created.
        ///
        /// > **Note:** `managed_resource_group_name` must be a new Resource Group
        pub managed_resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `managed_resources` block as defined below.
        pub managed_resources: pulumi_gestalt_rust::Output<
            Vec<super::super::types::purview::AccountManagedResource>,
        >,
        /// The name which should be used for this Purview Account. Changing this forces a new Purview Account to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Should the Purview Account be visible to the public network? Defaults to `true`.
        pub public_network_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the Resource Group where the Purview Account should exist. Changing this forces a new Purview Account to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Scan endpoint.
        pub scan_endpoint: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Purview Account.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccountArgs,
    ) -> AccountResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let identity_binding = args.identity.get_output(context);
        let location_binding = args.location.get_output(context);
        let managed_resource_group_name_binding = args
            .managed_resource_group_name
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let public_network_enabled_binding = args
            .public_network_enabled
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:purview/account:Account".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: &identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managedResourceGroupName".into(),
                    value: &managed_resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicNetworkEnabled".into(),
                    value: &public_network_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AccountResult {
            atlas_kafka_endpoint_primary_connection_string: o
                .get_field("atlasKafkaEndpointPrimaryConnectionString"),
            atlas_kafka_endpoint_secondary_connection_string: o
                .get_field("atlasKafkaEndpointSecondaryConnectionString"),
            catalog_endpoint: o.get_field("catalogEndpoint"),
            guardian_endpoint: o.get_field("guardianEndpoint"),
            identity: o.get_field("identity"),
            location: o.get_field("location"),
            managed_resource_group_name: o.get_field("managedResourceGroupName"),
            managed_resources: o.get_field("managedResources"),
            name: o.get_field("name"),
            public_network_enabled: o.get_field("publicNetworkEnabled"),
            resource_group_name: o.get_field("resourceGroupName"),
            scan_endpoint: o.get_field("scanEndpoint"),
            tags: o.get_field("tags"),
        }
    }
}
