#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_subscription {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSubscriptionArgs {
        /// The name of the Cloud Pub/Sub Subscription.
        ///
        /// - - -
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetSubscriptionResult {
        pub ack_deadline_seconds: pulumi_gestalt_rust::Output<i32>,
        pub bigquery_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::pubsub::GetSubscriptionBigqueryConfig>,
        >,
        pub cloud_storage_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::pubsub::GetSubscriptionCloudStorageConfig>,
        >,
        pub dead_letter_policies: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::pubsub::GetSubscriptionDeadLetterPolicy>,
        >,
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub enable_exactly_once_delivery: pulumi_gestalt_rust::Output<bool>,
        pub enable_message_ordering: pulumi_gestalt_rust::Output<bool>,
        pub expiration_policies: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::pubsub::GetSubscriptionExpirationPolicy>,
        >,
        pub filter: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub message_retention_duration: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub push_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::pubsub::GetSubscriptionPushConfig>,
        >,
        pub retain_acked_messages: pulumi_gestalt_rust::Output<bool>,
        pub retry_policies: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::pubsub::GetSubscriptionRetryPolicy>,
        >,
        pub topic: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetSubscriptionArgs,
    ) -> GetSubscriptionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:pubsub/getSubscription:getSubscription".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSubscriptionResult {
            ack_deadline_seconds: o.get_field("ackDeadlineSeconds"),
            bigquery_configs: o.get_field("bigqueryConfigs"),
            cloud_storage_configs: o.get_field("cloudStorageConfigs"),
            dead_letter_policies: o.get_field("deadLetterPolicies"),
            effective_labels: o.get_field("effectiveLabels"),
            enable_exactly_once_delivery: o.get_field("enableExactlyOnceDelivery"),
            enable_message_ordering: o.get_field("enableMessageOrdering"),
            expiration_policies: o.get_field("expirationPolicies"),
            filter: o.get_field("filter"),
            id: o.get_field("id"),
            labels: o.get_field("labels"),
            message_retention_duration: o.get_field("messageRetentionDuration"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            push_configs: o.get_field("pushConfigs"),
            retain_acked_messages: o.get_field("retainAckedMessages"),
            retry_policies: o.get_field("retryPolicies"),
            topic: o.get_field("topic"),
        }
    }
}
