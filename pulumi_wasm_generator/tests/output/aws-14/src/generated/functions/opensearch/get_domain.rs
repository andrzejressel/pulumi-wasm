pub mod get_domain {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDomainArgs {
        /// Name of the domain.
        #[builder(into)]
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// Off Peak update options
        #[builder(into, default)]
        pub off_peak_window_options: pulumi_wasm_rust::Output<
            Option<super::super::super::types::opensearch::GetDomainOffPeakWindowOptions>,
        >,
        /// Tags assigned to the domain.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetDomainResult {
        /// Policy document attached to the domain.
        pub access_policies: pulumi_wasm_rust::Output<String>,
        /// Key-value string pairs to specify advanced configuration options.
        pub advanced_options: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Status of the OpenSearch domain's advanced security options. The block consists of the following attributes:
        pub advanced_security_options: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::opensearch::GetDomainAdvancedSecurityOption>,
        >,
        /// ARN of the domain.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Configuration of the Auto-Tune options of the domain.
        pub auto_tune_options: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::opensearch::GetDomainAutoTuneOption>,
        >,
        /// Cluster configuration of the domain.
        pub cluster_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::opensearch::GetDomainClusterConfig>,
        >,
        /// Domain Amazon Cognito Authentication options for Dashboard.
        pub cognito_options: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::opensearch::GetDomainCognitoOption>,
        >,
        /// Status of the creation of the domain.
        pub created: pulumi_wasm_rust::Output<bool>,
        /// Domain-specific endpoint used to access the [Dashboard application](https://docs.aws.amazon.com/opensearch-service/latest/developerguide/dashboards.html).
        pub dashboard_endpoint: pulumi_wasm_rust::Output<String>,
        /// V2 domain-specific endpoint used to access the [Dashboard application](https://docs.aws.amazon.com/opensearch-service/latest/developerguide/dashboards.html)
        pub dashboard_endpoint_v2: pulumi_wasm_rust::Output<String>,
        /// Status of the deletion of the domain.
        pub deleted: pulumi_wasm_rust::Output<bool>,
        /// Dual stack hosted zone ID for the domain.
        pub domain_endpoint_v2_hosted_zone_id: pulumi_wasm_rust::Output<String>,
        /// Unique identifier for the domain.
        pub domain_id: pulumi_wasm_rust::Output<String>,
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// EBS Options for the instances in the domain.
        pub ebs_options: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::opensearch::GetDomainEbsOption>,
        >,
        /// Domain encryption at rest related options.
        pub encryption_at_rests: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::opensearch::GetDomainEncryptionAtRest>,
        >,
        /// Domain-specific endpoint used to submit index, search, and data upload requests.
        pub endpoint: pulumi_wasm_rust::Output<String>,
        /// V2 domain-specific endpoint that works with both IPv4 and IPv6 addresses, used to submit index, search, and data upload requests.
        pub endpoint_v2: pulumi_wasm_rust::Output<String>,
        /// OpenSearch version for the domain.
        pub engine_version: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Type of IP addresses supported by the endpoint for the domain.
        pub ip_address_type: pulumi_wasm_rust::Output<String>,
        /// (**Deprecated**) Domain-specific endpoint for kibana without https scheme. Use the `dashboard_endpoint` attribute instead.
        pub kibana_endpoint: pulumi_wasm_rust::Output<String>,
        /// Domain log publishing related options.
        pub log_publishing_options: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::opensearch::GetDomainLogPublishingOption>,
        >,
        /// Domain in transit encryption related options.
        pub node_to_node_encryptions: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::opensearch::GetDomainNodeToNodeEncryption>,
        >,
        /// Off Peak update options
        pub off_peak_window_options: pulumi_wasm_rust::Output<
            Option<super::super::super::types::opensearch::GetDomainOffPeakWindowOptions>,
        >,
        /// Status of a configuration change in the domain.
        pub processing: pulumi_wasm_rust::Output<bool>,
        /// Domain snapshot related options.
        pub snapshot_options: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::opensearch::GetDomainSnapshotOption>,
        >,
        /// Software update options for the domain
        pub software_update_options: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::opensearch::GetDomainSoftwareUpdateOption>,
        >,
        /// Tags assigned to the domain.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// VPC Options for private OpenSearch domains.
        pub vpc_options: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::opensearch::GetDomainVpcOption>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetDomainArgs) -> GetDomainResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let domain_name_binding = args.domain_name.get_inner();
        let off_peak_window_options_binding = args.off_peak_window_options.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:opensearch/getDomain:getDomain".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding,
                },
                register_interface::ObjectField {
                    name: "offPeakWindowOptions".into(),
                    value: &off_peak_window_options_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessPolicies".into(),
                },
                register_interface::ResultField {
                    name: "advancedOptions".into(),
                },
                register_interface::ResultField {
                    name: "advancedSecurityOptions".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "autoTuneOptions".into(),
                },
                register_interface::ResultField {
                    name: "clusterConfigs".into(),
                },
                register_interface::ResultField {
                    name: "cognitoOptions".into(),
                },
                register_interface::ResultField {
                    name: "created".into(),
                },
                register_interface::ResultField {
                    name: "dashboardEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "dashboardEndpointV2".into(),
                },
                register_interface::ResultField {
                    name: "deleted".into(),
                },
                register_interface::ResultField {
                    name: "domainEndpointV2HostedZoneId".into(),
                },
                register_interface::ResultField {
                    name: "domainId".into(),
                },
                register_interface::ResultField {
                    name: "domainName".into(),
                },
                register_interface::ResultField {
                    name: "ebsOptions".into(),
                },
                register_interface::ResultField {
                    name: "encryptionAtRests".into(),
                },
                register_interface::ResultField {
                    name: "endpoint".into(),
                },
                register_interface::ResultField {
                    name: "endpointV2".into(),
                },
                register_interface::ResultField {
                    name: "engineVersion".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ipAddressType".into(),
                },
                register_interface::ResultField {
                    name: "kibanaEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "logPublishingOptions".into(),
                },
                register_interface::ResultField {
                    name: "nodeToNodeEncryptions".into(),
                },
                register_interface::ResultField {
                    name: "offPeakWindowOptions".into(),
                },
                register_interface::ResultField {
                    name: "processing".into(),
                },
                register_interface::ResultField {
                    name: "snapshotOptions".into(),
                },
                register_interface::ResultField {
                    name: "softwareUpdateOptions".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "vpcOptions".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetDomainResult {
            access_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessPolicies").unwrap(),
            ),
            advanced_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("advancedOptions").unwrap(),
            ),
            advanced_security_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("advancedSecurityOptions").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            auto_tune_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoTuneOptions").unwrap(),
            ),
            cluster_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterConfigs").unwrap(),
            ),
            cognito_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cognitoOptions").unwrap(),
            ),
            created: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("created").unwrap(),
            ),
            dashboard_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dashboardEndpoint").unwrap(),
            ),
            dashboard_endpoint_v2: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dashboardEndpointV2").unwrap(),
            ),
            deleted: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deleted").unwrap(),
            ),
            domain_endpoint_v2_hosted_zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainEndpointV2HostedZoneId").unwrap(),
            ),
            domain_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainId").unwrap(),
            ),
            domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainName").unwrap(),
            ),
            ebs_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ebsOptions").unwrap(),
            ),
            encryption_at_rests: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionAtRests").unwrap(),
            ),
            endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpoint").unwrap(),
            ),
            endpoint_v2: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpointV2").unwrap(),
            ),
            engine_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engineVersion").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ip_address_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipAddressType").unwrap(),
            ),
            kibana_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kibanaEndpoint").unwrap(),
            ),
            log_publishing_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logPublishingOptions").unwrap(),
            ),
            node_to_node_encryptions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodeToNodeEncryptions").unwrap(),
            ),
            off_peak_window_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("offPeakWindowOptions").unwrap(),
            ),
            processing: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("processing").unwrap(),
            ),
            snapshot_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snapshotOptions").unwrap(),
            ),
            software_update_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("softwareUpdateOptions").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            vpc_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcOptions").unwrap(),
            ),
        }
    }
}
