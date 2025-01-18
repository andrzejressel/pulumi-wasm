/// Resource for managing an AWS Route 53 Profiles Resource Association.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:route53:ProfilesProfile
///     properties:
///       name: example
///   exampleVpc:
///     type: aws:ec2:Vpc
///     name: example
///     properties:
///       cidr: 10.0.0.0/16
///   exampleZone:
///     type: aws:route53:Zone
///     name: example
///     properties:
///       name: example.com
///       vpcs:
///         - vpcId: ${exampleVpc.id}
///   exampleProfilesResourceAssociation:
///     type: aws:route53:ProfilesResourceAssociation
///     name: example
///     properties:
///       name: example
///       profileId: ${example.id}
///       resourceArn: ${exampleZone.arn}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Route 53 Profiles Resource Association using the `example_id_arg`. For example:
///
/// ```sh
/// $ pulumi import aws:route53/profilesResourceAssociation:ProfilesResourceAssociation example rpa-id-12345678
/// ```
pub mod profiles_resource_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProfilesResourceAssociationArgs {
        /// Name of the Profile Resource Association.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of the profile associated with the VPC.
        #[builder(into)]
        pub profile_id: pulumi_wasm_rust::Output<String>,
        /// Resource ID of the resource to be associated with the profile.
        #[builder(into)]
        pub resource_arn: pulumi_wasm_rust::Output<String>,
        /// Resource properties for the resource to be associated with the profile.
        #[builder(into, default)]
        pub resource_properties: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::route53::ProfilesResourceAssociationTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct ProfilesResourceAssociationResult {
        /// Name of the Profile Resource Association.
        pub name: pulumi_wasm_rust::Output<String>,
        pub owner_id: pulumi_wasm_rust::Output<String>,
        /// ID of the profile associated with the VPC.
        pub profile_id: pulumi_wasm_rust::Output<String>,
        /// Resource ID of the resource to be associated with the profile.
        pub resource_arn: pulumi_wasm_rust::Output<String>,
        /// Resource properties for the resource to be associated with the profile.
        pub resource_properties: pulumi_wasm_rust::Output<Option<String>>,
        /// Type of resource associated with the profile.
        pub resource_type: pulumi_wasm_rust::Output<String>,
        /// Status of the Profile Association. Valid values [AWS docs](https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53profiles_Profile.html)
        pub status: pulumi_wasm_rust::Output<String>,
        /// Status message of the Profile Resource Association.
        pub status_message: pulumi_wasm_rust::Output<String>,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::route53::ProfilesResourceAssociationTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ProfilesResourceAssociationArgs,
    ) -> ProfilesResourceAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let profile_id_binding = args.profile_id.get_inner();
        let resource_arn_binding = args.resource_arn.get_inner();
        let resource_properties_binding = args.resource_properties.get_inner();
        let timeouts_binding = args.timeouts.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:route53/profilesResourceAssociation:ProfilesResourceAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "profileId".into(),
                    value: &profile_id_binding,
                },
                register_interface::ObjectField {
                    name: "resourceArn".into(),
                    value: &resource_arn_binding,
                },
                register_interface::ObjectField {
                    name: "resourceProperties".into(),
                    value: &resource_properties_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "ownerId".into(),
                },
                register_interface::ResultField {
                    name: "profileId".into(),
                },
                register_interface::ResultField {
                    name: "resourceArn".into(),
                },
                register_interface::ResultField {
                    name: "resourceProperties".into(),
                },
                register_interface::ResultField {
                    name: "resourceType".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "statusMessage".into(),
                },
                register_interface::ResultField {
                    name: "timeouts".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ProfilesResourceAssociationResult {
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerId").unwrap(),
            ),
            profile_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("profileId").unwrap(),
            ),
            resource_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceArn").unwrap(),
            ),
            resource_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceProperties").unwrap(),
            ),
            resource_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceType").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            status_message: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statusMessage").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
        }
    }
}
