/// Provides a Global Accelerator custom routing endpoint group.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = custom_routing_endpoint_group::create(
///         "example",
///         CustomRoutingEndpointGroupArgs::builder()
///             .destination_configurations(
///                 vec![
///                     CustomRoutingEndpointGroupDestinationConfiguration::builder()
///                     .fromPort(80).protocols(vec!["TCP",]).toPort(8080).build_struct(),
///                 ],
///             )
///             .endpoint_configurations(
///                 vec![
///                     CustomRoutingEndpointGroupEndpointConfiguration::builder()
///                     .endpointId("${exampleAwsSubnet.id}").build_struct(),
///                 ],
///             )
///             .listener_arn("${exampleAwsGlobalacceleratorCustomRoutingListener.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Global Accelerator custom routing endpoint groups using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:globalaccelerator/customRoutingEndpointGroup:CustomRoutingEndpointGroup example arn:aws:globalaccelerator::111111111111:accelerator/xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx/listener/xxxxxxx/endpoint-group/xxxxxxxx
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod custom_routing_endpoint_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomRoutingEndpointGroupArgs {
        /// The port ranges and protocols for all endpoints in a custom routing endpoint group to accept client traffic on. Fields documented below.
        #[builder(into)]
        pub destination_configurations: pulumi_gestalt_rust::InputOrOutput<
            Vec<
                super::super::types::globalaccelerator::CustomRoutingEndpointGroupDestinationConfiguration,
            >,
        >,
        /// The list of endpoint objects. Fields documented below.
        #[builder(into, default)]
        pub endpoint_configurations: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::globalaccelerator::CustomRoutingEndpointGroupEndpointConfiguration,
                >,
            >,
        >,
        /// The name of the AWS Region where the custom routing endpoint group is located.
        #[builder(into, default)]
        pub endpoint_group_region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Amazon Resource Name (ARN) of the custom routing listener.
        #[builder(into)]
        pub listener_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CustomRoutingEndpointGroupResult {
        /// The Amazon Resource Name (ARN) of the custom routing endpoint group.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The port ranges and protocols for all endpoints in a custom routing endpoint group to accept client traffic on. Fields documented below.
        pub destination_configurations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::globalaccelerator::CustomRoutingEndpointGroupDestinationConfiguration,
            >,
        >,
        /// The list of endpoint objects. Fields documented below.
        pub endpoint_configurations: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::globalaccelerator::CustomRoutingEndpointGroupEndpointConfiguration,
                >,
            >,
        >,
        /// The name of the AWS Region where the custom routing endpoint group is located.
        pub endpoint_group_region: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the custom routing listener.
        pub listener_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CustomRoutingEndpointGroupArgs,
    ) -> CustomRoutingEndpointGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let destination_configurations_binding = args
            .destination_configurations
            .get_output(context);
        let endpoint_configurations_binding = args
            .endpoint_configurations
            .get_output(context);
        let endpoint_group_region_binding = args
            .endpoint_group_region
            .get_output(context);
        let listener_arn_binding = args.listener_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:globalaccelerator/customRoutingEndpointGroup:CustomRoutingEndpointGroup"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinationConfigurations".into(),
                    value: destination_configurations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endpointConfigurations".into(),
                    value: endpoint_configurations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endpointGroupRegion".into(),
                    value: endpoint_group_region_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "listenerArn".into(),
                    value: listener_arn_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        CustomRoutingEndpointGroupResult {
            arn: o.get_field("arn"),
            destination_configurations: o.get_field("destinationConfigurations"),
            endpoint_configurations: o.get_field("endpointConfigurations"),
            endpoint_group_region: o.get_field("endpointGroupRegion"),
            listener_arn: o.get_field("listenerArn"),
        }
    }
}
