#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_workspace {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetWorkspaceArgs {
        /// ID of the directory for the WorkSpace. You have to specify `user_name` along with `directory_id`. You cannot combine this parameter with `workspace_id`.
        #[builder(into, default)]
        pub directory_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Tags for the WorkSpace.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// User name of the user for the WorkSpace. This user name must exist in the directory for the WorkSpace. You cannot combine this parameter with `workspace_id`.
        #[builder(into, default)]
        pub user_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of the WorkSpace. You cannot combine this parameter with `directory_id`.
        #[builder(into, default)]
        pub workspace_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetWorkspaceResult {
        pub bundle_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the WorkSpace, as seen by the operating system.
        pub computer_name: pulumi_gestalt_rust::Output<String>,
        pub directory_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// IP address of the WorkSpace.
        pub ip_address: pulumi_gestalt_rust::Output<String>,
        pub root_volume_encryption_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Operational state of the WorkSpace.
        pub state: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        pub user_name: pulumi_gestalt_rust::Output<String>,
        pub user_volume_encryption_enabled: pulumi_gestalt_rust::Output<bool>,
        pub volume_encryption_key: pulumi_gestalt_rust::Output<String>,
        pub workspace_id: pulumi_gestalt_rust::Output<String>,
        pub workspace_properties: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::workspaces::GetWorkspaceWorkspaceProperty>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetWorkspaceArgs,
    ) -> GetWorkspaceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let directory_id_binding = args.directory_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let user_name_binding = args.user_name.get_output(context);
        let workspace_id_binding = args.workspace_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:workspaces/getWorkspace:getWorkspace".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "directoryId".into(),
                    value: directory_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userName".into(),
                    value: user_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workspaceId".into(),
                    value: workspace_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetWorkspaceResult {
            bundle_id: o.get_field("bundleId"),
            computer_name: o.get_field("computerName"),
            directory_id: o.get_field("directoryId"),
            id: o.get_field("id"),
            ip_address: o.get_field("ipAddress"),
            root_volume_encryption_enabled: o.get_field("rootVolumeEncryptionEnabled"),
            state: o.get_field("state"),
            tags: o.get_field("tags"),
            user_name: o.get_field("userName"),
            user_volume_encryption_enabled: o.get_field("userVolumeEncryptionEnabled"),
            volume_encryption_key: o.get_field("volumeEncryptionKey"),
            workspace_id: o.get_field("workspaceId"),
            workspace_properties: o.get_field("workspaceProperties"),
        }
    }
}
