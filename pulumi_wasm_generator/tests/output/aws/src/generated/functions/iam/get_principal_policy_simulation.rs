pub mod get_principal_policy_simulation {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPrincipalPolicySimulationArgs {
        /// A set of IAM action names to run simulations for. Each entry in this set adds an additional hypothetical request to the simulation.
        ///
        /// Action names consist of a service prefix and an action verb separated by a colon, such as `s3:GetObject`. Refer to [Actions, resources, and condition keys for AWS services](https://docs.aws.amazon.com/service-authorization/latest/reference/reference_policies_actions-resources-contextkeys.html) to see the full set of possible IAM action names across all AWS services.
        #[builder(into)]
        pub action_names: pulumi_wasm_rust::Output<Vec<String>>,
        /// A set of additional principal policy documents to include in the simulation. The simulator will behave as if each of these policies were associated with the object specified in `policy_source_arn`, allowing you to test the effect of hypothetical policies not yet created.
        #[builder(into, default)]
        pub additional_policies_jsons: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The ARN of an user that will appear as the "caller" of the simulated requests. If you do not specify `caller_arn` then the simulation will use the `policy_source_arn` instead, if it contains a user ARN.
        #[builder(into, default)]
        pub caller_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Each `context` block defines an entry in the table of additional context keys in the simulated request.
        ///
        /// IAM uses context keys for both custom conditions and for interpolating dynamic request-specific values into policy values. If you use policies that include those features then you will need to provide suitable example values for those keys to achieve a realistic simulation.
        #[builder(into, default)]
        pub contexts: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::super::types::iam::GetPrincipalPolicySimulationContext>,
            >,
        >,
        /// A set of [permissions boundary policy documents](https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_boundaries.html) to include in the simulation.
        #[builder(into, default)]
        pub permissions_boundary_policies_jsons: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        /// The [ARN](https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html) of the IAM user, group, or role whose policies will be included in the simulation.
        ///
        /// You must closely match the form of the real service request you are simulating in order to achieve a realistic result. You can use the following additional arguments to specify other characteristics of the simulated requests:
        #[builder(into)]
        pub policy_source_arn: pulumi_wasm_rust::Output<String>,
        /// A set of ARNs of resources to include in the simulation.
        ///
        /// This argument is important for actions that have either required or optional resource types listed in [Actions, resources, and condition keys for AWS services](https://docs.aws.amazon.com/service-authorization/latest/reference/reference_policies_actions-resources-contextkeys.html), and you must provide ARNs that identify AWS objects of the appropriate types for the chosen actions.
        ///
        /// The policy simulator only automatically loads policies associated with the `policy_source_arn`, so if your given resources have their own resource-level policy then you'll also need to provide that explicitly using the `resource_policy_json` argument to achieve a realistic simulation.
        #[builder(into, default)]
        pub resource_arns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies a special simulation type to run. Some EC2 actions require special simulation behaviors and a particular set of resource ARNs to achieve a realistic result.
        ///
        /// For more details, see the `ResourceHandlingOption` request parameter for [the underlying `iam:SimulatePrincipalPolicy` action](https://docs.aws.amazon.com/IAM/latest/APIReference/API_SimulatePrincipalPolicy.html).
        #[builder(into, default)]
        pub resource_handling_option: pulumi_wasm_rust::Output<Option<String>>,
        /// An AWS account ID to use for any resource ARN in `resource_arns` that doesn't include its own AWS account ID. If unspecified, the simulator will use the account ID from the `caller_arn` argument as a placeholder.
        #[builder(into, default)]
        pub resource_owner_account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// An IAM policy document representing the resource-level policy of all of the resources specified in `resource_arns`.
        ///
        /// The policy simulator cannot automatically load policies that are associated with individual resources, as described in the documentation for `resource_arns` above.
        #[builder(into, default)]
        pub resource_policy_json: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetPrincipalPolicySimulationResult {
        pub action_names: pulumi_wasm_rust::Output<Vec<String>>,
        pub additional_policies_jsons: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// `true` if all of the simulation results have decision "allowed", or `false` otherwise.
        pub all_allowed: pulumi_wasm_rust::Output<bool>,
        pub caller_arn: pulumi_wasm_rust::Output<Option<String>>,
        pub contexts: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::super::types::iam::GetPrincipalPolicySimulationContext>,
            >,
        >,
        pub id: pulumi_wasm_rust::Output<String>,
        pub permissions_boundary_policies_jsons: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        pub policy_source_arn: pulumi_wasm_rust::Output<String>,
        pub resource_arns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub resource_handling_option: pulumi_wasm_rust::Output<Option<String>>,
        pub resource_owner_account_id: pulumi_wasm_rust::Output<Option<String>>,
        pub resource_policy_json: pulumi_wasm_rust::Output<Option<String>>,
        /// A set of result objects, one for each of the simulated requests, with the following nested attributes:
        pub results: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::iam::GetPrincipalPolicySimulationResult>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetPrincipalPolicySimulationArgs,
    ) -> GetPrincipalPolicySimulationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let action_names_binding = args.action_names.get_inner();
        let additional_policies_jsons_binding = args
            .additional_policies_jsons
            .get_inner();
        let caller_arn_binding = args.caller_arn.get_inner();
        let contexts_binding = args.contexts.get_inner();
        let permissions_boundary_policies_jsons_binding = args
            .permissions_boundary_policies_jsons
            .get_inner();
        let policy_source_arn_binding = args.policy_source_arn.get_inner();
        let resource_arns_binding = args.resource_arns.get_inner();
        let resource_handling_option_binding = args.resource_handling_option.get_inner();
        let resource_owner_account_id_binding = args
            .resource_owner_account_id
            .get_inner();
        let resource_policy_json_binding = args.resource_policy_json.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:iam/getPrincipalPolicySimulation:getPrincipalPolicySimulation"
                .into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "actionNames".into(),
                    value: &action_names_binding,
                },
                register_interface::ObjectField {
                    name: "additionalPoliciesJsons".into(),
                    value: &additional_policies_jsons_binding,
                },
                register_interface::ObjectField {
                    name: "callerArn".into(),
                    value: &caller_arn_binding,
                },
                register_interface::ObjectField {
                    name: "contexts".into(),
                    value: &contexts_binding,
                },
                register_interface::ObjectField {
                    name: "permissionsBoundaryPoliciesJsons".into(),
                    value: &permissions_boundary_policies_jsons_binding,
                },
                register_interface::ObjectField {
                    name: "policySourceArn".into(),
                    value: &policy_source_arn_binding,
                },
                register_interface::ObjectField {
                    name: "resourceArns".into(),
                    value: &resource_arns_binding,
                },
                register_interface::ObjectField {
                    name: "resourceHandlingOption".into(),
                    value: &resource_handling_option_binding,
                },
                register_interface::ObjectField {
                    name: "resourceOwnerAccountId".into(),
                    value: &resource_owner_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "resourcePolicyJson".into(),
                    value: &resource_policy_json_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "actionNames".into(),
                },
                register_interface::ResultField {
                    name: "additionalPoliciesJsons".into(),
                },
                register_interface::ResultField {
                    name: "allAllowed".into(),
                },
                register_interface::ResultField {
                    name: "callerArn".into(),
                },
                register_interface::ResultField {
                    name: "contexts".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "permissionsBoundaryPoliciesJsons".into(),
                },
                register_interface::ResultField {
                    name: "policySourceArn".into(),
                },
                register_interface::ResultField {
                    name: "resourceArns".into(),
                },
                register_interface::ResultField {
                    name: "resourceHandlingOption".into(),
                },
                register_interface::ResultField {
                    name: "resourceOwnerAccountId".into(),
                },
                register_interface::ResultField {
                    name: "resourcePolicyJson".into(),
                },
                register_interface::ResultField {
                    name: "results".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetPrincipalPolicySimulationResult {
            action_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("actionNames").unwrap(),
            ),
            additional_policies_jsons: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("additionalPoliciesJsons").unwrap(),
            ),
            all_allowed: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allAllowed").unwrap(),
            ),
            caller_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("callerArn").unwrap(),
            ),
            contexts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contexts").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            permissions_boundary_policies_jsons: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("permissionsBoundaryPoliciesJsons").unwrap(),
            ),
            policy_source_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policySourceArn").unwrap(),
            ),
            resource_arns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceArns").unwrap(),
            ),
            resource_handling_option: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceHandlingOption").unwrap(),
            ),
            resource_owner_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceOwnerAccountId").unwrap(),
            ),
            resource_policy_json: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourcePolicyJson").unwrap(),
            ),
            results: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("results").unwrap(),
            ),
        }
    }
}