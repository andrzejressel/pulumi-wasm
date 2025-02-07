/// ## Example Usage
///
/// ### Workstation Config Basic
///
///
/// ```yaml
/// resources:
///   tagKey1:
///     type: gcp:tags:TagKey
///     name: tag_key1
///     properties:
///       parent: organizations/123456789
///       shortName: keyname
///   tagValue1:
///     type: gcp:tags:TagValue
///     name: tag_value1
///     properties:
///       parent: ${tagKey1.id}
///       shortName: valuename
///   default:
///     type: gcp:compute:Network
///     properties:
///       name: workstation-cluster
///       autoCreateSubnetworks: false
///   defaultSubnetwork:
///     type: gcp:compute:Subnetwork
///     name: default
///     properties:
///       name: workstation-cluster
///       ipCidrRange: 10.0.0.0/24
///       region: us-central1
///       network: ${default.name}
///   defaultWorkstationCluster:
///     type: gcp:workstations:WorkstationCluster
///     name: default
///     properties:
///       workstationClusterId: workstation-cluster
///       network: ${default.id}
///       subnetwork: ${defaultSubnetwork.id}
///       location: us-central1
///       labels:
///         label: key
///       annotations:
///         label-one: value-one
///   defaultWorkstationConfig:
///     type: gcp:workstations:WorkstationConfig
///     name: default
///     properties:
///       workstationConfigId: workstation-config
///       workstationClusterId: ${defaultWorkstationCluster.workstationClusterId}
///       location: us-central1
///       idleTimeout: 600s
///       runningTimeout: 21600s
///       replicaZones:
///         - us-central1-a
///         - us-central1-b
///       annotations:
///         label-one: value-one
///       labels:
///         label: key
///       maxUsableWorkstations: 1
///       host:
///         gceInstance:
///           machineType: e2-standard-4
///           bootDiskSizeGb: 35
///           disablePublicIpAddresses: true
///           disableSsh: false
///           vmTags:
///             ${tagKey1.id}: ${tagValue1.id}
/// ```
/// ### Workstation Config Container
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:Network
///     properties:
///       name: workstation-cluster
///       autoCreateSubnetworks: false
///   defaultSubnetwork:
///     type: gcp:compute:Subnetwork
///     name: default
///     properties:
///       name: workstation-cluster
///       ipCidrRange: 10.0.0.0/24
///       region: us-central1
///       network: ${default.name}
///   defaultWorkstationCluster:
///     type: gcp:workstations:WorkstationCluster
///     name: default
///     properties:
///       workstationClusterId: workstation-cluster
///       network: ${default.id}
///       subnetwork: ${defaultSubnetwork.id}
///       location: us-central1
///       labels:
///         label: key
///       annotations:
///         label-one: value-one
///   defaultWorkstationConfig:
///     type: gcp:workstations:WorkstationConfig
///     name: default
///     properties:
///       workstationConfigId: workstation-config
///       workstationClusterId: ${defaultWorkstationCluster.workstationClusterId}
///       location: us-central1
///       host:
///         gceInstance:
///           machineType: n1-standard-4
///           bootDiskSizeGb: 35
///           disablePublicIpAddresses: true
///           enableNestedVirtualization: true
///       container:
///         image: intellij
///         env:
///           NAME: FOO
///           BABE: bar
/// ```
/// ### Workstation Config Persistent Directories
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:Network
///     properties:
///       name: workstation-cluster
///       autoCreateSubnetworks: false
///   defaultSubnetwork:
///     type: gcp:compute:Subnetwork
///     name: default
///     properties:
///       name: workstation-cluster
///       ipCidrRange: 10.0.0.0/24
///       region: us-central1
///       network: ${default.name}
///   defaultWorkstationCluster:
///     type: gcp:workstations:WorkstationCluster
///     name: default
///     properties:
///       workstationClusterId: workstation-cluster
///       network: ${default.id}
///       subnetwork: ${defaultSubnetwork.id}
///       location: us-central1
///       labels:
///         label: key
///       annotations:
///         label-one: value-one
///   defaultWorkstationConfig:
///     type: gcp:workstations:WorkstationConfig
///     name: default
///     properties:
///       workstationConfigId: workstation-config
///       workstationClusterId: ${defaultWorkstationCluster.workstationClusterId}
///       location: us-central1
///       host:
///         gceInstance:
///           machineType: e2-standard-4
///           bootDiskSizeGb: 35
///           disablePublicIpAddresses: true
///           shieldedInstanceConfig:
///             enableSecureBoot: true
///             enableVtpm: true
///       persistentDirectories:
///         - mountPath: /home
///           gcePd:
///             sizeGb: 200
///             fsType: ext4
///             diskType: pd-standard
///             reclaimPolicy: DELETE
/// ```
/// ### Workstation Config Source Snapshot
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = network::create(
///         "default",
///         NetworkArgs::builder()
///             .auto_create_subnetworks(false)
///             .name("workstation-cluster")
///             .build_struct(),
///     );
///     let defaultSubnetwork = subnetwork::create(
///         "defaultSubnetwork",
///         SubnetworkArgs::builder()
///             .ip_cidr_range("10.0.0.0/24")
///             .name("workstation-cluster")
///             .network("${default.name}")
///             .region("us-central1")
///             .build_struct(),
///     );
///     let defaultWorkstationCluster = workstation_cluster::create(
///         "defaultWorkstationCluster",
///         WorkstationClusterArgs::builder()
///             .location("us-central1")
///             .network("${default.id}")
///             .subnetwork("${defaultSubnetwork.id}")
///             .workstation_cluster_id("workstation-cluster")
///             .build_struct(),
///     );
///     let defaultWorkstationConfig = workstation_config::create(
///         "defaultWorkstationConfig",
///         WorkstationConfigArgs::builder()
///             .location("${defaultWorkstationCluster.location}")
///             .persistent_directories(
///                 vec![
///                     WorkstationConfigPersistentDirectory::builder()
///                     .gcePd(WorkstationConfigPersistentDirectoryGcePd::builder()
///                     .reclaimPolicy("DELETE").sourceSnapshot("${mySourceSnapshot.id}")
///                     .build_struct()).mountPath("/home").build_struct(),
///                 ],
///             )
///             .workstation_cluster_id("${defaultWorkstationCluster.workstationClusterId}")
///             .workstation_config_id("workstation-config")
///             .build_struct(),
///     );
///     let mySourceDisk = disk::create(
///         "mySourceDisk",
///         DiskArgs::builder()
///             .name("workstation-config")
///             .size(10)
///             .type_("pd-ssd")
///             .zone("us-central1-a")
///             .build_struct(),
///     );
///     let mySourceSnapshot = snapshot::create(
///         "mySourceSnapshot",
///         SnapshotArgs::builder()
///             .name("workstation-config")
///             .source_disk("${mySourceDisk.name}")
///             .zone("us-central1-a")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Workstation Config Shielded Instance Config
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:Network
///     properties:
///       name: workstation-cluster
///       autoCreateSubnetworks: false
///   defaultSubnetwork:
///     type: gcp:compute:Subnetwork
///     name: default
///     properties:
///       name: workstation-cluster
///       ipCidrRange: 10.0.0.0/24
///       region: us-central1
///       network: ${default.name}
///   defaultWorkstationCluster:
///     type: gcp:workstations:WorkstationCluster
///     name: default
///     properties:
///       workstationClusterId: workstation-cluster
///       network: ${default.id}
///       subnetwork: ${defaultSubnetwork.id}
///       location: us-central1
///       labels:
///         label: key
///       annotations:
///         label-one: value-one
///   defaultWorkstationConfig:
///     type: gcp:workstations:WorkstationConfig
///     name: default
///     properties:
///       workstationConfigId: workstation-config
///       workstationClusterId: ${defaultWorkstationCluster.workstationClusterId}
///       location: us-central1
///       host:
///         gceInstance:
///           machineType: e2-standard-4
///           bootDiskSizeGb: 35
///           disablePublicIpAddresses: true
///           shieldedInstanceConfig:
///             enableSecureBoot: true
///             enableVtpm: true
/// ```
/// ### Workstation Config Accelerators
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:Network
///     properties:
///       name: workstation-cluster
///       autoCreateSubnetworks: false
///   defaultSubnetwork:
///     type: gcp:compute:Subnetwork
///     name: default
///     properties:
///       name: workstation-cluster
///       ipCidrRange: 10.0.0.0/24
///       region: us-central1
///       network: ${default.name}
///   defaultWorkstationCluster:
///     type: gcp:workstations:WorkstationCluster
///     name: default
///     properties:
///       workstationClusterId: workstation-cluster
///       network: ${default.id}
///       subnetwork: ${defaultSubnetwork.id}
///       location: us-central1
///       labels:
///         label: key
///       annotations:
///         label-one: value-one
///   defaultWorkstationConfig:
///     type: gcp:workstations:WorkstationConfig
///     name: default
///     properties:
///       workstationConfigId: workstation-config
///       workstationClusterId: ${defaultWorkstationCluster.workstationClusterId}
///       location: us-central1
///       host:
///         gceInstance:
///           machineType: n1-standard-2
///           bootDiskSizeGb: 35
///           disablePublicIpAddresses: true
///           accelerators:
///             - type: nvidia-tesla-t4
///               count: '1'
/// ```
/// ### Workstation Config Boost
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:Network
///     properties:
///       name: workstation-cluster
///       autoCreateSubnetworks: false
///   defaultSubnetwork:
///     type: gcp:compute:Subnetwork
///     name: default
///     properties:
///       name: workstation-cluster
///       ipCidrRange: 10.0.0.0/24
///       region: us-central1
///       network: ${default.name}
///   defaultWorkstationCluster:
///     type: gcp:workstations:WorkstationCluster
///     name: default
///     properties:
///       workstationClusterId: workstation-cluster
///       network: ${default.id}
///       subnetwork: ${defaultSubnetwork.id}
///       location: us-central1
///       labels:
///         label: key
///       annotations:
///         label-one: value-one
///   defaultWorkstationConfig:
///     type: gcp:workstations:WorkstationConfig
///     name: default
///     properties:
///       workstationConfigId: workstation-config
///       workstationClusterId: ${defaultWorkstationCluster.workstationClusterId}
///       location: us-central1
///       host:
///         gceInstance:
///           machineType: e2-standard-4
///           bootDiskSizeGb: 35
///           disablePublicIpAddresses: true
///           boostConfigs:
///             - id: boost-1
///               machineType: n1-standard-2
///               accelerators:
///                 - type: nvidia-tesla-t4
///                   count: '1'
///             - id: boost-2
///               machineType: n1-standard-2
///               poolSize: 2
///               bootDiskSizeGb: 30
///               enableNestedVirtualization: true
/// ```
/// ### Workstation Config Encryption Key
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:Network
///     properties:
///       name: workstation-cluster
///       autoCreateSubnetworks: false
///   defaultSubnetwork:
///     type: gcp:compute:Subnetwork
///     name: default
///     properties:
///       name: workstation-cluster
///       ipCidrRange: 10.0.0.0/24
///       region: us-central1
///       network: ${default.name}
///   defaultWorkstationCluster:
///     type: gcp:workstations:WorkstationCluster
///     name: default
///     properties:
///       workstationClusterId: workstation-cluster
///       network: ${default.id}
///       subnetwork: ${defaultSubnetwork.id}
///       location: us-central1
///       labels:
///         label: key
///       annotations:
///         label-one: value-one
///   defaultKeyRing:
///     type: gcp:kms:KeyRing
///     name: default
///     properties:
///       name: workstation-cluster
///       location: us-central1
///   defaultCryptoKey:
///     type: gcp:kms:CryptoKey
///     name: default
///     properties:
///       name: workstation-cluster
///       keyRing: ${defaultKeyRing.id}
///   defaultAccount:
///     type: gcp:serviceaccount:Account
///     name: default
///     properties:
///       accountId: my-account
///       displayName: Service Account
///   defaultWorkstationConfig:
///     type: gcp:workstations:WorkstationConfig
///     name: default
///     properties:
///       workstationConfigId: workstation-config
///       workstationClusterId: ${defaultWorkstationCluster.workstationClusterId}
///       location: us-central1
///       host:
///         gceInstance:
///           machineType: e2-standard-4
///           bootDiskSizeGb: 35
///           disablePublicIpAddresses: true
///           shieldedInstanceConfig:
///             enableSecureBoot: true
///             enableVtpm: true
///       encryptionKey:
///         kmsKey: ${defaultCryptoKey.id}
///         kmsKeyServiceAccount: ${defaultAccount.email}
/// ```
/// ### Workstation Config Allowed Ports
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:Network
///     properties:
///       name: workstation-cluster
///       autoCreateSubnetworks: false
///   defaultSubnetwork:
///     type: gcp:compute:Subnetwork
///     name: default
///     properties:
///       name: workstation-cluster
///       ipCidrRange: 10.0.0.0/24
///       region: us-central1
///       network: ${default.name}
///   defaultWorkstationCluster:
///     type: gcp:workstations:WorkstationCluster
///     name: default
///     properties:
///       workstationClusterId: workstation-cluster
///       network: ${default.id}
///       subnetwork: ${defaultSubnetwork.id}
///       location: us-central1
///       labels:
///         label: key
///       annotations:
///         label-one: value-one
///   defaultWorkstationConfig:
///     type: gcp:workstations:WorkstationConfig
///     name: default
///     properties:
///       workstationConfigId: workstation-config
///       workstationClusterId: ${defaultWorkstationCluster.workstationClusterId}
///       location: us-central1
///       host:
///         gceInstance:
///           machineType: e2-standard-4
///           bootDiskSizeGb: 35
///           disablePublicIpAddresses: true
///       allowedPorts:
///         - first: 80
///           last: 80
///         - first: 22
///           last: 22
///         - first: 1024
///           last: 65535
/// ```
///
/// ## Import
///
/// WorkstationConfig can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/workstationClusters/{{workstation_cluster_id}}/workstationConfigs/{{workstation_config_id}}`
///
/// * `{{project}}/{{location}}/{{workstation_cluster_id}}/{{workstation_config_id}}`
///
/// * `{{location}}/{{workstation_cluster_id}}/{{workstation_config_id}}`
///
/// When using the `pulumi import` command, WorkstationConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:workstations/workstationConfig:WorkstationConfig default projects/{{project}}/locations/{{location}}/workstationClusters/{{workstation_cluster_id}}/workstationConfigs/{{workstation_config_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:workstations/workstationConfig:WorkstationConfig default {{project}}/{{location}}/{{workstation_cluster_id}}/{{workstation_config_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:workstations/workstationConfig:WorkstationConfig default {{location}}/{{workstation_cluster_id}}/{{workstation_config_id}}
/// ```
///
pub mod workstation_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkstationConfigArgs {
        /// A list of port ranges specifying single ports or ranges of ports that are externally accessible in the workstation. Allowed ports must be one of 22, 80, or within range 1024-65535. If not specified defaults to ports 22, 80, and ports 1024-65535.
        /// Structure is documented below.
        #[builder(into, default)]
        pub allowed_ports: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::workstations::WorkstationConfigAllowedPort>>,
        >,
        /// Client-specified annotations. This is distinct from labels.
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Container that will be run for each workstation using this configuration when that workstation is started.
        /// Structure is documented below.
        #[builder(into, default)]
        pub container: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::workstations::WorkstationConfigContainer>,
        >,
        /// Disables support for plain TCP connections in the workstation. By default the service supports TCP connections via a websocket relay. Setting this option to true disables that relay, which prevents the usage of services that require plain tcp connections, such as ssh. When enabled, all communication must occur over https or wss.
        #[builder(into, default)]
        pub disable_tcp_connections: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Human-readable name for this resource.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether to enable Linux `auditd` logging on the workstation. When enabled, a service account must also be specified that has `logging.buckets.write` permission on the project. Operating system audit logging is distinct from Cloud Audit Logs.
        #[builder(into, default)]
        pub enable_audit_agent: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Encrypts resources of this workstation configuration using a customer-managed encryption key.
        /// If specified, the boot disk of the Compute Engine instance and the persistent disk are encrypted using this encryption key. If this field is not set, the disks are encrypted using a generated key. Customer-managed encryption keys do not protect disk metadata.
        /// If the customer-managed encryption key is rotated, when the workstation instance is stopped, the system attempts to recreate the persistent disk with the new version of the key. Be sure to keep older versions of the key until the persistent disk is recreated. Otherwise, data on the persistent disk will be lost.
        /// If the encryption key is revoked, the workstation session will automatically be stopped within 7 hours.
        /// Structure is documented below.
        #[builder(into, default)]
        pub encryption_key: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::workstations::WorkstationConfigEncryptionKey>,
        >,
        /// Ephemeral directories which won't persist across workstation sessions.
        /// Structure is documented below.
        #[builder(into, default)]
        pub ephemeral_directories: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::workstations::WorkstationConfigEphemeralDirectory,
                >,
            >,
        >,
        /// Runtime host for a workstation.
        /// Structure is documented below.
        #[builder(into, default)]
        pub host: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::workstations::WorkstationConfigHost>,
        >,
        /// How long to wait before automatically stopping an instance that hasn't recently received any user traffic. A value of 0 indicates that this instance should never time out from idleness. Defaults to 20 minutes.
        /// A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
        #[builder(into, default)]
        pub idle_timeout: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Client-specified labels that are applied to the resource and that are also propagated to the underlying Compute Engine resources.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location where the workstation cluster config should reside.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Maximum number of workstations under this configuration a user can have workstations.workstation.use permission on. Only enforced on CreateWorkstation API calls on the user issuing the API request.
        #[builder(into, default)]
        pub max_usable_workstations: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Directories to persist across workstation sessions.
        /// Structure is documented below.
        #[builder(into, default)]
        pub persistent_directories: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::workstations::WorkstationConfigPersistentDirectory,
                >,
            >,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Readiness checks to be performed on a workstation.
        /// Structure is documented below.
        #[builder(into, default)]
        pub readiness_checks: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::workstations::WorkstationConfigReadinessCheck>,
            >,
        >,
        /// Specifies the zones used to replicate the VM and disk resources within the region. If set, exactly two zones within the workstation cluster's region must be specified—for example, `['us-central1-a', 'us-central1-f']`.
        /// If this field is empty, two default zones within the region are used. Immutable after the workstation configuration is created.
        #[builder(into, default)]
        pub replica_zones: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// How long to wait before automatically stopping a workstation after it was started. A value of 0 indicates that workstations using this configuration should never time out from running duration. Must be greater than 0 and less than 24 hours if `encryption_key` is set. Defaults to 12 hours.
        /// A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
        #[builder(into, default)]
        pub running_timeout: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the parent workstation cluster.
        #[builder(into)]
        pub workstation_cluster_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID to be assigned to the workstation cluster config.
        #[builder(into)]
        pub workstation_config_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WorkstationConfigResult {
        /// A list of port ranges specifying single ports or ranges of ports that are externally accessible in the workstation. Allowed ports must be one of 22, 80, or within range 1024-65535. If not specified defaults to ports 22, 80, and ports 1024-65535.
        /// Structure is documented below.
        pub allowed_ports: pulumi_gestalt_rust::Output<
            Vec<super::super::types::workstations::WorkstationConfigAllowedPort>,
        >,
        /// Client-specified annotations. This is distinct from labels.
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        pub annotations: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Status conditions describing the current resource state.
        /// Structure is documented below.
        pub conditions: pulumi_gestalt_rust::Output<
            Vec<super::super::types::workstations::WorkstationConfigCondition>,
        >,
        /// Container that will be run for each workstation using this configuration when that workstation is started.
        /// Structure is documented below.
        pub container: pulumi_gestalt_rust::Output<
            super::super::types::workstations::WorkstationConfigContainer,
        >,
        /// Time when this resource was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Whether this resource is in degraded mode, in which case it may require user action to restore full functionality. Details can be found in the conditions field.
        pub degraded: pulumi_gestalt_rust::Output<bool>,
        /// Disables support for plain TCP connections in the workstation. By default the service supports TCP connections via a websocket relay. Setting this option to true disables that relay, which prevents the usage of services that require plain tcp connections, such as ssh. When enabled, all communication must occur over https or wss.
        pub disable_tcp_connections: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Human-readable name for this resource.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        pub effective_annotations: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Whether to enable Linux `auditd` logging on the workstation. When enabled, a service account must also be specified that has `logging.buckets.write` permission on the project. Operating system audit logging is distinct from Cloud Audit Logs.
        pub enable_audit_agent: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Encrypts resources of this workstation configuration using a customer-managed encryption key.
        /// If specified, the boot disk of the Compute Engine instance and the persistent disk are encrypted using this encryption key. If this field is not set, the disks are encrypted using a generated key. Customer-managed encryption keys do not protect disk metadata.
        /// If the customer-managed encryption key is rotated, when the workstation instance is stopped, the system attempts to recreate the persistent disk with the new version of the key. Be sure to keep older versions of the key until the persistent disk is recreated. Otherwise, data on the persistent disk will be lost.
        /// If the encryption key is revoked, the workstation session will automatically be stopped within 7 hours.
        /// Structure is documented below.
        pub encryption_key: pulumi_gestalt_rust::Output<
            Option<super::super::types::workstations::WorkstationConfigEncryptionKey>,
        >,
        /// Ephemeral directories which won't persist across workstation sessions.
        /// Structure is documented below.
        pub ephemeral_directories: pulumi_gestalt_rust::Output<
            Vec<super::super::types::workstations::WorkstationConfigEphemeralDirectory>,
        >,
        /// Checksum computed by the server.
        /// May be sent on update and delete requests to ensure that the client has an up-to-date value before proceeding.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// Runtime host for a workstation.
        /// Structure is documented below.
        pub host: pulumi_gestalt_rust::Output<
            super::super::types::workstations::WorkstationConfigHost,
        >,
        /// How long to wait before automatically stopping an instance that hasn't recently received any user traffic. A value of 0 indicates that this instance should never time out from idleness. Defaults to 20 minutes.
        /// A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
        pub idle_timeout: pulumi_gestalt_rust::Output<Option<String>>,
        /// Client-specified labels that are applied to the resource and that are also propagated to the underlying Compute Engine resources.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location where the workstation cluster config should reside.
        ///
        ///
        /// - - -
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Maximum number of workstations under this configuration a user can have workstations.workstation.use permission on. Only enforced on CreateWorkstation API calls on the user issuing the API request.
        pub max_usable_workstations: pulumi_gestalt_rust::Output<i32>,
        /// Full name of this resource.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Directories to persist across workstation sessions.
        /// Structure is documented below.
        pub persistent_directories: pulumi_gestalt_rust::Output<
            Vec<super::super::types::workstations::WorkstationConfigPersistentDirectory>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Readiness checks to be performed on a workstation.
        /// Structure is documented below.
        pub readiness_checks: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::workstations::WorkstationConfigReadinessCheck>,
            >,
        >,
        /// Specifies the zones used to replicate the VM and disk resources within the region. If set, exactly two zones within the workstation cluster's region must be specified—for example, `['us-central1-a', 'us-central1-f']`.
        /// If this field is empty, two default zones within the region are used. Immutable after the workstation configuration is created.
        pub replica_zones: pulumi_gestalt_rust::Output<Vec<String>>,
        /// How long to wait before automatically stopping a workstation after it was started. A value of 0 indicates that workstations using this configuration should never time out from running duration. Must be greater than 0 and less than 24 hours if `encryption_key` is set. Defaults to 12 hours.
        /// A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
        pub running_timeout: pulumi_gestalt_rust::Output<Option<String>>,
        /// The system-generated UID of the resource.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// The ID of the parent workstation cluster.
        pub workstation_cluster_id: pulumi_gestalt_rust::Output<String>,
        /// The ID to be assigned to the workstation cluster config.
        pub workstation_config_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: WorkstationConfigArgs,
    ) -> WorkstationConfigResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let allowed_ports_binding = args.allowed_ports.get_output(context).get_inner();
        let annotations_binding = args.annotations.get_output(context).get_inner();
        let container_binding = args.container.get_output(context).get_inner();
        let disable_tcp_connections_binding = args
            .disable_tcp_connections
            .get_output(context)
            .get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let enable_audit_agent_binding = args
            .enable_audit_agent
            .get_output(context)
            .get_inner();
        let encryption_key_binding = args.encryption_key.get_output(context).get_inner();
        let ephemeral_directories_binding = args
            .ephemeral_directories
            .get_output(context)
            .get_inner();
        let host_binding = args.host.get_output(context).get_inner();
        let idle_timeout_binding = args.idle_timeout.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let max_usable_workstations_binding = args
            .max_usable_workstations
            .get_output(context)
            .get_inner();
        let persistent_directories_binding = args
            .persistent_directories
            .get_output(context)
            .get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let readiness_checks_binding = args
            .readiness_checks
            .get_output(context)
            .get_inner();
        let replica_zones_binding = args.replica_zones.get_output(context).get_inner();
        let running_timeout_binding = args
            .running_timeout
            .get_output(context)
            .get_inner();
        let workstation_cluster_id_binding = args
            .workstation_cluster_id
            .get_output(context)
            .get_inner();
        let workstation_config_id_binding = args
            .workstation_config_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:workstations/workstationConfig:WorkstationConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allowedPorts".into(),
                    value: &allowed_ports_binding,
                },
                register_interface::ObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding,
                },
                register_interface::ObjectField {
                    name: "container".into(),
                    value: &container_binding,
                },
                register_interface::ObjectField {
                    name: "disableTcpConnections".into(),
                    value: &disable_tcp_connections_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "enableAuditAgent".into(),
                    value: &enable_audit_agent_binding,
                },
                register_interface::ObjectField {
                    name: "encryptionKey".into(),
                    value: &encryption_key_binding,
                },
                register_interface::ObjectField {
                    name: "ephemeralDirectories".into(),
                    value: &ephemeral_directories_binding,
                },
                register_interface::ObjectField {
                    name: "host".into(),
                    value: &host_binding,
                },
                register_interface::ObjectField {
                    name: "idleTimeout".into(),
                    value: &idle_timeout_binding,
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
                    name: "maxUsableWorkstations".into(),
                    value: &max_usable_workstations_binding,
                },
                register_interface::ObjectField {
                    name: "persistentDirectories".into(),
                    value: &persistent_directories_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "readinessChecks".into(),
                    value: &readiness_checks_binding,
                },
                register_interface::ObjectField {
                    name: "replicaZones".into(),
                    value: &replica_zones_binding,
                },
                register_interface::ObjectField {
                    name: "runningTimeout".into(),
                    value: &running_timeout_binding,
                },
                register_interface::ObjectField {
                    name: "workstationClusterId".into(),
                    value: &workstation_cluster_id_binding,
                },
                register_interface::ObjectField {
                    name: "workstationConfigId".into(),
                    value: &workstation_config_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        WorkstationConfigResult {
            allowed_ports: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("allowedPorts"),
            ),
            annotations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("annotations"),
            ),
            conditions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("conditions"),
            ),
            container: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("container"),
            ),
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            degraded: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("degraded"),
            ),
            disable_tcp_connections: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("disableTcpConnections"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            effective_annotations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveAnnotations"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            enable_audit_agent: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableAuditAgent"),
            ),
            encryption_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("encryptionKey"),
            ),
            ephemeral_directories: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ephemeralDirectories"),
            ),
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            host: pulumi_gestalt_rust::__private::into_domain(o.extract_field("host")),
            idle_timeout: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("idleTimeout"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            max_usable_workstations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maxUsableWorkstations"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            persistent_directories: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("persistentDirectories"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            readiness_checks: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("readinessChecks"),
            ),
            replica_zones: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("replicaZones"),
            ),
            running_timeout: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("runningTimeout"),
            ),
            uid: pulumi_gestalt_rust::__private::into_domain(o.extract_field("uid")),
            workstation_cluster_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workstationClusterId"),
            ),
            workstation_config_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workstationConfigId"),
            ),
        }
    }
}
