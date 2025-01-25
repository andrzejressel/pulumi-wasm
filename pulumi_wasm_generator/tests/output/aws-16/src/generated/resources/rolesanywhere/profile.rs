/// Resource for managing a Roles Anywhere Profile.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   test:
///     type: aws:iam:Role
///     properties:
///       name: test
///       path: /
///       assumeRolePolicy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Action:
///                 - sts:AssumeRole
///                 - sts:TagSession
///                 - sts:SetSourceIdentity
///               Principal:
///                 Service: rolesanywhere.amazonaws.com
///               Effect: Allow
///               Sid: ""
///   testProfile:
///     type: aws:rolesanywhere:Profile
///     name: test
///     properties:
///       name: example
///       roleArns:
///         - ${test.arn}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_rolesanywhere_profile` using its `id`. For example:
///
/// ```sh
/// $ pulumi import aws:rolesanywhere/profile:Profile example db138a85-8925-4f9f-a409-08231233cacf
/// ```
pub mod profile {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProfileArgs {
        /// The number of seconds the vended session credentials are valid for. Defaults to 3600.
        #[builder(into, default)]
        pub duration_seconds: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Whether or not the Profile is enabled.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// A list of managed policy ARNs that apply to the vended session credentials.
        #[builder(into, default)]
        pub managed_policy_arns: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The name of the Profile.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies whether instance properties are required in [CreateSession](https://docs.aws.amazon.com/rolesanywhere/latest/APIReference/API_CreateSession.html) requests with this profile.
        #[builder(into, default)]
        pub require_instance_properties: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// A list of IAM roles that this profile can assume
        #[builder(into, default)]
        pub role_arns: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// A session policy that applies to the trust boundary of the vended session credentials.
        #[builder(into, default)]
        pub session_policy: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ProfileResult {
        /// Amazon Resource Name (ARN) of the Profile
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The number of seconds the vended session credentials are valid for. Defaults to 3600.
        pub duration_seconds: pulumi_wasm_rust::Output<i32>,
        /// Whether or not the Profile is enabled.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A list of managed policy ARNs that apply to the vended session credentials.
        pub managed_policy_arns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The name of the Profile.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies whether instance properties are required in [CreateSession](https://docs.aws.amazon.com/rolesanywhere/latest/APIReference/API_CreateSession.html) requests with this profile.
        pub require_instance_properties: pulumi_wasm_rust::Output<Option<bool>>,
        /// A list of IAM roles that this profile can assume
        pub role_arns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A session policy that applies to the trust boundary of the vended session credentials.
        pub session_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ProfileArgs,
    ) -> ProfileResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let duration_seconds_binding = args
            .duration_seconds
            .get_output(context)
            .get_inner();
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let managed_policy_arns_binding = args
            .managed_policy_arns
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let require_instance_properties_binding = args
            .require_instance_properties
            .get_output(context)
            .get_inner();
        let role_arns_binding = args.role_arns.get_output(context).get_inner();
        let session_policy_binding = args.session_policy.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:rolesanywhere/profile:Profile".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "durationSeconds".into(),
                    value: &duration_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "managedPolicyArns".into(),
                    value: &managed_policy_arns_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "requireInstanceProperties".into(),
                    value: &require_instance_properties_binding,
                },
                register_interface::ObjectField {
                    name: "roleArns".into(),
                    value: &role_arns_binding,
                },
                register_interface::ObjectField {
                    name: "sessionPolicy".into(),
                    value: &session_policy_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "durationSeconds".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "managedPolicyArns".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "requireInstanceProperties".into(),
                },
                register_interface::ResultField {
                    name: "roleArns".into(),
                },
                register_interface::ResultField {
                    name: "sessionPolicy".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ProfileResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            duration_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("durationSeconds").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            managed_policy_arns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managedPolicyArns").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            require_instance_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requireInstanceProperties").unwrap(),
            ),
            role_arns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArns").unwrap(),
            ),
            session_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sessionPolicy").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
