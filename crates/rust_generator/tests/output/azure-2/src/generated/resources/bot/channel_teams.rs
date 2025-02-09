/// Manages a MS Teams integration for a Bot Channel
///
/// > **Note** A bot can only have a single MS Teams Channel associated with it.
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
///   exampleChannelsRegistration:
///     type: azure:bot:ChannelsRegistration
///     name: example
///     properties:
///       name: example
///       location: global
///       resourceGroupName: ${example.name}
///       sku: F0
///       microsoftAppId: ${current.clientId}
///   exampleChannelTeams:
///     type: azure:bot:ChannelTeams
///     name: example
///     properties:
///       botName: ${exampleChannelsRegistration.name}
///       location: ${exampleChannelsRegistration.location}
///       resourceGroupName: ${example.name}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// The Microsoft Teams Integration for a Bot Channel can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:bot/channelTeams:ChannelTeams example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.BotService/botServices/example/channels/MsTeamsChannel
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod channel_teams {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ChannelTeamsArgs {
        /// The name of the Bot Resource this channel will be associated with. Changing this forces a new resource to be created.
        #[builder(into)]
        pub bot_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the webhook for Microsoft Teams channel calls.
        #[builder(into, default)]
        pub calling_web_hook: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The deployment environment for Microsoft Teams channel calls. Possible values are `CommercialDeployment` and `GCCModerateDeployment`. Defaults to `CommercialDeployment`.
        #[builder(into, default)]
        pub deployment_environment: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies whether to enable Microsoft Teams channel calls. This defaults to `false`.
        #[builder(into, default)]
        pub enable_calling: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the Bot Channel. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ChannelTeamsResult {
        /// The name of the Bot Resource this channel will be associated with. Changing this forces a new resource to be created.
        pub bot_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the webhook for Microsoft Teams channel calls.
        pub calling_web_hook: pulumi_gestalt_rust::Output<String>,
        /// The deployment environment for Microsoft Teams channel calls. Possible values are `CommercialDeployment` and `GCCModerateDeployment`. Defaults to `CommercialDeployment`.
        pub deployment_environment: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies whether to enable Microsoft Teams channel calls. This defaults to `false`.
        pub enable_calling: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which to create the Bot Channel. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ChannelTeamsArgs,
    ) -> ChannelTeamsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bot_name_binding = args.bot_name.get_output(context);
        let calling_web_hook_binding = args.calling_web_hook.get_output(context);
        let deployment_environment_binding = args
            .deployment_environment
            .get_output(context);
        let enable_calling_binding = args.enable_calling.get_output(context);
        let location_binding = args.location.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:bot/channelTeams:ChannelTeams".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "botName".into(),
                    value: bot_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "callingWebHook".into(),
                    value: calling_web_hook_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deploymentEnvironment".into(),
                    value: deployment_environment_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableCalling".into(),
                    value: enable_calling_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ChannelTeamsResult {
            bot_name: o.get_field("botName"),
            calling_web_hook: o.get_field("callingWebHook"),
            deployment_environment: o.get_field("deploymentEnvironment"),
            enable_calling: o.get_field("enableCalling"),
            location: o.get_field("location"),
            resource_group_name: o.get_field("resourceGroupName"),
        }
    }
}
