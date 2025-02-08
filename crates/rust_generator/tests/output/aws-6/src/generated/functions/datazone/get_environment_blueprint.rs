#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_environment_blueprint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEnvironmentBlueprintArgs {
        /// ID of the domain.
        #[builder(into)]
        pub domain_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether the blueprint is managed by Amazon DataZone.
        #[builder(into)]
        pub managed: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// Name of the blueprint.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetEnvironmentBlueprintResult {
        /// Provider of the blueprint
        pub blueprint_provider: pulumi_gestalt_rust::Output<String>,
        /// Description of the blueprint
        pub description: pulumi_gestalt_rust::Output<String>,
        pub domain_id: pulumi_gestalt_rust::Output<String>,
        /// ID of the environment blueprint
        pub id: pulumi_gestalt_rust::Output<String>,
        pub managed: pulumi_gestalt_rust::Output<bool>,
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetEnvironmentBlueprintArgs,
    ) -> GetEnvironmentBlueprintResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let domain_id_binding = args.domain_id.get_output(context).get_inner();
        let managed_binding = args.managed.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:datazone/getEnvironmentBlueprint:getEnvironmentBlueprint".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "domainId".into(),
                    value: &domain_id_binding,
                },
                register_interface::ObjectField {
                    name: "managed".into(),
                    value: &managed_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetEnvironmentBlueprintResult {
            blueprint_provider: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("blueprintProvider"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            domain_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("domainId"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            managed: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("managed"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
        }
    }
}
