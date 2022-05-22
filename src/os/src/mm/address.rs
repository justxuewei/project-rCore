use crate::config;
use core::fmt::{self, Debug, Formatter};

use super::page_table::PageTableEntry;

// PA = Page Address
const PA_WIDTH_SV39: usize = 56;
// VA = Virtual Address
const VA_WIDTH_SV39: usize = 39;
// PPN = Physical Page Number
const PPN_WIDTH_SV39: usize = PA_WIDTH_SV39 - config::PAGE_SIZE_BITS;
// VPN = Virtual Page Number
const VPN_WIDTH_SV39: usize = VA_WIDTH_SV39 - config::PAGE_SIZE_BITS;

#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub struct PhysAddr(pub usize);
#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub struct PhysPageNum(pub usize);

impl Debug for PhysAddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("PA:{:#x}", self.0))
    }
}

impl Debug for PhysPageNum {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("PPN:{:#x}", self.0))
    }
}

impl PhysAddr {
    pub fn floor(&self) -> PhysPageNum {
        PhysPageNum(self.0 / config::PAGE_SIZE)
    }

    pub fn ceil(&self) -> PhysPageNum {
        PhysPageNum((self.0 - 1 + config::PAGE_SIZE) / config::PAGE_SIZE)
    }

    pub fn page_offset(&self) -> usize {
        self.0 & (config::PAGE_SIZE - 1)
    }

    pub fn aligned(&self) -> bool {
        self.page_offset() == 0
    }
}

impl PhysPageNum {
    pub fn get_pte_array(&self) -> &'static mut [PageTableEntry] {
        let pa: PhysAddr = (*self).into();
        unsafe {
            core::slice::from_raw_parts_mut(
                pa.0 as *mut PageTableEntry,
                config::PAGE_SIZE / core::mem::size_of::<usize>(),
            )
        }
    }

    pub fn get_bytes_array(&self) -> &'static mut [u8] {
        let pa: PhysAddr = (*self).into();
        unsafe { core::slice::from_raw_parts_mut(pa.0 as *mut u8, config::PAGE_SIZE) }
    }

    pub fn get_mut<T>(&self) -> &'static mut T {
        let pa: PhysAddr = (*self).into();
        unsafe {
            (pa.0 as *mut T).as_mut().unwrap()
        }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub struct VirtAddr(pub usize);
#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub struct VirtPageNum(pub usize);

impl Debug for VirtAddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("VA:{:#x}", self.0))
    }
}

impl Debug for VirtPageNum {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("VPN:{:#x}", self.0))
    }
}

impl VirtAddr {
    pub fn floor(&self) -> VirtPageNum {
        VirtPageNum(self.0 / config::PAGE_SIZE)
    }

    pub fn ceil(&self) -> VirtPageNum {
        VirtPageNum((self.0 - 1 + config::PAGE_SIZE) / config::PAGE_SIZE)
    }

    pub fn page_offset(&self) -> usize {
        self.0 & (config::PAGE_SIZE - 1)
    }

    pub fn aligned(&self) -> bool {
        self.page_offset() == 0
    }
}

impl VirtPageNum {
    pub fn indexes(&self) -> [usize; 3] {
        let mut vpn = self.0;
        let mut idxs = [0usize; 3];
        let idxlen = VPN_WIDTH_SV39 / 3;
        for i in (0..3).rev() {
            idxs[i] = vpn & ((1usize << idxlen) - 1);
            vpn >>= idxlen;
        }
        idxs
    }
}

impl From<usize> for PhysAddr {
    fn from(v: usize) -> Self {
        Self(v & (1 << PA_WIDTH_SV39) - 1)
    }
}

impl From<usize> for PhysPageNum {
    fn from(v: usize) -> Self {
        Self(v & ((1 << PPN_WIDTH_SV39) - 1))
    }
}

impl From<PhysAddr> for usize {
    fn from(v: PhysAddr) -> Self {
        v.0
    }
}

impl From<PhysPageNum> for usize {
    fn from(v: PhysPageNum) -> Self {
        v.0
    }
}

impl From<PhysAddr> for PhysPageNum {
    fn from(v: PhysAddr) -> Self {
        assert_eq!(v.page_offset(), 0);
        v.floor()
    }
}

impl From<PhysPageNum> for PhysAddr {
    fn from(v: PhysPageNum) -> Self {
        Self(v.0 << config::PAGE_SIZE_BITS)
    }
}

impl From<VirtAddr> for VirtPageNum {
    fn from(v: VirtAddr) -> Self {
        assert_eq!(v.page_offset(), 0);
        v.floor()
    }
}

impl From<VirtPageNum> for VirtAddr {
    fn from(v: VirtPageNum) -> Self {
        Self(v.0 << config::PAGE_SIZE_BITS)
    }
}
