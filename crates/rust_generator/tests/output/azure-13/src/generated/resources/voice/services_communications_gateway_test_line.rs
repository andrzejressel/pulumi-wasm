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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServicesCommunicationsGatewayTestLineArgs,
    ) -> ServicesCommunicationsGatewayTestLineResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let phone_number_binding = args.phone_number.get_output(context);
        let purpose_binding = args.purpose.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let voice_services_communications_gateway_id_binding = args
            .voice_services_communications_gateway_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:voice/servicesCommunicationsGatewayTestLine:ServicesCommunicationsGatewayTestLine"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "phoneNumber".into(),
                    value: &phone_number_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "purpose".into(),
                    value: &purpose_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "voiceServicesCommunicationsGatewayId".into(),
                    value: &voice_services_communications_gateway_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ServicesCommunicationsGatewayTestLineResult {
            location: o.get_field("location"),
            name: o.get_field("name"),
            phone_number: o.get_field("phoneNumber"),
            purpose: o.get_field("purpose"),
            tags: o.get_field("tags"),
            voice_services_communications_gateway_id: o
                .get_field("voiceServicesCommunicationsGatewayId"),
        }
    }
}
