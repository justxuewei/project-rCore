use core::{arch::asm, slice, str};
use lazy_static::*;

use crate::sync::UPSafeCell;
use crate::trap::TrapContext;

const USER_STACK_SIZE: usize = 4096 * 2;
const KERNEL_STACK_SIZE: usize = 4096 * 2;
const MAX_APP_NUM: usize = 16;
const APP_BASE_ADDRESS: usize = 0x80400000;
const APP_SIZE_LIMIT: usize = 0x20000;

// 这里是指操作内存布局以 4096 (4Kb) 为单位对齐，
// 但是 data 的长度为 8Kb，已经超过了 4Kb，那么这个对齐还有作用吗？
#[repr(align(4096))]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

#[repr(align(4096))]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

// KERNEL_STACK 用于保存
static KERNEL_STACK: KernelStack = KernelStack {
    data: [0; KERNEL_STACK_SIZE],
};
static USER_STACK: UserStack = UserStack {
    data: [0; USER_STACK_SIZE],
};

impl UserStack {
    // 在 RISC-V 中 stack 都是向下增长的，比如 [|||||] 表示一个栈，
    // 左边是低地址 L，右边是高地址 H，那么栈底是在右边，即 sp == H，
    // self.data 的首地址 == L。
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}

impl KernelStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }

    // push_context 将 TrapContext 压入 KernelStack 并返回该 TrapContext
    pub fn push_context(&self, cx: TrapContext) -> &'static mut TrapContext {
        let cx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *cx_ptr = cx;
        }
        unsafe { cx_ptr.as_mut().unwrap() }
    }
}

struct AppManager {
    num_app: usize,
    current_app: usize,
    app_start: [usize; MAX_APP_NUM + 1],
    app_name_start: [usize; MAX_APP_NUM + 1],
}

impl AppManager {
    pub fn print_app_info(&self) {
        println!("[kernel] num_app = {}", self.num_app);
        for i in 0..self.num_app {
            println!(
                "[kernel] app_{} [{:#x},{:#x}]",
                i,
                self.app_start[i],
                self.app_start[i + 1],
            );
        }
    }

    // load_app 主要的作用是将一个 user application 从代码段复制到 APP_BASE_ADDRESS
    // 位置，让 user application 处于待执行状态。
    unsafe fn load_app(&self, app_id: usize) {
        if app_id >= self.num_app {
            panic!("All applications completed!");
        }
        println!("[kernel] Loading app_{}", app_id);
        asm!("fence.i");
        slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, APP_SIZE_LIMIT).fill(0);
        let app_src = slice::from_raw_parts(
            self.app_start[app_id] as *const u8,
            self.app_start[app_id + 1] - self.app_start[app_id],
        );
        let app_dst = slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, app_src.len());
        app_dst.copy_from_slice(app_src);
    }

    unsafe fn get_app_name(&self, app_id: usize) -> &str {
        if app_id >= self.num_app {
            panic!("App not found!");
        }

        let app_name_raw = slice::from_raw_parts(
            self.app_name_start[app_id - 1] as *const u8,
            self.app_name_start[app_id] - self.app_name_start[app_id - 1],
        );

        core::str::from_utf8(app_name_raw).unwrap()
    }

    pub fn get_current_app(&self) -> usize {
        self.current_app
    }

    pub fn move_to_next_app(&mut self) {
        self.current_app += 1;
    }
}

// 在第一次被调用的时候才真正开始加载静态变量
lazy_static! {
    static ref APP_MANAGER: UPSafeCell<AppManager> = unsafe {
        UPSafeCell::new({
            extern "C" {
                fn _num_app();
                fn _num_app_name();
            }
            // 从 link_app.S 加载各个应用程序的在内存中的位置，同时初始化 AppManager
            let num_app_ptr = _num_app as usize as *const usize;
            let num_app = num_app_ptr.read_volatile();
            let mut app_start: [usize; MAX_APP_NUM + 1] = [0; MAX_APP_NUM + 1];
            let app_start_raw: &[usize] = slice::from_raw_parts(num_app_ptr.add(1), num_app + 1);
            app_start[..=num_app].copy_from_slice(app_start_raw);

            let num_app_name_ptr = _num_app_name as usize as *const usize;
            let num_app_name = num_app_name_ptr.read_volatile();
            let mut app_name_start: [usize; MAX_APP_NUM + 1] = [0; MAX_APP_NUM + 1];
            let app_name_start_raw: &[usize] = slice::from_raw_parts(num_app_name_ptr.add(1), num_app_name + 1);
            app_name_start[..=num_app_name].copy_from_slice(app_name_start_raw);

            AppManager {
                num_app,
                current_app: 0,
                app_start,
                app_name_start,
            }
        })
    };
}

pub fn init() {
    print_app_info();
}

pub fn print_app_info() {
    APP_MANAGER.exclusive_access().print_app_info();
}

pub fn run_next_app() -> ! {
    let mut app_manager = APP_MANAGER.exclusive_access();
    let current_app = app_manager.get_current_app();
    unsafe {
        app_manager.load_app(current_app);
    }
    app_manager.move_to_next_app();
    drop(app_manager);

    extern "C" {
        fn __restore(cx_addr: usize);
    }
    unsafe {
        __restore(KERNEL_STACK.push_context(TrapContext::app_init_context(
            APP_BASE_ADDRESS,
            USER_STACK.get_sp(),
        )) as *const _ as usize)
    }

    panic!("Unreachable in batch::run_next_app!");
}

pub unsafe fn current_app_info() -> (usize, &'static str) {
    let app_manager = APP_MANAGER.exclusive_access();
    let app_id = app_manager.get_current_app() - 1;
    let app_name = app_manager.get_app_name(app_id);
    drop(app_manager);

    (app_id, app_name)
}
