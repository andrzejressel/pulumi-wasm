/// Manages a Synapse SQL Pool Workload Classifier.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountKind: BlobStorage
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleDataLakeGen2Filesystem:
///     type: azure:storage:DataLakeGen2Filesystem
///     name: example
///     properties:
///       name: example
///       storageAccountId: ${exampleAccount.id}
///   exampleWorkspace:
///     type: azure:synapse:Workspace
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       storageDataLakeGen2FilesystemId: ${exampleDataLakeGen2Filesystem.id}
///       sqlAdministratorLogin: sqladminuser
///       sqlAdministratorLoginPassword: H@Sh1CoR3!
///       identity:
///         type: SystemAssigned
///   exampleSqlPool:
///     type: azure:synapse:SqlPool
///     name: example
///     properties:
///       name: example
///       synapseWorkspaceId: ${exampleWorkspace.id}
///       skuName: DW100c
///       createMode: Default
///   exampleSqlPoolWorkloadGroup:
///     type: azure:synapse:SqlPoolWorkloadGroup
///     name: example
///     properties:
///       name: example
///       sqlPoolId: ${exampleSqlPool.id}
///       importance: normal
///       maxResourcePercent: 100
///       minResourcePercent: 0
///       maxResourcePercentPerRequest: 3
///       minResourcePercentPerRequest: 3
///       queryExecutionTimeoutInSeconds: 0
///   exampleSqlPoolWorkloadClassifier:
///     type: azure:synapse:SqlPoolWorkloadClassifier
///     name: example
///     properties:
///       name: example
///       workloadGroupId: ${exampleSqlPoolWorkloadGroup.id}
///       context: example_context
///       endTime: 14:00
///       importance: high
///       label: example_label
///       memberName: dbo
///       startTime: 12:00
/// ```
///
/// ## Import
///
/// Synapse SQL Pool Workload Classifiers can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:synapse/sqlPoolWorkloadClassifier:SqlPoolWorkloadClassifier example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.Synapse/workspaces/workspace1/sqlPools/sqlPool1/workloadGroups/workloadGroup1/workloadClassifiers/workloadClassifier1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod sql_pool_workload_classifier {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SqlPoolWorkloadClassifierArgs {
        /// Specifies the session context value that a request can be classified against.
        #[builder(into, default)]
        pub context: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The workload classifier end time for classification. It's of the `HH:MM` format in UTC time zone.
        #[builder(into, default)]
        pub end_time: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The workload classifier importance. The allowed values are `low`, `below_normal`, `normal`, `above_normal` and `high`.
        #[builder(into, default)]
        pub importance: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the label value that a request can be classified against.
        #[builder(into, default)]
        pub label: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The workload classifier member name used to classified against.
        #[builder(into)]
        pub member_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this Synapse SQL Pool Workload Classifier. Changing this forces a new Synapse SQL Pool Workload Classifier to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The workload classifier start time for classification. It's of the `HH:MM` format in UTC time zone.
        #[builder(into, default)]
        pub start_time: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Synapse SQL Pool Workload Group. Changing this forces a new Synapse SQL Pool Workload Classifier to be created.
        #[builder(into)]
        pub workload_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SqlPoolWorkloadClassifierResult {
        /// Specifies the session context value that a request can be classified against.
        pub context: pulumi_gestalt_rust::Output<Option<String>>,
        /// The workload classifier end time for classification. It's of the `HH:MM` format in UTC time zone.
        pub end_time: pulumi_gestalt_rust::Output<Option<String>>,
        /// The workload classifier importance. The allowed values are `low`, `below_normal`, `normal`, `above_normal` and `high`.
        pub importance: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the label value that a request can be classified against.
        pub label: pulumi_gestalt_rust::Output<Option<String>>,
        /// The workload classifier member name used to classified against.
        pub member_name: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Synapse SQL Pool Workload Classifier. Changing this forces a new Synapse SQL Pool Workload Classifier to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The workload classifier start time for classification. It's of the `HH:MM` format in UTC time zone.
        pub start_time: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Synapse SQL Pool Workload Group. Changing this forces a new Synapse SQL Pool Workload Classifier to be created.
        pub workload_group_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SqlPoolWorkloadClassifierArgs,
    ) -> SqlPoolWorkloadClassifierResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let context_binding_1 = args.context.get_output(context);
        let context_binding = context_binding_1.get_inner();
        let end_time_binding_1 = args.end_time.get_output(context);
        let end_time_binding = end_time_binding_1.get_inner();
        let importance_binding_1 = args.importance.get_output(context);
        let importance_binding = importance_binding_1.get_inner();
        let label_binding_1 = args.label.get_output(context);
        let label_binding = label_binding_1.get_inner();
        let member_name_binding_1 = args.member_name.get_output(context);
        let member_name_binding = member_name_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let start_time_binding_1 = args.start_time.get_output(context);
        let start_time_binding = start_time_binding_1.get_inner();
        let workload_group_id_binding_1 = args.workload_group_id.get_output(context);
        let workload_group_id_binding = workload_group_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:synapse/sqlPoolWorkloadClassifier:SqlPoolWorkloadClassifier"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "context".into(),
                    value: &context_binding,
                },
                register_interface::ObjectField {
                    name: "endTime".into(),
                    value: &end_time_binding,
                },
                register_interface::ObjectField {
                    name: "importance".into(),
                    value: &importance_binding,
                },
                register_interface::ObjectField {
                    name: "label".into(),
                    value: &label_binding,
                },
                register_interface::ObjectField {
                    name: "memberName".into(),
                    value: &member_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "startTime".into(),
                    value: &start_time_binding,
                },
                register_interface::ObjectField {
                    name: "workloadGroupId".into(),
                    value: &workload_group_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SqlPoolWorkloadClassifierResult {
            context: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("context"),
            ),
            end_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endTime"),
            ),
            importance: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("importance"),
            ),
            label: pulumi_gestalt_rust::__private::into_domain(o.extract_field("label")),
            member_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("memberName"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            start_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("startTime"),
            ),
            workload_group_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workloadGroupId"),
            ),
        }
    }
}
