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
pub mod certificate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CertificateArgs {
        /// Specifies the name of the Batch account. Changing this forces a new resource to be created.
        #[builder(into)]
        pub account_name: pulumi_wasm_rust::Output<String>,
        /// The base64-encoded contents of the certificate.
        #[builder(into)]
        pub certificate: pulumi_wasm_rust::Output<String>,
        /// The format of the certificate. Possible values are `Cer` or `Pfx`.
        #[builder(into)]
        pub format: pulumi_wasm_rust::Output<String>,
        /// The password to access the certificate's private key. This can only be specified when `format` is `Pfx`.
        #[builder(into, default)]
        pub password: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the Batch account. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The thumbprint of the certificate. Changing this forces a new resource to be created.
        #[builder(into)]
        pub thumbprint: pulumi_wasm_rust::Output<String>,
        /// The algorithm of the certificate thumbprint. At this time the only supported value is `SHA1`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub thumbprint_algorithm: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct CertificateResult {
        /// Specifies the name of the Batch account. Changing this forces a new resource to be created.
        pub account_name: pulumi_wasm_rust::Output<String>,
        /// The base64-encoded contents of the certificate.
        pub certificate: pulumi_wasm_rust::Output<String>,
        /// The format of the certificate. Possible values are `Cer` or `Pfx`.
        pub format: pulumi_wasm_rust::Output<String>,
        /// The generated name of the certificate.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The password to access the certificate's private key. This can only be specified when `format` is `Pfx`.
        pub password: pulumi_wasm_rust::Output<Option<String>>,
        /// The public key of the certificate.
        pub public_data: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which to create the Batch account. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The thumbprint of the certificate. Changing this forces a new resource to be created.
        pub thumbprint: pulumi_wasm_rust::Output<String>,
        /// The algorithm of the certificate thumbprint. At this time the only supported value is `SHA1`. Changing this forces a new resource to be created.
        pub thumbprint_algorithm: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: CertificateArgs) -> CertificateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_name_binding = args.account_name.get_inner();
        let certificate_binding = args.certificate.get_inner();
        let format_binding = args.format.get_inner();
        let password_binding = args.password.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let thumbprint_binding = args.thumbprint.get_inner();
        let thumbprint_algorithm_binding = args.thumbprint_algorithm.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:batch/certificate:Certificate".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountName".into(),
                },
                register_interface::ResultField {
                    name: "certificate".into(),
                },
                register_interface::ResultField {
                    name: "format".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "password".into(),
                },
                register_interface::ResultField {
                    name: "publicData".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "thumbprint".into(),
                },
                register_interface::ResultField {
                    name: "thumbprintAlgorithm".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CertificateResult {
            account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountName").unwrap(),
            ),
            certificate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificate").unwrap(),
            ),
            format: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("format").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("password").unwrap(),
            ),
            public_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicData").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            thumbprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("thumbprint").unwrap(),
            ),
            thumbprint_algorithm: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("thumbprintAlgorithm").unwrap(),
            ),
        }
    }
}