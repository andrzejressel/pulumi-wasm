/// Provides a conditional forwarder for managed Microsoft AD in AWS Directory Service.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod conditional_forwader {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConditionalForwaderArgs {
        /// ID of directory.
        #[builder(into)]
        pub directory_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of forwarder IP addresses.
        #[builder(into)]
        pub dns_ips: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The fully qualified domain name of the remote domain for which forwarders will be used.
        #[builder(into)]
        pub remote_domain_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ConditionalForwaderResult {
        /// ID of directory.
        pub directory_id: pulumi_gestalt_rust::Output<String>,
        /// A list of forwarder IP addresses.
        pub dns_ips: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The fully qualified domain name of the remote domain for which forwarders will be used.
        pub remote_domain_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ConditionalForwaderArgs,
    ) -> ConditionalForwaderResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let directory_id_binding = args.directory_id.get_output(context);
        let dns_ips_binding = args.dns_ips.get_output(context);
        let remote_domain_name_binding = args.remote_domain_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:directoryservice/conditionalForwader:ConditionalForwader".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "directoryId".into(),
                    value: directory_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dnsIps".into(),
                    value: dns_ips_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "remoteDomainName".into(),
                    value: remote_domain_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ConditionalForwaderResult {
            directory_id: o.get_field("directoryId"),
            dns_ips: o.get_field("dnsIps"),
            remote_domain_name: o.get_field("remoteDomainName"),
        }
    }
}
