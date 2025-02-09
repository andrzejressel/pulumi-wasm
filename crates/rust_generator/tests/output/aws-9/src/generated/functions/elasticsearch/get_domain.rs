#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_domain {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDomainArgs {
        /// Name of the domain.
        #[builder(into)]
        pub domain_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Tags assigned to the domain.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetDomainResult {
        /// The policy document attached to the domain.
        pub access_policies: pulumi_gestalt_rust::Output<String>,
        /// Key-value string pairs to specify advanced configuration options.
        pub advanced_options: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Status of the Elasticsearch domain's advanced security options. The block consists of the following attributes:
        pub advanced_security_options: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::elasticsearch::GetDomainAdvancedSecurityOption,
            >,
        >,
        /// The ARN of the domain.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Configuration of the Auto-Tune options of the domain.
        pub auto_tune_options: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::elasticsearch::GetDomainAutoTuneOption>,
        >,
        /// Cluster configuration of the domain.
        pub cluster_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::elasticsearch::GetDomainClusterConfig>,
        >,
        /// Domain Amazon Cognito Authentication options for Kibana.
        pub cognito_options: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::elasticsearch::GetDomainCognitoOption>,
        >,
        /// Status of the creation of the domain.
        pub created: pulumi_gestalt_rust::Output<bool>,
        /// Status of the deletion of the domain.
        pub deleted: pulumi_gestalt_rust::Output<bool>,
        /// Unique identifier for the domain.
        pub domain_id: pulumi_gestalt_rust::Output<String>,
        pub domain_name: pulumi_gestalt_rust::Output<String>,
        /// EBS Options for the instances in the domain.
        pub ebs_options: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::elasticsearch::GetDomainEbsOption>,
        >,
        /// Elasticsearch version for the domain.
        pub elasticsearch_version: pulumi_gestalt_rust::Output<String>,
        /// Domain encryption at rest related options.
        pub encryption_at_rests: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::elasticsearch::GetDomainEncryptionAtRest>,
        >,
        /// Domain-specific endpoint used to submit index, search, and data upload requests.
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Domain-specific endpoint used to access the Kibana application.
        pub kibana_endpoint: pulumi_gestalt_rust::Output<String>,
        /// Domain log publishing related options.
        pub log_publishing_options: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::elasticsearch::GetDomainLogPublishingOption>,
        >,
        /// Domain in transit encryption related options.
        pub node_to_node_encryptions: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::elasticsearch::GetDomainNodeToNodeEncryption>,
        >,
        /// Status of a configuration change in the domain.
        pub processing: pulumi_gestalt_rust::Output<bool>,
        /// Domain snapshot related options.
        pub snapshot_options: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::elasticsearch::GetDomainSnapshotOption>,
        >,
        /// Tags assigned to the domain.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// VPC Options for private Elasticsearch domains.
        pub vpc_options: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::elasticsearch::GetDomainVpcOption>,
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
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let domain_name_binding = args.domain_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:elasticsearch/getDomain:getDomain".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domainName".into(),
                    value: domain_name_binding.get_id(),
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
            deleted: o.get_field("deleted"),
            domain_id: o.get_field("domainId"),
            domain_name: o.get_field("domainName"),
            ebs_options: o.get_field("ebsOptions"),
            elasticsearch_version: o.get_field("elasticsearchVersion"),
            encryption_at_rests: o.get_field("encryptionAtRests"),
            endpoint: o.get_field("endpoint"),
            id: o.get_field("id"),
            kibana_endpoint: o.get_field("kibanaEndpoint"),
            log_publishing_options: o.get_field("logPublishingOptions"),
            node_to_node_encryptions: o.get_field("nodeToNodeEncryptions"),
            processing: o.get_field("processing"),
            snapshot_options: o.get_field("snapshotOptions"),
            tags: o.get_field("tags"),
            vpc_options: o.get_field("vpcOptions"),
        }
    }
}
