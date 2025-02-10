/// > **Note:** There is only a single account alias per AWS account.
///
/// Manages the account alias for the AWS Account.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let alias = account_alias::create(
///         "alias",
///         AccountAliasArgs::builder().account_alias("my-account-alias").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import the current Account Alias using the `account_alias`. For example:
///
/// ```sh
/// $ pulumi import aws:iam/accountAlias:AccountAlias alias my-account-alias
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod account_alias {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountAliasArgs {
        /// The account alias
        #[builder(into)]
        pub account_alias: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AccountAliasResult {
        /// The account alias
        pub account_alias: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccountAliasArgs,
    ) -> AccountAliasResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_alias_binding = args.account_alias.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:iam/accountAlias:AccountAlias".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountAlias".into(),
                    value: account_alias_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AccountAliasResult {
            account_alias: o.get_field("accountAlias"),
        }
    }
}
