/// Resource for managing an AWS FinSpace Kx Dataview.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod kx_dataview {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KxDataviewArgs {
        /// The option to specify whether you want to apply all the future additions and corrections automatically to the dataview, when you ingest new changesets. The default value is false.
        #[builder(into)]
        pub auto_update: pulumi_wasm_rust::Output<bool>,
        /// The identifier of the availability zones. If attaching a volume, the volume must be in the same availability zone as the dataview that you are attaching to.
        #[builder(into, default)]
        pub availability_zone_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The number of availability zones you want to assign per cluster. This can be one of the following:
        /// * `SINGLE` - Assigns one availability zone per cluster.
        /// * `MULTI` - Assigns all the availability zones per cluster.
        #[builder(into)]
        pub az_mode: pulumi_wasm_rust::Output<String>,
        /// A unique identifier of the changeset of the database that you want to use to ingest data.
        #[builder(into, default)]
        pub changeset_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the database where you want to create a dataview.
        #[builder(into)]
        pub database_name: pulumi_wasm_rust::Output<String>,
        /// A description for the dataview.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Unique identifier for the KX environment.
        #[builder(into)]
        pub environment_id: pulumi_wasm_rust::Output<String>,
        /// A unique identifier for the dataview.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The option to specify whether you want to make the dataview writable to perform database maintenance. The following are some considerations related to writable dataviews.
        /// * You cannot create partial writable dataviews. When you create writeable dataviews you must provide the entire database path. You cannot perform updates on a writeable dataview. Hence, `auto_update` must be set as `false` if `read_write` is `true` for a dataview.
        /// * You must also use a unique volume for creating a writeable dataview. So, if you choose a volume that is already in use by another dataview, the dataview creation fails.
        /// * Once you create a dataview as writeable, you cannot change it to read-only. So, you cannot update the `read_write` parameter later.
        #[builder(into, default)]
        pub read_write: pulumi_wasm_rust::Output<Option<bool>>,
        /// The configuration that contains the database path of the data that you want to place on each selected volume. Each segment must have a unique database path for each volume. If you do not explicitly specify any database path for a volume, they are accessible from the cluster through the default S3/object store segment. See segment_configurations below.
        #[builder(into, default)]
        pub segment_configurations: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::finspace::KxDataviewSegmentConfiguration>>,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct KxDataviewResult {
        /// Amazon Resource Name (ARN) identifier of the KX dataview.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The option to specify whether you want to apply all the future additions and corrections automatically to the dataview, when you ingest new changesets. The default value is false.
        pub auto_update: pulumi_wasm_rust::Output<bool>,
        /// The identifier of the availability zones. If attaching a volume, the volume must be in the same availability zone as the dataview that you are attaching to.
        pub availability_zone_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The number of availability zones you want to assign per cluster. This can be one of the following:
        /// * `SINGLE` - Assigns one availability zone per cluster.
        /// * `MULTI` - Assigns all the availability zones per cluster.
        pub az_mode: pulumi_wasm_rust::Output<String>,
        /// A unique identifier of the changeset of the database that you want to use to ingest data.
        pub changeset_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Timestamp at which the dataview was created in FinSpace. Value determined as epoch time in milliseconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000000.
        pub created_timestamp: pulumi_wasm_rust::Output<String>,
        /// The name of the database where you want to create a dataview.
        pub database_name: pulumi_wasm_rust::Output<String>,
        /// A description for the dataview.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Unique identifier for the KX environment.
        pub environment_id: pulumi_wasm_rust::Output<String>,
        /// The last time that the dataview was updated in FinSpace. The value is determined as epoch time in milliseconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000000.
        pub last_modified_timestamp: pulumi_wasm_rust::Output<String>,
        /// A unique identifier for the dataview.
        ///
        /// The following arguments are optional:
        pub name: pulumi_wasm_rust::Output<String>,
        /// The option to specify whether you want to make the dataview writable to perform database maintenance. The following are some considerations related to writable dataviews.
        /// * You cannot create partial writable dataviews. When you create writeable dataviews you must provide the entire database path. You cannot perform updates on a writeable dataview. Hence, `auto_update` must be set as `false` if `read_write` is `true` for a dataview.
        /// * You must also use a unique volume for creating a writeable dataview. So, if you choose a volume that is already in use by another dataview, the dataview creation fails.
        /// * Once you create a dataview as writeable, you cannot change it to read-only. So, you cannot update the `read_write` parameter later.
        pub read_write: pulumi_wasm_rust::Output<Option<bool>>,
        /// The configuration that contains the database path of the data that you want to place on each selected volume. Each segment must have a unique database path for each volume. If you do not explicitly specify any database path for a volume, they are accessible from the cluster through the default S3/object store segment. See segment_configurations below.
        pub segment_configurations: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::finspace::KxDataviewSegmentConfiguration>>,
        >,
        pub status: pulumi_wasm_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: KxDataviewArgs) -> KxDataviewResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let auto_update_binding = args.auto_update.get_inner();
        let availability_zone_id_binding = args.availability_zone_id.get_inner();
        let az_mode_binding = args.az_mode.get_inner();
        let changeset_id_binding = args.changeset_id.get_inner();
        let database_name_binding = args.database_name.get_inner();
        let description_binding = args.description.get_inner();
        let environment_id_binding = args.environment_id.get_inner();
        let name_binding = args.name.get_inner();
        let read_write_binding = args.read_write.get_inner();
        let segment_configurations_binding = args.segment_configurations.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:finspace/kxDataview:KxDataview".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoUpdate".into(),
                    value: &auto_update_binding,
                },
                register_interface::ObjectField {
                    name: "availabilityZoneId".into(),
                    value: &availability_zone_id_binding,
                },
                register_interface::ObjectField {
                    name: "azMode".into(),
                    value: &az_mode_binding,
                },
                register_interface::ObjectField {
                    name: "changesetId".into(),
                    value: &changeset_id_binding,
                },
                register_interface::ObjectField {
                    name: "databaseName".into(),
                    value: &database_name_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "environmentId".into(),
                    value: &environment_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "readWrite".into(),
                    value: &read_write_binding,
                },
                register_interface::ObjectField {
                    name: "segmentConfigurations".into(),
                    value: &segment_configurations_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "autoUpdate".into(),
                },
                register_interface::ResultField {
                    name: "availabilityZoneId".into(),
                },
                register_interface::ResultField {
                    name: "azMode".into(),
                },
                register_interface::ResultField {
                    name: "changesetId".into(),
                },
                register_interface::ResultField {
                    name: "createdTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "databaseName".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "environmentId".into(),
                },
                register_interface::ResultField {
                    name: "lastModifiedTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "readWrite".into(),
                },
                register_interface::ResultField {
                    name: "segmentConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        KxDataviewResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            auto_update: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoUpdate").unwrap(),
            ),
            availability_zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZoneId").unwrap(),
            ),
            az_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("azMode").unwrap(),
            ),
            changeset_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("changesetId").unwrap(),
            ),
            created_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdTimestamp").unwrap(),
            ),
            database_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("databaseName").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            environment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("environmentId").unwrap(),
            ),
            last_modified_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastModifiedTimestamp").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            read_write: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("readWrite").unwrap(),
            ),
            segment_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("segmentConfigurations").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}