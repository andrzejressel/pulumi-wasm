pub mod get_workspace {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetWorkspaceArgs {
        /// Tags assigned to the resource
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Grafana workspace ID.
        #[builder(into)]
        pub workspace_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetWorkspaceResult {
        /// (Required) Type of account access for the workspace. Valid values are `CURRENT_ACCOUNT` and `ORGANIZATION`. If `ORGANIZATION` is specified, then `organizational_units` must also be present.
        pub account_access_type: pulumi_wasm_rust::Output<String>,
        /// ARN of the Grafana workspace.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// (Required) Authentication providers for the workspace. Valid values are `AWS_SSO`, `SAML`, or both.
        pub authentication_providers: pulumi_wasm_rust::Output<Vec<String>>,
        /// Creation date of the Grafana workspace.
        pub created_date: pulumi_wasm_rust::Output<String>,
        /// Data sources for the workspace.
        pub data_sources: pulumi_wasm_rust::Output<Vec<String>>,
        /// Workspace description.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Endpoint of the Grafana workspace.
        pub endpoint: pulumi_wasm_rust::Output<String>,
        /// Version of Grafana running on the workspace.
        pub grafana_version: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Last updated date of the Grafana workspace.
        pub last_updated_date: pulumi_wasm_rust::Output<String>,
        /// Grafana workspace name.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The notification destinations.
        pub notification_destinations: pulumi_wasm_rust::Output<Vec<String>>,
        /// The role name that the workspace uses to access resources through Amazon Organizations.
        pub organization_role_name: pulumi_wasm_rust::Output<String>,
        /// The Amazon Organizations organizational units that the workspace is authorized to use data sources from.
        pub organizational_units: pulumi_wasm_rust::Output<Vec<String>>,
        /// Permission type of the workspace.
        pub permission_type: pulumi_wasm_rust::Output<String>,
        /// IAM role ARN that the workspace assumes.
        pub role_arn: pulumi_wasm_rust::Output<String>,
        pub saml_configuration_status: pulumi_wasm_rust::Output<String>,
        /// AWS CloudFormation stack set name that provisions IAM roles to be used by the workspace.
        pub stack_set_name: pulumi_wasm_rust::Output<String>,
        /// Status of the Grafana workspace.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Tags assigned to the resource
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub workspace_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetWorkspaceArgs) -> GetWorkspaceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let tags_binding = args.tags.get_inner();
        let workspace_id_binding = args.workspace_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:grafana/getWorkspace:getWorkspace".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "workspaceId".into(),
                    value: &workspace_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountAccessType".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "authenticationProviders".into(),
                },
                register_interface::ResultField {
                    name: "createdDate".into(),
                },
                register_interface::ResultField {
                    name: "dataSources".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "endpoint".into(),
                },
                register_interface::ResultField {
                    name: "grafanaVersion".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "lastUpdatedDate".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "notificationDestinations".into(),
                },
                register_interface::ResultField {
                    name: "organizationRoleName".into(),
                },
                register_interface::ResultField {
                    name: "organizationalUnits".into(),
                },
                register_interface::ResultField {
                    name: "permissionType".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
                register_interface::ResultField {
                    name: "samlConfigurationStatus".into(),
                },
                register_interface::ResultField {
                    name: "stackSetName".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "workspaceId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetWorkspaceResult {
            account_access_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountAccessType").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            authentication_providers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authenticationProviders").unwrap(),
            ),
            created_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdDate").unwrap(),
            ),
            data_sources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataSources").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpoint").unwrap(),
            ),
            grafana_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("grafanaVersion").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            last_updated_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastUpdatedDate").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            notification_destinations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notificationDestinations").unwrap(),
            ),
            organization_role_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("organizationRoleName").unwrap(),
            ),
            organizational_units: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("organizationalUnits").unwrap(),
            ),
            permission_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("permissionType").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
            saml_configuration_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("samlConfigurationStatus").unwrap(),
            ),
            stack_set_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stackSetName").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            workspace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workspaceId").unwrap(),
            ),
        }
    }
}
