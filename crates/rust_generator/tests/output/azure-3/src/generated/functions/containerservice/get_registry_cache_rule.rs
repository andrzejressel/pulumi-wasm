#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_registry_cache_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRegistryCacheRuleArgs {
        /// The ID of the container registry where the cache rule should apply. Changing this forces a new resource to be created.
        #[builder(into)]
        pub container_registry_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Container Registry Cache Rule. Only Alphanumeric characters allowed. Changing this forces a new resource to be created.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetRegistryCacheRuleResult {
        pub container_registry_id: pulumi_gestalt_rust::Output<String>,
        /// The ARM resource ID of the credential store which is associated with the cache rule.
        pub credential_set_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the source repository path.
        pub source_repo: pulumi_gestalt_rust::Output<String>,
        /// The name of the new repository path to store artifacts.
        pub target_repo: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetRegistryCacheRuleArgs,
    ) -> GetRegistryCacheRuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let container_registry_id_binding = args
            .container_registry_id
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:containerservice/getRegistryCacheRule:getRegistryCacheRule"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "containerRegistryId".into(),
                    value: container_registry_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetRegistryCacheRuleResult {
            container_registry_id: o.get_field("containerRegistryId"),
            credential_set_id: o.get_field("credentialSetId"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            source_repo: o.get_field("sourceRepo"),
            target_repo: o.get_field("targetRepo"),
        }
    }
}
