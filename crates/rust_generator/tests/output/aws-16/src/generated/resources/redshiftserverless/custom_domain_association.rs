/// Resource for managing an AWS Redshift Serverless Custom Domain Association.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = certificate::create(
///         "example",
///         CertificateArgs::builder().domain_name("example.com").build_struct(),
///     );
///     let exampleCustomDomainAssociation = custom_domain_association::create(
///         "exampleCustomDomainAssociation",
///         CustomDomainAssociationArgs::builder()
///             .custom_domain_certificate_arn("${example.arn}")
///             .custom_domain_name("example.com")
///             .workgroup_name("${exampleWorkgroup.workgroupName}")
///             .build_struct(),
///     );
///     let exampleNamespace = namespace::create(
///         "exampleNamespace",
///         NamespaceArgs::builder().namespace_name("example-namespace").build_struct(),
///     );
///     let exampleWorkgroup = workgroup::create(
///         "exampleWorkgroup",
///         WorkgroupArgs::builder()
///             .namespace_name("${exampleNamespace.namespaceName}")
///             .workgroup_name("example-workgroup")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Redshift Serverless Custom Domain Association using the `workgroup_name` and `custom_domain_name`, separated by the coma. For example:
///
/// ```sh
/// $ pulumi import aws:redshiftserverless/customDomainAssociation:CustomDomainAssociation example example-workgroup,example.com
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod custom_domain_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomDomainAssociationArgs {
        /// ARN of the certificate for the custom domain association.
        #[builder(into)]
        pub custom_domain_certificate_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Custom domain to associate with the workgroup.
        #[builder(into)]
        pub custom_domain_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the workgroup.
        #[builder(into)]
        pub workgroup_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CustomDomainAssociationResult {
        /// ARN of the certificate for the custom domain association.
        pub custom_domain_certificate_arn: pulumi_gestalt_rust::Output<String>,
        /// Expiration time for the certificate.
        pub custom_domain_certificate_expiry_time: pulumi_gestalt_rust::Output<String>,
        /// Custom domain to associate with the workgroup.
        pub custom_domain_name: pulumi_gestalt_rust::Output<String>,
        /// Name of the workgroup.
        pub workgroup_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: CustomDomainAssociationArgs,
    ) -> CustomDomainAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let custom_domain_certificate_arn_binding_1 = args
            .custom_domain_certificate_arn
            .get_output(context);
        let custom_domain_certificate_arn_binding = custom_domain_certificate_arn_binding_1
            .get_inner();
        let custom_domain_name_binding_1 = args.custom_domain_name.get_output(context);
        let custom_domain_name_binding = custom_domain_name_binding_1.get_inner();
        let workgroup_name_binding_1 = args.workgroup_name.get_output(context);
        let workgroup_name_binding = workgroup_name_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:redshiftserverless/customDomainAssociation:CustomDomainAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "customDomainCertificateArn".into(),
                    value: &custom_domain_certificate_arn_binding,
                },
                register_interface::ObjectField {
                    name: "customDomainName".into(),
                    value: &custom_domain_name_binding,
                },
                register_interface::ObjectField {
                    name: "workgroupName".into(),
                    value: &workgroup_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        CustomDomainAssociationResult {
            custom_domain_certificate_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customDomainCertificateArn"),
            ),
            custom_domain_certificate_expiry_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customDomainCertificateExpiryTime"),
            ),
            custom_domain_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customDomainName"),
            ),
            workgroup_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workgroupName"),
            ),
        }
    }
}
