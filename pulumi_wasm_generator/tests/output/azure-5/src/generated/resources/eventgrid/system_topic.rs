/// Manages an Event Grid System Topic.
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
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: examplestoracct
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///       tags:
///         environment: staging
///   exampleSystemTopic:
///     type: azure:eventgrid:SystemTopic
///     name: example
///     properties:
///       name: example-topic
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       sourceArmResourceId: ${exampleAccount.id}
///       topicType: Microsoft.Storage.StorageAccounts
/// ```
///
/// ## Import
///
/// Event Grid System Topic can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:eventgrid/systemTopic:SystemTopic example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.EventGrid/systemTopics/systemTopic1
/// ```
///
pub mod system_topic {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SystemTopicArgs {
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::eventgrid::SystemTopicIdentity>,
        >,
        /// The Azure Region where the Event Grid System Topic should exist. Changing this forces a new Event Grid System Topic to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Event Grid System Topic. Changing this forces a new Event Grid System Topic to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group where the Event Grid System Topic should exist. Changing this forces a new Event Grid System Topic to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Event Grid System Topic ARM Source. Changing this forces a new Event Grid System Topic to be created.
        #[builder(into)]
        pub source_arm_resource_id: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Topic Type of the Event Grid System Topic. The topic type is validated by Azure and there may be additional topic types beyond the following: `Microsoft.AppConfiguration.ConfigurationStores`, `Microsoft.Communication.CommunicationServices`, `Microsoft.ContainerRegistry.Registries`, `Microsoft.Devices.IoTHubs`, `Microsoft.EventGrid.Domains`, `Microsoft.EventGrid.Topics`, `Microsoft.Eventhub.Namespaces`, `Microsoft.KeyVault.vaults`, `Microsoft.MachineLearningServices.Workspaces`, `Microsoft.Maps.Accounts`, `Microsoft.Media.MediaServices`, `Microsoft.Resources.ResourceGroups`, `Microsoft.Resources.Subscriptions`, `Microsoft.ServiceBus.Namespaces`, `Microsoft.SignalRService.SignalR`, `Microsoft.Storage.StorageAccounts`, `Microsoft.Web.ServerFarms` and `Microsoft.Web.Sites`. Changing this forces a new Event Grid System Topic to be created.
        ///
        /// > **NOTE:** Some `topic_type`s (e.g. **Microsoft.Resources.Subscriptions**) requires location to be set to `Global` instead of a real location like `West US`.
        ///
        /// > **NOTE:** You can use Azure CLI to get a full list of the available topic types: `az eventgrid topic-type  list --output json | grep -w id`
        #[builder(into)]
        pub topic_type: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct SystemTopicResult {
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::eventgrid::SystemTopicIdentity>,
        >,
        /// The Azure Region where the Event Grid System Topic should exist. Changing this forces a new Event Grid System Topic to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The Metric ARM Resource ID of the Event Grid System Topic.
        pub metric_arm_resource_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Event Grid System Topic. Changing this forces a new Event Grid System Topic to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Event Grid System Topic should exist. Changing this forces a new Event Grid System Topic to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Event Grid System Topic ARM Source. Changing this forces a new Event Grid System Topic to be created.
        pub source_arm_resource_id: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Topic Type of the Event Grid System Topic. The topic type is validated by Azure and there may be additional topic types beyond the following: `Microsoft.AppConfiguration.ConfigurationStores`, `Microsoft.Communication.CommunicationServices`, `Microsoft.ContainerRegistry.Registries`, `Microsoft.Devices.IoTHubs`, `Microsoft.EventGrid.Domains`, `Microsoft.EventGrid.Topics`, `Microsoft.Eventhub.Namespaces`, `Microsoft.KeyVault.vaults`, `Microsoft.MachineLearningServices.Workspaces`, `Microsoft.Maps.Accounts`, `Microsoft.Media.MediaServices`, `Microsoft.Resources.ResourceGroups`, `Microsoft.Resources.Subscriptions`, `Microsoft.ServiceBus.Namespaces`, `Microsoft.SignalRService.SignalR`, `Microsoft.Storage.StorageAccounts`, `Microsoft.Web.ServerFarms` and `Microsoft.Web.Sites`. Changing this forces a new Event Grid System Topic to be created.
        ///
        /// > **NOTE:** Some `topic_type`s (e.g. **Microsoft.Resources.Subscriptions**) requires location to be set to `Global` instead of a real location like `West US`.
        ///
        /// > **NOTE:** You can use Azure CLI to get a full list of the available topic types: `az eventgrid topic-type  list --output json | grep -w id`
        pub topic_type: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SystemTopicArgs) -> SystemTopicResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let identity_binding = args.identity.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let source_arm_resource_id_binding = args.source_arm_resource_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let topic_type_binding = args.topic_type.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:eventgrid/systemTopic:SystemTopic".into(),
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
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "sourceArmResourceId".into(),
                    value: &source_arm_resource_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "topicType".into(),
                    value: &topic_type_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "metricArmResourceId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "sourceArmResourceId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "topicType".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SystemTopicResult {
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            metric_arm_resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metricArmResourceId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            source_arm_resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceArmResourceId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            topic_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("topicType").unwrap(),
            ),
        }
    }
}
