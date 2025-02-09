/// Manages a [Lighthouse](https://docs.microsoft.com/azure/lighthouse) Assignment to a subscription, or to a resource group.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:lighthouse:Assignment
///     properties:
///       scope: ${primary.id}
///       lighthouseDefinitionId: /subscriptions/00000000-0000-0000-0000-000000000000/providers/Microsoft.ManagedServices/registrationDefinitions/00000000-0000-0000-0000-000000000000
/// variables:
///   primary:
///     fn::invoke:
///       function: azure:core:getSubscription
///       arguments: {}
/// ```
///
/// ## Import
///
/// Lighthouse Assignments can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:lighthouse/assignment:Assignment example /subscriptions/00000000-0000-0000-0000-000000000000/providers/Microsoft.ManagedServices/registrationAssignments/00000000-0000-0000-0000-000000000000
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod assignment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AssignmentArgs {
        /// A Fully qualified path of the lighthouse definition, such as `/subscriptions/0afefe50-734e-4610-8c82-a144aff49dea/providers/Microsoft.ManagedServices/registrationDefinitions/26c128c2-fefa-4340-9bb1-8e081c90ada2`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub lighthouse_definition_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A unique UUID/GUID which identifies this lighthouse assignment- one will be generated if not specified. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The scope at which the Lighthouse Assignment applies too, such as `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333` or `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333/resourceGroups/myGroup`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub scope: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AssignmentResult {
        /// A Fully qualified path of the lighthouse definition, such as `/subscriptions/0afefe50-734e-4610-8c82-a144aff49dea/providers/Microsoft.ManagedServices/registrationDefinitions/26c128c2-fefa-4340-9bb1-8e081c90ada2`. Changing this forces a new resource to be created.
        pub lighthouse_definition_id: pulumi_gestalt_rust::Output<String>,
        /// A unique UUID/GUID which identifies this lighthouse assignment- one will be generated if not specified. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The scope at which the Lighthouse Assignment applies too, such as `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333` or `/subscriptions/0b1f6471-1bf0-4dda-aec3-111122223333/resourceGroups/myGroup`. Changing this forces a new resource to be created.
        pub scope: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AssignmentArgs,
    ) -> AssignmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let lighthouse_definition_id_binding_1 = args
            .lighthouse_definition_id
            .get_output(context);
        let lighthouse_definition_id_binding = lighthouse_definition_id_binding_1
            .get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let scope_binding_1 = args.scope.get_output(context);
        let scope_binding = scope_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:lighthouse/assignment:Assignment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "lighthouseDefinitionId".into(),
                    value: &lighthouse_definition_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "scope".into(),
                    value: &scope_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AssignmentResult {
            lighthouse_definition_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lighthouseDefinitionId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            scope: pulumi_gestalt_rust::__private::into_domain(o.extract_field("scope")),
        }
    }
}
