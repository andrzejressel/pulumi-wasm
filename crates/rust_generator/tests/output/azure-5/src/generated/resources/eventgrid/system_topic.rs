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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod system_topic {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SystemTopicArgs {
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::eventgrid::SystemTopicIdentity>,
        >,
        /// The Azure Region where the Event Grid System Topic should exist. Changing this forces a new Event Grid System Topic to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Event Grid System Topic. Changing this forces a new Event Grid System Topic to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Event Grid System Topic should exist. Changing this forces a new Event Grid System Topic to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Event Grid System Topic ARM Source. Changing this forces a new Event Grid System Topic to be created.
        #[builder(into)]
        pub source_arm_resource_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Topic Type of the Event Grid System Topic. The topic type is validated by Azure and there may be additional topic types beyond the following: `Microsoft.AppConfiguration.ConfigurationStores`, `Microsoft.Communication.CommunicationServices`, `Microsoft.ContainerRegistry.Registries`, `Microsoft.Devices.IoTHubs`, `Microsoft.EventGrid.Domains`, `Microsoft.EventGrid.Topics`, `Microsoft.Eventhub.Namespaces`, `Microsoft.KeyVault.vaults`, `Microsoft.MachineLearningServices.Workspaces`, `Microsoft.Maps.Accounts`, `Microsoft.Media.MediaServices`, `Microsoft.Resources.ResourceGroups`, `Microsoft.Resources.Subscriptions`, `Microsoft.ServiceBus.Namespaces`, `Microsoft.SignalRService.SignalR`, `Microsoft.Storage.StorageAccounts`, `Microsoft.Web.ServerFarms` and `Microsoft.Web.Sites`. Changing this forces a new Event Grid System Topic to be created.
        ///
        /// > **NOTE:** Some `topic_type`s (e.g. **Microsoft.Resources.Subscriptions**) requires location to be set to `Global` instead of a real location like `West US`.
        ///
        /// > **NOTE:** You can use Azure CLI to get a full list of the available topic types: `az eventgrid topic-type  list --output json | grep -w id`
        #[builder(into)]
        pub topic_type: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SystemTopicResult {
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::eventgrid::SystemTopicIdentity>,
        >,
        /// The Azure Region where the Event Grid System Topic should exist. Changing this forces a new Event Grid System Topic to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The Metric ARM Resource ID of the Event Grid System Topic.
        pub metric_arm_resource_id: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Event Grid System Topic. Changing this forces a new Event Grid System Topic to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Event Grid System Topic should exist. Changing this forces a new Event Grid System Topic to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Event Grid System Topic ARM Source. Changing this forces a new Event Grid System Topic to be created.
        pub source_arm_resource_id: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Topic Type of the Event Grid System Topic. The topic type is validated by Azure and there may be additional topic types beyond the following: `Microsoft.AppConfiguration.ConfigurationStores`, `Microsoft.Communication.CommunicationServices`, `Microsoft.ContainerRegistry.Registries`, `Microsoft.Devices.IoTHubs`, `Microsoft.EventGrid.Domains`, `Microsoft.EventGrid.Topics`, `Microsoft.Eventhub.Namespaces`, `Microsoft.KeyVault.vaults`, `Microsoft.MachineLearningServices.Workspaces`, `Microsoft.Maps.Accounts`, `Microsoft.Media.MediaServices`, `Microsoft.Resources.ResourceGroups`, `Microsoft.Resources.Subscriptions`, `Microsoft.ServiceBus.Namespaces`, `Microsoft.SignalRService.SignalR`, `Microsoft.Storage.StorageAccounts`, `Microsoft.Web.ServerFarms` and `Microsoft.Web.Sites`. Changing this forces a new Event Grid System Topic to be created.
        ///
        /// > **NOTE:** Some `topic_type`s (e.g. **Microsoft.Resources.Subscriptions**) requires location to be set to `Global` instead of a real location like `West US`.
        ///
        /// > **NOTE:** You can use Azure CLI to get a full list of the available topic types: `az eventgrid topic-type  list --output json | grep -w id`
        pub topic_type: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SystemTopicArgs,
    ) -> SystemTopicResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let identity_binding = args.identity.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let source_arm_resource_id_binding = args
            .source_arm_resource_id
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let topic_type_binding = args.topic_type.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:eventgrid/systemTopic:SystemTopic".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: identity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceArmResourceId".into(),
                    value: source_arm_resource_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "topicType".into(),
                    value: topic_type_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SystemTopicResult {
            identity: o.get_field("identity"),
            location: o.get_field("location"),
            metric_arm_resource_id: o.get_field("metricArmResourceId"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            source_arm_resource_id: o.get_field("sourceArmResourceId"),
            tags: o.get_field("tags"),
            topic_type: o.get_field("topicType"),
        }
    }
}
