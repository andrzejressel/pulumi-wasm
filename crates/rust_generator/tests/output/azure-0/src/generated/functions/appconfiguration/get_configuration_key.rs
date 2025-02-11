#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_configuration_key {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetConfigurationKeyArgs {
        /// Specifies the id of the App Configuration.
        #[builder(into)]
        pub configuration_store_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the App Configuration Key.
        #[builder(into)]
        pub key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The label of the App Configuration Key.
        #[builder(into, default)]
        pub label: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetConfigurationKeyResult {
        pub configuration_store_id: pulumi_gestalt_rust::Output<String>,
        /// The content type of the App Configuration Key.
        pub content_type: pulumi_gestalt_rust::Output<String>,
        /// The ETag of the key.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub key: pulumi_gestalt_rust::Output<String>,
        pub label: pulumi_gestalt_rust::Output<Option<String>>,
        /// Is this App Configuration Key be Locked to prevent changes.
        pub locked: pulumi_gestalt_rust::Output<bool>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The type of the App Configuration Key. It can either be `kv` (simple [key/value](https://docs.microsoft.com/azure/azure-app-configuration/concept-key-value)) or `vault` (where the value is a reference to a [Key Vault Secret](https://azure.microsoft.com/en-gb/services/key-vault/).
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// The value of the App Configuration Key.
        pub value: pulumi_gestalt_rust::Output<String>,
        /// The ID of the vault secret this App Configuration Key refers to, when `type` is `vault`.
        pub vault_key_reference: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetConfigurationKeyArgs,
    ) -> GetConfigurationKeyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let configuration_store_id_binding = args
            .configuration_store_id
            .get_output(context);
        let key_binding = args.key.get_output(context);
        let label_binding = args.label.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:appconfiguration/getConfigurationKey:getConfigurationKey"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configurationStoreId".into(),
                    value: &configuration_store_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "key".into(),
                    value: &key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "label".into(),
                    value: &label_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetConfigurationKeyResult {
            configuration_store_id: o.get_field("configurationStoreId"),
            content_type: o.get_field("contentType"),
            etag: o.get_field("etag"),
            id: o.get_field("id"),
            key: o.get_field("key"),
            label: o.get_field("label"),
            locked: o.get_field("locked"),
            tags: o.get_field("tags"),
            type_: o.get_field("type"),
            value: o.get_field("value"),
            vault_key_reference: o.get_field("vaultKeyReference"),
        }
    }
}
