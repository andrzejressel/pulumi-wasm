/// Manages an AWS Elasticsearch Domain.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:elasticsearch:Domain
///     properties:
///       domainName: example
///       elasticsearchVersion: '7.10'
///       clusterConfig:
///         instanceType: r4.large.elasticsearch
///       tags:
///         Domain: TestDomain
/// ```
///
/// ### Access Policy
///
/// > See also: `aws.elasticsearch.DomainPolicy` resource
///
/// ```yaml
/// configuration:
///   domain:
///     type: string
///     default: tf-test
/// resources:
///   example:
///     type: aws:elasticsearch:Domain
///     properties:
///       domainName: ${domain}
///       accessPolicies: |
///         {
///           "Version": "2012-10-17",
///           "Statement": [
///             {
///               "Action": "es:*",
///               "Principal": "*",
///               "Effect": "Allow",
///               "Resource": "arn:aws:es:${current.name}:${currentGetCallerIdentity.accountId}:domain/${domain}/*",
///               "Condition": {
///                 "IpAddress": {"aws:SourceIp": ["66.193.100.22/32"]}
///               }
///             }
///           ]
///         }
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getRegion
///       arguments: {}
///   currentGetCallerIdentity:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
/// ```
///
/// ### Log Publishing to CloudWatch Logs
///
/// ```yaml
/// resources:
///   exampleLogGroup:
///     type: aws:cloudwatch:LogGroup
///     name: example
///     properties:
///       name: example
///   exampleLogResourcePolicy:
///     type: aws:cloudwatch:LogResourcePolicy
///     name: example
///     properties:
///       policyName: example
///       policyDocument: ${example.json}
///   exampleDomain:
///     type: aws:elasticsearch:Domain
///     name: example
///     properties:
///       logPublishingOptions:
///         - cloudwatchLogGroupArn: ${exampleLogGroup.arn}
///           logType: INDEX_SLOW_LOGS
/// variables:
///   example:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - es.amazonaws.com
///             actions:
///               - logs:PutLogEvents
///               - logs:PutLogEventsBatch
///               - logs:CreateLogStream
///             resources:
///               - arn:aws:logs:*
/// ```
///
/// ### VPC based ES
///
/// ```yaml
/// configuration:
///   vpc:
///     type: dynamic
///   domain:
///     type: string
///     default: tf-test
/// resources:
///   es:
///     type: aws:ec2:SecurityGroup
///     properties:
///       name: ${vpc}-elasticsearch-${domain}
///       description: Managed by Pulumi
///       vpcId: ${selected.id}
///       ingress:
///         - fromPort: 443
///           toPort: 443
///           protocol: tcp
///           cidrBlocks:
///             - ${selected.cidrBlock}
///   esServiceLinkedRole:
///     type: aws:iam:ServiceLinkedRole
///     name: es
///     properties:
///       awsServiceName: opensearchservice.amazonaws.com
///   esDomain:
///     type: aws:elasticsearch:Domain
///     name: es
///     properties:
///       domainName: ${domain}
///       elasticsearchVersion: '6.3'
///       clusterConfig:
///         instanceType: m4.large.elasticsearch
///         zoneAwarenessEnabled: true
///       vpcOptions:
///         subnetIds:
///           - ${selectedGetSubnets.ids[0]}
///           - ${selectedGetSubnets.ids[1]}
///         securityGroupIds:
///           - ${es.id}
///       advancedOptions:
///         rest.action.multi.allow_explicit_index: 'true'
///       accessPolicies: |
///         {
///         	"Version": "2012-10-17",
///         	"Statement": [
///         		{
///         			"Action": "es:*",
///         			"Principal": "*",
///         			"Effect": "Allow",
///         			"Resource": "arn:aws:es:${current.name}:${currentGetCallerIdentity.accountId}:domain/${domain}/*"
///         		}
///         	]
///         }
///       tags:
///         Domain: TestDomain
///     options:
///       dependsOn:
///         - ${esServiceLinkedRole}
/// variables:
///   selected:
///     fn::invoke:
///       function: aws:ec2:getVpc
///       arguments:
///         tags:
///           Name: ${vpc}
///   selectedGetSubnets:
///     fn::invoke:
///       function: aws:ec2:getSubnets
///       arguments:
///         filters:
///           - name: vpc-id
///             values:
///               - ${selected.id}
///         tags:
///           Tier: private
///   current:
///     fn::invoke:
///       function: aws:getRegion
///       arguments: {}
///   currentGetCallerIdentity:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Elasticsearch domains using the `domain_name`. For example:
///
/// ```sh
/// $ pulumi import aws:elasticsearch/domain:Domain example domain_name
/// ```
pub mod domain {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainArgs {
        /// IAM policy document specifying the access policies for the domain.
        #[builder(into, default)]
        pub access_policies: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value string pairs to specify advanced configuration options. Note that the values for these configuration options must be strings (wrapped in quotes) or they may be wrong and cause a perpetual diff, causing the provider to want to recreate your Elasticsearch domain on every apply.
        #[builder(into, default)]
        pub advanced_options: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration block for [fine-grained access control](https://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/fgac.html). Detailed below.
        #[builder(into, default)]
        pub advanced_security_options: pulumi_wasm_rust::Output<
            Option<super::super::types::elasticsearch::DomainAdvancedSecurityOptions>,
        >,
        /// Configuration block for the Auto-Tune options of the domain. Detailed below.
        #[builder(into, default)]
        pub auto_tune_options: pulumi_wasm_rust::Output<
            Option<super::super::types::elasticsearch::DomainAutoTuneOptions>,
        >,
        /// Configuration block for the cluster of the domain. Detailed below.
        #[builder(into, default)]
        pub cluster_config: pulumi_wasm_rust::Output<
            Option<super::super::types::elasticsearch::DomainClusterConfig>,
        >,
        /// Configuration block for authenticating Kibana with Cognito. Detailed below.
        #[builder(into, default)]
        pub cognito_options: pulumi_wasm_rust::Output<
            Option<super::super::types::elasticsearch::DomainCognitoOptions>,
        >,
        /// Configuration block for domain endpoint HTTP(S) related options. Detailed below.
        #[builder(into, default)]
        pub domain_endpoint_options: pulumi_wasm_rust::Output<
            Option<super::super::types::elasticsearch::DomainDomainEndpointOptions>,
        >,
        /// Name of the domain.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub domain_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block for EBS related options, may be required based on chosen [instance size](https://aws.amazon.com/elasticsearch-service/pricing/). Detailed below.
        #[builder(into, default)]
        pub ebs_options: pulumi_wasm_rust::Output<
            Option<super::super::types::elasticsearch::DomainEbsOptions>,
        >,
        /// Version of Elasticsearch to deploy. Defaults to `1.5`.
        #[builder(into, default)]
        pub elasticsearch_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block for encrypt at rest options. Only available for [certain instance types](http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/aes-supported-instance-types.html). Detailed below.
        #[builder(into, default)]
        pub encrypt_at_rest: pulumi_wasm_rust::Output<
            Option<super::super::types::elasticsearch::DomainEncryptAtRest>,
        >,
        /// Configuration block for publishing slow and application logs to CloudWatch Logs. This block can be declared multiple times, for each log_type, within the same resource. Detailed below.
        #[builder(into, default)]
        pub log_publishing_options: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::elasticsearch::DomainLogPublishingOption>>,
        >,
        /// Configuration block for node-to-node encryption options. Detailed below.
        #[builder(into, default)]
        pub node_to_node_encryption: pulumi_wasm_rust::Output<
            Option<super::super::types::elasticsearch::DomainNodeToNodeEncryption>,
        >,
        /// Configuration block for snapshot related options. Detailed below. DEPRECATED. For domains running Elasticsearch 5.3 and later, Amazon ES takes hourly automated snapshots, making this setting irrelevant. For domains running earlier versions of Elasticsearch, Amazon ES takes daily automated snapshots.
        #[builder(into, default)]
        pub snapshot_options: pulumi_wasm_rust::Output<
            Option<super::super::types::elasticsearch::DomainSnapshotOptions>,
        >,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration block for VPC related options. Adding or removing this configuration forces a new resource ([documentation](https://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-vpc.html#es-vpc-limitations)). Detailed below.
        #[builder(into, default)]
        pub vpc_options: pulumi_wasm_rust::Output<
            Option<super::super::types::elasticsearch::DomainVpcOptions>,
        >,
    }
    #[allow(dead_code)]
    pub struct DomainResult {
        /// IAM policy document specifying the access policies for the domain.
        pub access_policies: pulumi_wasm_rust::Output<String>,
        /// Key-value string pairs to specify advanced configuration options. Note that the values for these configuration options must be strings (wrapped in quotes) or they may be wrong and cause a perpetual diff, causing the provider to want to recreate your Elasticsearch domain on every apply.
        pub advanced_options: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration block for [fine-grained access control](https://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/fgac.html). Detailed below.
        pub advanced_security_options: pulumi_wasm_rust::Output<
            super::super::types::elasticsearch::DomainAdvancedSecurityOptions,
        >,
        /// ARN of the domain.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Configuration block for the Auto-Tune options of the domain. Detailed below.
        pub auto_tune_options: pulumi_wasm_rust::Output<
            super::super::types::elasticsearch::DomainAutoTuneOptions,
        >,
        /// Configuration block for the cluster of the domain. Detailed below.
        pub cluster_config: pulumi_wasm_rust::Output<
            super::super::types::elasticsearch::DomainClusterConfig,
        >,
        /// Configuration block for authenticating Kibana with Cognito. Detailed below.
        pub cognito_options: pulumi_wasm_rust::Output<
            Option<super::super::types::elasticsearch::DomainCognitoOptions>,
        >,
        /// Configuration block for domain endpoint HTTP(S) related options. Detailed below.
        pub domain_endpoint_options: pulumi_wasm_rust::Output<
            super::super::types::elasticsearch::DomainDomainEndpointOptions,
        >,
        /// Unique identifier for the domain.
        pub domain_id: pulumi_wasm_rust::Output<String>,
        /// Name of the domain.
        ///
        /// The following arguments are optional:
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// Configuration block for EBS related options, may be required based on chosen [instance size](https://aws.amazon.com/elasticsearch-service/pricing/). Detailed below.
        pub ebs_options: pulumi_wasm_rust::Output<
            super::super::types::elasticsearch::DomainEbsOptions,
        >,
        /// Version of Elasticsearch to deploy. Defaults to `1.5`.
        pub elasticsearch_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block for encrypt at rest options. Only available for [certain instance types](http://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/aes-supported-instance-types.html). Detailed below.
        pub encrypt_at_rest: pulumi_wasm_rust::Output<
            super::super::types::elasticsearch::DomainEncryptAtRest,
        >,
        /// Domain-specific endpoint used to submit index, search, and data upload requests.
        pub endpoint: pulumi_wasm_rust::Output<String>,
        /// Domain-specific endpoint for kibana without https scheme.
        pub kibana_endpoint: pulumi_wasm_rust::Output<String>,
        /// Configuration block for publishing slow and application logs to CloudWatch Logs. This block can be declared multiple times, for each log_type, within the same resource. Detailed below.
        pub log_publishing_options: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::elasticsearch::DomainLogPublishingOption>>,
        >,
        /// Configuration block for node-to-node encryption options. Detailed below.
        pub node_to_node_encryption: pulumi_wasm_rust::Output<
            super::super::types::elasticsearch::DomainNodeToNodeEncryption,
        >,
        /// Configuration block for snapshot related options. Detailed below. DEPRECATED. For domains running Elasticsearch 5.3 and later, Amazon ES takes hourly automated snapshots, making this setting irrelevant. For domains running earlier versions of Elasticsearch, Amazon ES takes daily automated snapshots.
        pub snapshot_options: pulumi_wasm_rust::Output<
            Option<super::super::types::elasticsearch::DomainSnapshotOptions>,
        >,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration block for VPC related options. Adding or removing this configuration forces a new resource ([documentation](https://docs.aws.amazon.com/elasticsearch-service/latest/developerguide/es-vpc.html#es-vpc-limitations)). Detailed below.
        pub vpc_options: pulumi_wasm_rust::Output<
            Option<super::super::types::elasticsearch::DomainVpcOptions>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DomainArgs) -> DomainResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_policies_binding = args.access_policies.get_inner();
        let advanced_options_binding = args.advanced_options.get_inner();
        let advanced_security_options_binding = args
            .advanced_security_options
            .get_inner();
        let auto_tune_options_binding = args.auto_tune_options.get_inner();
        let cluster_config_binding = args.cluster_config.get_inner();
        let cognito_options_binding = args.cognito_options.get_inner();
        let domain_endpoint_options_binding = args.domain_endpoint_options.get_inner();
        let domain_name_binding = args.domain_name.get_inner();
        let ebs_options_binding = args.ebs_options.get_inner();
        let elasticsearch_version_binding = args.elasticsearch_version.get_inner();
        let encrypt_at_rest_binding = args.encrypt_at_rest.get_inner();
        let log_publishing_options_binding = args.log_publishing_options.get_inner();
        let node_to_node_encryption_binding = args.node_to_node_encryption.get_inner();
        let snapshot_options_binding = args.snapshot_options.get_inner();
        let tags_binding = args.tags.get_inner();
        let vpc_options_binding = args.vpc_options.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:elasticsearch/domain:Domain".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessPolicies".into(),
                    value: &access_policies_binding,
                },
                register_interface::ObjectField {
                    name: "advancedOptions".into(),
                    value: &advanced_options_binding,
                },
                register_interface::ObjectField {
                    name: "advancedSecurityOptions".into(),
                    value: &advanced_security_options_binding,
                },
                register_interface::ObjectField {
                    name: "autoTuneOptions".into(),
                    value: &auto_tune_options_binding,
                },
                register_interface::ObjectField {
                    name: "clusterConfig".into(),
                    value: &cluster_config_binding,
                },
                register_interface::ObjectField {
                    name: "cognitoOptions".into(),
                    value: &cognito_options_binding,
                },
                register_interface::ObjectField {
                    name: "domainEndpointOptions".into(),
                    value: &domain_endpoint_options_binding,
                },
                register_interface::ObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding,
                },
                register_interface::ObjectField {
                    name: "ebsOptions".into(),
                    value: &ebs_options_binding,
                },
                register_interface::ObjectField {
                    name: "elasticsearchVersion".into(),
                    value: &elasticsearch_version_binding,
                },
                register_interface::ObjectField {
                    name: "encryptAtRest".into(),
                    value: &encrypt_at_rest_binding,
                },
                register_interface::ObjectField {
                    name: "logPublishingOptions".into(),
                    value: &log_publishing_options_binding,
                },
                register_interface::ObjectField {
                    name: "nodeToNodeEncryption".into(),
                    value: &node_to_node_encryption_binding,
                },
                register_interface::ObjectField {
                    name: "snapshotOptions".into(),
                    value: &snapshot_options_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "vpcOptions".into(),
                    value: &vpc_options_binding,
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
                    name: "clusterConfig".into(),
                },
                register_interface::ResultField {
                    name: "cognitoOptions".into(),
                },
                register_interface::ResultField {
                    name: "domainEndpointOptions".into(),
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
                    name: "encryptAtRest".into(),
                },
                register_interface::ResultField {
                    name: "endpoint".into(),
                },
                register_interface::ResultField {
                    name: "kibanaEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "logPublishingOptions".into(),
                },
                register_interface::ResultField {
                    name: "nodeToNodeEncryption".into(),
                },
                register_interface::ResultField {
                    name: "snapshotOptions".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "vpcOptions".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DomainResult {
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
            cluster_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterConfig").unwrap(),
            ),
            cognito_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cognitoOptions").unwrap(),
            ),
            domain_endpoint_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainEndpointOptions").unwrap(),
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
            encrypt_at_rest: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptAtRest").unwrap(),
            ),
            endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpoint").unwrap(),
            ),
            kibana_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kibanaEndpoint").unwrap(),
            ),
            log_publishing_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logPublishingOptions").unwrap(),
            ),
            node_to_node_encryption: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodeToNodeEncryption").unwrap(),
            ),
            snapshot_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snapshotOptions").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            vpc_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcOptions").unwrap(),
            ),
        }
    }
}
