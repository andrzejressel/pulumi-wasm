/// Manages an HTTP Action within a Logic App Workflow
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
///     let exampleActionHttp = action_http::create(
///         "exampleActionHttp",
///         ActionHttpArgs::builder()
///             .logic_app_id("${exampleWorkflow.id}")
///             .method("GET")
///             .name("webhook")
///             .uri("http://example.com/some-webhook")
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
/// Logic App HTTP Actions can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:logicapps/actionHttp:ActionHttp webhook1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Logic/workflows/workflow1/actions/webhook1
/// ```
///
pub mod action_http {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ActionHttpArgs {
        /// Specifies the HTTP Body that should be sent to the `uri` when this HTTP Action is triggered.
        #[builder(into, default)]
        pub body: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies a Map of Key-Value Pairs that should be sent to the `uri` when this HTTP Action is triggered.
        #[builder(into, default)]
        pub headers: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the ID of the Logic App Workflow. Changing this forces a new resource to be created.
        #[builder(into)]
        pub logic_app_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the HTTP Method which should be used for this HTTP Action. Possible values include `DELETE`, `GET`, `PATCH`, `POST` and `PUT`.
        #[builder(into)]
        pub method: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the HTTP Action to be created within the Logic App Workflow. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** This name must be unique across all Actions within the Logic App Workflow.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies a Map of Key-Value Pairs that should be sent to the `uri` when this HTTP Action is triggered.
        #[builder(into, default)]
        pub queries: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the place of the HTTP Action in the Logic App Workflow. If not specified, the HTTP Action is right after the Trigger. A `run_after` block is as defined below.
        #[builder(into, default)]
        pub run_afters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::logicapps::ActionHttpRunAfter>>,
        >,
        /// Specifies the URI which will be called when this HTTP Action is triggered.
        #[builder(into)]
        pub uri: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ActionHttpResult {
        /// Specifies the HTTP Body that should be sent to the `uri` when this HTTP Action is triggered.
        pub body: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies a Map of Key-Value Pairs that should be sent to the `uri` when this HTTP Action is triggered.
        pub headers: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the ID of the Logic App Workflow. Changing this forces a new resource to be created.
        pub logic_app_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the HTTP Method which should be used for this HTTP Action. Possible values include `DELETE`, `GET`, `PATCH`, `POST` and `PUT`.
        pub method: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the HTTP Action to be created within the Logic App Workflow. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** This name must be unique across all Actions within the Logic App Workflow.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies a Map of Key-Value Pairs that should be sent to the `uri` when this HTTP Action is triggered.
        pub queries: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the place of the HTTP Action in the Logic App Workflow. If not specified, the HTTP Action is right after the Trigger. A `run_after` block is as defined below.
        pub run_afters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::logicapps::ActionHttpRunAfter>>,
        >,
        /// Specifies the URI which will be called when this HTTP Action is triggered.
        pub uri: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ActionHttpArgs) -> ActionHttpResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let body_binding = args.body.get_inner();
        let headers_binding = args.headers.get_inner();
        let logic_app_id_binding = args.logic_app_id.get_inner();
        let method_binding = args.method.get_inner();
        let name_binding = args.name.get_inner();
        let queries_binding = args.queries.get_inner();
        let run_afters_binding = args.run_afters.get_inner();
        let uri_binding = args.uri.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:logicapps/actionHttp:ActionHttp".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "body".into(),
                    value: &body_binding,
                },
                register_interface::ObjectField {
                    name: "headers".into(),
                    value: &headers_binding,
                },
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
                    name: "queries".into(),
                    value: &queries_binding,
                },
                register_interface::ObjectField {
                    name: "runAfters".into(),
                    value: &run_afters_binding,
                },
                register_interface::ObjectField {
                    name: "uri".into(),
                    value: &uri_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "body".into(),
                },
                register_interface::ResultField {
                    name: "headers".into(),
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
                    name: "queries".into(),
                },
                register_interface::ResultField {
                    name: "runAfters".into(),
                },
                register_interface::ResultField {
                    name: "uri".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ActionHttpResult {
            body: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("body").unwrap(),
            ),
            headers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("headers").unwrap(),
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
            queries: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("queries").unwrap(),
            ),
            run_afters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("runAfters").unwrap(),
            ),
            uri: pulumi_wasm_rust::__private::into_domain(hashmap.remove("uri").unwrap()),
        }
    }
}
