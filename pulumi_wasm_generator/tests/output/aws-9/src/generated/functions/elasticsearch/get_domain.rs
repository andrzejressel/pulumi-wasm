pub mod get_domain {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDomainArgs {
        /// Name of the domain.
        #[builder(into)]
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// Tags assigned to the domain.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetDomainResult {
        /// The policy document attached to the domain.
        pub access_policies: pulumi_wasm_rust::Output<String>,
        /// Key-value string pairs to specify advanced configuration options.
        pub advanced_options: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Status of the Elasticsearch domain's advanced security options. The block consists of the following attributes:
        pub advanced_security_options: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::elasticsearch::GetDomainAdvancedSecurityOption,
            >,
        >,
        /// The ARN of the domain.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Configuration of the Auto-Tune options of the domain.
        pub auto_tune_options: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::elasticsearch::GetDomainAutoTuneOption>,
        >,
        /// Cluster configuration of the domain.
        pub cluster_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::elasticsearch::GetDomainClusterConfig>,
        >,
        /// Domain Amazon Cognito Authentication options for Kibana.
        pub cognito_options: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::elasticsearch::GetDomainCognitoOption>,
        >,
        /// Status of the creation of the domain.
        pub created: pulumi_wasm_rust::Output<bool>,
        /// Status of the deletion of the domain.
        pub deleted: pulumi_wasm_rust::Output<bool>,
        /// Unique identifier for the domain.
        pub domain_id: pulumi_wasm_rust::Output<String>,
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// EBS Options for the instances in the domain.
        pub ebs_options: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::elasticsearch::GetDomainEbsOption>,
        >,
        /// Elasticsearch version for the domain.
        pub elasticsearch_version: pulumi_wasm_rust::Output<String>,
        /// Domain encryption at rest related options.
        pub encryption_at_rests: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::elasticsearch::GetDomainEncryptionAtRest>,
        >,
        /// Domain-specific endpoint used to submit index, search, and data upload requests.
        pub endpoint: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Domain-specific endpoint used to access the Kibana application.
        pub kibana_endpoint: pulumi_wasm_rust::Output<String>,
        /// Domain log publishing related options.
        pub log_publishing_options: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::elasticsearch::GetDomainLogPublishingOption>,
        >,
        /// Domain in transit encryption related options.
        pub node_to_node_encryptions: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::elasticsearch::GetDomainNodeToNodeEncryption>,
        >,
        /// Status of a configuration change in the domain.
        pub processing: pulumi_wasm_rust::Output<bool>,
        /// Domain snapshot related options.
        pub snapshot_options: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::elasticsearch::GetDomainSnapshotOption>,
        >,
        /// Tags assigned to the domain.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// VPC Options for private Elasticsearch domains.
        pub vpc_options: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::elasticsearch::GetDomainVpcOption>,
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
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:elasticsearch/getDomain:getDomain".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding,
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
                    name: "deleted".into(),
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
                    name: "elasticsearchVersion".into(),
                },
                register_interface::ResultField {
                    name: "encryptionAtRests".into(),
                },
                register_interface::ResultField {
                    name: "endpoint".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
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
                    name: "processing".into(),
                },
                register_interface::ResultField {
                    name: "snapshotOptions".into(),
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
            deleted: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deleted").unwrap(),
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
            elasticsearch_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("elasticsearchVersion").unwrap(),
            ),
            encryption_at_rests: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionAtRests").unwrap(),
            ),
            endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpoint").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            kibana_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kibanaEndpoint").unwrap(),
            ),
            log_publishing_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logPublishingOptions").unwrap(),
            ),
            node_to_node_encryptions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodeToNodeEncryptions").unwrap(),
            ),
            processing: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("processing").unwrap(),
            ),
            snapshot_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snapshotOptions").unwrap(),
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
