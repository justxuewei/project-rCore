use crate::{
    batch::{run_next_app, app_running_time, stat_syscall}, 
    syscall::MAX_SYSCALL_ID};

pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);

    println!("[kernel] Application elpased time {} (unit unknown)", app_running_time());

    let mut syscall_slot = [0; MAX_SYSCALL_ID];
    stat_syscall(&mut syscall_slot);
    (0..syscall_slot.len()).for_each(|syscall_id| {
        if syscall_slot[syscall_id] != 0 {
            println!("[kernel] syscall {} was executed {} times", syscall_id, syscall_slot[syscall_id]);
        }
    });

    run_next_app()
}
