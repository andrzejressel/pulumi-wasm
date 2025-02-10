/// Provides a WAF Regional IPSet Resource for use with Application Load Balancer.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let ipset = ip_set::create(
///         "ipset",
///         IpSetArgs::builder()
///             .ip_set_descriptors(
///                 vec![
///                     IpSetIpSetDescriptor::builder(). type ("IPV4").value("192.0.7.0/24")
///                     .build_struct(), IpSetIpSetDescriptor::builder(). type ("IPV4")
///                     .value("10.16.16.0/16").build_struct(),
///                 ],
///             )
///             .name("tfIPSet")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import WAF Regional IPSets using their ID. For example:
///
/// ```sh
/// $ pulumi import aws:wafregional/ipSet:IpSet example a1b2c3d4-d5f6-7777-8888-9999aaaabbbbcccc
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod ip_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IpSetArgs {
        /// One or more pairs specifying the IP address type (IPV4 or IPV6) and the IP address range (in CIDR notation) from which web requests originate.
        #[builder(into, default)]
        pub ip_set_descriptors: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::wafregional::IpSetIpSetDescriptor>>,
        >,
        /// The name or description of the IPSet.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct IpSetResult {
        /// The ARN of the WAF IPSet.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// One or more pairs specifying the IP address type (IPV4 or IPV6) and the IP address range (in CIDR notation) from which web requests originate.
        pub ip_set_descriptors: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::wafregional::IpSetIpSetDescriptor>>,
        >,
        /// The name or description of the IPSet.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IpSetArgs,
    ) -> IpSetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let ip_set_descriptors_binding = args.ip_set_descriptors.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:wafregional/ipSet:IpSet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipSetDescriptors".into(),
                    value: ip_set_descriptors_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        IpSetResult {
            arn: o.get_field("arn"),
            ip_set_descriptors: o.get_field("ipSetDescriptors"),
            name: o.get_field("name"),
        }
    }
}
