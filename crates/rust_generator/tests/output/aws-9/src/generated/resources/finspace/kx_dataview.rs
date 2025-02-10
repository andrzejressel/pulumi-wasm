/// Resource for managing an AWS FinSpace Kx Dataview.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = kx_dataview::create(
///         "example",
///         KxDataviewArgs::builder()
///             .auto_update(true)
///             .availability_zone_id("use1-az2")
///             .az_mode("SINGLE")
///             .database_name("${exampleAwsFinspaceKxDatabase.name}")
///             .description("Terraform managed Kx Dataview")
///             .environment_id("${exampleAwsFinspaceKxEnvironment.id}")
///             .name("my-tf-kx-dataview")
///             .segment_configurations(
///                 vec![
///                     KxDataviewSegmentConfiguration::builder().dbPaths(vec!["/*",])
///                     .volumeName("${exampleAwsFinspaceKxVolume.name}").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import an AWS FinSpace Kx Cluster using the `id` (environment ID and cluster name, comma-delimited). For example:
///
/// ```sh
/// $ pulumi import aws:finspace/kxDataview:KxDataview example n3ceo7wqxoxcti5tujqwzs,my-tf-kx-database,my-tf-kx-dataview
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod kx_dataview {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KxDataviewArgs {
        /// The option to specify whether you want to apply all the future additions and corrections automatically to the dataview, when you ingest new changesets. The default value is false.
        #[builder(into)]
        pub auto_update: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// The identifier of the availability zones. If attaching a volume, the volume must be in the same availability zone as the dataview that you are attaching to.
        #[builder(into, default)]
        pub availability_zone_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The number of availability zones you want to assign per cluster. This can be one of the following:
        /// * `SINGLE` - Assigns one availability zone per cluster.
        /// * `MULTI` - Assigns all the availability zones per cluster.
        #[builder(into)]
        pub az_mode: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A unique identifier of the changeset of the database that you want to use to ingest data.
        #[builder(into, default)]
        pub changeset_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the database where you want to create a dataview.
        #[builder(into)]
        pub database_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A description for the dataview.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Unique identifier for the KX environment.
        #[builder(into)]
        pub environment_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A unique identifier for the dataview.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The option to specify whether you want to make the dataview writable to perform database maintenance. The following are some considerations related to writable dataviews.
        /// * You cannot create partial writable dataviews. When you create writeable dataviews you must provide the entire database path. You cannot perform updates on a writeable dataview. Hence, `auto_update` must be set as `false` if `read_write` is `true` for a dataview.
        /// * You must also use a unique volume for creating a writeable dataview. So, if you choose a volume that is already in use by another dataview, the dataview creation fails.
        /// * Once you create a dataview as writeable, you cannot change it to read-only. So, you cannot update the `read_write` parameter later.
        #[builder(into, default)]
        pub read_write: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The configuration that contains the database path of the data that you want to place on each selected volume. Each segment must have a unique database path for each volume. If you do not explicitly specify any database path for a volume, they are accessible from the cluster through the default S3/object store segment. See segment_configurations below.
        #[builder(into, default)]
        pub segment_configurations: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::finspace::KxDataviewSegmentConfiguration>>,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct KxDataviewResult {
        /// Amazon Resource Name (ARN) identifier of the KX dataview.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The option to specify whether you want to apply all the future additions and corrections automatically to the dataview, when you ingest new changesets. The default value is false.
        pub auto_update: pulumi_gestalt_rust::Output<bool>,
        /// The identifier of the availability zones. If attaching a volume, the volume must be in the same availability zone as the dataview that you are attaching to.
        pub availability_zone_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The number of availability zones you want to assign per cluster. This can be one of the following:
        /// * `SINGLE` - Assigns one availability zone per cluster.
        /// * `MULTI` - Assigns all the availability zones per cluster.
        pub az_mode: pulumi_gestalt_rust::Output<String>,
        /// A unique identifier of the changeset of the database that you want to use to ingest data.
        pub changeset_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Timestamp at which the dataview was created in FinSpace. Value determined as epoch time in milliseconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000000.
        pub created_timestamp: pulumi_gestalt_rust::Output<String>,
        /// The name of the database where you want to create a dataview.
        pub database_name: pulumi_gestalt_rust::Output<String>,
        /// A description for the dataview.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Unique identifier for the KX environment.
        pub environment_id: pulumi_gestalt_rust::Output<String>,
        /// The last time that the dataview was updated in FinSpace. The value is determined as epoch time in milliseconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000000.
        pub last_modified_timestamp: pulumi_gestalt_rust::Output<String>,
        /// A unique identifier for the dataview.
        ///
        /// The following arguments are optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The option to specify whether you want to make the dataview writable to perform database maintenance. The following are some considerations related to writable dataviews.
        /// * You cannot create partial writable dataviews. When you create writeable dataviews you must provide the entire database path. You cannot perform updates on a writeable dataview. Hence, `auto_update` must be set as `false` if `read_write` is `true` for a dataview.
        /// * You must also use a unique volume for creating a writeable dataview. So, if you choose a volume that is already in use by another dataview, the dataview creation fails.
        /// * Once you create a dataview as writeable, you cannot change it to read-only. So, you cannot update the `read_write` parameter later.
        pub read_write: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The configuration that contains the database path of the data that you want to place on each selected volume. Each segment must have a unique database path for each volume. If you do not explicitly specify any database path for a volume, they are accessible from the cluster through the default S3/object store segment. See segment_configurations below.
        pub segment_configurations: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::finspace::KxDataviewSegmentConfiguration>>,
        >,
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: KxDataviewArgs,
    ) -> KxDataviewResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let auto_update_binding = args.auto_update.get_output(context);
        let availability_zone_id_binding = args.availability_zone_id.get_output(context);
        let az_mode_binding = args.az_mode.get_output(context);
        let changeset_id_binding = args.changeset_id.get_output(context);
        let database_name_binding = args.database_name.get_output(context);
        let description_binding = args.description.get_output(context);
        let environment_id_binding = args.environment_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let read_write_binding = args.read_write.get_output(context);
        let segment_configurations_binding = args
            .segment_configurations
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:finspace/kxDataview:KxDataview".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoUpdate".into(),
                    value: auto_update_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "availabilityZoneId".into(),
                    value: availability_zone_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "azMode".into(),
                    value: az_mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "changesetId".into(),
                    value: changeset_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "databaseName".into(),
                    value: database_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "environmentId".into(),
                    value: environment_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "readWrite".into(),
                    value: read_write_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "segmentConfigurations".into(),
                    value: segment_configurations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        KxDataviewResult {
            arn: o.get_field("arn"),
            auto_update: o.get_field("autoUpdate"),
            availability_zone_id: o.get_field("availabilityZoneId"),
            az_mode: o.get_field("azMode"),
            changeset_id: o.get_field("changesetId"),
            created_timestamp: o.get_field("createdTimestamp"),
            database_name: o.get_field("databaseName"),
            description: o.get_field("description"),
            environment_id: o.get_field("environmentId"),
            last_modified_timestamp: o.get_field("lastModifiedTimestamp"),
            name: o.get_field("name"),
            read_write: o.get_field("readWrite"),
            segment_configurations: o.get_field("segmentConfigurations"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
