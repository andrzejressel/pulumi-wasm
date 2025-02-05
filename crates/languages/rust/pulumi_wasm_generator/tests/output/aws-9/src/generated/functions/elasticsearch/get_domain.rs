pub mod get_domain {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDomainArgs {
        /// Name of the domain.
        #[builder(into)]
        pub domain_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Tags assigned to the domain.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetDomainArgs,
    ) -> GetDomainResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let domain_name_binding = args.domain_name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDomainResult {
            access_policies: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accessPolicies"),
            ),
            advanced_options: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("advancedOptions"),
            ),
            advanced_security_options: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("advancedSecurityOptions"),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            auto_tune_options: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("autoTuneOptions"),
            ),
            cluster_configs: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clusterConfigs"),
            ),
            cognito_options: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cognitoOptions"),
            ),
            created: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("created"),
            ),
            deleted: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deleted"),
            ),
            domain_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("domainId"),
            ),
            domain_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("domainName"),
            ),
            ebs_options: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ebsOptions"),
            ),
            elasticsearch_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("elasticsearchVersion"),
            ),
            encryption_at_rests: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("encryptionAtRests"),
            ),
            endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("endpoint"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            kibana_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("kibanaEndpoint"),
            ),
            log_publishing_options: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("logPublishingOptions"),
            ),
            node_to_node_encryptions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("nodeToNodeEncryptions"),
            ),
            processing: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("processing"),
            ),
            snapshot_options: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("snapshotOptions"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            vpc_options: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("vpcOptions"),
            ),
        }
    }
}
