/// Resource for managing QuickSight User
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = user::create(
///         "example",
///         UserArgs::builder()
///             .email("author@example.com")
///             .iam_arn("arn:aws:iam::123456789012:user/Example")
///             .identity_type("IAM")
///             .namespace("foo")
///             .session_name("an-author")
///             .user_role("AUTHOR")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// You cannot import this resource.
///
pub mod user {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserArgs {
        /// The ID for the AWS account that the user is in. Currently, you use the ID for the AWS account that contains your Amazon QuickSight account.
        #[builder(into, default)]
        pub aws_account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The email address of the user that you want to register.
        #[builder(into)]
        pub email: pulumi_wasm_rust::Output<String>,
        /// The ARN of the IAM user or role that you are registering with Amazon QuickSight.
        #[builder(into, default)]
        pub iam_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Amazon QuickSight supports several ways of managing the identity of users. This parameter accepts either  `IAM` or `QUICKSIGHT`. If `IAM` is specified, the `iam_arn` must also be specified.
        #[builder(into)]
        pub identity_type: pulumi_wasm_rust::Output<String>,
        /// The Amazon Quicksight namespace to create the user in. Defaults to `default`.
        #[builder(into, default)]
        pub namespace: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the IAM session to use when assuming roles that can embed QuickSight dashboards. Only valid for registering users using an assumed IAM role. Additionally, if registering multiple users using the same IAM role, each user needs to have a unique session name.
        #[builder(into, default)]
        pub session_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The Amazon QuickSight user name that you want to create for the user you are registering. Only valid for registering a user with `identity_type` set to `QUICKSIGHT`.
        #[builder(into, default)]
        pub user_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The Amazon QuickSight role of the user. The user role can be one of the following: `READER`, `AUTHOR`, `ADMIN`, `READER_PRO`, `AUTHOR_PRO` or `ADMIN_PRO`.
        #[builder(into)]
        pub user_role: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct UserResult {
        /// Amazon Resource Name (ARN) of the user
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The ID for the AWS account that the user is in. Currently, you use the ID for the AWS account that contains your Amazon QuickSight account.
        pub aws_account_id: pulumi_wasm_rust::Output<String>,
        /// The email address of the user that you want to register.
        pub email: pulumi_wasm_rust::Output<String>,
        /// The ARN of the IAM user or role that you are registering with Amazon QuickSight.
        pub iam_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Amazon QuickSight supports several ways of managing the identity of users. This parameter accepts either  `IAM` or `QUICKSIGHT`. If `IAM` is specified, the `iam_arn` must also be specified.
        pub identity_type: pulumi_wasm_rust::Output<String>,
        /// The Amazon Quicksight namespace to create the user in. Defaults to `default`.
        pub namespace: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the IAM session to use when assuming roles that can embed QuickSight dashboards. Only valid for registering users using an assumed IAM role. Additionally, if registering multiple users using the same IAM role, each user needs to have a unique session name.
        pub session_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The Amazon QuickSight user name that you want to create for the user you are registering. Only valid for registering a user with `identity_type` set to `QUICKSIGHT`.
        pub user_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The Amazon QuickSight role of the user. The user role can be one of the following: `READER`, `AUTHOR`, `ADMIN`, `READER_PRO`, `AUTHOR_PRO` or `ADMIN_PRO`.
        pub user_role: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: UserArgs) -> UserResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let aws_account_id_binding = args.aws_account_id.get_inner();
        let email_binding = args.email.get_inner();
        let iam_arn_binding = args.iam_arn.get_inner();
        let identity_type_binding = args.identity_type.get_inner();
        let namespace_binding = args.namespace.get_inner();
        let session_name_binding = args.session_name.get_inner();
        let user_name_binding = args.user_name.get_inner();
        let user_role_binding = args.user_role.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:quicksight/user:User".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "awsAccountId".into(),
                    value: &aws_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "email".into(),
                    value: &email_binding,
                },
                register_interface::ObjectField {
                    name: "iamArn".into(),
                    value: &iam_arn_binding,
                },
                register_interface::ObjectField {
                    name: "identityType".into(),
                    value: &identity_type_binding,
                },
                register_interface::ObjectField {
                    name: "namespace".into(),
                    value: &namespace_binding,
                },
                register_interface::ObjectField {
                    name: "sessionName".into(),
                    value: &session_name_binding,
                },
                register_interface::ObjectField {
                    name: "userName".into(),
                    value: &user_name_binding,
                },
                register_interface::ObjectField {
                    name: "userRole".into(),
                    value: &user_role_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "awsAccountId".into(),
                },
                register_interface::ResultField {
                    name: "email".into(),
                },
                register_interface::ResultField {
                    name: "iamArn".into(),
                },
                register_interface::ResultField {
                    name: "identityType".into(),
                },
                register_interface::ResultField {
                    name: "namespace".into(),
                },
                register_interface::ResultField {
                    name: "sessionName".into(),
                },
                register_interface::ResultField {
                    name: "userName".into(),
                },
                register_interface::ResultField {
                    name: "userRole".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        UserResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            aws_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("awsAccountId").unwrap(),
            ),
            email: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("email").unwrap(),
            ),
            iam_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iamArn").unwrap(),
            ),
            identity_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identityType").unwrap(),
            ),
            namespace: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namespace").unwrap(),
            ),
            session_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sessionName").unwrap(),
            ),
            user_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userName").unwrap(),
            ),
            user_role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userRole").unwrap(),
            ),
        }
    }
}
