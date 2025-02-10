#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_folder {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFolderArgs {
        /// The name of the Folder in the form `{folder_id}` or `folders/{folder_id}`.
        #[builder(into)]
        pub folder: pulumi_gestalt_rust::InputOrOutput<String>,
        /// `true` to find the organization that the folder belongs, `false` to avoid the lookup. It searches up the tree. (defaults to `false`)
        #[builder(into, default)]
        pub lookup_organization: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct GetFolderResult {
        /// Timestamp when the Organization was created. A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_gestalt_rust::Output<String>,
        pub deletion_protection: pulumi_gestalt_rust::Output<bool>,
        /// The folder's display name.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        pub folder: pulumi_gestalt_rust::Output<String>,
        pub folder_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Folder's current lifecycle state.
        pub lifecycle_state: pulumi_gestalt_rust::Output<String>,
        pub lookup_organization: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The resource name of the Folder in the form `folders/{folder_id}`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// If `lookup_organization` is enable, the resource name of the Organization that the folder belongs.
        pub organization: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the parent Folder or Organization.
        pub parent: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetFolderArgs,
    ) -> GetFolderResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let folder_binding = args.folder.get_output(context);
        let lookup_organization_binding = args.lookup_organization.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:organizations/getFolder:getFolder".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "folder".into(),
                    value: folder_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lookupOrganization".into(),
                    value: lookup_organization_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetFolderResult {
            create_time: o.get_field("createTime"),
            deletion_protection: o.get_field("deletionProtection"),
            display_name: o.get_field("displayName"),
            folder: o.get_field("folder"),
            folder_id: o.get_field("folderId"),
            id: o.get_field("id"),
            lifecycle_state: o.get_field("lifecycleState"),
            lookup_organization: o.get_field("lookupOrganization"),
            name: o.get_field("name"),
            organization: o.get_field("organization"),
            parent: o.get_field("parent"),
        }
    }
}
