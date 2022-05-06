use crate::batch::current_app_info;

pub fn sys_task_info() -> isize {
    unsafe {
        let (task_id, task_name) = current_app_info();
        println!("[kernel] task id = {}, task name = {}", task_id, task_name);
    }
    0
}
