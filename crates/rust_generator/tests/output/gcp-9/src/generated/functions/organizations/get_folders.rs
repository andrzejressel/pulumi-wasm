#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_folders {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFoldersArgs {
        /// A string parent as defined in the [REST API](https://cloud.google.com/resource-manager/reference/rest/v3/folders/list#query-parameters).
        #[builder(into)]
        pub parent_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetFoldersResult {
        /// A list of folders matching the provided filter. Structure is defined below.
        pub folders: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::organizations::GetFoldersFolder>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub parent_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetFoldersArgs,
    ) -> GetFoldersResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let parent_id_binding = args.parent_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:organizations/getFolders:getFolders".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "parentId".into(),
                    value: &parent_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetFoldersResult {
            folders: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("folders"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            parent_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parentId"),
            ),
        }
    }
}
