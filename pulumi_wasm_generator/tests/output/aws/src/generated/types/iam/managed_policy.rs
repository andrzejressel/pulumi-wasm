#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum ManagedPolicy {
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/APIGatewayServiceRolePolicy")]
    APIGatewayServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSAccountActivityAccess")]
    AWSAccountActivityAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSAccountManagementFullAccess")]
    AWSAccountManagementFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSAccountManagementReadOnlyAccess")]
    AWSAccountManagementReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSAccountUsageReportAccess")]
    AWSAccountUsageReportAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSAgentlessDiscoveryService")]
    AWSAgentlessDiscoveryService,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSAppFabricFullAccess")]
    AWSAppFabricFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSAppFabricReadOnlyAccess")]
    AWSAppFabricReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSAppFabricServiceRolePolicy")]
    AWSAppFabricServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSAppMeshEnvoyAccess")]
    AWSAppMeshEnvoyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSAppMeshFullAccess")]
    AWSAppMeshFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSAppMeshPreviewEnvoyAccess")]
    AWSAppMeshPreviewEnvoyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSAppMeshPreviewServiceRolePolicy")]
    AWSAppMeshPreviewServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSAppMeshReadOnly")]
    AWSAppMeshReadOnly,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSAppMeshServiceRolePolicy")]
    AWSAppMeshServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSAppRunnerFullAccess")]
    AWSAppRunnerFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSAppRunnerReadOnlyAccess")]
    AWSAppRunnerReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSAppRunnerServicePolicyForECRAccess")]
    AWSAppRunnerServicePolicyForECRAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSAppSyncAdministrator")]
    AWSAppSyncAdministrator,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSAppSyncInvokeFullAccess")]
    AWSAppSyncInvokeFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSAppSyncPushToCloudWatchLogs")]
    AWSAppSyncPushToCloudWatchLogs,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSAppSyncSchemaAuthor")]
    AWSAppSyncSchemaAuthor,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSAppSyncServiceRolePolicy")]
    AWSAppSyncServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSApplicationAutoScalingCustomResourcePolicy")]
    AWSApplicationAutoScalingCustomResourcePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSApplicationAutoscalingAppStreamFleetPolicy")]
    AWSApplicationAutoscalingAppStreamFleetPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSApplicationAutoscalingCassandraTablePolicy")]
    AWSApplicationAutoscalingCassandraTablePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSApplicationAutoscalingComprehendEndpointPolicy")]
    AWSApplicationAutoscalingComprehendEndpointPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSApplicationAutoscalingDynamoDBTablePolicy")]
    AWSApplicationAutoscalingDynamoDBTablePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSApplicationAutoscalingEC2SpotFleetRequestPolicy")]
    AWSApplicationAutoscalingEC2SpotFleetRequestPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSApplicationAutoscalingECSServicePolicy")]
    AWSApplicationAutoscalingECSServicePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSApplicationAutoscalingEMRInstanceGroupPolicy")]
    AWSApplicationAutoscalingEMRInstanceGroupPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSApplicationAutoscalingElastiCacheRGPolicy")]
    AWSApplicationAutoscalingElastiCacheRGPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSApplicationAutoscalingKafkaClusterPolicy")]
    AWSApplicationAutoscalingKafkaClusterPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSApplicationAutoscalingLambdaConcurrencyPolicy")]
    AWSApplicationAutoscalingLambdaConcurrencyPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSApplicationAutoscalingNeptuneClusterPolicy")]
    AWSApplicationAutoscalingNeptuneClusterPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSApplicationAutoscalingRDSClusterPolicy")]
    AWSApplicationAutoscalingRDSClusterPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSApplicationAutoscalingSageMakerEndpointPolicy")]
    AWSApplicationAutoscalingSageMakerEndpointPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSApplicationDiscoveryAgentAccess")]
    AWSApplicationDiscoveryAgentAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSApplicationDiscoveryAgentlessCollectorAccess")]
    AWSApplicationDiscoveryAgentlessCollectorAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSApplicationDiscoveryServiceFullAccess")]
    AWSApplicationDiscoveryServiceFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSApplicationMigrationAgentInstallationPolicy")]
    AWSApplicationMigrationAgentInstallationPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSApplicationMigrationAgentPolicy")]
    AWSApplicationMigrationAgentPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSApplicationMigrationAgentPolicy_v2")]
    AWSApplicationMigrationAgentPolicy_v2,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSApplicationMigrationConversionServerPolicy")]
    AWSApplicationMigrationConversionServerPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSApplicationMigrationEC2Access")]
    AWSApplicationMigrationEC2Access,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSApplicationMigrationFullAccess")]
    AWSApplicationMigrationFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSApplicationMigrationMGHAccess")]
    AWSApplicationMigrationMGHAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSApplicationMigrationReadOnlyAccess")]
    AWSApplicationMigrationReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSApplicationMigrationReplicationServerPolicy")]
    AWSApplicationMigrationReplicationServerPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSApplicationMigrationSSMAccess")]
    AWSApplicationMigrationSSMAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSApplicationMigrationServiceEc2InstancePolicy")]
    AWSApplicationMigrationServiceEc2InstancePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSApplicationMigrationServiceRolePolicy")]
    AWSApplicationMigrationServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSApplicationMigrationVCenterClientPolicy")]
    AWSApplicationMigrationVCenterClientPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSArtifactAccountSync")]
    AWSArtifactAccountSync,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSArtifactReportsReadOnlyAccess")]
    AWSArtifactReportsReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSArtifactServiceRolePolicy")]
    AWSArtifactServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSAuditManagerAdministratorAccess")]
    AWSAuditManagerAdministratorAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSAuditManagerServiceRolePolicy")]
    AWSAuditManagerServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSAutoScalingPlansEC2AutoScalingPolicy")]
    AWSAutoScalingPlansEC2AutoScalingPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSBackupAuditAccess")]
    AWSBackupAuditAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSBackupDataTransferAccess")]
    AWSBackupDataTransferAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSBackupFullAccess")]
    AWSBackupFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSBackupGatewayServiceRolePolicyForVirtualMachineMetadataSync")]
    AWSBackupGatewayServiceRolePolicyForVirtualMachineMetadataSync,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSBackupOperatorAccess")]
    AWSBackupOperatorAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSBackupOrganizationAdminAccess")]
    AWSBackupOrganizationAdminAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSBackupRestoreAccessForSAPHANA")]
    AWSBackupRestoreAccessForSAPHANA,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSBackupServiceLinkedRolePolicyForBackup")]
    AWSBackupServiceLinkedRolePolicyForBackup,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSBackupServiceLinkedRolePolicyForBackupTest")]
    AWSBackupServiceLinkedRolePolicyForBackupTest,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSBackupServiceRolePolicyForBackup")]
    AWSBackupServiceRolePolicyForBackup,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSBackupServiceRolePolicyForRestores")]
    AWSBackupServiceRolePolicyForRestores,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSBackupServiceRolePolicyForS3Backup")]
    AWSBackupServiceRolePolicyForS3Backup,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSBackupServiceRolePolicyForS3Restore")]
    AWSBackupServiceRolePolicyForS3Restore,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSBatchFullAccess")]
    AWSBatchFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSBatchServiceEventTargetRole")]
    AWSBatchServiceEventTargetRole,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSBatchServiceRole")]
    AWSBatchServiceRole,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSBillingConductorFullAccess")]
    AWSBillingConductorFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSBillingConductorReadOnlyAccess")]
    AWSBillingConductorReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSBillingReadOnlyAccess")]
    AWSBillingReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSBudgetsActionsWithAWSResourceControlAccess")]
    AWSBudgetsActionsWithAWSResourceControlAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSBudgetsActions_RolePolicyForResourceAdministrationWithSSM")]
    AWSBudgetsActions_RolePolicyForResourceAdministrationWithSSM,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSBudgetsReadOnlyAccess")]
    AWSBudgetsReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSBugBustFullAccess")]
    AWSBugBustFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSBugBustPlayerAccess")]
    AWSBugBustPlayerAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSBugBustServiceRolePolicy")]
    AWSBugBustServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCertificateManagerFullAccess")]
    AWSCertificateManagerFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCertificateManagerPrivateCAAuditor")]
    AWSCertificateManagerPrivateCAAuditor,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCertificateManagerPrivateCAFullAccess")]
    AWSCertificateManagerPrivateCAFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCertificateManagerPrivateCAPrivilegedUser")]
    AWSCertificateManagerPrivateCAPrivilegedUser,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCertificateManagerPrivateCAReadOnly")]
    AWSCertificateManagerPrivateCAReadOnly,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCertificateManagerPrivateCAUser")]
    AWSCertificateManagerPrivateCAUser,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCertificateManagerReadOnly")]
    AWSCertificateManagerReadOnly,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSChatbotServiceLinkedRolePolicy")]
    AWSChatbotServiceLinkedRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCleanRoomsFullAccess")]
    AWSCleanRoomsFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCleanRoomsFullAccessNoQuerying")]
    AWSCleanRoomsFullAccessNoQuerying,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCleanRoomsMLFullAccess")]
    AWSCleanRoomsMLFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCleanRoomsMLReadOnlyAccess")]
    AWSCleanRoomsMLReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCleanRoomsReadOnlyAccess")]
    AWSCleanRoomsReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCloud9Administrator")]
    AWSCloud9Administrator,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCloud9EnvironmentMember")]
    AWSCloud9EnvironmentMember,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCloud9SSMInstanceProfile")]
    AWSCloud9SSMInstanceProfile,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSCloud9ServiceRolePolicy")]
    AWSCloud9ServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCloud9User")]
    AWSCloud9User,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCloudFormationFullAccess")]
    AWSCloudFormationFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCloudFormationReadOnlyAccess")]
    AWSCloudFormationReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSCloudFrontLogger")]
    AWSCloudFrontLogger,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCloudHSMFullAccess")]
    AWSCloudHSMFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCloudHSMReadOnlyAccess")]
    AWSCloudHSMReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSCloudHSMRole")]
    AWSCloudHSMRole,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCloudTrailFullAccess")]
    AWSCloudTrailFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCloudTrailReadOnlyAccess")]
    AWSCloudTrailReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCloudWatchLambdaInsightsExecutionRolePolicy")]
    AWSCloudWatchLambdaInsightsExecutionRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCloudMapDiscoverInstanceAccess")]
    AWSCloudMapDiscoverInstanceAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCloudMapFullAccess")]
    AWSCloudMapFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCloudMapReadOnlyAccess")]
    AWSCloudMapReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCloudMapRegisterInstanceAccess")]
    AWSCloudMapRegisterInstanceAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCloudShellFullAccess")]
    AWSCloudShellFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCloudTrail_FullAccess")]
    CloudTrail_FullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCloudTrail_ReadOnlyAccess")]
    CloudTrail_ReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSCloudWatchAlarms_ActionSSMIncidentsServiceRolePolicy")]
    AWSCloudWatchAlarms_ActionSSMIncidentsServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCodeArtifactAdminAccess")]
    AWSCodeArtifactAdminAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCodeArtifactReadOnlyAccess")]
    AWSCodeArtifactReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCodeBuildAdminAccess")]
    AWSCodeBuildAdminAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCodeBuildDeveloperAccess")]
    AWSCodeBuildDeveloperAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCodeBuildReadOnlyAccess")]
    AWSCodeBuildReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCodeCommitFullAccess")]
    AWSCodeCommitFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCodeCommitPowerUser")]
    AWSCodeCommitPowerUser,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCodeCommitReadOnly")]
    AWSCodeCommitReadOnly,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCodeDeployDeployerAccess")]
    AWSCodeDeployDeployerAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCodeDeployFullAccess")]
    AWSCodeDeployFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCodeDeployReadOnlyAccess")]
    AWSCodeDeployReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSCodeDeployRole")]
    AWSCodeDeployRole,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSCodeDeployRoleForCloudFormation")]
    AWSCodeDeployRoleForCloudFormation,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCodeDeployRoleForECS")]
    AWSCodeDeployRoleForECS,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCodeDeployRoleForECSLimited")]
    AWSCodeDeployRoleForECSLimited,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSCodeDeployRoleForLambda")]
    AWSCodeDeployRoleForLambda,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSCodeDeployRoleForLambdaLimited")]
    AWSCodeDeployRoleForLambdaLimited,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCodePipelineApproverAccess")]
    AWSCodePipelineApproverAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCodePipelineCustomActionAccess")]
    AWSCodePipelineCustomActionAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCodePipelineFullAccess")]
    AWSCodePipelineFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCodePipelineReadOnlyAccess")]
    AWSCodePipelineReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCodePipeline_FullAccess")]
    CodePipeline_FullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCodePipeline_ReadOnlyAccess")]
    CodePipeline_ReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCodeStarFullAccess")]
    AWSCodeStarFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSCodeStarNotificationsServiceRolePolicy")]
    AWSCodeStarNotificationsServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSCodeStarServiceRole")]
    AWSCodeStarServiceRole,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWS_ConfigRole")]
    AWS_ConfigRole,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSConfigRole")]
    AWSConfigRole,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCompromisedKeyQuarantine")]
    AWSCompromisedKeyQuarantine,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSCompromisedKeyQuarantineV2")]
    AWSCompromisedKeyQuarantineV2,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSConfigMultiAccountSetupPolicy")]
    AWSConfigMultiAccountSetupPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSConfigRemediationServiceRolePolicy")]
    AWSConfigRemediationServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSConfigRoleForOrganizations")]
    AWSConfigRoleForOrganizations,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSConfigRulesExecutionRole")]
    AWSConfigRulesExecutionRole,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSConfigServiceRolePolicy")]
    AWSConfigServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSConfigUserAccess")]
    AWSConfigUserAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSConnector")]
    AWSConnector,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSControlTowerAccountServiceRolePolicy")]
    AWSControlTowerAccountServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSControlTowerServiceRolePolicy")]
    AWSControlTowerServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSCostAndUsageReportAutomationPolicy")]
    AWSCostAndUsageReportAutomationPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSDMSFleetAdvisorServiceRolePolicy")]
    AWSDMSFleetAdvisorServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSDMSServerlessServiceRolePolicy")]
    AWSDMSServerlessServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSDataExchangeFullAccess")]
    AWSDataExchangeFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSDataExchangeProviderFullAccess")]
    AWSDataExchangeProviderFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSDataExchangeReadOnly")]
    AWSDataExchangeReadOnly,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSDataExchangeSubscriberFullAccess")]
    AWSDataExchangeSubscriberFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSDataLifecycleManagerSSMFullAccess")]
    AWSDataLifecycleManagerSSMFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSDataLifecycleManagerServiceRole")]
    AWSDataLifecycleManagerServiceRole,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSDataLifecycleManagerServiceRoleForAMIManagement")]
    AWSDataLifecycleManagerServiceRoleForAMIManagement,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSDataPipelineRole")]
    AWSDataPipelineRole,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSDataPipeline_FullAccess")]
    AWSDataPipeline_FullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSDataPipeline_PowerUser")]
    AWSDataPipeline_PowerUser,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSDataSyncDiscoveryServiceRolePolicy")]
    AWSDataSyncDiscoveryServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSDataSyncFullAccess")]
    AWSDataSyncFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSDataSyncReadOnlyAccess")]
    AWSDataSyncReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSDeepLensLambdaFunctionAccessPolicy")]
    AWSDeepLensLambdaFunctionAccessPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSDeepLensServiceRolePolicy")]
    AWSDeepLensServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSDeepRacerAccountAdminAccess")]
    AWSDeepRacerAccountAdminAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSDeepRacerCloudFormationAccessPolicy")]
    AWSDeepRacerCloudFormationAccessPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSDeepRacerDefaultMultiUserAccess")]
    AWSDeepRacerDefaultMultiUserAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSDeepRacerFullAccess")]
    AWSDeepRacerFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSDeepRacerRoboMakerAccessPolicy")]
    AWSDeepRacerRoboMakerAccessPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSDeepRacerServiceRolePolicy")]
    AWSDeepRacerServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSDenyAll")]
    AWSDenyAll,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSDeviceFarmFullAccess")]
    AWSDeviceFarmFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSDeviceFarmServiceRolePolicy")]
    AWSDeviceFarmServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSDeviceFarmTestGridServiceRolePolicy")]
    AWSDeviceFarmTestGridServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSDirectConnectFullAccess")]
    AWSDirectConnectFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSDirectConnectReadOnlyAccess")]
    AWSDirectConnectReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSDirectConnectServiceRolePolicy")]
    AWSDirectConnectServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSDirectoryServiceFullAccess")]
    AWSDirectoryServiceFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSDirectoryServiceReadOnlyAccess")]
    AWSDirectoryServiceReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSDiscoveryContinuousExportFirehosePolicy")]
    AWSDiscoveryContinuousExportFirehosePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSEC2CapacityReservationFleetRolePolicy")]
    AWSEC2CapacityReservationFleetRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSEC2FleetServiceRolePolicy")]
    AWSEC2FleetServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSEC2SpotFleetServiceRolePolicy")]
    AWSEC2SpotFleetServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSEC2SpotServiceRolePolicy")]
    AWSEC2SpotServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSECRPullThroughCache_ServiceRolePolicy")]
    AWSECRPullThroughCache_ServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSElasticBeanstalkCustomPlatformforEC2Role")]
    AWSElasticBeanstalkCustomPlatformforEC2Role,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSElasticBeanstalkEnhancedHealth")]
    AWSElasticBeanstalkEnhancedHealth,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSElasticBeanstalkFullAccess")]
    AWSElasticBeanstalkFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSElasticBeanstalkMaintenance")]
    AWSElasticBeanstalkMaintenance,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSElasticBeanstalkManagedUpdatesCustomerRolePolicy")]
    AWSElasticBeanstalkManagedUpdatesCustomerRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSElasticBeanstalkManagedUpdatesServiceRolePolicy")]
    AWSElasticBeanstalkManagedUpdatesServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSElasticBeanstalkMulticontainerDocker")]
    AWSElasticBeanstalkMulticontainerDocker,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSElasticBeanstalkReadOnlyAccess")]
    AWSElasticBeanstalkReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSElasticBeanstalkReadOnly")]
    AWSElasticBeanstalkReadOnly,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSElasticBeanstalkRoleCWL")]
    AWSElasticBeanstalkRoleCWL,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSElasticBeanstalkRoleCore")]
    AWSElasticBeanstalkRoleCore,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSElasticBeanstalkRoleECS")]
    AWSElasticBeanstalkRoleECS,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSElasticBeanstalkRoleRDS")]
    AWSElasticBeanstalkRoleRDS,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSElasticBeanstalkRoleSNS")]
    AWSElasticBeanstalkRoleSNS,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSElasticBeanstalkRoleWorkerTier")]
    AWSElasticBeanstalkRoleWorkerTier,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSElasticBeanstalkService")]
    AWSElasticBeanstalkService,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSElasticBeanstalkServiceRolePolicy")]
    AWSElasticBeanstalkServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSElasticBeanstalkWebTier")]
    AWSElasticBeanstalkWebTier,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSElasticBeanstalkWorkerTier")]
    AWSElasticBeanstakWorkerTier,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSElasticBeanstalkWorkerTier")]
    AWSElasticBeanstalkWorkerTier,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSElasticDisasterRecoveryAgentInstallationPolicy")]
    AWSElasticDisasterRecoveryAgentInstallationPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSElasticDisasterRecoveryAgentPolicy")]
    AWSElasticDisasterRecoveryAgentPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSElasticDisasterRecoveryConsoleFullAccess")]
    AWSElasticDisasterRecoveryConsoleFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSElasticDisasterRecoveryConsoleFullAccess_v2")]
    AWSElasticDisasterRecoveryConsoleFullAccess_v2,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSElasticDisasterRecoveryConversionServerPolicy")]
    AWSElasticDisasterRecoveryConversionServerPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSElasticDisasterRecoveryCrossAccountReplicationPolicy")]
    AWSElasticDisasterRecoveryCrossAccountReplicationPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSElasticDisasterRecoveryEc2InstancePolicy")]
    AWSElasticDisasterRecoveryEc2InstancePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSElasticDisasterRecoveryFailbackInstallationPolicy")]
    AWSElasticDisasterRecoveryFailbackInstallationPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSElasticDisasterRecoveryFailbackPolicy")]
    AWSElasticDisasterRecoveryFailbackPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSElasticDisasterRecoveryLaunchActionsPolicy")]
    AWSElasticDisasterRecoveryLaunchActionsPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSElasticDisasterRecoveryNetworkReplicationPolicy")]
    AWSElasticDisasterRecoveryNetworkReplicationPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSElasticDisasterRecoveryReadOnlyAccess")]
    AWSElasticDisasterRecoveryReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSElasticDisasterRecoveryRecoveryInstancePolicy")]
    AWSElasticDisasterRecoveryRecoveryInstancePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSElasticDisasterRecoveryReplicationServerPolicy")]
    AWSElasticDisasterRecoveryReplicationServerPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSElasticDisasterRecoveryServiceRolePolicy")]
    AWSElasticDisasterRecoveryServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSElasticDisasterRecoveryStagingAccountPolicy")]
    AWSElasticDisasterRecoveryStagingAccountPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSElasticDisasterRecoveryStagingAccountPolicy_v2")]
    AWSElasticDisasterRecoveryStagingAccountPolicy_v2,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSElasticLoadBalancingClassicServiceRolePolicy")]
    AWSElasticLoadBalancingClassicServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSElasticLoadBalancingServiceRolePolicy")]
    AWSElasticLoadBalancingServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSElementalMediaConvertFullAccess")]
    AWSElementalMediaConvertFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSElementalMediaConvertReadOnly")]
    AWSElementalMediaConvertReadOnly,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSElementalMediaLiveFullAccess")]
    AWSElementalMediaLiveFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSElementalMediaLiveReadOnly")]
    AWSElementalMediaLiveReadOnly,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSElementalMediaPackageFullAccess")]
    AWSElementalMediaPackageFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSElementalMediaPackageReadOnly")]
    AWSElementalMediaPackageReadOnly,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSElementalMediaPackageV2FullAccess")]
    AWSElementalMediaPackageV2FullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSElementalMediaPackageV2ReadOnly")]
    AWSElementalMediaPackageV2ReadOnly,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSElementalMediaStoreFullAccess")]
    AWSElementalMediaStoreFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSElementalMediaStoreReadOnly")]
    AWSElementalMediaStoreReadOnly,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSElementalMediaTailorFullAccess")]
    AWSElementalMediaTailorFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSElementalMediaTailorReadOnly")]
    AWSElementalMediaTailorReadOnly,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSEnhancedClassicNetworkingMangementPolicy")]
    AWSEnhancedClassicNetworkingMangementPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSEntityResolutionConsoleFullAccess")]
    AWSEntityResolutionConsoleFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSEntityResolutionConsoleReadOnlyAccess")]
    AWSEntityResolutionConsoleReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSFMAdminFullAccess")]
    AWSFMAdminFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSFMAdminReadOnlyAccess")]
    AWSFMAdminReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSFMMemberReadOnlyAccess")]
    AWSFMMemberReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSFaultInjectionSimulatorEC2Access")]
    AWSFaultInjectionSimulatorEC2Access,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSFaultInjectionSimulatorECSAccess")]
    AWSFaultInjectionSimulatorECSAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSFaultInjectionSimulatorEKSAccess")]
    AWSFaultInjectionSimulatorEKSAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSFaultInjectionSimulatorNetworkAccess")]
    AWSFaultInjectionSimulatorNetworkAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSFaultInjectionSimulatorRDSAccess")]
    AWSFaultInjectionSimulatorRDSAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSFaultInjectionSimulatorSSMAccess")]
    AWSFaultInjectionSimulatorSSMAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSFinSpaceServiceRolePolicy")]
    AWSFinSpaceServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSForWordPressPluginPolicy")]
    AWSForWordPressPluginPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSGitSyncServiceRolePolicy")]
    AWSGitSyncServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSGlobalAcceleratorSLRPolicy")]
    AWSGlobalAcceleratorSLRPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSGlueConsoleFullAccess")]
    AWSGlueConsoleFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSGlueConsoleSageMakerNotebookFullAccess")]
    AWSGlueConsoleSageMakerNotebookFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSGlueDataBrewServiceRole")]
    AWSGlueDataBrewServiceRole,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSGlueSchemaRegistryFullAccess")]
    AWSGlueSchemaRegistryFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSGlueSchemaRegistryReadonlyAccess")]
    AWSGlueSchemaRegistryReadonlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSGlueServiceNotebookRole")]
    AWSGlueServiceNotebookRole,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSGlueServiceRole")]
    AWSGlueServiceRole,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSGrafanaAccountAdministrator")]
    AWSGrafanaAccountAdministrator,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSGrafanaConsoleReadOnlyAccess")]
    AWSGrafanaConsoleReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSGrafanaWorkspacePermissionManagement")]
    AWSGrafanaWorkspacePermissionManagement,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSGrafanaWorkspacePermissionManagementV2")]
    AWSGrafanaWorkspacePermissionManagementV2,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSGreengrassFullAccess")]
    AWSGreengrassFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSGreengrassFullAccess")]
    AWSGreengrassFullccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSGreengrassReadOnlyAccess")]
    AWSGreengrassReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSGreengrassResourceAccessRolePolicy")]
    AWSGreengrassResourceAccessRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSGroundStationAgentInstancePolicy")]
    AWSGroundStationAgentInstancePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSHealthFullAccess")]
    AWSHealthFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSHealthImagingFullAccess")]
    AWSHealthImagingFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSHealthImagingReadOnlyAccess")]
    AWSHealthImagingReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSHealth_EventProcessorServiceRolePolicy")]
    AWSHealth_EventProcessorServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSIAMIdentityCenterAllowListForIdentityContext")]
    AWSIAMIdentityCenterAllowListForIdentityContext,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSIPAMServiceRolePolicy")]
    AWSIPAMServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSIQContractServiceRolePolicy")]
    AWSIQContractServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSIQFullAccess")]
    AWSIQFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSIQPermissionServiceRolePolicy")]
    AWSIQPermissionServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSIdentitySyncFullAccess")]
    AWSIdentitySyncFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSIdentitySyncReadOnlyAccess")]
    AWSIdentitySyncReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSImageBuilderFullAccess")]
    AWSImageBuilderFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSImageBuilderReadOnlyAccess")]
    AWSImageBuilderReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSImportExportFullAccess")]
    AWSImportExportFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSImportExportReadOnlyAccess")]
    AWSImportExportReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSIncidentManagerIncidentAccessServiceRolePolicy")]
    AWSIncidentManagerIncidentAccessServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSIncidentManagerResolverAccess")]
    AWSIncidentManagerResolverAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSIncidentManagerServiceRolePolicy")]
    AWSIncidentManagerServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSIoT1ClickFullAccess")]
    AWSIoT1ClickFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSIoT1ClickReadOnlyAccess")]
    AWSIoT1ClickReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSIoTAnalyticsFullAccess")]
    AWSIoTAnalyticsFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSIoTAnalyticsReadOnlyAccess")]
    AWSIoTAnalyticsReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSIoTConfigAccess")]
    AWSIoTConfigAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSIoTConfigReadOnlyAccess")]
    AWSIoTConfigReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSIoTDataAccess")]
    AWSIoTDataAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSIoTDeviceDefenderAddThingsToThingGroupMitigationAction")]
    AWSIoTDeviceDefenderAddThingsToThingGroupMitigationAction,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSIoTDeviceDefenderAudit")]
    AWSIoTDeviceDefenderAudit,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSIoTDeviceDefenderEnableIoTLoggingMitigationAction")]
    AWSIoTDeviceDefenderEnableIoTLoggingMitigationAction,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSIoTDeviceDefenderPublishFindingsToSNSMitigationAction")]
    AWSIoTDeviceDefenderPublishFindingsToSNSMitigationAction,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSIoTDeviceDefenderReplaceDefaultPolicyMitigationAction")]
    AWSIoTDeviceDefenderReplaceDefaultPolicyMitigationAction,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSIoTDeviceDefenderUpdateCACertMitigationAction")]
    AWSIoTDeviceDefenderUpdateCACertMitigationAction,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSIoTDeviceDefenderUpdateDeviceCertMitigationAction")]
    AWSIoTDeviceDefenderUpdateDeviceCertMitigationAction,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSIoTDeviceTesterForFreeRTOSFullAccess")]
    AWSIoTDeviceTesterForFreeRTOSFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSIoTDeviceTesterForGreengrassFullAccess")]
    AWSIoTDeviceTesterForGreengrassFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSIoTEventsFullAccess")]
    AWSIoTEventsFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSIoTEventsReadOnlyAccess")]
    AWSIoTEventsReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSIoTFleetHubFederationAccess")]
    AWSIoTFleetHubFederationAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSIoTFleetwiseServiceRolePolicy")]
    AWSIoTFleetwiseServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSIoTFullAccess")]
    AWSIoTFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSIoTLogging")]
    AWSIoTLogging,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSIoTOTAUpdate")]
    AWSIoTOTAUpdate,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSIoTRuleActions")]
    AWSIoTRuleActions,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSIoTSiteWiseConsoleFullAccess")]
    AWSIoTSiteWiseConsoleFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSIoTSiteWiseFullAccess")]
    AWSIoTSiteWiseFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSIoTSiteWiseMonitorPortalAccess")]
    AWSIoTSiteWiseMonitorPortalAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSIoTSiteWiseMonitorServiceRolePolicy")]
    AWSIoTSiteWiseMonitorServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSIoTSiteWiseReadOnlyAccess")]
    AWSIoTSiteWiseReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSIoTThingsRegistration")]
    AWSIoTThingsRegistration,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSIoTTwinMakerServiceRolePolicy")]
    AWSIoTTwinMakerServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSIoTWirelessDataAccess")]
    AWSIoTWirelessDataAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSIoTWirelessFullAccess")]
    AWSIoTWirelessFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSIoTWirelessFullPublishAccess")]
    AWSIoTWirelessFullPublishAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSIoTWirelessGatewayCertManager")]
    AWSIoTWirelessGatewayCertManager,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSIoTWirelessLogging")]
    AWSIoTWirelessLogging,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSIoTWirelessReadOnlyAccess")]
    AWSIoTWirelessReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSIotRoboRunnerFullAccess")]
    AWSIotRoboRunnerFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSIotRoboRunnerReadOnly")]
    AWSIotRoboRunnerReadOnly,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSIotRoboRunnerServiceRolePolicy")]
    AWSIotRoboRunnerServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSKeyManagementServiceCustomKeyStoresServiceRolePolicy")]
    AWSKeyManagementServiceCustomKeyStoresServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSKeyManagementServiceMultiRegionKeysServiceRolePolicy")]
    AWSKeyManagementServiceMultiRegionKeysServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSKeyManagementServicePowerUser")]
    AWSKeyManagementServicePowerUser,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSLakeFormationCrossAccountManager")]
    AWSLakeFormationCrossAccountManager,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSLakeFormationDataAdmin")]
    AWSLakeFormationDataAdmin,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole")]
    AWSLambdaBasicExecutionRole,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSLambdaDynamoDBExecutionRole")]
    AWSLambdaDynamoDBExecutionRole,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSLambdaENIManagementAccess")]
    AWSLambdaENIManagementAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSLambdaExecute")]
    AWSLambdaExecute,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSLambdaFullAccess")]
    AWSLambdaFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSLambda_FullAccess")]
    LambdaFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSLambdaInvocation-DynamoDB")]
    AWSLambdaInvocationDynamoDB,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSLambdaKinesisExecutionRole")]
    AWSLambdaKinesisExecutionRole,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSLambdaReadOnlyAccess")]
    AWSLambdaReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSLambda_ReadOnlyAccess")]
    LambdaReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSLambdaMSKExecutionRole")]
    AWSLambdaMSKExecutionRole,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSLambdaReplicator")]
    AWSLambdaReplicator,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSLambdaRole")]
    AWSLambdaRole,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSLambdaSQSQueueExecutionRole")]
    AWSLambdaSQSQueueExecutionRole,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole")]
    AWSLambdaVPCAccessExecutionRole,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSLicenseManagerConsumptionPolicy")]
    AWSLicenseManagerConsumptionPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSLicenseManagerLinuxSubscriptionsServiceRolePolicy")]
    AWSLicenseManagerLinuxSubscriptionsServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSLicenseManagerMasterAccountRolePolicy")]
    AWSLicenseManagerMasterAccountRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSLicenseManagerMemberAccountRolePolicy")]
    AWSLicenseManagerMemberAccountRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSLicenseManagerServiceRolePolicy")]
    AWSLicenseManagerServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSLicenseManagerUserSubscriptionsServiceRolePolicy")]
    AWSLicenseManagerUserSubscriptionsServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSM2ServicePolicy")]
    AWSM2ServicePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSMSKReplicatorExecutionRole")]
    AWSMSKReplicatorExecutionRole,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSManagedServicesDeploymentToolkitPolicy")]
    AWSManagedServicesDeploymentToolkitPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSManagedServices_ContactsServiceRolePolicy")]
    AWSManagedServices_ContactsServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSManagedServices_DetectiveControlsConfig_ServiceRolePolicy")]
    AWSManagedServices_DetectiveControlsConfig_ServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSManagedServices_EventsServiceRolePolicy")]
    AWSManagedServices_EventsServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSMarketplaceAmiIngestion")]
    AWSMarketplaceAmiIngestion,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSMarketplaceDeploymentServiceRolePolicy")]
    AWSMarketplaceDeploymentServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSMarketplaceFullAccess")]
    AWSMarketplaceFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSMarketplaceGetEntitlements")]
    AWSMarketplaceGetEntitlements,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSMarketplaceImageBuildFullAccess")]
    AWSMarketplaceImageBuildFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSMarketplaceLicenseManagementServiceRolePolicy")]
    AWSMarketplaceLicenseManagementServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSMarketplaceManageSubscriptions")]
    AWSMarketplaceManageSubscriptions,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSMarketplaceMeteringFullAccess")]
    AWSMarketplaceMeteringFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSMarketplaceMeteringRegisterUsage")]
    AWSMarketplaceMeteringRegisterUsage,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSMarketplaceProcurementSystemAdminFullAccess")]
    AWSMarketplaceProcurementSystemAdminFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSMarketplacePurchaseOrdersServiceRolePolicy")]
    AWSMarketplacePurchaseOrdersServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSMarketplaceRead-only")]
    AWSMarketplaceReadonly,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSMarketplaceResaleAuthorizationServiceRolePolicy")]
    AWSMarketplaceResaleAuthorizationServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSMarketplaceSellerFullAccess")]
    AWSMarketplaceSellerFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSMarketplaceSellerProductsFullAccess")]
    AWSMarketplaceSellerProductsFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSMarketplaceSellerProductsReadOnly")]
    AWSMarketplaceSellerProductsReadOnly,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSMediaConnectServicePolicy")]
    AWSMediaConnectServicePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSMediaTailorServiceRolePolicy")]
    AWSMediaTailorServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSMigrationHubDMSAccess")]
    AWSMigrationHubDMSAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSMigrationHubDiscoveryAccess")]
    AWSMigrationHubDiscoveryAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSMigrationHubFullAccess")]
    AWSMigrationHubFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSMigrationHubOrchestratorConsoleFullAccess")]
    AWSMigrationHubOrchestratorConsoleFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSMigrationHubOrchestratorInstanceRolePolicy")]
    AWSMigrationHubOrchestratorInstanceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSMigrationHubOrchestratorPlugin")]
    AWSMigrationHubOrchestratorPlugin,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSMigrationHubOrchestratorServiceRolePolicy")]
    AWSMigrationHubOrchestratorServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSMigrationHubRefactorSpaces-EnvironmentsWithoutBridgesFullAccess")]
    AWSMigrationHubRefactorSpacesEnvironmentsWithoutBridgesFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSMigrationHubRefactorSpaces-SSMAutomationPolicy")]
    AWSMigrationHubRefactorSpacesSSMAutomationPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSMigrationHubRefactorSpacesFullAccess")]
    AWSMigrationHubRefactorSpacesFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSMigrationHubRefactorSpacesServiceRolePolicy")]
    AWSMigrationHubRefactorSpacesServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSMigrationHubSMSAccess")]
    AWSMigrationHubSMSAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSMigrationHubStrategyCollector")]
    AWSMigrationHubStrategyCollector,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSMigrationHubStrategyConsoleFullAccess")]
    AWSMigrationHubStrategyConsoleFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSMigrationHubStrategyServiceRolePolicy")]
    AWSMigrationHubStrategyServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSMobileHub_FullAccess")]
    AWSMobileHub_FullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSMobileHub_ReadOnly")]
    AWSMobileHub_ReadOnly,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSMobileHub_ServiceUseOnly")]
    AWSMobileHub_ServiceUseOnly,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSNetworkFirewallServiceRolePolicy")]
    AWSNetworkFirewallServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSNetworkManagerCloudWANServiceRolePolicy")]
    AWSNetworkManagerCloudWANServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSNetworkManagerFullAccess")]
    AWSNetworkManagerFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSNetworkManagerReadOnlyAccess")]
    AWSNetworkManagerReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSNetworkManagerServiceRolePolicy")]
    AWSNetworkManagerServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSOpsWorksCMInstanceProfileRole")]
    AWSOpsWorksCMInstanceProfileRole,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSOpsWorksCMServiceRole")]
    AWSOpsWorksCMServiceRole,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSOpsWorksCloudWatchLogs")]
    AWSOpsWorksCloudWatchLogs,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSOpsWorksFullAccess")]
    AWSOpsWorksFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSOpsWorksInstanceRegistration")]
    AWSOpsWorksInstanceRegistration,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSOpsWorksRegisterCLI")]
    AWSOpsWorksRegisterCLI,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSOpsWorksRole")]
    AWSOpsWorksRole,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSQuickSightDescribeRDS")]
    AWSQuickSightDescribeRD,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSOpsWorksRegisterCLI_EC2")]
    AWSOpsWorksRegisterCLI_EC2,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSOpsWorksRegisterCLI_OnPremises")]
    AWSOpsWorksRegisterCLI_OnPremises,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSOpsWorks_FullAccess")]
    OpsWorks_FullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSOrganizationsFullAccess")]
    AWSOrganizationsFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSOrganizationsReadOnlyAccess")]
    AWSOrganizationsReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSOrganizationsServiceTrustPolicy")]
    AWSOrganizationsServiceTrustPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSOutpostsAuthorizeServerPolicy")]
    AWSOutpostsAuthorizeServerPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSOutpostsServiceRolePolicy")]
    AWSOutpostsServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSPanoramaApplianceRolePolicy")]
    AWSPanoramaApplianceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSPanoramaApplianceServiceRolePolicy")]
    AWSPanoramaApplianceServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSPanoramaFullAccess")]
    AWSPanoramaFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSPanoramaGreengrassGroupRolePolicy")]
    AWSPanoramaGreengrassGroupRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSPanoramaSageMakerRolePolicy")]
    AWSPanoramaSageMakerRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSPanoramaServiceLinkedRolePolicy")]
    AWSPanoramaServiceLinkedRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSPanoramaServiceRolePolicy")]
    AWSPanoramaServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSPriceListServiceFullAccess")]
    AWSPriceListServiceFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSPrivateCAAuditor")]
    AWSPrivateCAAuditor,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSPrivateCAFullAccess")]
    AWSPrivateCAFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSPrivateCAPrivilegedUser")]
    AWSPrivateCAPrivilegedUser,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSPrivateCAReadOnly")]
    AWSPrivateCAReadOnly,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSPrivateCAUser")]
    AWSPrivateCAUser,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSPrivateMarketplaceAdminFullAccess")]
    AWSPrivateMarketplaceAdminFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSPrivateMarketplaceRequests")]
    AWSPrivateMarketplaceRequests,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSPrivateNetworksServiceRolePolicy")]
    AWSPrivateNetworksServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSProtonCodeBuildProvisioningBasicAccess")]
    AWSProtonCodeBuildProvisioningBasicAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSProtonCodeBuildProvisioningServiceRolePolicy")]
    AWSProtonCodeBuildProvisioningServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSProtonDeveloperAccess")]
    AWSProtonDeveloperAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSProtonFullAccess")]
    AWSProtonFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSProtonReadOnlyAccess")]
    AWSProtonReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSProtonServiceGitSyncServiceRolePolicy")]
    AWSProtonServiceGitSyncServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSProtonSyncServiceRolePolicy")]
    AWSProtonSyncServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSPurchaseOrdersServiceRolePolicy")]
    AWSPurchaseOrdersServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSQuickSightDescribeRDS")]
    AWSQuickSightDescribeRDS,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSQuickSightDescribeRedshift")]
    AWSQuickSightDescribeRedshift,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSQuickSightElasticsearchPolicy")]
    AWSQuickSightElasticsearchPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSQuickSightIoTAnalyticsAccess")]
    AWSQuickSightIoTAnalyticsAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSQuickSightListIAM")]
    AWSQuickSightListIAM,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSQuickSightSageMakerPolicy")]
    AWSQuickSightSageMakerPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSQuickSightTimestreamPolicy")]
    AWSQuickSightTimestreamPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSQuicksightAthenaAccess")]
    AWSQuicksightAthenaAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSQuicksightOpenSearchPolicy")]
    AWSQuicksightOpenSearchPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSReachabilityAnalyzerServiceRolePolicy")]
    AWSReachabilityAnalyzerServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSRefactoringToolkitFullAccess")]
    AWSRefactoringToolkitFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSRefactoringToolkitSidecarPolicy")]
    AWSRefactoringToolkitSidecarPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSRepostSpaceSupportOperationsPolicy")]
    AWSRepostSpaceSupportOperationsPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSResilienceHubAsssessmentExecutionPolicy")]
    AWSResilienceHubAsssessmentExecutionPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSResourceAccessManagerFullAccess")]
    AWSResourceAccessManagerFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSResourceAccessManagerReadOnlyAccess")]
    AWSResourceAccessManagerReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSResourceAccessManagerResourceShareParticipantAccess")]
    AWSResourceAccessManagerResourceShareParticipantAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSResourceAccessManagerServiceRolePolicy")]
    AWSResourceAccessManagerServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSResourceExplorerFullAccess")]
    AWSResourceExplorerFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSResourceExplorerOrganizationsAccess")]
    AWSResourceExplorerOrganizationsAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSResourceExplorerReadOnlyAccess")]
    AWSResourceExplorerReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSResourceExplorerServiceRolePolicy")]
    AWSResourceExplorerServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSResourceGroupsReadOnlyAccess")]
    AWSResourceGroupsReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSRoboMakerReadOnlyAccess")]
    AWSRoboMakerReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSRoboMakerServicePolicy")]
    AWSRoboMakerServicePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSRoboMakerServiceRolePolicy")]
    AWSRoboMakerServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSRoboMaker_FullAccess")]
    AWSRoboMaker_FullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSRolesAnywhereServicePolicy")]
    AWSRolesAnywhereServicePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSS3OnOutpostsServiceRolePolicy")]
    AWSS3OnOutpostsServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSSSMForSAPServiceLinkedRolePolicy")]
    AWSSSMForSAPServiceLinkedRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSSSMOpsInsightsServiceRolePolicy")]
    AWSSSMOpsInsightsServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSSSODirectoryAdministrator")]
    AWSSSODirectoryAdministrator,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSSSODirectoryReadOnly")]
    AWSSSODirectoryReadOnly,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSSSOMasterAccountAdministrator")]
    AWSSSOMasterAccountAdministrator,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSSSOMemberAccountAdministrator")]
    AWSSSOMemberAccountAdministrator,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSSSOReadOnly")]
    AWSSSOReadOnly,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSSSOServiceRolePolicy")]
    AWSSSOServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSSavingsPlansFullAccess")]
    AWSSavingsPlansFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSSavingsPlansReadOnlyAccess")]
    AWSSavingsPlansReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSSecurityHubFullAccess")]
    AWSSecurityHubFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSSecurityHubOrganizationsAccess")]
    AWSSecurityHubOrganizationsAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSSecurityHubReadOnlyAccess")]
    AWSSecurityHubReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSSecurityHubServiceRolePolicy")]
    AWSSecurityHubServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSServiceCatalogAdminFullAccess")]
    AWSServiceCatalogAdminFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSServiceCatalogAdminReadOnlyAccess")]
    AWSServiceCatalogAdminReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSServiceCatalogAppRegistryFullAccess")]
    AWSServiceCatalogAppRegistryFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSServiceCatalogAppRegistryReadOnlyAccess")]
    AWSServiceCatalogAppRegistryReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSServiceCatalogAppRegistryServiceRolePolicy")]
    AWSServiceCatalogAppRegistryServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSServiceCatalogEndUserFullAccess")]
    AWSServiceCatalogEndUserFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSServiceCatalogEndUserReadOnlyAccess")]
    AWSServiceCatalogEndUserReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSServiceCatalogOrgsDataSyncServiceRolePolicy")]
    AWSServiceCatalogOrgsDataSyncServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSServiceCatalogSyncServiceRolePolicy")]
    AWSServiceCatalogSyncServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSServiceRoleForAmazonEKSNodegroup")]
    AWSServiceRoleForAmazonEKSNodegroup,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSServiceRoleForCloudWatchAlarmsActionSSMServiceRolePolicy")]
    AWSServiceRoleForCloudWatchAlarmsActionSSMServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSServiceRoleForCloudWatchMetrics_DbPerfInsightsServiceRolePolicy")]
    AWSServiceRoleForCloudWatchMetrics_DbPerfInsightsServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSServiceRoleForCodeGuru-Profiler")]
    AWSServiceRoleForCodeGuruProfiler,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSServiceRoleForCodeWhispererPolicy")]
    AWSServiceRoleForCodeWhispererPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSServiceRoleForEC2ScheduledInstances")]
    AWSServiceRoleForEC2ScheduledInstances,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSServiceRoleForGroundStationDataflowEndpointGroupPolicy")]
    AWSServiceRoleForGroundStationDataflowEndpointGroupPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSServiceRoleForImageBuilder")]
    AWSServiceRoleForImageBuilder,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSServiceRoleForIoTSiteWise")]
    AWSServiceRoleForIoTSiteWise,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSServiceRoleForLogDeliveryPolicy")]
    AWSServiceRoleForLogDeliveryPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSServiceRoleForMonitronPolicy")]
    AWSServiceRoleForMonitronPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSServiceRoleForNeptuneGraphPolicy")]
    AWSServiceRoleForNeptuneGraphPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSServiceRoleForPrivateMarketplaceAdminPolicy")]
    AWSServiceRoleForPrivateMarketplaceAdminPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSServiceRoleForSMS")]
    AWSServiceRoleForSMS,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSServiceRolePolicyForBackupReports")]
    AWSServiceRolePolicyForBackupReports,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSServiceRolePolicyForBackupRestoreTesting")]
    AWSServiceRolePolicyForBackupRestoreTesting,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSShieldDRTAccessPolicy")]
    AWSShieldDRTAccessPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSShieldServiceRolePolicy")]
    AWSShieldServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSStepFunctionsConsoleFullAccess")]
    AWSStepFunctionsConsoleFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSStepFunctionsFullAccess")]
    AWSStepFunctionsFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSStepFunctionsReadOnlyAccess")]
    AWSStepFunctionsReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSStorageGatewayFullAccess")]
    AWSStorageGatewayFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSStorageGatewayReadOnlyAccess")]
    AWSStorageGatewayReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSStorageGatewayServiceRolePolicy")]
    AWSStorageGatewayServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSSupplyChainFederationAdminAccess")]
    AWSSupplyChainFederationAdminAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSSupportAccess")]
    AWSSupportAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSSupportAppFullAccess")]
    AWSSupportAppFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSSupportAppReadOnlyAccess")]
    AWSSupportAppReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSSupportPlansFullAccess")]
    AWSSupportPlansFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSSupportPlansReadOnlyAccess")]
    AWSSupportPlansReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSSupportServiceRolePolicy")]
    AWSSupportServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSSystemsManagerAccountDiscoveryServicePolicy")]
    AWSSystemsManagerAccountDiscoveryServicePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSSystemsManagerChangeManagementServicePolicy")]
    AWSSystemsManagerChangeManagementServicePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSSystemsManagerForSAPFullAccess")]
    AWSSystemsManagerForSAPFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSSystemsManagerForSAPReadOnlyAccess")]
    AWSSystemsManagerForSAPReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSSystemsManagerOpsDataSyncServiceRolePolicy")]
    AWSSystemsManagerOpsDataSyncServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSThinkboxAWSPortalAdminPolicy")]
    AWSThinkboxAWSPortalAdminPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSThinkboxAWSPortalGatewayPolicy")]
    AWSThinkboxAWSPortalGatewayPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSThinkboxAWSPortalWorkerPolicy")]
    AWSThinkboxAWSPortalWorkerPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSThinkboxAssetServerPolicy")]
    AWSThinkboxAssetServerPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSThinkboxDeadlineResourceTrackerAccessPolicy")]
    AWSThinkboxDeadlineResourceTrackerAccessPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSThinkboxDeadlineResourceTrackerAdminPolicy")]
    AWSThinkboxDeadlineResourceTrackerAdminPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSThinkboxDeadlineSpotEventPluginAdminPolicy")]
    AWSThinkboxDeadlineSpotEventPluginAdminPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSThinkboxDeadlineSpotEventPluginWorkerPolicy")]
    AWSThinkboxDeadlineSpotEventPluginWorkerPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSTransferConsoleFullAccess")]
    AWSTransferConsoleFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSTransferFullAccess")]
    AWSTransferFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AWSTransferLoggingAccess")]
    AWSTransferLoggingAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSTransferReadOnlyAccess")]
    AWSTransferReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSTrustedAdvisorPriorityFullAccess")]
    AWSTrustedAdvisorPriorityFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSTrustedAdvisorPriorityReadOnlyAccess")]
    AWSTrustedAdvisorPriorityReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSTrustedAdvisorReportingServiceRolePolicy")]
    AWSTrustedAdvisorReportingServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSTrustedAdvisorServiceRolePolicy")]
    AWSTrustedAdvisorServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSUserNotificationsServiceLinkedRolePolicy")]
    AWSUserNotificationsServiceLinkedRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSVPCS2SVpnServiceRolePolicy")]
    AWSVPCS2SVpnServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSVPCTransitGatewayServiceRolePolicy")]
    AWSVPCTransitGatewayServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSVPCVerifiedAccessServiceRolePolicy")]
    AWSVPCVerifiedAccessServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSVendorInsightsAssessorFullAccess")]
    AWSVendorInsightsAssessorFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSVendorInsightsAssessorReadOnly")]
    AWSVendorInsightsAssessorReadOnly,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSVendorInsightsVendorFullAccess")]
    AWSVendorInsightsVendorFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSVendorInsightsVendorReadOnly")]
    AWSVendorInsightsVendorReadOnly,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSVpcLatticeServiceRolePolicy")]
    AWSVpcLatticeServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSWAFConsoleFullAccess")]
    AWSWAFConsoleFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSWAFConsoleReadOnlyAccess")]
    AWSWAFConsoleReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSWAFFullAccess")]
    AWSWAFFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSWAFReadOnlyAccess")]
    AWSWAFReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSWellArchitectedDiscoveryServiceRolePolicy")]
    AWSWellArchitectedDiscoveryServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSWellArchitectedOrganizationsServiceRolePolicy")]
    AWSWellArchitectedOrganizationsServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSWickrFullAccess")]
    AWSWickrFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSXRayDaemonWriteAccess")]
    AWSXRayDaemonWriteAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSXrayCrossAccountSharingConfiguration")]
    AWSXrayCrossAccountSharingConfiguration,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSXrayFullAccess")]
    AWSXrayFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSXrayReadOnlyAccess")]
    AWSXrayReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AWSXrayWriteOnlyAccess")]
    AWSXrayWriteOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSZonalAutoshiftPracticeRunSLRPolicy")]
    AWSZonalAutoshiftPracticeRunSLRPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AWSrePostPrivateCloudWatchAccess")]
    AWSrePostPrivateCloudWatchAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AccessAnalyzerServiceRolePolicy")]
    AccessAnalyzerServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AdministratorAccess")]
    AdministratorAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AdministratorAccess-AWSElasticBeanstalk")]
    AdministratorAccessAWSElasticBeanstalk,
    #[serde(rename = "arn:aws:iam::aws:policy/AdministratorAccess-Amplify")]
    AdministratorAccessAmplify,
    #[serde(rename = "arn:aws:iam::aws:policy/AlexaForBusinessDeviceSetup")]
    AlexaForBusinessDeviceSetup,
    #[serde(rename = "arn:aws:iam::aws:policy/AlexaForBusinessFullAccess")]
    AlexaForBusinessFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AlexaForBusinessGatewayExecution")]
    AlexaForBusinessGatewayExecution,
    #[serde(rename = "arn:aws:iam::aws:policy/AlexaForBusinessLifesizeDelegatedAccessPolicy")]
    AlexaForBusinessLifesizeDelegatedAccessPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AlexaForBusinessNetworkProfileServicePolicy")]
    AlexaForBusinessNetworkProfileServicePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AlexaForBusinessPolyDelegatedAccessPolicy")]
    AlexaForBusinessPolyDelegatedAccessPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AlexaForBusinessReadOnlyAccess")]
    AlexaForBusinessReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonAPIGatewayAdministrator")]
    AmazonAPIGatewayAdministrator,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonAPIGatewayInvokeFullAccess")]
    AmazonAPIGatewayInvokeFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonAPIGatewayPushToCloudWatchLogs")]
    AmazonAPIGatewayPushToCloudWatchLogs,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonAppFlowFullAccess")]
    AmazonAppFlowFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonAppFlowReadOnlyAccess")]
    AmazonAppFlowReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonAppStreamFullAccess")]
    AmazonAppStreamFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonAppStreamPCAAccess")]
    AmazonAppStreamPCAAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonAppStreamReadOnlyAccess")]
    AmazonAppStreamReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonAppStreamServiceAccess")]
    AmazonAppStreamServiceAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonAthenaFullAccess")]
    AmazonAthenaFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonAugmentedAIFullAccess")]
    AmazonAugmentedAIFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonAugmentedAIHumanLoopFullAccess")]
    AmazonAugmentedAIHumanLoopFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonAugmentedAIIntegratedAPIAccess")]
    AmazonAugmentedAIIntegratedAPIAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonBedrockFullAccess")]
    AmazonBedrockFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonBedrockReadOnly")]
    AmazonBedrockReadOnly,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonBraketFullAccess")]
    AmazonBraketFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonBraketJobsExecutionPolicy")]
    AmazonBraketJobsExecutionPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonBraketServiceRolePolicy")]
    AmazonBraketServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonChimeFullAccess")]
    AmazonChimeFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonChimeReadOnly")]
    AmazonChimeReadOnly,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonChimeSDK")]
    AmazonChimeSDK,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonChimeSDKMediaPipelinesServiceLinkedRolePolicy")]
    AmazonChimeSDKMediaPipelinesServiceLinkedRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonChimeSDKMessagingServiceRolePolicy")]
    AmazonChimeSDKMessagingServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonChimeServiceRolePolicy")]
    AmazonChimeServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonChimeTranscriptionServiceLinkedRolePolicy")]
    AmazonChimeTranscriptionServiceLinkedRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonChimeUserManagement")]
    AmazonChimeUserManagement,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonChimeVoiceConnectorServiceLinkedRolePolicy")]
    AmazonChimeVoiceConnectorServiceLinkedRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonCloudDirectoryFullAccess")]
    AmazonCloudDirectoryFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonCloudDirectoryReadOnlyAccess")]
    AmazonCloudDirectoryReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonCloudWatchEvidentlyFullAccess")]
    AmazonCloudWatchEvidentlyFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonCloudWatchEvidentlyReadOnlyAccess")]
    AmazonCloudWatchEvidentlyReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonCloudWatchEvidentlyServiceRolePolicy")]
    AmazonCloudWatchEvidentlyServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonCloudWatchRUMFullAccess")]
    AmazonCloudWatchRUMFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonCloudWatchRUMReadOnlyAccess")]
    AmazonCloudWatchRUMReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonCloudWatchRUMServiceRolePolicy")]
    AmazonCloudWatchRUMServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonCodeCatalystFullAccess")]
    AmazonCodeCatalystFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonCodeCatalystReadOnlyAccess")]
    AmazonCodeCatalystReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonCodeCatalystSupportAccess")]
    AmazonCodeCatalystSupportAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonCodeGuruProfilerAgentAccess")]
    AmazonCodeGuruProfilerAgentAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonCodeGuruProfilerFullAccess")]
    AmazonCodeGuruProfilerFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonCodeGuruProfilerReadOnlyAccess")]
    AmazonCodeGuruProfilerReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonCodeGuruReviewerFullAccess")]
    AmazonCodeGuruReviewerFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonCodeGuruReviewerReadOnlyAccess")]
    AmazonCodeGuruReviewerReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonCodeGuruReviewerServiceRolePolicy")]
    AmazonCodeGuruReviewerServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonCodeGuruSecurityFullAccess")]
    AmazonCodeGuruSecurityFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonCodeGuruSecurityScanAccess")]
    AmazonCodeGuruSecurityScanAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonCognitoDeveloperAuthenticatedIdentities")]
    AmazonCognitoDeveloperAuthenticatedIdentities,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonCognitoIdpEmailServiceRolePolicy")]
    AmazonCognitoIdpEmailServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonCognitoIdpServiceRolePolicy")]
    AmazonCognitoIdpServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonCognitoPowerUser")]
    AmazonCognitoPowerUser,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonCognitoReadOnly")]
    AmazonCognitoReadOnly,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonCognitoUnAuthedIdentitiesSessionPolicy")]
    AmazonCognitoUnAuthedIdentitiesSessionPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonCognitoUnauthenticatedIdentities")]
    AmazonCognitoUnauthenticatedIdentities,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonConnectCampaignsServiceLinkedRolePolicy")]
    AmazonConnectCampaignsServiceLinkedRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonConnectReadOnlyAccess")]
    AmazonConnectReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonConnectServiceLinkedRolePolicy")]
    AmazonConnectServiceLinkedRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonConnectSynchronizationServiceRolePolicy")]
    AmazonConnectSynchronizationServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonConnectVoiceIDFullAccess")]
    AmazonConnectVoiceIDFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonConnect_FullAccess")]
    AmazonConnect_FullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonDMSCloudWatchLogsRole")]
    AmazonDMSCloudWatchLogsRole,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonDMSRedshiftS3Role")]
    AmazonDMSRedshiftS3Role,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonDMSVPCManagementRole")]
    AmazonDMSVPCManagementRole,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonDRSVPCManagement")]
    AmazonDRSVPCManagement,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonDataZoneDomainExecutionRolePolicy")]
    AmazonDataZoneDomainExecutionRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonDataZoneEnvironmentRolePermissionsBoundary")]
    AmazonDataZoneEnvironmentRolePermissionsBoundary,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonDataZoneFullAccess")]
    AmazonDataZoneFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonDataZoneFullUserAccess")]
    AmazonDataZoneFullUserAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonDataZoneGlueManageAccessRolePolicy")]
    AmazonDataZoneGlueManageAccessRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonDataZoneRedshiftGlueProvisioningPolicy")]
    AmazonDataZoneRedshiftGlueProvisioningPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonDataZoneRedshiftManageAccessRolePolicy")]
    AmazonDataZoneRedshiftManageAccessRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonDetectiveFullAccess")]
    AmazonDetectiveFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonDetectiveInvestigatorAccess")]
    AmazonDetectiveInvestigatorAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonDetectiveMemberAccess")]
    AmazonDetectiveMemberAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonDetectiveOrganizationsAccess")]
    AmazonDetectiveOrganizationsAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonDetectiveServiceLinkedRolePolicy")]
    AmazonDetectiveServiceLinkedRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonDevOpsGuruConsoleFullAccess")]
    AmazonDevOpsGuruConsoleFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonDevOpsGuruFullAccess")]
    AmazonDevOpsGuruFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonDevOpsGuruOrganizationsAccess")]
    AmazonDevOpsGuruOrganizationsAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonDevOpsGuruReadOnlyAccess")]
    AmazonDevOpsGuruReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonDevOpsGuruServiceRolePolicy")]
    AmazonDevOpsGuruServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonDocDB-ElasticServiceRolePolicy")]
    AmazonDocDBElasticServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonDocDBConsoleFullAccess")]
    AmazonDocDBConsoleFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonDocDBElasticFullAccess")]
    AmazonDocDBElasticFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonDocDBElasticReadOnlyAccess")]
    AmazonDocDBElasticReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonDocDBFullAccess")]
    AmazonDocDBFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonDocDBReadOnlyAccess")]
    AmazonDocDBReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess")]
    AmazonDynamoDBFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccesswithDataPipeline")]
    AmazonDynamoDBFullAccesswithDataPipeline,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonDynamoDBReadOnlyAccess")]
    AmazonDynamoDBReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonEBSCSIDriverPolicy")]
    AmazonEBSCSIDriverPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonEC2ContainerRegistryFullAccess")]
    AmazonEC2ContainerRegistryFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonEC2ContainerRegistryPowerUser")]
    AmazonEC2ContainerRegistryPowerUser,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonEC2ContainerRegistryReadOnly")]
    AmazonEC2ContainerRegistryReadOnly,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonEC2ContainerServiceAutoscaleRole")]
    AmazonEC2ContainerServiceAutoscaleRole,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonEC2ContainerServiceEventsRole")]
    AmazonEC2ContainerServiceEventsRole,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonEC2ContainerServiceFullAccess")]
    AmazonEC2ContainerServiceFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonEC2ContainerServiceRole")]
    AmazonEC2ContainerServiceRole,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonEC2ContainerServiceforEC2Role")]
    AmazonEC2ContainerServiceforEC2Role,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonEC2FullAccess")]
    AmazonEC2FullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonEC2ReadOnlyAccess")]
    AmazonEC2ReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonEC2ReportsAccess")]
    AmazonEC2ReportsAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonEC2RolePolicyForLaunchWizard")]
    AmazonEC2RolePolicyForLaunchWizard,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonEC2RoleforAWSCodeDeploy")]
    AmazonEC2RoleforAWSCodeDeploy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonEC2RoleforAWSCodeDeployLimited")]
    AmazonEC2RoleforAWSCodeDeployLimited,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonEC2RoleforDataPipelineRole")]
    AmazonEC2RoleforDataPipelineRole,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonEC2RoleforSSM")]
    AmazonEC2RoleforSSM,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonEC2SpotFleetAutoscaleRole")]
    AmazonEC2SpotFleetAutoscaleRole,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonEC2SpotFleetRole")]
    AmazonEC2SpotFleetRole,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonEC2SpotFleetTaggingRole")]
    AmazonEC2SpotFleetTaggingRole,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonECS_FullAccess")]
    AmazonECSFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonECSInfrastructureRolePolicyForServiceConnectTransportLayerSecurity")]
    AmazonECSInfrastructureRolePolicyForServiceConnectTransportLayerSecurity,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonECSInfrastructureRolePolicyForVolumes")]
    AmazonECSInfrastructureRolePolicyForVolumes,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonECSServiceRolePolicy")]
    AmazonECSServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonECSTaskExecutionRolePolicy")]
    AmazonECSTaskExecutionRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonEFSCSIDriverPolicy")]
    AmazonEFSCSIDriverPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonEKSClusterPolicy")]
    AmazonEKSClusterPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonEKSConnectorServiceRolePolicy")]
    AmazonEKSConnectorServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonEKSFargatePodExecutionRolePolicy")]
    AmazonEKSFargatePodExecutionRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonEKSForFargateServiceRolePolicy")]
    AmazonEKSForFargateServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonEKSLocalOutpostClusterPolicy")]
    AmazonEKSLocalOutpostClusterPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonEKSLocalOutpostServiceRolePolicy")]
    AmazonEKSLocalOutpostServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonEKSServicePolicy")]
    AmazonEKSServicePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonEKSServiceRolePolicy")]
    AmazonEKSServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonEKSVPCResourceController")]
    AmazonEKSVPCResourceController,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonEKSWorkerNodePolicy")]
    AmazonEKSWorkerNodePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonEKS_CNI_Policy")]
    AmazonEKS_CNI_Policy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonEMRCleanupPolicy")]
    AmazonEMRCleanupPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonEMRContainersServiceRolePolicy")]
    AmazonEMRContainersServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonEMRFullAccessPolicy_v2")]
    AmazonEMRFullAccessPolicy_v2,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonEMRReadOnlyAccessPolicy_v2")]
    AmazonEMRReadOnlyAccessPolicy_v2,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonEMRServerlessServiceRolePolicy")]
    AmazonEMRServerlessServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonEMRServicePolicy_v2")]
    AmazonEMRServicePolicy_v2,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonESCognitoAccess")]
    AmazonESCognitoAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonESFullAccess")]
    AmazonESFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonESReadOnlyAccess")]
    AmazonESReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonElastiCacheFullAccess")]
    AmazonElastiCacheFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonElastiCacheReadOnlyAccess")]
    AmazonElastiCacheReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonElasticContainerRegistryPublicFullAccess")]
    AmazonElasticContainerRegistryPublicFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonElasticContainerRegistryPublicPowerUser")]
    AmazonElasticContainerRegistryPublicPowerUser,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonElasticContainerRegistryPublicReadOnly")]
    AmazonElasticContainerRegistryPublicReadOnly,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonElasticFileSystemClientFullAccess")]
    AmazonElasticFileSystemClientFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonElasticFileSystemClientReadOnlyAccess")]
    AmazonElasticFileSystemClientReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonElasticFileSystemClientReadWriteAccess")]
    AmazonElasticFileSystemClientReadWriteAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonElasticFileSystemFullAccess")]
    AmazonElasticFileSystemFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonElasticFileSystemReadOnlyAccess")]
    AmazonElasticFileSystemReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonElasticFileSystemServiceRolePolicy")]
    AmazonElasticFileSystemServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonElasticFileSystemsUtils")]
    AmazonElasticFileSystemsUtils,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonElasticMapReduceEditorsRole")]
    AmazonElasticMapReduceEditorsRole,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonElasticMapReduceFullAccess")]
    AmazonElasticMapReduceFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonElasticMapReducePlacementGroupPolicy")]
    AmazonElasticMapReducePlacementGroupPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonElasticMapReduceReadOnlyAccess")]
    AmazonElasticMapReduceReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonElasticMapReduceRole")]
    AmazonElasticMapReduceRole,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonElasticMapReduceforAutoScalingRole")]
    AmazonElasticMapReduceforAutoScalingRole,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonElasticMapReduceforEC2Role")]
    AmazonElasticMapReduceforEC2Role,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonElasticTranscoderFullAccess")]
    AmazonElasticTranscoderFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonElasticTranscoderJobsSubmitter")]
    AmazonElasticTranscoderJobsSubmitter,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonElasticTranscoderReadOnlyAccess")]
    AmazonElasticTranscoderReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonElasticTranscoderRole")]
    AmazonElasticTranscoderRole,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonElasticTranscoder_FullAccess")]
    ElasticTranscoder_FullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonElasticTranscoder_JobsSubmitter")]
    ElasticTranscoder_JobsSubmitter,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonElasticTranscoder_ReadOnlyAccess")]
    ElasticTranscoder_ReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonElasticsearchServiceRolePolicy")]
    AmazonElasticsearchServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonEventBridgeApiDestinationsServiceRolePolicy")]
    AmazonEventBridgeApiDestinationsServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonEventBridgeFullAccess")]
    AmazonEventBridgeFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonEventBridgePipesFullAccess")]
    AmazonEventBridgePipesFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonEventBridgePipesOperatorAccess")]
    AmazonEventBridgePipesOperatorAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonEventBridgePipesReadOnlyAccess")]
    AmazonEventBridgePipesReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonEventBridgeReadOnlyAccess")]
    AmazonEventBridgeReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonEventBridgeSchedulerFullAccess")]
    AmazonEventBridgeSchedulerFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonEventBridgeSchedulerReadOnlyAccess")]
    AmazonEventBridgeSchedulerReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonEventBridgeSchemasFullAccess")]
    AmazonEventBridgeSchemasFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonEventBridgeSchemasReadOnlyAccess")]
    AmazonEventBridgeSchemasReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonEventBridgeSchemasServiceRolePolicy")]
    AmazonEventBridgeSchemasServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonFISServiceRolePolicy")]
    AmazonFISServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonFSxConsoleFullAccess")]
    AmazonFSxConsoleFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonFSxConsoleReadOnlyAccess")]
    AmazonFSxConsoleReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonFSxFullAccess")]
    AmazonFSxFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonFSxReadOnlyAccess")]
    AmazonFSxReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonFSxServiceRolePolicy")]
    AmazonFSxServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonForecastFullAccess")]
    AmazonForecastFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonFraudDetectorFullAccessPolicy")]
    AmazonFraudDetectorFullAccessPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonFreeRTOSFullAccess")]
    AmazonFreeRTOSFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonFreeRTOSOTAUpdate")]
    AmazonFreeRTOSOTAUpdate,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonGlacierFullAccess")]
    AmazonGlacierFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonGlacierReadOnlyAccess")]
    AmazonGlacierReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonGrafanaAthenaAccess")]
    AmazonGrafanaAthenaAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonGrafanaCloudWatchAccess")]
    AmazonGrafanaCloudWatchAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonGrafanaRedshiftAccess")]
    AmazonGrafanaRedshiftAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonGrafanaServiceLinkedRolePolicy")]
    AmazonGrafanaServiceLinkedRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonGuardDutyFullAccess")]
    AmazonGuardDutyFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonGuardDutyMalwareProtectionServiceRolePolicy")]
    AmazonGuardDutyMalwareProtectionServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonGuardDutyReadOnlyAccess")]
    AmazonGuardDutyReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonGuardDutyServiceRolePolicy")]
    AmazonGuardDutyServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonHealthLakeFullAccess")]
    AmazonHealthLakeFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonHealthLakeReadOnlyAccess")]
    AmazonHealthLakeReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonHoneycodeFullAccess")]
    AmazonHoneycodeFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonHoneycodeReadOnlyAccess")]
    AmazonHoneycodeReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonHoneycodeServiceRolePolicy")]
    AmazonHoneycodeServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonHoneycodeTeamAssociationFullAccess")]
    AmazonHoneycodeTeamAssociationFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonHoneycodeTeamAssociationReadOnlyAccess")]
    AmazonHoneycodeTeamAssociationReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonHoneycodeWorkbookFullAccess")]
    AmazonHoneycodeWorkbookFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonHoneycodeWorkbookReadOnlyAccess")]
    AmazonHoneycodeWorkbookReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonInspector2AgentlessServiceRolePolicy")]
    AmazonInspector2AgentlessServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonInspector2FullAccess")]
    AmazonInspector2FullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonInspector2ManagedCisPolicy")]
    AmazonInspector2ManagedCisPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonInspector2ReadOnlyAccess")]
    AmazonInspector2ReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonInspector2ServiceRolePolicy")]
    AmazonInspector2ServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonInspectorFullAccess")]
    AmazonInspectorFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonInspectorReadOnlyAccess")]
    AmazonInspectorReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonInspectorServiceRolePolicy")]
    AmazonInspectorServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonKendraFullAccess")]
    AmazonKendraFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonKendraReadOnlyAccess")]
    AmazonKendraReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonKeyspacesFullAccess")]
    AmazonKeyspacesFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonKeyspacesReadOnlyAccess")]
    AmazonKeyspacesReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonKeyspacesReadOnlyAccess_v2")]
    AmazonKeyspacesReadOnlyAccess_v2,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonKinesisAnalyticsFullAccess")]
    AmazonKinesisAnalyticsFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonKinesisAnalyticsReadOnly")]
    AmazonKinesisAnalyticsReadOnly,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonKinesisFirehoseFullAccess")]
    AmazonKinesisFirehoseFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonKinesisFirehoseReadOnlyAccess")]
    AmazonKinesisFirehoseReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonKinesisFullAccess")]
    AmazonKinesisFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonKinesisReadOnlyAccess")]
    AmazonKinesisReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonKinesisVideoStreamsFullAccess")]
    AmazonKinesisVideoStreamsFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonKinesisVideoStreamsReadOnlyAccess")]
    AmazonKinesisVideoStreamsReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonLaunchWizard_Fullaccess")]
    AmazonLaunchWizard_Fullaccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonLaunchWizardFullAccessV2")]
    AmazonLaunchWizardFullAccessV2,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonLexChannelsAccess")]
    AmazonLexChannelsAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonLexFullAccess")]
    AmazonLexFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonLexReadOnly")]
    AmazonLexReadOnly,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonLexReplicationPolicy")]
    AmazonLexReplicationPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonLexRunBotsOnly")]
    AmazonLexRunBotsOnly,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonLexV2BotPolicy")]
    AmazonLexV2BotPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonLookoutEquipmentFullAccess")]
    AmazonLookoutEquipmentFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonLookoutEquipmentReadOnlyAccess")]
    AmazonLookoutEquipmentReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonLookoutMetricsFullAccess")]
    AmazonLookoutMetricsFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonLookoutMetricsReadOnlyAccess")]
    AmazonLookoutMetricsReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonLookoutVisionConsoleFullAccess")]
    AmazonLookoutVisionConsoleFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonLookoutVisionConsoleReadOnlyAccess")]
    AmazonLookoutVisionConsoleReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonLookoutVisionFullAccess")]
    AmazonLookoutVisionFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonLookoutVisionReadOnlyAccess")]
    AmazonLookoutVisionReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonMCSFullAccess")]
    AmazonMCSFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonMCSReadOnlyAccess")]
    AmazonMCSReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonMQApiFullAccess")]
    AmazonMQApiFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonMQApiReadOnlyAccess")]
    AmazonMQApiReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonMQFullAccess")]
    AmazonMQFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonMQReadOnlyAccess")]
    AmazonMQReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonMQServiceRolePolicy")]
    AmazonMQServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonMSKConnectReadOnlyAccess")]
    AmazonMSKConnectReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonMSKFullAccess")]
    AmazonMSKFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonMSKReadOnlyAccess")]
    AmazonMSKReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonMWAAServiceRolePolicy")]
    AmazonMWAAServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonMachineLearningBatchPredictionsAccess")]
    AmazonMachineLearningBatchPredictionsAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonMachineLearningCreateOnlyAccess")]
    AmazonMachineLearningCreateOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonMachineLearningFullAccess")]
    AmazonMachineLearningFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonMachineLearningManageRealTimeEndpointOnlyAccess")]
    AmazonMachineLearningManageRealTimeEndpointOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonMachineLearningReadOnlyAccess")]
    AmazonMachineLearningReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonMachineLearningRealTimePredictionOnlyAccess")]
    AmazonMachineLearningRealTimePredictionOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonMachineLearningRoleforRedshiftDataSource")]
    AmazonMachineLearningRoleforRedshiftDataSource,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonMachineLearningRoleforRedshiftDataSourceV3")]
    AmazonMachineLearningRoleforRedshiftDataSourceV3,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonMacieFullAccess")]
    AmazonMacieFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonMacieHandshakeRole")]
    AmazonMacieHandshakeRole,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonMacieReadOnlyAccess")]
    AmazonMacieReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonMacieServiceRole")]
    AmazonMacieServiceRole,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonMacieServiceRolePolicy")]
    AmazonMacieServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonManagedBlockchainConsoleFullAccess")]
    AmazonManagedBlockchainConsoleFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonManagedBlockchainFullAccess")]
    AmazonManagedBlockchainFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonManagedBlockchainReadOnlyAccess")]
    AmazonManagedBlockchainReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonManagedBlockchainServiceRolePolicy")]
    AmazonManagedBlockchainServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonMechanicalTurkFullAccess")]
    AmazonMechanicalTurkFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonMechanicalTurkReadOnly")]
    AmazonMechanicalTurkReadOnly,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonMemoryDBFullAccess")]
    AmazonMemoryDBFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonMemoryDBReadOnlyAccess")]
    AmazonMemoryDBReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonMobileAnalyticsFinancialReportAccess")]
    AmazonMobileAnalyticsFinancialReportAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonMobileAnalyticsFullAccess")]
    AmazonMobileAnalyticsFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonMobileAnalyticsNon-financialReportAccess")]
    AmazonMobileAnalyticsNonfinancialReportAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonMobileAnalyticsWriteOnlyAccess")]
    AmazonMobileAnalyticsWriteOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonMonitronFullAccess")]
    AmazonMonitronFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonNimbleStudio-LaunchProfileWorker")]
    AmazonNimbleStudioLaunchProfileWorker,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonNimbleStudio-StudioAdmin")]
    AmazonNimbleStudioStudioAdmin,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonNimbleStudio-StudioUser")]
    AmazonNimbleStudioStudioUser,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonOmicsFullAccess")]
    AmazonOmicsFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonOmicsReadOnlyAccess")]
    AmazonOmicsReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonOneEnterpriseFullAccess")]
    AmazonOneEnterpriseFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonOneEnterpriseInstallerAccess")]
    AmazonOneEnterpriseInstallerAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonOneEnterpriseReadOnlyAccess")]
    AmazonOneEnterpriseReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonOpenSearchDashboardsServiceRolePolicy")]
    AmazonOpenSearchDashboardsServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonOpenSearchIngestionFullAccess")]
    AmazonOpenSearchIngestionFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonOpenSearchIngestionReadOnlyAccess")]
    AmazonOpenSearchIngestionReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonOpenSearchIngestionServiceRolePolicy")]
    AmazonOpenSearchIngestionServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonOpenSearchServerlessServiceRolePolicy")]
    AmazonOpenSearchServerlessServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonOpenSearchServiceCognitoAccess")]
    AmazonOpenSearchServiceCognitoAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonOpenSearchServiceFullAccess")]
    AmazonOpenSearchServiceFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonOpenSearchServiceReadOnlyAccess")]
    AmazonOpenSearchServiceReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonOpenSearchServiceRolePolicy")]
    AmazonOpenSearchServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonPersonalizeFullAccess")]
    AmazonPersonalizeFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonPollyFullAccess")]
    AmazonPollyFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonPollyReadOnlyAccess")]
    AmazonPollyReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonPrometheusConsoleFullAccess")]
    AmazonPrometheusConsoleFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonPrometheusFullAccess")]
    AmazonPrometheusFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonPrometheusQueryAccess")]
    AmazonPrometheusQueryAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonPrometheusRemoteWriteAccess")]
    AmazonPrometheusRemoteWriteAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonPrometheusScraperServiceRolePolicy")]
    AmazonPrometheusScraperServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonQFullAccess")]
    AmazonQFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonQLDBConsoleFullAccess")]
    AmazonQLDBConsoleFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonQLDBFullAccess")]
    AmazonQLDBFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonQLDBReadOnly")]
    AmazonQLDBReadOnly,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonRDSBetaServiceRolePolicy")]
    AmazonRDSBetaServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonRDSCustomInstanceProfileRolePolicy")]
    AmazonRDSCustomInstanceProfileRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonRDSCustomPreviewServiceRolePolicy")]
    AmazonRDSCustomPreviewServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonRDSCustomServiceRolePolicy")]
    AmazonRDSCustomServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonRDSDataFullAccess")]
    AmazonRDSDataFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonRDSDirectoryServiceAccess")]
    AmazonRDSDirectoryServiceAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonRDSEnhancedMonitoringRole")]
    AmazonRDSEnhancedMonitoringRole,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonRDSFullAccess")]
    AmazonRDSFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonRDSPerformanceInsightsFullAccess")]
    AmazonRDSPerformanceInsightsFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonRDSPerformanceInsightsReadOnly")]
    AmazonRDSPerformanceInsightsReadOnly,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonRDSPreviewServiceRolePolicy")]
    AmazonRDSPreviewServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonRDSReadOnlyAccess")]
    AmazonRDSReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonRDSServiceRolePolicy")]
    AmazonRDSServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonRedshiftAllCommandsFullAccess")]
    AmazonRedshiftAllCommandsFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonRedshiftDataFullAccess")]
    AmazonRedshiftDataFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonRedshiftFullAccess")]
    AmazonRedshiftFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonRedshiftQueryEditor")]
    AmazonRedshiftQueryEditor,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonRedshiftQueryEditorV2FullAccess")]
    AmazonRedshiftQueryEditorV2FullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonRedshiftQueryEditorV2NoSharing")]
    AmazonRedshiftQueryEditorV2NoSharing,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonRedshiftQueryEditorV2ReadSharing")]
    AmazonRedshiftQueryEditorV2ReadSharing,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonRedshiftQueryEditorV2ReadWriteSharing")]
    AmazonRedshiftQueryEditorV2ReadWriteSharing,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonRedshiftReadOnlyAccess")]
    AmazonRedshiftReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonRedshiftServiceLinkedRolePolicy")]
    AmazonRedshiftServiceLinkedRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonRekognitionCustomLabelsFullAccess")]
    AmazonRekognitionCustomLabelsFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonRekognitionFullAccess")]
    AmazonRekognitionFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonRekognitionReadOnlyAccess")]
    AmazonRekognitionReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonRekognitionServiceRole")]
    AmazonRekognitionServiceRole,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonRoute53AutoNamingFullAccess")]
    AmazonRoute53AutoNamingFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonRoute53AutoNamingReadOnlyAccess")]
    AmazonRoute53AutoNamingReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonRoute53AutoNamingRegistrantAccess")]
    AmazonRoute53AutoNamingRegistrantAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonRoute53DomainsFullAccess")]
    AmazonRoute53DomainsFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonRoute53DomainsReadOnlyAccess")]
    AmazonRoute53DomainsReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonRoute53FullAccess")]
    AmazonRoute53FullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonRoute53ReadOnlyAccess")]
    AmazonRoute53ReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonRoute53RecoveryClusterFullAccess")]
    AmazonRoute53RecoveryClusterFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonRoute53RecoveryClusterReadOnlyAccess")]
    AmazonRoute53RecoveryClusterReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonRoute53RecoveryControlConfigFullAccess")]
    AmazonRoute53RecoveryControlConfigFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonRoute53RecoveryControlConfigReadOnlyAccess")]
    AmazonRoute53RecoveryControlConfigReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonRoute53RecoveryReadinessFullAccess")]
    AmazonRoute53RecoveryReadinessFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonRoute53RecoveryReadinessReadOnlyAccess")]
    AmazonRoute53RecoveryReadinessReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonRoute53ResolverFullAccess")]
    AmazonRoute53ResolverFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonRoute53ResolverReadOnlyAccess")]
    AmazonRoute53ResolverReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonS3FullAccess")]
    AmazonS3FullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonS3ObjectLambdaExecutionRolePolicy")]
    AmazonS3ObjectLambdaExecutionRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonS3OutpostsFullAccess")]
    AmazonS3OutpostsFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonS3OutpostsReadOnlyAccess")]
    AmazonS3OutpostsReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonS3ReadOnlyAccess")]
    AmazonS3ReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonSESFullAccess")]
    AmazonSESFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonSESReadOnlyAccess")]
    AmazonSESReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonSNSFullAccess")]
    AmazonSNSFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonSNSReadOnlyAccess")]
    AmazonSNSReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonSNSRole")]
    AmazonSNSRole,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonSQSFullAccess")]
    AmazonSQSFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonSQSReadOnlyAccess")]
    AmazonSQSReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonSSMAutomationApproverAccess")]
    AmazonSSMAutomationApproverAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonSSMAutomationRole")]
    AmazonSSMAutomationRole,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonSSMDirectoryServiceAccess")]
    AmazonSSMDirectoryServiceAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonSSMFullAccess")]
    AmazonSSMFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonSSMMaintenanceWindowRole")]
    AmazonSSMMaintenanceWindowRole,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonSSMManagedEC2InstanceDefaultPolicy")]
    AmazonSSMManagedEC2InstanceDefaultPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonSSMManagedInstanceCore")]
    AmazonSSMManagedInstanceCore,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonSSMPatchAssociation")]
    AmazonSSMPatchAssociation,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonSSMReadOnlyAccess")]
    AmazonSSMReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonSSMServiceRolePolicy")]
    AmazonSSMServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonSageMakerAdmin-ServiceCatalogProductsServiceRolePolicy")]
    AmazonSageMakerAdminServiceCatalogProductsServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonSageMakerCanvasAIServicesAccess")]
    AmazonSageMakerCanvasAIServicesAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonSageMakerCanvasBedrockAccess")]
    AmazonSageMakerCanvasBedrockAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonSageMakerCanvasDataPrepFullAccess")]
    AmazonSageMakerCanvasDataPrepFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonSageMakerCanvasDirectDeployAccess")]
    AmazonSageMakerCanvasDirectDeployAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonSageMakerCanvasForecastAccess")]
    AmazonSageMakerCanvasForecastAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonSageMakerCanvasFullAccess")]
    AmazonSageMakerCanvasFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonSageMakerClusterInstanceRolePolicy")]
    AmazonSageMakerClusterInstanceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonSageMakerCoreServiceRolePolicy")]
    AmazonSageMakerCoreServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonSageMakerEdgeDeviceFleetPolicy")]
    AmazonSageMakerEdgeDeviceFleetPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonSageMakerFeatureStoreAccess")]
    AmazonSageMakerFeatureStoreAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonSageMakerFullAccess")]
    AmazonSageMakerFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonSageMakerGeospatialExecutionRole")]
    AmazonSageMakerGeospatialExecutionRole,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonSageMakerGeospatialFullAccess")]
    AmazonSageMakerGeospatialFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonSageMakerGroundTruthExecution")]
    AmazonSageMakerGroundTruthExecution,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonSageMakerMechanicalTurkAccess")]
    AmazonSageMakerMechanicalTurkAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonSageMakerModelGovernanceUseAccess")]
    AmazonSageMakerModelGovernanceUseAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonSageMakerModelRegistryFullAccess")]
    AmazonSageMakerModelRegistryFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonSageMakerNotebooksServiceRolePolicy")]
    AmazonSageMakerNotebooksServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonSageMakerPartnerServiceCatalogProductsApiGatewayServiceRolePolicy")]
    AmazonSageMakerPartnerServiceCatalogProductsApiGatewayServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonSageMakerPartnerServiceCatalogProductsCloudFormationServiceRolePolicy")]
    AmazonSageMakerPartnerServiceCatalogProductsCloudFormationServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonSageMakerPartnerServiceCatalogProductsLambdaServiceRolePolicy")]
    AmazonSageMakerPartnerServiceCatalogProductsLambdaServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonSageMakerPipelinesIntegrations")]
    AmazonSageMakerPipelinesIntegrations,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonSageMakerReadOnly")]
    AmazonSageMakerReadOnly,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonSageMakerServiceCatalogProductsApiGatewayServiceRolePolicy")]
    AmazonSageMakerServiceCatalogProductsApiGatewayServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonSageMakerServiceCatalogProductsCloudformationServiceRolePolicy")]
    AmazonSageMakerServiceCatalogProductsCloudformationServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonSageMakerServiceCatalogProductsCodeBuildServiceRolePolicy")]
    AmazonSageMakerServiceCatalogProductsCodeBuildServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonSageMakerServiceCatalogProductsCodePipelineServiceRolePolicy")]
    AmazonSageMakerServiceCatalogProductsCodePipelineServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonSageMakerServiceCatalogProductsEventsServiceRolePolicy")]
    AmazonSageMakerServiceCatalogProductsEventsServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonSageMakerServiceCatalogProductsFirehoseServiceRolePolicy")]
    AmazonSageMakerServiceCatalogProductsFirehoseServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonSageMakerServiceCatalogProductsGlueServiceRolePolicy")]
    AmazonSageMakerServiceCatalogProductsGlueServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonSageMakerServiceCatalogProductsLambdaServiceRolePolicy")]
    AmazonSageMakerServiceCatalogProductsLambdaServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonSecurityLakeAdministrator")]
    AmazonSecurityLakeAdministrator,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonSecurityLakeMetastoreManager")]
    AmazonSecurityLakeMetastoreManager,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonSecurityLakePermissionsBoundary")]
    AmazonSecurityLakePermissionsBoundary,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonSumerianFullAccess")]
    AmazonSumerianFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonTextractFullAccess")]
    AmazonTextractFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmazonTextractServiceRole")]
    AmazonTextractServiceRole,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonTimestreamConsoleFullAccess")]
    AmazonTimestreamConsoleFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonTimestreamFullAccess")]
    AmazonTimestreamFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonTimestreamInfluxDBFullAccess")]
    AmazonTimestreamInfluxDBFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonTimestreamInfluxDBServiceRolePolicy")]
    AmazonTimestreamInfluxDBServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonTimestreamReadOnlyAccess")]
    AmazonTimestreamReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonTranscribeFullAccess")]
    AmazonTranscribeFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonTranscribeReadOnlyAccess")]
    AmazonTranscribeReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonVPCCrossAccountNetworkInterfaceOperations")]
    AmazonVPCCrossAccountNetworkInterfaceOperations,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonVPCFullAccess")]
    AmazonVPCFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonVPCNetworkAccessAnalyzerFullAccessPolicy")]
    AmazonVPCNetworkAccessAnalyzerFullAccessPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonVPCReachabilityAnalyzerFullAccessPolicy")]
    AmazonVPCReachabilityAnalyzerFullAccessPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonVPCReachabilityAnalyzerPathComponentReadPolicy")]
    AmazonVPCReachabilityAnalyzerPathComponentReadPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonVPCReadOnlyAccess")]
    AmazonVPCReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonWorkDocsFullAccess")]
    AmazonWorkDocsFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonWorkDocsReadOnlyAccess")]
    AmazonWorkDocsReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonWorkMailEventsServiceRolePolicy")]
    AmazonWorkMailEventsServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonWorkMailFullAccess")]
    AmazonWorkMailFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonWorkMailMessageFlowFullAccess")]
    AmazonWorkMailMessageFlowFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonWorkMailMessageFlowReadOnlyAccess")]
    AmazonWorkMailMessageFlowReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonWorkMailReadOnlyAccess")]
    AmazonWorkMailReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonWorkSpacesAdmin")]
    AmazonWorkSpacesAdmin,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonWorkSpacesApplicationManagerAdminAccess")]
    AmazonWorkSpacesApplicationManagerAdminAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonWorkSpacesSelfServiceAccess")]
    AmazonWorkSpacesSelfServiceAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonWorkSpacesServiceAccess")]
    AmazonWorkSpacesServiceAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonWorkSpacesWebReadOnly")]
    AmazonWorkSpacesWebReadOnly,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AmazonWorkSpacesWebServiceRolePolicy")]
    AmazonWorkSpacesWebServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonWorkspacesPCAAccess")]
    AmazonWorkspacesPCAAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonZocaloFullAccess")]
    AmazonZocaloFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AmazonZocaloReadOnlyAccess")]
    AmazonZocaloReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AmplifyBackendDeployFullAccess")]
    AmplifyBackendDeployFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AppIntegrationsServiceLinkedRolePolicy")]
    AppIntegrationsServiceLinkedRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AppRunnerNetworkingServiceRolePolicy")]
    AppRunnerNetworkingServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AppRunnerServiceRolePolicy")]
    AppRunnerServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/ApplicationAutoScalingForAmazonAppStreamAccess")]
    ApplicationAutoScalingForAmazonAppStreamAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/ApplicationDiscoveryServiceContinuousExportServiceRolePolicy")]
    ApplicationDiscoveryServiceContinuousExportServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AutoScalingConsoleFullAccess")]
    AutoScalingConsoleFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AutoScalingConsoleReadOnlyAccess")]
    AutoScalingConsoleReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/AutoScalingFullAccess")]
    AutoScalingFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AutoScalingNotificationAccessRole")]
    AutoScalingNotificationAccessRole,
    #[serde(rename = "arn:aws:iam::aws:policy/AutoScalingReadOnlyAccess")]
    AutoScalingReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/AutoScalingServiceRolePolicy")]
    AutoScalingServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AwsGlueDataBrewFullAccessPolicy")]
    AwsGlueDataBrewFullAccessPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/AwsGlueSessionUserRestrictedNotebookPolicy")]
    AwsGlueSessionUserRestrictedNotebookPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AwsGlueSessionUserRestrictedNotebookServiceRole")]
    AwsGlueSessionUserRestrictedNotebookServiceRole,
    #[serde(rename = "arn:aws:iam::aws:policy/AwsGlueSessionUserRestrictedPolicy")]
    AwsGlueSessionUserRestrictedPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/AwsGlueSessionUserRestrictedServiceRole")]
    AwsGlueSessionUserRestrictedServiceRole,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/BatchServiceRolePolicy")]
    BatchServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/job-function/Billing")]
    Billing,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/CertificateManagerServiceRolePolicy")]
    CertificateManagerServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/ClientVPNServiceConnectionsRolePolicy")]
    ClientVPNServiceConnectionsRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/ClientVPNServiceRolePolicy")]
    ClientVPNServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/CloudFormationStackSetsOrgAdminServiceRolePolicy")]
    CloudFormationStackSetsOrgAdminServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/CloudFormationStackSetsOrgMemberServiceRolePolicy")]
    CloudFormationStackSetsOrgMemberServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/CloudFrontFullAccess")]
    CloudFrontFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/CloudFrontReadOnlyAccess")]
    CloudFrontReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/CloudHSMServiceRolePolicy")]
    CloudHSMServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/CloudSearchFullAccess")]
    CloudSearchFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/CloudSearchReadOnlyAccess")]
    CloudSearchReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/CloudTrailServiceRolePolicy")]
    CloudTrailServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/CloudWatch-CrossAccountAccess")]
    CloudWatchCrossAccountAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/CloudWatchActionsEC2Access")]
    CloudWatchActionsEC2Access,
    #[serde(rename = "arn:aws:iam::aws:policy/CloudWatchAgentAdminPolicy")]
    CloudWatchAgentAdminPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/CloudWatchAgentServerPolicy")]
    CloudWatchAgentServerPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/CloudWatchApplicationInsightsFullAccess")]
    CloudWatchApplicationInsightsFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/CloudWatchApplicationInsightsReadOnlyAccess")]
    CloudWatchApplicationInsightsReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/CloudWatchApplicationSignalsServiceRolePolicy")]
    CloudWatchApplicationSignalsServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/CloudWatchAutomaticDashboardsAccess")]
    CloudWatchAutomaticDashboardsAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/CloudWatchCrossAccountSharingConfiguration")]
    CloudWatchCrossAccountSharingConfiguration,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/CloudWatchEventsBuiltInTargetExecutionAccess")]
    CloudWatchEventsBuiltInTargetExecutionAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/CloudWatchEventsFullAccess")]
    CloudWatchEventsFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/CloudWatchEventsInvocationAccess")]
    CloudWatchEventsInvocationAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/CloudWatchEventsReadOnlyAccess")]
    CloudWatchEventsReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/CloudWatchEventsServiceRolePolicy")]
    CloudWatchEventsServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/CloudWatchFullAccess")]
    CloudWatchFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/CloudWatchFullAccessV2")]
    CloudWatchFullAccessV2,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/CloudWatchInternetMonitorServiceRolePolicy")]
    CloudWatchInternetMonitorServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/CloudWatchLambdaInsightsExecutionRolePolicy")]
    CloudWatchLambdaInsightsExecutionRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/CloudWatchLogsCrossAccountSharingConfiguration")]
    CloudWatchLogsCrossAccountSharingConfiguration,
    #[serde(rename = "arn:aws:iam::aws:policy/CloudWatchLogsFullAccess")]
    CloudWatchLogsFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/CloudWatchLogsReadOnlyAccess")]
    CloudWatchLogsReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/CloudWatchNetworkMonitorServiceRolePolicy")]
    CloudWatchNetworkMonitorServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/CloudWatchReadOnlyAccess")]
    CloudWatchReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/CloudWatchSyntheticsFullAccess")]
    CloudWatchSyntheticsFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/CloudWatchSyntheticsReadOnlyAccess")]
    CloudWatchSyntheticsReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/CloudwatchApplicationInsightsServiceLinkedRolePolicy")]
    CloudwatchApplicationInsightsServiceLinkedRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/ComprehendDataAccessRolePolicy")]
    ComprehendDataAccessRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/ComprehendFullAccess")]
    ComprehendFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/ComprehendMedicalFullAccess")]
    ComprehendMedicalFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/ComprehendReadOnly")]
    ComprehendReadOnly,
    #[serde(rename = "arn:aws:iam::aws:policy/ComputeOptimizerReadOnlyAccess")]
    ComputeOptimizerReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/ComputeOptimizerServiceRolePolicy")]
    ComputeOptimizerServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/ConfigConformsServiceRolePolicy")]
    ConfigConformsServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/CostOptimizationHubAdminAccess")]
    CostOptimizationHubAdminAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/CostOptimizationHubReadOnlyAccess")]
    CostOptimizationHubReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/CostOptimizationHubServiceRolePolicy")]
    CostOptimizationHubServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/CustomerProfilesServiceLinkedRolePolicy")]
    CustomerProfilesServiceLinkedRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/DAXServiceRolePolicy")]
    DAXServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/job-function/DataScientist")]
    DataScientist,
    #[serde(rename = "arn:aws:iam::aws:policy/job-function/DatabaseAdministrator")]
    DatabaseAdministrator,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/DynamoDBCloudWatchContributorInsightsServiceRolePolicy")]
    DynamoDBCloudWatchContributorInsightsServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/DynamoDBKinesisReplicationServiceRolePolicy")]
    DynamoDBKinesisReplicationServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/DynamoDBReplicationServiceRolePolicy")]
    DynamoDBReplicationServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/EC2FastLaunchServiceRolePolicy")]
    EC2FastLaunchServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/EC2FleetTimeShiftableServiceRolePolicy")]
    EC2FleetTimeShiftableServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/EC2ImageBuilderLifecycleExecutionPolicy")]
    EC2ImageBuilderLifecycleExecutionPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/EC2InstanceConnect")]
    EC2InstanceConnect,
    #[serde(rename = "arn:aws:iam::aws:policy/EC2InstanceProfileForImageBuilder")]
    EC2InstanceProfileForImageBuilder,
    #[serde(rename = "arn:aws:iam::aws:policy/EC2InstanceProfileForImageBuilderECRContainerBuilds")]
    EC2InstanceProfileForImageBuilderECRContainerBuilds,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/ECRReplicationServiceRolePolicy")]
    ECRReplicationServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/EMRDescribeClusterPolicyForEMRWAL")]
    EMRDescribeClusterPolicyForEMRWAL,
    #[serde(rename = "arn:aws:iam::aws:policy/Ec2ImageBuilderCrossAccountDistributionAccess")]
    Ec2ImageBuilderCrossAccountDistributionAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/Ec2InstanceConnectEndpoint")]
    Ec2InstanceConnectEndpoint,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/ElastiCacheServiceRolePolicy")]
    ElastiCacheServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/ElasticLoadBalancingFullAccess")]
    ElasticLoadBalancingFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/ElasticLoadBalancingReadOnly")]
    ElasticLoadBalancingReadOnly,
    #[serde(rename = "arn:aws:iam::aws:policy/ElementalActivationsDownloadSoftwareAccess")]
    ElementalActivationsDownloadSoftwareAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/ElementalActivationsFullAccess")]
    ElementalActivationsFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/ElementalActivationsGenerateLicenses")]
    ElementalActivationsGenerateLicenses,
    #[serde(rename = "arn:aws:iam::aws:policy/ElementalActivationsReadOnlyAccess")]
    ElementalActivationsReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/ElementalAppliancesSoftwareFullAccess")]
    ElementalAppliancesSoftwareFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/ElementalAppliancesSoftwareReadOnlyAccess")]
    ElementalAppliancesSoftwareReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/ElementalSupportCenterFullAccess")]
    ElementalSupportCenterFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/FMSServiceRolePolicy")]
    FMSServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/FSxDeleteServiceLinkedRoleAccess")]
    FSxDeleteServiceLinkedRoleAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/FusionDevInternalServiceRolePolicy")]
    FusionDevInternalServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/GameLiftGameServerGroupPolicy")]
    GameLiftGameServerGroupPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/GlobalAcceleratorFullAccess")]
    GlobalAcceleratorFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/GlobalAcceleratorReadOnlyAccess")]
    GlobalAcceleratorReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/GreengrassOTAUpdateArtifactAccess")]
    GreengrassOTAUpdateArtifactAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/GroundTruthSyntheticConsoleFullAccess")]
    GroundTruthSyntheticConsoleFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/GroundTruthSyntheticConsoleReadOnlyAccess")]
    GroundTruthSyntheticConsoleReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/Health_OrganizationsServiceRolePolicy")]
    Health_OrganizationsServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/IAMAccessAdvisorReadOnly")]
    IAMAccessAdvisorReadOnly,
    #[serde(rename = "arn:aws:iam::aws:policy/IAMAccessAnalyzerFullAccess")]
    IAMAccessAnalyzerFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/IAMAccessAnalyzerReadOnlyAccess")]
    IAMAccessAnalyzerReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/IAMFullAccess")]
    IAMFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/IAMReadOnlyAccess")]
    IAMReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/IAMSelfManageServiceSpecificCredentials")]
    IAMSelfManageServiceSpecificCredentials,
    #[serde(rename = "arn:aws:iam::aws:policy/IAMUserChangePassword")]
    IAMUserChangePassword,
    #[serde(rename = "arn:aws:iam::aws:policy/IAMUserSSHKeys")]
    IAMUserSSHKeys,
    #[serde(rename = "arn:aws:iam::aws:policy/IVSFullAccess")]
    IVSFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/IVSReadOnlyAccess")]
    IVSReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/IVSRecordToS3")]
    IVSRecordToS3,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/KafkaConnectServiceRolePolicy")]
    KafkaConnectServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/KafkaServiceRolePolicy")]
    KafkaServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/KeyspacesReplicationServiceRolePolicy")]
    KeyspacesReplicationServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/LakeFormationDataAccessServiceRolePolicy")]
    LakeFormationDataAccessServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/LexBotPolicy")]
    LexBotPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/LexChannelPolicy")]
    LexChannelPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/LightsailExportAccess")]
    LightsailExportAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/MediaConnectGatewayInstanceRolePolicy")]
    MediaConnectGatewayInstanceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/MediaPackageServiceRolePolicy")]
    MediaPackageServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/MemoryDBServiceRolePolicy")]
    MemoryDBServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/MigrationHubDMSAccessServiceRolePolicy")]
    MigrationHubDMSAccessServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/MigrationHubSMSAccessServiceRolePolicy")]
    MigrationHubSMSAccessServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/MigrationHubServiceRolePolicy")]
    MigrationHubServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/MonitronServiceRolePolicy")]
    MonitronServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/NeptuneConsoleFullAccess")]
    NeptuneConsoleFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/NeptuneFullAccess")]
    NeptuneFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/NeptuneGraphReadOnlyAccess")]
    NeptuneGraphReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/NeptuneReadOnlyAccess")]
    NeptuneReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/job-function/NetworkAdministrator")]
    NetworkAdministrator,
    #[serde(rename = "arn:aws:iam::aws:policy/OAMFullAccess")]
    OAMFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/OAMReadOnlyAccess")]
    OAMReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/PartnerCentralAccountManagementUserRoleAssociation")]
    PartnerCentralAccountManagementUserRoleAssociation,
    #[serde(rename = "arn:aws:iam::aws:policy/PowerUserAccess")]
    PowerUserAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/QuickSightAccessForS3StorageManagementAnalyticsReadOnly")]
    QuickSightAccessForS3StorageManagementAnalyticsReadOnly,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/RDSCloudHsmAuthorizationRole")]
    RDSCloudHsmAuthorizationRole,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/ROSAAmazonEBSCSIDriverOperatorPolicy")]
    ROSAAmazonEBSCSIDriverOperatorPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/ROSACloudNetworkConfigOperatorPolicy")]
    ROSACloudNetworkConfigOperatorPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/ROSAControlPlaneOperatorPolicy")]
    ROSAControlPlaneOperatorPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/ROSAImageRegistryOperatorPolicy")]
    ROSAImageRegistryOperatorPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/ROSAIngressOperatorPolicy")]
    ROSAIngressOperatorPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/ROSAInstallerPolicy")]
    ROSAInstallerPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/ROSAKMSProviderPolicy")]
    ROSAKMSProviderPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/ROSAKubeControllerPolicy")]
    ROSAKubeControllerPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/ROSAManageSubscription")]
    ROSAManageSubscription,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/ROSANodePoolManagementPolicy")]
    ROSANodePoolManagementPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/ROSASRESupportPolicy")]
    ROSASRESupportPolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/ROSAWorkerInstancePolicy")]
    ROSAWorkerInstancePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/ReadOnlyAccess")]
    ReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/ResourceGroupsServiceRolePolicy")]
    ResourceGroupsServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/ResourceGroupsandTagEditorFullAccess")]
    ResourceGroupsandTagEditorFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/ResourceGroupsandTagEditorReadOnlyAccess")]
    ResourceGroupsandTagEditorReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/Route53RecoveryReadinessServiceRolePolicy")]
    Route53RecoveryReadinessServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/Route53ResolverServiceRolePolicy")]
    Route53ResolverServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/S3StorageLensServiceRolePolicy")]
    S3StorageLensServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/SecretsManagerReadWrite")]
    SecretsManagerReadWrite,
    #[serde(rename = "arn:aws:iam::aws:policy/SecurityAudit")]
    SecurityAudit,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/SecurityLakeServiceLinkedRole")]
    SecurityLakeServiceLinkedRole,
    #[serde(rename = "arn:aws:iam::aws:policy/ServerMigrationConnector")]
    ServerMigrationConnector,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/ServerMigrationServiceRole")]
    ServerMigrationServiceRole,
    #[serde(rename = "arn:aws:iam::aws:policy/ServiceCatalogAdminFullAccess")]
    ServiceCatalogAdminFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/ServiceCatalogAdminReadOnlyAccess")]
    ServiceCatalogAdminReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/ServiceCatalogEndUserAccess")]
    ServiceCatalogEndUserAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/ServiceCatalogEndUserFullAccess")]
    ServiceCatalogEndUserFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/ServerMigrationServiceConsoleFullAccess")]
    ServerMigrationServiceConsoleFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/ServerMigrationServiceLaunchRole")]
    ServerMigrationServiceLaunchRole,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/ServerMigrationServiceRoleForInstanceValidation")]
    ServerMigrationServiceRoleForInstanceValidation,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/ServerMigration_ServiceRole")]
    AWSServerMigration_ServiceRole,
    #[serde(rename = "arn:aws:iam::aws:policy/ServiceQuotasFullAccess")]
    ServiceQuotasFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/ServiceQuotasReadOnlyAccess")]
    ServiceQuotasReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/ServiceQuotasServiceRolePolicy")]
    ServiceQuotasServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/SimpleWorkflowFullAccess")]
    SimpleWorkflowFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/job-function/SupportUser")]
    SupportUser,
    #[serde(rename = "arn:aws:iam::aws:policy/job-function/SystemAdministrator")]
    SystemAdministrator,
    #[serde(rename = "arn:aws:iam::aws:policy/TranslateFullAccess")]
    TranslateFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/TranslateReadOnly")]
    TranslateReadOnly,
    #[serde(rename = "arn:aws:iam::aws:policy/service-role/VMImportExportRoleForAWSConnector")]
    VMImportExportRoleForAWSConnector,
    #[serde(rename = "arn:aws:iam::aws:policy/VPCLatticeFullAccess")]
    VPCLatticeFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/VPCLatticeReadOnlyAccess")]
    VPCLatticeReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/VPCLatticeServicesInvokeAccess")]
    VPCLatticeServicesInvokeAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/job-function/ViewOnlyAccess")]
    ViewOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/WAFLoggingServiceRolePolicy")]
    WAFLoggingServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/WAFRegionalLoggingServiceRolePolicy")]
    WAFRegionalLoggingServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/aws-service-role/WAFV2LoggingServiceRolePolicy")]
    WAFV2LoggingServiceRolePolicy,
    #[serde(rename = "arn:aws:iam::aws:policy/WellArchitectedConsoleFullAccess")]
    WellArchitectedConsoleFullAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/WellArchitectedConsoleReadOnlyAccess")]
    WellArchitectedConsoleReadOnlyAccess,
    #[serde(rename = "arn:aws:iam::aws:policy/WorkLinkServiceRolePolicy")]
    WorkLinkServiceRolePolicy,
}
