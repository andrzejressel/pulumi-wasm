/// Provides an AppStream image builder.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   testFleet:
///     type: aws:appstream:ImageBuilder
///     name: test_fleet
///     properties:
///       name: Name
///       description: Description of a ImageBuilder
///       displayName: Display name of a ImageBuilder
///       enableDefaultInternetAccess: false
///       imageName: AppStream-WinServer2019-10-05-2022
///       instanceType: stream.standard.large
///       vpcConfig:
///         subnetIds:
///           - ${example.id}
///       tags:
///         Name: Example Image Builder
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_appstream_image_builder` using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:appstream/imageBuilder:ImageBuilder example imageBuilderExample
/// ```
pub mod image_builder {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ImageBuilderArgs {
        /// Set of interface VPC endpoint (interface endpoint) objects. Maximum of 4. See below.
        #[builder(into, default)]
        pub access_endpoints: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::appstream::ImageBuilderAccessEndpoint>>,
        >,
        /// Version of the AppStream 2.0 agent to use for this image builder.
        #[builder(into, default)]
        pub appstream_agent_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Description to display.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Human-readable friendly name for the AppStream image builder.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block for the name of the directory and organizational unit (OU) to use to join the image builder to a Microsoft Active Directory domain. See below.
        #[builder(into, default)]
        pub domain_join_info: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appstream::ImageBuilderDomainJoinInfo>,
        >,
        /// Enables or disables default internet access for the image builder.
        #[builder(into, default)]
        pub enable_default_internet_access: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// ARN of the IAM role to apply to the image builder.
        #[builder(into, default)]
        pub iam_role_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ARN of the public, private, or shared image to use.
        #[builder(into, default)]
        pub image_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the image used to create the image builder.
        #[builder(into, default)]
        pub image_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Instance type to use when launching the image builder.
        #[builder(into)]
        pub instance_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Unique name for the image builder.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags to assign to the instance. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration block for the VPC configuration for the image builder. See below.
        #[builder(into, default)]
        pub vpc_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appstream::ImageBuilderVpcConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct ImageBuilderResult {
        /// Set of interface VPC endpoint (interface endpoint) objects. Maximum of 4. See below.
        pub access_endpoints: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::appstream::ImageBuilderAccessEndpoint>>,
        >,
        /// Version of the AppStream 2.0 agent to use for this image builder.
        pub appstream_agent_version: pulumi_gestalt_rust::Output<String>,
        /// ARN of the appstream image builder.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Date and time, in UTC and extended RFC 3339 format, when the image builder was created.
        pub created_time: pulumi_gestalt_rust::Output<String>,
        /// Description to display.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Human-readable friendly name for the AppStream image builder.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// Configuration block for the name of the directory and organizational unit (OU) to use to join the image builder to a Microsoft Active Directory domain. See below.
        pub domain_join_info: pulumi_gestalt_rust::Output<
            super::super::types::appstream::ImageBuilderDomainJoinInfo,
        >,
        /// Enables or disables default internet access for the image builder.
        pub enable_default_internet_access: pulumi_gestalt_rust::Output<bool>,
        /// ARN of the IAM role to apply to the image builder.
        pub iam_role_arn: pulumi_gestalt_rust::Output<String>,
        /// ARN of the public, private, or shared image to use.
        pub image_arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the image used to create the image builder.
        pub image_name: pulumi_gestalt_rust::Output<String>,
        /// Instance type to use when launching the image builder.
        pub instance_type: pulumi_gestalt_rust::Output<String>,
        /// Unique name for the image builder.
        ///
        /// The following arguments are optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        /// State of the image builder. For valid values, refer to the [AWS documentation](https://docs.aws.amazon.com/appstream2/latest/APIReference/API_ImageBuilder.html#AppStream2-Type-ImageBuilder-State).
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Map of tags to assign to the instance. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration block for the VPC configuration for the image builder. See below.
        pub vpc_config: pulumi_gestalt_rust::Output<
            super::super::types::appstream::ImageBuilderVpcConfig,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ImageBuilderArgs,
    ) -> ImageBuilderResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let access_endpoints_binding = args
            .access_endpoints
            .get_output(context)
            .get_inner();
        let appstream_agent_version_binding = args
            .appstream_agent_version
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let domain_join_info_binding = args
            .domain_join_info
            .get_output(context)
            .get_inner();
        let enable_default_internet_access_binding = args
            .enable_default_internet_access
            .get_output(context)
            .get_inner();
        let iam_role_arn_binding = args.iam_role_arn.get_output(context).get_inner();
        let image_arn_binding = args.image_arn.get_output(context).get_inner();
        let image_name_binding = args.image_name.get_output(context).get_inner();
        let instance_type_binding = args.instance_type.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let vpc_config_binding = args.vpc_config.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:appstream/imageBuilder:ImageBuilder".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessEndpoints".into(),
                    value: &access_endpoints_binding,
                },
                register_interface::ObjectField {
                    name: "appstreamAgentVersion".into(),
                    value: &appstream_agent_version_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "domainJoinInfo".into(),
                    value: &domain_join_info_binding,
                },
                register_interface::ObjectField {
                    name: "enableDefaultInternetAccess".into(),
                    value: &enable_default_internet_access_binding,
                },
                register_interface::ObjectField {
                    name: "iamRoleArn".into(),
                    value: &iam_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "imageArn".into(),
                    value: &image_arn_binding,
                },
                register_interface::ObjectField {
                    name: "imageName".into(),
                    value: &image_name_binding,
                },
                register_interface::ObjectField {
                    name: "instanceType".into(),
                    value: &instance_type_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "vpcConfig".into(),
                    value: &vpc_config_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ImageBuilderResult {
            access_endpoints: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accessEndpoints"),
            ),
            appstream_agent_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("appstreamAgentVersion"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            created_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createdTime"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            domain_join_info: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("domainJoinInfo"),
            ),
            enable_default_internet_access: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableDefaultInternetAccess"),
            ),
            iam_role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("iamRoleArn"),
            ),
            image_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("imageArn"),
            ),
            image_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("imageName"),
            ),
            instance_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceType"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            vpc_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vpcConfig"),
            ),
        }
    }
}
