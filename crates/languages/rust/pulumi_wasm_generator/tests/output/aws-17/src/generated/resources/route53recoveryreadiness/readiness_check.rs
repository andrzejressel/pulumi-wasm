/// Provides an AWS Route 53 Recovery Readiness Readiness Check.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = readiness_check::create(
///         "example",
///         ReadinessCheckArgs::builder()
///             .readiness_check_name("${[\"my-cw-alarm-check\"]}")
///             .resource_set_name("${[\"my-cw-alarm-set\"]}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Route53 Recovery Readiness readiness checks using the readiness check name. For example:
///
/// ```sh
/// $ pulumi import aws:route53recoveryreadiness/readinessCheck:ReadinessCheck my-cw-alarm-check example
/// ```
pub mod readiness_check {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReadinessCheckArgs {
        /// Unique name describing the readiness check.
        #[builder(into)]
        pub readiness_check_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Name describing the resource set that will be monitored for readiness.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub resource_set_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ReadinessCheckResult {
        /// ARN of the readiness_check
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Unique name describing the readiness check.
        pub readiness_check_name: pulumi_wasm_rust::Output<String>,
        /// Name describing the resource set that will be monitored for readiness.
        ///
        /// The following arguments are optional:
        pub resource_set_name: pulumi_wasm_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ReadinessCheckArgs,
    ) -> ReadinessCheckResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let readiness_check_name_binding = args
            .readiness_check_name
            .get_output(context)
            .get_inner();
        let resource_set_name_binding = args
            .resource_set_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:route53recoveryreadiness/readinessCheck:ReadinessCheck".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "readinessCheckName".into(),
                    value: &readiness_check_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceSetName".into(),
                    value: &resource_set_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ReadinessCheckResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            readiness_check_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("readinessCheckName"),
            ),
            resource_set_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceSetName"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
