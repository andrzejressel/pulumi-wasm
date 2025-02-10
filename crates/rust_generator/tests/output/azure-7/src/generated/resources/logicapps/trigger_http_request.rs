/// Manages a HTTP Request Trigger within a Logic App Workflow
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod trigger_http_request {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TriggerHttpRequestArgs {
        /// Specifies the ID of the Logic App Workflow. Changing this forces a new resource to be created.
        #[builder(into)]
        pub logic_app_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the HTTP Method which the request be using. Possible values include `DELETE`, `GET`, `PATCH`, `POST` or `PUT`.
        #[builder(into, default)]
        pub method: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the HTTP Request Trigger to be created within the Logic App Workflow. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** This name must be unique across all Triggers within the Logic App Workflow.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the Relative Path used for this Request.
        ///
        /// > **NOTE:** When `relative_path` is set a `method` must also be set.
        #[builder(into, default)]
        pub relative_path: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A JSON Blob defining the Schema of the incoming request. This needs to be valid JSON.
        #[builder(into)]
        pub schema: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TriggerHttpRequestResult {
        /// The URL of the Trigger within the Logic App Workflow. For use with certain resources like monitor_action_group and security_center_automation.
        pub callback_url: pulumi_gestalt_rust::Output<String>,
        /// Specifies the ID of the Logic App Workflow. Changing this forces a new resource to be created.
        pub logic_app_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the HTTP Method which the request be using. Possible values include `DELETE`, `GET`, `PATCH`, `POST` or `PUT`.
        pub method: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the HTTP Request Trigger to be created within the Logic App Workflow. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** This name must be unique across all Triggers within the Logic App Workflow.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Relative Path used for this Request.
        ///
        /// > **NOTE:** When `relative_path` is set a `method` must also be set.
        pub relative_path: pulumi_gestalt_rust::Output<Option<String>>,
        /// A JSON Blob defining the Schema of the incoming request. This needs to be valid JSON.
        pub schema: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TriggerHttpRequestArgs,
    ) -> TriggerHttpRequestResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let logic_app_id_binding = args.logic_app_id.get_output(context);
        let method_binding = args.method.get_output(context);
        let name_binding = args.name.get_output(context);
        let relative_path_binding = args.relative_path.get_output(context);
        let schema_binding = args.schema.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:logicapps/triggerHttpRequest:TriggerHttpRequest".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
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
                    name: "relativePath".into(),
                    value: relative_path_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "schema".into(),
                    value: schema_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TriggerHttpRequestResult {
            callback_url: o.get_field("callbackUrl"),
            logic_app_id: o.get_field("logicAppId"),
            method: o.get_field("method"),
            name: o.get_field("name"),
            relative_path: o.get_field("relativePath"),
            schema: o.get_field("schema"),
        }
    }
}
