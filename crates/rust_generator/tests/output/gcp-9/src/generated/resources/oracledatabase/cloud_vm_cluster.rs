/// A CloudVmCluster resource.
///
///
/// To get more information about CloudVmCluster, see:
///
/// * [API documentation](https://cloud.google.com/oracle/database/docs/reference/rest/v1/projects.locations.cloudVmClusters)
/// * How-to Guides
///     * [Create VM clusters](https://cloud.google.com/oracle/database/docs/create-clusters)
///
/// ## Example Usage
///
/// ### Oracledatabase Cloud Vmcluster Basic
///
///
/// ```yaml
/// resources:
///   myVmcluster:
///     type: gcp:oracledatabase:CloudVmCluster
///     name: my_vmcluster
///     properties:
///       cloudVmClusterId: my-instance
///       displayName: my-instance displayname
///       location: us-east4
///       project: my-project
///       exadataInfrastructure: ${cloudExadataInfrastructures.id}
///       network: ${default.id}
///       cidr: 10.5.0.0/24
///       backupSubnetCidr: 10.6.0.0/24
///       properties:
///         licenseType: LICENSE_INCLUDED
///         sshPublicKeys:
///           - ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAACAQCz1X2744t+6vRLmE5u6nHi6/QWh8bQDgHmd+OIxRQIGA/IWUtCs2FnaCNZcqvZkaeyjk5v0lTA/n+9jvO42Ipib53athrfVG8gRt8fzPL66C6ZqHq+6zZophhrCdfJh/0G4x9xJh5gdMprlaCR1P8yAaVvhBQSKGc4SiIkyMNBcHJ5YTtMQMTfxaB4G1sHZ6SDAY9a6Cq/zNjDwfPapWLsiP4mRhE5SSjJX6l6EYbkm0JeLQg+AbJiNEPvrvDp1wtTxzlPJtIivthmLMThFxK7+DkrYFuLvN5AHUdo9KTDLvHtDCvV70r8v0gafsrKkM/OE9Jtzoo0e1N/5K/ZdyFRbAkFT4QSF3nwpbmBWLf2Evg//YyEuxnz4CwPqFST2mucnrCCGCVWp1vnHZ0y30nM35njLOmWdRDFy5l27pKUTwLp02y3UYiiZyP7d3/u5pKiN4vC27VuvzprSdJxWoAvluOiDeRh+/oeQDowxoT/Oop8DzB9uJmjktXw8jyMW2+Rpg+ENQqeNgF1OGlEzypaWiRskEFlkpLb4v/s3ZDYkL1oW0Nv/J8LTjTOTEaYt2Udjoe9x2xWiGnQixhdChWuG+MaoWffzUgx1tsVj/DBXijR5DjkPkrA1GA98zd3q8GKEaAdcDenJjHhNYSd4+rE9pIsnYn7fo5X/tFfcQH1XQ== nobody@google.com
///         cpuCoreCount: '4'
///         giVersion: 19.0.0.0
///         hostnamePrefix: hostname1
///       deletionProtection: 'true'
///   cloudExadataInfrastructures:
///     type: gcp:oracledatabase:CloudExadataInfrastructure
///     properties:
///       cloudExadataInfrastructureId: my-exadata
///       displayName: my-exadata displayname
///       location: us-east4
///       project: my-project
///       properties:
///         shape: Exadata.X9M
///         computeCount: '2'
///         storageCount: '3'
///       deletionProtection: 'true'
/// variables:
///   default:
///     fn::invoke:
///       function: gcp:compute:getNetwork
///       arguments:
///         name: new
///         project: my-project
/// ```
/// ### Oracledatabase Cloud Vmcluster Full
///
///
/// ```yaml
/// resources:
///   myVmcluster:
///     type: gcp:oracledatabase:CloudVmCluster
///     name: my_vmcluster
///     properties:
///       cloudVmClusterId: my-instance
///       displayName: my-instance displayname
///       location: us-east4
///       project: my-project
///       exadataInfrastructure: ${cloudExadataInfrastructures.id}
///       network: ${default.id}
///       cidr: 10.5.0.0/24
///       backupSubnetCidr: 10.6.0.0/24
///       labels:
///         label-one: value-one
///       properties:
///         licenseType: LICENSE_INCLUDED
///         sshPublicKeys:
///           - ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAACAQCz1X2744t+6vRLmE5u6nHi6/QWh8bQDgHmd+OIxRQIGA/IWUtCs2FnaCNZcqvZkaeyjk5v0lTA/n+9jvO42Ipib53athrfVG8gRt8fzPL66C6ZqHq+6zZophhrCdfJh/0G4x9xJh5gdMprlaCR1P8yAaVvhBQSKGc4SiIkyMNBcHJ5YTtMQMTfxaB4G1sHZ6SDAY9a6Cq/zNjDwfPapWLsiP4mRhE5SSjJX6l6EYbkm0JeLQg+AbJiNEPvrvDp1wtTxzlPJtIivthmLMThFxK7+DkrYFuLvN5AHUdo9KTDLvHtDCvV70r8v0gafsrKkM/OE9Jtzoo0e1N/5K/ZdyFRbAkFT4QSF3nwpbmBWLf2Evg//YyEuxnz4CwPqFST2mucnrCCGCVWp1vnHZ0y30nM35njLOmWdRDFy5l27pKUTwLp02y3UYiiZyP7d3/u5pKiN4vC27VuvzprSdJxWoAvluOiDeRh+/oeQDowxoT/Oop8DzB9uJmjktXw8jyMW2+Rpg+ENQqeNgF1OGlEzypaWiRskEFlkpLb4v/s3ZDYkL1oW0Nv/J8LTjTOTEaYt2Udjoe9x2xWiGnQixhdChWuG+MaoWffzUgx1tsVj/DBXijR5DjkPkrA1GA98zd3q8GKEaAdcDenJjHhNYSd4+rE9pIsnYn7fo5X/tFfcQH1XQ== nobody@google.com
///         cpuCoreCount: '4'
///         giVersion: 19.0.0.0
///         timeZone:
///           id: UTC
///         nodeCount: '2'
///         ocpuCount: '4.0'
///         dataStorageSizeTb: 2
///         dbNodeStorageSizeGb: 120
///         dbServerOcids:
///           - ${mydbserver.dbServers[0].properties[0].ocid}
///           - ${mydbserver.dbServers[1].properties[0].ocid}
///         diskRedundancy: HIGH
///         sparseDiskgroupEnabled: false
///         localBackupEnabled: false
///         clusterName: pq-ppat4
///         hostnamePrefix: hostname1
///         diagnosticsDataCollectionOptions:
///           diagnosticsEventsEnabled: true
///           healthMonitoringEnabled: true
///           incidentLogsEnabled: true
///         memorySizeGb: 60
///       deletionProtection: 'true'
///   cloudExadataInfrastructures:
///     type: gcp:oracledatabase:CloudExadataInfrastructure
///     properties:
///       cloudExadataInfrastructureId: my-exadata
///       displayName: my-exadata displayname
///       location: us-east4
///       project: my-project
///       properties:
///         shape: Exadata.X9M
///         computeCount: '2'
///         storageCount: '3'
///       deletionProtection: 'true'
/// variables:
///   default:
///     fn::invoke:
///       function: gcp:compute:getNetwork
///       arguments:
///         name: new
///         project: my-project
///   mydbserver:
///     fn::invoke:
///       function: gcp:oracledatabase:getDbServers
///       arguments:
///         location: us-east4
///         project: my-project
///         cloudExadataInfrastructure: ${cloudExadataInfrastructures.cloudExadataInfrastructureId}
/// ```
///
/// ## Import
///
/// CloudVmCluster can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/cloudVmClusters/{{cloud_vm_cluster_id}}`
///
/// * `{{project}}/{{location}}/{{cloud_vm_cluster_id}}`
///
/// * `{{location}}/{{cloud_vm_cluster_id}}`
///
/// When using the `pulumi import` command, CloudVmCluster can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:oracledatabase/cloudVmCluster:CloudVmCluster default projects/{{project}}/locations/{{location}}/cloudVmClusters/{{cloud_vm_cluster_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:oracledatabase/cloudVmCluster:CloudVmCluster default {{project}}/{{location}}/{{cloud_vm_cluster_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:oracledatabase/cloudVmCluster:CloudVmCluster default {{location}}/{{cloud_vm_cluster_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cloud_vm_cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CloudVmClusterArgs {
        /// CIDR range of the backup subnet.
        #[builder(into)]
        pub backup_subnet_cidr: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Network settings. CIDR to use for cluster IP allocation.
        #[builder(into)]
        pub cidr: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the VM Cluster to create. This value is restricted
        /// to (^a-z?$) and must be a maximum of 63
        /// characters in length. The value must start with a letter and end with
        /// a letter or a number.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub cloud_vm_cluster_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub deletion_protection: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// User friendly name for this resource.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Exadata Infrastructure resource on which VM cluster
        /// resource is created, in the following format:
        /// projects/{project}/locations/{region}/cloudExadataInfrastuctures/{cloud_extradata_infrastructure}
        #[builder(into)]
        pub exadata_infrastructure: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Labels or tags associated with the VM Cluster.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Resource ID segment making up resource `name`. See documentation for resource type `oracledatabase.googleapis.com/DbNode`.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the VPC network.
        /// Format: projects/{project}/global/networks/{network}
        #[builder(into)]
        pub network: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Various properties and settings associated with Exadata VM cluster.
        /// Structure is documented below.
        #[builder(into, default)]
        pub properties: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::oracledatabase::CloudVmClusterProperties>,
        >,
    }
    #[allow(dead_code)]
    pub struct CloudVmClusterResult {
        /// CIDR range of the backup subnet.
        pub backup_subnet_cidr: pulumi_gestalt_rust::Output<String>,
        /// Network settings. CIDR to use for cluster IP allocation.
        pub cidr: pulumi_gestalt_rust::Output<String>,
        /// The ID of the VM Cluster to create. This value is restricted
        /// to (^a-z?$) and must be a maximum of 63
        /// characters in length. The value must start with a letter and end with
        /// a letter or a number.
        ///
        ///
        /// - - -
        pub cloud_vm_cluster_id: pulumi_gestalt_rust::Output<String>,
        /// The date and time that the VM cluster was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        pub deletion_protection: pulumi_gestalt_rust::Output<Option<bool>>,
        /// User friendly name for this resource.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The name of the Exadata Infrastructure resource on which VM cluster
        /// resource is created, in the following format:
        /// projects/{project}/locations/{region}/cloudExadataInfrastuctures/{cloud_extradata_infrastructure}
        pub exadata_infrastructure: pulumi_gestalt_rust::Output<String>,
        /// GCP location where Oracle Exadata is hosted. It is same as GCP Oracle zone
        /// of Exadata infrastructure.
        pub gcp_oracle_zone: pulumi_gestalt_rust::Output<String>,
        /// Labels or tags associated with the VM Cluster.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Resource ID segment making up resource `name`. See documentation for resource type `oracledatabase.googleapis.com/DbNode`.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Identifier. The name of the VM Cluster resource with the format:
        /// projects/{project}/locations/{region}/cloudVmClusters/{cloud_vm_cluster}
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the VPC network.
        /// Format: projects/{project}/global/networks/{network}
        pub network: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Various properties and settings associated with Exadata VM cluster.
        /// Structure is documented below.
        pub properties: pulumi_gestalt_rust::Output<
            Option<super::super::types::oracledatabase::CloudVmClusterProperties>,
        >,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
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
        args: CloudVmClusterArgs,
    ) -> CloudVmClusterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let backup_subnet_cidr_binding = args.backup_subnet_cidr.get_output(context);
        let cidr_binding = args.cidr.get_output(context);
        let cloud_vm_cluster_id_binding = args.cloud_vm_cluster_id.get_output(context);
        let deletion_protection_binding = args.deletion_protection.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let exadata_infrastructure_binding = args
            .exadata_infrastructure
            .get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let network_binding = args.network.get_output(context);
        let project_binding = args.project.get_output(context);
        let properties_binding = args.properties.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:oracledatabase/cloudVmCluster:CloudVmCluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backupSubnetCidr".into(),
                    value: &backup_subnet_cidr_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cidr".into(),
                    value: &cidr_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cloudVmClusterId".into(),
                    value: &cloud_vm_cluster_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deletionProtection".into(),
                    value: &deletion_protection_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "exadataInfrastructure".into(),
                    value: &exadata_infrastructure_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "network".into(),
                    value: &network_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "properties".into(),
                    value: &properties_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        CloudVmClusterResult {
            backup_subnet_cidr: o.get_field("backupSubnetCidr"),
            cidr: o.get_field("cidr"),
            cloud_vm_cluster_id: o.get_field("cloudVmClusterId"),
            create_time: o.get_field("createTime"),
            deletion_protection: o.get_field("deletionProtection"),
            display_name: o.get_field("displayName"),
            effective_labels: o.get_field("effectiveLabels"),
            exadata_infrastructure: o.get_field("exadataInfrastructure"),
            gcp_oracle_zone: o.get_field("gcpOracleZone"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            network: o.get_field("network"),
            project: o.get_field("project"),
            properties: o.get_field("properties"),
            pulumi_labels: o.get_field("pulumiLabels"),
        }
    }
}
