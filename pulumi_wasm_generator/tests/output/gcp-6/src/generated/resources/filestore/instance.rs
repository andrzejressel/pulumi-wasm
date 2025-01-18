/// A Google Cloud Filestore instance.
///
///
/// To get more information about Instance, see:
///
/// * [API documentation](https://cloud.google.com/filestore/docs/reference/rest/v1beta1/projects.locations.instances/create)
/// * How-to Guides
///     * [Copying Data In/Out](https://cloud.google.com/filestore/docs/copying-data)
///     * [Official Documentation](https://cloud.google.com/filestore/docs/creating-instances)
///     * [Use with Kubernetes](https://cloud.google.com/filestore/docs/accessing-fileshares)
///
/// ## Example Usage
///
/// ### Filestore Instance Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let instance = instance::create(
///         "instance",
///         InstanceArgs::builder()
///             .file_shares(
///                 InstanceFileShares::builder()
///                     .capacityGb(1024)
///                     .name("share1")
///                     .build_struct(),
///             )
///             .location("us-central1-b")
///             .name("test-instance")
///             .networks(
///                 vec![
///                     InstanceNetwork::builder().modes(vec!["MODE_IPV4",])
///                     .network("default").build_struct(),
///                 ],
///             )
///             .tier("BASIC_HDD")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Filestore Instance Full
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let instance = instance::create(
///         "instance",
///         InstanceArgs::builder()
///             .file_shares(
///                 InstanceFileShares::builder()
///                     .capacityGb(2560)
///                     .name("share1")
///                     .nfsExportOptions(
///                         vec![
///                             InstanceFileSharesNfsExportOption::builder()
///                             .accessMode("READ_WRITE").ipRanges(vec!["10.0.0.0/24",])
///                             .squashMode("NO_ROOT_SQUASH").build_struct(),
///                             InstanceFileSharesNfsExportOption::builder()
///                             .accessMode("READ_ONLY").anonGid(456).anonUid(123)
///                             .ipRanges(vec!["10.10.0.0/24",]).squashMode("ROOT_SQUASH")
///                             .build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .location("us-central1-b")
///             .name("test-instance")
///             .networks(
///                 vec![
///                     InstanceNetwork::builder().connectMode("DIRECT_PEERING")
///                     .modes(vec!["MODE_IPV4",]).network("default").build_struct(),
///                 ],
///             )
///             .tier("BASIC_SSD")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Filestore Instance Protocol
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let instance = instance::create(
///         "instance",
///         InstanceArgs::builder()
///             .file_shares(
///                 InstanceFileShares::builder()
///                     .capacityGb(1024)
///                     .name("share1")
///                     .build_struct(),
///             )
///             .location("us-central1")
///             .name("test-instance")
///             .networks(
///                 vec![
///                     InstanceNetwork::builder().modes(vec!["MODE_IPV4",])
///                     .network("default").build_struct(),
///                 ],
///             )
///             .protocol("NFS_V4_1")
///             .tier("ENTERPRISE")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Filestore Instance Enterprise
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let filestoreKey = crypto_key::create(
///         "filestoreKey",
///         CryptoKeyArgs::builder()
///             .key_ring("${filestoreKeyring.id}")
///             .name("filestore-key")
///             .build_struct(),
///     );
///     let filestoreKeyring = key_ring::create(
///         "filestoreKeyring",
///         KeyRingArgs::builder()
///             .location("us-central1")
///             .name("filestore-keyring")
///             .build_struct(),
///     );
///     let instance = instance::create(
///         "instance",
///         InstanceArgs::builder()
///             .file_shares(
///                 InstanceFileShares::builder()
///                     .capacityGb(1024)
///                     .name("share1")
///                     .build_struct(),
///             )
///             .kms_key_name("${filestoreKey.id}")
///             .location("us-central1")
///             .name("test-instance")
///             .networks(
///                 vec![
///                     InstanceNetwork::builder().modes(vec!["MODE_IPV4",])
///                     .network("default").build_struct(),
///                 ],
///             )
///             .tier("ENTERPRISE")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Instance can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/instances/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, Instance can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:filestore/instance:Instance default projects/{{project}}/locations/{{location}}/instances/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:filestore/instance:Instance default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:filestore/instance:Instance default {{location}}/{{name}}
/// ```
///
pub mod instance {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceArgs {
        /// Indicates whether the instance is protected against deletion.
        #[builder(into, default)]
        pub deletion_protection_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The reason for enabling deletion protection.
        #[builder(into, default)]
        pub deletion_protection_reason: pulumi_wasm_rust::Output<Option<String>>,
        /// A description of the instance.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// File system shares on the instance. For this version, only a
        /// single file share is supported.
        /// Structure is documented below.
        #[builder(into)]
        pub file_shares: pulumi_wasm_rust::Output<
            super::super::types::filestore::InstanceFileShares,
        >,
        /// KMS key name used for data encryption.
        #[builder(into, default)]
        pub kms_key_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Resource labels to represent user-provided metadata. **Note**: This field is non-authoritative, and will only manage the
        /// labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels present on the
        /// resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the location of the instance. This can be a region for ENTERPRISE tier instances.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The resource name of the instance.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// VPC networks to which the instance is connected. For this version,
        /// only a single network is supported.
        /// Structure is documented below.
        #[builder(into)]
        pub networks: pulumi_wasm_rust::Output<
            Vec<super::super::types::filestore::InstanceNetwork>,
        >,
        /// Performance configuration for the instance. If not provided, the default performance settings will be used.
        #[builder(into, default)]
        pub performance_config: pulumi_wasm_rust::Output<
            Option<super::super::types::filestore::InstancePerformanceConfig>,
        >,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Either NFSv3, for using NFS version 3 as file sharing protocol, or NFSv4.1, for using NFS version 4.1 as file sharing
        /// protocol. NFSv4.1 can be used with HIGH_SCALE_SSD, ZONAL, REGIONAL and ENTERPRISE. The default is NFSv3. Default value:
        /// "NFS_V3" Possible values: ["NFS_V3", "NFS_V4_1"]
        #[builder(into, default)]
        pub protocol: pulumi_wasm_rust::Output<Option<String>>,
        /// The service tier of the instance.
        /// Possible values include: STANDARD, PREMIUM, BASIC_HDD, BASIC_SSD, HIGH_SCALE_SSD, ZONAL, REGIONAL and ENTERPRISE
        #[builder(into)]
        pub tier: pulumi_wasm_rust::Output<String>,
        /// The name of the Filestore zone of the instance.
        #[builder(into, default)]
        pub zone: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct InstanceResult {
        /// Creation timestamp in RFC3339 text format.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Indicates whether the instance is protected against deletion.
        pub deletion_protection_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The reason for enabling deletion protection.
        pub deletion_protection_reason: pulumi_wasm_rust::Output<Option<String>>,
        /// A description of the instance.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Server-specified ETag for the instance resource to prevent
        /// simultaneous updates from overwriting each other.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// File system shares on the instance. For this version, only a
        /// single file share is supported.
        /// Structure is documented below.
        pub file_shares: pulumi_wasm_rust::Output<
            super::super::types::filestore::InstanceFileShares,
        >,
        /// KMS key name used for data encryption.
        pub kms_key_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Resource labels to represent user-provided metadata. **Note**: This field is non-authoritative, and will only manage the
        /// labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels present on the
        /// resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the location of the instance. This can be a region for ENTERPRISE tier instances.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The resource name of the instance.
        pub name: pulumi_wasm_rust::Output<String>,
        /// VPC networks to which the instance is connected. For this version,
        /// only a single network is supported.
        /// Structure is documented below.
        pub networks: pulumi_wasm_rust::Output<
            Vec<super::super::types::filestore::InstanceNetwork>,
        >,
        /// Performance configuration for the instance. If not provided, the default performance settings will be used.
        pub performance_config: pulumi_wasm_rust::Output<
            Option<super::super::types::filestore::InstancePerformanceConfig>,
        >,
        pub project: pulumi_wasm_rust::Output<String>,
        /// Either NFSv3, for using NFS version 3 as file sharing protocol, or NFSv4.1, for using NFS version 4.1 as file sharing
        /// protocol. NFSv4.1 can be used with HIGH_SCALE_SSD, ZONAL, REGIONAL and ENTERPRISE. The default is NFSv3. Default value:
        /// "NFS_V3" Possible values: ["NFS_V3", "NFS_V4_1"]
        pub protocol: pulumi_wasm_rust::Output<Option<String>>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The service tier of the instance.
        /// Possible values include: STANDARD, PREMIUM, BASIC_HDD, BASIC_SSD, HIGH_SCALE_SSD, ZONAL, REGIONAL and ENTERPRISE
        pub tier: pulumi_wasm_rust::Output<String>,
        /// The name of the Filestore zone of the instance.
        pub zone: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: InstanceArgs) -> InstanceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let deletion_protection_enabled_binding = args
            .deletion_protection_enabled
            .get_inner();
        let deletion_protection_reason_binding = args
            .deletion_protection_reason
            .get_inner();
        let description_binding = args.description.get_inner();
        let file_shares_binding = args.file_shares.get_inner();
        let kms_key_name_binding = args.kms_key_name.get_inner();
        let labels_binding = args.labels.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let networks_binding = args.networks.get_inner();
        let performance_config_binding = args.performance_config.get_inner();
        let project_binding = args.project.get_inner();
        let protocol_binding = args.protocol.get_inner();
        let tier_binding = args.tier.get_inner();
        let zone_binding = args.zone.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:filestore/instance:Instance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "deletionProtectionEnabled".into(),
                    value: &deletion_protection_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "deletionProtectionReason".into(),
                    value: &deletion_protection_reason_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "fileShares".into(),
                    value: &file_shares_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyName".into(),
                    value: &kms_key_name_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networks".into(),
                    value: &networks_binding,
                },
                register_interface::ObjectField {
                    name: "performanceConfig".into(),
                    value: &performance_config_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "protocol".into(),
                    value: &protocol_binding,
                },
                register_interface::ObjectField {
                    name: "tier".into(),
                    value: &tier_binding,
                },
                register_interface::ObjectField {
                    name: "zone".into(),
                    value: &zone_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "deletionProtectionEnabled".into(),
                },
                register_interface::ResultField {
                    name: "deletionProtectionReason".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "fileShares".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyName".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networks".into(),
                },
                register_interface::ResultField {
                    name: "performanceConfig".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "protocol".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "tier".into(),
                },
                register_interface::ResultField {
                    name: "zone".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        InstanceResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            deletion_protection_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionProtectionEnabled").unwrap(),
            ),
            deletion_protection_reason: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionProtectionReason").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            file_shares: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fileShares").unwrap(),
            ),
            kms_key_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyName").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            networks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networks").unwrap(),
            ),
            performance_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("performanceConfig").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protocol").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            tier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tier").unwrap(),
            ),
            zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zone").unwrap(),
            ),
        }
    }
}
