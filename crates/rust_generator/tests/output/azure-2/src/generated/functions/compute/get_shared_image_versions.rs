#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_shared_image_versions {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSharedImageVersionsArgs {
        /// The name of the Shared Image in which the Shared Image exists.
        #[builder(into)]
        pub gallery_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Shared Image in which this Version exists.
        #[builder(into)]
        pub image_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group in which the Shared Image Gallery exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to filter the list of images against.
        #[builder(into, default)]
        pub tags_filter: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetSharedImageVersionsResult {
        pub gallery_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub image_name: pulumi_gestalt_rust::Output<String>,
        /// An `images` block as defined below:
        pub images: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetSharedImageVersionsImage>,
        >,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        pub tags_filter: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetSharedImageVersionsArgs,
    ) -> GetSharedImageVersionsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let gallery_name_binding_1 = args.gallery_name.get_output(context);
        let gallery_name_binding = gallery_name_binding_1.get_inner();
        let image_name_binding_1 = args.image_name.get_output(context);
        let image_name_binding = image_name_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let tags_filter_binding_1 = args.tags_filter.get_output(context);
        let tags_filter_binding = tags_filter_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:compute/getSharedImageVersions:getSharedImageVersions".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "galleryName".into(),
                    value: &gallery_name_binding,
                },
                register_interface::ObjectField {
                    name: "imageName".into(),
                    value: &image_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tagsFilter".into(),
                    value: &tags_filter_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSharedImageVersionsResult {
            gallery_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("galleryName"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            image_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("imageName"),
            ),
            images: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("images"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags_filter: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsFilter"),
            ),
        }
    }
}
