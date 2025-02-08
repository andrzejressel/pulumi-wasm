#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_integration_runtime_object_metadatum {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetIntegrationRuntimeObjectMetadatumArgs {
        /// The factory name.
        #[builder(into)]
        pub factory_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The integration runtime name.
        #[builder(into)]
        pub integration_runtime_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Metadata path.
        #[builder(into, default)]
        pub metadata_path: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The resource group name.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetIntegrationRuntimeObjectMetadatumResult {
        /// The link to the next page of results, if any remaining results exist.
        pub next_link: pulumi_gestalt_rust::Output<Option<String>>,
        /// List of SSIS object metadata.
        pub value: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    pulumi_gestalt_rust::OneOf4<
                        super::super::types::SsisEnvironmentResponse,
                        super::super::types::SsisFolderResponse,
                        super::super::types::SsisPackageResponse,
                        super::super::types::SsisProjectResponse,
                    >,
                >,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetIntegrationRuntimeObjectMetadatumArgs,
    ) -> GetIntegrationRuntimeObjectMetadatumResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let factory_name_binding = args.factory_name.get_output(context).get_inner();
        let integration_runtime_name_binding = args
            .integration_runtime_name
            .get_output(context)
            .get_inner();
        let metadata_path_binding = args.metadata_path.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "mypkg::getIntegrationRuntimeObjectMetadatum".into(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "factoryName".into(),
                    value: &factory_name_binding,
                },
                register_interface::ObjectField {
                    name: "integrationRuntimeName".into(),
                    value: &integration_runtime_name_binding,
                },
                register_interface::ObjectField {
                    name: "metadataPath".into(),
                    value: &metadata_path_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetIntegrationRuntimeObjectMetadatumResult {
            next_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nextLink"),
            ),
            value: pulumi_gestalt_rust::__private::into_domain(o.extract_field("value")),
        }
    }
}
