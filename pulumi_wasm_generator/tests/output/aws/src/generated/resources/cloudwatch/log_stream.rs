/// Provides a CloudWatch Log Stream resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foo = log_stream::create(
///         "foo",
///         LogStreamArgs::builder()
///             .log_group_name("${yada.name}")
///             .name("SampleLogStream1234")
///             .build_struct(),
///     );
///     let yada = log_group::create(
///         "yada",
///         LogGroupArgs::builder().name("Yada").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Cloudwatch Log Stream using the stream's `log_group_name` and `name`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudwatch/logStream:LogStream foo Yada:SampleLogStream1234
/// ```
pub mod log_stream {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LogStreamArgs {
        /// The name of the log group under which the log stream is to be created.
        #[builder(into)]
        pub log_group_name: pulumi_wasm_rust::Output<String>,
        /// The name of the log stream. Must not be longer than 512 characters and must not contain `:`
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct LogStreamResult {
        /// The Amazon Resource Name (ARN) specifying the log stream.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The name of the log group under which the log stream is to be created.
        pub log_group_name: pulumi_wasm_rust::Output<String>,
        /// The name of the log stream. Must not be longer than 512 characters and must not contain `:`
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: LogStreamArgs) -> LogStreamResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let log_group_name_binding = args.log_group_name.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudwatch/logStream:LogStream".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "logGroupName".into(),
                    value: &log_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "logGroupName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LogStreamResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            log_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logGroupName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}