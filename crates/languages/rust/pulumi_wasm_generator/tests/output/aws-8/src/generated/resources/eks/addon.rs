/// Manages an EKS add-on.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = addon::create(
///         "example",
///         AddonArgs::builder()
///             .addon_name("vpc-cni")
///             .cluster_name("${exampleAwsEksCluster.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Example Update add-on usage with resolve_conflicts_on_update and PRESERVE
///
/// `resolve_conflicts_on_update` with `PRESERVE` can be used to retain the config changes applied to the add-on with kubectl while upgrading to a newer version of the add-on.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = addon::create(
///         "example",
///         AddonArgs::builder()
///             .addon_name("coredns")
///             .addon_version("v1.10.1-eksbuild.1")
///             .cluster_name("${exampleAwsEksCluster.name}")
///             .resolve_conflicts_on_update("PRESERVE")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Example add-on usage with custom configuration_values
///
/// Custom add-on configuration can be passed using `configuration_values` as a single JSON string while creating or updating the add-on.
///
/// > **Note:** `configuration_values` is a single JSON string should match the valid JSON schema for each add-on with specific version.
///
/// To find the correct JSON schema for each add-on can be extracted using [describe-addon-configuration](https://docs.aws.amazon.com/cli/latest/reference/eks/describe-addon-configuration.html) call.
/// This below is an example for extracting the `configuration_values` schema for `coredns`.
///
/// ```bash
///  aws eks describe-addon-configuration \
///  --addon-name coredns \
///  --addon-version v1.10.1-eksbuild.1
/// ```
///
/// Example to create a `coredns` managed addon with custom `configuration_values`.
///
/// ```yaml
/// resources:
///   example:
///     type: aws:eks:Addon
///     properties:
///       clusterName: mycluster
///       addonName: coredns
///       addonVersion: v1.10.1-eksbuild.1
///       resolveConflictsOnCreate: OVERWRITE
///       configurationValues:
///         fn::toJSON:
///           replicaCount: 4
///           resources:
///             limits:
///               cpu: 100m
///               memory: 150Mi
///             requests:
///               cpu: 100m
///               memory: 150Mi
/// ```
///
/// ### Example IAM Role for EKS Addon "vpc-cni" with AWS managed policy
///
/// ```yaml
/// resources:
///   exampleCluster:
///     type: aws:eks:Cluster
///     name: example
///   exampleOpenIdConnectProvider:
///     type: aws:iam:OpenIdConnectProvider
///     name: example
///     properties:
///       clientIdLists:
///         - sts.amazonaws.com
///       thumbprintLists:
///         - ${example.certificates[0].sha1Fingerprint}
///       url: ${exampleCluster.identities[0].oidcs[0].issuer}
///   exampleRole:
///     type: aws:iam:Role
///     name: example
///     properties:
///       assumeRolePolicy: ${exampleAssumeRolePolicy.json}
///       name: example-vpc-cni-role
///   exampleRolePolicyAttachment:
///     type: aws:iam:RolePolicyAttachment
///     name: example
///     properties:
///       policyArn: arn:aws:iam::aws:policy/AmazonEKS_CNI_Policy
///       role: ${exampleRole.name}
/// variables:
///   example:
///     fn::invoke:
///       function: tls:getCertificate
///       arguments:
///         url: ${exampleCluster.identities[0].oidcs[0].issuer}
///   exampleAssumeRolePolicy:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - actions:
///               - sts:AssumeRoleWithWebIdentity
///             effect: Allow
///             conditions:
///               - test: StringEquals
///                 variable:
///                   fn::join:
///                     - ""
///                     - - fn::invoke:
///                           function: std:replace
///                           arguments:
///                             text: ${exampleOpenIdConnectProvider.url}
///                             search: https://
///                             replace: ""
///                           return: result
///                       - :sub
///                 values:
///                   - system:serviceaccount:kube-system:aws-node
///             principals:
///               - identifiers:
///                   - ${exampleOpenIdConnectProvider.arn}
///                 type: Federated
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EKS add-on using the `cluster_name` and `addon_name` separated by a colon (`:`). For example:
///
/// ```sh
/// $ pulumi import aws:eks/addon:Addon my_eks_addon my_cluster_name:my_addon_name
/// ```
pub mod addon {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AddonArgs {
        /// Name of the EKS add-on. The name must match one of
        /// the names returned by [describe-addon-versions](https://docs.aws.amazon.com/cli/latest/reference/eks/describe-addon-versions.html).
        #[builder(into)]
        pub addon_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The version of the EKS add-on. The version must
        /// match one of the versions returned by [describe-addon-versions](https://docs.aws.amazon.com/cli/latest/reference/eks/describe-addon-versions.html).
        #[builder(into, default)]
        pub addon_version: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of the EKS Cluster.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub cluster_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// custom configuration values for addons with single JSON string. This JSON string value must match the JSON schema derived from [describe-addon-configuration](https://docs.aws.amazon.com/cli/latest/reference/eks/describe-addon-configuration.html).
        #[builder(into, default)]
        pub configuration_values: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Configuration block with EKS Pod Identity association settings. See `pod_identity_association` below for details.
        #[builder(into, default)]
        pub pod_identity_associations: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::eks::AddonPodIdentityAssociation>>,
        >,
        /// Indicates if you want to preserve the created resources when deleting the EKS add-on.
        #[builder(into, default)]
        pub preserve: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Define how to resolve parameter value conflicts when migrating an existing add-on to an Amazon EKS add-on or when applying version updates to the add-on. Valid values are `NONE`, `OVERWRITE` and `PRESERVE`. Note that `PRESERVE` is only valid on addon update, not for initial addon creation. If you need to set this to `PRESERVE`, use the `resolve_conflicts_on_create` and `resolve_conflicts_on_update` attributes instead. For more details check [UpdateAddon](https://docs.aws.amazon.com/eks/latest/APIReference/API_UpdateAddon.html) API Docs.
        #[builder(into, default)]
        pub resolve_conflicts: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// How to resolve field value conflicts when migrating a self-managed add-on to an Amazon EKS add-on. Valid values are `NONE` and `OVERWRITE`. For more details see the [CreateAddon](https://docs.aws.amazon.com/eks/latest/APIReference/API_CreateAddon.html) API Docs.
        #[builder(into, default)]
        pub resolve_conflicts_on_create: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// How to resolve field value conflicts for an Amazon EKS add-on if you've changed a value from the Amazon EKS default value. Valid values are `NONE`, `OVERWRITE`, and `PRESERVE`. For more details see the [UpdateAddon](https://docs.aws.amazon.com/eks/latest/APIReference/API_UpdateAddon.html) API Docs.
        #[builder(into, default)]
        pub resolve_conflicts_on_update: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Amazon Resource Name (ARN) of an
        /// existing IAM role to bind to the add-on's service account. The role must be
        /// assigned the IAM permissions required by the add-on. If you don't specify
        /// an existing IAM role, then the add-on uses the permissions assigned to the node
        /// IAM role. For more information, see [Amazon EKS node IAM role](https://docs.aws.amazon.com/eks/latest/userguide/create-node-role.html)
        /// in the Amazon EKS User Guide.
        ///
        /// > **Note:** To specify an existing IAM role, you must have an IAM OpenID Connect (OIDC)
        /// provider created for your cluster. For more information, [see Enabling IAM roles
        /// for service accounts on your cluster](https://docs.aws.amazon.com/eks/latest/userguide/enable-iam-roles-for-service-accounts.html)
        /// in the Amazon EKS User Guide.
        #[builder(into, default)]
        pub service_account_role_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AddonResult {
        /// Name of the EKS add-on. The name must match one of
        /// the names returned by [describe-addon-versions](https://docs.aws.amazon.com/cli/latest/reference/eks/describe-addon-versions.html).
        pub addon_name: pulumi_wasm_rust::Output<String>,
        /// The version of the EKS add-on. The version must
        /// match one of the versions returned by [describe-addon-versions](https://docs.aws.amazon.com/cli/latest/reference/eks/describe-addon-versions.html).
        pub addon_version: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the EKS add-on.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Name of the EKS Cluster.
        ///
        /// The following arguments are optional:
        pub cluster_name: pulumi_wasm_rust::Output<String>,
        /// custom configuration values for addons with single JSON string. This JSON string value must match the JSON schema derived from [describe-addon-configuration](https://docs.aws.amazon.com/cli/latest/reference/eks/describe-addon-configuration.html).
        pub configuration_values: pulumi_wasm_rust::Output<String>,
        /// Date and time in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) that the EKS add-on was created.
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// Date and time in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) that the EKS add-on was updated.
        pub modified_at: pulumi_wasm_rust::Output<String>,
        /// Configuration block with EKS Pod Identity association settings. See `pod_identity_association` below for details.
        pub pod_identity_associations: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::eks::AddonPodIdentityAssociation>>,
        >,
        /// Indicates if you want to preserve the created resources when deleting the EKS add-on.
        pub preserve: pulumi_wasm_rust::Output<Option<bool>>,
        /// Define how to resolve parameter value conflicts when migrating an existing add-on to an Amazon EKS add-on or when applying version updates to the add-on. Valid values are `NONE`, `OVERWRITE` and `PRESERVE`. Note that `PRESERVE` is only valid on addon update, not for initial addon creation. If you need to set this to `PRESERVE`, use the `resolve_conflicts_on_create` and `resolve_conflicts_on_update` attributes instead. For more details check [UpdateAddon](https://docs.aws.amazon.com/eks/latest/APIReference/API_UpdateAddon.html) API Docs.
        pub resolve_conflicts: pulumi_wasm_rust::Output<Option<String>>,
        /// How to resolve field value conflicts when migrating a self-managed add-on to an Amazon EKS add-on. Valid values are `NONE` and `OVERWRITE`. For more details see the [CreateAddon](https://docs.aws.amazon.com/eks/latest/APIReference/API_CreateAddon.html) API Docs.
        pub resolve_conflicts_on_create: pulumi_wasm_rust::Output<Option<String>>,
        /// How to resolve field value conflicts for an Amazon EKS add-on if you've changed a value from the Amazon EKS default value. Valid values are `NONE`, `OVERWRITE`, and `PRESERVE`. For more details see the [UpdateAddon](https://docs.aws.amazon.com/eks/latest/APIReference/API_UpdateAddon.html) API Docs.
        pub resolve_conflicts_on_update: pulumi_wasm_rust::Output<Option<String>>,
        /// The Amazon Resource Name (ARN) of an
        /// existing IAM role to bind to the add-on's service account. The role must be
        /// assigned the IAM permissions required by the add-on. If you don't specify
        /// an existing IAM role, then the add-on uses the permissions assigned to the node
        /// IAM role. For more information, see [Amazon EKS node IAM role](https://docs.aws.amazon.com/eks/latest/userguide/create-node-role.html)
        /// in the Amazon EKS User Guide.
        ///
        /// > **Note:** To specify an existing IAM role, you must have an IAM OpenID Connect (OIDC)
        /// provider created for your cluster. For more information, [see Enabling IAM roles
        /// for service accounts on your cluster](https://docs.aws.amazon.com/eks/latest/userguide/enable-iam-roles-for-service-accounts.html)
        /// in the Amazon EKS User Guide.
        pub service_account_role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// (Optional) Key-value map of resource tags, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AddonArgs,
    ) -> AddonResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let addon_name_binding = args.addon_name.get_output(context).get_inner();
        let addon_version_binding = args.addon_version.get_output(context).get_inner();
        let cluster_name_binding = args.cluster_name.get_output(context).get_inner();
        let configuration_values_binding = args
            .configuration_values
            .get_output(context)
            .get_inner();
        let pod_identity_associations_binding = args
            .pod_identity_associations
            .get_output(context)
            .get_inner();
        let preserve_binding = args.preserve.get_output(context).get_inner();
        let resolve_conflicts_binding = args
            .resolve_conflicts
            .get_output(context)
            .get_inner();
        let resolve_conflicts_on_create_binding = args
            .resolve_conflicts_on_create
            .get_output(context)
            .get_inner();
        let resolve_conflicts_on_update_binding = args
            .resolve_conflicts_on_update
            .get_output(context)
            .get_inner();
        let service_account_role_arn_binding = args
            .service_account_role_arn
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:eks/addon:Addon".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "addonName".into(),
                    value: &addon_name_binding,
                },
                register_interface::ObjectField {
                    name: "addonVersion".into(),
                    value: &addon_version_binding,
                },
                register_interface::ObjectField {
                    name: "clusterName".into(),
                    value: &cluster_name_binding,
                },
                register_interface::ObjectField {
                    name: "configurationValues".into(),
                    value: &configuration_values_binding,
                },
                register_interface::ObjectField {
                    name: "podIdentityAssociations".into(),
                    value: &pod_identity_associations_binding,
                },
                register_interface::ObjectField {
                    name: "preserve".into(),
                    value: &preserve_binding,
                },
                register_interface::ObjectField {
                    name: "resolveConflicts".into(),
                    value: &resolve_conflicts_binding,
                },
                register_interface::ObjectField {
                    name: "resolveConflictsOnCreate".into(),
                    value: &resolve_conflicts_on_create_binding,
                },
                register_interface::ObjectField {
                    name: "resolveConflictsOnUpdate".into(),
                    value: &resolve_conflicts_on_update_binding,
                },
                register_interface::ObjectField {
                    name: "serviceAccountRoleArn".into(),
                    value: &service_account_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AddonResult {
            addon_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("addonName"),
            ),
            addon_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("addonVersion"),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            cluster_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clusterName"),
            ),
            configuration_values: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("configurationValues"),
            ),
            created_at: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createdAt"),
            ),
            modified_at: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("modifiedAt"),
            ),
            pod_identity_associations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("podIdentityAssociations"),
            ),
            preserve: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("preserve"),
            ),
            resolve_conflicts: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resolveConflicts"),
            ),
            resolve_conflicts_on_create: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resolveConflictsOnCreate"),
            ),
            resolve_conflicts_on_update: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resolveConflictsOnUpdate"),
            ),
            service_account_role_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceAccountRoleArn"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
