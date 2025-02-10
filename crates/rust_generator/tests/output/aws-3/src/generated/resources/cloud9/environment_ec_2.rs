/// Provides a Cloud9 EC2 Development Environment.
///
/// ## Example Usage
///
/// Basic usage:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = environment_ec_2::create(
///         "example",
///         EnvironmentEc2Args::builder()
///             .image_id("amazonlinux-2023-x86_64")
///             .instance_type("t2.micro")
///             .name("example-env")
///             .build_struct(),
///     );
/// }
/// ```
///
/// Get the URL of the Cloud9 environment after creation:
///
/// ```yaml
/// resources:
///   example:
///     type: aws:cloud9:EnvironmentEC2
///     properties:
///       instanceType: t2.micro
/// variables:
///   cloud9Instance:
///     fn::invoke:
///       function: aws:ec2:getInstance
///       arguments:
///         filters:
///           - name: tag:aws:cloud9:environment
///             values:
///               - ${example.id}
/// outputs:
///   cloud9Url: https://${region}.console.aws.amazon.com/cloud9/ide/${example.id}
/// ```
///
/// Allocate a static IP to the Cloud9 environment:
///
/// ```yaml
/// resources:
///   example:
///     type: aws:cloud9:EnvironmentEC2
///     properties:
///       instanceType: t2.micro
///   cloud9Eip:
///     type: aws:ec2:Eip
///     name: cloud9_eip
///     properties:
///       instance: ${cloud9Instance.id}
///       domain: vpc
/// variables:
///   cloud9Instance:
///     fn::invoke:
///       function: aws:ec2:getInstance
///       arguments:
///         filters:
///           - name: tag:aws:cloud9:environment
///             values:
///               - ${example.id}
/// outputs:
///   cloud9PublicIp: ${cloud9Eip.publicIp}
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod environment_ec_2 {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnvironmentEC2Args {
        /// The number of minutes until the running instance is shut down after the environment has last been used.
        #[builder(into, default)]
        pub automatic_stop_time_minutes: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The connection type used for connecting to an Amazon EC2 environment. Valid values are `CONNECT_SSH` and `CONNECT_SSM`. For more information please refer [AWS documentation for Cloud9](https://docs.aws.amazon.com/cloud9/latest/user-guide/ec2-ssm.html).
        #[builder(into, default)]
        pub connection_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The description of the environment.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The identifier for the Amazon Machine Image (AMI) that's used to create the EC2 instance. Valid values are
        /// * `amazonlinux-2-x86_64`
        /// * `amazonlinux-2023-x86_64`
        /// * `ubuntu-18.04-x86_64`
        /// * `ubuntu-22.04-x86_64`
        /// * `resolve:ssm:/aws/service/cloud9/amis/amazonlinux-2-x86_64`
        /// * `resolve:ssm:/aws/service/cloud9/amis/amazonlinux-2023-x86_64`
        /// * `resolve:ssm:/aws/service/cloud9/amis/ubuntu-18.04-x86_64`
        /// * `resolve:ssm:/aws/service/cloud9/amis/ubuntu-22.04-x86_64`
        #[builder(into)]
        pub image_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of instance to connect to the environment, e.g., `t2.micro`.
        #[builder(into)]
        pub instance_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the environment.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ARN of the environment owner. This can be ARN of any AWS IAM principal. Defaults to the environment's creator.
        #[builder(into, default)]
        pub owner_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the subnet in Amazon VPC that AWS Cloud9 will use to communicate with the Amazon EC2 instance.
        #[builder(into, default)]
        pub subnet_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct EnvironmentEC2Result {
        /// The ARN of the environment.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The number of minutes until the running instance is shut down after the environment has last been used.
        pub automatic_stop_time_minutes: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The connection type used for connecting to an Amazon EC2 environment. Valid values are `CONNECT_SSH` and `CONNECT_SSM`. For more information please refer [AWS documentation for Cloud9](https://docs.aws.amazon.com/cloud9/latest/user-guide/ec2-ssm.html).
        pub connection_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The description of the environment.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The identifier for the Amazon Machine Image (AMI) that's used to create the EC2 instance. Valid values are
        /// * `amazonlinux-2-x86_64`
        /// * `amazonlinux-2023-x86_64`
        /// * `ubuntu-18.04-x86_64`
        /// * `ubuntu-22.04-x86_64`
        /// * `resolve:ssm:/aws/service/cloud9/amis/amazonlinux-2-x86_64`
        /// * `resolve:ssm:/aws/service/cloud9/amis/amazonlinux-2023-x86_64`
        /// * `resolve:ssm:/aws/service/cloud9/amis/ubuntu-18.04-x86_64`
        /// * `resolve:ssm:/aws/service/cloud9/amis/ubuntu-22.04-x86_64`
        pub image_id: pulumi_gestalt_rust::Output<String>,
        /// The type of instance to connect to the environment, e.g., `t2.micro`.
        pub instance_type: pulumi_gestalt_rust::Output<String>,
        /// The name of the environment.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the environment owner. This can be ARN of any AWS IAM principal. Defaults to the environment's creator.
        pub owner_arn: pulumi_gestalt_rust::Output<String>,
        /// The ID of the subnet in Amazon VPC that AWS Cloud9 will use to communicate with the Amazon EC2 instance.
        pub subnet_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The type of the environment (e.g., `ssh` or `ec2`).
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EnvironmentEC2Args,
    ) -> EnvironmentEC2Result {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let automatic_stop_time_minutes_binding = args
            .automatic_stop_time_minutes
            .get_output(context);
        let connection_type_binding = args.connection_type.get_output(context);
        let description_binding = args.description.get_output(context);
        let image_id_binding = args.image_id.get_output(context);
        let instance_type_binding = args.instance_type.get_output(context);
        let name_binding = args.name.get_output(context);
        let owner_arn_binding = args.owner_arn.get_output(context);
        let subnet_id_binding = args.subnet_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cloud9/environmentEC2:EnvironmentEC2".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "automaticStopTimeMinutes".into(),
                    value: automatic_stop_time_minutes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionType".into(),
                    value: connection_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "imageId".into(),
                    value: image_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceType".into(),
                    value: instance_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ownerArn".into(),
                    value: owner_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetId".into(),
                    value: subnet_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        EnvironmentEC2Result {
            arn: o.get_field("arn"),
            automatic_stop_time_minutes: o.get_field("automaticStopTimeMinutes"),
            connection_type: o.get_field("connectionType"),
            description: o.get_field("description"),
            image_id: o.get_field("imageId"),
            instance_type: o.get_field("instanceType"),
            name: o.get_field("name"),
            owner_arn: o.get_field("ownerArn"),
            subnet_id: o.get_field("subnetId"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            type_: o.get_field("type"),
        }
    }
}
