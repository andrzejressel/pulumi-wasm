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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod workspace {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkspaceArgs {
        /// The type of account access for the workspace. Valid values are `CURRENT_ACCOUNT` and `ORGANIZATION`. If `ORGANIZATION` is specified, then `organizational_units` must also be present.
        #[builder(into)]
        pub account_access_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The authentication providers for the workspace. Valid values are `AWS_SSO`, `SAML`, or both.
        #[builder(into)]
        pub authentication_providers: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The configuration string for the workspace that you create. For more information about the format and configuration options available, see [Working in your Grafana workspace](https://docs.aws.amazon.com/grafana/latest/userguide/AMG-configure-workspace.html).
        #[builder(into, default)]
        pub configuration: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The data sources for the workspace. Valid values are `AMAZON_OPENSEARCH_SERVICE`, `ATHENA`, `CLOUDWATCH`, `PROMETHEUS`, `REDSHIFT`, `SITEWISE`, `TIMESTREAM`, `XRAY`
        #[builder(into, default)]
        pub data_sources: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The workspace description.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the version of Grafana to support in the new workspace. Supported values are `8.4`, `9.4` and `10.4`. If not specified, defaults to the latest version.
        #[builder(into, default)]
        pub grafana_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Grafana workspace name.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration for network access to your workspace.See Network Access Control below.
        #[builder(into, default)]
        pub network_access_control: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::grafana::WorkspaceNetworkAccessControl>,
        >,
        /// The notification destinations. If a data source is specified here, Amazon Managed Grafana will create IAM roles and permissions needed to use these destinations. Must be set to `SNS`.
        #[builder(into, default)]
        pub notification_destinations: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The role name that the workspace uses to access resources through Amazon Organizations.
        #[builder(into, default)]
        pub organization_role_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Amazon Organizations organizational units that the workspace is authorized to use data sources from.
        #[builder(into, default)]
        pub organizational_units: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The permission type of the workspace. If `SERVICE_MANAGED` is specified, the IAM roles and IAM policy attachments are generated automatically. If `CUSTOMER_MANAGED` is specified, the IAM roles and IAM policy attachments will not be created.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub permission_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The IAM role ARN that the workspace assumes.
        #[builder(into, default)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The AWS CloudFormation stack set name that provisions IAM roles to be used by the workspace.
        #[builder(into, default)]
        pub stack_set_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The configuration settings for an Amazon VPC that contains data sources for your Grafana workspace to connect to. See VPC Configuration below.
        #[builder(into, default)]
        pub vpc_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::grafana::WorkspaceVpcConfiguration>,
        >,
    }
    #[allow(dead_code)]
    pub struct WorkspaceResult {
        /// The type of account access for the workspace. Valid values are `CURRENT_ACCOUNT` and `ORGANIZATION`. If `ORGANIZATION` is specified, then `organizational_units` must also be present.
        pub account_access_type: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the Grafana workspace.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The authentication providers for the workspace. Valid values are `AWS_SSO`, `SAML`, or both.
        pub authentication_providers: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The configuration string for the workspace that you create. For more information about the format and configuration options available, see [Working in your Grafana workspace](https://docs.aws.amazon.com/grafana/latest/userguide/AMG-configure-workspace.html).
        pub configuration: pulumi_gestalt_rust::Output<String>,
        /// The data sources for the workspace. Valid values are `AMAZON_OPENSEARCH_SERVICE`, `ATHENA`, `CLOUDWATCH`, `PROMETHEUS`, `REDSHIFT`, `SITEWISE`, `TIMESTREAM`, `XRAY`
        pub data_sources: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The workspace description.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The endpoint of the Grafana workspace.
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// Specifies the version of Grafana to support in the new workspace. Supported values are `8.4`, `9.4` and `10.4`. If not specified, defaults to the latest version.
        pub grafana_version: pulumi_gestalt_rust::Output<String>,
        /// The Grafana workspace name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Configuration for network access to your workspace.See Network Access Control below.
        pub network_access_control: pulumi_gestalt_rust::Output<
            Option<super::super::types::grafana::WorkspaceNetworkAccessControl>,
        >,
        /// The notification destinations. If a data source is specified here, Amazon Managed Grafana will create IAM roles and permissions needed to use these destinations. Must be set to `SNS`.
        pub notification_destinations: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The role name that the workspace uses to access resources through Amazon Organizations.
        pub organization_role_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Amazon Organizations organizational units that the workspace is authorized to use data sources from.
        pub organizational_units: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The permission type of the workspace. If `SERVICE_MANAGED` is specified, the IAM roles and IAM policy attachments are generated automatically. If `CUSTOMER_MANAGED` is specified, the IAM roles and IAM policy attachments will not be created.
        ///
        /// The following arguments are optional:
        pub permission_type: pulumi_gestalt_rust::Output<String>,
        /// The IAM role ARN that the workspace assumes.
        pub role_arn: pulumi_gestalt_rust::Output<Option<String>>,
        pub saml_configuration_status: pulumi_gestalt_rust::Output<String>,
        /// The AWS CloudFormation stack set name that provisions IAM roles to be used by the workspace.
        pub stack_set_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The configuration settings for an Amazon VPC that contains data sources for your Grafana workspace to connect to. See VPC Configuration below.
        pub vpc_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::grafana::WorkspaceVpcConfiguration>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkspaceArgs,
    ) -> WorkspaceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_access_type_binding = args.account_access_type.get_output(context);
        let authentication_providers_binding = args
            .authentication_providers
            .get_output(context);
        let configuration_binding = args.configuration.get_output(context);
        let data_sources_binding = args.data_sources.get_output(context);
        let description_binding = args.description.get_output(context);
        let grafana_version_binding = args.grafana_version.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_access_control_binding = args
            .network_access_control
            .get_output(context);
        let notification_destinations_binding = args
            .notification_destinations
            .get_output(context);
        let organization_role_name_binding = args
            .organization_role_name
            .get_output(context);
        let organizational_units_binding = args.organizational_units.get_output(context);
        let permission_type_binding = args.permission_type.get_output(context);
        let role_arn_binding = args.role_arn.get_output(context);
        let stack_set_name_binding = args.stack_set_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let vpc_configuration_binding = args.vpc_configuration.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:grafana/workspace:Workspace".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountAccessType".into(),
                    value: &account_access_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authenticationProviders".into(),
                    value: &authentication_providers_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configuration".into(),
                    value: &configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataSources".into(),
                    value: &data_sources_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "grafanaVersion".into(),
                    value: &grafana_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkAccessControl".into(),
                    value: &network_access_control_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "notificationDestinations".into(),
                    value: &notification_destinations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "organizationRoleName".into(),
                    value: &organization_role_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "organizationalUnits".into(),
                    value: &organizational_units_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "permissionType".into(),
                    value: &permission_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "stackSetName".into(),
                    value: &stack_set_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcConfiguration".into(),
                    value: &vpc_configuration_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        WorkspaceResult {
            account_access_type: o.get_field("accountAccessType"),
            arn: o.get_field("arn"),
            authentication_providers: o.get_field("authenticationProviders"),
            configuration: o.get_field("configuration"),
            data_sources: o.get_field("dataSources"),
            description: o.get_field("description"),
            endpoint: o.get_field("endpoint"),
            grafana_version: o.get_field("grafanaVersion"),
            name: o.get_field("name"),
            network_access_control: o.get_field("networkAccessControl"),
            notification_destinations: o.get_field("notificationDestinations"),
            organization_role_name: o.get_field("organizationRoleName"),
            organizational_units: o.get_field("organizationalUnits"),
            permission_type: o.get_field("permissionType"),
            role_arn: o.get_field("roleArn"),
            saml_configuration_status: o.get_field("samlConfigurationStatus"),
            stack_set_name: o.get_field("stackSetName"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            vpc_configuration: o.get_field("vpcConfiguration"),
        }
    }
}
