#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_custom_routing_accelerator {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCustomRoutingAcceleratorArgs {
        /// Full ARN of the custom routing accelerator.
        #[builder(into, default)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Unique name of the custom routing accelerator.
        ///
        /// > **NOTE:** When both `arn` and `name` are specified, `arn` takes precedence.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetCustomRoutingAcceleratorResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub attributes: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::globalaccelerator::GetCustomRoutingAcceleratorAttribute,
            >,
        >,
        pub dns_name: pulumi_gestalt_rust::Output<String>,
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        pub hosted_zone_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub ip_address_type: pulumi_gestalt_rust::Output<String>,
        pub ip_sets: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::globalaccelerator::GetCustomRoutingAcceleratorIpSet,
            >,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetCustomRoutingAcceleratorArgs,
    ) -> GetCustomRoutingAcceleratorResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let arn_binding = args.arn.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:globalaccelerator/getCustomRoutingAccelerator:getCustomRoutingAccelerator"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arn".into(),
                    value: &arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetCustomRoutingAcceleratorResult {
            arn: o.get_field("arn"),
            attributes: o.get_field("attributes"),
            dns_name: o.get_field("dnsName"),
            enabled: o.get_field("enabled"),
            hosted_zone_id: o.get_field("hostedZoneId"),
            id: o.get_field("id"),
            ip_address_type: o.get_field("ipAddressType"),
            ip_sets: o.get_field("ipSets"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
        }
    }
}
