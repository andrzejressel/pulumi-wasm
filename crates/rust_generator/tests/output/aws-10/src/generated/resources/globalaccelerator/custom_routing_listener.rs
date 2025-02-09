/// Provides a Global Accelerator custom routing listener.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = custom_routing_accelerator::create(
///         "example",
///         CustomRoutingAcceleratorArgs::builder()
///             .attributes(
///                 CustomRoutingAcceleratorAttributes::builder()
///                     .flowLogsEnabled(true)
///                     .flowLogsS3Bucket("example-bucket")
///                     .flowLogsS3Prefix("flow-logs/")
///                     .build_struct(),
///             )
///             .enabled(true)
///             .ip_address_type("IPV4")
///             .name("Example")
///             .build_struct(),
///     );
///     let exampleCustomRoutingListener = custom_routing_listener::create(
///         "exampleCustomRoutingListener",
///         CustomRoutingListenerArgs::builder()
///             .accelerator_arn("${example.id}")
///             .port_ranges(
///                 vec![
///                     CustomRoutingListenerPortRange::builder().fromPort(80).toPort(80)
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Global Accelerator custom routing listeners using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:globalaccelerator/customRoutingListener:CustomRoutingListener example arn:aws:globalaccelerator::111111111111:accelerator/xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx/listener/xxxxxxxx
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod custom_routing_listener {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomRoutingListenerArgs {
        /// The Amazon Resource Name (ARN) of a custom routing accelerator.
        #[builder(into)]
        pub accelerator_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The list of port ranges for the connections from clients to the accelerator. Fields documented below.
        #[builder(into)]
        pub port_ranges: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::globalaccelerator::CustomRoutingListenerPortRange>,
        >,
    }
    #[allow(dead_code)]
    pub struct CustomRoutingListenerResult {
        /// The Amazon Resource Name (ARN) of a custom routing accelerator.
        pub accelerator_arn: pulumi_gestalt_rust::Output<String>,
        /// The list of port ranges for the connections from clients to the accelerator. Fields documented below.
        pub port_ranges: pulumi_gestalt_rust::Output<
            Vec<super::super::types::globalaccelerator::CustomRoutingListenerPortRange>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CustomRoutingListenerArgs,
    ) -> CustomRoutingListenerResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let accelerator_arn_binding = args.accelerator_arn.get_output(context);
        let port_ranges_binding = args.port_ranges.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:globalaccelerator/customRoutingListener:CustomRoutingListener"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "acceleratorArn".into(),
                    value: accelerator_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "portRanges".into(),
                    value: port_ranges_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        CustomRoutingListenerResult {
            accelerator_arn: o.get_field("acceleratorArn"),
            port_ranges: o.get_field("portRanges"),
        }
    }
}
