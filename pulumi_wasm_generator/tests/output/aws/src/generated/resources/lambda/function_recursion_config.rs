/// Resource for managing an AWS Lambda Function Recursion Config.
///
/// > Destruction of this resource will return the `recursive_loop` configuration back to the default value of `Terminate`.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = function_recursion_config::create(
///         "example",
///         FunctionRecursionConfigArgs::builder()
///             .function_name("SomeFunction")
///             .recursive_loop("Allow")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AWS Lambda Function Recursion Config using the `function_name`. For example:
///
/// ```sh
/// $ pulumi import aws:lambda/functionRecursionConfig:FunctionRecursionConfig example SomeFunction
/// ```
pub mod function_recursion_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FunctionRecursionConfigArgs {
        /// Lambda function name.
        #[builder(into)]
        pub function_name: pulumi_wasm_rust::Output<String>,
        /// Lambda function recursion configuration. Valid values are `Allow` or `Terminate`.
        #[builder(into)]
        pub recursive_loop: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct FunctionRecursionConfigResult {
        /// Lambda function name.
        pub function_name: pulumi_wasm_rust::Output<String>,
        /// Lambda function recursion configuration. Valid values are `Allow` or `Terminate`.
        pub recursive_loop: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: FunctionRecursionConfigArgs,
    ) -> FunctionRecursionConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let function_name_binding = args.function_name.get_inner();
        let recursive_loop_binding = args.recursive_loop.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lambda/functionRecursionConfig:FunctionRecursionConfig".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "functionName".into(),
                    value: &function_name_binding,
                },
                register_interface::ObjectField {
                    name: "recursiveLoop".into(),
                    value: &recursive_loop_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "functionName".into(),
                },
                register_interface::ResultField {
                    name: "recursiveLoop".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FunctionRecursionConfigResult {
            function_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("functionName").unwrap(),
            ),
            recursive_loop: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("recursiveLoop").unwrap(),
            ),
        }
    }
}
