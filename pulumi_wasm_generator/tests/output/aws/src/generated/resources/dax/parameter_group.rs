/// Provides a DAX Parameter Group resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = parameter_group::create(
///         "example",
///         ParameterGroupArgs::builder()
///             .name("example")
///             .parameters(
///                 vec![
///                     ParameterGroupParameter::builder().name("query-ttl-millis")
///                     .value("100000").build_struct(), ParameterGroupParameter::builder()
///                     .name("record-ttl-millis").value("100000").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DAX Parameter Group using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:dax/parameterGroup:ParameterGroup example my_dax_pg
/// ```
pub mod parameter_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ParameterGroupArgs {
        /// A description of the parameter group.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the parameter group.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The parameters of the parameter group.
        #[builder(into, default)]
        pub parameters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::dax::ParameterGroupParameter>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ParameterGroupResult {
        /// A description of the parameter group.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the parameter group.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The parameters of the parameter group.
        pub parameters: pulumi_wasm_rust::Output<
            Vec<super::super::types::dax::ParameterGroupParameter>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ParameterGroupArgs) -> ParameterGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let parameters_binding = args.parameters.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:dax/parameterGroup:ParameterGroup".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parameters".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ParameterGroupResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameters").unwrap(),
            ),
        }
    }
}