use anyhow::Error;

use pulumi_wasm_docker::{container, image, DockerBuild};
use pulumi_wasm_rust::{add_export, pulumi_main, Output};

#[pulumi_main]
fn test_main() -> Result<(), Error> {
    let cont = container::create(
        "container",
        container::ContainerArgs {
            attach: true.into(),
            capabilities: Output::empty(),
            cgroupns_mode: Output::empty(),
            command: vec!["echo".to_string(), "Hello World!".to_string()].into(),
            container_read_refresh_timeout_milliseconds: Output::empty(),
            cpu_set: Output::empty(),
            cpu_shares: Output::empty(),
            destroy_grace_seconds: Output::empty(),
            devices: Output::empty(),
            dns: Output::empty(),
            dns_opts: Output::empty(),
            dns_searches: Output::empty(),
            domainname: Output::empty(),
            entrypoints: Output::empty(),
            envs: Output::empty(),
            gpus: Output::empty(),
            group_adds: Output::empty(),
            healthcheck: Output::empty(),
            hostname: Output::empty(),
            hosts: Output::empty(),
            image: "public.ecr.aws/ubuntu/ubuntu:latest".to_string().into(),
            init: Output::empty(),
            ipc_mode: Output::empty(),
            labels: Output::empty(),
            log_driver: Output::empty(),
            log_opts: Output::empty(),
            logs: true.into(),
            max_retry_count: Output::empty(),
            memory: Output::empty(),
            memory_swap: Output::empty(),
            mounts: Output::empty(),
            must_run: false.into(),
            name: Output::empty(),
            network_mode: Output::empty(),
            networks_advanced: Output::empty(),
            pid_mode: Output::empty(),
            ports: Output::empty(),
            privileged: Output::empty(),
            publish_all_ports: Output::empty(),
            read_only: Output::empty(),
            remove_volumes: Output::empty(),
            restart: Output::empty(),
            rm: Output::empty(),
            runtime: Output::empty(),
            security_opts: Output::empty(),
            shm_size: Output::empty(),
            start: Output::empty(),
            stdin_open: Output::empty(),
            stop_signal: Output::empty(),
            stop_timeout: Output::empty(),
            storage_opts: Output::empty(),
            sysctls: Output::empty(),
            tmpfs: Output::empty(),
            tty: Output::empty(),
            ulimits: Output::empty(),
            uploads: Output::empty(),
            user: Output::empty(),
            userns_mode: Output::empty(),
            volumes: Output::empty(),
            wait: Output::empty(),
            wait_timeout: Output::empty(),
            working_dir: Output::empty(),
        },
    );

    let image = image::create(
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
            build_on_preview: Output::empty(),
            image_name: "image:test".to_string().into(),
            registry: Output::empty(),
            skip_push: true.into(),
        },
    );

    add_export("logs", &cont.container_logs);
    add_export("image_id", &image.image_name);
    Ok(())
}
