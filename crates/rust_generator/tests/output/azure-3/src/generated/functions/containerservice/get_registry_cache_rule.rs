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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetRegistryCacheRuleArgs,
    ) -> GetRegistryCacheRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let container_registry_id_binding = args
            .container_registry_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:containerservice/getRegistryCacheRule:getRegistryCacheRule"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "containerRegistryId".into(),
                    value: &container_registry_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetRegistryCacheRuleResult {
            container_registry_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("containerRegistryId"),
            ),
            credential_set_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("credentialSetId"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            source_repo: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceRepo"),
            ),
            target_repo: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetRepo"),
            ),
        }
    }
}
