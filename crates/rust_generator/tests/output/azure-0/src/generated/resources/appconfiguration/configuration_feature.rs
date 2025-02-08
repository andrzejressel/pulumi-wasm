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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod configuration_feature {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConfigurationFeatureArgs {
        /// Specifies the id of the App Configuration. Changing this forces a new resource to be created.
        #[builder(into)]
        pub configuration_store_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description of the App Configuration Feature.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The status of the App Configuration Feature. By default, this is set to false.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub etag: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The key of the App Configuration Feature. The value for `name` will be used if this is unspecified. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The label of the App Configuration Feature. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub label: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Should this App Configuration Feature be Locked to prevent changes?
        #[builder(into, default)]
        pub locked: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the App Configuration Feature. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A number representing the value of the percentage required to enable this feature.
        #[builder(into, default)]
        pub percentage_filter_value: pulumi_gestalt_rust::InputOrOutput<Option<f64>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `targeting_filter` block as defined below.
        #[builder(into, default)]
        pub targeting_filters: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::appconfiguration::ConfigurationFeatureTargetingFilter,
                >,
            >,
        >,
        /// A `timewindow_filter` block as defined below.
        #[builder(into, default)]
        pub timewindow_filters: pulumi_gestalt_rust::InputOrOutput<
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
        pub configuration_store_id: pulumi_gestalt_rust::Output<String>,
        /// The description of the App Configuration Feature.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The status of the App Configuration Feature. By default, this is set to false.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The key of the App Configuration Feature. The value for `name` will be used if this is unspecified. Changing this forces a new resource to be created.
        pub key: pulumi_gestalt_rust::Output<String>,
        /// The label of the App Configuration Feature. Changing this forces a new resource to be created.
        pub label: pulumi_gestalt_rust::Output<Option<String>>,
        /// Should this App Configuration Feature be Locked to prevent changes?
        pub locked: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the App Configuration Feature. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A number representing the value of the percentage required to enable this feature.
        pub percentage_filter_value: pulumi_gestalt_rust::Output<Option<f64>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `targeting_filter` block as defined below.
        pub targeting_filters: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::appconfiguration::ConfigurationFeatureTargetingFilter,
                >,
            >,
        >,
        /// A `timewindow_filter` block as defined below.
        pub timewindow_filters: pulumi_gestalt_rust::Output<
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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ConfigurationFeatureArgs,
    ) -> ConfigurationFeatureResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let configuration_store_id_binding = args
            .configuration_store_id
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let etag_binding = args.etag.get_output(context).get_inner();
        let key_binding = args.key.get_output(context).get_inner();
        let label_binding = args.label.get_output(context).get_inner();
        let locked_binding = args.locked.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let percentage_filter_value_binding = args
            .percentage_filter_value
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let targeting_filters_binding = args
            .targeting_filters
            .get_output(context)
            .get_inner();
        let timewindow_filters_binding = args
            .timewindow_filters
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appconfiguration/configurationFeature:ConfigurationFeature"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        ConfigurationFeatureResult {
            configuration_store_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("configurationStoreId"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            key: pulumi_gestalt_rust::__private::into_domain(o.extract_field("key")),
            label: pulumi_gestalt_rust::__private::into_domain(o.extract_field("label")),
            locked: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("locked"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            percentage_filter_value: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("percentageFilterValue"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            targeting_filters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetingFilters"),
            ),
            timewindow_filters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timewindowFilters"),
            ),
        }
    }
}
