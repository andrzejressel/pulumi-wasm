/// A Bigquery Analytics Hub data exchange
///
///
/// To get more information about DataExchange, see:
///
/// * [API documentation](https://cloud.google.com/bigquery/docs/reference/analytics-hub/rest/v1/projects.locations.dataExchanges)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/bigquery/docs/analytics-hub-introduction)
///
/// ## Example Usage
///
/// ### Bigquery Analyticshub Data Exchange Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let dataExchange = data_exchange::create(
///         "dataExchange",
///         DataExchangeArgs::builder()
///             .data_exchange_id("my_data_exchange")
///             .description("example data exchange")
///             .display_name("my_data_exchange")
///             .location("US")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Bigquery Analyticshub Data Exchange Dcr
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let dataExchange = data_exchange::create(
///         "dataExchange",
///         DataExchangeArgs::builder()
///             .data_exchange_id("dcr_data_exchange")
///             .description("example dcr data exchange")
///             .display_name("dcr_data_exchange")
///             .location("US")
///             .sharing_environment_config(
///                 DataExchangeSharingEnvironmentConfig::builder()
///                     .dcrExchangeConfig(
///                         DataExchangeSharingEnvironmentConfigDcrExchangeConfig::builder()
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// DataExchange can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/dataExchanges/{{data_exchange_id}}`
///
/// * `{{project}}/{{location}}/{{data_exchange_id}}`
///
/// * `{{location}}/{{data_exchange_id}}`
///
/// * `{{data_exchange_id}}`
///
/// When using the `pulumi import` command, DataExchange can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:bigqueryanalyticshub/dataExchange:DataExchange default projects/{{project}}/locations/{{location}}/dataExchanges/{{data_exchange_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:bigqueryanalyticshub/dataExchange:DataExchange default {{project}}/{{location}}/{{data_exchange_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:bigqueryanalyticshub/dataExchange:DataExchange default {{location}}/{{data_exchange_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:bigqueryanalyticshub/dataExchange:DataExchange default {{data_exchange_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod data_exchange {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataExchangeArgs {
        /// The ID of the data exchange. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces.
        #[builder(into)]
        pub data_exchange_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Description of the data exchange.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Human-readable display name of the data exchange. The display name must contain only Unicode letters, numbers (0-9), underscores (_), dashes (-), spaces ( ), and must not start or end with spaces.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Documentation describing the data exchange.
        #[builder(into, default)]
        pub documentation: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Base64 encoded image representing the data exchange.
        #[builder(into, default)]
        pub icon: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the location this data exchange.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Email or URL of the primary point of contact of the data exchange.
        #[builder(into, default)]
        pub primary_contact: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configurable data sharing environment option for a data exchange.
        /// This field is required for data clean room exchanges.
        /// Structure is documented below.
        #[builder(into, default)]
        pub sharing_environment_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::bigqueryanalyticshub::DataExchangeSharingEnvironmentConfig,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct DataExchangeResult {
        /// The ID of the data exchange. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces.
        pub data_exchange_id: pulumi_gestalt_rust::Output<String>,
        /// Description of the data exchange.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Human-readable display name of the data exchange. The display name must contain only Unicode letters, numbers (0-9), underscores (_), dashes (-), spaces ( ), and must not start or end with spaces.
        ///
        ///
        /// - - -
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// Documentation describing the data exchange.
        pub documentation: pulumi_gestalt_rust::Output<Option<String>>,
        /// Base64 encoded image representing the data exchange.
        pub icon: pulumi_gestalt_rust::Output<Option<String>>,
        /// Number of listings contained in the data exchange.
        pub listing_count: pulumi_gestalt_rust::Output<i32>,
        /// The name of the location this data exchange.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the data exchange, for example:
        /// "projects/myproject/locations/US/dataExchanges/123"
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Email or URL of the primary point of contact of the data exchange.
        pub primary_contact: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Configurable data sharing environment option for a data exchange.
        /// This field is required for data clean room exchanges.
        /// Structure is documented below.
        pub sharing_environment_config: pulumi_gestalt_rust::Output<
            super::super::types::bigqueryanalyticshub::DataExchangeSharingEnvironmentConfig,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DataExchangeArgs,
    ) -> DataExchangeResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let data_exchange_id_binding = args.data_exchange_id.get_output(context);
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let documentation_binding = args.documentation.get_output(context);
        let icon_binding = args.icon.get_output(context);
        let location_binding = args.location.get_output(context);
        let primary_contact_binding = args.primary_contact.get_output(context);
        let project_binding = args.project.get_output(context);
        let sharing_environment_config_binding = args
            .sharing_environment_config
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:bigqueryanalyticshub/dataExchange:DataExchange".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataExchangeId".into(),
                    value: data_exchange_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "documentation".into(),
                    value: documentation_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "icon".into(),
                    value: icon_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "primaryContact".into(),
                    value: primary_contact_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sharingEnvironmentConfig".into(),
                    value: sharing_environment_config_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DataExchangeResult {
            data_exchange_id: o.get_field("dataExchangeId"),
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            documentation: o.get_field("documentation"),
            icon: o.get_field("icon"),
            listing_count: o.get_field("listingCount"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            primary_contact: o.get_field("primaryContact"),
            project: o.get_field("project"),
            sharing_environment_config: o.get_field("sharingEnvironmentConfig"),
        }
    }
}
