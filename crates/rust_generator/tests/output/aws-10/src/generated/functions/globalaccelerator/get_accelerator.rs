#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_accelerator {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAcceleratorArgs {
        /// Full ARN of the Global Accelerator.
        #[builder(into, default)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Unique name of the Global Accelerator.
        ///
        /// > **NOTE:** When both `arn` and `name` are specified, `arn` takes precedence.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetAcceleratorResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub attributes: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::globalaccelerator::GetAcceleratorAttribute>,
        >,
        pub dns_name: pulumi_gestalt_rust::Output<String>,
        pub dual_stack_dns_name: pulumi_gestalt_rust::Output<String>,
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        pub hosted_zone_id: pulumi_gestalt_rust::Output<String>,
        pub id: pulumi_gestalt_rust::Output<String>,
        pub ip_address_type: pulumi_gestalt_rust::Output<String>,
        pub ip_sets: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::globalaccelerator::GetAcceleratorIpSet>,
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
        args: GetAcceleratorArgs,
    ) -> GetAcceleratorResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let arn_binding = args.arn.get_output(context);
        let id_binding = args.id.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:globalaccelerator/getAccelerator:getAccelerator".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arn".into(),
                    value: arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetAcceleratorResult {
            arn: o.get_field("arn"),
            attributes: o.get_field("attributes"),
            dns_name: o.get_field("dnsName"),
            dual_stack_dns_name: o.get_field("dualStackDnsName"),
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
