/// Manages an Azure App Configuration Feature.
///
/// > **Note:** App Configuration Features are provisioned using a Data Plane API which requires the role `App Configuration Data Owner` on either the App Configuration or a parent scope (such as the Resource Group/Subscription). [More information can be found in the Azure Documentation for App Configuration](https://docs.microsoft.com/azure/azure-app-configuration/concept-enable-rbac#azure-built-in-roles-for-azure-app-configuration). This is similar to providing App Configuration Keys.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   appconf:
///     type: azure:appconfiguration:ConfigurationStore
///     properties:
///       name: appConf1
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///   appconfDataowner:
///     type: azure:authorization:Assignment
///     name: appconf_dataowner
///     properties:
///       scope: ${appconf.id}
///       roleDefinitionName: App Configuration Data Owner
///       principalId: ${current.objectId}
///   test:
///     type: azure:appconfiguration:ConfigurationFeature
///     properties:
///       configurationStoreId: ${appconf.id}
///       description: test description
///       name: test-ackey
///       label: test-ackeylabel
///       enabled: true
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// App Configuration Features can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appconfiguration/configurationFeature:ConfigurationFeature test https://appconfname1.azconfig.io/kv/.appconfig.featureflag%2FkeyName?label=labelName
/// ```
///
/// If you wish to import with an empty label then simply leave the label's name blank:
///
/// ```sh
/// $ pulumi import azure:appconfiguration/configurationFeature:ConfigurationFeature test https://appconfname1.azconfig.io/kv/.appconfig.featureflag%2FkeyName?label=
/// ```
///
pub mod configuration_feature {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConfigurationFeatureArgs {
        /// Specifies the id of the App Configuration. Changing this forces a new resource to be created.
        #[builder(into)]
        pub configuration_store_id: pulumi_wasm_rust::Output<String>,
        /// The description of the App Configuration Feature.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The status of the App Configuration Feature. By default, this is set to false.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        #[builder(into, default)]
        pub etag: pulumi_wasm_rust::Output<Option<String>>,
        /// The key of the App Configuration Feature. The value for `name` will be used if this is unspecified. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub key: pulumi_wasm_rust::Output<Option<String>>,
        /// The label of the App Configuration Feature. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub label: pulumi_wasm_rust::Output<Option<String>>,
        /// Should this App Configuration Feature be Locked to prevent changes?
        #[builder(into, default)]
        pub locked: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the App Configuration Feature. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A number representing the value of the percentage required to enable this feature.
        #[builder(into, default)]
        pub percentage_filter_value: pulumi_wasm_rust::Output<Option<f64>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `targeting_filter` block as defined below.
        #[builder(into, default)]
        pub targeting_filters: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::appconfiguration::ConfigurationFeatureTargetingFilter,
                >,
            >,
        >,
        /// A `timewindow_filter` block as defined below.
        #[builder(into, default)]
        pub timewindow_filters: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::appconfiguration::ConfigurationFeatureTimewindowFilter,
                >,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct ConfigurationFeatureResult {
        /// Specifies the id of the App Configuration. Changing this forces a new resource to be created.
        pub configuration_store_id: pulumi_wasm_rust::Output<String>,
        /// The description of the App Configuration Feature.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The status of the App Configuration Feature. By default, this is set to false.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The key of the App Configuration Feature. The value for `name` will be used if this is unspecified. Changing this forces a new resource to be created.
        pub key: pulumi_wasm_rust::Output<String>,
        /// The label of the App Configuration Feature. Changing this forces a new resource to be created.
        pub label: pulumi_wasm_rust::Output<Option<String>>,
        /// Should this App Configuration Feature be Locked to prevent changes?
        pub locked: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the App Configuration Feature. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A number representing the value of the percentage required to enable this feature.
        pub percentage_filter_value: pulumi_wasm_rust::Output<Option<f64>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `targeting_filter` block as defined below.
        pub targeting_filters: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::appconfiguration::ConfigurationFeatureTargetingFilter,
                >,
            >,
        >,
        /// A `timewindow_filter` block as defined below.
        pub timewindow_filters: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::appconfiguration::ConfigurationFeatureTimewindowFilter,
                >,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ConfigurationFeatureArgs,
    ) -> ConfigurationFeatureResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let configuration_store_id_binding = args.configuration_store_id.get_inner();
        let description_binding = args.description.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let etag_binding = args.etag.get_inner();
        let key_binding = args.key.get_inner();
        let label_binding = args.label.get_inner();
        let locked_binding = args.locked.get_inner();
        let name_binding = args.name.get_inner();
        let percentage_filter_value_binding = args.percentage_filter_value.get_inner();
        let tags_binding = args.tags.get_inner();
        let targeting_filters_binding = args.targeting_filters.get_inner();
        let timewindow_filters_binding = args.timewindow_filters.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appconfiguration/configurationFeature:ConfigurationFeature"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "configurationStoreId".into(),
                    value: &configuration_store_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "etag".into(),
                    value: &etag_binding,
                },
                register_interface::ObjectField {
                    name: "key".into(),
                    value: &key_binding,
                },
                register_interface::ObjectField {
                    name: "label".into(),
                    value: &label_binding,
                },
                register_interface::ObjectField {
                    name: "locked".into(),
                    value: &locked_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "percentageFilterValue".into(),
                    value: &percentage_filter_value_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "targetingFilters".into(),
                    value: &targeting_filters_binding,
                },
                register_interface::ObjectField {
                    name: "timewindowFilters".into(),
                    value: &timewindow_filters_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "configurationStoreId".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "key".into(),
                },
                register_interface::ResultField {
                    name: "label".into(),
                },
                register_interface::ResultField {
                    name: "locked".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "percentageFilterValue".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "targetingFilters".into(),
                },
                register_interface::ResultField {
                    name: "timewindowFilters".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ConfigurationFeatureResult {
            configuration_store_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configurationStoreId").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("key").unwrap(),
            ),
            label: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("label").unwrap(),
            ),
            locked: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("locked").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            percentage_filter_value: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("percentageFilterValue").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            targeting_filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetingFilters").unwrap(),
            ),
            timewindow_filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timewindowFilters").unwrap(),
            ),
        }
    }
}
