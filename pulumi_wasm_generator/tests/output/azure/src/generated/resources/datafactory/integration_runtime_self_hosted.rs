/// Manages a Data Factory Self-hosted Integration Runtime.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod integration_runtime_self_hosted {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IntegrationRuntimeSelfHostedArgs {
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        #[builder(into)]
        pub data_factory_id: pulumi_wasm_rust::Output<String>,
        /// Integration runtime description.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Data Factory. Changing this forces a new Data Factory Self-hosted Integration Runtime to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A `rbac_authorization` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub rbac_authorizations: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::datafactory::IntegrationRuntimeSelfHostedRbacAuthorization,
                >,
            >,
        >,
        /// Specifies whether enable interactive authoring function when your self-hosted integration runtime is unable to establish a connection with Azure Relay.
        #[builder(into, default)]
        pub self_contained_interactive_authoring_enabled: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
    }
    #[allow(dead_code)]
    pub struct IntegrationRuntimeSelfHostedResult {
        /// The Data Factory ID in which to associate the Linked Service with. Changing this forces a new resource.
        pub data_factory_id: pulumi_wasm_rust::Output<String>,
        /// Integration runtime description.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Data Factory. Changing this forces a new Data Factory Self-hosted Integration Runtime to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The primary integration runtime authentication key.
        pub primary_authorization_key: pulumi_wasm_rust::Output<String>,
        /// A `rbac_authorization` block as defined below. Changing this forces a new resource to be created.
        pub rbac_authorizations: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::datafactory::IntegrationRuntimeSelfHostedRbacAuthorization,
                >,
            >,
        >,
        /// The secondary integration runtime authentication key.
        pub secondary_authorization_key: pulumi_wasm_rust::Output<String>,
        /// Specifies whether enable interactive authoring function when your self-hosted integration runtime is unable to establish a connection with Azure Relay.
        pub self_contained_interactive_authoring_enabled: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: IntegrationRuntimeSelfHostedArgs,
    ) -> IntegrationRuntimeSelfHostedResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let data_factory_id_binding = args.data_factory_id.get_inner();
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let rbac_authorizations_binding = args.rbac_authorizations.get_inner();
        let self_contained_interactive_authoring_enabled_binding = args
            .self_contained_interactive_authoring_enabled
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:datafactory/integrationRuntimeSelfHosted:IntegrationRuntimeSelfHosted"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dataFactoryId".into(),
                    value: &data_factory_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "rbacAuthorizations".into(),
                    value: &rbac_authorizations_binding,
                },
                register_interface::ObjectField {
                    name: "selfContainedInteractiveAuthoringEnabled".into(),
                    value: &self_contained_interactive_authoring_enabled_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "dataFactoryId".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "primaryAuthorizationKey".into(),
                },
                register_interface::ResultField {
                    name: "rbacAuthorizations".into(),
                },
                register_interface::ResultField {
                    name: "secondaryAuthorizationKey".into(),
                },
                register_interface::ResultField {
                    name: "selfContainedInteractiveAuthoringEnabled".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        IntegrationRuntimeSelfHostedResult {
            data_factory_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataFactoryId").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            primary_authorization_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryAuthorizationKey").unwrap(),
            ),
            rbac_authorizations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rbacAuthorizations").unwrap(),
            ),
            secondary_authorization_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryAuthorizationKey").unwrap(),
            ),
            self_contained_interactive_authoring_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfContainedInteractiveAuthoringEnabled").unwrap(),
            ),
        }
    }
}