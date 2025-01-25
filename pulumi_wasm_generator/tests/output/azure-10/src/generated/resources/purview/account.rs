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
pub mod account {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountArgs {
        /// An `identity` block as defined below.
        #[builder(into)]
        pub identity: pulumi_wasm_rust::InputOrOutput<
            super::super::types::purview::AccountIdentity,
        >,
        /// The Azure Region where the Purview Account should exist. Changing this forces a new Purview Account to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for the new Resource Group where Purview Account creates the managed resources. Changing this forces a new Purview Account to be created.
        ///
        /// > **Note:** `managed_resource_group_name` must be a new Resource Group
        #[builder(into, default)]
        pub managed_resource_group_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Purview Account. Changing this forces a new Purview Account to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Should the Purview Account be visible to the public network? Defaults to `true`.
        #[builder(into, default)]
        pub public_network_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The name of the Resource Group where the Purview Account should exist. Changing this forces a new Purview Account to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Purview Account.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AccountResult {
        /// Atlas Kafka endpoint primary connection string.
        pub atlas_kafka_endpoint_primary_connection_string: pulumi_wasm_rust::Output<
            String,
        >,
        /// Atlas Kafka endpoint secondary connection string.
        pub atlas_kafka_endpoint_secondary_connection_string: pulumi_wasm_rust::Output<
            String,
        >,
        /// Catalog endpoint.
        pub catalog_endpoint: pulumi_wasm_rust::Output<String>,
        /// Guardian endpoint.
        pub guardian_endpoint: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            super::super::types::purview::AccountIdentity,
        >,
        /// The Azure Region where the Purview Account should exist. Changing this forces a new Purview Account to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for the new Resource Group where Purview Account creates the managed resources. Changing this forces a new Purview Account to be created.
        ///
        /// > **Note:** `managed_resource_group_name` must be a new Resource Group
        pub managed_resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `managed_resources` block as defined below.
        pub managed_resources: pulumi_wasm_rust::Output<
            Vec<super::super::types::purview::AccountManagedResource>,
        >,
        /// The name which should be used for this Purview Account. Changing this forces a new Purview Account to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Should the Purview Account be visible to the public network? Defaults to `true`.
        pub public_network_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the Resource Group where the Purview Account should exist. Changing this forces a new Purview Account to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Scan endpoint.
        pub scan_endpoint: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Purview Account.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AccountArgs,
    ) -> AccountResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let identity_binding = args.identity.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let managed_resource_group_name_binding = args
            .managed_resource_group_name
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let public_network_enabled_binding = args
            .public_network_enabled
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:purview/account:Account".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "managedResourceGroupName".into(),
                    value: &managed_resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "publicNetworkEnabled".into(),
                    value: &public_network_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "atlasKafkaEndpointPrimaryConnectionString".into(),
                },
                register_interface::ResultField {
                    name: "atlasKafkaEndpointSecondaryConnectionString".into(),
                },
                register_interface::ResultField {
                    name: "catalogEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "guardianEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "managedResourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "managedResources".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "publicNetworkEnabled".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "scanEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AccountResult {
            atlas_kafka_endpoint_primary_connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("atlasKafkaEndpointPrimaryConnectionString").unwrap(),
            ),
            atlas_kafka_endpoint_secondary_connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("atlasKafkaEndpointSecondaryConnectionString").unwrap(),
            ),
            catalog_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("catalogEndpoint").unwrap(),
            ),
            guardian_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("guardianEndpoint").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            managed_resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managedResourceGroupName").unwrap(),
            ),
            managed_resources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managedResources").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            public_network_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicNetworkEnabled").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            scan_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scanEndpoint").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
