/// Resource for managing an AWS QuickSight Folder Membership.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod folder_membership {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FolderMembershipArgs {
        /// AWS account ID.
        #[builder(into, default)]
        pub aws_account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Identifier for the folder.
        #[builder(into)]
        pub folder_id: pulumi_wasm_rust::Output<String>,
        /// ID of the asset (the dashboard, analysis, or dataset).
        #[builder(into)]
        pub member_id: pulumi_wasm_rust::Output<String>,
        /// Type of the member. Valid values are `ANALYSIS`, `DASHBOARD`, and `DATASET`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub member_type: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct FolderMembershipResult {
        /// AWS account ID.
        pub aws_account_id: pulumi_wasm_rust::Output<String>,
        /// Identifier for the folder.
        pub folder_id: pulumi_wasm_rust::Output<String>,
        /// ID of the asset (the dashboard, analysis, or dataset).
        pub member_id: pulumi_wasm_rust::Output<String>,
        /// Type of the member. Valid values are `ANALYSIS`, `DASHBOARD`, and `DATASET`.
        ///
        /// The following arguments are optional:
        pub member_type: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: FolderMembershipArgs) -> FolderMembershipResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let aws_account_id_binding = args.aws_account_id.get_inner();
        let folder_id_binding = args.folder_id.get_inner();
        let member_id_binding = args.member_id.get_inner();
        let member_type_binding = args.member_type.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:quicksight/folderMembership:FolderMembership".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "awsAccountId".into(),
                    value: &aws_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "folderId".into(),
                    value: &folder_id_binding,
                },
                register_interface::ObjectField {
                    name: "memberId".into(),
                    value: &member_id_binding,
                },
                register_interface::ObjectField {
                    name: "memberType".into(),
                    value: &member_type_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "awsAccountId".into(),
                },
                register_interface::ResultField {
                    name: "folderId".into(),
                },
                register_interface::ResultField {
                    name: "memberId".into(),
                },
                register_interface::ResultField {
                    name: "memberType".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FolderMembershipResult {
            aws_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("awsAccountId").unwrap(),
            ),
            folder_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("folderId").unwrap(),
            ),
            member_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("memberId").unwrap(),
            ),
            member_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("memberType").unwrap(),
            ),
        }
    }
}
