/// Creates a new Amazon Redshift Usage Limit.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = usage_limit::create(
///         "example",
///         UsageLimitArgs::builder()
///             .amount(60)
///             .cluster_identifier("${exampleAwsRedshiftCluster.id}")
///             .feature_type("concurrency-scaling")
///             .limit_type("time")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Redshift usage limits using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:redshift/usageLimit:UsageLimit example example-id
/// ```
pub mod usage_limit {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UsageLimitArgs {
        /// The limit amount. If time-based, this amount is in minutes. If data-based, this amount is in terabytes (TB). The value must be a positive number.
        #[builder(into)]
        pub amount: pulumi_wasm_rust::Output<i32>,
        /// The action that Amazon Redshift takes when the limit is reached. The default is `log`. Valid values are `log`, `emit-metric`, and `disable`.
        #[builder(into, default)]
        pub breach_action: pulumi_wasm_rust::Output<Option<String>>,
        /// The identifier of the cluster that you want to limit usage.
        #[builder(into)]
        pub cluster_identifier: pulumi_wasm_rust::Output<String>,
        /// The Amazon Redshift feature that you want to limit. Valid values are `spectrum`, `concurrency-scaling`, and `cross-region-datasharing`.
        #[builder(into)]
        pub feature_type: pulumi_wasm_rust::Output<String>,
        /// The type of limit. Depending on the feature type, this can be based on a time duration or data size. If FeatureType is `spectrum`, then LimitType must be `data-scanned`. If FeatureType is `concurrency-scaling`, then LimitType must be `time`. If FeatureType is `cross-region-datasharing`, then LimitType must be `data-scanned`. Valid values are `data-scanned`, and `time`.
        #[builder(into)]
        pub limit_type: pulumi_wasm_rust::Output<String>,
        /// The time period that the amount applies to. A weekly period begins on Sunday. The default is `monthly`. Valid values are `daily`, `weekly`, and `monthly`.
        #[builder(into, default)]
        pub period: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct UsageLimitResult {
        /// The limit amount. If time-based, this amount is in minutes. If data-based, this amount is in terabytes (TB). The value must be a positive number.
        pub amount: pulumi_wasm_rust::Output<i32>,
        /// Amazon Resource Name (ARN) of the Redshift Usage Limit.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The action that Amazon Redshift takes when the limit is reached. The default is `log`. Valid values are `log`, `emit-metric`, and `disable`.
        pub breach_action: pulumi_wasm_rust::Output<Option<String>>,
        /// The identifier of the cluster that you want to limit usage.
        pub cluster_identifier: pulumi_wasm_rust::Output<String>,
        /// The Amazon Redshift feature that you want to limit. Valid values are `spectrum`, `concurrency-scaling`, and `cross-region-datasharing`.
        pub feature_type: pulumi_wasm_rust::Output<String>,
        /// The type of limit. Depending on the feature type, this can be based on a time duration or data size. If FeatureType is `spectrum`, then LimitType must be `data-scanned`. If FeatureType is `concurrency-scaling`, then LimitType must be `time`. If FeatureType is `cross-region-datasharing`, then LimitType must be `data-scanned`. Valid values are `data-scanned`, and `time`.
        pub limit_type: pulumi_wasm_rust::Output<String>,
        /// The time period that the amount applies to. A weekly period begins on Sunday. The default is `monthly`. Valid values are `daily`, `weekly`, and `monthly`.
        pub period: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: UsageLimitArgs) -> UsageLimitResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let amount_binding = args.amount.get_inner();
        let breach_action_binding = args.breach_action.get_inner();
        let cluster_identifier_binding = args.cluster_identifier.get_inner();
        let feature_type_binding = args.feature_type.get_inner();
        let limit_type_binding = args.limit_type.get_inner();
        let period_binding = args.period.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:redshift/usageLimit:UsageLimit".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "amount".into(),
                    value: &amount_binding,
                },
                register_interface::ObjectField {
                    name: "breachAction".into(),
                    value: &breach_action_binding,
                },
                register_interface::ObjectField {
                    name: "clusterIdentifier".into(),
                    value: &cluster_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "featureType".into(),
                    value: &feature_type_binding,
                },
                register_interface::ObjectField {
                    name: "limitType".into(),
                    value: &limit_type_binding,
                },
                register_interface::ObjectField {
                    name: "period".into(),
                    value: &period_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "amount".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "breachAction".into(),
                },
                register_interface::ResultField {
                    name: "clusterIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "featureType".into(),
                },
                register_interface::ResultField {
                    name: "limitType".into(),
                },
                register_interface::ResultField {
                    name: "period".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        UsageLimitResult {
            amount: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("amount").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            breach_action: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("breachAction").unwrap(),
            ),
            cluster_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterIdentifier").unwrap(),
            ),
            feature_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("featureType").unwrap(),
            ),
            limit_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("limitType").unwrap(),
            ),
            period: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("period").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}