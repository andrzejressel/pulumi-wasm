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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DefinitionArgs,
    ) -> DefinitionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let authorizations_binding = args.authorizations.get_output(context);
        let description_binding = args.description.get_output(context);
        let eligible_authorizations_binding = args
            .eligible_authorizations
            .get_output(context);
        let lighthouse_definition_id_binding = args
            .lighthouse_definition_id
            .get_output(context);
        let managing_tenant_id_binding = args.managing_tenant_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let plan_binding = args.plan.get_output(context);
        let scope_binding = args.scope.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:lighthouse/definition:Definition".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authorizations".into(),
                    value: authorizations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eligibleAuthorizations".into(),
                    value: eligible_authorizations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lighthouseDefinitionId".into(),
                    value: lighthouse_definition_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managingTenantId".into(),
                    value: managing_tenant_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "plan".into(),
                    value: plan_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scope".into(),
                    value: scope_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DefinitionResult {
            authorizations: o.get_field("authorizations"),
            description: o.get_field("description"),
            eligible_authorizations: o.get_field("eligibleAuthorizations"),
            lighthouse_definition_id: o.get_field("lighthouseDefinitionId"),
            managing_tenant_id: o.get_field("managingTenantId"),
            name: o.get_field("name"),
            plan: o.get_field("plan"),
            scope: o.get_field("scope"),
        }
    }
}
