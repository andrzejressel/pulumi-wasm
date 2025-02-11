/// Manages a certificate in an Azure Batch account.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: testbatch
///       location: West Europe
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: teststorage
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleAccount2:
///     type: azure:batch:Account
///     name: example
///     properties:
///       name: testbatchaccount
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       poolAllocationMode: BatchService
///       storageAccountId: ${exampleAccount.id}
///       storageAccountAuthenticationMode: StorageKeys
///       tags:
///         env: test
///   exampleCertificate:
///     type: azure:batch:Certificate
///     name: example
///     properties:
///       resourceGroupName: ${example.name}
///       accountName: ${exampleAccount2.name}
///       certificate:
///         fn::invoke:
///           function: std:filebase64
///           arguments:
///             input: certificate.pfx
///           return: result
///       format: Pfx
///       password: password
///       thumbprint: 42C107874FD0E4A9583292A2F1098E8FE4B2EDDA
///       thumbprintAlgorithm: SHA1
/// ```
///
/// ## Import
///
/// Batch Certificates can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:batch/certificate:Certificate example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example-rg/providers/Microsoft.Batch/batchAccounts/batch1/certificates/certificate1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CertificateArgs {
        /// Specifies the name of the Batch account. Changing this forces a new resource to be created.
        #[builder(into)]
        pub account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The base64-encoded contents of the certificate.
        #[builder(into)]
        pub certificate: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The format of the certificate. Possible values are `Cer` or `Pfx`.
        #[builder(into)]
        pub format: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The password to access the certificate's private key. This can only be specified when `format` is `Pfx`.
        #[builder(into, default)]
        pub password: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the Batch account. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The thumbprint of the certificate. Changing this forces a new resource to be created.
        #[builder(into)]
        pub thumbprint: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The algorithm of the certificate thumbprint. At this time the only supported value is `SHA1`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub thumbprint_algorithm: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CertificateResult {
        /// Specifies the name of the Batch account. Changing this forces a new resource to be created.
        pub account_name: pulumi_gestalt_rust::Output<String>,
        /// The base64-encoded contents of the certificate.
        pub certificate: pulumi_gestalt_rust::Output<String>,
        /// The format of the certificate. Possible values are `Cer` or `Pfx`.
        pub format: pulumi_gestalt_rust::Output<String>,
        /// The generated name of the certificate.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The password to access the certificate's private key. This can only be specified when `format` is `Pfx`.
        pub password: pulumi_gestalt_rust::Output<Option<String>>,
        /// The public key of the certificate.
        pub public_data: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which to create the Batch account. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The thumbprint of the certificate. Changing this forces a new resource to be created.
        pub thumbprint: pulumi_gestalt_rust::Output<String>,
        /// The algorithm of the certificate thumbprint. At this time the only supported value is `SHA1`. Changing this forces a new resource to be created.
        pub thumbprint_algorithm: pulumi_gestalt_rust::Output<String>,
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
        let account_name_binding = args.account_name.get_output(context);
        let certificate_binding = args.certificate.get_output(context);
        let format_binding = args.format.get_output(context);
        let password_binding = args.password.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let thumbprint_binding = args.thumbprint.get_output(context);
        let thumbprint_algorithm_binding = args.thumbprint_algorithm.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:batch/certificate:Certificate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountName".into(),
                    value: &account_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificate".into(),
                    value: &certificate_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "format".into(),
                    value: &format_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "password".into(),
                    value: &password_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "thumbprint".into(),
                    value: &thumbprint_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "thumbprintAlgorithm".into(),
                    value: &thumbprint_algorithm_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        CertificateResult {
            account_name: o.get_field("accountName"),
            certificate: o.get_field("certificate"),
            format: o.get_field("format"),
            name: o.get_field("name"),
            password: o.get_field("password"),
            public_data: o.get_field("publicData"),
            resource_group_name: o.get_field("resourceGroupName"),
            thumbprint: o.get_field("thumbprint"),
            thumbprint_algorithm: o.get_field("thumbprintAlgorithm"),
        }
    }
}
