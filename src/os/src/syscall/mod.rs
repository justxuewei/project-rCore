const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

mod fs;
mod process;

use fs::*;
use process::*;


pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        _ => panic!("Unsupported system_id: {}", syscall_id),
    }
}
