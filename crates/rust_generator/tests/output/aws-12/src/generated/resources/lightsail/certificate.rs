/// Provides a lightsail certificate.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = certificate::create(
///         "test",
///         CertificateArgs::builder()
///             .domain_name("testdomain.com")
///             .name("test")
///             .subject_alternative_names(vec!["www.testdomain.com",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_lightsail_certificate` using the certificate name. For example:
///
/// ```sh
/// $ pulumi import aws:lightsail/certificate:Certificate test CertificateName
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CertificateArgs {
        /// A domain name for which the certificate should be issued.
        #[builder(into, default)]
        pub domain_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Lightsail load balancer.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Set of domains that should be SANs in the issued certificate. `domain_name` attribute is automatically added as a Subject Alternative Name.
        #[builder(into, default)]
        pub subject_alternative_names: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// A map of tags to assign to the resource. To create a key-only tag, use an empty string as the value. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CertificateResult {
        /// The ARN of the lightsail certificate.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The timestamp when the instance was created.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// A domain name for which the certificate should be issued.
        pub domain_name: pulumi_gestalt_rust::Output<String>,
        /// Set of domain validation objects which can be used to complete certificate validation. Can have more than one element, e.g., if SANs are defined.
        pub domain_validation_options: pulumi_gestalt_rust::Output<
            Vec<super::super::types::lightsail::CertificateDomainValidationOption>,
        >,
        /// The name of the Lightsail load balancer.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Set of domains that should be SANs in the issued certificate. `domain_name` attribute is automatically added as a Subject Alternative Name.
        pub subject_alternative_names: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A map of tags to assign to the resource. To create a key-only tag, use an empty string as the value. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
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
        args: CertificateArgs,
    ) -> CertificateResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let domain_name_binding = args.domain_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let subject_alternative_names_binding = args
            .subject_alternative_names
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:lightsail/certificate:Certificate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainName".into(),
                    value: domain_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subjectAlternativeNames".into(),
                    value: subject_alternative_names_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        CertificateResult {
            arn: o.get_field("arn"),
            created_at: o.get_field("createdAt"),
            domain_name: o.get_field("domainName"),
            domain_validation_options: o.get_field("domainValidationOptions"),
            name: o.get_field("name"),
            subject_alternative_names: o.get_field("subjectAlternativeNames"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
