/// Manages a Stream Analytics Output powerBI.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleOutputPowerbi:
///     type: azure:streamanalytics:OutputPowerbi
///     name: example
///     properties:
///       name: output-to-powerbi
///       streamAnalyticsJobId: ${exampleGetJob.id}
///       dataset: example-dataset
///       table: example-table
///       groupId: 00000000-0000-0000-0000-000000000000
///       groupName: some-group-name
/// variables:
///   example:
///     fn::invoke:
///       function: azure:core:getResourceGroup
///       arguments:
///         name: example-resources
///   exampleGetJob:
///     fn::invoke:
///       function: azure:streamanalytics:getJob
///       arguments:
///         name: example-job
///         resourceGroupName: ${example.name}
/// ```
///
/// ## Import
///
/// Stream Analytics Output to Power BI can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:streamanalytics/outputPowerbi:OutputPowerbi example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.StreamAnalytics/streamingJobs/job1/outputs/output1
/// ```
///
pub mod output_powerbi {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OutputPowerbiArgs {
        /// The name of the Power BI dataset.
        #[builder(into)]
        pub dataset: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the Power BI group, this must be a valid UUID.
        #[builder(into)]
        pub group_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Power BI group. Use this property to help remember which specific Power BI group id was used.
        #[builder(into)]
        pub group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Stream Output. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the Stream Analytics Job. Changing this forces a new resource to be created.
        #[builder(into)]
        pub stream_analytics_job_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Power BI table under the specified dataset.
        #[builder(into)]
        pub table: pulumi_wasm_rust::InputOrOutput<String>,
        /// The user display name of the user that was used to obtain the refresh token.
        #[builder(into, default)]
        pub token_user_display_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The user principal name (UPN) of the user that was used to obtain the refresh token.
        #[builder(into, default)]
        pub token_user_principal_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct OutputPowerbiResult {
        /// The name of the Power BI dataset.
        pub dataset: pulumi_wasm_rust::Output<String>,
        /// The ID of the Power BI group, this must be a valid UUID.
        pub group_id: pulumi_wasm_rust::Output<String>,
        /// The name of the Power BI group. Use this property to help remember which specific Power BI group id was used.
        pub group_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Stream Output. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Stream Analytics Job. Changing this forces a new resource to be created.
        pub stream_analytics_job_id: pulumi_wasm_rust::Output<String>,
        /// The name of the Power BI table under the specified dataset.
        pub table: pulumi_wasm_rust::Output<String>,
        /// The user display name of the user that was used to obtain the refresh token.
        pub token_user_display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The user principal name (UPN) of the user that was used to obtain the refresh token.
        pub token_user_principal_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: OutputPowerbiArgs,
    ) -> OutputPowerbiResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let dataset_binding = args.dataset.get_output(context).get_inner();
        let group_id_binding = args.group_id.get_output(context).get_inner();
        let group_name_binding = args.group_name.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let stream_analytics_job_id_binding = args
            .stream_analytics_job_id
            .get_output(context)
            .get_inner();
        let table_binding = args.table.get_output(context).get_inner();
        let token_user_display_name_binding = args
            .token_user_display_name
            .get_output(context)
            .get_inner();
        let token_user_principal_name_binding = args
            .token_user_principal_name
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:streamanalytics/outputPowerbi:OutputPowerbi".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dataset".into(),
                    value: &dataset_binding,
                },
                register_interface::ObjectField {
                    name: "groupId".into(),
                    value: &group_id_binding,
                },
                register_interface::ObjectField {
                    name: "groupName".into(),
                    value: &group_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "streamAnalyticsJobId".into(),
                    value: &stream_analytics_job_id_binding,
                },
                register_interface::ObjectField {
                    name: "table".into(),
                    value: &table_binding,
                },
                register_interface::ObjectField {
                    name: "tokenUserDisplayName".into(),
                    value: &token_user_display_name_binding,
                },
                register_interface::ObjectField {
                    name: "tokenUserPrincipalName".into(),
                    value: &token_user_principal_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "dataset".into(),
                },
                register_interface::ResultField {
                    name: "groupId".into(),
                },
                register_interface::ResultField {
                    name: "groupName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "streamAnalyticsJobId".into(),
                },
                register_interface::ResultField {
                    name: "table".into(),
                },
                register_interface::ResultField {
                    name: "tokenUserDisplayName".into(),
                },
                register_interface::ResultField {
                    name: "tokenUserPrincipalName".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        OutputPowerbiResult {
            dataset: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataset").unwrap(),
            ),
            group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("groupId").unwrap(),
            ),
            group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("groupName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            stream_analytics_job_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("streamAnalyticsJobId").unwrap(),
            ),
            table: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("table").unwrap(),
            ),
            token_user_display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tokenUserDisplayName").unwrap(),
            ),
            token_user_principal_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tokenUserPrincipalName").unwrap(),
            ),
        }
    }
}
