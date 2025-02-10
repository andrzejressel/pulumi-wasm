/// Resource for associating accounts to existing Inspector instances.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = member_association::create(
///         "example",
///         MemberAssociationArgs::builder().account_id("123456789012").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Amazon Inspector Member Association using the `account_id`. For example:
///
/// ```sh
/// $ pulumi import aws:inspector2/memberAssociation:MemberAssociation example 123456789012
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod member_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MemberAssociationArgs {
        /// ID of the account to associate
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct MemberAssociationResult {
        /// ID of the account to associate
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// Account ID of the delegated administrator account
        pub delegated_admin_account_id: pulumi_gestalt_rust::Output<String>,
        /// Status of the member relationship
        pub relationship_status: pulumi_gestalt_rust::Output<String>,
        /// Date and time of the last update of the relationship
        pub updated_at: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MemberAssociationArgs,
    ) -> MemberAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:inspector2/memberAssociation:MemberAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        MemberAssociationResult {
            account_id: o.get_field("accountId"),
            delegated_admin_account_id: o.get_field("delegatedAdminAccountId"),
            relationship_status: o.get_field("relationshipStatus"),
            updated_at: o.get_field("updatedAt"),
        }
    }
}
