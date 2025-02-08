/// Provides a Resource Access Manager (RAM) principal association. Depending if [RAM Sharing with AWS Organizations is enabled](https://docs.aws.amazon.com/ram/latest/userguide/getting-started-sharing.html#getting-started-sharing-orgs), the RAM behavior with different principal types changes.
///
/// When RAM Sharing with AWS Organizations is enabled:
///
/// - For AWS Account ID, Organization, and Organizational Unit principals within the same AWS Organization, no resource share invitation is sent and resources become available automatically after creating the association.
/// - For AWS Account ID principals outside the AWS Organization, a resource share invitation is sent and must be accepted before resources become available. See the `aws.ram.ResourceShareAccepter` resource to accept these invitations.
///
/// When RAM Sharing with AWS Organizations is not enabled:
///
/// - Organization and Organizational Unit principals cannot be used.
/// - For AWS Account ID principals, a resource share invitation is sent and must be accepted before resources become available. See the `aws.ram.ResourceShareAccepter` resource to accept these invitations.
///
/// ## Example Usage
///
/// ### AWS Account ID
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_share::create(
///         "example",
///         ResourceShareArgs::builder().allow_external_principals(true).build_struct(),
///     );
///     let examplePrincipalAssociation = principal_association::create(
///         "examplePrincipalAssociation",
///         PrincipalAssociationArgs::builder()
///             .principal("111111111111")
///             .resource_share_arn("${example.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### AWS Organization
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = principal_association::create(
///         "example",
///         PrincipalAssociationArgs::builder()
///             .principal("${exampleAwsOrganizationsOrganization.arn}")
///             .resource_share_arn("${exampleAwsRamResourceShare.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import RAM Principal Associations using their Resource Share ARN and the `principal` separated by a comma. For example:
///
/// ```sh
/// $ pulumi import aws:ram/principalAssociation:PrincipalAssociation example arn:aws:ram:eu-west-1:123456789012:resource-share/73da1ab9-b94a-4ba3-8eb4-45917f7f4b12,123456789012
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod principal_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PrincipalAssociationArgs {
        /// The principal to associate with the resource share. Possible values are an AWS account ID, an AWS Organizations Organization ARN, or an AWS Organizations Organization Unit ARN.
        #[builder(into)]
        pub principal: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Amazon Resource Name (ARN) of the resource share.
        #[builder(into)]
        pub resource_share_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PrincipalAssociationResult {
        /// The principal to associate with the resource share. Possible values are an AWS account ID, an AWS Organizations Organization ARN, or an AWS Organizations Organization Unit ARN.
        pub principal: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the resource share.
        pub resource_share_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: PrincipalAssociationArgs,
    ) -> PrincipalAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let principal_binding = args.principal.get_output(context).get_inner();
        let resource_share_arn_binding = args
            .resource_share_arn
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ram/principalAssociation:PrincipalAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "principal".into(),
                    value: &principal_binding,
                },
                register_interface::ObjectField {
                    name: "resourceShareArn".into(),
                    value: &resource_share_arn_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        PrincipalAssociationResult {
            principal: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("principal"),
            ),
            resource_share_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceShareArn"),
            ),
        }
    }
}
