/// Manages a Voice Services Communications Gateway Test Line.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Central US
///   exampleServicesCommunicationsGateway:
///     type: azure:voice:ServicesCommunicationsGateway
///     name: example
///     properties:
///       name: example-vcg
///       resourceGroupName: ${example.name}
///   exampleServicesCommunicationsGatewayTestLine:
///     type: azure:voice:ServicesCommunicationsGatewayTestLine
///     name: example
///     properties:
///       name: example-vtl
///       location: West Central US
///       voiceServicesCommunicationsGatewayId: ${exampleServicesCommunicationsGateway.id}
///       phoneNumber: '123456789'
///       purpose: Automated
///       tags:
///         key: value
/// ```
///
/// ## Import
///
/// Voice Services Communications Gateway Test Line can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:voice/servicesCommunicationsGatewayTestLine:ServicesCommunicationsGatewayTestLine example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.VoiceServices/communicationsGateways/communicationsGateway1/testLines/testLine1
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod services_communications_gateway_test_line {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServicesCommunicationsGatewayTestLineArgs {
        /// Specifies the Azure Region where the Voice Services Communications Gateway Test Line should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name which should be used for this Voice Services Communications Gateway Test Line. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the phone number.
        #[builder(into)]
        pub phone_number: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The purpose of the Voice Services Communications Gateway Test Line. Possible values are `Automated` or `Manual`.
        #[builder(into)]
        pub purpose: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Voice Services Communications Gateway Test Line.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the ID of the Voice Services Communications Gateway. Changing this forces a new resource to be created.
        #[builder(into)]
        pub voice_services_communications_gateway_id: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
    }
    #[allow(dead_code)]
    pub struct ServicesCommunicationsGatewayTestLineResult {
        /// Specifies the Azure Region where the Voice Services Communications Gateway Test Line should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name which should be used for this Voice Services Communications Gateway Test Line. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the phone number.
        pub phone_number: pulumi_gestalt_rust::Output<String>,
        /// The purpose of the Voice Services Communications Gateway Test Line. Possible values are `Automated` or `Manual`.
        pub purpose: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Voice Services Communications Gateway Test Line.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the ID of the Voice Services Communications Gateway. Changing this forces a new resource to be created.
        pub voice_services_communications_gateway_id: pulumi_gestalt_rust::Output<
            String,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ServicesCommunicationsGatewayTestLineArgs,
    ) -> ServicesCommunicationsGatewayTestLineResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let phone_number_binding = args.phone_number.get_output(context).get_inner();
        let purpose_binding = args.purpose.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let voice_services_communications_gateway_id_binding = args
            .voice_services_communications_gateway_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:voice/servicesCommunicationsGatewayTestLine:ServicesCommunicationsGatewayTestLine"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "phoneNumber".into(),
                    value: &phone_number_binding,
                },
                register_interface::ObjectField {
                    name: "purpose".into(),
                    value: &purpose_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "voiceServicesCommunicationsGatewayId".into(),
                    value: &voice_services_communications_gateway_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ServicesCommunicationsGatewayTestLineResult {
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            phone_number: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("phoneNumber"),
            ),
            purpose: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("purpose"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            voice_services_communications_gateway_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("voiceServicesCommunicationsGatewayId"),
            ),
        }
    }
}
