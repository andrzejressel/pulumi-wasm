/// Provides a resource to manage an AWS Certificate Manager Private Certificate Authorities Permission.
/// Currently, this is only required in order to allow the ACM service to automatically renew certificates issued by a PCA.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = permission::create(
///         "example",
///         PermissionArgs::builder()
///             .actions(vec!["IssueCertificate", "GetCertificate", "ListPermissions",])
///             .certificate_authority_arn("${exampleCertificateAuthority.arn}")
///             .principal("acm.amazonaws.com")
///             .build_struct(),
///     );
///     let exampleCertificateAuthority = certificate_authority::create(
///         "exampleCertificateAuthority",
///         CertificateAuthorityArgs::builder()
///             .certificate_authority_configuration(
///                 CertificateAuthorityCertificateAuthorityConfiguration::builder()
///                     .keyAlgorithm("RSA_4096")
///                     .signingAlgorithm("SHA512WITHRSA")
///                     .subject(
///                         CertificateAuthorityCertificateAuthorityConfigurationSubject::builder()
///                             .commonName("example.com")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod permission {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PermissionArgs {
        /// Actions that the specified AWS service principal can use. These include `IssueCertificate`, `GetCertificate`, and `ListPermissions`. Note that in order for ACM to automatically rotate certificates issued by a PCA, it must be granted permission on all 3 actions, as per the example above.
        #[builder(into)]
        pub actions: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// ARN of the CA that grants the permissions.
        #[builder(into)]
        pub certificate_authority_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// AWS service or identity that receives the permission. At this time, the only valid principal is `acm.amazonaws.com`.
        #[builder(into)]
        pub principal: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ID of the calling account
        #[builder(into, default)]
        pub source_account: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct PermissionResult {
        /// Actions that the specified AWS service principal can use. These include `IssueCertificate`, `GetCertificate`, and `ListPermissions`. Note that in order for ACM to automatically rotate certificates issued by a PCA, it must be granted permission on all 3 actions, as per the example above.
        pub actions: pulumi_gestalt_rust::Output<Vec<String>>,
        /// ARN of the CA that grants the permissions.
        pub certificate_authority_arn: pulumi_gestalt_rust::Output<String>,
        /// IAM policy that is associated with the permission.
        pub policy: pulumi_gestalt_rust::Output<String>,
        /// AWS service or identity that receives the permission. At this time, the only valid principal is `acm.amazonaws.com`.
        pub principal: pulumi_gestalt_rust::Output<String>,
        /// ID of the calling account
        pub source_account: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PermissionArgs,
    ) -> PermissionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let actions_binding = args.actions.get_output(context);
        let certificate_authority_arn_binding = args
            .certificate_authority_arn
            .get_output(context);
        let principal_binding = args.principal.get_output(context);
        let source_account_binding = args.source_account.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:acmpca/permission:Permission".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "actions".into(),
                    value: actions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateAuthorityArn".into(),
                    value: certificate_authority_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "principal".into(),
                    value: principal_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceAccount".into(),
                    value: source_account_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        PermissionResult {
            actions: o.get_field("actions"),
            certificate_authority_arn: o.get_field("certificateAuthorityArn"),
            policy: o.get_field("policy"),
            principal: o.get_field("principal"),
            source_account: o.get_field("sourceAccount"),
        }
    }
}
