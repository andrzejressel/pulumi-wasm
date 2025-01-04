/// Provides an Elastic MapReduce Cluster, a web service that makes it easy to process large amounts of data efficiently. See [Amazon Elastic MapReduce Documentation](https://aws.amazon.com/documentation/elastic-mapreduce/) for more information.
///
/// To configure [Instance Groups](https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-instance-group-configuration.html#emr-plan-instance-groups) for [task nodes](https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-master-core-task-nodes.html#emr-plan-task), see the `aws.emr.InstanceGroup` resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   cluster:
///     type: aws:emr:Cluster
///     properties:
///       name: emr-test-arn
///       releaseLabel: emr-4.6.0
///       applications:
///         - Spark
///       additionalInfo: |
///         {
///           "instanceAwsClientConfiguration": {
///             "proxyPort": 8099,
///             "proxyHost": "myproxy.example.com"
///           }
///         }
///       terminationProtection: false
///       keepJobFlowAliveWhenNoSteps: true
///       ec2Attributes:
///         subnetId: ${main.id}
///         emrManagedMasterSecurityGroup: ${sg.id}
///         emrManagedSlaveSecurityGroup: ${sg.id}
///         instanceProfile: ${emrProfile.arn}
///       masterInstanceGroup:
///         instanceType: m4.large
///       coreInstanceGroup:
///         instanceType: c4.large
///         instanceCount: 1
///         ebsConfigs:
///           - size: '40'
///             type: gp2
///             volumesPerInstance: 1
///         bidPrice: '0.30'
///         autoscalingPolicy: |
///           {
///           "Constraints": {
///             "MinCapacity": 1,
///             "MaxCapacity": 2
///           },
///           "Rules": [
///             {
///               "Name": "ScaleOutMemoryPercentage",
///               "Description": "Scale out if YARNMemoryAvailablePercentage is less than 15",
///               "Action": {
///                 "SimpleScalingPolicyConfiguration": {
///                   "AdjustmentType": "CHANGE_IN_CAPACITY",
///                   "ScalingAdjustment": 1,
///                   "CoolDown": 300
///                 }
///               },
///               "Trigger": {
///                 "CloudWatchAlarmDefinition": {
///                   "ComparisonOperator": "LESS_THAN",
///                   "EvaluationPeriods": 1,
///                   "MetricName": "YARNMemoryAvailablePercentage",
///                   "Namespace": "AWS/ElasticMapReduce",
///                   "Period": 300,
///                   "Statistic": "AVERAGE",
///                   "Threshold": 15.0,
///                   "Unit": "PERCENT"
///                 }
///               }
///             }
///           ]
///           }
///       ebsRootVolumeSize: 100
///       tags:
///         role: rolename
///         env: env
///       bootstrapActions:
///         - path: s3://elasticmapreduce/bootstrap-actions/run-if
///           name: runif
///           args:
///             - instance.isMaster=true
///             - echo running on master node
///       configurationsJson: |2
///           [
///             {
///               "Classification": "hadoop-env",
///               "Configurations": [
///                 {
///                   "Classification": "export",
///                   "Properties": {
///                     "JAVA_HOME": "/usr/lib/jvm/java-1.8.0"
///                   }
///                 }
///               ],
///               "Properties": {}
///             },
///             {
///               "Classification": "spark-env",
///               "Configurations": [
///                 {
///                   "Classification": "export",
///                   "Properties": {
///                     "JAVA_HOME": "/usr/lib/jvm/java-1.8.0"
///                   }
///                 }
///               ],
///               "Properties": {}
///             }
///           ]
///       serviceRole: ${iamEmrServiceRole.arn}
/// ```
///
/// The `aws.emr.Cluster` resource typically requires two IAM roles, one for the EMR Cluster to use as a service role, and another is assigned to every EC2 instance in a cluster and each application process that runs on a cluster assumes this role for permissions to interact with other AWS services. An additional role, the Auto Scaling role, is required if your cluster uses automatic scaling in Amazon EMR.
///
/// The default AWS managed EMR service role is called `EMR_DefaultRole` with Amazon managed policy `AmazonEMRServicePolicy_v2` attached. The name of default instance profile role is `EMR_EC2_DefaultRole` with default managed policy `AmazonElasticMapReduceforEC2Role` attached, but it is on the path to deprecation and will not be replaced with another default managed policy. You'll need to create and specify an instance profile to replace the deprecated role and default policy. See the [Configure IAM service roles for Amazon EMR](https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-iam-roles.html) guide for more information on these IAM roles. There is also a fully-bootable example Pulumi configuration at the bottom of this page.
///
/// ### Instance Fleet
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = cluster::create(
///         "example",
///         ClusterArgs::builder()
///             .core_instance_fleet(
///                 ClusterCoreInstanceFleet::builder()
///                     .instanceTypeConfigs(
///                         vec![
///                             ClusterCoreInstanceFleetInstanceTypeConfig::builder()
///                             .bidPriceAsPercentageOfOnDemandPrice(80)
///                             .ebsConfigs(vec![ClusterCoreInstanceFleetInstanceTypeConfigEbsConfig::builder()
///                             .size(100). type ("gp2").volumesPerInstance(1)
///                             .build_struct(),]).instanceType("m3.xlarge")
///                             .weightedCapacity(1).build_struct(),
///                             ClusterCoreInstanceFleetInstanceTypeConfig::builder()
///                             .bidPriceAsPercentageOfOnDemandPrice(100)
///                             .ebsConfigs(vec![ClusterCoreInstanceFleetInstanceTypeConfigEbsConfig::builder()
///                             .size(100). type ("gp2").volumesPerInstance(1)
///                             .build_struct(),]).instanceType("m4.xlarge")
///                             .weightedCapacity(1).build_struct(),
///                             ClusterCoreInstanceFleetInstanceTypeConfig::builder()
///                             .bidPriceAsPercentageOfOnDemandPrice(100)
///                             .ebsConfigs(vec![ClusterCoreInstanceFleetInstanceTypeConfigEbsConfig::builder()
///                             .size(100). type ("gp2").volumesPerInstance(1)
///                             .build_struct(),]).instanceType("m4.2xlarge")
///                             .weightedCapacity(2).build_struct(),
///                         ],
///                     )
///                     .launchSpecifications(
///                         ClusterCoreInstanceFleetLaunchSpecifications::builder()
///                             .spotSpecifications(
///                                 vec![
///                                     ClusterCoreInstanceFleetLaunchSpecificationsSpotSpecification::builder()
///                                     .allocationStrategy("capacity-optimized")
///                                     .blockDurationMinutes(0)
///                                     .timeoutAction("SWITCH_TO_ON_DEMAND")
///                                     .timeoutDurationMinutes(10).build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .name("core fleet")
///                     .targetOnDemandCapacity(2)
///                     .targetSpotCapacity(2)
///                     .build_struct(),
///             )
///             .master_instance_fleet(
///                 ClusterMasterInstanceFleet::builder()
///                     .instanceTypeConfigs(
///                         vec![
///                             ClusterMasterInstanceFleetInstanceTypeConfig::builder()
///                             .instanceType("m4.xlarge").build_struct(),
///                         ],
///                     )
///                     .targetOnDemandCapacity(1)
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let task = instance_fleet::create(
///         "task",
///         InstanceFleetArgs::builder()
///             .cluster_id("${example.id}")
///             .instance_type_configs(
///                 vec![
///                     InstanceFleetInstanceTypeConfig::builder()
///                     .bidPriceAsPercentageOfOnDemandPrice(100)
///                     .ebsConfigs(vec![InstanceFleetInstanceTypeConfigEbsConfig::builder()
///                     .size(100). type ("gp2").volumesPerInstance(1).build_struct(),])
///                     .instanceType("m4.xlarge").weightedCapacity(1).build_struct(),
///                     InstanceFleetInstanceTypeConfig::builder()
///                     .bidPriceAsPercentageOfOnDemandPrice(100)
///                     .ebsConfigs(vec![InstanceFleetInstanceTypeConfigEbsConfig::builder()
///                     .size(100). type ("gp2").volumesPerInstance(1).build_struct(),])
///                     .instanceType("m4.2xlarge").weightedCapacity(2).build_struct(),
///                 ],
///             )
///             .launch_specifications(
///                 InstanceFleetLaunchSpecifications::builder()
///                     .spotSpecifications(
///                         vec![
///                             InstanceFleetLaunchSpecificationsSpotSpecification::builder()
///                             .allocationStrategy("capacity-optimized")
///                             .blockDurationMinutes(0).timeoutAction("TERMINATE_CLUSTER")
///                             .timeoutDurationMinutes(10).build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .name("task fleet")
///             .target_on_demand_capacity(1)
///             .target_spot_capacity(1)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Enable Debug Logging
///
/// [Debug logging in EMR](https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-plan-debugging.html) is implemented as a step. It is highly recommended that you utilize the resource options configuration with `ignoreChanges` if other steps are being managed outside of this provider.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = cluster::create(
///         "example",
///         ClusterArgs::builder()
///             .steps(
///                 vec![
///                     ClusterStep::builder().actionOnFailure("TERMINATE_CLUSTER")
///                     .hadoopJarStep(ClusterStepHadoopJarStep::builder()
///                     .args(vec!["state-pusher-script",]).jar("command-runner.jar")
///                     .build_struct()).name("Setup Hadoop Debugging").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Multiple Node Master Instance Group
///
/// Available in EMR version 5.23.0 and later, an EMR Cluster can be launched with three master nodes for high availability. Additional information about this functionality and its requirements can be found in the [EMR Management Guide](https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-plan-ha.html).
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = subnet::create(
///         "example",
///         SubnetArgs::builder().map_public_ip_on_launch(true).build_struct(),
///     );
///     let exampleCluster = cluster::create(
///         "exampleCluster",
///         ClusterArgs::builder()
///             .core_instance_group(ClusterCoreInstanceGroup::builder().build_struct())
///             .ec_2_attributes(
///                 ClusterEc2Attributes::builder().subnetId("${example.id}").build_struct(),
///             )
///             .master_instance_group(
///                 ClusterMasterInstanceGroup::builder().instanceCount(3).build_struct(),
///             )
///             .release_label("emr-5.24.1")
///             .termination_protection(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EMR clusters using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:emr/cluster:Cluster cluster j-123456ABCDEF
/// ```
/// Since the API does not return the actual values for Kerberos configurations, environments with those options set will need to use the `lifecycle` configuration block `ignore_changes` argument available to all Pulumi resources to prevent perpetual differences. For example:
///
pub mod cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterArgs {
        /// JSON string for selecting additional features such as adding proxy information. Note: Currently there is no API to retrieve the value of this argument after EMR cluster creation from provider, therefore the provider cannot detect drift from the actual EMR cluster if its value is changed outside the provider.
        #[builder(into, default)]
        pub additional_info: pulumi_wasm_rust::Output<Option<String>>,
        /// A case-insensitive list of applications for Amazon EMR to install and configure when launching the cluster. For a list of applications available for each Amazon EMR release version, see the [Amazon EMR Release Guide](https://docs.aws.amazon.com/emr/latest/ReleaseGuide/emr-release-components.html).
        #[builder(into, default)]
        pub applications: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// An auto-termination policy for an Amazon EMR cluster. An auto-termination policy defines the amount of idle time in seconds after which a cluster automatically terminates. See Auto Termination Policy Below.
        #[builder(into, default)]
        pub auto_termination_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::emr::ClusterAutoTerminationPolicy>,
        >,
        /// IAM role for automatic scaling policies. The IAM role provides permissions that the automatic scaling feature requires to launch and terminate EC2 instances in an instance group.
        #[builder(into, default)]
        pub autoscaling_role: pulumi_wasm_rust::Output<Option<String>>,
        /// Ordered list of bootstrap actions that will be run before Hadoop is started on the cluster nodes. See below.
        #[builder(into, default)]
        pub bootstrap_actions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::emr::ClusterBootstrapAction>>,
        >,
        /// List of configurations supplied for the EMR cluster you are creating. Supply a configuration object for applications to override their default configuration. See [AWS Documentation](https://docs.aws.amazon.com/emr/latest/ReleaseGuide/emr-configure-apps.html) for more information.
        #[builder(into, default)]
        pub configurations: pulumi_wasm_rust::Output<Option<String>>,
        /// JSON string for supplying list of configurations for the EMR cluster.
        ///
        /// > **NOTE on `configurations_json`:** If the `Configurations` value is empty then you should skip the `Configurations` field instead of providing an empty list as a value, `"Configurations": []`.
        ///
        /// ```ignore
        /// use pulumi_wasm_rust::Output;
        /// use pulumi_wasm_rust::{add_export, pulumi_main};
        /// #[pulumi_main]
        /// fn test_main() -> Result<(), Error> {
        ///     let cluster = cluster::create(
        ///         "cluster",
        ///         ClusterArgs::builder()
        ///             .configurations_json(
        ///                 "[\n{\n\"Classification\": \"hadoop-env\",\n\"Configurations\": [\n{\n\"Classification\": \"export\",\n\"Properties\": {\n\"JAVA_HOME\": \"/usr/lib/jvm/java-1.8.0\"\n}\n}\n],\n\"Properties\": {}\n}\n]",
        ///             )
        ///             .build_struct(),
        ///     );
        /// }
        /// ```
        #[builder(into, default)]
        pub configurations_json: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block to use an [Instance Fleet](https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-instance-fleet.html) for the core node type. Cannot be specified if any `core_instance_group` configuration blocks are set. Detailed below.
        #[builder(into, default)]
        pub core_instance_fleet: pulumi_wasm_rust::Output<
            Option<super::super::types::emr::ClusterCoreInstanceFleet>,
        >,
        /// Configuration block to use an [Instance Group](https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-instance-group-configuration.html#emr-plan-instance-groups) for the [core node type](https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-master-core-task-nodes.html#emr-plan-core).
        #[builder(into, default)]
        pub core_instance_group: pulumi_wasm_rust::Output<
            Option<super::super::types::emr::ClusterCoreInstanceGroup>,
        >,
        /// Custom Amazon Linux AMI for the cluster (instead of an EMR-owned AMI). Available in Amazon EMR version 5.7.0 and later.
        #[builder(into, default)]
        pub custom_ami_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Size in GiB of the EBS root device volume of the Linux AMI that is used for each EC2 instance. Available in Amazon EMR version 4.x and later.
        #[builder(into, default)]
        pub ebs_root_volume_size: pulumi_wasm_rust::Output<Option<i32>>,
        /// Attributes for the EC2 instances running the job flow. See below.
        #[builder(into, default)]
        pub ec2_attributes: pulumi_wasm_rust::Output<
            Option<super::super::types::emr::ClusterEc2Attributes>,
        >,
        /// Switch on/off run cluster with no steps or when all steps are complete (default is on)
        #[builder(into, default)]
        pub keep_job_flow_alive_when_no_steps: pulumi_wasm_rust::Output<Option<bool>>,
        /// Kerberos configuration for the cluster. See below.
        #[builder(into, default)]
        pub kerberos_attributes: pulumi_wasm_rust::Output<
            Option<super::super::types::emr::ClusterKerberosAttributes>,
        >,
        /// List of [step states](https://docs.aws.amazon.com/emr/latest/APIReference/API_StepStatus.html) used to filter returned steps
        #[builder(into, default)]
        pub list_steps_states: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// AWS KMS customer master key (CMK) key ID or arn used for encrypting log files. This attribute is only available with EMR version 5.30.0 and later, excluding EMR 6.0.0.
        #[builder(into, default)]
        pub log_encryption_kms_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// S3 bucket to write the log files of the job flow. If a value is not provided, logs are not created.
        #[builder(into, default)]
        pub log_uri: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block to use an [Instance Fleet](https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-instance-fleet.html) for the master node type. Cannot be specified if any `master_instance_group` configuration blocks are set. Detailed below.
        #[builder(into, default)]
        pub master_instance_fleet: pulumi_wasm_rust::Output<
            Option<super::super::types::emr::ClusterMasterInstanceFleet>,
        >,
        /// Configuration block to use an [Instance Group](https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-instance-group-configuration.html#emr-plan-instance-groups) for the [master node type](https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-master-core-task-nodes.html#emr-plan-master).
        #[builder(into, default)]
        pub master_instance_group: pulumi_wasm_rust::Output<
            Option<super::super::types::emr::ClusterMasterInstanceGroup>,
        >,
        /// Name of the job flow.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The specified placement group configuration for an Amazon EMR cluster.
        #[builder(into, default)]
        pub placement_group_configs: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::emr::ClusterPlacementGroupConfig>>,
        >,
        /// Release label for the Amazon EMR release.
        #[builder(into)]
        pub release_label: pulumi_wasm_rust::Output<String>,
        /// Way that individual Amazon EC2 instances terminate when an automatic scale-in activity occurs or an `instance group` is resized.
        #[builder(into, default)]
        pub scale_down_behavior: pulumi_wasm_rust::Output<Option<String>>,
        /// Security configuration name to attach to the EMR cluster. Only valid for EMR clusters with `release_label` 4.8.0 or greater.
        #[builder(into, default)]
        pub security_configuration: pulumi_wasm_rust::Output<Option<String>>,
        /// IAM role that will be assumed by the Amazon EMR service to access AWS resources.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub service_role: pulumi_wasm_rust::Output<String>,
        /// Number of steps that can be executed concurrently. You can specify a maximum of 256 steps. Only valid for EMR clusters with `release_label` 5.28.0 or greater (default is 1).
        #[builder(into, default)]
        pub step_concurrency_level: pulumi_wasm_rust::Output<Option<i32>>,
        /// List of steps to run when creating the cluster. See below. It is highly recommended to utilize the lifecycle resource options block with `ignoreChanges` if other steps are being managed outside of this provider.
        #[builder(into, default)]
        pub steps: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::emr::ClusterStep>>,
        >,
        /// list of tags to apply to the EMR Cluster. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Switch on/off termination protection (default is `false`, except when using multiple master nodes). Before attempting to destroy the resource when termination protection is enabled, this configuration must be applied with its value set to `false`.
        #[builder(into, default)]
        pub termination_protection: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether whether Amazon EMR should gracefully replace core nodes that have degraded within the cluster. Default value is `false`.
        #[builder(into, default)]
        pub unhealthy_node_replacement: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether the job flow is visible to all IAM users of the AWS account associated with the job flow. Default value is `true`.
        #[builder(into, default)]
        pub visible_to_all_users: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct ClusterResult {
        /// JSON string for selecting additional features such as adding proxy information. Note: Currently there is no API to retrieve the value of this argument after EMR cluster creation from provider, therefore the provider cannot detect drift from the actual EMR cluster if its value is changed outside the provider.
        pub additional_info: pulumi_wasm_rust::Output<Option<String>>,
        /// A case-insensitive list of applications for Amazon EMR to install and configure when launching the cluster. For a list of applications available for each Amazon EMR release version, see the [Amazon EMR Release Guide](https://docs.aws.amazon.com/emr/latest/ReleaseGuide/emr-release-components.html).
        pub applications: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// ARN of the cluster.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// An auto-termination policy for an Amazon EMR cluster. An auto-termination policy defines the amount of idle time in seconds after which a cluster automatically terminates. See Auto Termination Policy Below.
        pub auto_termination_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::emr::ClusterAutoTerminationPolicy>,
        >,
        /// IAM role for automatic scaling policies. The IAM role provides permissions that the automatic scaling feature requires to launch and terminate EC2 instances in an instance group.
        pub autoscaling_role: pulumi_wasm_rust::Output<Option<String>>,
        /// Ordered list of bootstrap actions that will be run before Hadoop is started on the cluster nodes. See below.
        pub bootstrap_actions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::emr::ClusterBootstrapAction>>,
        >,
        pub cluster_state: pulumi_wasm_rust::Output<String>,
        /// List of configurations supplied for the EMR cluster you are creating. Supply a configuration object for applications to override their default configuration. See [AWS Documentation](https://docs.aws.amazon.com/emr/latest/ReleaseGuide/emr-configure-apps.html) for more information.
        pub configurations: pulumi_wasm_rust::Output<Option<String>>,
        /// JSON string for supplying list of configurations for the EMR cluster.
        ///
        /// > **NOTE on `configurations_json`:** If the `Configurations` value is empty then you should skip the `Configurations` field instead of providing an empty list as a value, `"Configurations": []`.
        ///
        /// ```ignore
        /// use pulumi_wasm_rust::Output;
        /// use pulumi_wasm_rust::{add_export, pulumi_main};
        /// #[pulumi_main]
        /// fn test_main() -> Result<(), Error> {
        ///     let cluster = cluster::create(
        ///         "cluster",
        ///         ClusterArgs::builder()
        ///             .configurations_json(
        ///                 "[\n{\n\"Classification\": \"hadoop-env\",\n\"Configurations\": [\n{\n\"Classification\": \"export\",\n\"Properties\": {\n\"JAVA_HOME\": \"/usr/lib/jvm/java-1.8.0\"\n}\n}\n],\n\"Properties\": {}\n}\n]",
        ///             )
        ///             .build_struct(),
        ///     );
        /// }
        /// ```
        pub configurations_json: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block to use an [Instance Fleet](https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-instance-fleet.html) for the core node type. Cannot be specified if any `core_instance_group` configuration blocks are set. Detailed below.
        pub core_instance_fleet: pulumi_wasm_rust::Output<
            super::super::types::emr::ClusterCoreInstanceFleet,
        >,
        /// Configuration block to use an [Instance Group](https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-instance-group-configuration.html#emr-plan-instance-groups) for the [core node type](https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-master-core-task-nodes.html#emr-plan-core).
        pub core_instance_group: pulumi_wasm_rust::Output<
            super::super::types::emr::ClusterCoreInstanceGroup,
        >,
        /// Custom Amazon Linux AMI for the cluster (instead of an EMR-owned AMI). Available in Amazon EMR version 5.7.0 and later.
        pub custom_ami_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Size in GiB of the EBS root device volume of the Linux AMI that is used for each EC2 instance. Available in Amazon EMR version 4.x and later.
        pub ebs_root_volume_size: pulumi_wasm_rust::Output<Option<i32>>,
        /// Attributes for the EC2 instances running the job flow. See below.
        pub ec2_attributes: pulumi_wasm_rust::Output<
            Option<super::super::types::emr::ClusterEc2Attributes>,
        >,
        /// Switch on/off run cluster with no steps or when all steps are complete (default is on)
        pub keep_job_flow_alive_when_no_steps: pulumi_wasm_rust::Output<bool>,
        /// Kerberos configuration for the cluster. See below.
        pub kerberos_attributes: pulumi_wasm_rust::Output<
            Option<super::super::types::emr::ClusterKerberosAttributes>,
        >,
        /// List of [step states](https://docs.aws.amazon.com/emr/latest/APIReference/API_StepStatus.html) used to filter returned steps
        pub list_steps_states: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// AWS KMS customer master key (CMK) key ID or arn used for encrypting log files. This attribute is only available with EMR version 5.30.0 and later, excluding EMR 6.0.0.
        pub log_encryption_kms_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// S3 bucket to write the log files of the job flow. If a value is not provided, logs are not created.
        pub log_uri: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block to use an [Instance Fleet](https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-instance-fleet.html) for the master node type. Cannot be specified if any `master_instance_group` configuration blocks are set. Detailed below.
        pub master_instance_fleet: pulumi_wasm_rust::Output<
            super::super::types::emr::ClusterMasterInstanceFleet,
        >,
        /// Configuration block to use an [Instance Group](https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-instance-group-configuration.html#emr-plan-instance-groups) for the [master node type](https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-master-core-task-nodes.html#emr-plan-master).
        pub master_instance_group: pulumi_wasm_rust::Output<
            super::super::types::emr::ClusterMasterInstanceGroup,
        >,
        /// The DNS name of the master node. If the cluster is on a private subnet, this is the private DNS name. On a public subnet, this is the public DNS name.
        pub master_public_dns: pulumi_wasm_rust::Output<String>,
        /// Name of the job flow.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The specified placement group configuration for an Amazon EMR cluster.
        pub placement_group_configs: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::emr::ClusterPlacementGroupConfig>>,
        >,
        /// Release label for the Amazon EMR release.
        pub release_label: pulumi_wasm_rust::Output<String>,
        /// Way that individual Amazon EC2 instances terminate when an automatic scale-in activity occurs or an `instance group` is resized.
        pub scale_down_behavior: pulumi_wasm_rust::Output<String>,
        /// Security configuration name to attach to the EMR cluster. Only valid for EMR clusters with `release_label` 4.8.0 or greater.
        pub security_configuration: pulumi_wasm_rust::Output<Option<String>>,
        /// IAM role that will be assumed by the Amazon EMR service to access AWS resources.
        ///
        /// The following arguments are optional:
        pub service_role: pulumi_wasm_rust::Output<String>,
        /// Number of steps that can be executed concurrently. You can specify a maximum of 256 steps. Only valid for EMR clusters with `release_label` 5.28.0 or greater (default is 1).
        pub step_concurrency_level: pulumi_wasm_rust::Output<Option<i32>>,
        /// List of steps to run when creating the cluster. See below. It is highly recommended to utilize the lifecycle resource options block with `ignoreChanges` if other steps are being managed outside of this provider.
        pub steps: pulumi_wasm_rust::Output<Vec<super::super::types::emr::ClusterStep>>,
        /// list of tags to apply to the EMR Cluster. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Switch on/off termination protection (default is `false`, except when using multiple master nodes). Before attempting to destroy the resource when termination protection is enabled, this configuration must be applied with its value set to `false`.
        pub termination_protection: pulumi_wasm_rust::Output<bool>,
        /// Whether whether Amazon EMR should gracefully replace core nodes that have degraded within the cluster. Default value is `false`.
        pub unhealthy_node_replacement: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether the job flow is visible to all IAM users of the AWS account associated with the job flow. Default value is `true`.
        pub visible_to_all_users: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ClusterArgs) -> ClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let additional_info_binding = args.additional_info.get_inner();
        let applications_binding = args.applications.get_inner();
        let auto_termination_policy_binding = args.auto_termination_policy.get_inner();
        let autoscaling_role_binding = args.autoscaling_role.get_inner();
        let bootstrap_actions_binding = args.bootstrap_actions.get_inner();
        let configurations_binding = args.configurations.get_inner();
        let configurations_json_binding = args.configurations_json.get_inner();
        let core_instance_fleet_binding = args.core_instance_fleet.get_inner();
        let core_instance_group_binding = args.core_instance_group.get_inner();
        let custom_ami_id_binding = args.custom_ami_id.get_inner();
        let ebs_root_volume_size_binding = args.ebs_root_volume_size.get_inner();
        let ec2_attributes_binding = args.ec2_attributes.get_inner();
        let keep_job_flow_alive_when_no_steps_binding = args
            .keep_job_flow_alive_when_no_steps
            .get_inner();
        let kerberos_attributes_binding = args.kerberos_attributes.get_inner();
        let list_steps_states_binding = args.list_steps_states.get_inner();
        let log_encryption_kms_key_id_binding = args
            .log_encryption_kms_key_id
            .get_inner();
        let log_uri_binding = args.log_uri.get_inner();
        let master_instance_fleet_binding = args.master_instance_fleet.get_inner();
        let master_instance_group_binding = args.master_instance_group.get_inner();
        let name_binding = args.name.get_inner();
        let placement_group_configs_binding = args.placement_group_configs.get_inner();
        let release_label_binding = args.release_label.get_inner();
        let scale_down_behavior_binding = args.scale_down_behavior.get_inner();
        let security_configuration_binding = args.security_configuration.get_inner();
        let service_role_binding = args.service_role.get_inner();
        let step_concurrency_level_binding = args.step_concurrency_level.get_inner();
        let steps_binding = args.steps.get_inner();
        let tags_binding = args.tags.get_inner();
        let termination_protection_binding = args.termination_protection.get_inner();
        let unhealthy_node_replacement_binding = args
            .unhealthy_node_replacement
            .get_inner();
        let visible_to_all_users_binding = args.visible_to_all_users.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:emr/cluster:Cluster".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "additionalInfo".into(),
                    value: &additional_info_binding,
                },
                register_interface::ObjectField {
                    name: "applications".into(),
                    value: &applications_binding,
                },
                register_interface::ObjectField {
                    name: "autoTerminationPolicy".into(),
                    value: &auto_termination_policy_binding,
                },
                register_interface::ObjectField {
                    name: "autoscalingRole".into(),
                    value: &autoscaling_role_binding,
                },
                register_interface::ObjectField {
                    name: "bootstrapActions".into(),
                    value: &bootstrap_actions_binding,
                },
                register_interface::ObjectField {
                    name: "configurations".into(),
                    value: &configurations_binding,
                },
                register_interface::ObjectField {
                    name: "configurationsJson".into(),
                    value: &configurations_json_binding,
                },
                register_interface::ObjectField {
                    name: "coreInstanceFleet".into(),
                    value: &core_instance_fleet_binding,
                },
                register_interface::ObjectField {
                    name: "coreInstanceGroup".into(),
                    value: &core_instance_group_binding,
                },
                register_interface::ObjectField {
                    name: "customAmiId".into(),
                    value: &custom_ami_id_binding,
                },
                register_interface::ObjectField {
                    name: "ebsRootVolumeSize".into(),
                    value: &ebs_root_volume_size_binding,
                },
                register_interface::ObjectField {
                    name: "ec2Attributes".into(),
                    value: &ec2_attributes_binding,
                },
                register_interface::ObjectField {
                    name: "keepJobFlowAliveWhenNoSteps".into(),
                    value: &keep_job_flow_alive_when_no_steps_binding,
                },
                register_interface::ObjectField {
                    name: "kerberosAttributes".into(),
                    value: &kerberos_attributes_binding,
                },
                register_interface::ObjectField {
                    name: "listStepsStates".into(),
                    value: &list_steps_states_binding,
                },
                register_interface::ObjectField {
                    name: "logEncryptionKmsKeyId".into(),
                    value: &log_encryption_kms_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "logUri".into(),
                    value: &log_uri_binding,
                },
                register_interface::ObjectField {
                    name: "masterInstanceFleet".into(),
                    value: &master_instance_fleet_binding,
                },
                register_interface::ObjectField {
                    name: "masterInstanceGroup".into(),
                    value: &master_instance_group_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "placementGroupConfigs".into(),
                    value: &placement_group_configs_binding,
                },
                register_interface::ObjectField {
                    name: "releaseLabel".into(),
                    value: &release_label_binding,
                },
                register_interface::ObjectField {
                    name: "scaleDownBehavior".into(),
                    value: &scale_down_behavior_binding,
                },
                register_interface::ObjectField {
                    name: "securityConfiguration".into(),
                    value: &security_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "serviceRole".into(),
                    value: &service_role_binding,
                },
                register_interface::ObjectField {
                    name: "stepConcurrencyLevel".into(),
                    value: &step_concurrency_level_binding,
                },
                register_interface::ObjectField {
                    name: "steps".into(),
                    value: &steps_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "terminationProtection".into(),
                    value: &termination_protection_binding,
                },
                register_interface::ObjectField {
                    name: "unhealthyNodeReplacement".into(),
                    value: &unhealthy_node_replacement_binding,
                },
                register_interface::ObjectField {
                    name: "visibleToAllUsers".into(),
                    value: &visible_to_all_users_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "additionalInfo".into(),
                },
                register_interface::ResultField {
                    name: "applications".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "autoTerminationPolicy".into(),
                },
                register_interface::ResultField {
                    name: "autoscalingRole".into(),
                },
                register_interface::ResultField {
                    name: "bootstrapActions".into(),
                },
                register_interface::ResultField {
                    name: "clusterState".into(),
                },
                register_interface::ResultField {
                    name: "configurations".into(),
                },
                register_interface::ResultField {
                    name: "configurationsJson".into(),
                },
                register_interface::ResultField {
                    name: "coreInstanceFleet".into(),
                },
                register_interface::ResultField {
                    name: "coreInstanceGroup".into(),
                },
                register_interface::ResultField {
                    name: "customAmiId".into(),
                },
                register_interface::ResultField {
                    name: "ebsRootVolumeSize".into(),
                },
                register_interface::ResultField {
                    name: "ec2Attributes".into(),
                },
                register_interface::ResultField {
                    name: "keepJobFlowAliveWhenNoSteps".into(),
                },
                register_interface::ResultField {
                    name: "kerberosAttributes".into(),
                },
                register_interface::ResultField {
                    name: "listStepsStates".into(),
                },
                register_interface::ResultField {
                    name: "logEncryptionKmsKeyId".into(),
                },
                register_interface::ResultField {
                    name: "logUri".into(),
                },
                register_interface::ResultField {
                    name: "masterInstanceFleet".into(),
                },
                register_interface::ResultField {
                    name: "masterInstanceGroup".into(),
                },
                register_interface::ResultField {
                    name: "masterPublicDns".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "placementGroupConfigs".into(),
                },
                register_interface::ResultField {
                    name: "releaseLabel".into(),
                },
                register_interface::ResultField {
                    name: "scaleDownBehavior".into(),
                },
                register_interface::ResultField {
                    name: "securityConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "serviceRole".into(),
                },
                register_interface::ResultField {
                    name: "stepConcurrencyLevel".into(),
                },
                register_interface::ResultField {
                    name: "steps".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "terminationProtection".into(),
                },
                register_interface::ResultField {
                    name: "unhealthyNodeReplacement".into(),
                },
                register_interface::ResultField {
                    name: "visibleToAllUsers".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ClusterResult {
            additional_info: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("additionalInfo").unwrap(),
            ),
            applications: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applications").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            auto_termination_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoTerminationPolicy").unwrap(),
            ),
            autoscaling_role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoscalingRole").unwrap(),
            ),
            bootstrap_actions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bootstrapActions").unwrap(),
            ),
            cluster_state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterState").unwrap(),
            ),
            configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configurations").unwrap(),
            ),
            configurations_json: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configurationsJson").unwrap(),
            ),
            core_instance_fleet: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("coreInstanceFleet").unwrap(),
            ),
            core_instance_group: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("coreInstanceGroup").unwrap(),
            ),
            custom_ami_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customAmiId").unwrap(),
            ),
            ebs_root_volume_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ebsRootVolumeSize").unwrap(),
            ),
            ec2_attributes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ec2Attributes").unwrap(),
            ),
            keep_job_flow_alive_when_no_steps: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keepJobFlowAliveWhenNoSteps").unwrap(),
            ),
            kerberos_attributes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kerberosAttributes").unwrap(),
            ),
            list_steps_states: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("listStepsStates").unwrap(),
            ),
            log_encryption_kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logEncryptionKmsKeyId").unwrap(),
            ),
            log_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logUri").unwrap(),
            ),
            master_instance_fleet: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("masterInstanceFleet").unwrap(),
            ),
            master_instance_group: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("masterInstanceGroup").unwrap(),
            ),
            master_public_dns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("masterPublicDns").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            placement_group_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("placementGroupConfigs").unwrap(),
            ),
            release_label: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("releaseLabel").unwrap(),
            ),
            scale_down_behavior: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scaleDownBehavior").unwrap(),
            ),
            security_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityConfiguration").unwrap(),
            ),
            service_role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceRole").unwrap(),
            ),
            step_concurrency_level: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stepConcurrencyLevel").unwrap(),
            ),
            steps: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("steps").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            termination_protection: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("terminationProtection").unwrap(),
            ),
            unhealthy_node_replacement: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("unhealthyNodeReplacement").unwrap(),
            ),
            visible_to_all_users: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("visibleToAllUsers").unwrap(),
            ),
        }
    }
}
