/// Managing [IoT Thing indexing](https://docs.aws.amazon.com/iot/latest/developerguide/managing-index.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = indexing_configuration::create(
///         "example",
///         IndexingConfigurationArgs::builder()
///             .thing_indexing_configuration(
///                 IndexingConfigurationThingIndexingConfiguration::builder()
///                     .customFields(
///                         vec![
///                             IndexingConfigurationThingIndexingConfigurationCustomField::builder()
///                             .name("shadow.desired.power"). type ("Boolean")
///                             .build_struct(),
///                             IndexingConfigurationThingIndexingConfigurationCustomField::builder()
///                             .name("attributes.version"). type ("Number").build_struct(),
///                             IndexingConfigurationThingIndexingConfigurationCustomField::builder()
///                             .name("shadow.name.thing1shadow.desired.DefaultDesired").
///                             type ("String").build_struct(),
///                             IndexingConfigurationThingIndexingConfigurationCustomField::builder()
///                             .name("deviceDefender.securityProfile1.NUMBER_VALUE_BEHAVIOR.lastViolationValue.number")
///                             . type ("Number").build_struct(),
///                         ],
///                     )
///                     .deviceDefenderIndexingMode("VIOLATIONS")
///                     .filter(
///                         IndexingConfigurationThingIndexingConfigurationFilter::builder()
///                             .namedShadowNames(vec!["thing1shadow",])
///                             .build_struct(),
///                     )
///                     .namedShadowIndexingMode("ON")
///                     .thingConnectivityIndexingMode("STATUS")
///                     .thingIndexingMode("REGISTRY_AND_SHADOW")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
pub mod indexing_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IndexingConfigurationArgs {
        /// Thing group indexing configuration. See below.
        #[builder(into, default)]
        pub thing_group_indexing_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::iot::IndexingConfigurationThingGroupIndexingConfiguration,
            >,
        >,
        /// Thing indexing configuration. See below.
        #[builder(into, default)]
        pub thing_indexing_configuration: pulumi_wasm_rust::Output<
            Option<
                super::super::types::iot::IndexingConfigurationThingIndexingConfiguration,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct IndexingConfigurationResult {
        /// Thing group indexing configuration. See below.
        pub thing_group_indexing_configuration: pulumi_wasm_rust::Output<
            super::super::types::iot::IndexingConfigurationThingGroupIndexingConfiguration,
        >,
        /// Thing indexing configuration. See below.
        pub thing_indexing_configuration: pulumi_wasm_rust::Output<
            super::super::types::iot::IndexingConfigurationThingIndexingConfiguration,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: IndexingConfigurationArgs,
    ) -> IndexingConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let thing_group_indexing_configuration_binding = args
            .thing_group_indexing_configuration
            .get_inner();
        let thing_indexing_configuration_binding = args
            .thing_indexing_configuration
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iot/indexingConfiguration:IndexingConfiguration".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "thingGroupIndexingConfiguration".into(),
                    value: &thing_group_indexing_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "thingIndexingConfiguration".into(),
                    value: &thing_indexing_configuration_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "thingGroupIndexingConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "thingIndexingConfiguration".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        IndexingConfigurationResult {
            thing_group_indexing_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("thingGroupIndexingConfiguration").unwrap(),
            ),
            thing_indexing_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("thingIndexingConfiguration").unwrap(),
            ),
        }
    }
}