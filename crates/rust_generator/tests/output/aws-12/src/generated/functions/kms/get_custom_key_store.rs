#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_custom_key_store {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCustomKeyStoreArgs {
        /// The ID for the custom key store.
        #[builder(into, default)]
        pub custom_key_store_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The user-specified friendly name for the custom key store.
        #[builder(into, default)]
        pub custom_key_store_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetCustomKeyStoreResult {
        pub cloud_hsm_cluster_id: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether the custom key store is connected to its CloudHSM cluster.
        pub connection_state: pulumi_gestalt_rust::Output<String>,
        /// The date and time when the custom key store was created.
        pub creation_date: pulumi_gestalt_rust::Output<String>,
        pub custom_key_store_id: pulumi_gestalt_rust::Output<String>,
        pub custom_key_store_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The trust anchor certificate of the associated CloudHSM cluster.
        pub trust_anchor_certificate: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetCustomKeyStoreArgs,
    ) -> GetCustomKeyStoreResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let custom_key_store_id_binding = args.custom_key_store_id.get_output(context);
        let custom_key_store_name_binding = args
            .custom_key_store_name
            .get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:kms/getCustomKeyStore:getCustomKeyStore".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customKeyStoreId".into(),
                    value: custom_key_store_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customKeyStoreName".into(),
                    value: custom_key_store_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetCustomKeyStoreResult {
            cloud_hsm_cluster_id: o.get_field("cloudHsmClusterId"),
            connection_state: o.get_field("connectionState"),
            creation_date: o.get_field("creationDate"),
            custom_key_store_id: o.get_field("customKeyStoreId"),
            custom_key_store_name: o.get_field("customKeyStoreName"),
            id: o.get_field("id"),
            trust_anchor_certificate: o.get_field("trustAnchorCertificate"),
        }
    }
}
