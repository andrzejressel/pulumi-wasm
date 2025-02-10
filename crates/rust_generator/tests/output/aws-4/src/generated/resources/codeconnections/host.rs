/// Resource for managing an AWS CodeConnections Host.
///
/// > **NOTE:** The `aws.codeconnections.Host` resource is created in the state `PENDING`. Authentication with the host provider must be completed in the AWS Console. For more information visit [Set up a pending host](https://docs.aws.amazon.com/dtconsole/latest/userguide/connections-host-setup.html).
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = host::create(
///         "example",
///         HostArgs::builder()
///             .name("example-host")
///             .provider_endpoint("https://example.com")
///             .provider_type("GitHubEnterpriseServer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CodeConnections Host using the ARN. For example:
///
/// ```sh
/// $ pulumi import aws:codeconnections/host:Host example-host arn:aws:codeconnections:us-west-1:0123456789:host/79d4d357-a2ee-41e4-b350-2fe39ae59448
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod host {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HostArgs {
        /// The name of the host to be created. The name must be unique in the calling AWS account.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The endpoint of the infrastructure to be represented by the host after it is created.
        #[builder(into)]
        pub provider_endpoint: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the external provider where your third-party code repository is configured.
        #[builder(into)]
        pub provider_type: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::codeconnections::HostTimeouts>,
        >,
        /// The VPC configuration to be provisioned for the host. A VPC must be configured, and the infrastructure to be represented by the host must already be connected to the VPC.
        #[builder(into, default)]
        pub vpc_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::codeconnections::HostVpcConfiguration>,
        >,
    }
    #[allow(dead_code)]
    pub struct HostResult {
        /// The CodeConnections Host ARN.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the host to be created. The name must be unique in the calling AWS account.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The endpoint of the infrastructure to be represented by the host after it is created.
        pub provider_endpoint: pulumi_gestalt_rust::Output<String>,
        /// The name of the external provider where your third-party code repository is configured.
        pub provider_type: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::codeconnections::HostTimeouts>,
        >,
        /// The VPC configuration to be provisioned for the host. A VPC must be configured, and the infrastructure to be represented by the host must already be connected to the VPC.
        pub vpc_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::codeconnections::HostVpcConfiguration>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: HostArgs,
    ) -> HostResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let provider_endpoint_binding = args.provider_endpoint.get_output(context);
        let provider_type_binding = args.provider_type.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let vpc_configuration_binding = args.vpc_configuration.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:codeconnections/host:Host".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "providerEndpoint".into(),
                    value: provider_endpoint_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "providerType".into(),
                    value: provider_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: timeouts_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcConfiguration".into(),
                    value: vpc_configuration_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        HostResult {
            arn: o.get_field("arn"),
            name: o.get_field("name"),
            provider_endpoint: o.get_field("providerEndpoint"),
            provider_type: o.get_field("providerType"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            timeouts: o.get_field("timeouts"),
            vpc_configuration: o.get_field("vpcConfiguration"),
        }
    }
}
