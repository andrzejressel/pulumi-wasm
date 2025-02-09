/// test new feature with resoruces
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod foo {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FooArgs {
        #[builder(into, default)]
        pub argument: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Options for tuning the Kubernetes client used by a Provider.
        #[builder(into)]
        pub backup_kube_client_settings: pulumi_gestalt_rust::InputOrOutput<
            super::types::KubeClientSettings,
        >,
        /// Options for tuning the Kubernetes client used by a Provider.
        #[builder(into, default)]
        pub kube_client_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::KubeClientSettings>,
        >,
        /// describing things
        #[builder(into, default)]
        pub settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::LayeredType>,
        >,
    }
    #[allow(dead_code)]
    pub struct FooResult {
        /// A test for plain types
        pub default_kube_client_settings: pulumi_gestalt_rust::Output<
            Option<super::types::KubeClientSettings>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FooArgs,
    ) -> FooResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let argument_binding = args.argument.get_output(context);
        let backup_kube_client_settings_binding = args
            .backup_kube_client_settings
            .get_output(context);
        let kube_client_settings_binding = args.kube_client_settings.get_output(context);
        let settings_binding = args.settings.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "example:index:Foo".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "argument".into(),
                    value: argument_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backupKubeClientSettings".into(),
                    value: backup_kube_client_settings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kubeClientSettings".into(),
                    value: kube_client_settings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "settings".into(),
                    value: settings_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        FooResult {
            default_kube_client_settings: o.get_field("defaultKubeClientSettings"),
        }
    }
}
