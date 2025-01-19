/// Represents a sharedflow attachment to a flowhook point.
///
///
/// To get more information about Flowhook, see:
///
/// * [API documentation](https://cloud.google.com/apigee/docs/reference/apis/apigee/rest/v1/organizations.environments.flowhooks#FlowHook)
/// * How-to Guides
///     * [organizations.environments.flowhooks](https://cloud.google.com/apigee/docs/reference/apis/apigee/rest/v1/organizations.environments.flowhooks#FlowHook)
///
/// ## Import
///
/// Flowhook can be imported using any of these accepted formats:
///
/// * `organizations/{{org_id}}/environments/{{environment}}/flowhooks/{{flow_hook_point}}`
///
/// * `{{org_id}}/{{environment}}/{{flow_hook_point}}`
///
/// When using the `pulumi import` command, Flowhook can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:apigee/flowhook:Flowhook default organizations/{{org_id}}/environments/{{environment}}/flowhooks/{{flow_hook_point}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apigee/flowhook:Flowhook default {{org_id}}/{{environment}}/{{flow_hook_point}}
/// ```
///
pub mod flowhook {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FlowhookArgs {
        /// Flag that specifies whether execution should continue if the flow hook throws an exception. Set to true to continue execution. Set to false to stop execution if the flow hook throws an exception. Defaults to true.
        #[builder(into, default)]
        pub continue_on_error: pulumi_wasm_rust::Output<Option<bool>>,
        /// Description of the flow hook.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The resource ID of the environment.
        #[builder(into)]
        pub environment: pulumi_wasm_rust::Output<String>,
        /// Where in the API call flow the flow hook is invoked. Must be one of PreProxyFlowHook, PostProxyFlowHook, PreTargetFlowHook, or PostTargetFlowHook.
        #[builder(into)]
        pub flow_hook_point: pulumi_wasm_rust::Output<String>,
        /// The Apigee Organization associated with the environment
        #[builder(into)]
        pub org_id: pulumi_wasm_rust::Output<String>,
        /// Id of the Sharedflow attaching to a flowhook point.
        #[builder(into)]
        pub sharedflow: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct FlowhookResult {
        /// Flag that specifies whether execution should continue if the flow hook throws an exception. Set to true to continue execution. Set to false to stop execution if the flow hook throws an exception. Defaults to true.
        pub continue_on_error: pulumi_wasm_rust::Output<Option<bool>>,
        /// Description of the flow hook.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The resource ID of the environment.
        pub environment: pulumi_wasm_rust::Output<String>,
        /// Where in the API call flow the flow hook is invoked. Must be one of PreProxyFlowHook, PostProxyFlowHook, PreTargetFlowHook, or PostTargetFlowHook.
        pub flow_hook_point: pulumi_wasm_rust::Output<String>,
        /// The Apigee Organization associated with the environment
        pub org_id: pulumi_wasm_rust::Output<String>,
        /// Id of the Sharedflow attaching to a flowhook point.
        pub sharedflow: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: FlowhookArgs) -> FlowhookResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let continue_on_error_binding = args.continue_on_error.get_inner();
        let description_binding = args.description.get_inner();
        let environment_binding = args.environment.get_inner();
        let flow_hook_point_binding = args.flow_hook_point.get_inner();
        let org_id_binding = args.org_id.get_inner();
        let sharedflow_binding = args.sharedflow.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:apigee/flowhook:Flowhook".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "continueOnError".into(),
                    value: &continue_on_error_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "environment".into(),
                    value: &environment_binding,
                },
                register_interface::ObjectField {
                    name: "flowHookPoint".into(),
                    value: &flow_hook_point_binding,
                },
                register_interface::ObjectField {
                    name: "orgId".into(),
                    value: &org_id_binding,
                },
                register_interface::ObjectField {
                    name: "sharedflow".into(),
                    value: &sharedflow_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "continueOnError".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "environment".into(),
                },
                register_interface::ResultField {
                    name: "flowHookPoint".into(),
                },
                register_interface::ResultField {
                    name: "orgId".into(),
                },
                register_interface::ResultField {
                    name: "sharedflow".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FlowhookResult {
            continue_on_error: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("continueOnError").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            environment: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("environment").unwrap(),
            ),
            flow_hook_point: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("flowHookPoint").unwrap(),
            ),
            org_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("orgId").unwrap(),
            ),
            sharedflow: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sharedflow").unwrap(),
            ),
        }
    }
}
