/// Provides an Amazon Managed Grafana workspace resource.
///
/// ## Example Usage
///
/// ### Basic configuration
///
/// ```yaml
/// resources:
///   example:
///     type: aws:grafana:Workspace
///     properties:
///       accountAccessType: CURRENT_ACCOUNT
///       authenticationProviders:
///         - SAML
///       permissionType: SERVICE_MANAGED
///       roleArn: ${assume.arn}
///   assume:
///     type: aws:iam:Role
///     properties:
///       name: grafana-assume
///       assumeRolePolicy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Action: sts:AssumeRole
///               Effect: Allow
///               Sid: ""
///               Principal:
///                 Service: grafana.amazonaws.com
/// ```
///
/// ### Workspace configuration options
///
/// ```yaml
/// resources:
///   example:
///     type: aws:grafana:Workspace
///     properties:
///       accountAccessType: CURRENT_ACCOUNT
///       authenticationProviders:
///         - SAML
///       permissionType: SERVICE_MANAGED
///       roleArn: ${assume.arn}
///       configuration:
///         fn::toJSON:
///           plugins:
///             pluginAdminEnabled: true
///           unifiedAlerting:
///             enabled: false
/// ```
///
/// The optional argument `configuration` is a JSON string that enables the unified `Grafana Alerting` (Grafana version 10 or newer) and `Plugins Management` (Grafana version 9 or newer) on the Grafana Workspaces.
///
/// For more information about using Grafana alerting, and the effects of turning it on or off, see [Alerts in Grafana version 10](https://docs.aws.amazon.com/grafana/latest/userguide/v10-alerts.html).
///
/// ## Import
///
/// Using `pulumi import`, import Grafana Workspace using the workspace's `id`. For example:
///
/// ```sh
/// $ pulumi import aws:grafana/workspace:Workspace example g-2054c75a02
/// ```
pub mod workspace {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkspaceArgs {
        /// The type of account access for the workspace. Valid values are `CURRENT_ACCOUNT` and `ORGANIZATION`. If `ORGANIZATION` is specified, then `organizational_units` must also be present.
        #[builder(into)]
        pub account_access_type: pulumi_wasm_rust::Output<String>,
        /// The authentication providers for the workspace. Valid values are `AWS_SSO`, `SAML`, or both.
        #[builder(into)]
        pub authentication_providers: pulumi_wasm_rust::Output<Vec<String>>,
        /// The configuration string for the workspace that you create. For more information about the format and configuration options available, see [Working in your Grafana workspace](https://docs.aws.amazon.com/grafana/latest/userguide/AMG-configure-workspace.html).
        #[builder(into, default)]
        pub configuration: pulumi_wasm_rust::Output<Option<String>>,
        /// The data sources for the workspace. Valid values are `AMAZON_OPENSEARCH_SERVICE`, `ATHENA`, `CLOUDWATCH`, `PROMETHEUS`, `REDSHIFT`, `SITEWISE`, `TIMESTREAM`, `XRAY`
        #[builder(into, default)]
        pub data_sources: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The workspace description.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the version of Grafana to support in the new workspace. Supported values are `8.4`, `9.4` and `10.4`. If not specified, defaults to the latest version.
        #[builder(into, default)]
        pub grafana_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The Grafana workspace name.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration for network access to your workspace.See Network Access Control below.
        #[builder(into, default)]
        pub network_access_control: pulumi_wasm_rust::Output<
            Option<super::super::types::grafana::WorkspaceNetworkAccessControl>,
        >,
        /// The notification destinations. If a data source is specified here, Amazon Managed Grafana will create IAM roles and permissions needed to use these destinations. Must be set to `SNS`.
        #[builder(into, default)]
        pub notification_destinations: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The role name that the workspace uses to access resources through Amazon Organizations.
        #[builder(into, default)]
        pub organization_role_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The Amazon Organizations organizational units that the workspace is authorized to use data sources from.
        #[builder(into, default)]
        pub organizational_units: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The permission type of the workspace. If `SERVICE_MANAGED` is specified, the IAM roles and IAM policy attachments are generated automatically. If `CUSTOMER_MANAGED` is specified, the IAM roles and IAM policy attachments will not be created.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub permission_type: pulumi_wasm_rust::Output<String>,
        /// The IAM role ARN that the workspace assumes.
        #[builder(into, default)]
        pub role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The AWS CloudFormation stack set name that provisions IAM roles to be used by the workspace.
        #[builder(into, default)]
        pub stack_set_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The configuration settings for an Amazon VPC that contains data sources for your Grafana workspace to connect to. See VPC Configuration below.
        #[builder(into, default)]
        pub vpc_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::grafana::WorkspaceVpcConfiguration>,
        >,
    }
    #[allow(dead_code)]
    pub struct WorkspaceResult {
        /// The type of account access for the workspace. Valid values are `CURRENT_ACCOUNT` and `ORGANIZATION`. If `ORGANIZATION` is specified, then `organizational_units` must also be present.
        pub account_access_type: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the Grafana workspace.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The authentication providers for the workspace. Valid values are `AWS_SSO`, `SAML`, or both.
        pub authentication_providers: pulumi_wasm_rust::Output<Vec<String>>,
        /// The configuration string for the workspace that you create. For more information about the format and configuration options available, see [Working in your Grafana workspace](https://docs.aws.amazon.com/grafana/latest/userguide/AMG-configure-workspace.html).
        pub configuration: pulumi_wasm_rust::Output<String>,
        /// The data sources for the workspace. Valid values are `AMAZON_OPENSEARCH_SERVICE`, `ATHENA`, `CLOUDWATCH`, `PROMETHEUS`, `REDSHIFT`, `SITEWISE`, `TIMESTREAM`, `XRAY`
        pub data_sources: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The workspace description.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The endpoint of the Grafana workspace.
        pub endpoint: pulumi_wasm_rust::Output<String>,
        /// Specifies the version of Grafana to support in the new workspace. Supported values are `8.4`, `9.4` and `10.4`. If not specified, defaults to the latest version.
        pub grafana_version: pulumi_wasm_rust::Output<String>,
        /// The Grafana workspace name.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Configuration for network access to your workspace.See Network Access Control below.
        pub network_access_control: pulumi_wasm_rust::Output<
            Option<super::super::types::grafana::WorkspaceNetworkAccessControl>,
        >,
        /// The notification destinations. If a data source is specified here, Amazon Managed Grafana will create IAM roles and permissions needed to use these destinations. Must be set to `SNS`.
        pub notification_destinations: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The role name that the workspace uses to access resources through Amazon Organizations.
        pub organization_role_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The Amazon Organizations organizational units that the workspace is authorized to use data sources from.
        pub organizational_units: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The permission type of the workspace. If `SERVICE_MANAGED` is specified, the IAM roles and IAM policy attachments are generated automatically. If `CUSTOMER_MANAGED` is specified, the IAM roles and IAM policy attachments will not be created.
        ///
        /// The following arguments are optional:
        pub permission_type: pulumi_wasm_rust::Output<String>,
        /// The IAM role ARN that the workspace assumes.
        pub role_arn: pulumi_wasm_rust::Output<Option<String>>,
        pub saml_configuration_status: pulumi_wasm_rust::Output<String>,
        /// The AWS CloudFormation stack set name that provisions IAM roles to be used by the workspace.
        pub stack_set_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The configuration settings for an Amazon VPC that contains data sources for your Grafana workspace to connect to. See VPC Configuration below.
        pub vpc_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::grafana::WorkspaceVpcConfiguration>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: WorkspaceArgs) -> WorkspaceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_access_type_binding = args.account_access_type.get_inner();
        let authentication_providers_binding = args.authentication_providers.get_inner();
        let configuration_binding = args.configuration.get_inner();
        let data_sources_binding = args.data_sources.get_inner();
        let description_binding = args.description.get_inner();
        let grafana_version_binding = args.grafana_version.get_inner();
        let name_binding = args.name.get_inner();
        let network_access_control_binding = args.network_access_control.get_inner();
        let notification_destinations_binding = args
            .notification_destinations
            .get_inner();
        let organization_role_name_binding = args.organization_role_name.get_inner();
        let organizational_units_binding = args.organizational_units.get_inner();
        let permission_type_binding = args.permission_type.get_inner();
        let role_arn_binding = args.role_arn.get_inner();
        let stack_set_name_binding = args.stack_set_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let vpc_configuration_binding = args.vpc_configuration.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:grafana/workspace:Workspace".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountAccessType".into(),
                    value: &account_access_type_binding,
                },
                register_interface::ObjectField {
                    name: "authenticationProviders".into(),
                    value: &authentication_providers_binding,
                },
                register_interface::ObjectField {
                    name: "configuration".into(),
                    value: &configuration_binding,
                },
                register_interface::ObjectField {
                    name: "dataSources".into(),
                    value: &data_sources_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "grafanaVersion".into(),
                    value: &grafana_version_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networkAccessControl".into(),
                    value: &network_access_control_binding,
                },
                register_interface::ObjectField {
                    name: "notificationDestinations".into(),
                    value: &notification_destinations_binding,
                },
                register_interface::ObjectField {
                    name: "organizationRoleName".into(),
                    value: &organization_role_name_binding,
                },
                register_interface::ObjectField {
                    name: "organizationalUnits".into(),
                    value: &organizational_units_binding,
                },
                register_interface::ObjectField {
                    name: "permissionType".into(),
                    value: &permission_type_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "stackSetName".into(),
                    value: &stack_set_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "vpcConfiguration".into(),
                    value: &vpc_configuration_binding,
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
                    name: "configuration".into(),
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
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkAccessControl".into(),
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
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "vpcConfiguration".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        WorkspaceResult {
            account_access_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountAccessType").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            authentication_providers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authenticationProviders").unwrap(),
            ),
            configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configuration").unwrap(),
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
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_access_control: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkAccessControl").unwrap(),
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
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            vpc_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcConfiguration").unwrap(),
            ),
        }
    }
}
