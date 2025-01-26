/// Resource for managing an AWS AppSync Source Api Association.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = source_api_association::create(
///         "test",
///         SourceApiAssociationArgs::builder()
///             .description("My source API Merged")
///             .merged_api_id("gzos6bteufdunffzzifiowisoe")
///             .source_api_id("fzzifiowisoegzos6bteufdunf")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AppSync Source Api Association using the `gzos6bteufdunffzzifiowisoe,243685a0-9347-4a1a-89c1-9b57dea01e31`. For example:
///
/// ```sh
/// $ pulumi import aws:appsync/sourceApiAssociation:SourceApiAssociation example gzos6bteufdunffzzifiowisoe,243685a0-9347-4a1a-89c1-9b57dea01e31
/// ```
pub mod source_api_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SourceApiAssociationArgs {
        /// Description of the source API being merged.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// ARN of the merged API. One of `merged_api_arn` or `merged_api_id` must be specified.
        #[builder(into, default)]
        pub merged_api_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// ID of the merged API. One of `merged_api_arn` or `merged_api_id` must be specified.
        #[builder(into, default)]
        pub merged_api_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// ARN of the source API. One of `source_api_arn` or `source_api_id` must be specified.
        #[builder(into, default)]
        pub source_api_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub source_api_association_configs: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::appsync::SourceApiAssociationSourceApiAssociationConfig,
                >,
            >,
        >,
        /// ID of the source API. One of `source_api_arn` or `source_api_id` must be specified.
        #[builder(into, default)]
        pub source_api_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::appsync::SourceApiAssociationTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct SourceApiAssociationResult {
        /// ARN of the Source Api Association.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// ID of the Source Api Association.
        pub association_id: pulumi_wasm_rust::Output<String>,
        /// Description of the source API being merged.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the merged API. One of `merged_api_arn` or `merged_api_id` must be specified.
        pub merged_api_arn: pulumi_wasm_rust::Output<String>,
        /// ID of the merged API. One of `merged_api_arn` or `merged_api_id` must be specified.
        pub merged_api_id: pulumi_wasm_rust::Output<String>,
        /// ARN of the source API. One of `source_api_arn` or `source_api_id` must be specified.
        pub source_api_arn: pulumi_wasm_rust::Output<String>,
        pub source_api_association_configs: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::appsync::SourceApiAssociationSourceApiAssociationConfig,
            >,
        >,
        /// ID of the source API. One of `source_api_arn` or `source_api_id` must be specified.
        pub source_api_id: pulumi_wasm_rust::Output<String>,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::appsync::SourceApiAssociationTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: SourceApiAssociationArgs,
    ) -> SourceApiAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let merged_api_arn_binding = args.merged_api_arn.get_output(context).get_inner();
        let merged_api_id_binding = args.merged_api_id.get_output(context).get_inner();
        let source_api_arn_binding = args.source_api_arn.get_output(context).get_inner();
        let source_api_association_configs_binding = args
            .source_api_association_configs
            .get_output(context)
            .get_inner();
        let source_api_id_binding = args.source_api_id.get_output(context).get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:appsync/sourceApiAssociation:SourceApiAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "mergedApiArn".into(),
                    value: &merged_api_arn_binding,
                },
                register_interface::ObjectField {
                    name: "mergedApiId".into(),
                    value: &merged_api_id_binding,
                },
                register_interface::ObjectField {
                    name: "sourceApiArn".into(),
                    value: &source_api_arn_binding,
                },
                register_interface::ObjectField {
                    name: "sourceApiAssociationConfigs".into(),
                    value: &source_api_association_configs_binding,
                },
                register_interface::ObjectField {
                    name: "sourceApiId".into(),
                    value: &source_api_id_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SourceApiAssociationResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            association_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("associationId"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            merged_api_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("mergedApiArn"),
            ),
            merged_api_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("mergedApiId"),
            ),
            source_api_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sourceApiArn"),
            ),
            source_api_association_configs: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sourceApiAssociationConfigs"),
            ),
            source_api_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sourceApiId"),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
        }
    }
}
