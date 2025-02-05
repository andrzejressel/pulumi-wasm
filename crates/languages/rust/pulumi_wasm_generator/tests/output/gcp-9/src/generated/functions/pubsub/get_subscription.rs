pub mod get_subscription {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSubscriptionArgs {
        /// The name of the Cloud Pub/Sub Subscription.
        ///
        /// - - -
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetSubscriptionResult {
        pub ack_deadline_seconds: pulumi_wasm_rust::Output<i32>,
        pub bigquery_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::pubsub::GetSubscriptionBigqueryConfig>,
        >,
        pub cloud_storage_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::pubsub::GetSubscriptionCloudStorageConfig>,
        >,
        pub dead_letter_policies: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::pubsub::GetSubscriptionDeadLetterPolicy>,
        >,
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub enable_exactly_once_delivery: pulumi_wasm_rust::Output<bool>,
        pub enable_message_ordering: pulumi_wasm_rust::Output<bool>,
        pub expiration_policies: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::pubsub::GetSubscriptionExpirationPolicy>,
        >,
        pub filter: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub labels: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub message_retention_duration: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub push_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::pubsub::GetSubscriptionPushConfig>,
        >,
        pub retain_acked_messages: pulumi_wasm_rust::Output<bool>,
        pub retry_policies: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::pubsub::GetSubscriptionRetryPolicy>,
        >,
        pub topic: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetSubscriptionArgs,
    ) -> GetSubscriptionResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:pubsub/getSubscription:getSubscription".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSubscriptionResult {
            ack_deadline_seconds: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ackDeadlineSeconds"),
            ),
            bigquery_configs: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("bigqueryConfigs"),
            ),
            cloud_storage_configs: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cloudStorageConfigs"),
            ),
            dead_letter_policies: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deadLetterPolicies"),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            enable_exactly_once_delivery: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enableExactlyOnceDelivery"),
            ),
            enable_message_ordering: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enableMessageOrdering"),
            ),
            expiration_policies: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("expirationPolicies"),
            ),
            filter: pulumi_wasm_rust::__private::into_domain(o.extract_field("filter")),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            labels: pulumi_wasm_rust::__private::into_domain(o.extract_field("labels")),
            message_retention_duration: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("messageRetentionDuration"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            push_configs: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pushConfigs"),
            ),
            retain_acked_messages: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("retainAckedMessages"),
            ),
            retry_policies: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("retryPolicies"),
            ),
            topic: pulumi_wasm_rust::__private::into_domain(o.extract_field("topic")),
        }
    }
}
