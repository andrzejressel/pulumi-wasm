/// Provides an environment member to an AWS Cloud9 development environment.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = environment_ec_2::create(
///         "test",
///         EnvironmentEc2Args::builder()
///             .instance_type("t2.micro")
///             .name("some-env")
///             .build_struct(),
///     );
///     let testEnvironmentMembership = environment_membership::create(
///         "testEnvironmentMembership",
///         EnvironmentMembershipArgs::builder()
///             .environment_id("${test.id}")
///             .permissions("read-only")
///             .user_arn("${testUser.arn}")
///             .build_struct(),
///     );
///     let testUser = user::create(
///         "testUser",
///         UserArgs::builder().name("some-user").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Cloud9 environment membership using the `environment-id#user-arn`. For example:
///
/// ```sh
/// $ pulumi import aws:cloud9/environmentMembership:EnvironmentMembership test environment-id#user-arn
/// ```
pub mod environment_membership {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnvironmentMembershipArgs {
        /// The ID of the environment that contains the environment member you want to add.
        #[builder(into)]
        pub environment_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of environment member permissions you want to associate with this environment member. Allowed values are `read-only` and `read-write` .
        #[builder(into)]
        pub permissions: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Amazon Resource Name (ARN) of the environment member you want to add.
        #[builder(into)]
        pub user_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct EnvironmentMembershipResult {
        /// The ID of the environment that contains the environment member you want to add.
        pub environment_id: pulumi_gestalt_rust::Output<String>,
        /// The type of environment member permissions you want to associate with this environment member. Allowed values are `read-only` and `read-write` .
        pub permissions: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the environment member you want to add.
        pub user_arn: pulumi_gestalt_rust::Output<String>,
        /// The user ID in AWS Identity and Access Management (AWS IAM) of the environment member.
        pub user_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: EnvironmentMembershipArgs,
    ) -> EnvironmentMembershipResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let environment_id_binding = args.environment_id.get_output(context).get_inner();
        let permissions_binding = args.permissions.get_output(context).get_inner();
        let user_arn_binding = args.user_arn.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloud9/environmentMembership:EnvironmentMembership".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "environmentId".into(),
                    value: &environment_id_binding,
                },
                register_interface::ObjectField {
                    name: "permissions".into(),
                    value: &permissions_binding,
                },
                register_interface::ObjectField {
                    name: "userArn".into(),
                    value: &user_arn_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        EnvironmentMembershipResult {
            environment_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("environmentId"),
            ),
            permissions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("permissions"),
            ),
            user_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userArn"),
            ),
            user_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userId"),
            ),
        }
    }
}
