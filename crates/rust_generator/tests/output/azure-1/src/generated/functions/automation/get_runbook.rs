#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_runbook {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRunbookArgs {
        /// The name of the Automation Account the runbook belongs to.
        #[builder(into)]
        pub automation_account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Automation Runbook.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Automation exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetRunbookResult {
        pub automation_account_name: pulumi_gestalt_rust::Output<String>,
        /// The content of the Runbook.
        pub content: pulumi_gestalt_rust::Output<String>,
        /// The description of the Runbook.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Runbook exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The activity-level tracing of the Runbook.
        pub log_activity_trace_level: pulumi_gestalt_rust::Output<i32>,
        /// The Progress log option of the Runbook.
        pub log_progress: pulumi_gestalt_rust::Output<bool>,
        /// The Verbose log option of the Runbook.
        pub log_verbose: pulumi_gestalt_rust::Output<bool>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The type of Runbook.
        pub runbook_type: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags assigned to the Runbook.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetRunbookArgs,
    ) -> GetRunbookResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let automation_account_name_binding_1 = args
            .automation_account_name
            .get_output(context);
        let automation_account_name_binding = automation_account_name_binding_1
            .get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
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
            automation_account_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("automationAccountName"),
            ),
            content: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("content"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            log_activity_trace_level: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("logActivityTraceLevel"),
            ),
            log_progress: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("logProgress"),
            ),
            log_verbose: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("logVerbose"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            runbook_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("runbookType"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
