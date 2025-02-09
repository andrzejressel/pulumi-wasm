/// Provides an Elastic MapReduce Studio.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = studio::create(
///         "example",
///         StudioArgs::builder()
///             .auth_mode("SSO")
///             .default_s_3_location("s3://${test.bucket}/test")
///             .engine_security_group_id("${testAwsSecurityGroup.id}")
///             .name("example")
///             .service_role("${testAwsIamRole.arn}")
///             .subnet_ids(vec!["${testAwsSubnet.id}",])
///             .user_role("${testAwsIamRole.arn}")
///             .vpc_id("${testAwsVpc.id}")
///             .workspace_security_group_id("${testAwsSecurityGroup.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EMR studios using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:emr/studio:Studio studio es-123456ABCDEF
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod studio {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StudioArgs {
        /// Specifies whether the Studio authenticates users using IAM or Amazon Web Services SSO. Valid values are `SSO` or `IAM`.
        #[builder(into)]
        pub auth_mode: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Amazon S3 location to back up Amazon EMR Studio Workspaces and notebook files.
        #[builder(into)]
        pub default_s3_location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A detailed description of the Amazon EMR Studio.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Amazon EMR Studio Engine security group. The Engine security group allows inbound network traffic from the Workspace security group, and it must be in the same VPC specified by `vpc_id`.
        #[builder(into)]
        pub engine_security_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The authentication endpoint of your identity provider (IdP). Specify this value when you use IAM authentication and want to let federated users log in to a Studio with the Studio URL and credentials from your IdP. Amazon EMR Studio redirects users to this endpoint to enter credentials.
        #[builder(into, default)]
        pub idp_auth_url: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name that your identity provider (IdP) uses for its RelayState parameter. For example, RelayState or TargetSource. Specify this value when you use IAM authentication and want to let federated users log in to a Studio using the Studio URL. The RelayState parameter differs by IdP.
        #[builder(into, default)]
        pub idp_relay_state_parameter_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// A descriptive name for the Amazon EMR Studio.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The IAM role that the Amazon EMR Studio assumes. The service role provides a way for Amazon EMR Studio to interoperate with other Amazon Web Services services.
        #[builder(into)]
        pub service_role: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of subnet IDs to associate with the Amazon EMR Studio. A Studio can have a maximum of 5 subnets. The subnets must belong to the VPC specified by `vpc_id`. Studio users can create a Workspace in any of the specified subnets.
        #[builder(into)]
        pub subnet_ids: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// list of tags to apply to the EMR Cluster. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The IAM user role that users and groups assume when logged in to an Amazon EMR Studio. Only specify a User Role when you use Amazon Web Services SSO authentication. The permissions attached to the User Role can be scoped down for each user or group using session policies.
        #[builder(into, default)]
        pub user_role: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Amazon Virtual Private Cloud (Amazon VPC) to associate with the Studio.
        #[builder(into)]
        pub vpc_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Amazon EMR Studio Workspace security group. The Workspace security group allows outbound network traffic to resources in the Engine security group, and it must be in the same VPC specified by `vpc_id`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub workspace_security_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct StudioResult {
        /// ARN of the studio.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether the Studio authenticates users using IAM or Amazon Web Services SSO. Valid values are `SSO` or `IAM`.
        pub auth_mode: pulumi_gestalt_rust::Output<String>,
        /// The Amazon S3 location to back up Amazon EMR Studio Workspaces and notebook files.
        pub default_s3_location: pulumi_gestalt_rust::Output<String>,
        /// A detailed description of the Amazon EMR Studio.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Amazon EMR Studio Engine security group. The Engine security group allows inbound network traffic from the Workspace security group, and it must be in the same VPC specified by `vpc_id`.
        pub engine_security_group_id: pulumi_gestalt_rust::Output<String>,
        /// The authentication endpoint of your identity provider (IdP). Specify this value when you use IAM authentication and want to let federated users log in to a Studio with the Studio URL and credentials from your IdP. Amazon EMR Studio redirects users to this endpoint to enter credentials.
        pub idp_auth_url: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name that your identity provider (IdP) uses for its RelayState parameter. For example, RelayState or TargetSource. Specify this value when you use IAM authentication and want to let federated users log in to a Studio using the Studio URL. The RelayState parameter differs by IdP.
        pub idp_relay_state_parameter_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// A descriptive name for the Amazon EMR Studio.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The IAM role that the Amazon EMR Studio assumes. The service role provides a way for Amazon EMR Studio to interoperate with other Amazon Web Services services.
        pub service_role: pulumi_gestalt_rust::Output<String>,
        /// A list of subnet IDs to associate with the Amazon EMR Studio. A Studio can have a maximum of 5 subnets. The subnets must belong to the VPC specified by `vpc_id`. Studio users can create a Workspace in any of the specified subnets.
        pub subnet_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// list of tags to apply to the EMR Cluster. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The unique access URL of the Amazon EMR Studio.
        pub url: pulumi_gestalt_rust::Output<String>,
        /// The IAM user role that users and groups assume when logged in to an Amazon EMR Studio. Only specify a User Role when you use Amazon Web Services SSO authentication. The permissions attached to the User Role can be scoped down for each user or group using session policies.
        pub user_role: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Amazon Virtual Private Cloud (Amazon VPC) to associate with the Studio.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Amazon EMR Studio Workspace security group. The Workspace security group allows outbound network traffic to resources in the Engine security group, and it must be in the same VPC specified by `vpc_id`.
        ///
        /// The following arguments are optional:
        pub workspace_security_group_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: StudioArgs,
    ) -> StudioResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let auth_mode_binding = args.auth_mode.get_output(context);
        let default_s3_location_binding = args.default_s3_location.get_output(context);
        let description_binding = args.description.get_output(context);
        let engine_security_group_id_binding = args
            .engine_security_group_id
            .get_output(context);
        let idp_auth_url_binding = args.idp_auth_url.get_output(context);
        let idp_relay_state_parameter_name_binding = args
            .idp_relay_state_parameter_name
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let service_role_binding = args.service_role.get_output(context);
        let subnet_ids_binding = args.subnet_ids.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let user_role_binding = args.user_role.get_output(context);
        let vpc_id_binding = args.vpc_id.get_output(context);
        let workspace_security_group_id_binding = args
            .workspace_security_group_id
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:emr/studio:Studio".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authMode".into(),
                    value: auth_mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultS3Location".into(),
                    value: default_s3_location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "engineSecurityGroupId".into(),
                    value: engine_security_group_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "idpAuthUrl".into(),
                    value: idp_auth_url_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "idpRelayStateParameterName".into(),
                    value: idp_relay_state_parameter_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceRole".into(),
                    value: service_role_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetIds".into(),
                    value: subnet_ids_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userRole".into(),
                    value: user_role_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcId".into(),
                    value: vpc_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workspaceSecurityGroupId".into(),
                    value: workspace_security_group_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        StudioResult {
            arn: o.get_field("arn"),
            auth_mode: o.get_field("authMode"),
            default_s3_location: o.get_field("defaultS3Location"),
            description: o.get_field("description"),
            engine_security_group_id: o.get_field("engineSecurityGroupId"),
            idp_auth_url: o.get_field("idpAuthUrl"),
            idp_relay_state_parameter_name: o.get_field("idpRelayStateParameterName"),
            name: o.get_field("name"),
            service_role: o.get_field("serviceRole"),
            subnet_ids: o.get_field("subnetIds"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            url: o.get_field("url"),
            user_role: o.get_field("userRole"),
            vpc_id: o.get_field("vpcId"),
            workspace_security_group_id: o.get_field("workspaceSecurityGroupId"),
        }
    }
}
