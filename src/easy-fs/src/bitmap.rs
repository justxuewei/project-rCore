use alloc::sync::Arc;

use crate::{
    block_cache::{get_block_cache, BLOCK_SIZE},
    block_dev::BlockDevice,
};

// 一个 block 包含的 bits 的数量
const BLOCK_BITS: usize = BLOCK_SIZE * 8;

// BitmapBlock 表示一个在 block device 上的 bitmap block，easyfs 的 block 的
// size 为 512B，所以 bitmap block 的长度为 8B * 64 = 512B。
type BitmapBlock = [u64; 64];

// Bitmap 这个结构是保存在内存中的，它表示一个具体的位图在 block device 中的位
// 置（bitmap 开始于 start_block_id，长度为 blocks）
pub struct Bitmap {
    start_block_id: usize,
    blocks: usize,
}

impl Bitmap {
    pub fn new(start_block_id: usize, blocks: usize) -> Self {
        Self {
            start_block_id,
            blocks,
        }
    }

    // 在 bitmap 中找到一个空闲位置并标记占用，返回这个位置的索引
    pub fn alloc(&self, block_device: Arc<dyn BlockDevice>) -> Option<usize> {
        // iterate all bitmap blocks
        for block_id in 0..self.blocks {
            let pos = get_block_cache(self.start_block_id + block_id, block_device.clone())
                .lock()
                // f maps BitmapBlock to usize
                .modify(0, |bitmap_block: &mut BitmapBlock| {
                    // pos is offset in BitmapBlock, inner_pos is offset in a
                    // bitmap (u64)
                    let pair = bitmap_block
                        .iter()
                        .enumerate()
                        .find(|(_, bitmap)| **bitmap != u64::MAX)
                        .map(|(pos, bitmap)| (pos, bitmap.trailing_ones() as usize));

                    if let Some((pos, inner_pos)) = pair {
                        bitmap_block[pos] |= 1 << inner_pos;
                        return Some((block_id * BLOCK_BITS + pos * 64 + inner_pos) as usize);
                    }
                    None
                });
            if pos.is_some() {
                return pos;
            }
        }
        None
    }
}
