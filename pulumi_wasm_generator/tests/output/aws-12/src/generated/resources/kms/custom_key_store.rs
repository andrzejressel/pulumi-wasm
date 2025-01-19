/// Resource for managing an AWS KMS (Key Management) Custom Key Store.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   test:
///     type: aws:kms:CustomKeyStore
///     properties:
///       cloudHsmClusterId: ${cloudHsmClusterId}
///       customKeyStoreName: kms-custom-key-store-test
///       keyStorePassword: noplaintextpasswords1
///       trustAnchorCertificate:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: anchor-certificate.crt
///           return: result
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import KMS (Key Management) Custom Key Store using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:kms/customKeyStore:CustomKeyStore example cks-5ebd4ef395a96288e
/// ```
pub mod custom_key_store {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomKeyStoreArgs {
        /// Cluster ID of CloudHSM.
        #[builder(into)]
        pub cloud_hsm_cluster_id: pulumi_wasm_rust::Output<String>,
        /// Unique name for Custom Key Store.
        #[builder(into)]
        pub custom_key_store_name: pulumi_wasm_rust::Output<String>,
        /// Password for `kmsuser` on CloudHSM.
        #[builder(into)]
        pub key_store_password: pulumi_wasm_rust::Output<String>,
        /// Customer certificate used for signing on CloudHSM.
        #[builder(into)]
        pub trust_anchor_certificate: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct CustomKeyStoreResult {
        /// Cluster ID of CloudHSM.
        pub cloud_hsm_cluster_id: pulumi_wasm_rust::Output<String>,
        /// Unique name for Custom Key Store.
        pub custom_key_store_name: pulumi_wasm_rust::Output<String>,
        /// Password for `kmsuser` on CloudHSM.
        pub key_store_password: pulumi_wasm_rust::Output<String>,
        /// Customer certificate used for signing on CloudHSM.
        pub trust_anchor_certificate: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: CustomKeyStoreArgs) -> CustomKeyStoreResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cloud_hsm_cluster_id_binding = args.cloud_hsm_cluster_id.get_inner();
        let custom_key_store_name_binding = args.custom_key_store_name.get_inner();
        let key_store_password_binding = args.key_store_password.get_inner();
        let trust_anchor_certificate_binding = args.trust_anchor_certificate.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:kms/customKeyStore:CustomKeyStore".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cloudHsmClusterId".into(),
                    value: &cloud_hsm_cluster_id_binding,
                },
                register_interface::ObjectField {
                    name: "customKeyStoreName".into(),
                    value: &custom_key_store_name_binding,
                },
                register_interface::ObjectField {
                    name: "keyStorePassword".into(),
                    value: &key_store_password_binding,
                },
                register_interface::ObjectField {
                    name: "trustAnchorCertificate".into(),
                    value: &trust_anchor_certificate_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "cloudHsmClusterId".into(),
                },
                register_interface::ResultField {
                    name: "customKeyStoreName".into(),
                },
                register_interface::ResultField {
                    name: "keyStorePassword".into(),
                },
                register_interface::ResultField {
                    name: "trustAnchorCertificate".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CustomKeyStoreResult {
            cloud_hsm_cluster_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cloudHsmClusterId").unwrap(),
            ),
            custom_key_store_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customKeyStoreName").unwrap(),
            ),
            key_store_password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyStorePassword").unwrap(),
            ),
            trust_anchor_certificate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trustAnchorCertificate").unwrap(),
            ),
        }
    }
}
