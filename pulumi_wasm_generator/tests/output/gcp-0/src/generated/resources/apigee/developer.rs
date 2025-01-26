/// A `Developer` is an API consumer that can have apps registered in Apigee.
///
///
/// To get more information about Developer, see:
///
/// * [API documentation](https://cloud.google.com/apigee/docs/reference/apis/apigee/rest/v1/organizations.developers)
/// * How-to Guides
///     * [Creating a developer](https://cloud.google.com/apigee/docs/api-platform/publish/adding-developers-your-api-product)
///
/// ## Example Usage
///
/// ### Apigee Developer Basic
///
///
/// ```yaml
/// resources:
///   apigeeNetwork:
///     type: gcp:compute:Network
///     name: apigee_network
///     properties:
///       name: apigee-network
///   apigeeRange:
///     type: gcp:compute:GlobalAddress
///     name: apigee_range
///     properties:
///       name: apigee-range
///       purpose: VPC_PEERING
///       addressType: INTERNAL
///       prefixLength: 16
///       network: ${apigeeNetwork.id}
///   apigeeVpcConnection:
///     type: gcp:servicenetworking:Connection
///     name: apigee_vpc_connection
///     properties:
///       network: ${apigeeNetwork.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${apigeeRange.name}
///   apigeeOrg:
///     type: gcp:apigee:Organization
///     name: apigee_org
///     properties:
///       analyticsRegion: us-central1
///       projectId: ${current.project}
///       authorizedNetwork: ${apigeeNetwork.id}
///     options:
///       dependsOn:
///         - ${apigeeVpcConnection}
///   apigeeInstance:
///     type: gcp:apigee:Instance
///     name: apigee_instance
///     properties:
///       name: my-instance
///       location: us-central1
///       orgId: ${apigeeOrg.id}
///       peeringCidrRange: SLASH_22
///   apigeeDeveloper:
///     type: gcp:apigee:Developer
///     name: apigee_developer
///     properties:
///       email: john.doe@acme.com
///       firstName: John
///       lastName: Doe
///       userName: john.doe
///       orgId: ${apigeeOrg.id}
///     options:
///       dependsOn:
///         - ${apigeeInstance}
/// variables:
///   current:
///     fn::invoke:
///       function: gcp:organizations:getClientConfig
///       arguments: {}
/// ```
/// ### Apigee Developer With Attributes
///
///
/// ```yaml
/// resources:
///   apigeeNetwork:
///     type: gcp:compute:Network
///     name: apigee_network
///     properties:
///       name: apigee-network
///   apigeeRange:
///     type: gcp:compute:GlobalAddress
///     name: apigee_range
///     properties:
///       name: apigee-range
///       purpose: VPC_PEERING
///       addressType: INTERNAL
///       prefixLength: 16
///       network: ${apigeeNetwork.id}
///   apigeeVpcConnection:
///     type: gcp:servicenetworking:Connection
///     name: apigee_vpc_connection
///     properties:
///       network: ${apigeeNetwork.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${apigeeRange.name}
///   apigeeOrg:
///     type: gcp:apigee:Organization
///     name: apigee_org
///     properties:
///       analyticsRegion: us-central1
///       projectId: ${current.project}
///       authorizedNetwork: ${apigeeNetwork.id}
///     options:
///       dependsOn:
///         - ${apigeeVpcConnection}
///   apigeeInstance:
///     type: gcp:apigee:Instance
///     name: apigee_instance
///     properties:
///       name: my-instance
///       location: us-central1
///       orgId: ${apigeeOrg.id}
///       peeringCidrRange: SLASH_22
///   apigeeDeveloper:
///     type: gcp:apigee:Developer
///     name: apigee_developer
///     properties:
///       email: john.doe@acme.com
///       firstName: John
///       lastName: Doe
///       userName: john.doe
///       attributes:
///         - name: business_unit
///           value: HR
///         - name: department
///           value: payroll
///       orgId: ${apigeeOrg.id}
///     options:
///       dependsOn:
///         - ${apigeeInstance}
/// variables:
///   current:
///     fn::invoke:
///       function: gcp:organizations:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Developer can be imported using any of these accepted formats:
///
/// * `{{org_id}}/developers/{{email}}`
///
/// * `{{org_id}}/{{email}}`
///
/// When using the `pulumi import` command, Developer can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:apigee/developer:Developer default {{org_id}}/developers/{{email}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apigee/developer:Developer default {{org_id}}/{{email}}
/// ```
///
pub mod developer {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DeveloperArgs {
        /// Developer attributes (name/value pairs). The custom attribute limit is 18.
        /// Structure is documented below.
        #[builder(into, default)]
        pub attributes: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::apigee::DeveloperAttribute>>,
        >,
        /// Email address of the developer. This value is used to uniquely identify the developer in Apigee hybrid. Note that the email address has to be in lowercase only..
        #[builder(into)]
        pub email: pulumi_wasm_rust::InputOrOutput<String>,
        /// First name of the developer.
        #[builder(into)]
        pub first_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Last name of the developer.
        #[builder(into)]
        pub last_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Apigee Organization associated with the Apigee instance,
        /// in the format `organizations/{{org_name}}`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub org_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// User name of the developer. Not used by Apigee hybrid.
        #[builder(into)]
        pub user_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DeveloperResult {
        /// Developer attributes (name/value pairs). The custom attribute limit is 18.
        /// Structure is documented below.
        pub attributes: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::apigee::DeveloperAttribute>>,
        >,
        /// Time at which the developer was created in milliseconds since epoch.
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// Email address of the developer. This value is used to uniquely identify the developer in Apigee hybrid. Note that the email address has to be in lowercase only..
        pub email: pulumi_wasm_rust::Output<String>,
        /// First name of the developer.
        pub first_name: pulumi_wasm_rust::Output<String>,
        /// Time at which the developer was last modified in milliseconds since epoch.
        pub last_modified_at: pulumi_wasm_rust::Output<String>,
        /// Last name of the developer.
        pub last_name: pulumi_wasm_rust::Output<String>,
        /// The Apigee Organization associated with the Apigee instance,
        /// in the format `organizations/{{org_name}}`.
        ///
        ///
        /// - - -
        pub org_id: pulumi_wasm_rust::Output<String>,
        /// Name of the Apigee organization in which the developer resides.
        pub organizatio_name: pulumi_wasm_rust::Output<String>,
        /// Status of the developer. Valid values are active and inactive.
        pub status: pulumi_wasm_rust::Output<String>,
        /// User name of the developer. Not used by Apigee hybrid.
        pub user_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: DeveloperArgs,
    ) -> DeveloperResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let attributes_binding = args.attributes.get_output(context).get_inner();
        let email_binding = args.email.get_output(context).get_inner();
        let first_name_binding = args.first_name.get_output(context).get_inner();
        let last_name_binding = args.last_name.get_output(context).get_inner();
        let org_id_binding = args.org_id.get_output(context).get_inner();
        let user_name_binding = args.user_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:apigee/developer:Developer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "attributes".into(),
                    value: &attributes_binding,
                },
                register_interface::ObjectField {
                    name: "email".into(),
                    value: &email_binding,
                },
                register_interface::ObjectField {
                    name: "firstName".into(),
                    value: &first_name_binding,
                },
                register_interface::ObjectField {
                    name: "lastName".into(),
                    value: &last_name_binding,
                },
                register_interface::ObjectField {
                    name: "orgId".into(),
                    value: &org_id_binding,
                },
                register_interface::ObjectField {
                    name: "userName".into(),
                    value: &user_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DeveloperResult {
            attributes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("attributes"),
            ),
            created_at: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createdAt"),
            ),
            email: pulumi_wasm_rust::__private::into_domain(o.extract_field("email")),
            first_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("firstName"),
            ),
            last_modified_at: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lastModifiedAt"),
            ),
            last_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lastName"),
            ),
            org_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("orgId")),
            organizatio_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("organizatioName"),
            ),
            status: pulumi_wasm_rust::__private::into_domain(o.extract_field("status")),
            user_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("userName"),
            ),
        }
    }
}
