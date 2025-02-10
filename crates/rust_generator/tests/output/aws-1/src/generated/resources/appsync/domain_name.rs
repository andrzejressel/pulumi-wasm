/// Provides an AppSync Domain Name.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = domain_name::create(
///         "example",
///         DomainNameArgs::builder()
///             .certificate_arn("${exampleAwsAcmCertificate.arn}")
///             .domain_name("api.example.com")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_appsync_domain_name` using the AppSync domain name. For example:
///
/// ```sh
/// $ pulumi import aws:appsync/domainName:DomainName example example.com
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod domain_name {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainNameArgs {
        /// ARN of the certificate. This can be an Certificate Manager (ACM) certificate or an Identity and Access Management (IAM) server certificate. The certifiacte must reside in us-east-1.
        #[builder(into)]
        pub certificate_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A description of the Domain Name.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Domain name.
        #[builder(into)]
        pub domain_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DomainNameResult {
        /// Domain name that AppSync provides.
        pub appsync_domain_name: pulumi_gestalt_rust::Output<String>,
        /// ARN of the certificate. This can be an Certificate Manager (ACM) certificate or an Identity and Access Management (IAM) server certificate. The certifiacte must reside in us-east-1.
        pub certificate_arn: pulumi_gestalt_rust::Output<String>,
        /// A description of the Domain Name.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Domain name.
        pub domain_name: pulumi_gestalt_rust::Output<String>,
        /// ID of your Amazon Route 53 hosted zone.
        pub hosted_zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DomainNameArgs,
    ) -> DomainNameResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let certificate_arn_binding = args.certificate_arn.get_output(context);
        let description_binding = args.description.get_output(context);
        let domain_name_binding = args.domain_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:appsync/domainName:DomainName".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateArn".into(),
                    value: certificate_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainName".into(),
                    value: domain_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DomainNameResult {
            appsync_domain_name: o.get_field("appsyncDomainName"),
            certificate_arn: o.get_field("certificateArn"),
            description: o.get_field("description"),
            domain_name: o.get_field("domainName"),
            hosted_zone_id: o.get_field("hostedZoneId"),
        }
    }
}
