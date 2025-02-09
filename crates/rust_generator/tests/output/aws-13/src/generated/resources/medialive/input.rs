/// Resource for managing an AWS MediaLive Input.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:medialive:InputSecurityGroup
///     properties:
///       whitelistRules:
///         - cidr: 10.0.0.8/32
///       tags:
///         ENVIRONMENT: prod
///   exampleInput:
///     type: aws:medialive:Input
///     name: example
///     properties:
///       name: example-input
///       inputSecurityGroups:
///         - ${example.id}
///       type: UDP_PUSH
///       tags:
///         ENVIRONMENT: prod
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import MediaLive Input using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:medialive/input:Input example 12345678
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod input {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InputArgs {
        /// Destination settings for PUSH type inputs. See Destinations for more details.
        #[builder(into, default)]
        pub destinations: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::medialive::InputDestination>>,
        >,
        /// Settings for the devices. See Input Devices for more details.
        #[builder(into, default)]
        pub input_devices: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::medialive::InputInputDevice>>,
        >,
        /// List of input security groups.
        #[builder(into, default)]
        pub input_security_groups: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// A list of the MediaConnect Flows. See Media Connect Flows for more details.
        #[builder(into, default)]
        pub media_connect_flows: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::medialive::InputMediaConnectFlow>>,
        >,
        /// Name of the input.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ARN of the role this input assumes during and after creation.
        #[builder(into, default)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The source URLs for a PULL-type input. See Sources for more details.
        #[builder(into, default)]
        pub sources: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::medialive::InputSource>>,
        >,
        /// A map of tags to assign to the Input. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The different types of inputs that AWS Elemental MediaLive supports.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Settings for a private VPC Input. See VPC for more details.
        #[builder(into, default)]
        pub vpc: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::medialive::InputVpc>,
        >,
    }
    #[allow(dead_code)]
    pub struct InputResult {
        /// ARN of the Input.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Channels attached to Input.
        pub attached_channels: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Destination settings for PUSH type inputs. See Destinations for more details.
        pub destinations: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::medialive::InputDestination>>,
        >,
        /// The input class.
        pub input_class: pulumi_gestalt_rust::Output<String>,
        /// Settings for the devices. See Input Devices for more details.
        pub input_devices: pulumi_gestalt_rust::Output<
            Vec<super::super::types::medialive::InputInputDevice>,
        >,
        /// A list of IDs for all Inputs which are partners of this one.
        pub input_partner_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// List of input security groups.
        pub input_security_groups: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Source type of the input.
        pub input_source_type: pulumi_gestalt_rust::Output<String>,
        /// A list of the MediaConnect Flows. See Media Connect Flows for more details.
        pub media_connect_flows: pulumi_gestalt_rust::Output<
            Vec<super::super::types::medialive::InputMediaConnectFlow>,
        >,
        /// Name of the input.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the role this input assumes during and after creation.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// The source URLs for a PULL-type input. See Sources for more details.
        pub sources: pulumi_gestalt_rust::Output<
            Vec<super::super::types::medialive::InputSource>,
        >,
        /// A map of tags to assign to the Input. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The different types of inputs that AWS Elemental MediaLive supports.
        ///
        /// The following arguments are optional:
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// Settings for a private VPC Input. See VPC for more details.
        pub vpc: pulumi_gestalt_rust::Output<
            Option<super::super::types::medialive::InputVpc>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: InputArgs,
    ) -> InputResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let destinations_binding_1 = args.destinations.get_output(context);
        let destinations_binding = destinations_binding_1.get_inner();
        let input_devices_binding_1 = args.input_devices.get_output(context);
        let input_devices_binding = input_devices_binding_1.get_inner();
        let input_security_groups_binding_1 = args
            .input_security_groups
            .get_output(context);
        let input_security_groups_binding = input_security_groups_binding_1.get_inner();
        let media_connect_flows_binding_1 = args.media_connect_flows.get_output(context);
        let media_connect_flows_binding = media_connect_flows_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let role_arn_binding_1 = args.role_arn.get_output(context);
        let role_arn_binding = role_arn_binding_1.get_inner();
        let sources_binding_1 = args.sources.get_output(context);
        let sources_binding = sources_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let type__binding_1 = args.type_.get_output(context);
        let type__binding = type__binding_1.get_inner();
        let vpc_binding_1 = args.vpc.get_output(context);
        let vpc_binding = vpc_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:medialive/input:Input".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "destinations".into(),
                    value: &destinations_binding,
                },
                register_interface::ObjectField {
                    name: "inputDevices".into(),
                    value: &input_devices_binding,
                },
                register_interface::ObjectField {
                    name: "inputSecurityGroups".into(),
                    value: &input_security_groups_binding,
                },
                register_interface::ObjectField {
                    name: "mediaConnectFlows".into(),
                    value: &media_connect_flows_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "sources".into(),
                    value: &sources_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
                register_interface::ObjectField {
                    name: "vpc".into(),
                    value: &vpc_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        InputResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            attached_channels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("attachedChannels"),
            ),
            destinations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("destinations"),
            ),
            input_class: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("inputClass"),
            ),
            input_devices: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("inputDevices"),
            ),
            input_partner_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("inputPartnerIds"),
            ),
            input_security_groups: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("inputSecurityGroups"),
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
            sources: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sources"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
            vpc: pulumi_gestalt_rust::__private::into_domain(o.extract_field("vpc")),
        }
    }
}
