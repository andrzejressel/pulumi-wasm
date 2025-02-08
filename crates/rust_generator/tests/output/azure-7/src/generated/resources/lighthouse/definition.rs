/// Manages a [Lighthouse](https://docs.microsoft.com/azure/lighthouse) Definition.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:lighthouse:Definition
///     properties:
///       name: Sample definition
///       description: This is a lighthouse definition created IaC
///       managingTenantId: 00000000-0000-0000-0000-000000000000
///       scope: /subscriptions/00000000-0000-0000-0000-000000000000
///       authorizations:
///         - principalId: 00000000-0000-0000-0000-000000000000
///           roleDefinitionId: ${contributor.roleDefinitionId}
///           principalDisplayName: Tier 1 Support
/// variables:
///   contributor:
///     fn::invoke:
///       function: azure:authorization:getRoleDefinition
///       arguments:
///         roleDefinitionId: b24988ac-6180-42a0-ab88-20f7382dd24c
/// ```
///
/// ## Import
///
/// Lighthouse Definitions can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:lighthouse/definition:Definition example /subscriptions/00000000-0000-0000-0000-000000000000/providers/Microsoft.ManagedServices/registrationDefinitions/00000000-0000-0000-0000-000000000000
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod definition {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DefinitionArgs {
        /// An `authorization` block as defined below.
        #[builder(into)]
        pub authorizations: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::lighthouse::DefinitionAuthorization>,
        >,
        /// A description of the Lighthouse Definition.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An `eligible_authorization` block as defined below.
        #[builder(into, default)]
        pub eligible_authorizations: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::lighthouse::DefinitionEligibleAuthorization>>,
        >,
        /// A unique UUID/GUID which identifies this lighthouse definition - one will be generated if not specified. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub lighthouse_definition_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the managing tenant. Changing this forces a new resource to be created.
        #[builder(into)]
        pub managing_tenant_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Lighthouse Definition. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `plan` block as defined below.
        #[builder(into, default)]
        pub plan: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lighthouse::DefinitionPlan>,
        >,
        /// The ID of the managed subscription. Changing this forces a new resource to be created.
        #[builder(into)]
        pub scope: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DefinitionResult {
        /// An `authorization` block as defined below.
        pub authorizations: pulumi_gestalt_rust::Output<
            Vec<super::super::types::lighthouse::DefinitionAuthorization>,
        >,
        /// A description of the Lighthouse Definition.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// An `eligible_authorization` block as defined below.
        pub eligible_authorizations: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::lighthouse::DefinitionEligibleAuthorization>>,
        >,
        /// A unique UUID/GUID which identifies this lighthouse definition - one will be generated if not specified. Changing this forces a new resource to be created.
        pub lighthouse_definition_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the managing tenant. Changing this forces a new resource to be created.
        pub managing_tenant_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the Lighthouse Definition. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `plan` block as defined below.
        pub plan: pulumi_gestalt_rust::Output<
            Option<super::super::types::lighthouse::DefinitionPlan>,
        >,
        /// The ID of the managed subscription. Changing this forces a new resource to be created.
        pub scope: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DefinitionArgs,
    ) -> DefinitionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let authorizations_binding = args.authorizations.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let eligible_authorizations_binding = args
            .eligible_authorizations
            .get_output(context)
            .get_inner();
        let lighthouse_definition_id_binding = args
            .lighthouse_definition_id
            .get_output(context)
            .get_inner();
        let managing_tenant_id_binding = args
            .managing_tenant_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let plan_binding = args.plan.get_output(context).get_inner();
        let scope_binding = args.scope.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:lighthouse/definition:Definition".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authorizations".into(),
                    value: &authorizations_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "eligibleAuthorizations".into(),
                    value: &eligible_authorizations_binding,
                },
                register_interface::ObjectField {
                    name: "lighthouseDefinitionId".into(),
                    value: &lighthouse_definition_id_binding,
                },
                register_interface::ObjectField {
                    name: "managingTenantId".into(),
                    value: &managing_tenant_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "plan".into(),
                    value: &plan_binding,
                },
                register_interface::ObjectField {
                    name: "scope".into(),
                    value: &scope_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DefinitionResult {
            authorizations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("authorizations"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            eligible_authorizations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("eligibleAuthorizations"),
            ),
            lighthouse_definition_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lighthouseDefinitionId"),
            ),
            managing_tenant_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("managingTenantId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            plan: pulumi_gestalt_rust::__private::into_domain(o.extract_field("plan")),
            scope: pulumi_gestalt_rust::__private::into_domain(o.extract_field("scope")),
        }
    }
}
