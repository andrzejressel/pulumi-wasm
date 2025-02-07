/// Use the `aws.waf.SizeConstraintSet` resource to manage WAF size constraint sets.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   sizeConstraintSet:
///     type: aws:waf:SizeConstraintSet
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
/// Using `pulumi import`, import AWS WAF Size Constraint Set using their ID. For example:
///
/// ```sh
/// $ pulumi import aws:waf/sizeConstraintSet:SizeConstraintSet example a1b2c3d4-d5f6-7777-8888-9999aaaabbbbcccc
/// ```
pub mod size_constraint_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SizeConstraintSetArgs {
        /// Name or description of the Size Constraint Set.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Parts of web requests that you want to inspect the size of.
        #[builder(into, default)]
        pub size_constraints: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::waf::SizeConstraintSetSizeConstraint>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SizeConstraintSetResult {
        /// Amazon Resource Name (ARN).
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Name or description of the Size Constraint Set.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Parts of web requests that you want to inspect the size of.
        pub size_constraints: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::waf::SizeConstraintSetSizeConstraint>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SizeConstraintSetArgs,
    ) -> SizeConstraintSetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let size_constraints_binding = args
            .size_constraints
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:waf/sizeConstraintSet:SizeConstraintSet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "sizeConstraints".into(),
                    value: &size_constraints_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SizeConstraintSetResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            size_constraints: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sizeConstraints"),
            ),
        }
    }
}
