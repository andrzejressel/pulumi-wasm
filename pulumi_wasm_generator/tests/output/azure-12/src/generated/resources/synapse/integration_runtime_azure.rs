/// Manages a Synapse Azure Integration Runtime.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example
///       location: West Europe
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: example
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleContainer:
///     type: azure:storage:Container
///     name: example
///     properties:
///       name: content
///       storageAccountName: ${exampleAccount.name}
///       containerAccessType: private
///   exampleDataLakeGen2Filesystem:
///     type: azure:storage:DataLakeGen2Filesystem
///     name: example
///     properties:
///       name: example
///       storageAccountId: ${exampleAccount.id}
///   exampleWorkspace:
///     type: azure:synapse:Workspace
///     name: example
///     properties:
///       name: example
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       storageDataLakeGen2FilesystemId: ${exampleDataLakeGen2Filesystem.id}
///       sqlAdministratorLogin: sqladminuser
///       sqlAdministratorLoginPassword: H@Sh1CoR3!
///       managedVirtualNetworkEnabled: true
///       identity:
///         type: SystemAssigned
///   exampleFirewallRule:
///     type: azure:synapse:FirewallRule
///     name: example
///     properties:
///       name: AllowAll
///       synapseWorkspaceId: ${exampleWorkspace.id}
///       startIpAddress: 0.0.0.0
///       endIpAddress: 255.255.255.255
///   exampleIntegrationRuntimeAzure:
///     type: azure:synapse:IntegrationRuntimeAzure
///     name: example
///     properties:
///       name: example
///       synapseWorkspaceId: ${exampleWorkspace.id}
///       location: ${example.location}
/// ```
///
/// ## Import
///
/// Synapse Azure Integration Runtimes can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:synapse/integrationRuntimeAzure:IntegrationRuntimeAzure example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.Synapse/workspaces/workspace1/integrationRuntimes/IntegrationRuntime1
/// ```
///
pub mod integration_runtime_azure {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IntegrationRuntimeAzureArgs {
        /// Compute type of the cluster which will execute data flow job. Valid values are `General`, `ComputeOptimized` and `MemoryOptimized`. Defaults to `General`.
        #[builder(into, default)]
        pub compute_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Core count of the cluster which will execute data flow job. Valid values are `8`, `16`, `32`, `48`, `80`, `144` and `272`. Defaults to `8`.
        #[builder(into, default)]
        pub core_count: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Integration runtime description.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Azure Region where the Synapse Azure Integration Runtime should exist. Use `AutoResolve` to create an auto-resolve integration runtime. Changing this forces a new Synapse Azure Integration Runtime to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Synapse Azure Integration Runtime. Changing this forces a new Synapse Azure Integration Runtime to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Synapse Workspace ID in which to associate the Integration Runtime with. Changing this forces a new Synapse Azure Integration Runtime to be created.
        #[builder(into)]
        pub synapse_workspace_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Time to live (in minutes) setting of the cluster which will execute data flow job. Defaults to `0`.
        #[builder(into, default)]
        pub time_to_live_min: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct IntegrationRuntimeAzureResult {
        /// Compute type of the cluster which will execute data flow job. Valid values are `General`, `ComputeOptimized` and `MemoryOptimized`. Defaults to `General`.
        pub compute_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Core count of the cluster which will execute data flow job. Valid values are `8`, `16`, `32`, `48`, `80`, `144` and `272`. Defaults to `8`.
        pub core_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// Integration runtime description.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The Azure Region where the Synapse Azure Integration Runtime should exist. Use `AutoResolve` to create an auto-resolve integration runtime. Changing this forces a new Synapse Azure Integration Runtime to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Synapse Azure Integration Runtime. Changing this forces a new Synapse Azure Integration Runtime to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Synapse Workspace ID in which to associate the Integration Runtime with. Changing this forces a new Synapse Azure Integration Runtime to be created.
        pub synapse_workspace_id: pulumi_wasm_rust::Output<String>,
        /// Time to live (in minutes) setting of the cluster which will execute data flow job. Defaults to `0`.
        pub time_to_live_min: pulumi_wasm_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: IntegrationRuntimeAzureArgs,
    ) -> IntegrationRuntimeAzureResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let compute_type_binding = args.compute_type.get_output(context).get_inner();
        let core_count_binding = args.core_count.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let synapse_workspace_id_binding = args
            .synapse_workspace_id
            .get_output(context)
            .get_inner();
        let time_to_live_min_binding = args
            .time_to_live_min
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:synapse/integrationRuntimeAzure:IntegrationRuntimeAzure"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "computeType".into(),
                    value: &compute_type_binding,
                },
                register_interface::ObjectField {
                    name: "coreCount".into(),
                    value: &core_count_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "synapseWorkspaceId".into(),
                    value: &synapse_workspace_id_binding,
                },
                register_interface::ObjectField {
                    name: "timeToLiveMin".into(),
                    value: &time_to_live_min_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "computeType".into(),
                },
                register_interface::ResultField {
                    name: "coreCount".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "synapseWorkspaceId".into(),
                },
                register_interface::ResultField {
                    name: "timeToLiveMin".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        IntegrationRuntimeAzureResult {
            compute_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("computeType").unwrap(),
            ),
            core_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("coreCount").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            synapse_workspace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("synapseWorkspaceId").unwrap(),
            ),
            time_to_live_min: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeToLiveMin").unwrap(),
            ),
        }
    }
}
