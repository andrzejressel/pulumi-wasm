/// Creates a new Amazon Redshift Serverless Usage Limit.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = workgroup::create(
///         "example",
///         WorkgroupArgs::builder()
///             .namespace_name("${exampleAwsRedshiftserverlessNamespace.namespaceName}")
///             .workgroup_name("example")
///             .build_struct(),
///     );
///     let exampleUsageLimit = usage_limit::create(
///         "exampleUsageLimit",
///         UsageLimitArgs::builder()
///             .amount(60)
///             .resource_arn("${example.arn}")
///             .usage_type("serverless-compute")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Redshift Serverless Usage Limits using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:redshiftserverless/usageLimit:UsageLimit example example-id
/// ```
pub mod usage_limit {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UsageLimitArgs {
        /// The limit amount. If time-based, this amount is in Redshift Processing Units (RPU) consumed per hour. If data-based, this amount is in terabytes (TB) of data transferred between Regions in cross-account sharing. The value must be a positive number.
        #[builder(into)]
        pub amount: pulumi_wasm_rust::InputOrOutput<i32>,
        /// The action that Amazon Redshift Serverless takes when the limit is reached. Valid values are `log`, `emit-metric`, and `deactivate`. The default is `log`.
        #[builder(into, default)]
        pub breach_action: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The time period that the amount applies to. A weekly period begins on Sunday. Valid values are `daily`, `weekly`, and `monthly`. The default is `monthly`.
        #[builder(into, default)]
        pub period: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Amazon Resource Name (ARN) of the Amazon Redshift Serverless resource to create the usage limit for.
        #[builder(into)]
        pub resource_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// The type of Amazon Redshift Serverless usage to create a usage limit for. Valid values are `serverless-compute` or `cross-region-datasharing`.
        #[builder(into)]
        pub usage_type: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct UsageLimitResult {
        /// The limit amount. If time-based, this amount is in Redshift Processing Units (RPU) consumed per hour. If data-based, this amount is in terabytes (TB) of data transferred between Regions in cross-account sharing. The value must be a positive number.
        pub amount: pulumi_wasm_rust::Output<i32>,
        /// Amazon Resource Name (ARN) of the Redshift Serverless Usage Limit.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The action that Amazon Redshift Serverless takes when the limit is reached. Valid values are `log`, `emit-metric`, and `deactivate`. The default is `log`.
        pub breach_action: pulumi_wasm_rust::Output<Option<String>>,
        /// The time period that the amount applies to. A weekly period begins on Sunday. Valid values are `daily`, `weekly`, and `monthly`. The default is `monthly`.
        pub period: pulumi_wasm_rust::Output<Option<String>>,
        /// The Amazon Resource Name (ARN) of the Amazon Redshift Serverless resource to create the usage limit for.
        pub resource_arn: pulumi_wasm_rust::Output<String>,
        /// The type of Amazon Redshift Serverless usage to create a usage limit for. Valid values are `serverless-compute` or `cross-region-datasharing`.
        pub usage_type: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: UsageLimitArgs,
    ) -> UsageLimitResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let amount_binding = args.amount.get_output(context).get_inner();
        let breach_action_binding = args.breach_action.get_output(context).get_inner();
        let period_binding = args.period.get_output(context).get_inner();
        let resource_arn_binding = args.resource_arn.get_output(context).get_inner();
        let usage_type_binding = args.usage_type.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:redshiftserverless/usageLimit:UsageLimit".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
                    name: "period".into(),
                    value: &period_binding,
                },
                register_interface::ObjectField {
                    name: "resourceArn".into(),
                    value: &resource_arn_binding,
                },
                register_interface::ObjectField {
                    name: "usageType".into(),
                    value: &usage_type_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        UsageLimitResult {
            amount: pulumi_wasm_rust::__private::into_domain(o.extract_field("amount")),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            breach_action: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("breachAction"),
            ),
            period: pulumi_wasm_rust::__private::into_domain(o.extract_field("period")),
            resource_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceArn"),
            ),
            usage_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("usageType"),
            ),
        }
    }
}
