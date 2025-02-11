/// Resource for managing an AWS QuickSight Folder Membership.
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
///     let example = folder_membership::create(
///         "example",
///         FolderMembershipArgs::builder()
///             .folder_id("${exampleAwsQuicksightFolder.folderId}")
///             .member_id("${exampleAwsQuicksightDataSet.dataSetId}")
///             .member_type("DATASET")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import QuickSight Folder Membership using the AWS account ID, folder ID, member type, and member ID separated by commas (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:quicksight/folderMembership:FolderMembership example 123456789012,example-folder,DATASET,example-dataset
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod folder_membership {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FolderMembershipArgs {
        /// AWS account ID.
        #[builder(into, default)]
        pub aws_account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Identifier for the folder.
        #[builder(into)]
        pub folder_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ID of the asset (the dashboard, analysis, or dataset).
        #[builder(into)]
        pub member_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Type of the member. Valid values are `ANALYSIS`, `DASHBOARD`, and `DATASET`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub member_type: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct FolderMembershipResult {
        /// AWS account ID.
        pub aws_account_id: pulumi_gestalt_rust::Output<String>,
        /// Identifier for the folder.
        pub folder_id: pulumi_gestalt_rust::Output<String>,
        /// ID of the asset (the dashboard, analysis, or dataset).
        pub member_id: pulumi_gestalt_rust::Output<String>,
        /// Type of the member. Valid values are `ANALYSIS`, `DASHBOARD`, and `DATASET`.
        ///
        /// The following arguments are optional:
        pub member_type: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FolderMembershipArgs,
    ) -> FolderMembershipResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let aws_account_id_binding = args.aws_account_id.get_output(context);
        let folder_id_binding = args.folder_id.get_output(context);
        let member_id_binding = args.member_id.get_output(context);
        let member_type_binding = args.member_type.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:quicksight/folderMembership:FolderMembership".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "awsAccountId".into(),
                    value: &aws_account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "folderId".into(),
                    value: &folder_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "memberId".into(),
                    value: &member_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "memberType".into(),
                    value: &member_type_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        FolderMembershipResult {
            aws_account_id: o.get_field("awsAccountId"),
            folder_id: o.get_field("folderId"),
            member_id: o.get_field("memberId"),
            member_type: o.get_field("memberType"),
        }
    }
}
