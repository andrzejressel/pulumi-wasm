/// Manages a Kinesis Analytics v2 Application Snapshot.
/// Snapshots are the AWS implementation of [Flink Savepoints](https://ci.apache.org/projects/flink/flink-docs-release-1.11/ops/state/savepoints.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = application_snapshot::create(
///         "example",
///         ApplicationSnapshotArgs::builder()
///             .application_name("${exampleAwsKinesisanalyticsv2Application.name}")
///             .snapshot_name("example-snapshot")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_kinesisanalyticsv2_application` using `application_name` together with `snapshot_name`. For example:
///
/// ```sh
/// $ pulumi import aws:kinesisanalyticsv2/applicationSnapshot:ApplicationSnapshot example example-application/example-snapshot
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod application_snapshot {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationSnapshotArgs {
        /// The name of an existing  Kinesis Analytics v2 Application. Note that the application must be running for a snapshot to be created.
        #[builder(into)]
        pub application_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the application snapshot.
        #[builder(into)]
        pub snapshot_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ApplicationSnapshotResult {
        /// The name of an existing  Kinesis Analytics v2 Application. Note that the application must be running for a snapshot to be created.
        pub application_name: pulumi_gestalt_rust::Output<String>,
        /// The current application version ID when the snapshot was created.
        pub application_version_id: pulumi_gestalt_rust::Output<i32>,
        /// The timestamp of the application snapshot.
        pub snapshot_creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// The name of the application snapshot.
        pub snapshot_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApplicationSnapshotArgs,
    ) -> ApplicationSnapshotResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let application_name_binding = args.application_name.get_output(context);
        let snapshot_name_binding = args.snapshot_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:kinesisanalyticsv2/applicationSnapshot:ApplicationSnapshot"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationName".into(),
                    value: application_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "snapshotName".into(),
                    value: snapshot_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApplicationSnapshotResult {
            application_name: o.get_field("applicationName"),
            application_version_id: o.get_field("applicationVersionId"),
            snapshot_creation_timestamp: o.get_field("snapshotCreationTimestamp"),
            snapshot_name: o.get_field("snapshotName"),
        }
    }
}
