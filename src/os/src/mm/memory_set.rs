use alloc::{collections::BTreeMap, vec::Vec};
use bitflags::*;

use super::{
    address::{PhysPageNum, VPNRange, VirtAddr, VirtPageNum},
    frame_allocator::{FrameTracker, frame_alloc},
    page_table::{PageTable, PTEFlags},
};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MapType {
    Identical, // 一个 VPN 唯一的映射一个 PPN，比如内核就需要访问物理内存中的某个 PPN
    Framed,    // 一个 VPN 随机的映射一个 PPN
}

bitflags! {
    // a subset of PTEFlags
    pub struct MapPermission: u8 {
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
    }
}

pub struct MapArea {
    vpn_range: VPNRange,
    data_frames: BTreeMap<VirtPageNum, FrameTracker>,
    map_type: MapType,
    map_perm: MapPermission,
}

impl MapArea {
    pub fn new(
        start_va: VirtAddr,
        end_va: VirtAddr,
        map_type: MapType,
        map_perm: MapPermission,
    ) -> Self {
        Self {
            vpn_range: VPNRange::new(start_va.floor(), end_va.ceil()),
            data_frames: BTreeMap::new(),
            map_type: map_type,
            map_perm: map_perm,
        }
    }

    pub fn map_one(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
        let ppn: PhysPageNum;
        match self.map_type {
            MapType::Identical => {
                ppn = PhysPageNum(vpn.0)
            }
            MapType::Framed => {
                let frame = frame_alloc().unwrap();
                ppn = frame.ppn;
                self.data_frames.insert(vpn, frame);
            }
        }

        let pte_flags = PTEFlags::from_bits(self.map_perm.bits).unwrap();
        page_table.map(vpn, ppn, pte_flags);
    }

    #[allow(unused)]
    pub fn unmap_one(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
        if self.map_type == MapType::Framed {
            self.data_frames.remove(&vpn);
        }
        page_table.unmap(vpn);
    }

    pub fn map(&mut self, page_table: &mut PageTable) {}
}

pub struct MemorySet {
    page_table: PageTable,
    areas: Vec<MapArea>,
}

impl MemorySet {
    fn new_bare() -> Self {
        Self {
            page_table: PageTable::new(),
            areas: Vec::new(),
        }
    }

    // pub fn token(&self) -> usize {

    // }

    pub fn insert_framed_area(
        &mut self,
        start_va: VirtAddr,
        end_va: VirtAddr,
        permission: MapPermission,
    ) {
        self.push(
            MapArea::new(start_va, end_va, MapType::Framed, permission),
            None,
        );
    }

    fn push(&mut self, mut map_area: MapArea, data: Option<&[u8]>) {}
}
