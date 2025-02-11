/// Provides a Step Function State Machine Alias.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod alias {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AliasArgs {
        /// Description of the alias.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name for the alias you are creating.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The StateMachine alias' route configuration settings. Fields documented below
        #[builder(into)]
        pub routing_configurations: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::sfn::AliasRoutingConfiguration>,
        >,
    }
    #[allow(dead_code)]
    pub struct AliasResult {
        /// The Amazon Resource Name (ARN) identifying your state machine alias.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The date the state machine alias was created.
        pub creation_date: pulumi_gestalt_rust::Output<String>,
        /// Description of the alias.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name for the alias you are creating.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The StateMachine alias' route configuration settings. Fields documented below
        pub routing_configurations: pulumi_gestalt_rust::Output<
            Vec<super::super::types::sfn::AliasRoutingConfiguration>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AliasArgs,
    ) -> AliasResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let routing_configurations_binding = args
            .routing_configurations
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sfn/alias:Alias".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "routingConfigurations".into(),
                    value: &routing_configurations_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AliasResult {
            arn: o.get_field("arn"),
            creation_date: o.get_field("creationDate"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            routing_configurations: o.get_field("routingConfigurations"),
        }
    }
}
