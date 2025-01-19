/// Resource for managing a AWS Shield Proactive Engagement.
/// Proactive engagement authorizes the Shield Response Team (SRT) to use email and phone to notify contacts about escalations to the SRT and to initiate proactive customer support.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:shield:ProactiveEngagement
///     properties:
///       enabled: true
///       emergencyContacts:
///         - contactNotes: Notes
///           emailAddress: contact1@example.com
///           phoneNumber: '+12358132134'
///         - contactNotes: Notes 2
///           emailAddress: contact2@example.com
///           phoneNumber: '+12358132134'
///     options:
///       dependsOn:
///         - ${exampleDrtAccessRoleArnAssociation}
///   exampleRole:
///     type: aws:iam:Role
///     name: example
///     properties:
///       name: example-role
///       assumeRolePolicy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Sid: ""
///               Effect: Allow
///               Principal:
///                 Service: drt.shield.amazonaws.com
///               Action: sts:AssumeRole
///   exampleRolePolicyAttachment:
///     type: aws:iam:RolePolicyAttachment
///     name: example
///     properties:
///       role: ${exampleRole.name}
///       policyArn: arn:aws:iam::aws:policy/service-role/AWSShieldDRTAccessPolicy
///   exampleDrtAccessRoleArnAssociation:
///     type: aws:shield:DrtAccessRoleArnAssociation
///     name: example
///     properties:
///       roleArn: ${exampleRole.arn}
///   exampleProtectionGroup:
///     type: aws:shield:ProtectionGroup
///     name: example
///     properties:
///       protectionGroupId: example
///       aggregation: MAX
///       pattern: ALL
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Shield proactive engagement using the AWS account ID. For example:
///
/// ```sh
/// $ pulumi import aws:shield/proactiveEngagement:ProactiveEngagement example 123456789012
/// ```
pub mod proactive_engagement {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProactiveEngagementArgs {
        /// One or more emergency contacts. You must provide at least one phone number in the emergency contact list. See `emergency_contacts`.
        #[builder(into, default)]
        pub emergency_contacts: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::shield::ProactiveEngagementEmergencyContact>>,
        >,
        /// Boolean value indicating if Proactive Engagement should be enabled or not.
        #[builder(into)]
        pub enabled: pulumi_wasm_rust::Output<bool>,
    }
    #[allow(dead_code)]
    pub struct ProactiveEngagementResult {
        /// One or more emergency contacts. You must provide at least one phone number in the emergency contact list. See `emergency_contacts`.
        pub emergency_contacts: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::shield::ProactiveEngagementEmergencyContact>>,
        >,
        /// Boolean value indicating if Proactive Engagement should be enabled or not.
        pub enabled: pulumi_wasm_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ProactiveEngagementArgs,
    ) -> ProactiveEngagementResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let emergency_contacts_binding = args.emergency_contacts.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:shield/proactiveEngagement:ProactiveEngagement".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "emergencyContacts".into(),
                    value: &emergency_contacts_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "emergencyContacts".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ProactiveEngagementResult {
            emergency_contacts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("emergencyContacts").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
        }
    }
}
