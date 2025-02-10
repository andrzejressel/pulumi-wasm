/// Represents a Data Fusion instance.
///
///
/// To get more information about Instance, see:
///
/// * [API documentation](https://cloud.google.com/data-fusion/docs/reference/rest/v1beta1/projects.locations.instances)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/data-fusion/docs/)
///
/// ## Example Usage
///
/// ### Data Fusion Instance Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basicInstance = instance::create(
///         "basicInstance",
///         InstanceArgs::builder()
///             .name("my-instance")
///             .region("us-central1")
///             .type_("BASIC")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Data Fusion Instance Full
///
///
/// ```yaml
/// resources:
///   extendedInstance:
///     type: gcp:datafusion:Instance
///     name: extended_instance
///     properties:
///       name: my-instance
///       description: My Data Fusion instance
///       displayName: My Data Fusion instance
///       region: us-central1
///       type: BASIC
///       enableStackdriverLogging: true
///       enableStackdriverMonitoring: true
///       privateInstance: true
///       dataprocServiceAccount: ${default.email}
///       labels:
///         example_key: example_value
///       networkConfig:
///         network: default
///         ipAllocation: ${privateIpAlloc.address}/${privateIpAlloc.prefixLength}
///       accelerators:
///         - acceleratorType: CDC
///           state: ENABLED
///   network:
///     type: gcp:compute:Network
///     properties:
///       name: datafusion-full-network
///   privateIpAlloc:
///     type: gcp:compute:GlobalAddress
///     name: private_ip_alloc
///     properties:
///       name: datafusion-ip-alloc
///       addressType: INTERNAL
///       purpose: VPC_PEERING
///       prefixLength: 22
///       network: ${network.id}
/// variables:
///   default:
///     fn::invoke:
///       function: gcp:appengine:getDefaultServiceAccount
///       arguments: {}
/// ```
/// ### Data Fusion Instance Psc
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let psc = network::create(
///         "psc",
///         NetworkArgs::builder()
///             .auto_create_subnetworks(false)
///             .name("datafusion-psc-network")
///             .build_struct(),
///     );
///     let pscInstance = instance::create(
///         "pscInstance",
///         InstanceArgs::builder()
///             .name("psc-instance")
///             .network_config(
///                 InstanceNetworkConfig::builder()
///                     .connectionType("PRIVATE_SERVICE_CONNECT_INTERFACES")
///                     .privateServiceConnectConfig(
///                         InstanceNetworkConfigPrivateServiceConnectConfig::builder()
///                             .networkAttachment("${pscNetworkAttachment.id}")
///                             .unreachableCidrBlock("192.168.0.0/25")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .private_instance(true)
///             .region("us-central1")
///             .type_("BASIC")
///             .build_struct(),
///     );
///     let pscNetworkAttachment = network_attachment::create(
///         "pscNetworkAttachment",
///         NetworkAttachmentArgs::builder()
///             .connection_preference("ACCEPT_AUTOMATIC")
///             .name("datafusion-psc-attachment")
///             .region("us-central1")
///             .subnetworks(vec!["${pscSubnetwork.selfLink}",])
///             .build_struct(),
///     );
///     let pscSubnetwork = subnetwork::create(
///         "pscSubnetwork",
///         SubnetworkArgs::builder()
///             .ip_cidr_range("10.0.0.0/16")
///             .name("datafusion-psc-subnet")
///             .network("${psc.id}")
///             .region("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Data Fusion Instance Cmek
///
///
/// ```yaml
/// resources:
///   cmek:
///     type: gcp:datafusion:Instance
///     properties:
///       name: my-instance
///       region: us-central1
///       type: BASIC
///       cryptoKeyConfig:
///         keyReference: ${cryptoKey.id}
///     options:
///       dependsOn:
///         - ${cryptoKeyMember}
///   cryptoKey:
///     type: gcp:kms:CryptoKey
///     name: crypto_key
///     properties:
///       name: my-instance
///       keyRing: ${keyRing.id}
///   keyRing:
///     type: gcp:kms:KeyRing
///     name: key_ring
///     properties:
///       name: my-instance
///       location: us-central1
///   cryptoKeyMember:
///     type: gcp:kms:CryptoKeyIAMMember
///     name: crypto_key_member
///     properties:
///       cryptoKeyId: ${cryptoKey.id}
///       role: roles/cloudkms.cryptoKeyEncrypterDecrypter
///       member: serviceAccount:service-${project.number}@gcp-sa-datafusion.iam.gserviceaccount.com
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Data Fusion Instance Enterprise
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let enterpriseInstance = instance::create(
///         "enterpriseInstance",
///         InstanceArgs::builder()
///             .enable_rbac(true)
///             .name("my-instance")
///             .region("us-central1")
///             .type_("ENTERPRISE")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Data Fusion Instance Event
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let event = instance::create(
///         "event",
///         InstanceArgs::builder()
///             .event_publish_config(
///                 InstanceEventPublishConfig::builder()
///                     .enabled(true)
///                     .topic("${eventTopic.id}")
///                     .build_struct(),
///             )
///             .name("my-instance")
///             .region("us-central1")
///             .type_("BASIC")
///             .build_struct(),
///     );
///     let eventTopic = topic::create(
///         "eventTopic",
///         TopicArgs::builder().name("my-instance").build_struct(),
///     );
/// }
/// ```
/// ### Data Fusion Instance Zone
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let zone = instance::create(
///         "zone",
///         InstanceArgs::builder()
///             .name("my-instance")
///             .region("us-central1")
///             .type_("DEVELOPER")
///             .zone("us-central1-a")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Instance can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{region}}/instances/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Instance can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:datafusion/instance:Instance default projects/{{project}}/locations/{{region}}/instances/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:datafusion/instance:Instance default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:datafusion/instance:Instance default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:datafusion/instance:Instance default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceArgs {
        /// List of accelerators enabled for this CDF instance.
        /// If accelerators are enabled it is possible a permadiff will be created with the Options field.
        /// Users will need to either manually update their state file to include these diffed options, or include the field in a lifecycle ignore changes block.
        /// Structure is documented below.
        #[builder(into, default)]
        pub accelerators: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::datafusion::InstanceAccelerator>>,
        >,
        /// The crypto key configuration. This field is used by the Customer-Managed Encryption Keys (CMEK) feature.
        /// Structure is documented below.
        #[builder(into, default)]
        pub crypto_key_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datafusion::InstanceCryptoKeyConfig>,
        >,
        /// User-managed service account to set on Dataproc when Cloud Data Fusion creates Dataproc to run data processing pipelines.
        #[builder(into, default)]
        pub dataproc_service_account: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An optional description of the instance.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Display name for an instance.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Option to enable granular role-based access control.
        #[builder(into, default)]
        pub enable_rbac: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Option to enable Stackdriver Logging.
        #[builder(into, default)]
        pub enable_stackdriver_logging: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Option to enable Stackdriver Monitoring.
        #[builder(into, default)]
        pub enable_stackdriver_monitoring: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Option to enable and pass metadata for event publishing.
        /// Structure is documented below.
        #[builder(into, default)]
        pub event_publish_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datafusion::InstanceEventPublishConfig>,
        >,
        /// The resource labels for instance to use to annotate any related underlying resources,
        /// such as Compute Engine VMs.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the instance or a fully qualified identifier for the instance.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Network configuration options. These are required when a private Data Fusion instance is to be created.
        /// Structure is documented below.
        #[builder(into, default)]
        pub network_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datafusion::InstanceNetworkConfig>,
        >,
        /// Map of additional options used to configure the behavior of Data Fusion instance.
        #[builder(into, default)]
        pub options: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies whether the Data Fusion instance should be private. If set to
        /// true, all Data Fusion nodes will have private IP addresses and will not be
        /// able to access the public internet.
        #[builder(into, default)]
        pub private_instance: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The region of the Data Fusion instance.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Represents the type of Data Fusion instance. Each type is configured with
        /// the default settings for processing and memory.
        /// - BASIC: Basic Data Fusion instance. In Basic type, the user will be able to create data pipelines
        /// using point and click UI. However, there are certain limitations, such as fewer number
        /// of concurrent pipelines, no support for streaming pipelines, etc.
        /// - ENTERPRISE: Enterprise Data Fusion instance. In Enterprise type, the user will have more features
        /// available, such as support for streaming pipelines, higher number of concurrent pipelines, etc.
        /// - DEVELOPER: Developer Data Fusion instance. In Developer type, the user will have all features available but
        /// with restrictive capabilities. This is to help enterprises design and develop their data ingestion and integration
        /// pipelines at low cost.
        /// Possible values are: `BASIC`, `ENTERPRISE`, `DEVELOPER`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Current version of the Data Fusion.
        #[builder(into, default)]
        pub version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the zone in which the Data Fusion instance will be created. Only DEVELOPER instances use this field.
        #[builder(into, default)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct InstanceResult {
        /// List of accelerators enabled for this CDF instance.
        /// If accelerators are enabled it is possible a permadiff will be created with the Options field.
        /// Users will need to either manually update their state file to include these diffed options, or include the field in a lifecycle ignore changes block.
        /// Structure is documented below.
        pub accelerators: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::datafusion::InstanceAccelerator>>,
        >,
        /// Endpoint on which the REST APIs is accessible.
        pub api_endpoint: pulumi_gestalt_rust::Output<String>,
        /// The time the instance was created in RFC3339 UTC "Zulu" format, accurate to nanoseconds.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The crypto key configuration. This field is used by the Customer-Managed Encryption Keys (CMEK) feature.
        /// Structure is documented below.
        pub crypto_key_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::datafusion::InstanceCryptoKeyConfig>,
        >,
        /// User-managed service account to set on Dataproc when Cloud Data Fusion creates Dataproc to run data processing pipelines.
        pub dataproc_service_account: pulumi_gestalt_rust::Output<Option<String>>,
        /// An optional description of the instance.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Display name for an instance.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Option to enable granular role-based access control.
        pub enable_rbac: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Option to enable Stackdriver Logging.
        pub enable_stackdriver_logging: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Option to enable Stackdriver Monitoring.
        pub enable_stackdriver_monitoring: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Option to enable and pass metadata for event publishing.
        /// Structure is documented below.
        pub event_publish_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::datafusion::InstanceEventPublishConfig>,
        >,
        /// Cloud Storage bucket generated by Data Fusion in the customer project.
        pub gcs_bucket: pulumi_gestalt_rust::Output<String>,
        /// The resource labels for instance to use to annotate any related underlying resources,
        /// such as Compute Engine VMs.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ID of the instance or a fully qualified identifier for the instance.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Network configuration options. These are required when a private Data Fusion instance is to be created.
        /// Structure is documented below.
        pub network_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::datafusion::InstanceNetworkConfig>,
        >,
        /// Map of additional options used to configure the behavior of Data Fusion instance.
        pub options: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// P4 service account for the customer project.
        pub p4_service_account: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether the Data Fusion instance should be private. If set to
        /// true, all Data Fusion nodes will have private IP addresses and will not be
        /// able to access the public internet.
        pub private_instance: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The region of the Data Fusion instance.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// Service account which will be used to access resources in the customer project.
        pub service_account: pulumi_gestalt_rust::Output<String>,
        /// Endpoint on which the Data Fusion UI and REST APIs are accessible.
        pub service_endpoint: pulumi_gestalt_rust::Output<String>,
        /// The current state of this Data Fusion instance.
        /// - CREATING: Instance is being created
        /// - RUNNING: Instance is running and ready for requests
        /// - FAILED: Instance creation failed
        /// - DELETING: Instance is being deleted
        /// - UPGRADING: Instance is being upgraded
        /// - RESTARTING: Instance is being restarted
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Additional information about the current state of this Data Fusion instance if available.
        pub state_message: pulumi_gestalt_rust::Output<String>,
        /// The name of the tenant project.
        pub tenant_project_id: pulumi_gestalt_rust::Output<String>,
        /// Represents the type of Data Fusion instance. Each type is configured with
        /// the default settings for processing and memory.
        /// - BASIC: Basic Data Fusion instance. In Basic type, the user will be able to create data pipelines
        /// using point and click UI. However, there are certain limitations, such as fewer number
        /// of concurrent pipelines, no support for streaming pipelines, etc.
        /// - ENTERPRISE: Enterprise Data Fusion instance. In Enterprise type, the user will have more features
        /// available, such as support for streaming pipelines, higher number of concurrent pipelines, etc.
        /// - DEVELOPER: Developer Data Fusion instance. In Developer type, the user will have all features available but
        /// with restrictive capabilities. This is to help enterprises design and develop their data ingestion and integration
        /// pipelines at low cost.
        /// Possible values are: `BASIC`, `ENTERPRISE`, `DEVELOPER`.
        ///
        ///
        /// - - -
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// The time the instance was last updated in RFC3339 UTC "Zulu" format, accurate to nanoseconds.
        pub update_time: pulumi_gestalt_rust::Output<String>,
        /// Current version of the Data Fusion.
        pub version: pulumi_gestalt_rust::Output<String>,
        /// Name of the zone in which the Data Fusion instance will be created. Only DEVELOPER instances use this field.
        pub zone: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InstanceArgs,
    ) -> InstanceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let accelerators_binding = args.accelerators.get_output(context);
        let crypto_key_config_binding = args.crypto_key_config.get_output(context);
        let dataproc_service_account_binding = args
            .dataproc_service_account
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let enable_rbac_binding = args.enable_rbac.get_output(context);
        let enable_stackdriver_logging_binding = args
            .enable_stackdriver_logging
            .get_output(context);
        let enable_stackdriver_monitoring_binding = args
            .enable_stackdriver_monitoring
            .get_output(context);
        let event_publish_config_binding = args.event_publish_config.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_config_binding = args.network_config.get_output(context);
        let options_binding = args.options.get_output(context);
        let private_instance_binding = args.private_instance.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let type__binding = args.type_.get_output(context);
        let version_binding = args.version.get_output(context);
        let zone_binding = args.zone.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:datafusion/instance:Instance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accelerators".into(),
                    value: accelerators_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cryptoKeyConfig".into(),
                    value: crypto_key_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataprocServiceAccount".into(),
                    value: dataproc_service_account_binding.get_id(),
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
                    name: "enableRbac".into(),
                    value: enable_rbac_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableStackdriverLogging".into(),
                    value: enable_stackdriver_logging_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableStackdriverMonitoring".into(),
                    value: enable_stackdriver_monitoring_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventPublishConfig".into(),
                    value: event_publish_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkConfig".into(),
                    value: network_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "options".into(),
                    value: options_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "privateInstance".into(),
                    value: private_instance_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "version".into(),
                    value: version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zone".into(),
                    value: zone_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        InstanceResult {
            accelerators: o.get_field("accelerators"),
            api_endpoint: o.get_field("apiEndpoint"),
            create_time: o.get_field("createTime"),
            crypto_key_config: o.get_field("cryptoKeyConfig"),
            dataproc_service_account: o.get_field("dataprocServiceAccount"),
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            effective_labels: o.get_field("effectiveLabels"),
            enable_rbac: o.get_field("enableRbac"),
            enable_stackdriver_logging: o.get_field("enableStackdriverLogging"),
            enable_stackdriver_monitoring: o.get_field("enableStackdriverMonitoring"),
            event_publish_config: o.get_field("eventPublishConfig"),
            gcs_bucket: o.get_field("gcsBucket"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
            network_config: o.get_field("networkConfig"),
            options: o.get_field("options"),
            p4_service_account: o.get_field("p4ServiceAccount"),
            private_instance: o.get_field("privateInstance"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            region: o.get_field("region"),
            service_account: o.get_field("serviceAccount"),
            service_endpoint: o.get_field("serviceEndpoint"),
            state: o.get_field("state"),
            state_message: o.get_field("stateMessage"),
            tenant_project_id: o.get_field("tenantProjectId"),
            type_: o.get_field("type"),
            update_time: o.get_field("updateTime"),
            version: o.get_field("version"),
            zone: o.get_field("zone"),
        }
    }
}
