/// Manages a Data Factory Self-hosted Integration Runtime.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleFactory = factory::create(
///         "exampleFactory",
///         FactoryArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleIntegrationRuntimeSelfHosted = integration_runtime_self_hosted::create(
///         "exampleIntegrationRuntimeSelfHosted",
///         IntegrationRuntimeSelfHostedArgs::builder()
///             .data_factory_id("${exampleFactory.id}")
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Data Factories can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datafactory/integrationRuntimeSelfHosted:IntegrationRuntimeSelfHosted example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example/providers/Microsoft.DataFactory/factories/example/integrationruntimes/example
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod integration_runtime_self_hosted {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IntegrationRuntimeSelfHostedArgs {
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Integration runtime description.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Data Factory. Changing this forces a new Data Factory Self-hosted Integration Runtime to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `rbac_authorization` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub rbac_authorizations: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::datafactory::IntegrationRuntimeSelfHostedRbacAuthorization,
                >,
            >,
        >,
        /// Specifies whether enable interactive authoring function when your self-hosted integration runtime is unable to establish a connection with Azure Relay.
        #[builder(into, default)]
        pub self_contained_interactive_authoring_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
    }
    #[allow(dead_code)]
    pub struct IntegrationRuntimeSelfHostedResult {
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        pub data_factory_id: pulumi_gestalt_rust::Output<String>,
        /// Integration runtime description.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name which should be used for this Data Factory. Changing this forces a new Data Factory Self-hosted Integration Runtime to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The primary integration runtime authentication key.
        pub primary_authorization_key: pulumi_gestalt_rust::Output<String>,
        /// A `rbac_authorization` block as defined below. Changing this forces a new resource to be created.
        pub rbac_authorizations: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::datafactory::IntegrationRuntimeSelfHostedRbacAuthorization,
                >,
            >,
        >,
        /// The secondary integration runtime authentication key.
        pub secondary_authorization_key: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether enable interactive authoring function when your self-hosted integration runtime is unable to establish a connection with Azure Relay.
        pub self_contained_interactive_authoring_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IntegrationRuntimeSelfHostedArgs,
    ) -> IntegrationRuntimeSelfHostedResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let data_factory_id_binding = args.data_factory_id.get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let rbac_authorizations_binding = args.rbac_authorizations.get_output(context);
        let self_contained_interactive_authoring_enabled_binding = args
            .self_contained_interactive_authoring_enabled
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:datafactory/integrationRuntimeSelfHosted:IntegrationRuntimeSelfHosted"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataFactoryId".into(),
                    value: &data_factory_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rbacAuthorizations".into(),
                    value: &rbac_authorizations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "selfContainedInteractiveAuthoringEnabled".into(),
                    value: &self_contained_interactive_authoring_enabled_binding
                        .drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        IntegrationRuntimeSelfHostedResult {
            data_factory_id: o.get_field("dataFactoryId"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            primary_authorization_key: o.get_field("primaryAuthorizationKey"),
            rbac_authorizations: o.get_field("rbacAuthorizations"),
            secondary_authorization_key: o.get_field("secondaryAuthorizationKey"),
            self_contained_interactive_authoring_enabled: o
                .get_field("selfContainedInteractiveAuthoringEnabled"),
        }
    }
}
