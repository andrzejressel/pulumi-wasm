/// Provides a SageMaker Endpoint resource.
///
/// ## Example Usage
///
/// Basic usage:
///
/// ```yaml
/// resources:
///   e:
///     type: aws:sagemaker:Endpoint
///     properties:
///       name: my-endpoint
///       endpointConfigName: ${ec.name}
///       tags:
///         Name: foo
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import endpoints using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:sagemaker/endpoint:Endpoint test_endpoint my-endpoint
/// ```
pub mod endpoint {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EndpointArgs {
        /// The deployment configuration for an endpoint, which contains the desired deployment strategy and rollback configurations. See Deployment Config.
        #[builder(into, default)]
        pub deployment_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::sagemaker::EndpointDeploymentConfig>,
        >,
        /// The name of the endpoint configuration to use.
        #[builder(into)]
        pub endpoint_config_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the endpoint. If omitted, the provider will assign a random, unique name.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct EndpointResult {
        /// The Amazon Resource Name (ARN) assigned by AWS to this endpoint.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The deployment configuration for an endpoint, which contains the desired deployment strategy and rollback configurations. See Deployment Config.
        pub deployment_config: pulumi_wasm_rust::Output<
            Option<super::super::types::sagemaker::EndpointDeploymentConfig>,
        >,
        /// The name of the endpoint configuration to use.
        pub endpoint_config_name: pulumi_wasm_rust::Output<String>,
        /// The name of the endpoint. If omitted, the provider will assign a random, unique name.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: EndpointArgs,
    ) -> EndpointResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let deployment_config_binding = args
            .deployment_config
            .get_output(context)
            .get_inner();
        let endpoint_config_name_binding = args
            .endpoint_config_name
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sagemaker/endpoint:Endpoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "deploymentConfig".into(),
                    value: &deployment_config_binding,
                },
                register_interface::ObjectField {
                    name: "endpointConfigName".into(),
                    value: &endpoint_config_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        EndpointResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            deployment_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deploymentConfig"),
            ),
            endpoint_config_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("endpointConfigName"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
