#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod catalog {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CatalogArgs {
        #[builder(into, default)]
        pub catalog_adogit: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::devcenter::CatalogCatalogAdogit>,
        >,
        #[builder(into, default)]
        pub catalog_github: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::devcenter::CatalogCatalogGithub>,
        >,
        #[builder(into)]
        pub dev_center_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CatalogResult {
        pub catalog_adogit: pulumi_gestalt_rust::Output<
            Option<super::super::types::devcenter::CatalogCatalogAdogit>,
        >,
        pub catalog_github: pulumi_gestalt_rust::Output<
            Option<super::super::types::devcenter::CatalogCatalogGithub>,
        >,
        pub dev_center_id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CatalogArgs,
    ) -> CatalogResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let catalog_adogit_binding = args.catalog_adogit.get_output(context);
        let catalog_github_binding = args.catalog_github.get_output(context);
        let dev_center_id_binding = args.dev_center_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:devcenter/catalog:Catalog".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "catalogAdogit".into(),
                    value: &catalog_adogit_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "catalogGithub".into(),
                    value: &catalog_github_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "devCenterId".into(),
                    value: &dev_center_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        CatalogResult {
            catalog_adogit: o.get_field("catalogAdogit"),
            catalog_github: o.get_field("catalogGithub"),
            dev_center_id: o.get_field("devCenterId"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
        }
    }
}
