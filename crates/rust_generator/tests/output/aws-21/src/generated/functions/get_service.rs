#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServiceArgs {
        /// DNS name of the service (_e.g.,_ `rds.us-east-1.amazonaws.com`). One of `dns_name`, `reverse_dns_name`, or `service_id` is required.
        #[builder(into, default)]
        pub dns_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Region of the service (_e.g.,_ `us-west-2`, `ap-northeast-1`).
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Reverse DNS name of the service (_e.g.,_ `com.amazonaws.us-west-2.s3`). One of `dns_name`, `reverse_dns_name`, or `service_id` is required.
        #[builder(into, default)]
        pub reverse_dns_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Prefix of the service (_e.g.,_ `com.amazonaws` in AWS Commercial, `cn.com.amazonaws` in AWS China).
        #[builder(into, default)]
        pub reverse_dns_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Service endpoint ID (_e.g.,_ `s3`, `rds`, `ec2`). One of `dns_name`, `reverse_dns_name`, or `service_id` is required. A service's endpoint ID can be found in the [_AWS General Reference_](https://docs.aws.amazon.com/general/latest/gr/aws-service-information.html).
        #[builder(into, default)]
        pub service_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetServiceResult {
        pub dns_name: pulumi_gestalt_rust::Output<String>,
        pub id: pulumi_gestalt_rust::Output<String>,
        pub partition: pulumi_gestalt_rust::Output<String>,
        pub region: pulumi_gestalt_rust::Output<String>,
        pub reverse_dns_name: pulumi_gestalt_rust::Output<String>,
        pub reverse_dns_prefix: pulumi_gestalt_rust::Output<String>,
        pub service_id: pulumi_gestalt_rust::Output<String>,
        /// Whether the service is supported in the region's partition. New services may not be listed immediately as supported.
        pub supported: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetServiceArgs,
    ) -> GetServiceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let dns_name_binding = args.dns_name.get_output(context);
        let id_binding = args.id.get_output(context);
        let region_binding = args.region.get_output(context);
        let reverse_dns_name_binding = args.reverse_dns_name.get_output(context);
        let reverse_dns_prefix_binding = args.reverse_dns_prefix.get_output(context);
        let service_id_binding = args.service_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:index/getService:getService".into(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dnsName".into(),
                    value: dns_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "reverseDnsName".into(),
                    value: reverse_dns_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "reverseDnsPrefix".into(),
                    value: reverse_dns_prefix_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceId".into(),
                    value: service_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetServiceResult {
            dns_name: o.get_field("dnsName"),
            id: o.get_field("id"),
            partition: o.get_field("partition"),
            region: o.get_field("region"),
            reverse_dns_name: o.get_field("reverseDnsName"),
            reverse_dns_prefix: o.get_field("reverseDnsPrefix"),
            service_id: o.get_field("serviceId"),
            supported: o.get_field("supported"),
        }
    }
}
