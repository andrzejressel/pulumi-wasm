#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_ip_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetIpSetArgs {
        /// Name of the WAFv2 IP Set.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies whether this is for an AWS CloudFront distribution or for a regional application. Valid values are `CLOUDFRONT` or `REGIONAL`. To work with CloudFront, you must also specify the region `us-east-1` (N. Virginia) on the AWS provider.
        #[builder(into)]
        pub scope: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetIpSetResult {
        /// An array of strings that specifies zero or more IP addresses or blocks of IP addresses in Classless Inter-Domain Routing (CIDR) notation.
        pub addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// ARN of the entity.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Description of the set that helps with identification.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// IP address version of the set.
        pub ip_address_version: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub scope: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetIpSetArgs,
    ) -> GetIpSetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let scope_binding = args.scope.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:wafv2/getIpSet:getIpSet".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scope".into(),
                    value: scope_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetIpSetResult {
            addresses: o.get_field("addresses"),
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            ip_address_version: o.get_field("ipAddressVersion"),
            name: o.get_field("name"),
            scope: o.get_field("scope"),
        }
    }
}
