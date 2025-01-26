/// Provides a WorkSpaces directory in AWS WorkSpaces Service.
///
/// > **NOTE:** AWS WorkSpaces service requires [`workspaces_DefaultRole`](https://docs.aws.amazon.com/workspaces/latest/adminguide/workspaces-access-control.html#create-default-role) IAM role to operate normally.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:workspaces:Directory
///     properties:
///       directoryId: ${exampleDirectory.id}
///       subnetIds:
///         - ${exampleC.id}
///         - ${exampleD.id}
///       tags:
///         Example: true
///       samlProperties:
///         userAccessUrl: https://sso.example.com/
///         status: ENABLED
///       selfServicePermissions:
///         changeComputeType: true
///         increaseVolumeSize: true
///         rebuildWorkspace: true
///         restartWorkspace: true
///         switchRunningMode: true
///       workspaceAccessProperties:
///         deviceTypeAndroid: ALLOW
///         deviceTypeChromeos: ALLOW
///         deviceTypeIos: ALLOW
///         deviceTypeLinux: DENY
///         deviceTypeOsx: ALLOW
///         deviceTypeWeb: DENY
///         deviceTypeWindows: DENY
///         deviceTypeZeroclient: DENY
///       workspaceCreationProperties:
///         customSecurityGroupId: ${exampleAwsSecurityGroup.id}
///         defaultOu: OU=AWS,DC=Workgroup,DC=Example,DC=com
///         enableInternetAccess: true
///         enableMaintenanceMode: true
///         userEnabledAsLocalAdministrator: true
///     options:
///       dependsOn:
///         - ${workspacesDefaultServiceAccess}
///         - ${workspacesDefaultSelfServiceAccess}
///   exampleDirectory:
///     type: aws:directoryservice:Directory
///     name: example
///     properties:
///       name: corp.example.com
///       password: '#S1ncerely'
///       size: Small
///       vpcSettings:
///         vpcId: ${exampleVpc.id}
///         subnetIds:
///           - ${exampleA.id}
///           - ${exampleB.id}
///   workspacesDefault:
///     type: aws:iam:Role
///     name: workspaces_default
///     properties:
///       name: workspaces_DefaultRole
///       assumeRolePolicy: ${workspaces.json}
///   workspacesDefaultServiceAccess:
///     type: aws:iam:RolePolicyAttachment
///     name: workspaces_default_service_access
///     properties:
///       role: ${workspacesDefault.name}
///       policyArn: arn:aws:iam::aws:policy/AmazonWorkSpacesServiceAccess
///   workspacesDefaultSelfServiceAccess:
///     type: aws:iam:RolePolicyAttachment
///     name: workspaces_default_self_service_access
///     properties:
///       role: ${workspacesDefault.name}
///       policyArn: arn:aws:iam::aws:policy/AmazonWorkSpacesSelfServiceAccess
///   exampleVpc:
///     type: aws:ec2:Vpc
///     name: example
///     properties:
///       cidrBlock: 10.0.0.0/16
///   exampleA:
///     type: aws:ec2:Subnet
///     name: example_a
///     properties:
///       vpcId: ${exampleVpc.id}
///       availabilityZone: us-east-1a
///       cidrBlock: 10.0.0.0/24
///   exampleB:
///     type: aws:ec2:Subnet
///     name: example_b
///     properties:
///       vpcId: ${exampleVpc.id}
///       availabilityZone: us-east-1b
///       cidrBlock: 10.0.1.0/24
///   exampleC:
///     type: aws:ec2:Subnet
///     name: example_c
///     properties:
///       vpcId: ${exampleVpc.id}
///       availabilityZone: us-east-1c
///       cidrBlock: 10.0.2.0/24
///   exampleD:
///     type: aws:ec2:Subnet
///     name: example_d
///     properties:
///       vpcId: ${exampleVpc.id}
///       availabilityZone: us-east-1d
///       cidrBlock: 10.0.3.0/24
/// variables:
///   workspaces:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - actions:
///               - sts:AssumeRole
///             principals:
///               - type: Service
///                 identifiers:
///                   - workspaces.amazonaws.com
/// ```
///
/// ### IP Groups
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = directory::create(
///         "example",
///         DirectoryArgs::builder()
///             .directory_id("${exampleAwsDirectoryServiceDirectory.id}")
///             .ip_group_ids(vec!["${exampleIpGroup.id}",])
///             .build_struct(),
///     );
///     let exampleIpGroup = ip_group::create(
///         "exampleIpGroup",
///         IpGroupArgs::builder().name("example").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Workspaces directory using the directory ID. For example:
///
/// ```sh
/// $ pulumi import aws:workspaces/directory:Directory main d-4444444444
/// ```
pub mod directory {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DirectoryArgs {
        /// The directory identifier for registration in WorkSpaces service.
        #[builder(into)]
        pub directory_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The identifiers of the IP access control groups associated with the directory.
        #[builder(into, default)]
        pub ip_group_ids: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Configuration of SAML authentication integration. Defined below.
        #[builder(into, default)]
        pub saml_properties: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::workspaces::DirectorySamlProperties>,
        >,
        /// Permissions to enable or disable self-service capabilities. Defined below.
        #[builder(into, default)]
        pub self_service_permissions: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::workspaces::DirectorySelfServicePermissions>,
        >,
        /// The identifiers of the subnets where the directory resides.
        #[builder(into, default)]
        pub subnet_ids: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// A map of tags assigned to the WorkSpaces directory. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies which devices and operating systems users can use to access their WorkSpaces. Defined below.
        #[builder(into, default)]
        pub workspace_access_properties: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::workspaces::DirectoryWorkspaceAccessProperties>,
        >,
        /// Default properties that are used for creating WorkSpaces. Defined below.
        #[builder(into, default)]
        pub workspace_creation_properties: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::workspaces::DirectoryWorkspaceCreationProperties>,
        >,
    }
    #[allow(dead_code)]
    pub struct DirectoryResult {
        /// The directory alias.
        pub alias: pulumi_wasm_rust::Output<String>,
        /// The user name for the service account.
        pub customer_user_name: pulumi_wasm_rust::Output<String>,
        /// The directory identifier for registration in WorkSpaces service.
        pub directory_id: pulumi_wasm_rust::Output<String>,
        /// The name of the directory.
        pub directory_name: pulumi_wasm_rust::Output<String>,
        /// The directory type.
        pub directory_type: pulumi_wasm_rust::Output<String>,
        /// The IP addresses of the DNS servers for the directory.
        pub dns_ip_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// The identifier of the IAM role. This is the role that allows Amazon WorkSpaces to make calls to other services, such as Amazon EC2, on your behalf.
        pub iam_role_id: pulumi_wasm_rust::Output<String>,
        /// The identifiers of the IP access control groups associated with the directory.
        pub ip_group_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// The registration code for the directory. This is the code that users enter in their Amazon WorkSpaces client application to connect to the directory.
        pub registration_code: pulumi_wasm_rust::Output<String>,
        /// Configuration of SAML authentication integration. Defined below.
        pub saml_properties: pulumi_wasm_rust::Output<
            super::super::types::workspaces::DirectorySamlProperties,
        >,
        /// Permissions to enable or disable self-service capabilities. Defined below.
        pub self_service_permissions: pulumi_wasm_rust::Output<
            super::super::types::workspaces::DirectorySelfServicePermissions,
        >,
        /// The identifiers of the subnets where the directory resides.
        pub subnet_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// A map of tags assigned to the WorkSpaces directory. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Specifies which devices and operating systems users can use to access their WorkSpaces. Defined below.
        pub workspace_access_properties: pulumi_wasm_rust::Output<
            super::super::types::workspaces::DirectoryWorkspaceAccessProperties,
        >,
        /// Default properties that are used for creating WorkSpaces. Defined below.
        pub workspace_creation_properties: pulumi_wasm_rust::Output<
            super::super::types::workspaces::DirectoryWorkspaceCreationProperties,
        >,
        /// The identifier of the security group that is assigned to new WorkSpaces.
        pub workspace_security_group_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: DirectoryArgs,
    ) -> DirectoryResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let directory_id_binding = args.directory_id.get_output(context).get_inner();
        let ip_group_ids_binding = args.ip_group_ids.get_output(context).get_inner();
        let saml_properties_binding = args
            .saml_properties
            .get_output(context)
            .get_inner();
        let self_service_permissions_binding = args
            .self_service_permissions
            .get_output(context)
            .get_inner();
        let subnet_ids_binding = args.subnet_ids.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let workspace_access_properties_binding = args
            .workspace_access_properties
            .get_output(context)
            .get_inner();
        let workspace_creation_properties_binding = args
            .workspace_creation_properties
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:workspaces/directory:Directory".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "directoryId".into(),
                    value: &directory_id_binding,
                },
                register_interface::ObjectField {
                    name: "ipGroupIds".into(),
                    value: &ip_group_ids_binding,
                },
                register_interface::ObjectField {
                    name: "samlProperties".into(),
                    value: &saml_properties_binding,
                },
                register_interface::ObjectField {
                    name: "selfServicePermissions".into(),
                    value: &self_service_permissions_binding,
                },
                register_interface::ObjectField {
                    name: "subnetIds".into(),
                    value: &subnet_ids_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "workspaceAccessProperties".into(),
                    value: &workspace_access_properties_binding,
                },
                register_interface::ObjectField {
                    name: "workspaceCreationProperties".into(),
                    value: &workspace_creation_properties_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "alias".into(),
                },
                register_interface::ResultField {
                    name: "customerUserName".into(),
                },
                register_interface::ResultField {
                    name: "directoryId".into(),
                },
                register_interface::ResultField {
                    name: "directoryName".into(),
                },
                register_interface::ResultField {
                    name: "directoryType".into(),
                },
                register_interface::ResultField {
                    name: "dnsIpAddresses".into(),
                },
                register_interface::ResultField {
                    name: "iamRoleId".into(),
                },
                register_interface::ResultField {
                    name: "ipGroupIds".into(),
                },
                register_interface::ResultField {
                    name: "registrationCode".into(),
                },
                register_interface::ResultField {
                    name: "samlProperties".into(),
                },
                register_interface::ResultField {
                    name: "selfServicePermissions".into(),
                },
                register_interface::ResultField {
                    name: "subnetIds".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "workspaceAccessProperties".into(),
                },
                register_interface::ResultField {
                    name: "workspaceCreationProperties".into(),
                },
                register_interface::ResultField {
                    name: "workspaceSecurityGroupId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DirectoryResult {
            alias: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alias").unwrap(),
            ),
            customer_user_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerUserName").unwrap(),
            ),
            directory_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("directoryId").unwrap(),
            ),
            directory_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("directoryName").unwrap(),
            ),
            directory_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("directoryType").unwrap(),
            ),
            dns_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsIpAddresses").unwrap(),
            ),
            iam_role_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iamRoleId").unwrap(),
            ),
            ip_group_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipGroupIds").unwrap(),
            ),
            registration_code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("registrationCode").unwrap(),
            ),
            saml_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("samlProperties").unwrap(),
            ),
            self_service_permissions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfServicePermissions").unwrap(),
            ),
            subnet_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetIds").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            workspace_access_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workspaceAccessProperties").unwrap(),
            ),
            workspace_creation_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workspaceCreationProperties").unwrap(),
            ),
            workspace_security_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workspaceSecurityGroupId").unwrap(),
            ),
        }
    }
}
