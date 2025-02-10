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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SqlPoolWorkloadClassifierArgs,
    ) -> SqlPoolWorkloadClassifierResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let context_binding = args.context.get_output(context);
        let end_time_binding = args.end_time.get_output(context);
        let importance_binding = args.importance.get_output(context);
        let label_binding = args.label.get_output(context);
        let member_name_binding = args.member_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let start_time_binding = args.start_time.get_output(context);
        let workload_group_id_binding = args.workload_group_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:synapse/sqlPoolWorkloadClassifier:SqlPoolWorkloadClassifier"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "context".into(),
                    value: context_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endTime".into(),
                    value: end_time_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "importance".into(),
                    value: importance_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "label".into(),
                    value: label_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "memberName".into(),
                    value: member_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "startTime".into(),
                    value: start_time_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workloadGroupId".into(),
                    value: workload_group_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SqlPoolWorkloadClassifierResult {
            context: o.get_field("context"),
            end_time: o.get_field("endTime"),
            importance: o.get_field("importance"),
            label: o.get_field("label"),
            member_name: o.get_field("memberName"),
            name: o.get_field("name"),
            start_time: o.get_field("startTime"),
            workload_group_id: o.get_field("workloadGroupId"),
        }
    }
}
