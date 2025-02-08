/// Manages Resource Access Manager (RAM) Resource Sharing with AWS Organizations. If you enable sharing with your organization, you can share resources without using invitations. Refer to the [AWS RAM user guide](https://docs.aws.amazon.com/ram/latest/userguide/getting-started-sharing.html#getting-started-sharing-orgs) for more details.
///
/// > **NOTE:** Use this resource to manage resource sharing within your organization, **not** the `aws.organizations.Organization` resource with `ram.amazonaws.com` configured in `aws_service_access_principals`.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = sharing_with_organization::create(
///         "example",
///         SharingWithOrganizationArgs::builder().build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import the resource using the current AWS account ID. For example:
///
/// ```sh
/// $ pulumi import aws:ram/sharingWithOrganization:SharingWithOrganization example 123456789012
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod sharing_with_organization {
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(context: &pulumi_gestalt_rust::PulumiContext, name: &str) {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ram/sharingWithOrganization:SharingWithOrganization".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([]),
        };
        register_interface::register(context.get_inner(), &request);
    }
}
