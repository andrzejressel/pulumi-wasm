/// Assigns a static reverse DNS record to an Elastic IP addresses. See [Using reverse DNS for email applications](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/elastic-ip-addresses-eip.html#Using_Elastic_Addressing_Reverse_DNS).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod eip_domain_name {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EipDomainNameArgs {
        /// The allocation ID.
        #[builder(into)]
        pub allocation_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The domain name to modify for the IP address.
        #[builder(into)]
        pub domain_name: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::ec2::EipDomainNameTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct EipDomainNameResult {
        /// The allocation ID.
        pub allocation_id: pulumi_wasm_rust::Output<String>,
        /// The domain name to modify for the IP address.
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// The DNS pointer (PTR) record for the IP address.
        pub ptr_record: pulumi_wasm_rust::Output<String>,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::ec2::EipDomainNameTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: EipDomainNameArgs,
    ) -> EipDomainNameResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let allocation_id_binding = args.allocation_id.get_output(context).get_inner();
        let domain_name_binding = args.domain_name.get_output(context).get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/eipDomainName:EipDomainName".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allocationId".into(),
                    value: &allocation_id_binding,
                },
                register_interface::ObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "allocationId".into(),
                },
                register_interface::ResultField {
                    name: "domainName".into(),
                },
                register_interface::ResultField {
                    name: "ptrRecord".into(),
                },
                register_interface::ResultField {
                    name: "timeouts".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EipDomainNameResult {
            allocation_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allocationId").unwrap(),
            ),
            domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainName").unwrap(),
            ),
            ptr_record: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ptrRecord").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
        }
    }
}
