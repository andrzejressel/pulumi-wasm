pub mod get_directory {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDirectoryArgs {
        /// Directory identifier for registration in WorkSpaces service.
        #[builder(into)]
        pub directory_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// A map of tags assigned to the WorkSpaces directory.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetDirectoryResult {
        /// Directory alias.
        pub alias: pulumi_wasm_rust::Output<String>,
        /// User name for the service account.
        pub customer_user_name: pulumi_wasm_rust::Output<String>,
        pub directory_id: pulumi_wasm_rust::Output<String>,
        /// Name of the directory.
        pub directory_name: pulumi_wasm_rust::Output<String>,
        /// Directory type.
        pub directory_type: pulumi_wasm_rust::Output<String>,
        /// IP addresses of the DNS servers for the directory.
        pub dns_ip_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// Identifier of the IAM role. This is the role that allows Amazon WorkSpaces to make calls to other services, such as Amazon EC2, on your behalf.
        pub iam_role_id: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Identifiers of the IP access control groups associated with the directory.
        pub ip_group_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Registration code for the directory. This is the code that users enter in their Amazon WorkSpaces client application to connect to the directory.
        pub registration_code: pulumi_wasm_rust::Output<String>,
        pub saml_properties: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::workspaces::GetDirectorySamlProperty>,
        >,
        /// The permissions to enable or disable self-service capabilities.
        pub self_service_permissions: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::workspaces::GetDirectorySelfServicePermission,
            >,
        >,
        /// Identifiers of the subnets where the directory resides.
        pub subnet_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// A map of tags assigned to the WorkSpaces directory.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// (Optional) Specifies which devices and operating systems users can use to access their WorkSpaces. Defined below.
        pub workspace_access_properties: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::workspaces::GetDirectoryWorkspaceAccessProperty,
            >,
        >,
        /// The default properties that are used for creating WorkSpaces. Defined below.
        pub workspace_creation_properties: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::workspaces::GetDirectoryWorkspaceCreationProperty,
            >,
        >,
        /// The identifier of the security group that is assigned to new WorkSpaces. Defined below.
        pub workspace_security_group_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetDirectoryArgs,
    ) -> GetDirectoryResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let directory_id_binding = args.directory_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:workspaces/getDirectory:getDirectory".into(),
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
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDirectoryResult {
            alias: pulumi_wasm_rust::__private::into_domain(o.extract_field("alias")),
            customer_user_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customerUserName"),
            ),
            directory_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("directoryId"),
            ),
            directory_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("directoryName"),
            ),
            directory_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("directoryType"),
            ),
            dns_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dnsIpAddresses"),
            ),
            iam_role_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("iamRoleId"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            ip_group_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ipGroupIds"),
            ),
            registration_code: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("registrationCode"),
            ),
            saml_properties: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("samlProperties"),
            ),
            self_service_permissions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("selfServicePermissions"),
            ),
            subnet_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("subnetIds"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            workspace_access_properties: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("workspaceAccessProperties"),
            ),
            workspace_creation_properties: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("workspaceCreationProperties"),
            ),
            workspace_security_group_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("workspaceSecurityGroupId"),
            ),
        }
    }
}
