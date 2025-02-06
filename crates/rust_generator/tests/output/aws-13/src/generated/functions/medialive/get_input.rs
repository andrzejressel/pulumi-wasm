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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetInputArgs,
    ) -> GetInputResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let id_binding = args.id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:medialive/getInput:getInput".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetInputResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            attached_channels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("attachedChannels"),
            ),
            destinations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("destinations"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            input_class: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("inputClass"),
            ),
            input_devices: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("inputDevices"),
            ),
            input_partner_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("inputPartnerIds"),
            ),
            input_source_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("inputSourceType"),
            ),
            media_connect_flows: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("mediaConnectFlows"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("roleArn"),
            ),
            security_groups: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityGroups"),
            ),
            sources: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sources"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
