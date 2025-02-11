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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod custom_key_store {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomKeyStoreArgs {
        /// Cluster ID of CloudHSM.
        #[builder(into)]
        pub cloud_hsm_cluster_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Unique name for Custom Key Store.
        #[builder(into)]
        pub custom_key_store_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Password for `kmsuser` on CloudHSM.
        #[builder(into)]
        pub key_store_password: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Customer certificate used for signing on CloudHSM.
        #[builder(into)]
        pub trust_anchor_certificate: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CustomKeyStoreResult {
        /// Cluster ID of CloudHSM.
        pub cloud_hsm_cluster_id: pulumi_gestalt_rust::Output<String>,
        /// Unique name for Custom Key Store.
        pub custom_key_store_name: pulumi_gestalt_rust::Output<String>,
        /// Password for `kmsuser` on CloudHSM.
        pub key_store_password: pulumi_gestalt_rust::Output<String>,
        /// Customer certificate used for signing on CloudHSM.
        pub trust_anchor_certificate: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CustomKeyStoreArgs,
    ) -> CustomKeyStoreResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cloud_hsm_cluster_id_binding = args.cloud_hsm_cluster_id.get_output(context);
        let custom_key_store_name_binding = args
            .custom_key_store_name
            .get_output(context);
        let key_store_password_binding = args.key_store_password.get_output(context);
        let trust_anchor_certificate_binding = args
            .trust_anchor_certificate
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:kms/customKeyStore:CustomKeyStore".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cloudHsmClusterId".into(),
                    value: &cloud_hsm_cluster_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customKeyStoreName".into(),
                    value: &custom_key_store_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyStorePassword".into(),
                    value: &key_store_password_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trustAnchorCertificate".into(),
                    value: &trust_anchor_certificate_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        CustomKeyStoreResult {
            cloud_hsm_cluster_id: o.get_field("cloudHsmClusterId"),
            custom_key_store_name: o.get_field("customKeyStoreName"),
            key_store_password: o.get_field("keyStorePassword"),
            trust_anchor_certificate: o.get_field("trustAnchorCertificate"),
        }
    }
}
