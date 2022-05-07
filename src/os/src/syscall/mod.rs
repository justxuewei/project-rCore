const MAX_SYSCALL_ID: usize = 1024;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_TASK_INFO: usize = 100;

mod fs;
mod process;
mod task_info;

use fs::*;
use process::*;

use self::task_info::sys_task_info;
use crate::batch::record_syscall;

pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    record_syscall(&syscall_id);
    match syscall_id {
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_TASK_INFO => sys_task_info(),
        _ => panic!("Unsupported system_id: {}", syscall_id),
    }
}
