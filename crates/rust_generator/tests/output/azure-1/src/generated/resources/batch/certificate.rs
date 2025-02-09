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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: CertificateArgs,
    ) -> CertificateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_name_binding_1 = args.account_name.get_output(context);
        let account_name_binding = account_name_binding_1.get_inner();
        let certificate_binding_1 = args.certificate.get_output(context);
        let certificate_binding = certificate_binding_1.get_inner();
        let format_binding_1 = args.format.get_output(context);
        let format_binding = format_binding_1.get_inner();
        let password_binding_1 = args.password.get_output(context);
        let password_binding = password_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let thumbprint_binding_1 = args.thumbprint.get_output(context);
        let thumbprint_binding = thumbprint_binding_1.get_inner();
        let thumbprint_algorithm_binding_1 = args
            .thumbprint_algorithm
            .get_output(context);
        let thumbprint_algorithm_binding = thumbprint_algorithm_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:batch/certificate:Certificate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountName".into(),
                    value: &account_name_binding,
                },
                register_interface::ObjectField {
                    name: "certificate".into(),
                    value: &certificate_binding,
                },
                register_interface::ObjectField {
                    name: "format".into(),
                    value: &format_binding,
                },
                register_interface::ObjectField {
                    name: "password".into(),
                    value: &password_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "thumbprint".into(),
                    value: &thumbprint_binding,
                },
                register_interface::ObjectField {
                    name: "thumbprintAlgorithm".into(),
                    value: &thumbprint_algorithm_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        CertificateResult {
            account_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountName"),
            ),
            certificate: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificate"),
            ),
            format: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("format"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            password: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("password"),
            ),
            public_data: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publicData"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            thumbprint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("thumbprint"),
            ),
            thumbprint_algorithm: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("thumbprintAlgorithm"),
            ),
        }
    }
}
