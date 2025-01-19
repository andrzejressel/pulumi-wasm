/// Manages an App Runner VPC Connector.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let connector = vpc_connector::create(
///         "connector",
///         VpcConnectorArgs::builder()
///             .security_groups(vec!["sg1", "sg2",])
///             .subnets(vec!["subnet1", "subnet2",])
///             .vpc_connector_name("name")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import App Runner vpc connector using the `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:apprunner/vpcConnector:VpcConnector example arn:aws:apprunner:us-east-1:1234567890:vpcconnector/example/1/0a03292a89764e5882c41d8f991c82fe
/// ```
pub mod vpc_connector {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcConnectorArgs {
        /// List of IDs of security groups that App Runner should use for access to AWS resources under the specified subnets. If not specified, App Runner uses the default security group of the Amazon VPC. The default security group allows all outbound traffic.
        #[builder(into)]
        pub security_groups: pulumi_wasm_rust::Output<Vec<String>>,
        /// List of IDs of subnets that App Runner should use when it associates your service with a custom Amazon VPC. Specify IDs of subnets of a single Amazon VPC. App Runner determines the Amazon VPC from the subnets you specify.
        #[builder(into)]
        pub subnets: pulumi_wasm_rust::Output<Vec<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name for the VPC connector.
        #[builder(into)]
        pub vpc_connector_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct VpcConnectorResult {
        /// ARN of VPC connector.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// List of IDs of security groups that App Runner should use for access to AWS resources under the specified subnets. If not specified, App Runner uses the default security group of the Amazon VPC. The default security group allows all outbound traffic.
        pub security_groups: pulumi_wasm_rust::Output<Vec<String>>,
        /// Current state of the VPC connector. If the status of a connector revision is INACTIVE, it was deleted and can't be used. Inactive connector revisions are permanently removed some time after they are deleted.
        pub status: pulumi_wasm_rust::Output<String>,
        /// List of IDs of subnets that App Runner should use when it associates your service with a custom Amazon VPC. Specify IDs of subnets of a single Amazon VPC. App Runner determines the Amazon VPC from the subnets you specify.
        pub subnets: pulumi_wasm_rust::Output<Vec<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Name for the VPC connector.
        pub vpc_connector_name: pulumi_wasm_rust::Output<String>,
        /// The revision of VPC connector. It's unique among all the active connectors ("Status": "ACTIVE") that share the same Name.
        pub vpc_connector_revision: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: VpcConnectorArgs) -> VpcConnectorResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let security_groups_binding = args.security_groups.get_inner();
        let subnets_binding = args.subnets.get_inner();
        let tags_binding = args.tags.get_inner();
        let vpc_connector_name_binding = args.vpc_connector_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:apprunner/vpcConnector:VpcConnector".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "securityGroups".into(),
                    value: &security_groups_binding,
                },
                register_interface::ObjectField {
                    name: "subnets".into(),
                    value: &subnets_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "vpcConnectorName".into(),
                    value: &vpc_connector_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "securityGroups".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "subnets".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "vpcConnectorName".into(),
                },
                register_interface::ResultField {
                    name: "vpcConnectorRevision".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VpcConnectorResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            security_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroups").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            subnets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnets").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            vpc_connector_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcConnectorName").unwrap(),
            ),
            vpc_connector_revision: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcConnectorRevision").unwrap(),
            ),
        }
    }
}
