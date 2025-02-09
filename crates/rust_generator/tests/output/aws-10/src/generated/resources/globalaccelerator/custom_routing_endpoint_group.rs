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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: CustomRoutingEndpointGroupArgs,
    ) -> CustomRoutingEndpointGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let destination_configurations_binding_1 = args
            .destination_configurations
            .get_output(context);
        let destination_configurations_binding = destination_configurations_binding_1
            .get_inner();
        let endpoint_configurations_binding_1 = args
            .endpoint_configurations
            .get_output(context);
        let endpoint_configurations_binding = endpoint_configurations_binding_1
            .get_inner();
        let endpoint_group_region_binding_1 = args
            .endpoint_group_region
            .get_output(context);
        let endpoint_group_region_binding = endpoint_group_region_binding_1.get_inner();
        let listener_arn_binding_1 = args.listener_arn.get_output(context);
        let listener_arn_binding = listener_arn_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:globalaccelerator/customRoutingEndpointGroup:CustomRoutingEndpointGroup"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "destinationConfigurations".into(),
                    value: &destination_configurations_binding,
                },
                register_interface::ObjectField {
                    name: "endpointConfigurations".into(),
                    value: &endpoint_configurations_binding,
                },
                register_interface::ObjectField {
                    name: "endpointGroupRegion".into(),
                    value: &endpoint_group_region_binding,
                },
                register_interface::ObjectField {
                    name: "listenerArn".into(),
                    value: &listener_arn_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        CustomRoutingEndpointGroupResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            destination_configurations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("destinationConfigurations"),
            ),
            endpoint_configurations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpointConfigurations"),
            ),
            endpoint_group_region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpointGroupRegion"),
            ),
            listener_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("listenerArn"),
            ),
        }
    }
}
