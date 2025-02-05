/// Provides a Redshift Cluster IAM Roles resource.
///
/// > **NOTE:** A Redshift cluster's default IAM role can be managed both by this resource's `default_iam_role_arn` argument and the `aws.redshift.Cluster` resource's `default_iam_role_arn` argument. Do not configure different values for both arguments. Doing so will cause a conflict of default IAM roles.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = cluster_iam_roles::create(
///         "example",
///         ClusterIamRolesArgs::builder()
///             .cluster_identifier("${exampleAwsRedshiftCluster.clusterIdentifier}")
///             .iam_role_arns(vec!["${exampleAwsIamRole.arn}",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Redshift Cluster IAM Roless using the `cluster_identifier`. For example:
///
/// ```sh
/// $ pulumi import aws:redshift/clusterIamRoles:ClusterIamRoles examplegroup1 example
/// ```
pub mod cluster_iam_roles {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterIamRolesArgs {
        /// The name of the Redshift Cluster IAM Roles.
        #[builder(into)]
        pub cluster_identifier: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Amazon Resource Name (ARN) for the IAM role that was set as default for the cluster when the cluster was created.
        #[builder(into, default)]
        pub default_iam_role_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A list of IAM Role ARNs to associate with the cluster. A Maximum of 10 can be associated to the cluster at any time.
        #[builder(into, default)]
        pub iam_role_arns: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct ClusterIamRolesResult {
        /// The name of the Redshift Cluster IAM Roles.
        pub cluster_identifier: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) for the IAM role that was set as default for the cluster when the cluster was created.
        pub default_iam_role_arn: pulumi_wasm_rust::Output<String>,
        /// A list of IAM Role ARNs to associate with the cluster. A Maximum of 10 can be associated to the cluster at any time.
        pub iam_role_arns: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ClusterIamRolesArgs,
    ) -> ClusterIamRolesResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cluster_identifier_binding = args
            .cluster_identifier
            .get_output(context)
            .get_inner();
        let default_iam_role_arn_binding = args
            .default_iam_role_arn
            .get_output(context)
            .get_inner();
        let iam_role_arns_binding = args.iam_role_arns.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:redshift/clusterIamRoles:ClusterIamRoles".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterIdentifier".into(),
                    value: &cluster_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "defaultIamRoleArn".into(),
                    value: &default_iam_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "iamRoleArns".into(),
                    value: &iam_role_arns_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ClusterIamRolesResult {
            cluster_identifier: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clusterIdentifier"),
            ),
            default_iam_role_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultIamRoleArn"),
            ),
            iam_role_arns: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("iamRoleArns"),
            ),
        }
    }
}
