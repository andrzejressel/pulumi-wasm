/// Manages a Custom Trigger within a Logic App Workflow
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
///     let exampleTriggerCustom = trigger_custom::create(
///         "exampleTriggerCustom",
///         TriggerCustomArgs::builder()
///             .body(
///                 "{\n  \"recurrence\": {\n    \"frequency\": \"Day\",\n    \"interval\": 1\n  },\n  \"type\": \"Recurrence\"\n}",
///             )
///             .logic_app_id("${exampleWorkflow.id}")
///             .name("example-trigger")
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
/// Logic App Custom Triggers can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:logicapps/triggerCustom:TriggerCustom custom1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Logic/workflows/workflow1/triggers/custom1
/// ```
///
pub mod trigger_custom {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TriggerCustomArgs {
        /// Specifies the JSON Blob defining the Body of this Custom Trigger.
        #[builder(into)]
        pub body: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the ID of the Logic App Workflow. Changing this forces a new resource to be created.
        #[builder(into)]
        pub logic_app_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the name of the HTTP Trigger to be created within the Logic App Workflow. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** This name must be unique across all Triggers within the Logic App Workflow.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TriggerCustomResult {
        /// Specifies the JSON Blob defining the Body of this Custom Trigger.
        pub body: pulumi_wasm_rust::Output<String>,
        /// The URL of the Trigger within the Logic App Workflow. For use with certain resources like monitor_action_group and security_center_automation.
        pub callback_url: pulumi_wasm_rust::Output<String>,
        /// Specifies the ID of the Logic App Workflow. Changing this forces a new resource to be created.
        pub logic_app_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the HTTP Trigger to be created within the Logic App Workflow. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** This name must be unique across all Triggers within the Logic App Workflow.
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: TriggerCustomArgs,
    ) -> TriggerCustomResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let body_binding = args.body.get_output(context).get_inner();
        let logic_app_id_binding = args.logic_app_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:logicapps/triggerCustom:TriggerCustom".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "body".into(),
                    value: &body_binding,
                },
                register_interface::ObjectField {
                    name: "logicAppId".into(),
                    value: &logic_app_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        TriggerCustomResult {
            body: pulumi_wasm_rust::__private::into_domain(o.extract_field("body")),
            callback_url: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("callbackUrl"),
            ),
            logic_app_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("logicAppId"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
        }
    }
}
