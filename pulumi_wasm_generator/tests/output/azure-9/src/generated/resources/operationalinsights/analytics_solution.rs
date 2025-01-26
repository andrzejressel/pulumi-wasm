/// Manages a Log Analytics (formally Operational Insights) Solution.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: k8s-log-analytics-test
///       location: West Europe
///   workspace:
///     type: random:RandomId
///     properties:
///       keepers:
///         group_name: ${example.name}
///       byteLength: 8
///   exampleAnalyticsWorkspace:
///     type: azure:operationalinsights:AnalyticsWorkspace
///     name: example
///     properties:
///       name: k8s-workspace-${workspace.hex}
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku: PerGB2018
///   exampleAnalyticsSolution:
///     type: azure:operationalinsights:AnalyticsSolution
///     name: example
///     properties:
///       solutionName: ContainerInsights
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       workspaceResourceId: ${exampleAnalyticsWorkspace.id}
///       workspaceName: ${exampleAnalyticsWorkspace.name}
///       plan:
///         publisher: Microsoft
///         product: OMSGallery/ContainerInsights
/// ```
///
/// ## Import
///
/// Log Analytics Solutions can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:operationalinsights/analyticsSolution:AnalyticsSolution solution1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.OperationsManagement/solutions/solution1
/// ```
///
pub mod analytics_solution {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AnalyticsSolutionArgs {
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A `plan` block as documented below.
        #[builder(into)]
        pub plan: pulumi_wasm_rust::InputOrOutput<
            super::super::types::operationalinsights::AnalyticsSolutionPlan,
        >,
        /// The name of the resource group in which the Log Analytics solution is created. Changing this forces a new resource to be created. Note: The solution and its related workspace can only exist in the same resource group.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the name of the solution to be deployed. See [here for options](https://docs.microsoft.com/azure/log-analytics/log-analytics-add-solutions).Changing this forces a new resource to be created.
        #[builder(into)]
        pub solution_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The full name of the Log Analytics workspace with which the solution will be linked. Changing this forces a new resource to be created.
        #[builder(into)]
        pub workspace_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The full resource ID of the Log Analytics workspace with which the solution will be linked. Changing this forces a new resource to be created.
        #[builder(into)]
        pub workspace_resource_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AnalyticsSolutionResult {
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// A `plan` block as documented below.
        pub plan: pulumi_wasm_rust::Output<
            super::super::types::operationalinsights::AnalyticsSolutionPlan,
        >,
        /// The name of the resource group in which the Log Analytics solution is created. Changing this forces a new resource to be created. Note: The solution and its related workspace can only exist in the same resource group.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the solution to be deployed. See [here for options](https://docs.microsoft.com/azure/log-analytics/log-analytics-add-solutions).Changing this forces a new resource to be created.
        pub solution_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The full name of the Log Analytics workspace with which the solution will be linked. Changing this forces a new resource to be created.
        pub workspace_name: pulumi_wasm_rust::Output<String>,
        /// The full resource ID of the Log Analytics workspace with which the solution will be linked. Changing this forces a new resource to be created.
        pub workspace_resource_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AnalyticsSolutionArgs,
    ) -> AnalyticsSolutionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_output(context).get_inner();
        let plan_binding = args.plan.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let solution_name_binding = args.solution_name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let workspace_name_binding = args.workspace_name.get_output(context).get_inner();
        let workspace_resource_id_binding = args
            .workspace_resource_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:operationalinsights/analyticsSolution:AnalyticsSolution"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "plan".into(),
                    value: &plan_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "solutionName".into(),
                    value: &solution_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "workspaceName".into(),
                    value: &workspace_name_binding,
                },
                register_interface::ObjectField {
                    name: "workspaceResourceId".into(),
                    value: &workspace_resource_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AnalyticsSolutionResult {
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            plan: pulumi_wasm_rust::__private::into_domain(o.extract_field("plan")),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            solution_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("solutionName"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            workspace_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("workspaceName"),
            ),
            workspace_resource_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("workspaceResourceId"),
            ),
        }
    }
}
