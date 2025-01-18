pub mod get_subscription {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSubscriptionArgs {
        /// The name of the Cloud Pub/Sub Subscription.
        ///
        /// - - -
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
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
    pub fn invoke(args: GetSubscriptionArgs) -> GetSubscriptionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "ackDeadlineSeconds".into(),
                },
                register_interface::ResultField {
                    name: "bigqueryConfigs".into(),
                },
                register_interface::ResultField {
                    name: "cloudStorageConfigs".into(),
                },
                register_interface::ResultField {
                    name: "deadLetterPolicies".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "enableExactlyOnceDelivery".into(),
                },
                register_interface::ResultField {
                    name: "enableMessageOrdering".into(),
                },
                register_interface::ResultField {
                    name: "expirationPolicies".into(),
                },
                register_interface::ResultField {
                    name: "filter".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "messageRetentionDuration".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "pushConfigs".into(),
                },
                register_interface::ResultField {
                    name: "retainAckedMessages".into(),
                },
                register_interface::ResultField {
                    name: "retryPolicies".into(),
                },
                register_interface::ResultField {
                    name: "topic".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetSubscriptionResult {
            ack_deadline_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ackDeadlineSeconds").unwrap(),
            ),
            bigquery_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bigqueryConfigs").unwrap(),
            ),
            cloud_storage_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cloudStorageConfigs").unwrap(),
            ),
            dead_letter_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deadLetterPolicies").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            enable_exactly_once_delivery: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableExactlyOnceDelivery").unwrap(),
            ),
            enable_message_ordering: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableMessageOrdering").unwrap(),
            ),
            expiration_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expirationPolicies").unwrap(),
            ),
            filter: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filter").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            message_retention_duration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("messageRetentionDuration").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            push_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pushConfigs").unwrap(),
            ),
            retain_acked_messages: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retainAckedMessages").unwrap(),
            ),
            retry_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retryPolicies").unwrap(),
            ),
            topic: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("topic").unwrap(),
            ),
        }
    }
}
