/// Resource for managing an AWS FinSpace Kx Volume.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:finspace:KxVolume
///     properties:
///       name: my-tf-kx-volume
///       environmentId: ${exampleAwsFinspaceKxEnvironment.id}
///       availabilityZones: use1-az2
///       azMode: SINGLE
///       type: NAS_1
///       nas1Configurations:
///         - size: 1200
///           type: SSD_250
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import an AWS FinSpace Kx Volume using the `id` (environment ID and volume name, comma-delimited). For example:
///
/// ```sh
/// $ pulumi import aws:finspace/kxVolume:KxVolume example n3ceo7wqxoxcti5tujqwzs,my-tf-kx-volume
/// ```
pub mod kx_volume {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KxVolumeArgs {
        /// The identifier of the AWS Availability Zone IDs.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub availability_zones: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// The number of availability zones you want to assign per volume. Currently, Finspace only support SINGLE for volumes.
        /// * `SINGLE` - Assigns one availability zone per volume.
        #[builder(into)]
        pub az_mode: pulumi_wasm_rust::InputOrOutput<String>,
        /// Description of the volume.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A unique identifier for the kdb environment, whose clusters can attach to the volume.
        #[builder(into)]
        pub environment_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Unique name for the volumr that you want to create.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the configuration for the Network attached storage (`NAS_1`) file system volume. This parameter is required when `volume_type` is `NAS_1`. See `nas1_configuration` Argument Reference below.
        #[builder(into, default)]
        pub nas1_configurations: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::finspace::KxVolumeNas1Configuration>>,
        >,
        /// A list of key-value pairs to label the volume. You can add up to 50 tags to a volume
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of file system volume. Currently, FinSpace only supports the `NAS_1` volume type. When you select the `NAS_1` volume type, you must also provide `nas1_configuration`.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct KxVolumeResult {
        /// Amazon Resource Name (ARN) identifier of the KX volume.
        pub arn: pulumi_wasm_rust::Output<String>,
        pub attached_clusters: pulumi_wasm_rust::Output<
            Vec<super::super::types::finspace::KxVolumeAttachedCluster>,
        >,
        /// The identifier of the AWS Availability Zone IDs.
        ///
        /// The following arguments are optional:
        pub availability_zones: pulumi_wasm_rust::Output<Vec<String>>,
        /// The number of availability zones you want to assign per volume. Currently, Finspace only support SINGLE for volumes.
        /// * `SINGLE` - Assigns one availability zone per volume.
        pub az_mode: pulumi_wasm_rust::Output<String>,
        /// The timestamp at which the volume was created in FinSpace. The value is determined as epoch time in milliseconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000000.
        pub created_timestamp: pulumi_wasm_rust::Output<String>,
        /// Description of the volume.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A unique identifier for the kdb environment, whose clusters can attach to the volume.
        pub environment_id: pulumi_wasm_rust::Output<String>,
        /// Last timestamp at which the volume was updated in FinSpace. Value determined as epoch time in seconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000.
        pub last_modified_timestamp: pulumi_wasm_rust::Output<String>,
        /// Unique name for the volumr that you want to create.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the configuration for the Network attached storage (`NAS_1`) file system volume. This parameter is required when `volume_type` is `NAS_1`. See `nas1_configuration` Argument Reference below.
        pub nas1_configurations: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::finspace::KxVolumeNas1Configuration>>,
        >,
        /// The status of volume creation.
        /// * `CREATING` – The volume creation is in progress.
        /// * `CREATE_FAILED` – The volume creation has failed.
        /// * `ACTIVE` – The volume is active.
        /// * `UPDATING` – The volume is in the process of being updated.
        /// * `UPDATE_FAILED` – The update action failed.
        /// * `UPDATED` – The volume is successfully updated.
        /// * `DELETING` – The volume is in the process of being deleted.
        /// * `DELETE_FAILED` – The system failed to delete the volume.
        /// * `DELETED` – The volume is successfully deleted.
        pub status: pulumi_wasm_rust::Output<String>,
        /// The error message when a failed state occurs.
        pub status_reason: pulumi_wasm_rust::Output<String>,
        /// A list of key-value pairs to label the volume. You can add up to 50 tags to a volume
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The type of file system volume. Currently, FinSpace only supports the `NAS_1` volume type. When you select the `NAS_1` volume type, you must also provide `nas1_configuration`.
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: KxVolumeArgs,
    ) -> KxVolumeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let availability_zones_binding = args
            .availability_zones
            .get_output(context)
            .get_inner();
        let az_mode_binding = args.az_mode.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let environment_id_binding = args.environment_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let nas1_configurations_binding = args
            .nas1_configurations
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:finspace/kxVolume:KxVolume".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "availabilityZones".into(),
                    value: &availability_zones_binding,
                },
                register_interface::ObjectField {
                    name: "azMode".into(),
                    value: &az_mode_binding,
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
                    name: "nas1Configurations".into(),
                    value: &nas1_configurations_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "attachedClusters".into(),
                },
                register_interface::ResultField {
                    name: "availabilityZones".into(),
                },
                register_interface::ResultField {
                    name: "azMode".into(),
                },
                register_interface::ResultField {
                    name: "createdTimestamp".into(),
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
                    name: "nas1Configurations".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "statusReason".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        KxVolumeResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            attached_clusters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("attachedClusters").unwrap(),
            ),
            availability_zones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZones").unwrap(),
            ),
            az_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("azMode").unwrap(),
            ),
            created_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdTimestamp").unwrap(),
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
            nas1_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nas1Configurations").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            status_reason: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statusReason").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
