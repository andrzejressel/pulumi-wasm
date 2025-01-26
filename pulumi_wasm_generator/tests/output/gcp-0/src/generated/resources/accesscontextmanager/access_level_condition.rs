/// Allows configuring a single access level condition to be appended to an access level's conditions.
/// This resource is intended to be used in cases where it is not possible to compile a full list
/// of conditions to include in a `gcp.accesscontextmanager.AccessLevel` resource,
/// to enable them to be added separately.
///
/// > **Note:** If this resource is used alongside a `gcp.accesscontextmanager.AccessLevel` resource,
/// the access level resource must have a `lifecycle` block with `ignore_changes = [basic[0].conditions]` so
/// they don't fight over which service accounts should be included.
///
///
/// To get more information about AccessLevelCondition, see:
///
/// * [API documentation](https://cloud.google.com/access-context-manager/docs/reference/rest/v1/accessPolicies.accessLevels)
/// * How-to Guides
///     * [Access Policy Quickstart](https://cloud.google.com/access-context-manager/docs/quickstart)
///
/// > **Warning:** If you are using User ADCs (Application Default Credentials) with this resource,
/// you must specify a `billing_project` and set `user_project_override` to true
/// in the provider configuration. Otherwise the ACM API will return a 403 error.
/// Your account must have the `serviceusage.services.use` permission on the
/// `billing_project` you defined.
///
/// ## Example Usage
///
/// ### Access Context Manager Access Level Condition Basic
///
///
/// ```yaml
/// resources:
///   access-level-service-account:
///     type: gcp:accesscontextmanager:AccessLevel
///     properties:
///       parent: accessPolicies/${["access-policy"].name}
///       name: accessPolicies/${["access-policy"].name}/accessLevels/chromeos_no_lock
///       title: chromeos_no_lock
///       basic:
///         conditions:
///           - devicePolicy:
///               requireScreenLock: true
///               osConstraints:
///                 - osType: DESKTOP_CHROME_OS
///             regions:
///               - CH
///               - IT
///               - US
///   created-later:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: my-account-id
///   access-level-conditions:
///     type: gcp:accesscontextmanager:AccessLevelCondition
///     properties:
///       accessLevel: ${["access-level-service-account"].name}
///       ipSubnetworks:
///         - 192.0.4.0/24
///       members:
///         - user:test@google.com
///         - user:test2@google.com
///         - serviceAccount:${["created-later"].email}
///       negate: false
///       devicePolicy:
///         requireScreenLock: false
///         requireAdminApproval: false
///         requireCorpOwned: true
///         osConstraints:
///           - osType: DESKTOP_CHROME_OS
///       regions:
///         - IT
///         - US
///   access-policy:
///     type: gcp:accesscontextmanager:AccessPolicy
///     properties:
///       parent: organizations/123456789
///       title: my policy
/// ```
///
/// ## Import
///
/// This resource does not support import.
///
pub mod access_level_condition {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessLevelConditionArgs {
        /// The name of the Access Level to add this condition to.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub access_level: pulumi_wasm_rust::InputOrOutput<String>,
        /// Device specific restrictions, all restrictions must hold for
        /// the Condition to be true. If not specified, all devices are
        /// allowed.
        /// Structure is documented below.
        #[builder(into, default)]
        pub device_policy: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::accesscontextmanager::AccessLevelConditionDevicePolicy,
            >,
        >,
        /// A list of CIDR block IP subnetwork specification. May be IPv4
        /// or IPv6.
        /// Note that for a CIDR IP address block, the specified IP address
        /// portion must be properly truncated (i.e. all the host bits must
        /// be zero) or the input is considered malformed. For example,
        /// "192.0.2.0/24" is accepted but "192.0.2.1/24" is not. Similarly,
        /// for IPv6, "2001:db8::/32" is accepted whereas "2001:db8::1/32"
        /// is not. The originating IP of a request must be in one of the
        /// listed subnets in order for this Condition to be true.
        /// If empty, all IP addresses are allowed.
        #[builder(into, default)]
        pub ip_subnetworks: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// An allowed list of members (users, service accounts).
        /// Using groups is not supported yet.
        /// The signed-in user originating the request must be a part of one
        /// of the provided members. If not specified, a request may come
        /// from any user (logged in/not logged in, not present in any
        /// groups, etc.).
        /// Formats: `user:{emailid}`, `serviceAccount:{emailid}`
        #[builder(into, default)]
        pub members: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Whether to negate the Condition. If true, the Condition becomes
        /// a NAND over its non-empty fields, each field must be false for
        /// the Condition overall to be satisfied. Defaults to false.
        #[builder(into, default)]
        pub negate: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The request must originate from one of the provided
        /// countries/regions.
        /// Format: A valid ISO 3166-1 alpha-2 code.
        #[builder(into, default)]
        pub regions: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// A list of other access levels defined in the same Policy,
        /// referenced by resource name. Referencing an AccessLevel which
        /// does not exist is an error. All access levels listed must be
        /// granted for the Condition to be true.
        /// Format: accessPolicies/{policy_id}/accessLevels/{short_name}
        #[builder(into, default)]
        pub required_access_levels: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The request must originate from one of the provided VPC networks in Google Cloud. Cannot specify this field together with `ip_subnetworks`.
        /// Structure is documented below.
        #[builder(into, default)]
        pub vpc_network_sources: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::accesscontextmanager::AccessLevelConditionVpcNetworkSource,
                >,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct AccessLevelConditionResult {
        /// The name of the Access Level to add this condition to.
        ///
        ///
        /// - - -
        pub access_level: pulumi_wasm_rust::Output<String>,
        /// Device specific restrictions, all restrictions must hold for
        /// the Condition to be true. If not specified, all devices are
        /// allowed.
        /// Structure is documented below.
        pub device_policy: pulumi_wasm_rust::Output<
            Option<
                super::super::types::accesscontextmanager::AccessLevelConditionDevicePolicy,
            >,
        >,
        /// A list of CIDR block IP subnetwork specification. May be IPv4
        /// or IPv6.
        /// Note that for a CIDR IP address block, the specified IP address
        /// portion must be properly truncated (i.e. all the host bits must
        /// be zero) or the input is considered malformed. For example,
        /// "192.0.2.0/24" is accepted but "192.0.2.1/24" is not. Similarly,
        /// for IPv6, "2001:db8::/32" is accepted whereas "2001:db8::1/32"
        /// is not. The originating IP of a request must be in one of the
        /// listed subnets in order for this Condition to be true.
        /// If empty, all IP addresses are allowed.
        pub ip_subnetworks: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// An allowed list of members (users, service accounts).
        /// Using groups is not supported yet.
        /// The signed-in user originating the request must be a part of one
        /// of the provided members. If not specified, a request may come
        /// from any user (logged in/not logged in, not present in any
        /// groups, etc.).
        /// Formats: `user:{emailid}`, `serviceAccount:{emailid}`
        pub members: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Whether to negate the Condition. If true, the Condition becomes
        /// a NAND over its non-empty fields, each field must be false for
        /// the Condition overall to be satisfied. Defaults to false.
        pub negate: pulumi_wasm_rust::Output<Option<bool>>,
        /// The request must originate from one of the provided
        /// countries/regions.
        /// Format: A valid ISO 3166-1 alpha-2 code.
        pub regions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A list of other access levels defined in the same Policy,
        /// referenced by resource name. Referencing an AccessLevel which
        /// does not exist is an error. All access levels listed must be
        /// granted for the Condition to be true.
        /// Format: accessPolicies/{policy_id}/accessLevels/{short_name}
        pub required_access_levels: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The request must originate from one of the provided VPC networks in Google Cloud. Cannot specify this field together with `ip_subnetworks`.
        /// Structure is documented below.
        pub vpc_network_sources: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::accesscontextmanager::AccessLevelConditionVpcNetworkSource,
                >,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AccessLevelConditionArgs,
    ) -> AccessLevelConditionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_level_binding = args.access_level.get_output(context).get_inner();
        let device_policy_binding = args.device_policy.get_output(context).get_inner();
        let ip_subnetworks_binding = args.ip_subnetworks.get_output(context).get_inner();
        let members_binding = args.members.get_output(context).get_inner();
        let negate_binding = args.negate.get_output(context).get_inner();
        let regions_binding = args.regions.get_output(context).get_inner();
        let required_access_levels_binding = args
            .required_access_levels
            .get_output(context)
            .get_inner();
        let vpc_network_sources_binding = args
            .vpc_network_sources
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:accesscontextmanager/accessLevelCondition:AccessLevelCondition"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessLevel".into(),
                    value: &access_level_binding,
                },
                register_interface::ObjectField {
                    name: "devicePolicy".into(),
                    value: &device_policy_binding,
                },
                register_interface::ObjectField {
                    name: "ipSubnetworks".into(),
                    value: &ip_subnetworks_binding,
                },
                register_interface::ObjectField {
                    name: "members".into(),
                    value: &members_binding,
                },
                register_interface::ObjectField {
                    name: "negate".into(),
                    value: &negate_binding,
                },
                register_interface::ObjectField {
                    name: "regions".into(),
                    value: &regions_binding,
                },
                register_interface::ObjectField {
                    name: "requiredAccessLevels".into(),
                    value: &required_access_levels_binding,
                },
                register_interface::ObjectField {
                    name: "vpcNetworkSources".into(),
                    value: &vpc_network_sources_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessLevel".into(),
                },
                register_interface::ResultField {
                    name: "devicePolicy".into(),
                },
                register_interface::ResultField {
                    name: "ipSubnetworks".into(),
                },
                register_interface::ResultField {
                    name: "members".into(),
                },
                register_interface::ResultField {
                    name: "negate".into(),
                },
                register_interface::ResultField {
                    name: "regions".into(),
                },
                register_interface::ResultField {
                    name: "requiredAccessLevels".into(),
                },
                register_interface::ResultField {
                    name: "vpcNetworkSources".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AccessLevelConditionResult {
            access_level: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessLevel").unwrap(),
            ),
            device_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("devicePolicy").unwrap(),
            ),
            ip_subnetworks: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipSubnetworks").unwrap(),
            ),
            members: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("members").unwrap(),
            ),
            negate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("negate").unwrap(),
            ),
            regions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("regions").unwrap(),
            ),
            required_access_levels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requiredAccessLevels").unwrap(),
            ),
            vpc_network_sources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcNetworkSources").unwrap(),
            ),
        }
    }
}
