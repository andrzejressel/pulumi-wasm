#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_input {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInputArgs {
        /// The ID of the Input.
        #[builder(into)]
        pub id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetInputResult {
        /// ARN of the Input.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Channels attached to Input.
        pub attached_channels: pulumi_gestalt_rust::Output<Vec<String>>,
        pub destinations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::medialive::GetInputDestination>,
        >,
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The input class.
        pub input_class: pulumi_gestalt_rust::Output<String>,
        /// Settings for the devices.
        pub input_devices: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::medialive::GetInputInputDevice>,
        >,
        /// A list of IDs for all Inputs which are partners of this one.
        pub input_partner_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Source type of the input.
        pub input_source_type: pulumi_gestalt_rust::Output<String>,
        /// A list of the MediaConnect Flows.
        pub media_connect_flows: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::medialive::GetInputMediaConnectFlow>,
        >,
        /// Name of the input.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the role this input assumes during and after creation.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// List of input security groups.
        pub security_groups: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The source URLs for a PULL-type input.
        pub sources: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::medialive::GetInputSource>,
        >,
        /// The state of the input.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// A map of tags assigned to the Input.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The type of the input.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetInputArgs,
    ) -> GetInputResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let id_binding = args.id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:medialive/getInput:getInput".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetInputResult {
            arn: o.get_field("arn"),
            attached_channels: o.get_field("attachedChannels"),
            destinations: o.get_field("destinations"),
            id: o.get_field("id"),
            input_class: o.get_field("inputClass"),
            input_devices: o.get_field("inputDevices"),
            input_partner_ids: o.get_field("inputPartnerIds"),
            input_source_type: o.get_field("inputSourceType"),
            media_connect_flows: o.get_field("mediaConnectFlows"),
            name: o.get_field("name"),
            role_arn: o.get_field("roleArn"),
            security_groups: o.get_field("securityGroups"),
            sources: o.get_field("sources"),
            state: o.get_field("state"),
            tags: o.get_field("tags"),
            type_: o.get_field("type"),
        }
    }
}
