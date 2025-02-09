/// Manages an App Runner VPC Ingress Connection.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:apprunner:VpcIngressConnection
///     properties:
///       name: example
///       serviceArn: ${exampleAwsApprunnerService.arn}
///       ingressVpcConfiguration:
///         vpcId: ${default.id}
///         vpcEndpointId: ${apprunner.id}
///       tags:
///         foo: bar
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import App Runner VPC Ingress Connection using the `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:apprunner/vpcIngressConnection:VpcIngressConnection example "arn:aws:apprunner:us-west-2:837424938642:vpcingressconnection/example/b379f86381d74825832c2e82080342fa"
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vpc_ingress_connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcIngressConnectionArgs {
        /// Specifications for the customer’s Amazon VPC and the related AWS PrivateLink VPC endpoint that are used to create the VPC Ingress Connection resource. See Ingress VPC Configuration below for more details.
        #[builder(into)]
        pub ingress_vpc_configuration: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::apprunner::VpcIngressConnectionIngressVpcConfiguration,
        >,
        /// A name for the VPC Ingress Connection resource. It must be unique across all the active VPC Ingress Connections in your AWS account in the AWS Region.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Amazon Resource Name (ARN) for this App Runner service that is used to create the VPC Ingress Connection resource.
        #[builder(into)]
        pub service_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct VpcIngressConnectionResult {
        /// The Amazon Resource Name (ARN) of the VPC Ingress Connection.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The domain name associated with the VPC Ingress Connection resource.
        pub domain_name: pulumi_gestalt_rust::Output<String>,
        /// Specifications for the customer’s Amazon VPC and the related AWS PrivateLink VPC endpoint that are used to create the VPC Ingress Connection resource. See Ingress VPC Configuration below for more details.
        pub ingress_vpc_configuration: pulumi_gestalt_rust::Output<
            super::super::types::apprunner::VpcIngressConnectionIngressVpcConfiguration,
        >,
        /// A name for the VPC Ingress Connection resource. It must be unique across all the active VPC Ingress Connections in your AWS account in the AWS Region.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) for this App Runner service that is used to create the VPC Ingress Connection resource.
        pub service_arn: pulumi_gestalt_rust::Output<String>,
        /// The current status of the VPC Ingress Connection.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpcIngressConnectionArgs,
    ) -> VpcIngressConnectionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let ingress_vpc_configuration_binding = args
            .ingress_vpc_configuration
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let service_arn_binding = args.service_arn.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:apprunner/vpcIngressConnection:VpcIngressConnection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ingressVpcConfiguration".into(),
                    value: ingress_vpc_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceArn".into(),
                    value: service_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VpcIngressConnectionResult {
            arn: o.get_field("arn"),
            domain_name: o.get_field("domainName"),
            ingress_vpc_configuration: o.get_field("ingressVpcConfiguration"),
            name: o.get_field("name"),
            service_arn: o.get_field("serviceArn"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
