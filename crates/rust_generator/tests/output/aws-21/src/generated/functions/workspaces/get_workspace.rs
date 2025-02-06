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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetWorkspaceArgs,
    ) -> GetWorkspaceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let directory_id_binding = args.directory_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let user_name_binding = args.user_name.get_output(context).get_inner();
        let workspace_id_binding = args.workspace_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:workspaces/getWorkspace:getWorkspace".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "directoryId".into(),
                    value: &directory_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "userName".into(),
                    value: &user_name_binding,
                },
                register_interface::ObjectField {
                    name: "workspaceId".into(),
                    value: &workspace_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetWorkspaceResult {
            bundle_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bundleId"),
            ),
            computer_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("computerName"),
            ),
            directory_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("directoryId"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            ip_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipAddress"),
            ),
            root_volume_encryption_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("rootVolumeEncryptionEnabled"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            user_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userName"),
            ),
            user_volume_encryption_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userVolumeEncryptionEnabled"),
            ),
            volume_encryption_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("volumeEncryptionKey"),
            ),
            workspace_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workspaceId"),
            ),
            workspace_properties: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workspaceProperties"),
            ),
        }
    }
}
