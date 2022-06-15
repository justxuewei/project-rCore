mod context;
mod manager;
mod pid;
mod processor;
mod switch;
mod task;

use alloc::sync::Arc;
use lazy_static::*;

use crate::{loader, task::task::TaskControlBlock};

pub use context::TaskContext;

use self::{switch::__switch, task::TaskStatus, processor::schedule};

const INITPROC_NAME: &str = "initproc";

lazy_static! {
    pub static ref INITPROC: Arc<TaskControlBlock> = {
        Arc::new(TaskControlBlock::new(
            loader::get_app_data_by_name(INITPROC_NAME).unwrap(),
        ))
    };
}

pub fn add_initproc() {
    manager::add_task(INITPROC.clone());
}

// 暂停当前任务并切换为 idle 控制流
pub fn suspend_current_and_run_next() {
    let current_task = processor::take_current_task().unwrap();
    let mut current_task_inner = current_task.inner_exclusive_access();
    current_task_inner.task_status = TaskStatus::Ready;
    let current_task_cx_ptr = &mut current_task_inner.task_cx as *mut TaskContext;
    drop(current_task_inner);

    manager::add_task(current_task);
    processor::schedule(current_task_cx_ptr);
}

pub fn exit_current_and_run_next(exit_code: i32) {
    todo!()
}
