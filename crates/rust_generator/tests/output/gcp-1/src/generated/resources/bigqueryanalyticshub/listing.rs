/// A Bigquery Analytics Hub data exchange listing
///
///
/// To get more information about Listing, see:
///
/// * [API documentation](https://cloud.google.com/bigquery/docs/reference/analytics-hub/rest/v1/projects.locations.dataExchanges.listings)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/bigquery/docs/analytics-hub-introduction)
///
/// ## Example Usage
///
/// ### Bigquery Analyticshub Listing Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let listing = data_exchange::create(
///         "listing",
///         DataExchangeArgs::builder()
///             .data_exchange_id("my_data_exchange")
///             .description("example data exchange")
///             .display_name("my_data_exchange")
///             .location("US")
///             .build_struct(),
///     );
///     let listingDataset = dataset::create(
///         "listingDataset",
///         DatasetArgs::builder()
///             .dataset_id("my_listing")
///             .description("example data exchange")
///             .friendly_name("my_listing")
///             .location("US")
///             .build_struct(),
///     );
///     let listingListing = listing::create(
///         "listingListing",
///         ListingArgs::builder()
///             .bigquery_dataset(
///                 ListingBigqueryDataset::builder()
///                     .dataset("${listingDataset.id}")
///                     .build_struct(),
///             )
///             .data_exchange_id("${listing.dataExchangeId}")
///             .description("example data exchange")
///             .display_name("my_listing")
///             .listing_id("my_listing")
///             .location("US")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Bigquery Analyticshub Listing Restricted
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let listing = data_exchange::create(
///         "listing",
///         DataExchangeArgs::builder()
///             .data_exchange_id("my_data_exchange")
///             .description("example data exchange")
///             .display_name("my_data_exchange")
///             .location("US")
///             .build_struct(),
///     );
///     let listingDataset = dataset::create(
///         "listingDataset",
///         DatasetArgs::builder()
///             .dataset_id("my_listing")
///             .description("example data exchange")
///             .friendly_name("my_listing")
///             .location("US")
///             .build_struct(),
///     );
///     let listingListing = listing::create(
///         "listingListing",
///         ListingArgs::builder()
///             .bigquery_dataset(
///                 ListingBigqueryDataset::builder()
///                     .dataset("${listingDataset.id}")
///                     .build_struct(),
///             )
///             .data_exchange_id("${listing.dataExchangeId}")
///             .description("example data exchange")
///             .display_name("my_listing")
///             .listing_id("my_listing")
///             .location("US")
///             .restricted_export_config(
///                 ListingRestrictedExportConfig::builder()
///                     .enabled(true)
///                     .restrictQueryResult(true)
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Bigquery Analyticshub Listing Dcr
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let listing = data_exchange::create(
///         "listing",
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
///     let listingDataset = dataset::create(
///         "listingDataset",
///         DatasetArgs::builder()
///             .dataset_id("dcr_listing")
///             .description("example dcr data exchange")
///             .friendly_name("dcr_listing")
///             .location("US")
///             .build_struct(),
///     );
///     let listingListing = listing::create(
///         "listingListing",
///         ListingArgs::builder()
///             .bigquery_dataset(
///                 ListingBigqueryDataset::builder()
///                     .dataset("${listingDataset.id}")
///                     .selectedResources(
///                         vec![
///                             ListingBigqueryDatasetSelectedResource::builder()
///                             .table("${listingTable.id}").build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .data_exchange_id("${listing.dataExchangeId}")
///             .description("example dcr data exchange")
///             .display_name("dcr_listing")
///             .listing_id("dcr_listing")
///             .location("US")
///             .restricted_export_config(
///                 ListingRestrictedExportConfig::builder().enabled(true).build_struct(),
///             )
///             .build_struct(),
///     );
///     let listingTable = table::create(
///         "listingTable",
///         TableArgs::builder()
///             .dataset_id("${listingDataset.datasetId}")
///             .deletion_protection(false)
///             .schema(
///                 "[\n  {\n    \"name\": \"name\",\n    \"type\": \"STRING\",\n    \"mode\": \"NULLABLE\"\n  },\n  {\n    \"name\": \"post_abbr\",\n    \"type\": \"STRING\",\n    \"mode\": \"NULLABLE\"\n  },\n  {\n    \"name\": \"date\",\n    \"type\": \"DATE\",\n    \"mode\": \"NULLABLE\"\n  }\n]",
///             )
///             .table_id("dcr_listing")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Listing can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/dataExchanges/{{data_exchange_id}}/listings/{{listing_id}}`
///
/// * `{{project}}/{{location}}/{{data_exchange_id}}/{{listing_id}}`
///
/// * `{{location}}/{{data_exchange_id}}/{{listing_id}}`
///
/// When using the `pulumi import` command, Listing can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:bigqueryanalyticshub/listing:Listing default projects/{{project}}/locations/{{location}}/dataExchanges/{{data_exchange_id}}/listings/{{listing_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:bigqueryanalyticshub/listing:Listing default {{project}}/{{location}}/{{data_exchange_id}}/{{listing_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:bigqueryanalyticshub/listing:Listing default {{location}}/{{data_exchange_id}}/{{listing_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod listing {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ListingArgs {
        /// Shared dataset i.e. BigQuery dataset source.
        /// Structure is documented below.
        #[builder(into)]
        pub bigquery_dataset: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::bigqueryanalyticshub::ListingBigqueryDataset,
        >,
        /// Categories of the listing. Up to two categories are allowed.
        #[builder(into, default)]
        pub categories: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The ID of the data exchange. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces.
        #[builder(into)]
        pub data_exchange_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Details of the data provider who owns the source data.
        #[builder(into, default)]
        pub data_provider: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bigqueryanalyticshub::ListingDataProvider>,
        >,
        /// Short description of the listing. The description must not contain Unicode non-characters and C0 and C1 control codes
        /// except tabs (HT), new lines (LF), carriage returns (CR), and page breaks (FF).
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Human-readable display name of the listing. The display name must contain only Unicode letters, numbers (0-9), underscores (_), dashes (-), spaces ( ), ampersands (&) and can't start or end with spaces.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Documentation describing the listing.
        #[builder(into, default)]
        pub documentation: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Base64 encoded image representing the listing.
        #[builder(into, default)]
        pub icon: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the listing. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces.
        #[builder(into)]
        pub listing_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the location this data exchange listing.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Email or URL of the primary point of contact of the listing.
        #[builder(into, default)]
        pub primary_contact: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Details of the publisher who owns the listing and who can share the source data.
        #[builder(into, default)]
        pub publisher: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bigqueryanalyticshub::ListingPublisher>,
        >,
        /// Email or URL of the request access of the listing. Subscribers can use this reference to request access.
        #[builder(into, default)]
        pub request_access: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If set, restricted export configuration will be propagated and enforced on the linked dataset.
        #[builder(into, default)]
        pub restricted_export_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::bigqueryanalyticshub::ListingRestrictedExportConfig,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct ListingResult {
        /// Shared dataset i.e. BigQuery dataset source.
        /// Structure is documented below.
        pub bigquery_dataset: pulumi_gestalt_rust::Output<
            super::super::types::bigqueryanalyticshub::ListingBigqueryDataset,
        >,
        /// Categories of the listing. Up to two categories are allowed.
        pub categories: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The ID of the data exchange. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces.
        pub data_exchange_id: pulumi_gestalt_rust::Output<String>,
        /// Details of the data provider who owns the source data.
        pub data_provider: pulumi_gestalt_rust::Output<
            Option<super::super::types::bigqueryanalyticshub::ListingDataProvider>,
        >,
        /// Short description of the listing. The description must not contain Unicode non-characters and C0 and C1 control codes
        /// except tabs (HT), new lines (LF), carriage returns (CR), and page breaks (FF).
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Human-readable display name of the listing. The display name must contain only Unicode letters, numbers (0-9), underscores (_), dashes (-), spaces ( ), ampersands (&) and can't start or end with spaces.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// Documentation describing the listing.
        pub documentation: pulumi_gestalt_rust::Output<Option<String>>,
        /// Base64 encoded image representing the listing.
        pub icon: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the listing. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces.
        pub listing_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the location this data exchange listing.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the listing. e.g. "projects/myproject/locations/US/dataExchanges/123/listings/456"
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Email or URL of the primary point of contact of the listing.
        pub primary_contact: pulumi_gestalt_rust::Output<Option<String>>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Details of the publisher who owns the listing and who can share the source data.
        pub publisher: pulumi_gestalt_rust::Output<
            Option<super::super::types::bigqueryanalyticshub::ListingPublisher>,
        >,
        /// Email or URL of the request access of the listing. Subscribers can use this reference to request access.
        pub request_access: pulumi_gestalt_rust::Output<Option<String>>,
        /// If set, restricted export configuration will be propagated and enforced on the linked dataset.
        pub restricted_export_config: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::bigqueryanalyticshub::ListingRestrictedExportConfig,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ListingArgs,
    ) -> ListingResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bigquery_dataset_binding = args.bigquery_dataset.get_output(context);
        let categories_binding = args.categories.get_output(context);
        let data_exchange_id_binding = args.data_exchange_id.get_output(context);
        let data_provider_binding = args.data_provider.get_output(context);
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let documentation_binding = args.documentation.get_output(context);
        let icon_binding = args.icon.get_output(context);
        let listing_id_binding = args.listing_id.get_output(context);
        let location_binding = args.location.get_output(context);
        let primary_contact_binding = args.primary_contact.get_output(context);
        let project_binding = args.project.get_output(context);
        let publisher_binding = args.publisher.get_output(context);
        let request_access_binding = args.request_access.get_output(context);
        let restricted_export_config_binding = args
            .restricted_export_config
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:bigqueryanalyticshub/listing:Listing".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bigqueryDataset".into(),
                    value: &bigquery_dataset_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "categories".into(),
                    value: &categories_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataExchangeId".into(),
                    value: &data_exchange_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataProvider".into(),
                    value: &data_provider_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "documentation".into(),
                    value: &documentation_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "icon".into(),
                    value: &icon_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "listingId".into(),
                    value: &listing_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "primaryContact".into(),
                    value: &primary_contact_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publisher".into(),
                    value: &publisher_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requestAccess".into(),
                    value: &request_access_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "restrictedExportConfig".into(),
                    value: &restricted_export_config_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ListingResult {
            bigquery_dataset: o.get_field("bigqueryDataset"),
            categories: o.get_field("categories"),
            data_exchange_id: o.get_field("dataExchangeId"),
            data_provider: o.get_field("dataProvider"),
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            documentation: o.get_field("documentation"),
            icon: o.get_field("icon"),
            listing_id: o.get_field("listingId"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            primary_contact: o.get_field("primaryContact"),
            project: o.get_field("project"),
            publisher: o.get_field("publisher"),
            request_access: o.get_field("requestAccess"),
            restricted_export_config: o.get_field("restrictedExportConfig"),
        }
    }
}
