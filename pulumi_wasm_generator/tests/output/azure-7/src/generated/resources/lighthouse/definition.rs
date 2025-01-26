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
pub mod definition {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DefinitionArgs {
        /// An `authorization` block as defined below.
        #[builder(into)]
        pub authorizations: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::lighthouse::DefinitionAuthorization>,
        >,
        /// A description of the Lighthouse Definition.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// An `eligible_authorization` block as defined below.
        #[builder(into, default)]
        pub eligible_authorizations: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::lighthouse::DefinitionEligibleAuthorization>>,
        >,
        /// A unique UUID/GUID which identifies this lighthouse definition - one will be generated if not specified. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub lighthouse_definition_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the managing tenant. Changing this forces a new resource to be created.
        #[builder(into)]
        pub managing_tenant_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Lighthouse Definition. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A `plan` block as defined below.
        #[builder(into, default)]
        pub plan: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::lighthouse::DefinitionPlan>,
        >,
        /// The ID of the managed subscription. Changing this forces a new resource to be created.
        #[builder(into)]
        pub scope: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DefinitionResult {
        /// An `authorization` block as defined below.
        pub authorizations: pulumi_wasm_rust::Output<
            Vec<super::super::types::lighthouse::DefinitionAuthorization>,
        >,
        /// A description of the Lighthouse Definition.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// An `eligible_authorization` block as defined below.
        pub eligible_authorizations: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::lighthouse::DefinitionEligibleAuthorization>>,
        >,
        /// A unique UUID/GUID which identifies this lighthouse definition - one will be generated if not specified. Changing this forces a new resource to be created.
        pub lighthouse_definition_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the managing tenant. Changing this forces a new resource to be created.
        pub managing_tenant_id: pulumi_wasm_rust::Output<String>,
        /// The name of the Lighthouse Definition. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `plan` block as defined below.
        pub plan: pulumi_wasm_rust::Output<
            Option<super::super::types::lighthouse::DefinitionPlan>,
        >,
        /// The ID of the managed subscription. Changing this forces a new resource to be created.
        pub scope: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: DefinitionArgs,
    ) -> DefinitionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "authorizations".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "eligibleAuthorizations".into(),
                },
                register_interface::ResultField {
                    name: "lighthouseDefinitionId".into(),
                },
                register_interface::ResultField {
                    name: "managingTenantId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "plan".into(),
                },
                register_interface::ResultField {
                    name: "scope".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DefinitionResult {
            authorizations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizations").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            eligible_authorizations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eligibleAuthorizations").unwrap(),
            ),
            lighthouse_definition_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lighthouseDefinitionId").unwrap(),
            ),
            managing_tenant_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managingTenantId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            plan: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("plan").unwrap(),
            ),
            scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scope").unwrap(),
            ),
        }
    }
}
