pub mod get_directory {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDirectoryArgs {
        /// Directory identifier for registration in WorkSpaces service.
        #[builder(into)]
        pub directory_id: pulumi_wasm_rust::Output<String>,
        /// A map of tags assigned to the WorkSpaces directory.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
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
    pub fn invoke(args: GetDirectoryArgs) -> GetDirectoryResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let directory_id_binding = args.directory_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:workspaces/getDirectory:getDirectory".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "alias".into(),
                },
                register_interface::ResultField {
                    name: "customerUserName".into(),
                },
                register_interface::ResultField {
                    name: "directoryId".into(),
                },
                register_interface::ResultField {
                    name: "directoryName".into(),
                },
                register_interface::ResultField {
                    name: "directoryType".into(),
                },
                register_interface::ResultField {
                    name: "dnsIpAddresses".into(),
                },
                register_interface::ResultField {
                    name: "iamRoleId".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ipGroupIds".into(),
                },
                register_interface::ResultField {
                    name: "registrationCode".into(),
                },
                register_interface::ResultField {
                    name: "samlProperties".into(),
                },
                register_interface::ResultField {
                    name: "selfServicePermissions".into(),
                },
                register_interface::ResultField {
                    name: "subnetIds".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "workspaceAccessProperties".into(),
                },
                register_interface::ResultField {
                    name: "workspaceCreationProperties".into(),
                },
                register_interface::ResultField {
                    name: "workspaceSecurityGroupId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetDirectoryResult {
            alias: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alias").unwrap(),
            ),
            customer_user_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerUserName").unwrap(),
            ),
            directory_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("directoryId").unwrap(),
            ),
            directory_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("directoryName").unwrap(),
            ),
            directory_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("directoryType").unwrap(),
            ),
            dns_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsIpAddresses").unwrap(),
            ),
            iam_role_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iamRoleId").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ip_group_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipGroupIds").unwrap(),
            ),
            registration_code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("registrationCode").unwrap(),
            ),
            saml_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("samlProperties").unwrap(),
            ),
            self_service_permissions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfServicePermissions").unwrap(),
            ),
            subnet_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetIds").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            workspace_access_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workspaceAccessProperties").unwrap(),
            ),
            workspace_creation_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workspaceCreationProperties").unwrap(),
            ),
            workspace_security_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workspaceSecurityGroupId").unwrap(),
            ),
        }
    }
}
