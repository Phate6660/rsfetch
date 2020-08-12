pub fn main(matches: clap::ArgMatches) {
    crate::output::cpu::output_cpu(&matches);
    crate::output::device::output_device(&matches);
    crate::output::dewm::output_dewm(&matches);
    crate::output::distro::output_distro(&matches);
    crate::output::env::output_editor(&matches);
    crate::output::gpu::output_gpu(&matches);
    crate::output::hostname::output_hostname(&matches);
    crate::output::kernel::output_kernel(&matches);
    crate::output::memory::output_memory(&matches);
    crate::output::packages::output_packages(&matches);
    crate::output::env::output_shell(&matches);
    crate::output::terminal::output_terminal(&matches);
    crate::output::uptime::output_uptime(&matches);
    crate::output::env::output_user(&matches);
    crate::output::music::output_music(&matches);
}
