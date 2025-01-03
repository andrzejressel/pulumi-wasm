/// Provides a resource for managing a replication set in AWS Systems Manager Incident Manager.
///
/// > **NOTE:** Deleting a replication set also deletes all Incident Manager related data including response plans, incident records, contacts and escalation plans.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// Create a replication set.
///
/// ```yaml
/// resources:
///   replicationSetName:
///     type: aws:ssmincidents:ReplicationSet
///     properties:
///       regions:
///         - name: us-west-2
///       tags:
///         exampleTag: exampleValue
/// ```
///
/// Add a Region to a replication set. (You can add only one Region at a time.)
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let replicationSetName = replication_set::create(
///         "replicationSetName",
///         ReplicationSetArgs::builder()
///             .regions(
///                 vec![
///                     ReplicationSetRegion::builder().name("us-west-2").build_struct(),
///                     ReplicationSetRegion::builder().name("ap-southeast-2")
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// Delete a Region from a replication set. (You can delete only one Region at a time.)
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let replicationSetName = replication_set::create(
///         "replicationSetName",
///         ReplicationSetArgs::builder()
///             .regions(
///                 vec![ReplicationSetRegion::builder().name("us-west-2").build_struct(),],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Basic Usage with an AWS Customer Managed Key
///
/// Create a replication set with an AWS Key Management Service (AWS KMS) customer manager key:
///
/// ```yaml
/// resources:
///   exampleKey:
///     type: aws:kms:Key
///     name: example_key
///   replicationSetName:
///     type: aws:ssmincidents:ReplicationSet
///     properties:
///       regions:
///         - name: us-west-2
///           kmsKeyArn: ${exampleKey.arn}
///       tags:
///         exampleTag: exampleValue
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import an Incident Manager replication. For example:
///
/// ```sh
/// $ pulumi import aws:ssmincidents/replicationSet:ReplicationSet replicationSetName import
/// ```
pub mod replication_set {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReplicationSetArgs {
        #[builder(into)]
        pub regions: pulumi_wasm_rust::Output<
            Vec<super::super::types::ssmincidents::ReplicationSetRegion>,
        >,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ReplicationSetResult {
        /// The ARN of the replication set.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The ARN of the user who created the replication set.
        pub created_by: pulumi_wasm_rust::Output<String>,
        /// If `true`, the last region in a replication set cannot be deleted.
        pub deletion_protected: pulumi_wasm_rust::Output<bool>,
        /// A timestamp showing when the replication set was last modified.
        pub last_modified_by: pulumi_wasm_rust::Output<String>,
        pub regions: pulumi_wasm_rust::Output<
            Vec<super::super::types::ssmincidents::ReplicationSetRegion>,
        >,
        /// The current status of the Region.
        /// * Valid Values: `ACTIVE` | `CREATING` | `UPDATING` | `DELETING` | `FAILED`
        pub status: pulumi_wasm_rust::Output<String>,
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
    pub fn create(name: &str, args: ReplicationSetArgs) -> ReplicationSetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let regions_binding = args.regions.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ssmincidents/replicationSet:ReplicationSet".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "regions".into(),
                    value: &regions_binding,
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
                    name: "createdBy".into(),
                },
                register_interface::ResultField {
                    name: "deletionProtected".into(),
                },
                register_interface::ResultField {
                    name: "lastModifiedBy".into(),
                },
                register_interface::ResultField {
                    name: "regions".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ReplicationSetResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            created_by: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdBy").unwrap(),
            ),
            deletion_protected: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionProtected").unwrap(),
            ),
            last_modified_by: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastModifiedBy").unwrap(),
            ),
            regions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("regions").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
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
