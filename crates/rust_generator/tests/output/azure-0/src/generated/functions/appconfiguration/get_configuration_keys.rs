#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_configuration_keys {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetConfigurationKeysArgs {
        /// Specifies the id of the App Configuration.
        #[builder(into)]
        pub configuration_store_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the App Configuration Keys to look up.
        #[builder(into, default)]
        pub key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The label of the App Configuration Keys tp look up.
        #[builder(into, default)]
        pub label: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetConfigurationKeysResult {
        pub configuration_store_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A list of `items` blocks as defined below.
        pub items: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::appconfiguration::GetConfigurationKeysItem>,
        >,
        /// The name of the App Configuration Key.
        pub key: pulumi_gestalt_rust::Output<Option<String>>,
        /// The label of the App Configuration Key.
        pub label: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetConfigurationKeysArgs,
    ) -> GetConfigurationKeysResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let configuration_store_id_binding = args
            .configuration_store_id
            .get_output(context);
        let key_binding = args.key.get_output(context);
        let label_binding = args.label.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:appconfiguration/getConfigurationKeys:getConfigurationKeys"
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
        GetConfigurationKeysResult {
            configuration_store_id: o.get_field("configurationStoreId"),
            id: o.get_field("id"),
            items: o.get_field("items"),
            key: o.get_field("key"),
            label: o.get_field("label"),
        }
    }
}
