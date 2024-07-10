use anyhow::Error;

use pulumi_wasm_docker::resource::container;
use pulumi_wasm_docker::resource::container::container;
use pulumi_wasm_docker::resource::image;
use pulumi_wasm_docker::types::DockerBuild;
use pulumi_wasm_rust::{add_export, pulumi_main};

#[pulumi_main]
fn test_main() -> Result<(), Error> {
    let cont = container(
        "container",
        container::ContainerArgs {
            attach: true.into(),
            capabilities: None.into(),
            cgroupns_mode: None.into(),
            command: vec!["echo".to_string(), "Hello World!".to_string()].into(),
            container_read_refresh_timeout_milliseconds: None.into(),
            cpu_set: None.into(),
            cpu_shares: None.into(),
            destroy_grace_seconds: None.into(),
            devices: None.into(),
            dns: None.into(),
            dns_opts: None.into(),
            dns_searches: None.into(),
            domainname: None.into(),
            entrypoints: None.into(),
            envs: None.into(),
            gpus: None.into(),
            group_adds: None.into(),
            healthcheck: None.into(),
            hostname: None.into(),
            hosts: None.into(),
            image: "ubuntu".to_string().into(),
            init: None.into(),
            ipc_mode: None.into(),
            labels: None.into(),
            log_driver: None.into(),
            log_opts: None.into(),
            logs: true.into(),
            max_retry_count: None.into(),
            memory: None.into(),
            memory_swap: None.into(),
            mounts: None.into(),
            must_run: false.into(),
            name: None.into(),
            network_mode: None.into(),
            networks_advanced: None.into(),
            pid_mode: None.into(),
            ports: None.into(),
            privileged: None.into(),
            publish_all_ports: None.into(),
            read_only: None.into(),
            remove_volumes: None.into(),
            restart: None.into(),
            rm: None.into(),
            runtime: None.into(),
            security_opts: None.into(),
            shm_size: None.into(),
            start: None.into(),
            stdin_open: None.into(),
            stop_signal: None.into(),
            stop_timeout: None.into(),
            storage_opts: None.into(),
            sysctls: None.into(),
            tmpfs: None.into(),
            tty: None.into(),
            ulimits: None.into(),
            uploads: None.into(),
            user: None.into(),
            userns_mode: None.into(),
            volumes: None.into(),
            wait: None.into(),
            wait_timeout: None.into(),
            working_dir: None.into(),
        },
    );

    let image = image::image(
        "image",
        image::ImageArgs {
            build: DockerBuild {
                add_hosts: None.into(),
                args: None.into(),
                builder_version: None.into(),
                cache_from: None.into(),
                context: Some("docker/".to_string()).into(),
                dockerfile: None.into(),
                network: None.into(),
                platform: None.into(),
                target: None.into(),
            }
            .into(),
            build_on_preview: None.into(),
            image_name: "image:test".to_string().into(),
            registry: None.into(),
            skip_push: true.into(),
        },
    );

    add_export("logs", &cont.container_logs);
    add_export("image_id", &image.image_name);
    Ok(())
}
