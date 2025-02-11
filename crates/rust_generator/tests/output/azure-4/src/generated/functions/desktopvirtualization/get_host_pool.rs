#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_host_pool {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetHostPoolArgs {
        /// The name of the Virtual Desktop Host Pool to retrieve.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the resource group where the Virtual Desktop Host Pool exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetHostPoolResult {
        /// The custom RDP properties string for the Virtual Desktop Host Pool.
        pub custom_rdp_properties: pulumi_gestalt_rust::Output<String>,
        /// The description for the Virtual Desktop Host Pool.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The friendly name for the Virtual Desktop Host Pool.
        pub friendly_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The type of load balancing performed by the Host Pool
        pub load_balancer_type: pulumi_gestalt_rust::Output<String>,
        /// The location/region where the Virtual Desktop Host Pool is located.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The maximum number of users that can have concurrent sessions on a session host.
        pub maximum_sessions_allowed: pulumi_gestalt_rust::Output<i32>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The type of personal desktop assignment in use by the Host Pool
        pub personal_desktop_assignment_type: pulumi_gestalt_rust::Output<String>,
        /// The preferred Application Group type for the Virtual Desktop Host Pool.
        pub preferred_app_group_type: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `scheduled_agent_updates` block as defined below.
        pub scheduled_agent_updates: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::desktopvirtualization::GetHostPoolScheduledAgentUpdate,
            >,
        >,
        /// Returns `true` if the Start VM on Connection Feature is enabled.
        pub start_vm_on_connect: pulumi_gestalt_rust::Output<bool>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The type of the Virtual Desktop Host Pool.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// Returns `true` if the Host Pool is in Validation mode.
        pub validate_environment: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetHostPoolArgs,
    ) -> GetHostPoolResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:desktopvirtualization/getHostPool:getHostPool".into(),
            version: super::super::super::get_version(),
            object: &[
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
        GetHostPoolResult {
            custom_rdp_properties: o.get_field("customRdpProperties"),
            description: o.get_field("description"),
            friendly_name: o.get_field("friendlyName"),
            id: o.get_field("id"),
            load_balancer_type: o.get_field("loadBalancerType"),
            location: o.get_field("location"),
            maximum_sessions_allowed: o.get_field("maximumSessionsAllowed"),
            name: o.get_field("name"),
            personal_desktop_assignment_type: o
                .get_field("personalDesktopAssignmentType"),
            preferred_app_group_type: o.get_field("preferredAppGroupType"),
            resource_group_name: o.get_field("resourceGroupName"),
            scheduled_agent_updates: o.get_field("scheduledAgentUpdates"),
            start_vm_on_connect: o.get_field("startVmOnConnect"),
            tags: o.get_field("tags"),
            type_: o.get_field("type"),
            validate_environment: o.get_field("validateEnvironment"),
        }
    }
}
