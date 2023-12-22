#[doc = r"Register block"]
#[repr(C)]
pub struct STRUCT {
    acquire: ACQUIRE,
    release: RELEASE,
    notify: NOTIFY,
    data0: DATA0,
    data1: DATA1,
    _reserved5: [u8; 0x08],
    lock_status: LOCK_STATUS,
}
impl STRUCT {
    #[doc = "0x00 - IPC acquire"]
    #[inline(always)]
    pub const fn acquire(&self) -> &ACQUIRE {
        &self.acquire
    }
    #[doc = "0x04 - IPC release"]
    #[inline(always)]
    pub const fn release(&self) -> &RELEASE {
        &self.release
    }
    #[doc = "0x08 - IPC notification"]
    #[inline(always)]
    pub const fn notify(&self) -> &NOTIFY {
        &self.notify
    }
    #[doc = "0x0c - IPC data 0"]
    #[inline(always)]
    pub const fn data0(&self) -> &DATA0 {
        &self.data0
    }
    #[doc = "0x10 - IPC data 1"]
    #[inline(always)]
    pub const fn data1(&self) -> &DATA1 {
        &self.data1
    }
    #[doc = "0x1c - IPC lock status"]
    #[inline(always)]
    pub const fn lock_status(&self) -> &LOCK_STATUS {
        &self.lock_status
    }
}
#[doc = "ACQUIRE (r) register accessor: IPC acquire\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acquire::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acquire`]
module"]
pub type ACQUIRE = crate::Reg<acquire::ACQUIRE_SPEC>;
#[doc = "IPC acquire"]
pub mod acquire;
#[doc = "RELEASE (w) register accessor: IPC release\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`release::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@release`]
module"]
pub type RELEASE = crate::Reg<release::RELEASE_SPEC>;
#[doc = "IPC release"]
pub mod release;
#[doc = "NOTIFY (w) register accessor: IPC notification\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`notify::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@notify`]
module"]
pub type NOTIFY = crate::Reg<notify::NOTIFY_SPEC>;
#[doc = "IPC notification"]
pub mod notify;
#[doc = "DATA0 (rw) register accessor: IPC data 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data0`]
module"]
pub type DATA0 = crate::Reg<data0::DATA0_SPEC>;
#[doc = "IPC data 0"]
pub mod data0;
#[doc = "DATA1 (rw) register accessor: IPC data 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data1`]
module"]
pub type DATA1 = crate::Reg<data1::DATA1_SPEC>;
#[doc = "IPC data 1"]
pub mod data1;
#[doc = "LOCK_STATUS (r) register accessor: IPC lock status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lock_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock_status`]
module"]
pub type LOCK_STATUS = crate::Reg<lock_status::LOCK_STATUS_SPEC>;
#[doc = "IPC lock status"]
pub mod lock_status;
