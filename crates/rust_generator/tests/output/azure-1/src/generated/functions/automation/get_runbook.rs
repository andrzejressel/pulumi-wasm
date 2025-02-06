pub mod get_runbook {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRunbookArgs {
        /// The name of the Automation Account the runbook belongs to.
        #[builder(into)]
        pub automation_account_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Automation Runbook.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Automation exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetRunbookResult {
        pub automation_account_name: pulumi_wasm_rust::Output<String>,
        /// The content of the Runbook.
        pub content: pulumi_wasm_rust::Output<String>,
        /// The description of the Runbook.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the Runbook exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The activity-level tracing of the Runbook.
        pub log_activity_trace_level: pulumi_wasm_rust::Output<i32>,
        /// The Progress log option of the Runbook.
        pub log_progress: pulumi_wasm_rust::Output<bool>,
        /// The Verbose log option of the Runbook.
        pub log_verbose: pulumi_wasm_rust::Output<bool>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The type of Runbook.
        pub runbook_type: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the Runbook.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetRunbookArgs,
    ) -> GetRunbookResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let automation_account_name_binding = args
            .automation_account_name
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:automation/getRunbook:getRunbook".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "automationAccountName".into(),
                    value: &automation_account_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetRunbookResult {
            automation_account_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("automationAccountName"),
            ),
            content: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("content"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            log_activity_trace_level: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("logActivityTraceLevel"),
            ),
            log_progress: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("logProgress"),
            ),
            log_verbose: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("logVerbose"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            runbook_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("runbookType"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
