pub mod get_input {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInputArgs {
        /// The ID of the Input.
        #[builder(into)]
        pub id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetInputResult {
        /// ARN of the Input.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Channels attached to Input.
        pub attached_channels: pulumi_wasm_rust::Output<Vec<String>>,
        pub destinations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::medialive::GetInputDestination>,
        >,
        pub id: pulumi_wasm_rust::Output<String>,
        /// The input class.
        pub input_class: pulumi_wasm_rust::Output<String>,
        /// Settings for the devices.
        pub input_devices: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::medialive::GetInputInputDevice>,
        >,
        /// A list of IDs for all Inputs which are partners of this one.
        pub input_partner_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Source type of the input.
        pub input_source_type: pulumi_wasm_rust::Output<String>,
        /// A list of the MediaConnect Flows.
        pub media_connect_flows: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::medialive::GetInputMediaConnectFlow>,
        >,
        /// Name of the input.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ARN of the role this input assumes during and after creation.
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// List of input security groups.
        pub security_groups: pulumi_wasm_rust::Output<Vec<String>>,
        /// The source URLs for a PULL-type input.
        pub sources: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::medialive::GetInputSource>,
        >,
        /// The state of the input.
        pub state: pulumi_wasm_rust::Output<String>,
        /// A map of tags assigned to the Input.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The type of the input.
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetInputArgs,
    ) -> GetInputResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "attachedChannels".into(),
                },
                register_interface::ResultField {
                    name: "destinations".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "inputClass".into(),
                },
                register_interface::ResultField {
                    name: "inputDevices".into(),
                },
                register_interface::ResultField {
                    name: "inputPartnerIds".into(),
                },
                register_interface::ResultField {
                    name: "inputSourceType".into(),
                },
                register_interface::ResultField {
                    name: "mediaConnectFlows".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
                register_interface::ResultField {
                    name: "securityGroups".into(),
                },
                register_interface::ResultField {
                    name: "sources".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetInputResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            attached_channels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("attachedChannels").unwrap(),
            ),
            destinations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinations").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            input_class: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inputClass").unwrap(),
            ),
            input_devices: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inputDevices").unwrap(),
            ),
            input_partner_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inputPartnerIds").unwrap(),
            ),
            input_source_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inputSourceType").unwrap(),
            ),
            media_connect_flows: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mediaConnectFlows").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
            security_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroups").unwrap(),
            ),
            sources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sources").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
