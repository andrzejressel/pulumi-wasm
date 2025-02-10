/// Allows management of a Google Cloud Platform folder. For more information see
/// [the official documentation](https://cloud.google.com/resource-manager/docs/creating-managing-folders)
/// and
/// [API](https://cloud.google.com/resource-manager/reference/rest/v2/folders).
///
/// A folder can contain projects, other folders, or a combination of both. You can use folders to group projects under an organization in a hierarchy. For example, your organization might contain multiple departments, each with its own set of Cloud Platform resources. Folders allows you to group these resources on a per-department basis. Folders are used to group resources that share common IAM policies.
///
/// Folders created live inside an Organization. See the [Organization documentation](https://cloud.google.com/resource-manager/docs/quickstarts) for more details.
///
/// The service account used to run the provider when creating a `gcp.organizations.Folder`
/// resource must have `roles/resourcemanager.folderCreator`. See the
/// [Access Control for Folders Using IAM](https://cloud.google.com/resource-manager/docs/access-control-folders)
/// doc for more information.
///
/// > It may take a while for the attached tag bindings to be deleted after the folder is scheduled to be deleted.
///
/// ## Import
///
/// Folders can be imported using the folder's id, e.g.
///
/// * `folders/{{folder_id}}`
///
/// * `{{folder_id}}`
///
/// When using the `pulumi import` command, Folders can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:organizations/folder:Folder default {{folder_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:organizations/folder:Folder default folders/{{folder_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod folder {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FolderArgs {
        #[builder(into, default)]
        pub deletion_protection: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The folder’s display name.
        /// A folder’s display name must be unique amongst its siblings, e.g. no two folders with the same parent can share the same display name. The display name must start and end with a letter or digit, may contain letters, digits, spaces, hyphens and underscores and can be no longer than 30 characters.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The resource name of the parent Folder or Organization.
        /// Must be of the form `folders/{folder_id}` or `organizations/{org_id}`.
        #[builder(into)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of resource manager tags. Resource manager tag keys and values have the same definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/456. The field is ignored when empty. The field is immutable and causes resource replacement when  mutated. This field is only set at create time and modifying this field after creation will trigger recreation. To apply tags to an existing resource, see the `gcp.tags.TagValue` resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct FolderResult {
        /// Timestamp when the Folder was created. Assigned by the server.
        /// A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_gestalt_rust::Output<String>,
        pub deletion_protection: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The folder’s display name.
        /// A folder’s display name must be unique amongst its siblings, e.g. no two folders with the same parent can share the same display name. The display name must start and end with a letter or digit, may contain letters, digits, spaces, hyphens and underscores and can be no longer than 30 characters.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The folder id from the name "folders/{folder_id}"
        pub folder_id: pulumi_gestalt_rust::Output<String>,
        /// The lifecycle state of the folder such as `ACTIVE` or `DELETE_REQUESTED`.
        pub lifecycle_state: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the Folder. Its format is folders/{folder_id}.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the parent Folder or Organization.
        /// Must be of the form `folders/{folder_id}` or `organizations/{org_id}`.
        pub parent: pulumi_gestalt_rust::Output<String>,
        /// A map of resource manager tags. Resource manager tag keys and values have the same definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/456. The field is ignored when empty. The field is immutable and causes resource replacement when  mutated. This field is only set at create time and modifying this field after creation will trigger recreation. To apply tags to an existing resource, see the `gcp.tags.TagValue` resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FolderArgs,
    ) -> FolderResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let deletion_protection_binding = args.deletion_protection.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let parent_binding = args.parent.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:organizations/folder:Folder".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deletionProtection".into(),
                    value: deletion_protection_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parent".into(),
                    value: parent_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        FolderResult {
            create_time: o.get_field("createTime"),
            deletion_protection: o.get_field("deletionProtection"),
            display_name: o.get_field("displayName"),
            folder_id: o.get_field("folderId"),
            lifecycle_state: o.get_field("lifecycleState"),
            name: o.get_field("name"),
            parent: o.get_field("parent"),
            tags: o.get_field("tags"),
        }
    }
}
