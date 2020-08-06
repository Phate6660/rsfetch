pub async fn async_main(matches: clap::ArgMatches<'_>) {
    let f1 = crate::async_functions::async_cpu::async_cpu(&matches);
    let f2 = crate::async_functions::async_device::async_device(&matches);
    let f3 = crate::async_functions::async_dewm::async_dewm(&matches);
    let f4 = crate::async_functions::async_distro::async_distro(&matches);
    let f5 = crate::async_functions::async_env::async_editor(&matches);
    let f6 = crate::async_functions::async_hostname::async_hostname(&matches);
    let f7 = crate::async_functions::async_kernel::async_kernel(&matches);
    let f8 = crate::async_functions::async_memory::async_memory(&matches);
    let f9 = crate::async_functions::async_packages::async_packages(&matches);
    let f10 = crate::async_functions::async_env::async_shell(&matches);
    let f11 = crate::async_functions::async_uptime::async_uptime(&matches);
    let f12 = crate::async_functions::async_env::async_user(&matches);
    let f13 = crate::async_functions::async_music::async_music(&matches);
    futures::join!(f1, f2, f3, f4, f5, f6, f7, f8, f9, f10, f11, f12, f13);
}
