/// Provides a CodeStar Host.
///
/// > **NOTE:** The `aws.codestarconnections.Host` resource is created in the state `PENDING`. Authentication with the host provider must be completed in the AWS Console. For more information visit [Set up a pending host](https://docs.aws.amazon.com/dtconsole/latest/userguide/connections-host-setup.html).
///
/// ## Example Usage
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
/// Using `pulumi import`, import CodeStar Host using the ARN. For example:
///
/// ```sh
/// $ pulumi import aws:codestarconnections/host:Host example-host arn:aws:codestar-connections:us-west-1:0123456789:host/79d4d357-a2ee-41e4-b350-2fe39ae59448
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
        /// The VPC configuration to be provisioned for the host. A VPC must be configured, and the infrastructure to be represented by the host must already be connected to the VPC.
        #[builder(into, default)]
        pub vpc_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::codestarconnections::HostVpcConfiguration>,
        >,
    }
    #[allow(dead_code)]
    pub struct HostResult {
        /// The CodeStar Host ARN.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the host to be created. The name must be unique in the calling AWS account.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The endpoint of the infrastructure to be represented by the host after it is created.
        pub provider_endpoint: pulumi_gestalt_rust::Output<String>,
        /// The name of the external provider where your third-party code repository is configured.
        pub provider_type: pulumi_gestalt_rust::Output<String>,
        /// The CodeStar Host status. Possible values are `PENDING`, `AVAILABLE`, `VPC_CONFIG_DELETING`, `VPC_CONFIG_INITIALIZING`, and `VPC_CONFIG_FAILED_INITIALIZATION`.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// The VPC configuration to be provisioned for the host. A VPC must be configured, and the infrastructure to be represented by the host must already be connected to the VPC.
        pub vpc_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::codestarconnections::HostVpcConfiguration>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: HostArgs,
    ) -> HostResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let provider_endpoint_binding = args
            .provider_endpoint
            .get_output(context)
            .get_inner();
        let provider_type_binding = args.provider_type.get_output(context).get_inner();
        let vpc_configuration_binding = args
            .vpc_configuration
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:codestarconnections/host:Host".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "providerEndpoint".into(),
                    value: &provider_endpoint_binding,
                },
                register_interface::ObjectField {
                    name: "providerType".into(),
                    value: &provider_type_binding,
                },
                register_interface::ObjectField {
                    name: "vpcConfiguration".into(),
                    value: &vpc_configuration_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        HostResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            provider_endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("providerEndpoint"),
            ),
            provider_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("providerType"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            vpc_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vpcConfiguration"),
            ),
        }
    }
}
