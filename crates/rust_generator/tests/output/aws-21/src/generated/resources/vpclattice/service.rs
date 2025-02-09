/// Resource for managing an AWS VPC Lattice Service.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = service::create(
///         "example",
///         ServiceArgs::builder()
///             .auth_type("AWS_IAM")
///             .custom_domain_name("example.com")
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import VPC Lattice Service using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:vpclattice/service:Service example svc-06728e2357ea55f8a
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceArgs {
        /// Type of IAM policy. Either `NONE` or `AWS_IAM`.
        #[builder(into, default)]
        pub auth_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Amazon Resource Name (ARN) of the certificate.
        #[builder(into, default)]
        pub certificate_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Custom domain name of the service.
        #[builder(into, default)]
        pub custom_domain_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the service. The name must be unique within the account. The valid characters are a-z, 0-9, and hyphens (-). You can't use a hyphen as the first or last character, or immediately after another hyphen.Must be between 3 and 40 characters in length.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ServiceResult {
        /// ARN of the service.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Type of IAM policy. Either `NONE` or `AWS_IAM`.
        pub auth_type: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the certificate.
        pub certificate_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Custom domain name of the service.
        pub custom_domain_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// DNS name of the service.
        pub dns_entries: pulumi_gestalt_rust::Output<
            Vec<super::super::types::vpclattice::ServiceDnsEntry>,
        >,
        /// Name of the service. The name must be unique within the account. The valid characters are a-z, 0-9, and hyphens (-). You can't use a hyphen as the first or last character, or immediately after another hyphen.Must be between 3 and 40 characters in length.
        ///
        /// The following arguments are optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Status of the service.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServiceArgs,
    ) -> ServiceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let auth_type_binding = args.auth_type.get_output(context);
        let certificate_arn_binding = args.certificate_arn.get_output(context);
        let custom_domain_name_binding = args.custom_domain_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:vpclattice/service:Service".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authType".into(),
                    value: auth_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateArn".into(),
                    value: certificate_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customDomainName".into(),
                    value: custom_domain_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ServiceResult {
            arn: o.get_field("arn"),
            auth_type: o.get_field("authType"),
            certificate_arn: o.get_field("certificateArn"),
            custom_domain_name: o.get_field("customDomainName"),
            dns_entries: o.get_field("dnsEntries"),
            name: o.get_field("name"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
