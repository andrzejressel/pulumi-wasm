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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: FooArgs,
    ) -> FooResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let argument_binding = args.argument.get_output(context).get_inner();
        let backup_kube_client_settings_binding = args
            .backup_kube_client_settings
            .get_output(context)
            .get_inner();
        let kube_client_settings_binding = args
            .kube_client_settings
            .get_output(context)
            .get_inner();
        let settings_binding = args.settings.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "example:index:Foo".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "argument".into(),
                    value: &argument_binding,
                },
                register_interface::ObjectField {
                    name: "backupKubeClientSettings".into(),
                    value: &backup_kube_client_settings_binding,
                },
                register_interface::ObjectField {
                    name: "kubeClientSettings".into(),
                    value: &kube_client_settings_binding,
                },
                register_interface::ObjectField {
                    name: "settings".into(),
                    value: &settings_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        FooResult {
            default_kube_client_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultKubeClientSettings"),
            ),
        }
    }
}
