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
pub mod size_constraint_set {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SizeConstraintSetArgs {
        /// The name or description of the Size Constraint Set.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the parts of web requests that you want to inspect the size of.
        #[builder(into, default)]
        pub size_constraints: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::wafregional::SizeConstraintSetSizeConstraint>,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct SizeConstraintSetResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The name or description of the Size Constraint Set.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the parts of web requests that you want to inspect the size of.
        pub size_constraints: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::wafregional::SizeConstraintSetSizeConstraint>,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SizeConstraintSetArgs) -> SizeConstraintSetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let size_constraints_binding = args.size_constraints.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:wafregional/sizeConstraintSet:SizeConstraintSet".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "sizeConstraints".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SizeConstraintSetResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            size_constraints: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sizeConstraints").unwrap(),
            ),
        }
    }
}
