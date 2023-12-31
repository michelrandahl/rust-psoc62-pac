#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrl: CTRL,
    ctrl_clr: CTRL_CLR,
    ctrl_set: CTRL_SET,
    cmd_capture: CMD_CAPTURE,
    cmd_reload: CMD_RELOAD,
    cmd_stop: CMD_STOP,
    cmd_start: CMD_START,
    intr_cause: INTR_CAUSE,
    _reserved8: [u8; 0xe0],
    cnt: [CNT; 24],
}
impl RegisterBlock {
    #[doc = "0x00 - TCPWM control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - TCPWM control clear register"]
    #[inline(always)]
    pub const fn ctrl_clr(&self) -> &CTRL_CLR {
        &self.ctrl_clr
    }
    #[doc = "0x08 - TCPWM control set register"]
    #[inline(always)]
    pub const fn ctrl_set(&self) -> &CTRL_SET {
        &self.ctrl_set
    }
    #[doc = "0x0c - TCPWM capture command register"]
    #[inline(always)]
    pub const fn cmd_capture(&self) -> &CMD_CAPTURE {
        &self.cmd_capture
    }
    #[doc = "0x10 - TCPWM reload command register"]
    #[inline(always)]
    pub const fn cmd_reload(&self) -> &CMD_RELOAD {
        &self.cmd_reload
    }
    #[doc = "0x14 - TCPWM stop command register"]
    #[inline(always)]
    pub const fn cmd_stop(&self) -> &CMD_STOP {
        &self.cmd_stop
    }
    #[doc = "0x18 - TCPWM start command register"]
    #[inline(always)]
    pub const fn cmd_start(&self) -> &CMD_START {
        &self.cmd_start
    }
    #[doc = "0x1c - TCPWM Counter interrupt cause register"]
    #[inline(always)]
    pub const fn intr_cause(&self) -> &INTR_CAUSE {
        &self.intr_cause
    }
    #[doc = "0x100..0x700 - Timer/Counter/PWM Counter Module"]
    #[inline(always)]
    pub const fn cnt(&self, n: usize) -> &CNT {
        &self.cnt[n]
    }
}
#[doc = "CTRL (rw) register accessor: TCPWM control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "TCPWM control register"]
pub mod ctrl;
#[doc = "CTRL_CLR (rw) register accessor: TCPWM control clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_clr`]
module"]
pub type CTRL_CLR = crate::Reg<ctrl_clr::CTRL_CLR_SPEC>;
#[doc = "TCPWM control clear register"]
pub mod ctrl_clr;
#[doc = "CTRL_SET (rw) register accessor: TCPWM control set register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_set`]
module"]
pub type CTRL_SET = crate::Reg<ctrl_set::CTRL_SET_SPEC>;
#[doc = "TCPWM control set register"]
pub mod ctrl_set;
#[doc = "CMD_CAPTURE (rw) register accessor: TCPWM capture command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_capture::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_capture::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_capture`]
module"]
pub type CMD_CAPTURE = crate::Reg<cmd_capture::CMD_CAPTURE_SPEC>;
#[doc = "TCPWM capture command register"]
pub mod cmd_capture;
#[doc = "CMD_RELOAD (rw) register accessor: TCPWM reload command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_reload::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_reload::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_reload`]
module"]
pub type CMD_RELOAD = crate::Reg<cmd_reload::CMD_RELOAD_SPEC>;
#[doc = "TCPWM reload command register"]
pub mod cmd_reload;
#[doc = "CMD_STOP (rw) register accessor: TCPWM stop command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_stop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_stop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_stop`]
module"]
pub type CMD_STOP = crate::Reg<cmd_stop::CMD_STOP_SPEC>;
#[doc = "TCPWM stop command register"]
pub mod cmd_stop;
#[doc = "CMD_START (rw) register accessor: TCPWM start command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_start`]
module"]
pub type CMD_START = crate::Reg<cmd_start::CMD_START_SPEC>;
#[doc = "TCPWM start command register"]
pub mod cmd_start;
#[doc = "INTR_CAUSE (r) register accessor: TCPWM Counter interrupt cause register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_cause::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_cause`]
module"]
pub type INTR_CAUSE = crate::Reg<intr_cause::INTR_CAUSE_SPEC>;
#[doc = "TCPWM Counter interrupt cause register"]
pub mod intr_cause;
#[doc = "Timer/Counter/PWM Counter Module"]
pub use self::cnt::CNT;
#[doc = r"Cluster"]
#[doc = "Timer/Counter/PWM Counter Module"]
pub mod cnt;
