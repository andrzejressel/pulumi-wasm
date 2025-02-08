/// Creates Security Hub custom action.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = account::create("example", AccountArgs::builder().build_struct());
///     let exampleActionTarget = action_target::create(
///         "exampleActionTarget",
///         ActionTargetArgs::builder()
///             .description("This is custom action sends selected findings to chat")
///             .identifier("SendToChat")
///             .name("Send notification to chat")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Security Hub custom action using the action target ARN. For example:
///
/// ```sh
/// $ pulumi import aws:securityhub/actionTarget:ActionTarget example arn:aws:securityhub:eu-west-1:312940875350:action/custom/a
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod action_target {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ActionTargetArgs {
        /// The name of the custom action target.
        #[builder(into)]
        pub description: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID for the custom action target.
        #[builder(into)]
        pub identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description for the custom action target.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ActionTargetResult {
        /// Amazon Resource Name (ARN) of the Security Hub custom action target.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the custom action target.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The ID for the custom action target.
        pub identifier: pulumi_gestalt_rust::Output<String>,
        /// The description for the custom action target.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ActionTargetArgs,
    ) -> ActionTargetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let identifier_binding = args.identifier.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:securityhub/actionTarget:ActionTarget".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "identifier".into(),
                    value: &identifier_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ActionTargetResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identifier"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
        }
    }
}
