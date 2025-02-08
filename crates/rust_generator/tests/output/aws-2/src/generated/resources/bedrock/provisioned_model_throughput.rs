/// Manages [Provisioned Throughput](https://docs.aws.amazon.com/bedrock/latest/userguide/prov-throughput.html) for an Amazon Bedrock model.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = provisioned_model_throughput::create(
///         "example",
///         ProvisionedModelThroughputArgs::builder()
///             .commitment_duration("SixMonths")
///             .model_arn("arn:aws:bedrock:us-east-1::foundation-model/anthropic.claude-v2")
///             .model_units(1)
///             .provisioned_model_name("example-model")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Provisioned Throughput using the `provisioned_model_arn`. For example:
///
/// ```sh
/// $ pulumi import aws:bedrock/provisionedModelThroughput:ProvisionedModelThroughput example arn:aws:bedrock:us-west-2:123456789012:provisioned-model/1y5n57gh5y2e
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod provisioned_model_throughput {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProvisionedModelThroughputArgs {
        /// Commitment duration requested for the Provisioned Throughput. For custom models, you can purchase on-demand Provisioned Throughput by omitting this argument. Valid values: `OneMonth`, `SixMonths`.
        #[builder(into, default)]
        pub commitment_duration: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ARN of the model to associate with this Provisioned Throughput.
        #[builder(into)]
        pub model_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Number of model units to allocate. A model unit delivers a specific throughput level for the specified model.
        #[builder(into)]
        pub model_units: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Unique name for this Provisioned Throughput.
        #[builder(into)]
        pub provisioned_model_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bedrock::ProvisionedModelThroughputTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct ProvisionedModelThroughputResult {
        /// Commitment duration requested for the Provisioned Throughput. For custom models, you can purchase on-demand Provisioned Throughput by omitting this argument. Valid values: `OneMonth`, `SixMonths`.
        pub commitment_duration: pulumi_gestalt_rust::Output<Option<String>>,
        /// ARN of the model to associate with this Provisioned Throughput.
        pub model_arn: pulumi_gestalt_rust::Output<String>,
        /// Number of model units to allocate. A model unit delivers a specific throughput level for the specified model.
        pub model_units: pulumi_gestalt_rust::Output<i32>,
        /// The ARN of the Provisioned Throughput.
        pub provisioned_model_arn: pulumi_gestalt_rust::Output<String>,
        /// Unique name for this Provisioned Throughput.
        pub provisioned_model_name: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::bedrock::ProvisionedModelThroughputTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ProvisionedModelThroughputArgs,
    ) -> ProvisionedModelThroughputResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let commitment_duration_binding = args
            .commitment_duration
            .get_output(context)
            .get_inner();
        let model_arn_binding = args.model_arn.get_output(context).get_inner();
        let model_units_binding = args.model_units.get_output(context).get_inner();
        let provisioned_model_name_binding = args
            .provisioned_model_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:bedrock/provisionedModelThroughput:ProvisionedModelThroughput"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "commitmentDuration".into(),
                    value: &commitment_duration_binding,
                },
                register_interface::ObjectField {
                    name: "modelArn".into(),
                    value: &model_arn_binding,
                },
                register_interface::ObjectField {
                    name: "modelUnits".into(),
                    value: &model_units_binding,
                },
                register_interface::ObjectField {
                    name: "provisionedModelName".into(),
                    value: &provisioned_model_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ProvisionedModelThroughputResult {
            commitment_duration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("commitmentDuration"),
            ),
            model_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("modelArn"),
            ),
            model_units: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("modelUnits"),
            ),
            provisioned_model_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("provisionedModelArn"),
            ),
            provisioned_model_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("provisionedModelName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            timeouts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
        }
    }
}
