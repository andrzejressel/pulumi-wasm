/// Assigns a static reverse DNS record to an Elastic IP addresses. See [Using reverse DNS for email applications](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/elastic-ip-addresses-eip.html#Using_Elastic_Addressing_Reverse_DNS).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = eip::create(
///         "example",
///         EipArgs::builder().domain("vpc").build_struct(),
///     );
///     let exampleEipDomainName = eip_domain_name::create(
///         "exampleEipDomainName",
///         EipDomainNameArgs::builder()
///             .allocation_id("${example.allocationId}")
///             .domain_name("${exampleRecord.fqdn}")
///             .build_struct(),
///     );
///     let exampleRecord = record::create(
///         "exampleRecord",
///         RecordArgs::builder()
///             .name("reverse")
///             .records(vec!["${example.publicIp}",])
///             .type_("A")
///             .zone_id("${main.zoneId}")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod eip_domain_name {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EipDomainNameArgs {
        /// The allocation ID.
        #[builder(into)]
        pub allocation_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The domain name to modify for the IP address.
        #[builder(into)]
        pub domain_name: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ec2::EipDomainNameTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct EipDomainNameResult {
        /// The allocation ID.
        pub allocation_id: pulumi_gestalt_rust::Output<String>,
        /// The domain name to modify for the IP address.
        pub domain_name: pulumi_gestalt_rust::Output<String>,
        /// The DNS pointer (PTR) record for the IP address.
        pub ptr_record: pulumi_gestalt_rust::Output<String>,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::ec2::EipDomainNameTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EipDomainNameArgs,
    ) -> EipDomainNameResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let allocation_id_binding = args.allocation_id.get_output(context);
        let domain_name_binding = args.domain_name.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/eipDomainName:EipDomainName".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allocationId".into(),
                    value: allocation_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainName".into(),
                    value: domain_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: timeouts_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        EipDomainNameResult {
            allocation_id: o.get_field("allocationId"),
            domain_name: o.get_field("domainName"),
            ptr_record: o.get_field("ptrRecord"),
            timeouts: o.get_field("timeouts"),
        }
    }
}
