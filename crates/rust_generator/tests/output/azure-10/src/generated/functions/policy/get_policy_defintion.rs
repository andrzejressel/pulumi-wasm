#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_policy_defintion {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPolicyDefintionArgs {
        /// Specifies the display name of the Policy Definition. Conflicts with `name`.
        ///
        /// > **NOTE** Looking up policies by `display_name` is not recommended by the Azure Policy team as the property is not unique nor immutable. As such errors may occur when there are multiple policy definitions with same display name or the display name is changed. To avoid these types of errors you may wish to use the `name` property instead.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Only retrieve Policy Definitions from this Management Group.
        #[builder(into, default)]
        pub management_group_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Policy Definition. Conflicts with `display_name`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetPolicyDefintionResult {
        /// The Description of the Policy.
        pub description: pulumi_gestalt_rust::Output<String>,
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub management_group_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Any Metadata defined in the Policy.
        pub metadata: pulumi_gestalt_rust::Output<String>,
        /// The Mode of the Policy.
        pub mode: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Any Parameters defined in the Policy.
        pub parameters: pulumi_gestalt_rust::Output<String>,
        /// The Rule as defined (in JSON) in the Policy.
        pub policy_rule: pulumi_gestalt_rust::Output<String>,
        /// The Type of the Policy. Possible values are `BuiltIn`, `Custom` and `NotSpecified`.
        pub policy_type: pulumi_gestalt_rust::Output<String>,
        /// A list of role definition id extracted from `policy_rule` required for remediation.
        pub role_definition_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The Type of Policy.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetPolicyDefintionArgs,
    ) -> GetPolicyDefintionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let display_name_binding = args.display_name.get_output(context);
        let management_group_name_binding = args
            .management_group_name
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:policy/getPolicyDefintion:getPolicyDefintion".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managementGroupName".into(),
                    value: &management_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetPolicyDefintionResult {
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            id: o.get_field("id"),
            management_group_name: o.get_field("managementGroupName"),
            metadata: o.get_field("metadata"),
            mode: o.get_field("mode"),
            name: o.get_field("name"),
            parameters: o.get_field("parameters"),
            policy_rule: o.get_field("policyRule"),
            policy_type: o.get_field("policyType"),
            role_definition_ids: o.get_field("roleDefinitionIds"),
            type_: o.get_field("type"),
        }
    }
}
