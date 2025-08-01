#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    clk_en: CLK_EN,
    reset_en: RESET_EN,
    rng_data: RNG_DATA,
    cpu: CPU,
    bus_timeout: BUS_TIMEOUT,
    bus_timeout_addr: BUS_TIMEOUT_ADDR,
    bus_timeout_uid: BUS_TIMEOUT_UID,
    _reserved7: [u8; 0x04],
    interrupt_source: INTERRUPT_SOURCE,
    rng_cfg: RNG_CFG,
    rng_data_sync: RNG_DATA_SYNC,
    _reserved10: [u8; 0x03d0],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - need_des"]
    #[inline(always)]
    pub const fn clk_en(&self) -> &CLK_EN {
        &self.clk_en
    }
    #[doc = "0x04 - need_des"]
    #[inline(always)]
    pub const fn reset_en(&self) -> &RESET_EN {
        &self.reset_en
    }
    #[doc = "0x08 - need_des"]
    #[inline(always)]
    pub const fn rng_data(&self) -> &RNG_DATA {
        &self.rng_data
    }
    #[doc = "0x0c - need_des"]
    #[inline(always)]
    pub const fn cpu(&self) -> &CPU {
        &self.cpu
    }
    #[doc = "0x10 - need_des"]
    #[inline(always)]
    pub const fn bus_timeout(&self) -> &BUS_TIMEOUT {
        &self.bus_timeout
    }
    #[doc = "0x14 - need_des"]
    #[inline(always)]
    pub const fn bus_timeout_addr(&self) -> &BUS_TIMEOUT_ADDR {
        &self.bus_timeout_addr
    }
    #[doc = "0x18 - need_des"]
    #[inline(always)]
    pub const fn bus_timeout_uid(&self) -> &BUS_TIMEOUT_UID {
        &self.bus_timeout_uid
    }
    #[doc = "0x20 - need_des"]
    #[inline(always)]
    pub const fn interrupt_source(&self) -> &INTERRUPT_SOURCE {
        &self.interrupt_source
    }
    #[doc = "0x24 - need_des"]
    #[inline(always)]
    pub const fn rng_cfg(&self) -> &RNG_CFG {
        &self.rng_cfg
    }
    #[doc = "0x28 - need_des"]
    #[inline(always)]
    pub const fn rng_data_sync(&self) -> &RNG_DATA_SYNC {
        &self.rng_data_sync
    }
    #[doc = "0x3fc - need_des"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "CLK_EN (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_en`] module"]
pub type CLK_EN = crate::Reg<clk_en::CLK_EN_SPEC>;
#[doc = "need_des"]
pub mod clk_en;
#[doc = "RESET_EN (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`reset_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset_en`] module"]
pub type RESET_EN = crate::Reg<reset_en::RESET_EN_SPEC>;
#[doc = "need_des"]
pub mod reset_en;
#[doc = "RNG_DATA (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_data`] module"]
pub type RNG_DATA = crate::Reg<rng_data::RNG_DATA_SPEC>;
#[doc = "need_des"]
pub mod rng_data;
#[doc = "CPU (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu`] module"]
pub type CPU = crate::Reg<cpu::CPU_SPEC>;
#[doc = "need_des"]
pub mod cpu;
#[doc = "BUS_TIMEOUT (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_timeout`] module"]
pub type BUS_TIMEOUT = crate::Reg<bus_timeout::BUS_TIMEOUT_SPEC>;
#[doc = "need_des"]
pub mod bus_timeout;
#[doc = "BUS_TIMEOUT_ADDR (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_timeout_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_timeout_addr`] module"]
pub type BUS_TIMEOUT_ADDR = crate::Reg<bus_timeout_addr::BUS_TIMEOUT_ADDR_SPEC>;
#[doc = "need_des"]
pub mod bus_timeout_addr;
#[doc = "BUS_TIMEOUT_UID (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_timeout_uid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_timeout_uid`] module"]
pub type BUS_TIMEOUT_UID = crate::Reg<bus_timeout_uid::BUS_TIMEOUT_UID_SPEC>;
#[doc = "need_des"]
pub mod bus_timeout_uid;
#[doc = "INTERRUPT_SOURCE (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_source::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_source`] module"]
pub type INTERRUPT_SOURCE = crate::Reg<interrupt_source::INTERRUPT_SOURCE_SPEC>;
#[doc = "need_des"]
pub mod interrupt_source;
#[doc = "RNG_CFG (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_cfg`] module"]
pub type RNG_CFG = crate::Reg<rng_cfg::RNG_CFG_SPEC>;
#[doc = "need_des"]
pub mod rng_cfg;
#[doc = "RNG_DATA_SYNC (r) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_data_sync::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rng_data_sync`] module"]
pub type RNG_DATA_SYNC = crate::Reg<rng_data_sync::RNG_DATA_SYNC_SPEC>;
#[doc = "need_des"]
pub mod rng_data_sync;
#[doc = "DATE (rw) register accessor: need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "need_des"]
pub mod date;
