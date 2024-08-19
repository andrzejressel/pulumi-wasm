use bindings::exports::pulumi::docker::container;
use bindings::exports::pulumi::docker::image;
use bindings::exports::pulumi::docker::network;
use bindings::exports::pulumi::docker::plugin;
use bindings::exports::pulumi::docker::registry_image;
use bindings::exports::pulumi::docker::remote_image;
use bindings::exports::pulumi::docker::secret;
use bindings::exports::pulumi::docker::service;
use bindings::exports::pulumi::docker::service_config;
use bindings::exports::pulumi::docker::tag;
use bindings::exports::pulumi::docker::volume;
use std::collections::HashMap;

use crate::bindings::component::pulumi_wasm::register_interface::{
    register, ObjectField, RegisterResourceRequest, ResultField,
};
mod bindings;
bindings::export!(Component with_types_in bindings);

struct Component {}

impl container::Guest for Component {
    fn invoke(name: String, args: container::Args) -> container::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/container:Container".into(),
            name,
            object: vec![
                ObjectField {
                    name: "attach".into(),
                    value: args.attach,
                },
                ObjectField {
                    name: "capabilities".into(),
                    value: args.capabilities,
                },
                ObjectField {
                    name: "cgroupnsMode".into(),
                    value: args.cgroupns_mode,
                },
                ObjectField {
                    name: "command".into(),
                    value: args.command,
                },
                ObjectField {
                    name: "containerReadRefreshTimeoutMilliseconds".into(),
                    value: args.container_read_refresh_timeout_milliseconds,
                },
                ObjectField {
                    name: "cpuSet".into(),
                    value: args.cpu_set,
                },
                ObjectField {
                    name: "cpuShares".into(),
                    value: args.cpu_shares,
                },
                ObjectField {
                    name: "destroyGraceSeconds".into(),
                    value: args.destroy_grace_seconds,
                },
                ObjectField {
                    name: "devices".into(),
                    value: args.devices,
                },
                ObjectField {
                    name: "dns".into(),
                    value: args.dns,
                },
                ObjectField {
                    name: "dnsOpts".into(),
                    value: args.dns_opts,
                },
                ObjectField {
                    name: "dnsSearches".into(),
                    value: args.dns_searches,
                },
                ObjectField {
                    name: "domainname".into(),
                    value: args.domainname,
                },
                ObjectField {
                    name: "entrypoints".into(),
                    value: args.entrypoints,
                },
                ObjectField {
                    name: "envs".into(),
                    value: args.envs,
                },
                ObjectField {
                    name: "gpus".into(),
                    value: args.gpus,
                },
                ObjectField {
                    name: "groupAdds".into(),
                    value: args.group_adds,
                },
                ObjectField {
                    name: "healthcheck".into(),
                    value: args.healthcheck,
                },
                ObjectField {
                    name: "hostname".into(),
                    value: args.hostname,
                },
                ObjectField {
                    name: "hosts".into(),
                    value: args.hosts,
                },
                ObjectField {
                    name: "image".into(),
                    value: args.image,
                },
                ObjectField {
                    name: "init".into(),
                    value: args.init,
                },
                ObjectField {
                    name: "ipcMode".into(),
                    value: args.ipc_mode,
                },
                ObjectField {
                    name: "labels".into(),
                    value: args.labels,
                },
                ObjectField {
                    name: "logDriver".into(),
                    value: args.log_driver,
                },
                ObjectField {
                    name: "logOpts".into(),
                    value: args.log_opts,
                },
                ObjectField {
                    name: "logs".into(),
                    value: args.logs,
                },
                ObjectField {
                    name: "maxRetryCount".into(),
                    value: args.max_retry_count,
                },
                ObjectField {
                    name: "memory".into(),
                    value: args.memory,
                },
                ObjectField {
                    name: "memorySwap".into(),
                    value: args.memory_swap,
                },
                ObjectField {
                    name: "mounts".into(),
                    value: args.mounts,
                },
                ObjectField {
                    name: "mustRun".into(),
                    value: args.must_run,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "networkMode".into(),
                    value: args.network_mode,
                },
                ObjectField {
                    name: "networksAdvanced".into(),
                    value: args.networks_advanced,
                },
                ObjectField {
                    name: "pidMode".into(),
                    value: args.pid_mode,
                },
                ObjectField {
                    name: "ports".into(),
                    value: args.ports,
                },
                ObjectField {
                    name: "privileged".into(),
                    value: args.privileged,
                },
                ObjectField {
                    name: "publishAllPorts".into(),
                    value: args.publish_all_ports,
                },
                ObjectField {
                    name: "readOnly".into(),
                    value: args.read_only,
                },
                ObjectField {
                    name: "removeVolumes".into(),
                    value: args.remove_volumes,
                },
                ObjectField {
                    name: "restart".into(),
                    value: args.restart,
                },
                ObjectField {
                    name: "rm".into(),
                    value: args.rm,
                },
                ObjectField {
                    name: "runtime".into(),
                    value: args.runtime,
                },
                ObjectField {
                    name: "securityOpts".into(),
                    value: args.security_opts,
                },
                ObjectField {
                    name: "shmSize".into(),
                    value: args.shm_size,
                },
                ObjectField {
                    name: "start".into(),
                    value: args.start,
                },
                ObjectField {
                    name: "stdinOpen".into(),
                    value: args.stdin_open,
                },
                ObjectField {
                    name: "stopSignal".into(),
                    value: args.stop_signal,
                },
                ObjectField {
                    name: "stopTimeout".into(),
                    value: args.stop_timeout,
                },
                ObjectField {
                    name: "storageOpts".into(),
                    value: args.storage_opts,
                },
                ObjectField {
                    name: "sysctls".into(),
                    value: args.sysctls,
                },
                ObjectField {
                    name: "tmpfs".into(),
                    value: args.tmpfs,
                },
                ObjectField {
                    name: "tty".into(),
                    value: args.tty,
                },
                ObjectField {
                    name: "ulimits".into(),
                    value: args.ulimits,
                },
                ObjectField {
                    name: "uploads".into(),
                    value: args.uploads,
                },
                ObjectField {
                    name: "user".into(),
                    value: args.user,
                },
                ObjectField {
                    name: "usernsMode".into(),
                    value: args.userns_mode,
                },
                ObjectField {
                    name: "volumes".into(),
                    value: args.volumes,
                },
                ObjectField {
                    name: "wait".into(),
                    value: args.wait,
                },
                ObjectField {
                    name: "waitTimeout".into(),
                    value: args.wait_timeout,
                },
                ObjectField {
                    name: "workingDir".into(),
                    value: args.working_dir,
                },
            ],
            results: vec![
                ResultField {
                    name: "attach".into(),
                },
                ResultField {
                    name: "bridge".into(),
                },
                ResultField {
                    name: "capabilities".into(),
                },
                ResultField {
                    name: "cgroupnsMode".into(),
                },
                ResultField {
                    name: "command".into(),
                },
                ResultField {
                    name: "containerLogs".into(),
                },
                ResultField {
                    name: "containerReadRefreshTimeoutMilliseconds".into(),
                },
                ResultField {
                    name: "cpuSet".into(),
                },
                ResultField {
                    name: "cpuShares".into(),
                },
                ResultField {
                    name: "destroyGraceSeconds".into(),
                },
                ResultField {
                    name: "devices".into(),
                },
                ResultField { name: "dns".into() },
                ResultField {
                    name: "dnsOpts".into(),
                },
                ResultField {
                    name: "dnsSearches".into(),
                },
                ResultField {
                    name: "domainname".into(),
                },
                ResultField {
                    name: "entrypoints".into(),
                },
                ResultField {
                    name: "envs".into(),
                },
                ResultField {
                    name: "exitCode".into(),
                },
                ResultField {
                    name: "gpus".into(),
                },
                ResultField {
                    name: "groupAdds".into(),
                },
                ResultField {
                    name: "healthcheck".into(),
                },
                ResultField {
                    name: "hostname".into(),
                },
                ResultField {
                    name: "hosts".into(),
                },
                ResultField {
                    name: "image".into(),
                },
                ResultField {
                    name: "init".into(),
                },
                ResultField {
                    name: "ipcMode".into(),
                },
                ResultField {
                    name: "labels".into(),
                },
                ResultField {
                    name: "logDriver".into(),
                },
                ResultField {
                    name: "logOpts".into(),
                },
                ResultField {
                    name: "logs".into(),
                },
                ResultField {
                    name: "maxRetryCount".into(),
                },
                ResultField {
                    name: "memory".into(),
                },
                ResultField {
                    name: "memorySwap".into(),
                },
                ResultField {
                    name: "mounts".into(),
                },
                ResultField {
                    name: "mustRun".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "networkDatas".into(),
                },
                ResultField {
                    name: "networkMode".into(),
                },
                ResultField {
                    name: "networksAdvanced".into(),
                },
                ResultField {
                    name: "pidMode".into(),
                },
                ResultField {
                    name: "ports".into(),
                },
                ResultField {
                    name: "privileged".into(),
                },
                ResultField {
                    name: "publishAllPorts".into(),
                },
                ResultField {
                    name: "readOnly".into(),
                },
                ResultField {
                    name: "removeVolumes".into(),
                },
                ResultField {
                    name: "restart".into(),
                },
                ResultField { name: "rm".into() },
                ResultField {
                    name: "runtime".into(),
                },
                ResultField {
                    name: "securityOpts".into(),
                },
                ResultField {
                    name: "shmSize".into(),
                },
                ResultField {
                    name: "start".into(),
                },
                ResultField {
                    name: "stdinOpen".into(),
                },
                ResultField {
                    name: "stopSignal".into(),
                },
                ResultField {
                    name: "stopTimeout".into(),
                },
                ResultField {
                    name: "storageOpts".into(),
                },
                ResultField {
                    name: "sysctls".into(),
                },
                ResultField {
                    name: "tmpfs".into(),
                },
                ResultField { name: "tty".into() },
                ResultField {
                    name: "ulimits".into(),
                },
                ResultField {
                    name: "uploads".into(),
                },
                ResultField {
                    name: "user".into(),
                },
                ResultField {
                    name: "usernsMode".into(),
                },
                ResultField {
                    name: "volumes".into(),
                },
                ResultField {
                    name: "wait".into(),
                },
                ResultField {
                    name: "waitTimeout".into(),
                },
                ResultField {
                    name: "workingDir".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        container::Res {
            attach: hashmap.remove("attach").unwrap(),
            bridge: hashmap.remove("bridge").unwrap(),
            capabilities: hashmap.remove("capabilities").unwrap(),
            cgroupns_mode: hashmap.remove("cgroupnsMode").unwrap(),
            command: hashmap.remove("command").unwrap(),
            container_logs: hashmap.remove("containerLogs").unwrap(),
            container_read_refresh_timeout_milliseconds: hashmap
                .remove("containerReadRefreshTimeoutMilliseconds")
                .unwrap(),
            cpu_set: hashmap.remove("cpuSet").unwrap(),
            cpu_shares: hashmap.remove("cpuShares").unwrap(),
            destroy_grace_seconds: hashmap.remove("destroyGraceSeconds").unwrap(),
            devices: hashmap.remove("devices").unwrap(),
            dns: hashmap.remove("dns").unwrap(),
            dns_opts: hashmap.remove("dnsOpts").unwrap(),
            dns_searches: hashmap.remove("dnsSearches").unwrap(),
            domainname: hashmap.remove("domainname").unwrap(),
            entrypoints: hashmap.remove("entrypoints").unwrap(),
            envs: hashmap.remove("envs").unwrap(),
            exit_code: hashmap.remove("exitCode").unwrap(),
            gpus: hashmap.remove("gpus").unwrap(),
            group_adds: hashmap.remove("groupAdds").unwrap(),
            healthcheck: hashmap.remove("healthcheck").unwrap(),
            hostname: hashmap.remove("hostname").unwrap(),
            hosts: hashmap.remove("hosts").unwrap(),
            image: hashmap.remove("image").unwrap(),
            init: hashmap.remove("init").unwrap(),
            ipc_mode: hashmap.remove("ipcMode").unwrap(),
            labels: hashmap.remove("labels").unwrap(),
            log_driver: hashmap.remove("logDriver").unwrap(),
            log_opts: hashmap.remove("logOpts").unwrap(),
            logs: hashmap.remove("logs").unwrap(),
            max_retry_count: hashmap.remove("maxRetryCount").unwrap(),
            memory: hashmap.remove("memory").unwrap(),
            memory_swap: hashmap.remove("memorySwap").unwrap(),
            mounts: hashmap.remove("mounts").unwrap(),
            must_run: hashmap.remove("mustRun").unwrap(),
            name: hashmap.remove("name").unwrap(),
            network_datas: hashmap.remove("networkDatas").unwrap(),
            network_mode: hashmap.remove("networkMode").unwrap(),
            networks_advanced: hashmap.remove("networksAdvanced").unwrap(),
            pid_mode: hashmap.remove("pidMode").unwrap(),
            ports: hashmap.remove("ports").unwrap(),
            privileged: hashmap.remove("privileged").unwrap(),
            publish_all_ports: hashmap.remove("publishAllPorts").unwrap(),
            read_only: hashmap.remove("readOnly").unwrap(),
            remove_volumes: hashmap.remove("removeVolumes").unwrap(),
            restart: hashmap.remove("restart").unwrap(),
            rm: hashmap.remove("rm").unwrap(),
            runtime: hashmap.remove("runtime").unwrap(),
            security_opts: hashmap.remove("securityOpts").unwrap(),
            shm_size: hashmap.remove("shmSize").unwrap(),
            start: hashmap.remove("start").unwrap(),
            stdin_open: hashmap.remove("stdinOpen").unwrap(),
            stop_signal: hashmap.remove("stopSignal").unwrap(),
            stop_timeout: hashmap.remove("stopTimeout").unwrap(),
            storage_opts: hashmap.remove("storageOpts").unwrap(),
            sysctls: hashmap.remove("sysctls").unwrap(),
            tmpfs: hashmap.remove("tmpfs").unwrap(),
            tty: hashmap.remove("tty").unwrap(),
            ulimits: hashmap.remove("ulimits").unwrap(),
            uploads: hashmap.remove("uploads").unwrap(),
            user: hashmap.remove("user").unwrap(),
            userns_mode: hashmap.remove("usernsMode").unwrap(),
            volumes: hashmap.remove("volumes").unwrap(),
            wait: hashmap.remove("wait").unwrap(),
            wait_timeout: hashmap.remove("waitTimeout").unwrap(),
            working_dir: hashmap.remove("workingDir").unwrap(),
        }
    }
}
impl image::Guest for Component {
    fn invoke(name: String, args: image::Args) -> image::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/image:Image".into(),
            name,
            object: vec![
                ObjectField {
                    name: "build".into(),
                    value: args.build,
                },
                ObjectField {
                    name: "buildOnPreview".into(),
                    value: args.build_on_preview,
                },
                ObjectField {
                    name: "imageName".into(),
                    value: args.image_name,
                },
                ObjectField {
                    name: "registry".into(),
                    value: args.registry,
                },
                ObjectField {
                    name: "skipPush".into(),
                    value: args.skip_push,
                },
            ],
            results: vec![
                ResultField {
                    name: "baseImageName".into(),
                },
                ResultField {
                    name: "context".into(),
                },
                ResultField {
                    name: "dockerfile".into(),
                },
                ResultField {
                    name: "imageName".into(),
                },
                ResultField {
                    name: "platform".into(),
                },
                ResultField {
                    name: "registryServer".into(),
                },
                ResultField {
                    name: "repoDigest".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        image::Res {
            base_image_name: hashmap.remove("baseImageName").unwrap(),
            context: hashmap.remove("context").unwrap(),
            dockerfile: hashmap.remove("dockerfile").unwrap(),
            image_name: hashmap.remove("imageName").unwrap(),
            platform: hashmap.remove("platform").unwrap(),
            registry_server: hashmap.remove("registryServer").unwrap(),
            repo_digest: hashmap.remove("repoDigest").unwrap(),
        }
    }
}
impl network::Guest for Component {
    fn invoke(name: String, args: network::Args) -> network::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/network:Network".into(),
            name,
            object: vec![
                ObjectField {
                    name: "attachable".into(),
                    value: args.attachable,
                },
                ObjectField {
                    name: "checkDuplicate".into(),
                    value: args.check_duplicate,
                },
                ObjectField {
                    name: "driver".into(),
                    value: args.driver,
                },
                ObjectField {
                    name: "ingress".into(),
                    value: args.ingress,
                },
                ObjectField {
                    name: "internal".into(),
                    value: args.internal,
                },
                ObjectField {
                    name: "ipamConfigs".into(),
                    value: args.ipam_configs,
                },
                ObjectField {
                    name: "ipamDriver".into(),
                    value: args.ipam_driver,
                },
                ObjectField {
                    name: "ipamOptions".into(),
                    value: args.ipam_options,
                },
                ObjectField {
                    name: "ipv6".into(),
                    value: args.ipv6,
                },
                ObjectField {
                    name: "labels".into(),
                    value: args.labels,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "options".into(),
                    value: args.options,
                },
            ],
            results: vec![
                ResultField {
                    name: "attachable".into(),
                },
                ResultField {
                    name: "checkDuplicate".into(),
                },
                ResultField {
                    name: "driver".into(),
                },
                ResultField {
                    name: "ingress".into(),
                },
                ResultField {
                    name: "internal".into(),
                },
                ResultField {
                    name: "ipamConfigs".into(),
                },
                ResultField {
                    name: "ipamDriver".into(),
                },
                ResultField {
                    name: "ipamOptions".into(),
                },
                ResultField {
                    name: "ipv6".into(),
                },
                ResultField {
                    name: "labels".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "options".into(),
                },
                ResultField {
                    name: "scope".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        network::Res {
            attachable: hashmap.remove("attachable").unwrap(),
            check_duplicate: hashmap.remove("checkDuplicate").unwrap(),
            driver: hashmap.remove("driver").unwrap(),
            ingress: hashmap.remove("ingress").unwrap(),
            internal: hashmap.remove("internal").unwrap(),
            ipam_configs: hashmap.remove("ipamConfigs").unwrap(),
            ipam_driver: hashmap.remove("ipamDriver").unwrap(),
            ipam_options: hashmap.remove("ipamOptions").unwrap(),
            ipv6: hashmap.remove("ipv6").unwrap(),
            labels: hashmap.remove("labels").unwrap(),
            name: hashmap.remove("name").unwrap(),
            options: hashmap.remove("options").unwrap(),
            scope: hashmap.remove("scope").unwrap(),
        }
    }
}
impl plugin::Guest for Component {
    fn invoke(name: String, args: plugin::Args) -> plugin::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/plugin:Plugin".into(),
            name,
            object: vec![
                ObjectField {
                    name: "alias".into(),
                    value: args.alias,
                },
                ObjectField {
                    name: "enableTimeout".into(),
                    value: args.enable_timeout,
                },
                ObjectField {
                    name: "enabled".into(),
                    value: args.enabled,
                },
                ObjectField {
                    name: "envs".into(),
                    value: args.envs,
                },
                ObjectField {
                    name: "forceDestroy".into(),
                    value: args.force_destroy,
                },
                ObjectField {
                    name: "forceDisable".into(),
                    value: args.force_disable,
                },
                ObjectField {
                    name: "grantAllPermissions".into(),
                    value: args.grant_all_permissions,
                },
                ObjectField {
                    name: "grantPermissions".into(),
                    value: args.grant_permissions,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
            ],
            results: vec![
                ResultField {
                    name: "alias".into(),
                },
                ResultField {
                    name: "enableTimeout".into(),
                },
                ResultField {
                    name: "enabled".into(),
                },
                ResultField {
                    name: "envs".into(),
                },
                ResultField {
                    name: "forceDestroy".into(),
                },
                ResultField {
                    name: "forceDisable".into(),
                },
                ResultField {
                    name: "grantAllPermissions".into(),
                },
                ResultField {
                    name: "grantPermissions".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "pluginReference".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        plugin::Res {
            alias: hashmap.remove("alias").unwrap(),
            enable_timeout: hashmap.remove("enableTimeout").unwrap(),
            enabled: hashmap.remove("enabled").unwrap(),
            envs: hashmap.remove("envs").unwrap(),
            force_destroy: hashmap.remove("forceDestroy").unwrap(),
            force_disable: hashmap.remove("forceDisable").unwrap(),
            grant_all_permissions: hashmap.remove("grantAllPermissions").unwrap(),
            grant_permissions: hashmap.remove("grantPermissions").unwrap(),
            name: hashmap.remove("name").unwrap(),
            plugin_reference: hashmap.remove("pluginReference").unwrap(),
        }
    }
}
impl registry_image::Guest for Component {
    fn invoke(name: String, args: registry_image::Args) -> registry_image::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/registryImage:RegistryImage".into(),
            name,
            object: vec![
                ObjectField {
                    name: "insecureSkipVerify".into(),
                    value: args.insecure_skip_verify,
                },
                ObjectField {
                    name: "keepRemotely".into(),
                    value: args.keep_remotely,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "triggers".into(),
                    value: args.triggers,
                },
            ],
            results: vec![
                ResultField {
                    name: "insecureSkipVerify".into(),
                },
                ResultField {
                    name: "keepRemotely".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "sha256Digest".into(),
                },
                ResultField {
                    name: "triggers".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        registry_image::Res {
            insecure_skip_verify: hashmap.remove("insecureSkipVerify").unwrap(),
            keep_remotely: hashmap.remove("keepRemotely").unwrap(),
            name: hashmap.remove("name").unwrap(),
            sha256_digest: hashmap.remove("sha256Digest").unwrap(),
            triggers: hashmap.remove("triggers").unwrap(),
        }
    }
}
impl remote_image::Guest for Component {
    fn invoke(name: String, args: remote_image::Args) -> remote_image::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/remoteImage:RemoteImage".into(),
            name,
            object: vec![
                ObjectField {
                    name: "build".into(),
                    value: args.build,
                },
                ObjectField {
                    name: "forceRemove".into(),
                    value: args.force_remove,
                },
                ObjectField {
                    name: "keepLocally".into(),
                    value: args.keep_locally,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "platform".into(),
                    value: args.platform,
                },
                ObjectField {
                    name: "pullTriggers".into(),
                    value: args.pull_triggers,
                },
                ObjectField {
                    name: "triggers".into(),
                    value: args.triggers,
                },
            ],
            results: vec![
                ResultField {
                    name: "build".into(),
                },
                ResultField {
                    name: "forceRemove".into(),
                },
                ResultField {
                    name: "imageId".into(),
                },
                ResultField {
                    name: "keepLocally".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "platform".into(),
                },
                ResultField {
                    name: "pullTriggers".into(),
                },
                ResultField {
                    name: "repoDigest".into(),
                },
                ResultField {
                    name: "triggers".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        remote_image::Res {
            build: hashmap.remove("build").unwrap(),
            force_remove: hashmap.remove("forceRemove").unwrap(),
            image_id: hashmap.remove("imageId").unwrap(),
            keep_locally: hashmap.remove("keepLocally").unwrap(),
            name: hashmap.remove("name").unwrap(),
            platform: hashmap.remove("platform").unwrap(),
            pull_triggers: hashmap.remove("pullTriggers").unwrap(),
            repo_digest: hashmap.remove("repoDigest").unwrap(),
            triggers: hashmap.remove("triggers").unwrap(),
        }
    }
}
impl secret::Guest for Component {
    fn invoke(name: String, args: secret::Args) -> secret::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/secret:Secret".into(),
            name,
            object: vec![
                ObjectField {
                    name: "data".into(),
                    value: args.data,
                },
                ObjectField {
                    name: "labels".into(),
                    value: args.labels,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
            ],
            results: vec![
                ResultField {
                    name: "data".into(),
                },
                ResultField {
                    name: "labels".into(),
                },
                ResultField {
                    name: "name".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        secret::Res {
            data: hashmap.remove("data").unwrap(),
            labels: hashmap.remove("labels").unwrap(),
            name: hashmap.remove("name").unwrap(),
        }
    }
}
impl service::Guest for Component {
    fn invoke(name: String, args: service::Args) -> service::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/service:Service".into(),
            name,
            object: vec![
                ObjectField {
                    name: "auth".into(),
                    value: args.auth,
                },
                ObjectField {
                    name: "convergeConfig".into(),
                    value: args.converge_config,
                },
                ObjectField {
                    name: "endpointSpec".into(),
                    value: args.endpoint_spec,
                },
                ObjectField {
                    name: "labels".into(),
                    value: args.labels,
                },
                ObjectField {
                    name: "mode".into(),
                    value: args.mode,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
                ObjectField {
                    name: "rollbackConfig".into(),
                    value: args.rollback_config,
                },
                ObjectField {
                    name: "taskSpec".into(),
                    value: args.task_spec,
                },
                ObjectField {
                    name: "updateConfig".into(),
                    value: args.update_config,
                },
            ],
            results: vec![
                ResultField {
                    name: "auth".into(),
                },
                ResultField {
                    name: "convergeConfig".into(),
                },
                ResultField {
                    name: "endpointSpec".into(),
                },
                ResultField {
                    name: "labels".into(),
                },
                ResultField {
                    name: "mode".into(),
                },
                ResultField {
                    name: "name".into(),
                },
                ResultField {
                    name: "rollbackConfig".into(),
                },
                ResultField {
                    name: "taskSpec".into(),
                },
                ResultField {
                    name: "updateConfig".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        service::Res {
            auth: hashmap.remove("auth").unwrap(),
            converge_config: hashmap.remove("convergeConfig").unwrap(),
            endpoint_spec: hashmap.remove("endpointSpec").unwrap(),
            labels: hashmap.remove("labels").unwrap(),
            mode: hashmap.remove("mode").unwrap(),
            name: hashmap.remove("name").unwrap(),
            rollback_config: hashmap.remove("rollbackConfig").unwrap(),
            task_spec: hashmap.remove("taskSpec").unwrap(),
            update_config: hashmap.remove("updateConfig").unwrap(),
        }
    }
}
impl service_config::Guest for Component {
    fn invoke(name: String, args: service_config::Args) -> service_config::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/serviceConfig:ServiceConfig".into(),
            name,
            object: vec![
                ObjectField {
                    name: "data".into(),
                    value: args.data,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
            ],
            results: vec![
                ResultField {
                    name: "data".into(),
                },
                ResultField {
                    name: "name".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        service_config::Res {
            data: hashmap.remove("data").unwrap(),
            name: hashmap.remove("name").unwrap(),
        }
    }
}
impl tag::Guest for Component {
    fn invoke(name: String, args: tag::Args) -> tag::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/tag:Tag".into(),
            name,
            object: vec![
                ObjectField {
                    name: "sourceImage".into(),
                    value: args.source_image,
                },
                ObjectField {
                    name: "targetImage".into(),
                    value: args.target_image,
                },
            ],
            results: vec![
                ResultField {
                    name: "sourceImage".into(),
                },
                ResultField {
                    name: "sourceImageId".into(),
                },
                ResultField {
                    name: "targetImage".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        tag::Res {
            source_image: hashmap.remove("sourceImage").unwrap(),
            source_image_id: hashmap.remove("sourceImageId").unwrap(),
            target_image: hashmap.remove("targetImage").unwrap(),
        }
    }
}
impl volume::Guest for Component {
    fn invoke(name: String, args: volume::Args) -> volume::Res {
        pulumi_wasm_common::setup_logger();
        let request = RegisterResourceRequest {
            type_: "docker:index/volume:Volume".into(),
            name,
            object: vec![
                ObjectField {
                    name: "driver".into(),
                    value: args.driver,
                },
                ObjectField {
                    name: "driverOpts".into(),
                    value: args.driver_opts,
                },
                ObjectField {
                    name: "labels".into(),
                    value: args.labels,
                },
                ObjectField {
                    name: "name".into(),
                    value: args.name,
                },
            ],
            results: vec![
                ResultField {
                    name: "driver".into(),
                },
                ResultField {
                    name: "driverOpts".into(),
                },
                ResultField {
                    name: "labels".into(),
                },
                ResultField {
                    name: "mountpoint".into(),
                },
                ResultField {
                    name: "name".into(),
                },
            ],
        };

        let o = register(&request);

        let mut hashmap: HashMap<String, _> =
            o.fields.into_iter().map(|f| (f.name, f.output)).collect();

        volume::Res {
            driver: hashmap.remove("driver").unwrap(),
            driver_opts: hashmap.remove("driverOpts").unwrap(),
            labels: hashmap.remove("labels").unwrap(),
            mountpoint: hashmap.remove("mountpoint").unwrap(),
            name: hashmap.remove("name").unwrap(),
        }
    }
}
