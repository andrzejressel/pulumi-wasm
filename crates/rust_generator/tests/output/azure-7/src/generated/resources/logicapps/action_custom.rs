/// Manages a Custom Action within a Logic App Workflow
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
///     let exampleActionCustom = action_custom::create(
///         "exampleActionCustom",
///         ActionCustomArgs::builder()
///             .body(
///                 "{\n    \"description\": \"A variable to configure the auto expiration age in days. Configured in negative number. Default is -30 (30 days old).\",\n    \"inputs\": {\n        \"variables\": [\n            {\n                \"name\": \"ExpirationAgeInDays\",\n                \"type\": \"Integer\",\n                \"value\": -30\n            }\n        ]\n    },\n    \"runAfter\": {},\n    \"type\": \"InitializeVariable\"\n}",
///             )
///             .logic_app_id("${exampleWorkflow.id}")
///             .name("example-action")
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
/// Logic App Custom Actions can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:logicapps/actionCustom:ActionCustom custom1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Logic/workflows/workflow1/actions/custom1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod action_custom {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ActionCustomArgs {
        /// Specifies the JSON Blob defining the Body of this Custom Action.
        #[builder(into)]
        pub body: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the ID of the Logic App Workflow. Changing this forces a new resource to be created.
        #[builder(into)]
        pub logic_app_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the HTTP Action to be created within the Logic App Workflow. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** This name must be unique across all Actions within the Logic App Workflow.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ActionCustomResult {
        /// Specifies the JSON Blob defining the Body of this Custom Action.
        pub body: pulumi_gestalt_rust::Output<String>,
        /// Specifies the ID of the Logic App Workflow. Changing this forces a new resource to be created.
        pub logic_app_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the HTTP Action to be created within the Logic App Workflow. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** This name must be unique across all Actions within the Logic App Workflow.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ActionCustomArgs,
    ) -> ActionCustomResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let body_binding = args.body.get_output(context);
        let logic_app_id_binding = args.logic_app_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:logicapps/actionCustom:ActionCustom".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "body".into(),
                    value: body_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logicAppId".into(),
                    value: logic_app_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ActionCustomResult {
            body: o.get_field("body"),
            logic_app_id: o.get_field("logicAppId"),
            name: o.get_field("name"),
        }
    }
}
