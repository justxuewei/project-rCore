use alloc::sync::Arc;

use crate::{sync::UPSafeCell, trap::TrapContext};

use super::{
    context::TaskContext,
    manager,
    switch::__switch,
    task::{TaskControlBlock, TaskStatus},
};

use lazy_static::*;

pub struct Processor {
    current: Option<Arc<TaskControlBlock>>,
    idle_task_cx: TaskContext,
}

impl Processor {
    fn new() -> Self {
        Self {
            current: None,
            idle_task_cx: TaskContext::zero_init(),
        }
    }

    // 取出正在执行的任务的 TCB，此时 self.current 为 None
    fn take_current(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.current.take()
    }

    // 复制正在执行任务的 TCB
    fn current(&self) -> Option<Arc<TaskControlBlock>> {
        self.current.as_ref().map(|ptr| Arc::clone(ptr))
    }

    fn get_idle_task_cx_ptr(&mut self) -> *mut TaskContext {
        &mut self.idle_task_cx as *mut _
    }
}

lazy_static! {
    pub static ref PROCESSOR: UPSafeCell<Processor> = unsafe { UPSafeCell::new(Processor::new()) };
}

pub fn take_current_task() -> Option<Arc<TaskControlBlock>> {
    PROCESSOR.exclusive_access().take_current()
}

pub fn current_task() -> Option<Arc<TaskControlBlock>> {
    PROCESSOR.exclusive_access().current()
}

pub fn current_user_token() -> usize {
    let task = current_task().unwrap();
    task.inner_exclusive_access().get_user_token()
}

pub fn current_trap_cx() -> &'static mut TrapContext {
    let task = current_task().unwrap();
    task.inner_exclusive_access().get_trap_cx()
}

// 无限循环直至有一个 task 到来，此时使用 __switch 切换进程
pub fn run_tasks() {
    loop {
        let mut processor = PROCESSOR.exclusive_access();
        if let Some(task) = manager::fetch_task() {
            let idle_task_cx_ptr = processor.get_idle_task_cx_ptr();
            let mut next_tcb_inner = task.inner_exclusive_access();
            let next_task_cx_ptr = &next_tcb_inner.task_cx as *const TaskContext;
            next_tcb_inner.task_status = TaskStatus::Running;
            drop(next_tcb_inner);
            processor.current = Some(task);
            drop(processor);

            unsafe { __switch(idle_task_cx_ptr, next_task_cx_ptr) }
        }
    }
}

// 将当前任务 current_task_cx_ptr 切换为 idle 控制流
pub fn schedule(current_task_cx_ptr: *mut TaskContext) {
    let mut processer = PROCESSOR.exclusive_access();
    let idle_task_cx_ptr = processer.get_idle_task_cx_ptr() as *const TaskContext;
    drop(processer);

    unsafe { __switch(current_task_cx_ptr, idle_task_cx_ptr) }
}
