use crate::{
    config,
    mm::{address::VirtAddr, memory_set::MapPermission, KERNEL_SPACE},
    sync::UPSafeCell,
};
use alloc::vec::Vec;
use lazy_static::*;

pub struct PidHandle(pub usize);

pub struct PidAllocator {
    current: usize,
    recycled: Vec<usize>,
}

impl PidAllocator {
    pub fn new() -> Self {
        Self {
            current: 0,
            recycled: Vec::new(),
        }
    }

    pub fn alloc(&mut self) -> PidHandle {
        if let Some(pid) = self.recycled.pop() {
            PidHandle(pid)
        } else {
            let pid = self.current;
            self.current += 1;
            PidHandle(pid)
        }
    }

    pub fn dealloc(&mut self, pid: usize) {
        assert!(pid < self.current);
        assert!(
            !self.recycled.iter().any(|&i| i == pid),
            "pid {} has been deallocated",
            pid
        );
        self.recycled.push(pid);
    }
}

lazy_static! {
    pub static ref PID_ALLOCATOR: UPSafeCell<PidAllocator> =
        unsafe { UPSafeCell::new(PidAllocator::new()) };
}

impl Drop for PidHandle {
    fn drop(&mut self) {
        PID_ALLOCATOR.exclusive_access().dealloc(self.0);
    }
}

// 申请一个 pid 并返回 PidHandle
pub fn pid_alloc() -> PidHandle {
    PID_ALLOCATOR.exclusive_access().alloc()
}

pub struct KernelStack {
    pid: usize,
}

impl KernelStack {
    pub fn new(pid_handle: &PidHandle) -> Self {
        let pid = pid_handle.0;
        let (bottom, top) = kernel_stack_position(pid);
        KERNEL_SPACE.exclusive_access().insert_framed_area(
            VirtAddr::from(bottom),
            VirtAddr::from(top),
            MapPermission::R | MapPermission::W,
        );
        KernelStack { pid: pid }
    }

    #[allow(unused)]
    // push value 到 kernel stack
    pub fn push_on_top<T: Sized>(&self, value: T) -> *mut T {
        let size = core::mem::size_of::<T>();
        let top = self.get_top();
        let ptr_mut = (top - size) as *mut T;
        unsafe {
            *ptr_mut = value;
        }
        ptr_mut
    }

    // 获取 kernel stack 的 top virtual address (usize)
    pub fn get_top(&self) -> usize {
        let (_, top) = kernel_stack_position(self.pid);
        top
    }
}

impl Drop for KernelStack {
    // kernel stack 被释放的时候，其占用的物理内存 frames 被释放
    fn drop(&mut self) {
        let (bottom, _) = kernel_stack_position(self.pid);
        KERNEL_SPACE
            .exclusive_access()
            .remove_area_with_start_vpn(VirtAddr::from(bottom).into())
    }
}

// 返回 kernel stack 的 bottom 和 top 地址
pub fn kernel_stack_position(pid: usize) -> (usize, usize) {
    let top = config::TRAMPOLINE - pid * (config::KERNEL_STACK_SIZE + config::PAGE_SIZE);
    let bottom = top - config::KERNEL_STACK_SIZE;
    (bottom, top)
}
