/// Provides a Step Function State Machine Alias.
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
///     let mySfnAlias = alias::create(
///         "mySfnAlias",
///         AliasArgs::builder()
///             .name("my_sfn_alias")
///             .routing_configurations(
///                 vec![
///                     AliasRoutingConfiguration::builder()
///                     .stateMachineVersionArn("arn:aws:states:us-east-1:12345:stateMachine:demo:3")
///                     .weight(50).build_struct(), AliasRoutingConfiguration::builder()
///                     .stateMachineVersionArn("arn:aws:states:us-east-1:12345:stateMachine:demo:2")
///                     .weight(50).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let sfnAlias = alias::create(
///         "sfnAlias",
///         AliasArgs::builder()
///             .name("my_sfn_alias")
///             .routing_configurations(
///                 vec![
///                     AliasRoutingConfiguration::builder()
///                     .stateMachineVersionArn("${sfnTest.stateMachineVersionArn}")
///                     .weight(100).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SFN (Step Functions) Alias using the `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:sfn/alias:Alias foo arn:aws:states:us-east-1:123456789098:stateMachine:myStateMachine:foo
/// ```
pub mod alias {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AliasArgs {
        /// Description of the alias.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Name for the alias you are creating.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The StateMachine alias' route configuration settings. Fields documented below
        #[builder(into)]
        pub routing_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::types::sfn::AliasRoutingConfiguration>,
        >,
    }
    #[allow(dead_code)]
    pub struct AliasResult {
        /// The Amazon Resource Name (ARN) identifying your state machine alias.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The date the state machine alias was created.
        pub creation_date: pulumi_wasm_rust::Output<String>,
        /// Description of the alias.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Name for the alias you are creating.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The StateMachine alias' route configuration settings. Fields documented below
        pub routing_configurations: pulumi_wasm_rust::Output<
            Vec<super::super::types::sfn::AliasRoutingConfiguration>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AliasArgs) -> AliasResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let routing_configurations_binding = args.routing_configurations.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sfn/alias:Alias".into(),
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
                    name: "routingConfigurations".into(),
                    value: &routing_configurations_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "creationDate".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "routingConfigurations".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AliasResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            creation_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationDate").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            routing_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routingConfigurations").unwrap(),
            ),
        }
    }
}
