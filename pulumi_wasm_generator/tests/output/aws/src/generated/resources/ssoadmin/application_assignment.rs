/// Resource for managing an AWS SSO Admin Application Assignment.
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
///     let example = application_assignment::create(
///         "example",
///         ApplicationAssignmentArgs::builder()
///             .application_arn("${exampleAwsSsoadminApplication.applicationArn}")
///             .principal_id("${exampleAwsIdentitystoreUser.userId}")
///             .principal_type("USER")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Group Type
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = application_assignment::create(
///         "example",
///         ApplicationAssignmentArgs::builder()
///             .application_arn("${exampleAwsSsoadminApplication.applicationArn}")
///             .principal_id("${exampleAwsIdentitystoreGroup.groupId}")
///             .principal_type("GROUP")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SSO Admin Application Assignment using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ssoadmin/applicationAssignment:ApplicationAssignment example arn:aws:sso::123456789012:application/id-12345678,abcd1234,USER
/// ```
pub mod application_assignment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationAssignmentArgs {
        /// ARN of the application.
        #[builder(into)]
        pub application_arn: pulumi_wasm_rust::Output<String>,
        /// An identifier for an object in IAM Identity Center, such as a user or group.
        #[builder(into)]
        pub principal_id: pulumi_wasm_rust::Output<String>,
        /// Entity type for which the assignment will be created. Valid values are `USER` or `GROUP`.
        #[builder(into)]
        pub principal_type: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ApplicationAssignmentResult {
        /// ARN of the application.
        pub application_arn: pulumi_wasm_rust::Output<String>,
        /// An identifier for an object in IAM Identity Center, such as a user or group.
        pub principal_id: pulumi_wasm_rust::Output<String>,
        /// Entity type for which the assignment will be created. Valid values are `USER` or `GROUP`.
        pub principal_type: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ApplicationAssignmentArgs,
    ) -> ApplicationAssignmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_arn_binding = args.application_arn.get_inner();
        let principal_id_binding = args.principal_id.get_inner();
        let principal_type_binding = args.principal_type.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ssoadmin/applicationAssignment:ApplicationAssignment".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationArn".into(),
                    value: &application_arn_binding,
                },
                register_interface::ObjectField {
                    name: "principalId".into(),
                    value: &principal_id_binding,
                },
                register_interface::ObjectField {
                    name: "principalType".into(),
                    value: &principal_type_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "applicationArn".into(),
                },
                register_interface::ResultField {
                    name: "principalId".into(),
                },
                register_interface::ResultField {
                    name: "principalType".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ApplicationAssignmentResult {
            application_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationArn").unwrap(),
            ),
            principal_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("principalId").unwrap(),
            ),
            principal_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("principalType").unwrap(),
            ),
        }
    }
}
