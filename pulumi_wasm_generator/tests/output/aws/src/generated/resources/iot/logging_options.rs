/// Provides a resource to manage [default logging options](https://docs.aws.amazon.com/iot/latest/developerguide/configure-logging.html#configure-logging-console).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = logging_options::create(
///         "example",
///         LoggingOptionsArgs::builder()
///             .default_log_level("WARN")
///             .role_arn("${exampleAwsIamRole.arn}")
///             .build_struct(),
///     );
/// }
/// ```
pub mod logging_options {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LoggingOptionsArgs {
        /// The default logging level. Valid Values: `"DEBUG"`, `"INFO"`, `"ERROR"`, `"WARN"`, `"DISABLED"`.
        #[builder(into)]
        pub default_log_level: pulumi_wasm_rust::Output<String>,
        /// If `true` all logs are disabled. The default is `false`.
        #[builder(into, default)]
        pub disable_all_logs: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ARN of the role that allows IoT to write to Cloudwatch logs.
        #[builder(into)]
        pub role_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct LoggingOptionsResult {
        /// The default logging level. Valid Values: `"DEBUG"`, `"INFO"`, `"ERROR"`, `"WARN"`, `"DISABLED"`.
        pub default_log_level: pulumi_wasm_rust::Output<String>,
        /// If `true` all logs are disabled. The default is `false`.
        pub disable_all_logs: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ARN of the role that allows IoT to write to Cloudwatch logs.
        pub role_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: LoggingOptionsArgs) -> LoggingOptionsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let default_log_level_binding = args.default_log_level.get_inner();
        let disable_all_logs_binding = args.disable_all_logs.get_inner();
        let role_arn_binding = args.role_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iot/loggingOptions:LoggingOptions".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "defaultLogLevel".into(),
                    value: &default_log_level_binding,
                },
                register_interface::ObjectField {
                    name: "disableAllLogs".into(),
                    value: &disable_all_logs_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "defaultLogLevel".into(),
                },
                register_interface::ResultField {
                    name: "disableAllLogs".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LoggingOptionsResult {
            default_log_level: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultLogLevel").unwrap(),
            ),
            disable_all_logs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disableAllLogs").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
        }
    }
}