pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);

    // run_next_app()

    // TODO(justxuewei): do something...
    loop {}
}
