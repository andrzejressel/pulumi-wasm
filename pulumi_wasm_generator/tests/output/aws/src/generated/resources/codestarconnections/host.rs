/// Provides a CodeStar Host.
///
/// > **NOTE:** The `aws.codestarconnections.Host` resource is created in the state `PENDING`. Authentication with the host provider must be completed in the AWS Console. For more information visit [Set up a pending host](https://docs.aws.amazon.com/dtconsole/latest/userguide/connections-host-setup.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod host {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HostArgs {
        /// The name of the host to be created. The name must be unique in the calling AWS account.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The endpoint of the infrastructure to be represented by the host after it is created.
        #[builder(into)]
        pub provider_endpoint: pulumi_wasm_rust::Output<String>,
        /// The name of the external provider where your third-party code repository is configured.
        #[builder(into)]
        pub provider_type: pulumi_wasm_rust::Output<String>,
        /// The VPC configuration to be provisioned for the host. A VPC must be configured, and the infrastructure to be represented by the host must already be connected to the VPC.
        #[builder(into, default)]
        pub vpc_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::codestarconnections::HostVpcConfiguration>,
        >,
    }
    #[allow(dead_code)]
    pub struct HostResult {
        /// The CodeStar Host ARN.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The name of the host to be created. The name must be unique in the calling AWS account.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The endpoint of the infrastructure to be represented by the host after it is created.
        pub provider_endpoint: pulumi_wasm_rust::Output<String>,
        /// The name of the external provider where your third-party code repository is configured.
        pub provider_type: pulumi_wasm_rust::Output<String>,
        /// The CodeStar Host status. Possible values are `PENDING`, `AVAILABLE`, `VPC_CONFIG_DELETING`, `VPC_CONFIG_INITIALIZING`, and `VPC_CONFIG_FAILED_INITIALIZATION`.
        pub status: pulumi_wasm_rust::Output<String>,
        /// The VPC configuration to be provisioned for the host. A VPC must be configured, and the infrastructure to be represented by the host must already be connected to the VPC.
        pub vpc_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::codestarconnections::HostVpcConfiguration>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: HostArgs) -> HostResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let provider_endpoint_binding = args.provider_endpoint.get_inner();
        let provider_type_binding = args.provider_type.get_inner();
        let vpc_configuration_binding = args.vpc_configuration.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:codestarconnections/host:Host".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "providerEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "providerType".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
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
        HostResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            provider_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("providerEndpoint").unwrap(),
            ),
            provider_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("providerType").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            vpc_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcConfiguration").unwrap(),
            ),
        }
    }
}