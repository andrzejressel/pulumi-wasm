/// Manages an App Runner Observability Configuration.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:apprunner:ObservabilityConfiguration
///     properties:
///       observabilityConfigurationName: example
///       traceConfiguration:
///         vendor: AWSXRAY
///       tags:
///         Name: example-apprunner-observability-configuration
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import App Runner Observability Configuration using the `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:apprunner/observabilityConfiguration:ObservabilityConfiguration example arn:aws:apprunner:us-east-1:1234567890:observabilityconfiguration/example/1/d75bc7ea55b71e724fe5c23452fe22a1
/// ```
pub mod observability_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ObservabilityConfigurationArgs {
        /// Name of the observability configuration.
        #[builder(into)]
        pub observability_configuration_name: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration of the tracing feature within this observability configuration. If you don't specify it, App Runner doesn't enable tracing. See Trace Configuration below for more details.
        #[builder(into, default)]
        pub trace_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::apprunner::ObservabilityConfigurationTraceConfiguration,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct ObservabilityConfigurationResult {
        /// ARN of this observability configuration.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Whether the observability configuration has the highest `observability_configuration_revision` among all configurations that share the same `observability_configuration_name`.
        pub latest: pulumi_wasm_rust::Output<bool>,
        /// Name of the observability configuration.
        pub observability_configuration_name: pulumi_wasm_rust::Output<String>,
        /// The revision of this observability configuration.
        pub observability_configuration_revision: pulumi_wasm_rust::Output<i32>,
        /// Current state of the observability configuration. An INACTIVE configuration revision has been deleted and can't be used. It is permanently removed some time after deletion.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration of the tracing feature within this observability configuration. If you don't specify it, App Runner doesn't enable tracing. See Trace Configuration below for more details.
        pub trace_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::apprunner::ObservabilityConfigurationTraceConfiguration,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ObservabilityConfigurationArgs,
    ) -> ObservabilityConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let observability_configuration_name_binding = args
            .observability_configuration_name
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let trace_configuration_binding = args.trace_configuration.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:apprunner/observabilityConfiguration:ObservabilityConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "observabilityConfigurationName".into(),
                    value: &observability_configuration_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "traceConfiguration".into(),
                    value: &trace_configuration_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "latest".into(),
                },
                register_interface::ResultField {
                    name: "observabilityConfigurationName".into(),
                },
                register_interface::ResultField {
                    name: "observabilityConfigurationRevision".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "traceConfiguration".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ObservabilityConfigurationResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            latest: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("latest").unwrap(),
            ),
            observability_configuration_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("observabilityConfigurationName").unwrap(),
            ),
            observability_configuration_revision: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("observabilityConfigurationRevision").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            trace_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("traceConfiguration").unwrap(),
            ),
        }
    }
}
