use crate::{task::{self, processor, manager}, timer, mm::page_table, loader};

pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    task::exit_current_and_run_next(exit_code);
    panic!("Unreachable in sys_exit!");
}

pub fn sys_yield() -> isize {
    task::suspend_current_and_run_next();
    0
}

pub fn sys_get_time() -> isize {
    timer::get_time_ms() as isize
}

pub fn sys_fork() -> isize {
    let parent_tcb = processor::current_task().unwrap();
    let child_tcb = parent_tcb.fork();
    let mut child_trap_cx = child_tcb.inner_exclusive_access().get_trap_cx();
    // child process's return value is 0
    child_trap_cx.x[10] = 0;
    manager::add_task(child_tcb);

    child_tcb.getpid() as isize
}

pub fn sys_exec(path: *const u8) -> isize {
    let token = processor::current_user_token();
    let path = page_table::translated_str(token, path);
    if let Some(data) = loader::get_app_data_by_name(path.as_str()) {
        processor::current_task().unwrap().exec(data);
        return 0
    }
    -1
}
