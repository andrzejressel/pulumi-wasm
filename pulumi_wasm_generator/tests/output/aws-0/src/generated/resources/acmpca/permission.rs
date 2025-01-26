/// Provides a resource to manage an AWS Certificate Manager Private Certificate Authorities Permission.
/// Currently, this is only required in order to allow the ACM service to automatically renew certificates issued by a PCA.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod permission {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PermissionArgs {
        /// Actions that the specified AWS service principal can use. These include `IssueCertificate`, `GetCertificate`, and `ListPermissions`. Note that in order for ACM to automatically rotate certificates issued by a PCA, it must be granted permission on all 3 actions, as per the example above.
        #[builder(into)]
        pub actions: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// ARN of the CA that grants the permissions.
        #[builder(into)]
        pub certificate_authority_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// AWS service or identity that receives the permission. At this time, the only valid principal is `acm.amazonaws.com`.
        #[builder(into)]
        pub principal: pulumi_wasm_rust::InputOrOutput<String>,
        /// ID of the calling account
        #[builder(into, default)]
        pub source_account: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct PermissionResult {
        /// Actions that the specified AWS service principal can use. These include `IssueCertificate`, `GetCertificate`, and `ListPermissions`. Note that in order for ACM to automatically rotate certificates issued by a PCA, it must be granted permission on all 3 actions, as per the example above.
        pub actions: pulumi_wasm_rust::Output<Vec<String>>,
        /// ARN of the CA that grants the permissions.
        pub certificate_authority_arn: pulumi_wasm_rust::Output<String>,
        /// IAM policy that is associated with the permission.
        pub policy: pulumi_wasm_rust::Output<String>,
        /// AWS service or identity that receives the permission. At this time, the only valid principal is `acm.amazonaws.com`.
        pub principal: pulumi_wasm_rust::Output<String>,
        /// ID of the calling account
        pub source_account: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: PermissionArgs,
    ) -> PermissionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let actions_binding = args.actions.get_output(context).get_inner();
        let certificate_authority_arn_binding = args
            .certificate_authority_arn
            .get_output(context)
            .get_inner();
        let principal_binding = args.principal.get_output(context).get_inner();
        let source_account_binding = args.source_account.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:acmpca/permission:Permission".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "actions".into(),
                    value: &actions_binding,
                },
                register_interface::ObjectField {
                    name: "certificateAuthorityArn".into(),
                    value: &certificate_authority_arn_binding,
                },
                register_interface::ObjectField {
                    name: "principal".into(),
                    value: &principal_binding,
                },
                register_interface::ObjectField {
                    name: "sourceAccount".into(),
                    value: &source_account_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        PermissionResult {
            actions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("actions"),
            ),
            certificate_authority_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("certificateAuthorityArn"),
            ),
            policy: pulumi_wasm_rust::__private::into_domain(o.extract_field("policy")),
            principal: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("principal"),
            ),
            source_account: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sourceAccount"),
            ),
        }
    }
}
