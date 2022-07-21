use alloc::sync::Arc;

use crate::block_dev::BlockDevice;

pub const BLOCK_SIZE: usize = 512;

pub struct BlockCache {
    cache: [u8; BLOCK_SIZE],
    block_id: usize,
    block_device: Arc<dyn BlockDevice>,
    modified: bool,
}

impl BlockCache {
    // new 从块设备中缓存最新的数据
    pub fn new(block_id: usize, block_device: Arc<dyn BlockDevice>) -> Self {
        let mut cache = [0; BLOCK_SIZE];
        block_device.read_block(block_id, &mut cache);
        Self {
            cache,
            block_id,
            block_device,
            modified: false,
        }
    }

    // 获取块内偏移地址
    fn addr_of_offset(&self, offset: usize) -> usize {
        &self.cache[offset] as *const _ as usize
    }

    // 在 offset 位置获取结构体 T 的引用
    pub fn get_ref<T>(&self, offset: usize) -> &T
    where
        T: Sized,
    {
        let size = core::mem::size_of::<T>();
        assert!(offset + size <= BLOCK_SIZE);
        unsafe { &*(self.addr_of_offset(offset) as *const T) }
    }

    // 在 offset 位置获取结构体 T 的可变引用
    pub fn get_mut<T>(&mut self, offset: usize) -> &mut T
    where
        T: Sized,
    {
        let size = core::mem::size_of::<T>();
        assert!(offset + size <= BLOCK_SIZE);
        self.modified = true;
        unsafe { &mut *(self.addr_of_offset(offset) as *mut T) }
    }
}
