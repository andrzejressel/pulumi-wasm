/// An Anthos cluster running on AWS.
///
/// For more information, see:
/// * [Multicloud overview](https://cloud.google.com/kubernetes-engine/multi-cloud/docs)
/// ## Example Usage
///
/// ### Basic_aws_cluster
/// A basic example of a containeraws cluster
/// ```yaml
/// resources:
///   primary:
///     type: gcp:container:AwsCluster
///     properties:
///       authorization:
///         adminUsers:
///           - username: my@service-account.com
///         adminGroups:
///           - group: group@domain.com
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
/// variables:
///   versions:
///     fn::invoke:
///       function: gcp:container:getAwsVersions
///       arguments:
///         project: my-project-name
///         location: us-west1
/// ```
/// ### Basic_enum_aws_cluster
/// A basic example of a containeraws cluster with lowercase enums
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
///           volumeType: gp3
///         proxyConfig:
///           secretArn: arn:aws:secretsmanager:us-west-2:126285863215:secret:proxy_config20210824150329476300000001-ABCDEF
///           secretVersion: 12345678-ABCD-EFGH-IJKL-987654321098
///         rootVolume:
///           iops: 3000
///           kmsKeyArn: arn:aws:kms:my-aws-region:012345678910:key/12345678-1234-1234-1234-123456789111
///           sizeGib: 10
///           volumeType: gp3
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
/// variables:
///   versions:
///     fn::invoke:
///       function: gcp:container:getAwsVersions
///       arguments:
///         project: my-project-name
///         location: us-west1
/// ```
/// ### Beta_basic_enum_aws_cluster
/// A basic example of a containeraws cluster with lowercase enums (beta)
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
///           volumeType: gp3
///         proxyConfig:
///           secretArn: arn:aws:secretsmanager:us-west-2:126285863215:secret:proxy_config20210824150329476300000001-ABCDEF
///           secretVersion: 12345678-ABCD-EFGH-IJKL-987654321098
///         rootVolume:
///           iops: 3000
///           kmsKeyArn: arn:aws:kms:my-aws-region:012345678910:key/12345678-1234-1234-1234-123456789111
///           sizeGib: 10
///           volumeType: gp3
///         securityGroupIds:
///           - sg-00000000000000000
///         sshConfig:
///           ec2KeyPair: my--1p-dev-ssh
///         tags:
///           owner: my@service-account.com
///         instancePlacement:
///           tenancy: dedicated
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
///       loggingConfig:
///         componentConfig:
///           enableComponents:
///             - system_components
///             - workloads
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
/// Cluster can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/awsClusters/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, Cluster can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:container/awsCluster:AwsCluster default projects/{{project}}/locations/{{location}}/awsClusters/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:container/awsCluster:AwsCluster default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:container/awsCluster:AwsCluster default {{location}}/{{name}}
/// ```
///
pub mod aws_cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AwsClusterArgs {
        /// Optional. Annotations on the cluster. This field has the same restrictions as Kubernetes annotations. The total size of
        /// all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required),
        /// separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with
        /// alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. **Note**: This field is
        /// non-authoritative, and will only manage the annotations present in your configuration. Please refer to the field
        /// `effective_annotations` for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration related to the cluster RBAC settings.
        #[builder(into)]
        pub authorization: pulumi_wasm_rust::InputOrOutput<
            super::super::types::container::AwsClusterAuthorization,
        >,
        /// The AWS region where the cluster runs. Each Google Cloud region supports a subset of nearby AWS regions. You can call to list all supported AWS regions within a given Google Cloud region.
        #[builder(into)]
        pub aws_region: pulumi_wasm_rust::InputOrOutput<String>,
        /// Configuration options for the Binary Authorization feature.
        #[builder(into, default)]
        pub binary_authorization: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::container::AwsClusterBinaryAuthorization>,
        >,
        /// Configuration related to the cluster control plane.
        #[builder(into)]
        pub control_plane: pulumi_wasm_rust::InputOrOutput<
            super::super::types::container::AwsClusterControlPlane,
        >,
        /// Optional. A human readable description of this cluster. Cannot be longer than 255 UTF-8 encoded bytes.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Fleet configuration.
        #[builder(into)]
        pub fleet: pulumi_wasm_rust::InputOrOutput<
            super::super::types::container::AwsClusterFleet,
        >,
        /// The location for the resource
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// Logging configuration.
        #[builder(into, default)]
        pub logging_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::container::AwsClusterLoggingConfig>,
        >,
        /// The name of this resource.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Cluster-wide networking configuration.
        #[builder(into)]
        pub networking: pulumi_wasm_rust::InputOrOutput<
            super::super::types::container::AwsClusterNetworking,
        >,
        /// The project for the resource
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AwsClusterResult {
        /// Optional. Annotations on the cluster. This field has the same restrictions as Kubernetes annotations. The total size of
        /// all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required),
        /// separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with
        /// alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. **Note**: This field is
        /// non-authoritative, and will only manage the annotations present in your configuration. Please refer to the field
        /// `effective_annotations` for all of the annotations present on the resource.
        pub annotations: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration related to the cluster RBAC settings.
        pub authorization: pulumi_wasm_rust::Output<
            super::super::types::container::AwsClusterAuthorization,
        >,
        /// The AWS region where the cluster runs. Each Google Cloud region supports a subset of nearby AWS regions. You can call to list all supported AWS regions within a given Google Cloud region.
        pub aws_region: pulumi_wasm_rust::Output<String>,
        /// Configuration options for the Binary Authorization feature.
        pub binary_authorization: pulumi_wasm_rust::Output<
            super::super::types::container::AwsClusterBinaryAuthorization,
        >,
        /// Configuration related to the cluster control plane.
        pub control_plane: pulumi_wasm_rust::Output<
            super::super::types::container::AwsClusterControlPlane,
        >,
        /// Output only. The time at which this cluster was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Optional. A human readable description of this cluster. Cannot be longer than 255 UTF-8 encoded bytes.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub effective_annotations: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Output only. The endpoint of the cluster's API server.
        pub endpoint: pulumi_wasm_rust::Output<String>,
        /// Allows clients to perform consistent read-modify-writes through optimistic concurrency control. May be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// Fleet configuration.
        pub fleet: pulumi_wasm_rust::Output<
            super::super::types::container::AwsClusterFleet,
        >,
        /// The location for the resource
        pub location: pulumi_wasm_rust::Output<String>,
        /// Logging configuration.
        pub logging_config: pulumi_wasm_rust::Output<
            super::super::types::container::AwsClusterLoggingConfig,
        >,
        /// The name of this resource.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Cluster-wide networking configuration.
        pub networking: pulumi_wasm_rust::Output<
            super::super::types::container::AwsClusterNetworking,
        >,
        /// The project for the resource
        pub project: pulumi_wasm_rust::Output<String>,
        /// Output only. If set, there are currently changes in flight to the cluster.
        pub reconciling: pulumi_wasm_rust::Output<bool>,
        /// Output only. The current state of the cluster. Possible values: STATE_UNSPECIFIED, PROVISIONING, RUNNING, RECONCILING, STOPPING, ERROR, DEGRADED
        pub state: pulumi_wasm_rust::Output<String>,
        /// Output only. A globally unique identifier for the cluster.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// Output only. The time at which this cluster was last updated.
        pub update_time: pulumi_wasm_rust::Output<String>,
        /// Output only. Workload Identity settings.
        pub workload_identity_configs: pulumi_wasm_rust::Output<
            Vec<super::super::types::container::AwsClusterWorkloadIdentityConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AwsClusterArgs,
    ) -> AwsClusterResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let annotations_binding = args.annotations.get_output(context).get_inner();
        let authorization_binding = args.authorization.get_output(context).get_inner();
        let aws_region_binding = args.aws_region.get_output(context).get_inner();
        let binary_authorization_binding = args
            .binary_authorization
            .get_output(context)
            .get_inner();
        let control_plane_binding = args.control_plane.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let fleet_binding = args.fleet.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let logging_config_binding = args.logging_config.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let networking_binding = args.networking.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:container/awsCluster:AwsCluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding,
                },
                register_interface::ObjectField {
                    name: "authorization".into(),
                    value: &authorization_binding,
                },
                register_interface::ObjectField {
                    name: "awsRegion".into(),
                    value: &aws_region_binding,
                },
                register_interface::ObjectField {
                    name: "binaryAuthorization".into(),
                    value: &binary_authorization_binding,
                },
                register_interface::ObjectField {
                    name: "controlPlane".into(),
                    value: &control_plane_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "fleet".into(),
                    value: &fleet_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "loggingConfig".into(),
                    value: &logging_config_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networking".into(),
                    value: &networking_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AwsClusterResult {
            annotations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("annotations"),
            ),
            authorization: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("authorization"),
            ),
            aws_region: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("awsRegion"),
            ),
            binary_authorization: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("binaryAuthorization"),
            ),
            control_plane: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("controlPlane"),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            effective_annotations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveAnnotations"),
            ),
            endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("endpoint"),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(o.extract_field("etag")),
            fleet: pulumi_wasm_rust::__private::into_domain(o.extract_field("fleet")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            logging_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("loggingConfig"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            networking: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("networking"),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            reconciling: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("reconciling"),
            ),
            state: pulumi_wasm_rust::__private::into_domain(o.extract_field("state")),
            uid: pulumi_wasm_rust::__private::into_domain(o.extract_field("uid")),
            update_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
            workload_identity_configs: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("workloadIdentityConfigs"),
            ),
        }
    }
}
