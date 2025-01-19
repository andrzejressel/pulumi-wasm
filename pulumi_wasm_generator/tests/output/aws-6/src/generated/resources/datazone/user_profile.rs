/// Resource for managing an AWS DataZone User Profile.
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
///     let example = user_profile::create(
///         "example",
///         UserProfileArgs::builder()
///             .domain_identifier("${exampleAwsDatazoneDomain.id}")
///             .user_identifier("${exampleAwsIamUser.arn}")
///             .user_type("IAM_USER")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DataZone User Profile using the `user_identifier,domain_identifier,type`. For example:
///
/// ```sh
/// $ pulumi import aws:datazone/userProfile:UserProfile example arn:aws:iam::123456789012:user/example,dzd_54nakfrg9k6suo,IAM
/// ```
pub mod user_profile {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserProfileArgs {
        /// The domain identifier.
        #[builder(into)]
        pub domain_identifier: pulumi_wasm_rust::Output<String>,
        /// The user profile status.
        #[builder(into, default)]
        pub status: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::datazone::UserProfileTimeouts>,
        >,
        /// The user identifier.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub user_identifier: pulumi_wasm_rust::Output<String>,
        /// The user type.
        #[builder(into, default)]
        pub user_type: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct UserProfileResult {
        /// Details about the user profile.
        pub details: pulumi_wasm_rust::Output<
            Vec<super::super::types::datazone::UserProfileDetail>,
        >,
        /// The domain identifier.
        pub domain_identifier: pulumi_wasm_rust::Output<String>,
        /// The user profile status.
        pub status: pulumi_wasm_rust::Output<String>,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::datazone::UserProfileTimeouts>,
        >,
        /// The user profile type.
        pub type_: pulumi_wasm_rust::Output<String>,
        /// The user identifier.
        ///
        /// The following arguments are optional:
        pub user_identifier: pulumi_wasm_rust::Output<String>,
        /// The user type.
        pub user_type: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: UserProfileArgs) -> UserProfileResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let domain_identifier_binding = args.domain_identifier.get_inner();
        let status_binding = args.status.get_inner();
        let timeouts_binding = args.timeouts.get_inner();
        let user_identifier_binding = args.user_identifier.get_inner();
        let user_type_binding = args.user_type.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:datazone/userProfile:UserProfile".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "domainIdentifier".into(),
                    value: &domain_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "status".into(),
                    value: &status_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
                register_interface::ObjectField {
                    name: "userIdentifier".into(),
                    value: &user_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "userType".into(),
                    value: &user_type_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "details".into(),
                },
                register_interface::ResultField {
                    name: "domainIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "timeouts".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "userIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "userType".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        UserProfileResult {
            details: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("details").unwrap(),
            ),
            domain_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainIdentifier").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            user_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userIdentifier").unwrap(),
            ),
            user_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userType").unwrap(),
            ),
        }
    }
}
