/// Provides a conditional forwarder for managed Microsoft AD in AWS Directory Service.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = conditional_forwader::create(
///         "example",
///         ConditionalForwaderArgs::builder()
///             .directory_id("${ad.id}")
///             .dns_ips(vec!["8.8.8.8", "8.8.4.4",])
///             .remote_domain_name("example.com")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import conditional forwarders using the directory id and remote_domain_name. For example:
///
/// ```sh
/// $ pulumi import aws:directoryservice/conditionalForwader:ConditionalForwader example d-1234567890:example.com
/// ```
pub mod conditional_forwader {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConditionalForwaderArgs {
        /// ID of directory.
        #[builder(into)]
        pub directory_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// A list of forwarder IP addresses.
        #[builder(into)]
        pub dns_ips: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// The fully qualified domain name of the remote domain for which forwarders will be used.
        #[builder(into)]
        pub remote_domain_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ConditionalForwaderResult {
        /// ID of directory.
        pub directory_id: pulumi_wasm_rust::Output<String>,
        /// A list of forwarder IP addresses.
        pub dns_ips: pulumi_wasm_rust::Output<Vec<String>>,
        /// The fully qualified domain name of the remote domain for which forwarders will be used.
        pub remote_domain_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ConditionalForwaderArgs,
    ) -> ConditionalForwaderResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let directory_id_binding = args.directory_id.get_output(context).get_inner();
        let dns_ips_binding = args.dns_ips.get_output(context).get_inner();
        let remote_domain_name_binding = args
            .remote_domain_name
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:directoryservice/conditionalForwader:ConditionalForwader".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "directoryId".into(),
                    value: &directory_id_binding,
                },
                register_interface::ObjectField {
                    name: "dnsIps".into(),
                    value: &dns_ips_binding,
                },
                register_interface::ObjectField {
                    name: "remoteDomainName".into(),
                    value: &remote_domain_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "directoryId".into(),
                },
                register_interface::ResultField {
                    name: "dnsIps".into(),
                },
                register_interface::ResultField {
                    name: "remoteDomainName".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ConditionalForwaderResult {
            directory_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("directoryId").unwrap(),
            ),
            dns_ips: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsIps").unwrap(),
            ),
            remote_domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("remoteDomainName").unwrap(),
            ),
        }
    }
}
