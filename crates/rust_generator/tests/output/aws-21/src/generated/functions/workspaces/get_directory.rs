#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_directory {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDirectoryArgs {
        /// Directory identifier for registration in WorkSpaces service.
        #[builder(into)]
        pub directory_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of tags assigned to the WorkSpaces directory.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetDirectoryResult {
        /// Directory alias.
        pub alias: pulumi_gestalt_rust::Output<String>,
        /// User name for the service account.
        pub customer_user_name: pulumi_gestalt_rust::Output<String>,
        pub directory_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the directory.
        pub directory_name: pulumi_gestalt_rust::Output<String>,
        /// Directory type.
        pub directory_type: pulumi_gestalt_rust::Output<String>,
        /// IP addresses of the DNS servers for the directory.
        pub dns_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Identifier of the IAM role. This is the role that allows Amazon WorkSpaces to make calls to other services, such as Amazon EC2, on your behalf.
        pub iam_role_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Identifiers of the IP access control groups associated with the directory.
        pub ip_group_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Registration code for the directory. This is the code that users enter in their Amazon WorkSpaces client application to connect to the directory.
        pub registration_code: pulumi_gestalt_rust::Output<String>,
        pub saml_properties: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::workspaces::GetDirectorySamlProperty>,
        >,
        /// The permissions to enable or disable self-service capabilities.
        pub self_service_permissions: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::workspaces::GetDirectorySelfServicePermission,
            >,
        >,
        /// Identifiers of the subnets where the directory resides.
        pub subnet_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A map of tags assigned to the WorkSpaces directory.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// (Optional) Specifies which devices and operating systems users can use to access their WorkSpaces. Defined below.
        pub workspace_access_properties: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::workspaces::GetDirectoryWorkspaceAccessProperty,
            >,
        >,
        /// The default properties that are used for creating WorkSpaces. Defined below.
        pub workspace_creation_properties: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::workspaces::GetDirectoryWorkspaceCreationProperty,
            >,
        >,
        /// The identifier of the security group that is assigned to new WorkSpaces. Defined below.
        pub workspace_security_group_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetDirectoryArgs,
    ) -> GetDirectoryResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let directory_id_binding = args.directory_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:workspaces/getDirectory:getDirectory".into(),
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
            ],
        };
        let o = context.invoke_resource(request);
        GetDirectoryResult {
            alias: o.get_field("alias"),
            customer_user_name: o.get_field("customerUserName"),
            directory_id: o.get_field("directoryId"),
            directory_name: o.get_field("directoryName"),
            directory_type: o.get_field("directoryType"),
            dns_ip_addresses: o.get_field("dnsIpAddresses"),
            iam_role_id: o.get_field("iamRoleId"),
            id: o.get_field("id"),
            ip_group_ids: o.get_field("ipGroupIds"),
            registration_code: o.get_field("registrationCode"),
            saml_properties: o.get_field("samlProperties"),
            self_service_permissions: o.get_field("selfServicePermissions"),
            subnet_ids: o.get_field("subnetIds"),
            tags: o.get_field("tags"),
            workspace_access_properties: o.get_field("workspaceAccessProperties"),
            workspace_creation_properties: o.get_field("workspaceCreationProperties"),
            workspace_security_group_id: o.get_field("workspaceSecurityGroupId"),
        }
    }
}
