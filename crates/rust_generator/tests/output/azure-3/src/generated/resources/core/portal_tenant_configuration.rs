/// Manages Portal Tenant Configuration.
///
/// > **Note:** User has to be `Contributor` or `Owner` at scope `/` for managing this resource.
///
/// > **Note:** The Service Principal with Tenant Admin can be created by `az ad sp create-for-rbac --name "<sp name>" --role="Contributor" --scopes="/"`.
///
/// > **Note:** The Service Principal can be granted Tenant Admin permission by `az role assignment create --assignee "<app id>" --role "Contributor" --scope "/"`.
///
/// > **Note:** While assigning the role to the existing/new Service Principal at the Tenant Scope, the user assigning role must already have the `Owner` role assigned at the Tenant Scope.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = portal_tenant_configuration::create(
///         "example",
///         PortalTenantConfigurationArgs::builder()
///             .private_markdown_storage_enforced(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Portal Tenant Configurations can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:core/portalTenantConfiguration:PortalTenantConfiguration example /providers/Microsoft.Portal/tenantConfigurations/default
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod portal_tenant_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PortalTenantConfigurationArgs {
        /// Is the private tile markdown storage which used to display custom dynamic and static content enabled?
        ///
        /// > **Note:** When `private_markdown_storage_enforced` is set to `true`, only external storage configuration (URI) is allowed for Markdown tiles. Inline content configuration will be prohibited.
        #[builder(into)]
        pub private_markdown_storage_enforced: pulumi_gestalt_rust::InputOrOutput<bool>,
    }
    #[allow(dead_code)]
    pub struct PortalTenantConfigurationResult {
        /// Is the private tile markdown storage which used to display custom dynamic and static content enabled?
        ///
        /// > **Note:** When `private_markdown_storage_enforced` is set to `true`, only external storage configuration (URI) is allowed for Markdown tiles. Inline content configuration will be prohibited.
        pub private_markdown_storage_enforced: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PortalTenantConfigurationArgs,
    ) -> PortalTenantConfigurationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let private_markdown_storage_enforced_binding = args
            .private_markdown_storage_enforced
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:core/portalTenantConfiguration:PortalTenantConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "privateMarkdownStorageEnforced".into(),
                    value: private_markdown_storage_enforced_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        PortalTenantConfigurationResult {
            private_markdown_storage_enforced: o
                .get_field("privateMarkdownStorageEnforced"),
        }
    }
}
