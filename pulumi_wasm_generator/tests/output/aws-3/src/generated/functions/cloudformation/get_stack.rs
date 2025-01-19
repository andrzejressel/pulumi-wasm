pub mod get_stack {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetStackArgs {
        /// Name of the stack
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Map of tags associated with this stack.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetStackResult {
        /// List of capabilities
        pub capabilities: pulumi_wasm_rust::Output<Vec<String>>,
        /// Description of the stack
        pub description: pulumi_wasm_rust::Output<String>,
        /// Whether the rollback of the stack is disabled when stack creation fails
        pub disable_rollback: pulumi_wasm_rust::Output<bool>,
        /// ARN of the IAM role used to create the stack.
        pub iam_role_arn: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// List of SNS topic ARNs to publish stack related events
        pub notification_arns: pulumi_wasm_rust::Output<Vec<String>>,
        /// Map of outputs from the stack.
        pub outputs: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Map of parameters that specify input parameters for the stack.
        pub parameters: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Map of tags associated with this stack.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Structure containing the template body.
        pub template_body: pulumi_wasm_rust::Output<String>,
        /// Amount of time that can pass before the stack status becomes `CREATE_FAILED`
        pub timeout_in_minutes: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetStackArgs) -> GetStackResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:cloudformation/getStack:getStack".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "capabilities".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "disableRollback".into(),
                },
                register_interface::ResultField {
                    name: "iamRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "notificationArns".into(),
                },
                register_interface::ResultField {
                    name: "outputs".into(),
                },
                register_interface::ResultField {
                    name: "parameters".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "templateBody".into(),
                },
                register_interface::ResultField {
                    name: "timeoutInMinutes".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetStackResult {
            capabilities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("capabilities").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            disable_rollback: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disableRollback").unwrap(),
            ),
            iam_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iamRoleArn").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            notification_arns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notificationArns").unwrap(),
            ),
            outputs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outputs").unwrap(),
            ),
            parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameters").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            template_body: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("templateBody").unwrap(),
            ),
            timeout_in_minutes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeoutInMinutes").unwrap(),
            ),
        }
    }
}
