/// Manages a HTTP Request Trigger within a Logic App Workflow
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("workflow-resources")
///             .build_struct(),
///     );
///     let exampleTriggerHttpRequest = trigger_http_request::create(
///         "exampleTriggerHttpRequest",
///         TriggerHttpRequestArgs::builder()
///             .logic_app_id("${exampleWorkflow.id}")
///             .name("some-http-trigger")
///             .schema(
///                 "{\n    \"type\": \"object\",\n    \"properties\": {\n        \"hello\": {\n            \"type\": \"string\"\n        }\n    }\n}",
///             )
///             .build_struct(),
///     );
///     let exampleWorkflow = workflow::create(
///         "exampleWorkflow",
///         WorkflowArgs::builder()
///             .location("${example.location}")
///             .name("workflow1")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Logic App HTTP Request Triggers can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:logicapps/triggerHttpRequest:TriggerHttpRequest request1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Logic/workflows/workflow1/triggers/request1
/// ```
///
pub mod trigger_http_request {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TriggerHttpRequestArgs {
        /// Specifies the ID of the Logic App Workflow. Changing this forces a new resource to be created.
        #[builder(into)]
        pub logic_app_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the HTTP Method which the request be using. Possible values include `DELETE`, `GET`, `PATCH`, `POST` or `PUT`.
        #[builder(into, default)]
        pub method: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the HTTP Request Trigger to be created within the Logic App Workflow. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** This name must be unique across all Triggers within the Logic App Workflow.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the Relative Path used for this Request.
        ///
        /// > **NOTE:** When `relative_path` is set a `method` must also be set.
        #[builder(into, default)]
        pub relative_path: pulumi_wasm_rust::Output<Option<String>>,
        /// A JSON Blob defining the Schema of the incoming request. This needs to be valid JSON.
        #[builder(into)]
        pub schema: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct TriggerHttpRequestResult {
        /// The URL of the Trigger within the Logic App Workflow. For use with certain resources like monitor_action_group and security_center_automation.
        pub callback_url: pulumi_wasm_rust::Output<String>,
        /// Specifies the ID of the Logic App Workflow. Changing this forces a new resource to be created.
        pub logic_app_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the HTTP Method which the request be using. Possible values include `DELETE`, `GET`, `PATCH`, `POST` or `PUT`.
        pub method: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the HTTP Request Trigger to be created within the Logic App Workflow. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** This name must be unique across all Triggers within the Logic App Workflow.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the Relative Path used for this Request.
        ///
        /// > **NOTE:** When `relative_path` is set a `method` must also be set.
        pub relative_path: pulumi_wasm_rust::Output<Option<String>>,
        /// A JSON Blob defining the Schema of the incoming request. This needs to be valid JSON.
        pub schema: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TriggerHttpRequestArgs) -> TriggerHttpRequestResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let logic_app_id_binding = args.logic_app_id.get_inner();
        let method_binding = args.method.get_inner();
        let name_binding = args.name.get_inner();
        let relative_path_binding = args.relative_path.get_inner();
        let schema_binding = args.schema.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:logicapps/triggerHttpRequest:TriggerHttpRequest".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "logicAppId".into(),
                    value: &logic_app_id_binding,
                },
                register_interface::ObjectField {
                    name: "method".into(),
                    value: &method_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "relativePath".into(),
                    value: &relative_path_binding,
                },
                register_interface::ObjectField {
                    name: "schema".into(),
                    value: &schema_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "callbackUrl".into(),
                },
                register_interface::ResultField {
                    name: "logicAppId".into(),
                },
                register_interface::ResultField {
                    name: "method".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "relativePath".into(),
                },
                register_interface::ResultField {
                    name: "schema".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TriggerHttpRequestResult {
            callback_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("callbackUrl").unwrap(),
            ),
            logic_app_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logicAppId").unwrap(),
            ),
            method: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("method").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            relative_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("relativePath").unwrap(),
            ),
            schema: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schema").unwrap(),
            ),
        }
    }
}
