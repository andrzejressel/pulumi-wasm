/// Manages an HTTP Action within a Logic App Workflow
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod action_http {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ActionHttpArgs {
        /// Specifies the HTTP Body that should be sent to the `uri` when this HTTP Action is triggered.
        #[builder(into, default)]
        pub body: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies a Map of Key-Value Pairs that should be sent to the `uri` when this HTTP Action is triggered.
        #[builder(into, default)]
        pub headers: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the ID of the Logic App Workflow. Changing this forces a new resource to be created.
        #[builder(into)]
        pub logic_app_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the HTTP Method which should be used for this HTTP Action. Possible values include `DELETE`, `GET`, `PATCH`, `POST` and `PUT`.
        #[builder(into)]
        pub method: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the HTTP Action to be created within the Logic App Workflow. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** This name must be unique across all Actions within the Logic App Workflow.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies a Map of Key-Value Pairs that should be sent to the `uri` when this HTTP Action is triggered.
        #[builder(into, default)]
        pub queries: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the place of the HTTP Action in the Logic App Workflow. If not specified, the HTTP Action is right after the Trigger. A `run_after` block is as defined below.
        #[builder(into, default)]
        pub run_afters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::logicapps::ActionHttpRunAfter>>,
        >,
        /// Specifies the URI which will be called when this HTTP Action is triggered.
        #[builder(into)]
        pub uri: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ActionHttpResult {
        /// Specifies the HTTP Body that should be sent to the `uri` when this HTTP Action is triggered.
        pub body: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies a Map of Key-Value Pairs that should be sent to the `uri` when this HTTP Action is triggered.
        pub headers: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the ID of the Logic App Workflow. Changing this forces a new resource to be created.
        pub logic_app_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the HTTP Method which should be used for this HTTP Action. Possible values include `DELETE`, `GET`, `PATCH`, `POST` and `PUT`.
        pub method: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the HTTP Action to be created within the Logic App Workflow. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** This name must be unique across all Actions within the Logic App Workflow.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies a Map of Key-Value Pairs that should be sent to the `uri` when this HTTP Action is triggered.
        pub queries: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the place of the HTTP Action in the Logic App Workflow. If not specified, the HTTP Action is right after the Trigger. A `run_after` block is as defined below.
        pub run_afters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::logicapps::ActionHttpRunAfter>>,
        >,
        /// Specifies the URI which will be called when this HTTP Action is triggered.
        pub uri: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ActionHttpArgs,
    ) -> ActionHttpResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let body_binding = args.body.get_output(context);
        let headers_binding = args.headers.get_output(context);
        let logic_app_id_binding = args.logic_app_id.get_output(context);
        let method_binding = args.method.get_output(context);
        let name_binding = args.name.get_output(context);
        let queries_binding = args.queries.get_output(context);
        let run_afters_binding = args.run_afters.get_output(context);
        let uri_binding = args.uri.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:logicapps/actionHttp:ActionHttp".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "body".into(),
                    value: body_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "headers".into(),
                    value: headers_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logicAppId".into(),
                    value: logic_app_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "method".into(),
                    value: method_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "queries".into(),
                    value: queries_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "runAfters".into(),
                    value: run_afters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "uri".into(),
                    value: uri_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ActionHttpResult {
            body: o.get_field("body"),
            headers: o.get_field("headers"),
            logic_app_id: o.get_field("logicAppId"),
            method: o.get_field("method"),
            name: o.get_field("name"),
            queries: o.get_field("queries"),
            run_afters: o.get_field("runAfters"),
            uri: o.get_field("uri"),
        }
    }
}
