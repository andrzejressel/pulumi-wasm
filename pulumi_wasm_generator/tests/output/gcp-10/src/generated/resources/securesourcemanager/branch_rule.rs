/// BranchRule is the protection rule to enforce pre-defined rules on designated branches within a repository.
///
///
/// To get more information about BranchRule, see:
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/secure-source-manager/docs/overview)
///
/// ## Example Usage
///
/// ### Secure Source Manager Branch Rule Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basic = branch_rule::create(
///         "basic",
///         BranchRuleArgs::builder()
///             .branch_rule_id("my-basic-branchrule")
///             .include_pattern("main")
///             .location("${repository.location}")
///             .repository_id("${repository.repositoryId}")
///             .build_struct(),
///     );
///     let instance = instance::create(
///         "instance",
///         InstanceArgs::builder()
///             .instance_id("my-basic-instance")
///             .location("us-central1")
///             .build_struct(),
///     );
///     let repository = repository::create(
///         "repository",
///         RepositoryArgs::builder()
///             .instance("${instance.name}")
///             .location("${instance.location}")
///             .repository_id("my-basic-repository")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Secure Source Manager Branch Rule With Fields
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = branch_rule::create(
///         "default",
///         BranchRuleArgs::builder()
///             .allow_stale_reviews(false)
///             .branch_rule_id("my-initial-branchrule")
///             .disabled(false)
///             .include_pattern("test")
///             .location("${repository.location}")
///             .minimum_approvals_count(2)
///             .minimum_reviews_count(2)
///             .repository_id("${repository.repositoryId}")
///             .require_comments_resolved(true)
///             .require_linear_history(true)
///             .require_pull_request(true)
///             .build_struct(),
///     );
///     let instance = instance::create(
///         "instance",
///         InstanceArgs::builder()
///             .instance_id("my-initial-instance")
///             .location("us-central1")
///             .build_struct(),
///     );
///     let repository = repository::create(
///         "repository",
///         RepositoryArgs::builder()
///             .instance("${instance.name}")
///             .location("${instance.location}")
///             .repository_id("my-initial-repository")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// BranchRule can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/repositories/{{repository_id}}/branchRules/{{branch_rule_id}}`
///
/// * `{{project}}/{{location}}/{{repository_id}}/{{branch_rule_id}}`
///
/// * `{{location}}/{{repository_id}}/{{branch_rule_id}}`
///
/// * `{{branch_rule_id}}`
///
/// When using the `pulumi import` command, BranchRule can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:securesourcemanager/branchRule:BranchRule default projects/{{project}}/locations/{{location}}/repositories/{{repository_id}}/branchRules/{{branch_rule_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:securesourcemanager/branchRule:BranchRule default {{project}}/{{location}}/{{repository_id}}/{{branch_rule_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:securesourcemanager/branchRule:BranchRule default {{location}}/{{repository_id}}/{{branch_rule_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:securesourcemanager/branchRule:BranchRule default {{branch_rule_id}}
/// ```
///
pub mod branch_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BranchRuleArgs {
        /// Determines if allow stale reviews or approvals before merging to the branch.
        #[builder(into, default)]
        pub allow_stale_reviews: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The ID for the BranchRule.
        #[builder(into)]
        pub branch_rule_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Determines if the branch rule is disabled or not.
        #[builder(into, default)]
        pub disabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The BranchRule matches branches based on the specified regular expression. Use .* to match all branches.
        #[builder(into)]
        pub include_pattern: pulumi_wasm_rust::InputOrOutput<String>,
        /// The location for the Repository.
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// The minimum number of approvals required for the branch rule to be matched.
        #[builder(into, default)]
        pub minimum_approvals_count: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The minimum number of reviews required for the branch rule to be matched.
        #[builder(into, default)]
        pub minimum_reviews_count: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID for the Repository.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub repository_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Determines if require comments resolved before merging to the branch.
        #[builder(into, default)]
        pub require_comments_resolved: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Determines if require linear history before merging to the branch.
        #[builder(into, default)]
        pub require_linear_history: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Determines if the branch rule requires a pull request or not.
        #[builder(into, default)]
        pub require_pull_request: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct BranchRuleResult {
        /// Determines if allow stale reviews or approvals before merging to the branch.
        pub allow_stale_reviews: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID for the BranchRule.
        pub branch_rule_id: pulumi_wasm_rust::Output<String>,
        /// Time the BranchRule was created in UTC.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Determines if the branch rule is disabled or not.
        pub disabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The BranchRule matches branches based on the specified regular expression. Use .* to match all branches.
        pub include_pattern: pulumi_wasm_rust::Output<String>,
        /// The location for the Repository.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The minimum number of approvals required for the branch rule to be matched.
        pub minimum_approvals_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// The minimum number of reviews required for the branch rule to be matched.
        pub minimum_reviews_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// The resource name for the BranchRule.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The ID for the Repository.
        ///
        ///
        /// - - -
        pub repository_id: pulumi_wasm_rust::Output<String>,
        /// Determines if require comments resolved before merging to the branch.
        pub require_comments_resolved: pulumi_wasm_rust::Output<Option<bool>>,
        /// Determines if require linear history before merging to the branch.
        pub require_linear_history: pulumi_wasm_rust::Output<Option<bool>>,
        /// Determines if the branch rule requires a pull request or not.
        pub require_pull_request: pulumi_wasm_rust::Output<Option<bool>>,
        /// Unique identifier of the BranchRule.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// Time the BranchRule was updated in UTC.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: BranchRuleArgs,
    ) -> BranchRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let allow_stale_reviews_binding = args
            .allow_stale_reviews
            .get_output(context)
            .get_inner();
        let branch_rule_id_binding = args.branch_rule_id.get_output(context).get_inner();
        let disabled_binding = args.disabled.get_output(context).get_inner();
        let include_pattern_binding = args
            .include_pattern
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let minimum_approvals_count_binding = args
            .minimum_approvals_count
            .get_output(context)
            .get_inner();
        let minimum_reviews_count_binding = args
            .minimum_reviews_count
            .get_output(context)
            .get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let repository_id_binding = args.repository_id.get_output(context).get_inner();
        let require_comments_resolved_binding = args
            .require_comments_resolved
            .get_output(context)
            .get_inner();
        let require_linear_history_binding = args
            .require_linear_history
            .get_output(context)
            .get_inner();
        let require_pull_request_binding = args
            .require_pull_request
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:securesourcemanager/branchRule:BranchRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allowStaleReviews".into(),
                    value: &allow_stale_reviews_binding,
                },
                register_interface::ObjectField {
                    name: "branchRuleId".into(),
                    value: &branch_rule_id_binding,
                },
                register_interface::ObjectField {
                    name: "disabled".into(),
                    value: &disabled_binding,
                },
                register_interface::ObjectField {
                    name: "includePattern".into(),
                    value: &include_pattern_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "minimumApprovalsCount".into(),
                    value: &minimum_approvals_count_binding,
                },
                register_interface::ObjectField {
                    name: "minimumReviewsCount".into(),
                    value: &minimum_reviews_count_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "repositoryId".into(),
                    value: &repository_id_binding,
                },
                register_interface::ObjectField {
                    name: "requireCommentsResolved".into(),
                    value: &require_comments_resolved_binding,
                },
                register_interface::ObjectField {
                    name: "requireLinearHistory".into(),
                    value: &require_linear_history_binding,
                },
                register_interface::ObjectField {
                    name: "requirePullRequest".into(),
                    value: &require_pull_request_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        BranchRuleResult {
            allow_stale_reviews: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("allowStaleReviews"),
            ),
            branch_rule_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("branchRuleId"),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            disabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("disabled"),
            ),
            include_pattern: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("includePattern"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            minimum_approvals_count: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("minimumApprovalsCount"),
            ),
            minimum_reviews_count: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("minimumReviewsCount"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            repository_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("repositoryId"),
            ),
            require_comments_resolved: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("requireCommentsResolved"),
            ),
            require_linear_history: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("requireLinearHistory"),
            ),
            require_pull_request: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("requirePullRequest"),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(o.extract_field("uid")),
            update_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
