#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_domain {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDomainArgs {
        /// Name of the domain.
        #[builder(into)]
        pub domain_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Off Peak update options
        #[builder(into, default)]
        pub off_peak_window_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::super::types::opensearch::GetDomainOffPeakWindowOptions>,
        >,
        /// Tags assigned to the domain.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetDomainResult {
        /// Policy document attached to the domain.
        pub access_policies: pulumi_gestalt_rust::Output<String>,
        /// Key-value string pairs to specify advanced configuration options.
        pub advanced_options: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Status of the OpenSearch domain's advanced security options. The block consists of the following attributes:
        pub advanced_security_options: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::opensearch::GetDomainAdvancedSecurityOption>,
        >,
        /// ARN of the domain.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Configuration of the Auto-Tune options of the domain.
        pub auto_tune_options: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::opensearch::GetDomainAutoTuneOption>,
        >,
        /// Cluster configuration of the domain.
        pub cluster_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::opensearch::GetDomainClusterConfig>,
        >,
        /// Domain Amazon Cognito Authentication options for Dashboard.
        pub cognito_options: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::opensearch::GetDomainCognitoOption>,
        >,
        /// Status of the creation of the domain.
        pub created: pulumi_gestalt_rust::Output<bool>,
        /// Domain-specific endpoint used to access the [Dashboard application](https://docs.aws.amazon.com/opensearch-service/latest/developerguide/dashboards.html).
        pub dashboard_endpoint: pulumi_gestalt_rust::Output<String>,
        /// V2 domain-specific endpoint used to access the [Dashboard application](https://docs.aws.amazon.com/opensearch-service/latest/developerguide/dashboards.html)
        pub dashboard_endpoint_v2: pulumi_gestalt_rust::Output<String>,
        /// Status of the deletion of the domain.
        pub deleted: pulumi_gestalt_rust::Output<bool>,
        /// Dual stack hosted zone ID for the domain.
        pub domain_endpoint_v2_hosted_zone_id: pulumi_gestalt_rust::Output<String>,
        /// Unique identifier for the domain.
        pub domain_id: pulumi_gestalt_rust::Output<String>,
        pub domain_name: pulumi_gestalt_rust::Output<String>,
        /// EBS Options for the instances in the domain.
        pub ebs_options: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::opensearch::GetDomainEbsOption>,
        >,
        /// Domain encryption at rest related options.
        pub encryption_at_rests: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::opensearch::GetDomainEncryptionAtRest>,
        >,
        /// Domain-specific endpoint used to submit index, search, and data upload requests.
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// V2 domain-specific endpoint that works with both IPv4 and IPv6 addresses, used to submit index, search, and data upload requests.
        pub endpoint_v2: pulumi_gestalt_rust::Output<String>,
        /// OpenSearch version for the domain.
        pub engine_version: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Type of IP addresses supported by the endpoint for the domain.
        pub ip_address_type: pulumi_gestalt_rust::Output<String>,
        /// (**Deprecated**) Domain-specific endpoint for kibana without https scheme. Use the `dashboard_endpoint` attribute instead.
        pub kibana_endpoint: pulumi_gestalt_rust::Output<String>,
        /// Domain log publishing related options.
        pub log_publishing_options: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::opensearch::GetDomainLogPublishingOption>,
        >,
        /// Domain in transit encryption related options.
        pub node_to_node_encryptions: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::opensearch::GetDomainNodeToNodeEncryption>,
        >,
        /// Off Peak update options
        pub off_peak_window_options: pulumi_gestalt_rust::Output<
            Option<super::super::super::types::opensearch::GetDomainOffPeakWindowOptions>,
        >,
        /// Status of a configuration change in the domain.
        pub processing: pulumi_gestalt_rust::Output<bool>,
        /// Domain snapshot related options.
        pub snapshot_options: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::opensearch::GetDomainSnapshotOption>,
        >,
        /// Software update options for the domain
        pub software_update_options: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::opensearch::GetDomainSoftwareUpdateOption>,
        >,
        /// Tags assigned to the domain.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// VPC Options for private OpenSearch domains.
        pub vpc_options: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::opensearch::GetDomainVpcOption>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetDomainArgs,
    ) -> GetDomainResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let domain_name_binding = args.domain_name.get_output(context);
        let off_peak_window_options_binding = args
            .off_peak_window_options
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:opensearch/getDomain:getDomain".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainName".into(),
                    value: domain_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "offPeakWindowOptions".into(),
                    value: off_peak_window_options_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDomainResult {
            access_policies: o.get_field("accessPolicies"),
            advanced_options: o.get_field("advancedOptions"),
            advanced_security_options: o.get_field("advancedSecurityOptions"),
            arn: o.get_field("arn"),
            auto_tune_options: o.get_field("autoTuneOptions"),
            cluster_configs: o.get_field("clusterConfigs"),
            cognito_options: o.get_field("cognitoOptions"),
            created: o.get_field("created"),
            dashboard_endpoint: o.get_field("dashboardEndpoint"),
            dashboard_endpoint_v2: o.get_field("dashboardEndpointV2"),
            deleted: o.get_field("deleted"),
            domain_endpoint_v2_hosted_zone_id: o
                .get_field("domainEndpointV2HostedZoneId"),
            domain_id: o.get_field("domainId"),
            domain_name: o.get_field("domainName"),
            ebs_options: o.get_field("ebsOptions"),
            encryption_at_rests: o.get_field("encryptionAtRests"),
            endpoint: o.get_field("endpoint"),
            endpoint_v2: o.get_field("endpointV2"),
            engine_version: o.get_field("engineVersion"),
            id: o.get_field("id"),
            ip_address_type: o.get_field("ipAddressType"),
            kibana_endpoint: o.get_field("kibanaEndpoint"),
            log_publishing_options: o.get_field("logPublishingOptions"),
            node_to_node_encryptions: o.get_field("nodeToNodeEncryptions"),
            off_peak_window_options: o.get_field("offPeakWindowOptions"),
            processing: o.get_field("processing"),
            snapshot_options: o.get_field("snapshotOptions"),
            software_update_options: o.get_field("softwareUpdateOptions"),
            tags: o.get_field("tags"),
            vpc_options: o.get_field("vpcOptions"),
        }
    }
}
