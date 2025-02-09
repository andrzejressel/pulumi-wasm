#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_active_folder {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetActiveFolderArgs {
        /// The API method to use to search for the folder. Valid values are `LIST` and `SEARCH`. Default Value is `LIST`. `LIST` is [strongly consistent](https://cloud.google.com/resource-manager/reference/rest/v3/folders/list#:~:text=list()%20provides%20a-,strongly%20consistent,-view%20of%20the) and requires `resourcemanager.folders.list` on the parent folder, while `SEARCH` is [eventually consistent](https://cloud.google.com/resource-manager/reference/rest/v3/folders/search#:~:text=eventually%20consistent) and only returns folders that the user has `resourcemanager.folders.get` permission on.
        #[builder(into, default)]
        pub api_method: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The folder's display name.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The resource name of the parent Folder or Organization.
        #[builder(into)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetActiveFolderResult {
        pub api_method: pulumi_gestalt_rust::Output<Option<String>>,
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the Folder. This uniquely identifies the folder.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub parent: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetActiveFolderArgs,
    ) -> GetActiveFolderResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let api_method_binding_1 = args.api_method.get_output(context);
        let api_method_binding = api_method_binding_1.get_inner();
        let display_name_binding_1 = args.display_name.get_output(context);
        let display_name_binding = display_name_binding_1.get_inner();
        let parent_binding_1 = args.parent.get_output(context);
        let parent_binding = parent_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:organizations/getActiveFolder:getActiveFolder".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiMethod".into(),
                    value: &api_method_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetActiveFolderResult {
            api_method: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("apiMethod"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            parent: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parent"),
            ),
        }
    }
}
