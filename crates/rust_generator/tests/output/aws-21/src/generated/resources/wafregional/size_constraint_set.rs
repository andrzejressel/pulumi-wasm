/// Provides a WAF Regional Size Constraint Set Resource for use with Application Load Balancer.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   sizeConstraintSet:
///     type: aws:wafregional:SizeConstraintSet
///     name: size_constraint_set
///     properties:
///       name: tfsize_constraints
///       sizeConstraints:
///         - textTransformation: NONE
///           comparisonOperator: EQ
///           size: '4096'
///           fieldToMatch:
///             type: BODY
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import WAF Size Constraint Set using the id. For example:
///
/// ```sh
/// $ pulumi import aws:wafregional/sizeConstraintSet:SizeConstraintSet size_constraint_set a1b2c3d4-d5f6-7777-8888-9999aaaabbbbcccc
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod size_constraint_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SizeConstraintSetArgs {
        /// The name or description of the Size Constraint Set.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the parts of web requests that you want to inspect the size of.
        #[builder(into, default)]
        pub size_constraints: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::wafregional::SizeConstraintSetSizeConstraint>,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct SizeConstraintSetResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The name or description of the Size Constraint Set.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the parts of web requests that you want to inspect the size of.
        pub size_constraints: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::wafregional::SizeConstraintSetSizeConstraint>,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SizeConstraintSetArgs,
    ) -> SizeConstraintSetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let size_constraints_binding = args.size_constraints.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:wafregional/sizeConstraintSet:SizeConstraintSet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sizeConstraints".into(),
                    value: size_constraints_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SizeConstraintSetResult {
            arn: o.get_field("arn"),
            name: o.get_field("name"),
            size_constraints: o.get_field("sizeConstraints"),
        }
    }
}
