/// An Anthos node pool running on AWS.
///
/// For more information, see:
/// * [Multicloud overview](https://cloud.google.com/kubernetes-engine/multi-cloud/docs)
/// ## Example Usage
///
/// ### Basic_aws_cluster
/// A basic example of a containeraws node pool
/// ```yaml
/// resources:
///   primary:
///     type: gcp:container:AwsCluster
///     properties:
///       authorization:
///         adminUsers:
///           - username: my@service-account.com
///       awsRegion: my-aws-region
///       controlPlane:
///         awsServicesAuthentication:
///           roleArn: arn:aws:iam::012345678910:role/my--1p-dev-oneplatform
///           roleSessionName: my--1p-dev-session
///         configEncryption:
///           kmsKeyArn: arn:aws:kms:my-aws-region:012345678910:key/12345678-1234-1234-1234-123456789111
///         databaseEncryption:
///           kmsKeyArn: arn:aws:kms:my-aws-region:012345678910:key/12345678-1234-1234-1234-123456789111
///         iamInstanceProfile: my--1p-dev-controlplane
///         subnetIds:
///           - subnet-00000000000000000
///         version: ${versions.validVersions[0]}
///         instanceType: t3.medium
///         mainVolume:
///           iops: 3000
///           kmsKeyArn: arn:aws:kms:my-aws-region:012345678910:key/12345678-1234-1234-1234-123456789111
///           sizeGib: 10
///           volumeType: GP3
///         proxyConfig:
///           secretArn: arn:aws:secretsmanager:us-west-2:126285863215:secret:proxy_config20210824150329476300000001-ABCDEF
///           secretVersion: 12345678-ABCD-EFGH-IJKL-987654321098
///         rootVolume:
///           iops: 3000
///           kmsKeyArn: arn:aws:kms:my-aws-region:012345678910:key/12345678-1234-1234-1234-123456789111
///           sizeGib: 10
///           volumeType: GP3
///         securityGroupIds:
///           - sg-00000000000000000
///         sshConfig:
///           ec2KeyPair: my--1p-dev-ssh
///         tags:
///           owner: my@service-account.com
///       fleet:
///         project: my-project-number
///       location: us-west1
///       name: name
///       networking:
///         podAddressCidrBlocks:
///           - 10.2.0.0/16
///         serviceAddressCidrBlocks:
///           - 10.1.0.0/16
///         vpcId: vpc-00000000000000000
///       annotations:
///         label-one: value-one
///       description: A sample aws cluster
///       project: my-project-name
///   primaryAwsNodePool:
///     type: gcp:container:AwsNodePool
///     name: primary
///     properties:
///       autoscaling:
///         maxNodeCount: 5
///         minNodeCount: 1
///       cluster: ${primary.name}
///       config:
///         configEncryption:
///           kmsKeyArn: arn:aws:kms:my-aws-region:012345678910:key/12345678-1234-1234-1234-123456789111
///         iamInstanceProfile: my--1p-dev-nodepool
///         instanceType: t3.medium
///         labels:
///           label-one: value-one
///         rootVolume:
///           iops: 3000
///           kmsKeyArn: arn:aws:kms:my-aws-region:012345678910:key/12345678-1234-1234-1234-123456789111
///           sizeGib: 10
///           volumeType: GP3
///         securityGroupIds:
///           - sg-00000000000000000
///         proxyConfig:
///           secretArn: arn:aws:secretsmanager:us-west-2:126285863215:secret:proxy_config20210824150329476300000001-ABCDEF
///           secretVersion: 12345678-ABCD-EFGH-IJKL-987654321098
///         sshConfig:
///           ec2KeyPair: my--1p-dev-ssh
///         tags:
///           tag-one: value-one
///         taints:
///           - effect: PREFER_NO_SCHEDULE
///             key: taint-key
///             value: taint-value
///       location: us-west1
///       maxPodsConstraint:
///         maxPodsPerNode: 110
///       name: node-pool-name
///       subnetId: subnet-00000000000000000
///       version: ${versions.validVersions[0]}
///       annotations:
///         label-one: value-one
///       management:
///         autoRepair: true
///       kubeletConfig:
///         cpuManagerPolicy: none
///         cpuCfsQuota: true
///         cpuCfsQuotaPeriod: 100ms
///         podPidsLimit: 1024
///       project: my-project-name
/// variables:
///   versions:
///     fn::invoke:
///       function: gcp:container:getAwsVersions
///       arguments:
///         project: my-project-name
///         location: us-west1
/// ```
/// ### Basic_enum_aws_cluster
/// A basic example of a containeraws node pool with lowercase enums
/// ```yaml
/// resources:
///   primary:
///     type: gcp:container:AwsCluster
///     properties:
///       authorization:
///         adminUsers:
///           - username: my@service-account.com
///       awsRegion: my-aws-region
///       controlPlane:
///         awsServicesAuthentication:
///           roleArn: arn:aws:iam::012345678910:role/my--1p-dev-oneplatform
///           roleSessionName: my--1p-dev-session
///         configEncryption:
///           kmsKeyArn: arn:aws:kms:my-aws-region:012345678910:key/12345678-1234-1234-1234-123456789111
///         databaseEncryption:
///           kmsKeyArn: arn:aws:kms:my-aws-region:012345678910:key/12345678-1234-1234-1234-123456789111
///         iamInstanceProfile: my--1p-dev-controlplane
///         subnetIds:
///           - subnet-00000000000000000
///         version: ${versions.validVersions[0]}
///         instanceType: t3.medium
///         mainVolume:
///           iops: 3000
///           kmsKeyArn: arn:aws:kms:my-aws-region:012345678910:key/12345678-1234-1234-1234-123456789111
///           sizeGib: 10
///           volumeType: GP3
///         proxyConfig:
///           secretArn: arn:aws:secretsmanager:us-west-2:126285863215:secret:proxy_config20210824150329476300000001-ABCDEF
///           secretVersion: 12345678-ABCD-EFGH-IJKL-987654321098
///         rootVolume:
///           iops: 3000
///           kmsKeyArn: arn:aws:kms:my-aws-region:012345678910:key/12345678-1234-1234-1234-123456789111
///           sizeGib: 10
///           volumeType: GP3
///         securityGroupIds:
///           - sg-00000000000000000
///         sshConfig:
///           ec2KeyPair: my--1p-dev-ssh
///         tags:
///           owner: my@service-account.com
///       fleet:
///         project: my-project-number
///       location: us-west1
///       name: name
///       networking:
///         podAddressCidrBlocks:
///           - 10.2.0.0/16
///         serviceAddressCidrBlocks:
///           - 10.1.0.0/16
///         vpcId: vpc-00000000000000000
///       annotations:
///         label-one: value-one
///       description: A sample aws cluster
///       project: my-project-name
///   primaryAwsNodePool:
///     type: gcp:container:AwsNodePool
///     name: primary
///     properties:
///       autoscaling:
///         maxNodeCount: 5
///         minNodeCount: 1
///       cluster: ${primary.name}
///       config:
///         configEncryption:
///           kmsKeyArn: arn:aws:kms:my-aws-region:012345678910:key/12345678-1234-1234-1234-123456789111
///         iamInstanceProfile: my--1p-dev-nodepool
///         instanceType: t3.medium
///         labels:
///           label-one: value-one
///         rootVolume:
///           iops: 3000
///           kmsKeyArn: arn:aws:kms:my-aws-region:012345678910:key/12345678-1234-1234-1234-123456789111
///           sizeGib: 10
///           volumeType: gp3
///         securityGroupIds:
///           - sg-00000000000000000
///         proxyConfig:
///           secretArn: arn:aws:secretsmanager:us-west-2:126285863215:secret:proxy_config20210824150329476300000001-ABCDEF
///           secretVersion: 12345678-ABCD-EFGH-IJKL-987654321098
///         sshConfig:
///           ec2KeyPair: my--1p-dev-ssh
///         tags:
///           tag-one: value-one
///         taints:
///           - effect: prefer_no_schedule
///             key: taint-key
///             value: taint-value
///       location: us-west1
///       maxPodsConstraint:
///         maxPodsPerNode: 110
///       name: node-pool-name
///       subnetId: subnet-00000000000000000
///       version: ${versions.validVersions[0]}
///       annotations:
///         label-one: value-one
///       project: my-project-name
/// variables:
///   versions:
///     fn::invoke:
///       function: gcp:container:getAwsVersions
///       arguments:
///         project: my-project-name
///         location: us-west1
/// ```
/// ### Beta_basic_enum_aws_cluster
/// A basic example of a containeraws node pool with lowercase enums (beta)
/// ```yaml
/// resources:
///   primary:
///     type: gcp:container:AwsCluster
///     properties:
///       authorization:
///         adminUsers:
///           - username: my@service-account.com
///       awsRegion: my-aws-region
///       controlPlane:
///         awsServicesAuthentication:
///           roleArn: arn:aws:iam::012345678910:role/my--1p-dev-oneplatform
///           roleSessionName: my--1p-dev-session
///         configEncryption:
///           kmsKeyArn: arn:aws:kms:my-aws-region:012345678910:key/12345678-1234-1234-1234-123456789111
///         databaseEncryption:
///           kmsKeyArn: arn:aws:kms:my-aws-region:012345678910:key/12345678-1234-1234-1234-123456789111
///         iamInstanceProfile: my--1p-dev-controlplane
///         subnetIds:
///           - subnet-00000000000000000
///         version: ${versions.validVersions[0]}
///         instanceType: t3.medium
///         mainVolume:
///           iops: 3000
///           kmsKeyArn: arn:aws:kms:my-aws-region:012345678910:key/12345678-1234-1234-1234-123456789111
///           sizeGib: 10
///           volumeType: GP3
///         proxyConfig:
///           secretArn: arn:aws:secretsmanager:us-west-2:126285863215:secret:proxy_config20210824150329476300000001-ABCDEF
///           secretVersion: 12345678-ABCD-EFGH-IJKL-987654321098
///         rootVolume:
///           iops: 3000
///           kmsKeyArn: arn:aws:kms:my-aws-region:012345678910:key/12345678-1234-1234-1234-123456789111
///           sizeGib: 10
///           volumeType: GP3
///         securityGroupIds:
///           - sg-00000000000000000
///         sshConfig:
///           ec2KeyPair: my--1p-dev-ssh
///         tags:
///           owner: my@service-account.com
///       fleet:
///         project: my-project-number
///       location: us-west1
///       name: name
///       networking:
///         podAddressCidrBlocks:
///           - 10.2.0.0/16
///         serviceAddressCidrBlocks:
///           - 10.1.0.0/16
///         vpcId: vpc-00000000000000000
///       annotations:
///         label-one: value-one
///       description: A sample aws cluster
///       project: my-project-name
///   primaryAwsNodePool:
///     type: gcp:container:AwsNodePool
///     name: primary
///     properties:
///       autoscaling:
///         maxNodeCount: 5
///         minNodeCount: 1
///       cluster: ${primary.name}
///       config:
///         configEncryption:
///           kmsKeyArn: arn:aws:kms:my-aws-region:012345678910:key/12345678-1234-1234-1234-123456789111
///         iamInstanceProfile: my--1p-dev-nodepool
///         instanceType: t3.medium
///         labels:
///           label-one: value-one
///         rootVolume:
///           iops: 3000
///           kmsKeyArn: arn:aws:kms:my-aws-region:012345678910:key/12345678-1234-1234-1234-123456789111
///           sizeGib: 10
///           volumeType: gp3
///         securityGroupIds:
///           - sg-00000000000000000
///         proxyConfig:
///           secretArn: arn:aws:secretsmanager:us-west-2:126285863215:secret:proxy_config20210824150329476300000001-ABCDEF
///           secretVersion: 12345678-ABCD-EFGH-IJKL-987654321098
///         sshConfig:
///           ec2KeyPair: my--1p-dev-ssh
///         tags:
///           tag-one: value-one
///         taints:
///           - effect: prefer_no_schedule
///             key: taint-key
///             value: taint-value
///         instancePlacement:
///           tenancy: dedicated
///         imageType: ubuntu
///       location: us-west1
///       maxPodsConstraint:
///         maxPodsPerNode: 110
///       name: node-pool-name
///       subnetId: subnet-00000000000000000
///       version: ${versions.validVersions[0]}
///       annotations:
///         label-one: value-one
///       project: my-project-name
/// variables:
///   versions:
///     fn::invoke:
///       function: gcp:container:getAwsVersions
///       arguments:
///         project: my-project-name
///         location: us-west1
/// ```
///
/// ## Import
///
/// NodePool can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/awsClusters/{{cluster}}/awsNodePools/{{name}}`
///
/// * `{{project}}/{{location}}/{{cluster}}/{{name}}`
///
/// * `{{location}}/{{cluster}}/{{name}}`
///
/// When using the `pulumi import` command, NodePool can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:container/awsNodePool:AwsNodePool default projects/{{project}}/locations/{{location}}/awsClusters/{{cluster}}/awsNodePools/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:container/awsNodePool:AwsNodePool default {{project}}/{{location}}/{{cluster}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:container/awsNodePool:AwsNodePool default {{location}}/{{cluster}}/{{name}}
/// ```
///
pub mod aws_node_pool {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AwsNodePoolArgs {
        /// Optional. Annotations on the node pool. This field has the same restrictions as Kubernetes annotations. The total size
        /// of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required),
        /// separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with
        /// alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. **Note**: This field is
        /// non-authoritative, and will only manage the annotations present in your configuration. Please refer to the field
        /// `effective_annotations` for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Autoscaler configuration for this node pool.
        #[builder(into)]
        pub autoscaling: pulumi_wasm_rust::InputOrOutput<
            super::super::types::container::AwsNodePoolAutoscaling,
        >,
        /// The awsCluster for the resource
        #[builder(into)]
        pub cluster: pulumi_wasm_rust::InputOrOutput<String>,
        /// The configuration of the node pool.
        #[builder(into)]
        pub config: pulumi_wasm_rust::InputOrOutput<
            super::super::types::container::AwsNodePoolConfig,
        >,
        /// The kubelet configuration for the node pool.
        #[builder(into, default)]
        pub kubelet_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::container::AwsNodePoolKubeletConfig>,
        >,
        /// The location for the resource
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Management configuration for this node pool.
        #[builder(into, default)]
        pub management: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::container::AwsNodePoolManagement>,
        >,
        /// The constraint on the maximum number of pods that can be run simultaneously on a node in the node pool.
        #[builder(into)]
        pub max_pods_constraint: pulumi_wasm_rust::InputOrOutput<
            super::super::types::container::AwsNodePoolMaxPodsConstraint,
        >,
        /// The name of this resource.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The project for the resource
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The subnet where the node pool node run.
        #[builder(into)]
        pub subnet_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Optional. Update settings control the speed and disruption of the node pool update.
        #[builder(into, default)]
        pub update_settings: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::container::AwsNodePoolUpdateSettings>,
        >,
        /// The Kubernetes version to run on this node pool (e.g. `1.19.10-gke.1000`). You can list all supported versions on a given Google Cloud region by calling GetAwsServerConfig.
        #[builder(into)]
        pub version: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AwsNodePoolResult {
        /// Optional. Annotations on the node pool. This field has the same restrictions as Kubernetes annotations. The total size
        /// of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required),
        /// separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with
        /// alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. **Note**: This field is
        /// non-authoritative, and will only manage the annotations present in your configuration. Please refer to the field
        /// `effective_annotations` for all of the annotations present on the resource.
        pub annotations: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Autoscaler configuration for this node pool.
        pub autoscaling: pulumi_wasm_rust::Output<
            super::super::types::container::AwsNodePoolAutoscaling,
        >,
        /// The awsCluster for the resource
        pub cluster: pulumi_wasm_rust::Output<String>,
        /// The configuration of the node pool.
        pub config: pulumi_wasm_rust::Output<
            super::super::types::container::AwsNodePoolConfig,
        >,
        /// Output only. The time at which this node pool was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        pub effective_annotations: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Allows clients to perform consistent read-modify-writes through optimistic concurrency control. May be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The kubelet configuration for the node pool.
        pub kubelet_config: pulumi_wasm_rust::Output<
            super::super::types::container::AwsNodePoolKubeletConfig,
        >,
        /// The location for the resource
        pub location: pulumi_wasm_rust::Output<String>,
        /// The Management configuration for this node pool.
        pub management: pulumi_wasm_rust::Output<
            super::super::types::container::AwsNodePoolManagement,
        >,
        /// The constraint on the maximum number of pods that can be run simultaneously on a node in the node pool.
        pub max_pods_constraint: pulumi_wasm_rust::Output<
            super::super::types::container::AwsNodePoolMaxPodsConstraint,
        >,
        /// The name of this resource.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The project for the resource
        pub project: pulumi_wasm_rust::Output<String>,
        /// Output only. If set, there are currently changes in flight to the node pool.
        pub reconciling: pulumi_wasm_rust::Output<bool>,
        /// Output only. The lifecycle state of the node pool. Possible values: STATE_UNSPECIFIED, PROVISIONING, RUNNING, RECONCILING, STOPPING, ERROR, DEGRADED
        pub state: pulumi_wasm_rust::Output<String>,
        /// The subnet where the node pool node run.
        pub subnet_id: pulumi_wasm_rust::Output<String>,
        /// Output only. A globally unique identifier for the node pool.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// Optional. Update settings control the speed and disruption of the node pool update.
        pub update_settings: pulumi_wasm_rust::Output<
            super::super::types::container::AwsNodePoolUpdateSettings,
        >,
        /// Output only. The time at which this node pool was last updated.
        pub update_time: pulumi_wasm_rust::Output<String>,
        /// The Kubernetes version to run on this node pool (e.g. `1.19.10-gke.1000`). You can list all supported versions on a given Google Cloud region by calling GetAwsServerConfig.
        pub version: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AwsNodePoolArgs,
    ) -> AwsNodePoolResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let annotations_binding = args.annotations.get_output(context).get_inner();
        let autoscaling_binding = args.autoscaling.get_output(context).get_inner();
        let cluster_binding = args.cluster.get_output(context).get_inner();
        let config_binding = args.config.get_output(context).get_inner();
        let kubelet_config_binding = args.kubelet_config.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let management_binding = args.management.get_output(context).get_inner();
        let max_pods_constraint_binding = args
            .max_pods_constraint
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let subnet_id_binding = args.subnet_id.get_output(context).get_inner();
        let update_settings_binding = args
            .update_settings
            .get_output(context)
            .get_inner();
        let version_binding = args.version.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:container/awsNodePool:AwsNodePool".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding,
                },
                register_interface::ObjectField {
                    name: "autoscaling".into(),
                    value: &autoscaling_binding,
                },
                register_interface::ObjectField {
                    name: "cluster".into(),
                    value: &cluster_binding,
                },
                register_interface::ObjectField {
                    name: "config".into(),
                    value: &config_binding,
                },
                register_interface::ObjectField {
                    name: "kubeletConfig".into(),
                    value: &kubelet_config_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "management".into(),
                    value: &management_binding,
                },
                register_interface::ObjectField {
                    name: "maxPodsConstraint".into(),
                    value: &max_pods_constraint_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding,
                },
                register_interface::ObjectField {
                    name: "updateSettings".into(),
                    value: &update_settings_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "annotations".into(),
                },
                register_interface::ResultField {
                    name: "autoscaling".into(),
                },
                register_interface::ResultField {
                    name: "cluster".into(),
                },
                register_interface::ResultField {
                    name: "config".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "effectiveAnnotations".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "kubeletConfig".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "management".into(),
                },
                register_interface::ResultField {
                    name: "maxPodsConstraint".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "reconciling".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "subnetId".into(),
                },
                register_interface::ResultField {
                    name: "uid".into(),
                },
                register_interface::ResultField {
                    name: "updateSettings".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AwsNodePoolResult {
            annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("annotations").unwrap(),
            ),
            autoscaling: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoscaling").unwrap(),
            ),
            cluster: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cluster").unwrap(),
            ),
            config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("config").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            effective_annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveAnnotations").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            kubelet_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kubeletConfig").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            management: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("management").unwrap(),
            ),
            max_pods_constraint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxPodsConstraint").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            reconciling: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reconciling").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetId").unwrap(),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uid").unwrap(),
            ),
            update_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateSettings").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
