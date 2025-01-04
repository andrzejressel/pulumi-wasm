/// Manages an Amazon OpenSearch Domain.
///
/// ## Elasticsearch vs. OpenSearch
///
/// Amazon OpenSearch Service is the successor to Amazon Elasticsearch Service and supports OpenSearch and legacy Elasticsearch OSS (up to 7.10, the final open source version of the software).
///
/// OpenSearch Domain configurations are similar in many ways to Elasticsearch Domain configurations. However, there are important differences including these:
///
/// * OpenSearch has `engine_version` while Elasticsearch has `elasticsearch_version`
/// * Versions are specified differently - _e.g._, `Elasticsearch_7.10` with OpenSearch vs. `7.10` for Elasticsearch.
/// * `instance_type` argument values end in `search` for OpenSearch vs. `elasticsearch` for Elasticsearch (_e.g._, `t2.micro.search` vs. `t2.micro.elasticsearch`).
/// * The AWS-managed service-linked role for OpenSearch is called `AWSServiceRoleForAmazonOpenSearchService` instead of `AWSServiceRoleForAmazonElasticsearchService` for Elasticsearch.
///
/// There are also some potentially unexpected similarities in configurations:
///
/// * ARNs for both are prefaced with `arn:aws:es:`.
/// * Both OpenSearch and Elasticsearch use assume role policies that refer to the `Principal` `Service` as `es.amazonaws.com`.
/// * IAM policy actions, such as those you will find in `access_policies`, are prefaced with `es:` for both.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:opensearch:Domain
///     properties:
///       domainName: example
///       engineVersion: Elasticsearch_7.10
///       clusterConfig:
///         instanceType: r4.large.search
///       tags:
///         Domain: TestDomain
/// ```
///
/// ### Access Policy
///
/// > See also: `aws.opensearch.DomainPolicy` resource
///
/// ```yaml
/// configuration:
///   domain:
///     type: string
///     default: tf-test
/// resources:
///   exampleDomain:
///     type: aws:opensearch:Domain
///     name: example
///     properties:
///       domainName: ${domain}
///       accessPolicies: ${example.json}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getRegion
///       arguments: {}
///   currentGetCallerIdentity:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
///   example:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: '*'
///                 identifiers:
///                   - '*'
///             actions:
///               - es:*
///             resources:
///               - arn:aws:es:${current.name}:${currentGetCallerIdentity.accountId}:domain/${domain}/*
///             conditions:
///               - test: IpAddress
///                 variable: aws:SourceIp
///                 values:
///                   - 66.193.100.22/32
/// ```
///
/// ### Log publishing to CloudWatch Logs
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
///     type: aws:opensearch:Domain
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
/// ### VPC based OpenSearch
///
/// ```yaml
/// configuration:
///   vpc:
///     type: dynamic
///   domain:
///     type: string
///     default: tf-test
/// resources:
///   exampleSecurityGroup:
///     type: aws:ec2:SecurityGroup
///     name: example
///     properties:
///       name: ${vpc}-opensearch-${domain}
///       description: Managed by Pulumi
///       vpcId: ${example.id}
///       ingress:
///         - fromPort: 443
///           toPort: 443
///           protocol: tcp
///           cidrBlocks:
///             - ${example.cidrBlock}
///   exampleServiceLinkedRole:
///     type: aws:iam:ServiceLinkedRole
///     name: example
///     properties:
///       awsServiceName: opensearchservice.amazonaws.com
///   exampleDomain:
///     type: aws:opensearch:Domain
///     name: example
///     properties:
///       domainName: ${domain}
///       engineVersion: OpenSearch_1.0
///       clusterConfig:
///         instanceType: m4.large.search
///         zoneAwarenessEnabled: true
///       vpcOptions:
///         subnetIds:
///           - ${exampleGetSubnets.ids[0]}
///           - ${exampleGetSubnets.ids[1]}
///         securityGroupIds:
///           - ${exampleSecurityGroup.id}
///       advancedOptions:
///         rest.action.multi.allow_explicit_index: 'true'
///       accessPolicies: ${exampleGetPolicyDocument.json}
///       tags:
///         Domain: TestDomain
///     options:
///       dependsOn:
///         - ${exampleServiceLinkedRole}
/// variables:
///   example:
///     fn::invoke:
///       function: aws:ec2:getVpc
///       arguments:
///         tags:
///           Name: ${vpc}
///   exampleGetSubnets:
///     fn::invoke:
///       function: aws:ec2:getSubnets
///       arguments:
///         filters:
///           - name: vpc-id
///             values:
///               - ${example.id}
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
///   exampleGetPolicyDocument:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: '*'
///                 identifiers:
///                   - '*'
///             actions:
///               - es:*
///             resources:
///               - arn:aws:es:${current.name}:${currentGetCallerIdentity.accountId}:domain/${domain}/*
/// ```
///
/// ### Enabling fine-grained access control on an existing domain
///
/// This example shows two configurations: one to create a domain without fine-grained access control and the second to modify the domain to enable fine-grained access control. For more information, see [Enabling fine-grained access control](https://docs.aws.amazon.com/opensearch-service/latest/developerguide/fgac.html).
///
/// ### First apply
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = domain::create(
///         "example",
///         DomainArgs::builder()
///             .advanced_security_options(
///                 DomainAdvancedSecurityOptions::builder()
///                     .anonymousAuthEnabled(true)
///                     .enabled(false)
///                     .internalUserDatabaseEnabled(true)
///                     .masterUserOptions(
///                         DomainAdvancedSecurityOptionsMasterUserOptions::builder()
///                             .masterUserName("example")
///                             .masterUserPassword("Barbarbarbar1!")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .cluster_config(
///                 DomainClusterConfig::builder()
///                     .instanceType("r5.large.search")
///                     .build_struct(),
///             )
///             .domain_endpoint_options(
///                 DomainDomainEndpointOptions::builder()
///                     .enforceHttps(true)
///                     .tlsSecurityPolicy("Policy-Min-TLS-1-2-2019-07")
///                     .build_struct(),
///             )
///             .domain_name("ggkitty")
///             .ebs_options(
///                 DomainEbsOptions::builder()
///                     .ebsEnabled(true)
///                     .volumeSize(10)
///                     .build_struct(),
///             )
///             .encrypt_at_rest(DomainEncryptAtRest::builder().enabled(true).build_struct())
///             .engine_version("Elasticsearch_7.1")
///             .node_to_node_encryption(
///                 DomainNodeToNodeEncryption::builder().enabled(true).build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Second apply
///
/// Notice that the only change is `advanced_security_options.0.enabled` is now set to `true`.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = domain::create(
///         "example",
///         DomainArgs::builder()
///             .advanced_security_options(
///                 DomainAdvancedSecurityOptions::builder()
///                     .anonymousAuthEnabled(true)
///                     .enabled(true)
///                     .internalUserDatabaseEnabled(true)
///                     .masterUserOptions(
///                         DomainAdvancedSecurityOptionsMasterUserOptions::builder()
///                             .masterUserName("example")
///                             .masterUserPassword("Barbarbarbar1!")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .cluster_config(
///                 DomainClusterConfig::builder()
///                     .instanceType("r5.large.search")
///                     .build_struct(),
///             )
///             .domain_endpoint_options(
///                 DomainDomainEndpointOptions::builder()
///                     .enforceHttps(true)
///                     .tlsSecurityPolicy("Policy-Min-TLS-1-2-2019-07")
///                     .build_struct(),
///             )
///             .domain_name("ggkitty")
///             .ebs_options(
///                 DomainEbsOptions::builder()
///                     .ebsEnabled(true)
///                     .volumeSize(10)
///                     .build_struct(),
///             )
///             .encrypt_at_rest(DomainEncryptAtRest::builder().enabled(true).build_struct())
///             .engine_version("Elasticsearch_7.1")
///             .node_to_node_encryption(
///                 DomainNodeToNodeEncryption::builder().enabled(true).build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import OpenSearch domains using the `domain_name`. For example:
///
/// ```sh
/// $ pulumi import aws:opensearch/domain:Domain example domain_name
/// ```
pub mod domain {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainArgs {
        /// IAM policy document specifying the access policies for the domain.
        #[builder(into, default)]
        pub access_policies: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value string pairs to specify advanced configuration options. Note that the values for these configuration options must be strings (wrapped in quotes) or they may be wrong and cause a perpetual diff, causing the provider to want to recreate your OpenSearch domain on every apply.
        #[builder(into, default)]
        pub advanced_options: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration block for [fine-grained access control](https://docs.aws.amazon.com/opensearch-service/latest/developerguide/fgac.html). Detailed below.
        #[builder(into, default)]
        pub advanced_security_options: pulumi_wasm_rust::Output<
            Option<super::super::types::opensearch::DomainAdvancedSecurityOptions>,
        >,
        /// Configuration block for the Auto-Tune options of the domain. Detailed below.
        #[builder(into, default)]
        pub auto_tune_options: pulumi_wasm_rust::Output<
            Option<super::super::types::opensearch::DomainAutoTuneOptions>,
        >,
        /// Configuration block for the cluster of the domain. Detailed below.
        #[builder(into, default)]
        pub cluster_config: pulumi_wasm_rust::Output<
            Option<super::super::types::opensearch::DomainClusterConfig>,
        >,
        /// Configuration block for authenticating dashboard with Cognito. Detailed below.
        #[builder(into, default)]
        pub cognito_options: pulumi_wasm_rust::Output<
            Option<super::super::types::opensearch::DomainCognitoOptions>,
        >,
        /// Configuration block for domain endpoint HTTP(S) related options. Detailed below.
        #[builder(into, default)]
        pub domain_endpoint_options: pulumi_wasm_rust::Output<
            Option<super::super::types::opensearch::DomainDomainEndpointOptions>,
        >,
        /// Name of the domain.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub domain_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block for EBS related options, may be required based on chosen [instance size](https://aws.amazon.com/opensearch-service/pricing/). Detailed below.
        #[builder(into, default)]
        pub ebs_options: pulumi_wasm_rust::Output<
            Option<super::super::types::opensearch::DomainEbsOptions>,
        >,
        /// Configuration block for encrypt at rest options. Only available for [certain instance types](https://docs.aws.amazon.com/opensearch-service/latest/developerguide/encryption-at-rest.html). Detailed below.
        #[builder(into, default)]
        pub encrypt_at_rest: pulumi_wasm_rust::Output<
            Option<super::super::types::opensearch::DomainEncryptAtRest>,
        >,
        /// Either `Elasticsearch_X.Y` or `OpenSearch_X.Y` to specify the engine version for the Amazon OpenSearch Service domain. For example, `OpenSearch_1.0` or `Elasticsearch_7.9`.
        /// See [Creating and managing Amazon OpenSearch Service domains](http://docs.aws.amazon.com/opensearch-service/latest/developerguide/createupdatedomains.html#createdomains).
        /// Defaults to the lastest version of OpenSearch.
        #[builder(into, default)]
        pub engine_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The IP address type for the endpoint. Valid values are `ipv4` and `dualstack`.
        #[builder(into, default)]
        pub ip_address_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block for publishing slow and application logs to CloudWatch Logs. This block can be declared multiple times, for each log_type, within the same resource. Detailed below.
        #[builder(into, default)]
        pub log_publishing_options: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::opensearch::DomainLogPublishingOption>>,
        >,
        /// Configuration block for node-to-node encryption options. Detailed below.
        #[builder(into, default)]
        pub node_to_node_encryption: pulumi_wasm_rust::Output<
            Option<super::super::types::opensearch::DomainNodeToNodeEncryption>,
        >,
        /// Configuration to add Off Peak update options. ([documentation](https://docs.aws.amazon.com/opensearch-service/latest/developerguide/off-peak.html)). Detailed below.
        #[builder(into, default)]
        pub off_peak_window_options: pulumi_wasm_rust::Output<
            Option<super::super::types::opensearch::DomainOffPeakWindowOptions>,
        >,
        /// Configuration block for snapshot related options. Detailed below. DEPRECATED. For domains running OpenSearch 5.3 and later, Amazon OpenSearch takes hourly automated snapshots, making this setting irrelevant. For domains running earlier versions, OpenSearch takes daily automated snapshots.
        #[builder(into, default)]
        pub snapshot_options: pulumi_wasm_rust::Output<
            Option<super::super::types::opensearch::DomainSnapshotOptions>,
        >,
        /// Software update options for the domain. Detailed below.
        #[builder(into, default)]
        pub software_update_options: pulumi_wasm_rust::Output<
            Option<super::super::types::opensearch::DomainSoftwareUpdateOptions>,
        >,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration block for VPC related options. Adding or removing this configuration forces a new resource ([documentation](https://docs.aws.amazon.com/opensearch-service/latest/developerguide/vpc.html)). Detailed below.
        #[builder(into, default)]
        pub vpc_options: pulumi_wasm_rust::Output<
            Option<super::super::types::opensearch::DomainVpcOptions>,
        >,
    }
    #[allow(dead_code)]
    pub struct DomainResult {
        /// IAM policy document specifying the access policies for the domain.
        pub access_policies: pulumi_wasm_rust::Output<String>,
        /// Key-value string pairs to specify advanced configuration options. Note that the values for these configuration options must be strings (wrapped in quotes) or they may be wrong and cause a perpetual diff, causing the provider to want to recreate your OpenSearch domain on every apply.
        pub advanced_options: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration block for [fine-grained access control](https://docs.aws.amazon.com/opensearch-service/latest/developerguide/fgac.html). Detailed below.
        pub advanced_security_options: pulumi_wasm_rust::Output<
            super::super::types::opensearch::DomainAdvancedSecurityOptions,
        >,
        /// ARN of the domain.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Configuration block for the Auto-Tune options of the domain. Detailed below.
        pub auto_tune_options: pulumi_wasm_rust::Output<
            super::super::types::opensearch::DomainAutoTuneOptions,
        >,
        /// Configuration block for the cluster of the domain. Detailed below.
        pub cluster_config: pulumi_wasm_rust::Output<
            super::super::types::opensearch::DomainClusterConfig,
        >,
        /// Configuration block for authenticating dashboard with Cognito. Detailed below.
        pub cognito_options: pulumi_wasm_rust::Output<
            Option<super::super::types::opensearch::DomainCognitoOptions>,
        >,
        /// Domain-specific endpoint for Dashboard without https scheme.
        pub dashboard_endpoint: pulumi_wasm_rust::Output<String>,
        /// V2 domain endpoint for Dashboard that works with both IPv4 and IPv6 addresses, without https scheme.
        pub dashboard_endpoint_v2: pulumi_wasm_rust::Output<String>,
        /// Configuration block for domain endpoint HTTP(S) related options. Detailed below.
        pub domain_endpoint_options: pulumi_wasm_rust::Output<
            super::super::types::opensearch::DomainDomainEndpointOptions,
        >,
        /// Dual stack hosted zone ID for the domain.
        pub domain_endpoint_v2_hosted_zone_id: pulumi_wasm_rust::Output<String>,
        /// Unique identifier for the domain.
        pub domain_id: pulumi_wasm_rust::Output<String>,
        /// Name of the domain.
        ///
        /// The following arguments are optional:
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// Configuration block for EBS related options, may be required based on chosen [instance size](https://aws.amazon.com/opensearch-service/pricing/). Detailed below.
        pub ebs_options: pulumi_wasm_rust::Output<
            super::super::types::opensearch::DomainEbsOptions,
        >,
        /// Configuration block for encrypt at rest options. Only available for [certain instance types](https://docs.aws.amazon.com/opensearch-service/latest/developerguide/encryption-at-rest.html). Detailed below.
        pub encrypt_at_rest: pulumi_wasm_rust::Output<
            super::super::types::opensearch::DomainEncryptAtRest,
        >,
        /// Domain-specific endpoint used to submit index, search, and data upload requests.
        pub endpoint: pulumi_wasm_rust::Output<String>,
        /// V2 domain endpoint that works with both IPv4 and IPv6 addresses, used to submit index, search, and data upload requests.
        pub endpoint_v2: pulumi_wasm_rust::Output<String>,
        /// Either `Elasticsearch_X.Y` or `OpenSearch_X.Y` to specify the engine version for the Amazon OpenSearch Service domain. For example, `OpenSearch_1.0` or `Elasticsearch_7.9`.
        /// See [Creating and managing Amazon OpenSearch Service domains](http://docs.aws.amazon.com/opensearch-service/latest/developerguide/createupdatedomains.html#createdomains).
        /// Defaults to the lastest version of OpenSearch.
        pub engine_version: pulumi_wasm_rust::Output<String>,
        /// The IP address type for the endpoint. Valid values are `ipv4` and `dualstack`.
        pub ip_address_type: pulumi_wasm_rust::Output<String>,
        /// (**Deprecated**) Domain-specific endpoint for kibana without https scheme. Use the `dashboard_endpoint` attribute instead.
        pub kibana_endpoint: pulumi_wasm_rust::Output<String>,
        /// Configuration block for publishing slow and application logs to CloudWatch Logs. This block can be declared multiple times, for each log_type, within the same resource. Detailed below.
        pub log_publishing_options: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::opensearch::DomainLogPublishingOption>>,
        >,
        /// Configuration block for node-to-node encryption options. Detailed below.
        pub node_to_node_encryption: pulumi_wasm_rust::Output<
            super::super::types::opensearch::DomainNodeToNodeEncryption,
        >,
        /// Configuration to add Off Peak update options. ([documentation](https://docs.aws.amazon.com/opensearch-service/latest/developerguide/off-peak.html)). Detailed below.
        pub off_peak_window_options: pulumi_wasm_rust::Output<
            super::super::types::opensearch::DomainOffPeakWindowOptions,
        >,
        /// Configuration block for snapshot related options. Detailed below. DEPRECATED. For domains running OpenSearch 5.3 and later, Amazon OpenSearch takes hourly automated snapshots, making this setting irrelevant. For domains running earlier versions, OpenSearch takes daily automated snapshots.
        pub snapshot_options: pulumi_wasm_rust::Output<
            Option<super::super::types::opensearch::DomainSnapshotOptions>,
        >,
        /// Software update options for the domain. Detailed below.
        pub software_update_options: pulumi_wasm_rust::Output<
            super::super::types::opensearch::DomainSoftwareUpdateOptions,
        >,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration block for VPC related options. Adding or removing this configuration forces a new resource ([documentation](https://docs.aws.amazon.com/opensearch-service/latest/developerguide/vpc.html)). Detailed below.
        pub vpc_options: pulumi_wasm_rust::Output<
            Option<super::super::types::opensearch::DomainVpcOptions>,
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
        let encrypt_at_rest_binding = args.encrypt_at_rest.get_inner();
        let engine_version_binding = args.engine_version.get_inner();
        let ip_address_type_binding = args.ip_address_type.get_inner();
        let log_publishing_options_binding = args.log_publishing_options.get_inner();
        let node_to_node_encryption_binding = args.node_to_node_encryption.get_inner();
        let off_peak_window_options_binding = args.off_peak_window_options.get_inner();
        let snapshot_options_binding = args.snapshot_options.get_inner();
        let software_update_options_binding = args.software_update_options.get_inner();
        let tags_binding = args.tags.get_inner();
        let vpc_options_binding = args.vpc_options.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:opensearch/domain:Domain".into(),
            name: name.to_string(),
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
                    name: "encryptAtRest".into(),
                    value: &encrypt_at_rest_binding,
                },
                register_interface::ObjectField {
                    name: "engineVersion".into(),
                    value: &engine_version_binding,
                },
                register_interface::ObjectField {
                    name: "ipAddressType".into(),
                    value: &ip_address_type_binding,
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
                    name: "offPeakWindowOptions".into(),
                    value: &off_peak_window_options_binding,
                },
                register_interface::ObjectField {
                    name: "snapshotOptions".into(),
                    value: &snapshot_options_binding,
                },
                register_interface::ObjectField {
                    name: "softwareUpdateOptions".into(),
                    value: &software_update_options_binding,
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
                    name: "dashboardEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "dashboardEndpointV2".into(),
                },
                register_interface::ResultField {
                    name: "domainEndpointOptions".into(),
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
                    name: "encryptAtRest".into(),
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
                    name: "ipAddressType".into(),
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
                    name: "offPeakWindowOptions".into(),
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
            dashboard_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dashboardEndpoint").unwrap(),
            ),
            dashboard_endpoint_v2: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dashboardEndpointV2").unwrap(),
            ),
            domain_endpoint_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainEndpointOptions").unwrap(),
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
            encrypt_at_rest: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptAtRest").unwrap(),
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
            ip_address_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipAddressType").unwrap(),
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
            off_peak_window_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("offPeakWindowOptions").unwrap(),
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
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            vpc_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcOptions").unwrap(),
            ),
        }
    }
}
