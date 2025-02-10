/// ## Example Usage
///
/// ### Alloydb Instance Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:alloydb:Instance
///     properties:
///       cluster: ${defaultCluster.name}
///       instanceId: alloydb-instance
///       instanceType: PRIMARY
///       machineConfig:
///         cpuCount: 2
///     options:
///       dependsOn:
///         - ${vpcConnection}
///   defaultCluster:
///     type: gcp:alloydb:Cluster
///     name: default
///     properties:
///       clusterId: alloydb-cluster
///       location: us-central1
///       networkConfig:
///         network: ${defaultNetwork.id}
///       initialUser:
///         password: alloydb-cluster
///   defaultNetwork:
///     type: gcp:compute:Network
///     name: default
///     properties:
///       name: alloydb-network
///   privateIpAlloc:
///     type: gcp:compute:GlobalAddress
///     name: private_ip_alloc
///     properties:
///       name: alloydb-cluster
///       addressType: INTERNAL
///       purpose: VPC_PEERING
///       prefixLength: 16
///       network: ${defaultNetwork.id}
///   vpcConnection:
///     type: gcp:servicenetworking:Connection
///     name: vpc_connection
///     properties:
///       network: ${defaultNetwork.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${privateIpAlloc.name}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Alloydb Secondary Instance Basic
///
///
/// ```yaml
/// resources:
///   primary:
///     type: gcp:alloydb:Cluster
///     properties:
///       clusterId: alloydb-primary-cluster
///       location: us-central1
///       networkConfig:
///         network: ${default.id}
///   primaryInstance:
///     type: gcp:alloydb:Instance
///     name: primary
///     properties:
///       cluster: ${primary.name}
///       instanceId: alloydb-primary-instance
///       instanceType: PRIMARY
///       machineConfig:
///         cpuCount: 2
///     options:
///       dependsOn:
///         - ${vpcConnection}
///   secondary:
///     type: gcp:alloydb:Cluster
///     properties:
///       clusterId: alloydb-secondary-cluster
///       location: us-east1
///       networkConfig:
///         network: ${defaultGoogleComputeNetwork.id}
///       clusterType: SECONDARY
///       continuousBackupConfig:
///         enabled: false
///       secondaryConfig:
///         primaryClusterName: ${primary.name}
///       deletionPolicy: FORCE
///     options:
///       dependsOn:
///         - ${primaryInstance}
///   secondaryInstance:
///     type: gcp:alloydb:Instance
///     name: secondary
///     properties:
///       cluster: ${secondary.name}
///       instanceId: alloydb-secondary-instance
///       instanceType: ${secondary.clusterType}
///       machineConfig:
///         cpuCount: 2
///     options:
///       dependsOn:
///         - ${vpcConnection}
///   default:
///     type: gcp:compute:Network
///     properties:
///       name: alloydb-secondary-network
///   privateIpAlloc:
///     type: gcp:compute:GlobalAddress
///     name: private_ip_alloc
///     properties:
///       name: alloydb-secondary-instance
///       addressType: INTERNAL
///       purpose: VPC_PEERING
///       prefixLength: 16
///       network: ${default.id}
///   vpcConnection:
///     type: gcp:servicenetworking:Connection
///     name: vpc_connection
///     properties:
///       network: ${default.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${privateIpAlloc.name}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// Instance can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/clusters/{{cluster}}/instances/{{instance_id}}`
///
/// * `{{project}}/{{location}}/{{cluster}}/{{instance_id}}`
///
/// * `{{location}}/{{cluster}}/{{instance_id}}`
///
/// When using the `pulumi import` command, Instance can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:alloydb/instance:Instance default projects/{{project}}/locations/{{location}}/clusters/{{cluster}}/instances/{{instance_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:alloydb/instance:Instance default {{project}}/{{location}}/{{cluster}}/{{instance_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:alloydb/instance:Instance default {{location}}/{{cluster}}/{{instance_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceArgs {
        /// Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels.
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// 'Availability type of an Instance. Defaults to REGIONAL for both primary and read instances.
        /// Note that primary and read instances can have different availability types.
        /// Only READ_POOL instance supports ZONAL type. Users can't specify the zone for READ_POOL instance.
        /// Zone is automatically chosen from the list of zones in the region specified.
        /// Read pool of size 1 can only have zonal availability. Read pools with node count of 2 or more
        /// can have regional availability (nodes are present in 2 or more zones in a region).'
        /// Possible values are: `AVAILABILITY_TYPE_UNSPECIFIED`, `ZONAL`, `REGIONAL`.
        #[builder(into, default)]
        pub availability_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Client connection specific configurations.
        /// Structure is documented below.
        #[builder(into, default)]
        pub client_connection_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::alloydb::InstanceClientConnectionConfig>,
        >,
        /// Identifies the alloydb cluster. Must be in the format
        /// 'projects/{project}/locations/{location}/clusters/{cluster_id}'
        #[builder(into)]
        pub cluster: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Database flags. Set at instance level. * They are copied from primary instance on read instance creation. * Read instances can set new or override existing flags that are relevant for reads, e.g. for enabling columnar cache on a read instance. Flags set on read instance may or may not be present on primary.
        #[builder(into, default)]
        pub database_flags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// User-settable and human-readable display name for the Instance.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Compute Engine zone that the instance should serve from, per https://cloud.google.com/compute/docs/regions-zones This can ONLY be specified for ZONAL instances. If present for a REGIONAL instance, an error will be thrown. If this is absent for a ZONAL instance, instance is created in a random zone with available capacity.
        #[builder(into, default)]
        pub gce_zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the alloydb instance.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into)]
        pub instance_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// User-defined labels for the alloydb instance.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configurations for the machines that host the underlying database engine.
        /// Structure is documented below.
        #[builder(into, default)]
        pub machine_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::alloydb::InstanceMachineConfig>,
        >,
        /// Instance level network configuration.
        /// Structure is documented below.
        #[builder(into, default)]
        pub network_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::alloydb::InstanceNetworkConfig>,
        >,
        /// Configuration for enhanced query insights.
        /// Structure is documented below.
        #[builder(into, default)]
        pub observability_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::alloydb::InstanceObservabilityConfig>,
        >,
        /// Configuration for Private Service Connect (PSC) for the instance.
        /// Structure is documented below.
        #[builder(into, default)]
        pub psc_instance_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::alloydb::InstancePscInstanceConfig>,
        >,
        /// Configuration for query insights.
        /// Structure is documented below.
        #[builder(into, default)]
        pub query_insights_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::alloydb::InstanceQueryInsightsConfig>,
        >,
        /// Read pool specific config. If the instance type is READ_POOL, this configuration must be provided.
        /// Structure is documented below.
        #[builder(into, default)]
        pub read_pool_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::alloydb::InstanceReadPoolConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct InstanceResult {
        /// Annotations to allow client tools to store small amount of arbitrary data. This is distinct from labels.
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        pub annotations: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// 'Availability type of an Instance. Defaults to REGIONAL for both primary and read instances.
        /// Note that primary and read instances can have different availability types.
        /// Only READ_POOL instance supports ZONAL type. Users can't specify the zone for READ_POOL instance.
        /// Zone is automatically chosen from the list of zones in the region specified.
        /// Read pool of size 1 can only have zonal availability. Read pools with node count of 2 or more
        /// can have regional availability (nodes are present in 2 or more zones in a region).'
        /// Possible values are: `AVAILABILITY_TYPE_UNSPECIFIED`, `ZONAL`, `REGIONAL`.
        pub availability_type: pulumi_gestalt_rust::Output<String>,
        /// Client connection specific configurations.
        /// Structure is documented below.
        pub client_connection_config: pulumi_gestalt_rust::Output<
            super::super::types::alloydb::InstanceClientConnectionConfig,
        >,
        /// Identifies the alloydb cluster. Must be in the format
        /// 'projects/{project}/locations/{location}/clusters/{cluster_id}'
        pub cluster: pulumi_gestalt_rust::Output<String>,
        /// Time the Instance was created in UTC.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Database flags. Set at instance level. * They are copied from primary instance on read instance creation. * Read instances can set new or override existing flags that are relevant for reads, e.g. for enabling columnar cache on a read instance. Flags set on read instance may or may not be present on primary.
        pub database_flags: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// User-settable and human-readable display name for the Instance.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        pub effective_annotations: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The Compute Engine zone that the instance should serve from, per https://cloud.google.com/compute/docs/regions-zones This can ONLY be specified for ZONAL instances. If present for a REGIONAL instance, an error will be thrown. If this is absent for a ZONAL instance, instance is created in a random zone with available capacity.
        pub gce_zone: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the alloydb instance.
        ///
        ///
        /// - - -
        pub instance_id: pulumi_gestalt_rust::Output<String>,
        pub instance_type: pulumi_gestalt_rust::Output<String>,
        /// The IP address for the Instance. This is the connection endpoint for an end-user application.
        pub ip_address: pulumi_gestalt_rust::Output<String>,
        /// User-defined labels for the alloydb instance.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configurations for the machines that host the underlying database engine.
        /// Structure is documented below.
        pub machine_config: pulumi_gestalt_rust::Output<
            super::super::types::alloydb::InstanceMachineConfig,
        >,
        /// The name of the instance resource.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Instance level network configuration.
        /// Structure is documented below.
        pub network_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::alloydb::InstanceNetworkConfig>,
        >,
        /// Configuration for enhanced query insights.
        /// Structure is documented below.
        pub observability_config: pulumi_gestalt_rust::Output<
            super::super::types::alloydb::InstanceObservabilityConfig,
        >,
        /// The outbound public IP addresses for the instance. This is available ONLY when
        /// networkConfig.enableOutboundPublicIp is set to true. These IP addresses are used
        /// for outbound connections.
        pub outbound_public_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Configuration for Private Service Connect (PSC) for the instance.
        /// Structure is documented below.
        pub psc_instance_config: pulumi_gestalt_rust::Output<
            super::super::types::alloydb::InstancePscInstanceConfig,
        >,
        /// The public IP addresses for the Instance. This is available ONLY when
        /// networkConfig.enablePublicIp is set to true. This is the connection
        /// endpoint for an end-user application.
        pub public_ip_address: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration for query insights.
        /// Structure is documented below.
        pub query_insights_config: pulumi_gestalt_rust::Output<
            super::super::types::alloydb::InstanceQueryInsightsConfig,
        >,
        /// Read pool specific config. If the instance type is READ_POOL, this configuration must be provided.
        /// Structure is documented below.
        pub read_pool_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::alloydb::InstanceReadPoolConfig>,
        >,
        /// Set to true if the current state of Instance does not match the user's intended state, and the service is actively updating the resource to reconcile them. This can happen due to user-triggered updates or system actions like failover or maintenance.
        pub reconciling: pulumi_gestalt_rust::Output<bool>,
        /// The current state of the alloydb instance.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// The system-generated UID of the resource.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// Time the Instance was updated in UTC.
        pub update_time: pulumi_gestalt_rust::Output<String>,
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
        let annotations_binding = args.annotations.get_output(context);
        let availability_type_binding = args.availability_type.get_output(context);
        let client_connection_config_binding = args
            .client_connection_config
            .get_output(context);
        let cluster_binding = args.cluster.get_output(context);
        let database_flags_binding = args.database_flags.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let gce_zone_binding = args.gce_zone.get_output(context);
        let instance_id_binding = args.instance_id.get_output(context);
        let instance_type_binding = args.instance_type.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let machine_config_binding = args.machine_config.get_output(context);
        let network_config_binding = args.network_config.get_output(context);
        let observability_config_binding = args.observability_config.get_output(context);
        let psc_instance_config_binding = args.psc_instance_config.get_output(context);
        let query_insights_config_binding = args
            .query_insights_config
            .get_output(context);
        let read_pool_config_binding = args.read_pool_config.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:alloydb/instance:Instance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "annotations".into(),
                    value: annotations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "availabilityType".into(),
                    value: availability_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientConnectionConfig".into(),
                    value: client_connection_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cluster".into(),
                    value: cluster_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "databaseFlags".into(),
                    value: database_flags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gceZone".into(),
                    value: gce_zone_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceId".into(),
                    value: instance_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceType".into(),
                    value: instance_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "machineConfig".into(),
                    value: machine_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkConfig".into(),
                    value: network_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "observabilityConfig".into(),
                    value: observability_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pscInstanceConfig".into(),
                    value: psc_instance_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "queryInsightsConfig".into(),
                    value: query_insights_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "readPoolConfig".into(),
                    value: read_pool_config_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        InstanceResult {
            annotations: o.get_field("annotations"),
            availability_type: o.get_field("availabilityType"),
            client_connection_config: o.get_field("clientConnectionConfig"),
            cluster: o.get_field("cluster"),
            create_time: o.get_field("createTime"),
            database_flags: o.get_field("databaseFlags"),
            display_name: o.get_field("displayName"),
            effective_annotations: o.get_field("effectiveAnnotations"),
            effective_labels: o.get_field("effectiveLabels"),
            gce_zone: o.get_field("gceZone"),
            instance_id: o.get_field("instanceId"),
            instance_type: o.get_field("instanceType"),
            ip_address: o.get_field("ipAddress"),
            labels: o.get_field("labels"),
            machine_config: o.get_field("machineConfig"),
            name: o.get_field("name"),
            network_config: o.get_field("networkConfig"),
            observability_config: o.get_field("observabilityConfig"),
            outbound_public_ip_addresses: o.get_field("outboundPublicIpAddresses"),
            psc_instance_config: o.get_field("pscInstanceConfig"),
            public_ip_address: o.get_field("publicIpAddress"),
            pulumi_labels: o.get_field("pulumiLabels"),
            query_insights_config: o.get_field("queryInsightsConfig"),
            read_pool_config: o.get_field("readPoolConfig"),
            reconciling: o.get_field("reconciling"),
            state: o.get_field("state"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
        }
    }
}
