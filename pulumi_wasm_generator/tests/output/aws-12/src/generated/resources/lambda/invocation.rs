/// Use this resource to invoke a lambda function. The lambda function is invoked with the [RequestResponse](https://docs.aws.amazon.com/lambda/latest/dg/API_Invoke.html#API_Invoke_RequestSyntax) invocation type.
///
/// > **NOTE:** By default this resource _only_ invokes the function when the arguments call for a create or replace. In other words, after an initial invocation on _apply_, if the arguments do not change, a subsequent _apply_ does not invoke the function again. To dynamically invoke the function, see the `triggers` example below. To always invoke a function on each _apply_, see the `aws.lambda.Invocation` data source. To invoke the lambda function when the Pulumi resource is updated and deleted, see the CRUD Lifecycle Scope example below.
///
/// > **NOTE:** If you get a `KMSAccessDeniedException: Lambda was unable to decrypt the environment variables because KMS access was denied` error when invoking an `aws.lambda.Function` with environment variables, the IAM role associated with the function may have been deleted and recreated _after_ the function was created. You can fix the problem two ways: 1) updating the function's role to another role and then updating it back again to the recreated role, or 2) by using Pulumi to `taint` the function and `apply` your configuration again to recreate the function. (When you create a function, Lambda grants permissions on the KMS key to the function's IAM role. If the IAM role is recreated, the grant is no longer valid. Changing the function's role or recreating the function causes Lambda to update the grant.)
///
/// ## Example Usage
///
/// ### Dynamic Invocation Example Using Triggers
///
/// ```yaml
/// resources:
///   example:
///     type: aws:lambda:Invocation
///     properties:
///       functionName: ${lambdaFunctionTest.functionName}
///       triggers:
///         redeployment:
///           fn::invoke:
///             function: std:sha1
///             arguments:
///               input:
///                 fn::toJSON:
///                   - ${exampleAwsLambdaFunction.environment}
///             return: result
///       input:
///         fn::toJSON:
///           key1: value1
///           key2: value2
/// ```
///
/// ### CRUD Lifecycle Scope
///
/// ```yaml
/// resources:
///   example:
///     type: aws:lambda:Invocation
///     properties:
///       functionName: ${lambdaFunctionTest.functionName}
///       input:
///         fn::toJSON:
///           key1: value1
///           key2: value2
///       lifecycleScope: CRUD
/// ```
///
/// > **NOTE:** `lifecycle_scope = "CRUD"` will inject a key `tf` in the input event to pass lifecycle information! This allows the lambda function to handle different lifecycle transitions uniquely.  If you need to use a key `tf` in your own input JSON, the default key name can be overridden with the `pulumi_key` argument.
///
/// The key `tf` gets added with subkeys:
///
/// * `action` - Action Pulumi performs on the resource. Values are `create`, `update`, or `delete`.
/// * `prev_input` - Input JSON payload from the previous invocation. This can be used to handle update and delete events.
///
/// When the resource from the example above is created, the Lambda will get following JSON payload:
///
/// ```json
/// {
///   "key1": "value1",
///   "key2": "value2",
///   "tf": {
///     "action": "create",
///     "prev_input": null
///   }
/// }
/// ```
///
/// If the input value of `key1` changes to "valueB", then the lambda will be invoked again with the following JSON payload:
///
/// ```json
/// {
///   "key1": "valueB",
///   "key2": "value2",
///   "tf": {
///     "action": "update",
///     "prev_input": {
///       "key1": "value1",
///       "key2": "value2"
///     }
///   }
/// }
/// ```
///
/// When the invocation resource is removed, the final invocation will have the following JSON payload:
///
/// ```json
/// {
///   "key1": "valueB",
///   "key2": "value2",
///   "tf": {
///     "action": "delete",
///     "prev_input": {
///       "key1": "valueB",
///       "key2": "value2"
///     }
///   }
/// }
/// ```
pub mod invocation {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InvocationArgs {
        /// Name of the lambda function.
        #[builder(into)]
        pub function_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// JSON payload to the lambda function.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub input: pulumi_wasm_rust::InputOrOutput<String>,
        /// Lifecycle scope of the resource to manage. Valid values are `CREATE_ONLY` and `CRUD`. Defaults to `CREATE_ONLY`. `CREATE_ONLY` will invoke the function only on creation or replacement. `CRUD` will invoke the function on each lifecycle event, and augment the input JSON payload with additional lifecycle information.
        #[builder(into, default)]
        pub lifecycle_scope: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Qualifier (i.e., version) of the lambda function. Defaults to `$LATEST`.
        #[builder(into, default)]
        pub qualifier: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub terraform_key: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Map of arbitrary keys and values that, when changed, will trigger a re-invocation.
        #[builder(into, default)]
        pub triggers: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct InvocationResult {
        /// Name of the lambda function.
        pub function_name: pulumi_wasm_rust::Output<String>,
        /// JSON payload to the lambda function.
        ///
        /// The following arguments are optional:
        pub input: pulumi_wasm_rust::Output<String>,
        /// Lifecycle scope of the resource to manage. Valid values are `CREATE_ONLY` and `CRUD`. Defaults to `CREATE_ONLY`. `CREATE_ONLY` will invoke the function only on creation or replacement. `CRUD` will invoke the function on each lifecycle event, and augment the input JSON payload with additional lifecycle information.
        pub lifecycle_scope: pulumi_wasm_rust::Output<Option<String>>,
        /// Qualifier (i.e., version) of the lambda function. Defaults to `$LATEST`.
        pub qualifier: pulumi_wasm_rust::Output<Option<String>>,
        /// String result of the lambda function invocation.
        pub result: pulumi_wasm_rust::Output<String>,
        pub terraform_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of arbitrary keys and values that, when changed, will trigger a re-invocation.
        pub triggers: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: InvocationArgs,
    ) -> InvocationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let function_name_binding = args.function_name.get_output(context).get_inner();
        let input_binding = args.input.get_output(context).get_inner();
        let lifecycle_scope_binding = args
            .lifecycle_scope
            .get_output(context)
            .get_inner();
        let qualifier_binding = args.qualifier.get_output(context).get_inner();
        let terraform_key_binding = args.terraform_key.get_output(context).get_inner();
        let triggers_binding = args.triggers.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lambda/invocation:Invocation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "functionName".into(),
                    value: &function_name_binding,
                },
                register_interface::ObjectField {
                    name: "input".into(),
                    value: &input_binding,
                },
                register_interface::ObjectField {
                    name: "lifecycleScope".into(),
                    value: &lifecycle_scope_binding,
                },
                register_interface::ObjectField {
                    name: "qualifier".into(),
                    value: &qualifier_binding,
                },
                register_interface::ObjectField {
                    name: "terraformKey".into(),
                    value: &terraform_key_binding,
                },
                register_interface::ObjectField {
                    name: "triggers".into(),
                    value: &triggers_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "functionName".into(),
                },
                register_interface::ResultField {
                    name: "input".into(),
                },
                register_interface::ResultField {
                    name: "lifecycleScope".into(),
                },
                register_interface::ResultField {
                    name: "qualifier".into(),
                },
                register_interface::ResultField {
                    name: "result".into(),
                },
                register_interface::ResultField {
                    name: "terraformKey".into(),
                },
                register_interface::ResultField {
                    name: "triggers".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        InvocationResult {
            function_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("functionName").unwrap(),
            ),
            input: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("input").unwrap(),
            ),
            lifecycle_scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lifecycleScope").unwrap(),
            ),
            qualifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("qualifier").unwrap(),
            ),
            result: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("result").unwrap(),
            ),
            terraform_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("terraformKey").unwrap(),
            ),
            triggers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("triggers").unwrap(),
            ),
        }
    }
}
