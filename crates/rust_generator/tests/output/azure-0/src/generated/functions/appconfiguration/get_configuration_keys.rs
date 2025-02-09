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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetConfigurationKeysArgs,
    ) -> GetConfigurationKeysResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let configuration_store_id_binding_1 = args
            .configuration_store_id
            .get_output(context);
        let configuration_store_id_binding = configuration_store_id_binding_1
            .get_inner();
        let key_binding_1 = args.key.get_output(context);
        let key_binding = key_binding_1.get_inner();
        let label_binding_1 = args.label.get_output(context);
        let label_binding = label_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:appconfiguration/getConfigurationKeys:getConfigurationKeys"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "configurationStoreId".into(),
                    value: &configuration_store_id_binding,
                },
                register_interface::ObjectField {
                    name: "key".into(),
                    value: &key_binding,
                },
                register_interface::ObjectField {
                    name: "label".into(),
                    value: &label_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetConfigurationKeysResult {
            configuration_store_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("configurationStoreId"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            items: pulumi_gestalt_rust::__private::into_domain(o.extract_field("items")),
            key: pulumi_gestalt_rust::__private::into_domain(o.extract_field("key")),
            label: pulumi_gestalt_rust::__private::into_domain(o.extract_field("label")),
        }
    }
}
