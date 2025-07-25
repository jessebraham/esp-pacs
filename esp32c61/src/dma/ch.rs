#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Cluster CH%s, containing IN_CONF0_CH?, IN_CONF1_CH?, INFIFO_STATUS_CH?, IN_POP_CH?, IN_LINK_CH?, IN_STATE_CH?, IN_SUC_EOF_DES_ADDR_CH?, IN_ERR_EOF_DES_ADDR_CH?, IN_DSCR_CH?, IN_DSCR_BF0_CH?, IN_DSCR_BF1_CH?, IN_PERI_SEL_CH?, OUT_CONF0_CH?, OUT_CONF1_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_EOF_BFR_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_PERI_SEL_CH?"]
pub struct CH {
    in_conf0: IN_CONF0,
    in_conf1: IN_CONF1,
    infifo_status: INFIFO_STATUS,
    in_pop: IN_POP,
    in_link: IN_LINK,
    in_state: IN_STATE,
    in_suc_eof_des_addr: IN_SUC_EOF_DES_ADDR,
    in_err_eof_des_addr: IN_ERR_EOF_DES_ADDR,
    in_dscr: IN_DSCR,
    in_dscr_bf0: IN_DSCR_BF0,
    in_dscr_bf1: IN_DSCR_BF1,
    _reserved11: [u8; 0x04],
    in_peri_sel: IN_PERI_SEL,
    _reserved12: [u8; 0x2c],
    out_conf0: OUT_CONF0,
    out_conf1: OUT_CONF1,
    outfifo_status: OUTFIFO_STATUS,
    out_push: OUT_PUSH,
    out_link: OUT_LINK,
    out_state: OUT_STATE,
    out_eof_des_addr: OUT_EOF_DES_ADDR,
    out_eof_bfr_des_addr: OUT_EOF_BFR_DES_ADDR,
    out_dscr: OUT_DSCR,
    out_dscr_bf0: OUT_DSCR_BF0,
    out_dscr_bf1: OUT_DSCR_BF1,
    _reserved23: [u8; 0x04],
    out_peri_sel: OUT_PERI_SEL,
    _reserved_end: [u8; 0x2c],
}
impl CH {
    #[doc = "0x00 - Configuration register 0 of RX channel 0"]
    #[inline(always)]
    pub const fn in_conf0(&self) -> &IN_CONF0 {
        &self.in_conf0
    }
    #[doc = "0x04 - Configuration register 1 of RX channel 0"]
    #[inline(always)]
    pub const fn in_conf1(&self) -> &IN_CONF1 {
        &self.in_conf1
    }
    #[doc = "0x08 - Receive FIFO status of RX channel 0"]
    #[inline(always)]
    pub const fn infifo_status(&self) -> &INFIFO_STATUS {
        &self.infifo_status
    }
    #[doc = "0x0c - Receive FIFO status of RX channel 0"]
    #[inline(always)]
    pub const fn in_pop(&self) -> &IN_POP {
        &self.in_pop
    }
    #[doc = "0x10 - Receive FIFO status of RX channel 0"]
    #[inline(always)]
    pub const fn in_link(&self) -> &IN_LINK {
        &self.in_link
    }
    #[doc = "0x14 - Receive status of RX channel 0"]
    #[inline(always)]
    pub const fn in_state(&self) -> &IN_STATE {
        &self.in_state
    }
    #[doc = "0x18 - Receive descriptor address when EOF occurs on RX channel 0"]
    #[inline(always)]
    pub const fn in_suc_eof_des_addr(&self) -> &IN_SUC_EOF_DES_ADDR {
        &self.in_suc_eof_des_addr
    }
    #[doc = "0x1c - Receive descriptor address when errors occur of RX channel 0"]
    #[inline(always)]
    pub const fn in_err_eof_des_addr(&self) -> &IN_ERR_EOF_DES_ADDR {
        &self.in_err_eof_des_addr
    }
    #[doc = "0x20 - Current receive descriptor address of RX channel 0"]
    #[inline(always)]
    pub const fn in_dscr(&self) -> &IN_DSCR {
        &self.in_dscr
    }
    #[doc = "0x24 - The last receive descriptor address of RX channel 0"]
    #[inline(always)]
    pub const fn in_dscr_bf0(&self) -> &IN_DSCR_BF0 {
        &self.in_dscr_bf0
    }
    #[doc = "0x28 - The second-to-last receive descriptor address of RX channel 0"]
    #[inline(always)]
    pub const fn in_dscr_bf1(&self) -> &IN_DSCR_BF1 {
        &self.in_dscr_bf1
    }
    #[doc = "0x30 - Peripheral selection register of RX channel 0"]
    #[inline(always)]
    pub const fn in_peri_sel(&self) -> &IN_PERI_SEL {
        &self.in_peri_sel
    }
    #[doc = "0x60 - Configuration register 0 of TX channel 0"]
    #[inline(always)]
    pub const fn out_conf0(&self) -> &OUT_CONF0 {
        &self.out_conf0
    }
    #[doc = "0x64 - Configuration register 1 of TX channel 0"]
    #[inline(always)]
    pub const fn out_conf1(&self) -> &OUT_CONF1 {
        &self.out_conf1
    }
    #[doc = "0x68 - Receive FIFO status of RX channel 0"]
    #[inline(always)]
    pub const fn outfifo_status(&self) -> &OUTFIFO_STATUS {
        &self.outfifo_status
    }
    #[doc = "0x6c - Push control register of TX channel 0"]
    #[inline(always)]
    pub const fn out_push(&self) -> &OUT_PUSH {
        &self.out_push
    }
    #[doc = "0x70 - Push control register of TX channel 0"]
    #[inline(always)]
    pub const fn out_link(&self) -> &OUT_LINK {
        &self.out_link
    }
    #[doc = "0x74 - Transmit status of TX channel 0"]
    #[inline(always)]
    pub const fn out_state(&self) -> &OUT_STATE {
        &self.out_state
    }
    #[doc = "0x78 - Transmit descriptor address when EOF occurs on TX channel 0"]
    #[inline(always)]
    pub const fn out_eof_des_addr(&self) -> &OUT_EOF_DES_ADDR {
        &self.out_eof_des_addr
    }
    #[doc = "0x7c - The last transmit descriptor address when EOF occurs on TX channel 0"]
    #[inline(always)]
    pub const fn out_eof_bfr_des_addr(&self) -> &OUT_EOF_BFR_DES_ADDR {
        &self.out_eof_bfr_des_addr
    }
    #[doc = "0x80 - Current transmit descriptor address of TX channel 0"]
    #[inline(always)]
    pub const fn out_dscr(&self) -> &OUT_DSCR {
        &self.out_dscr
    }
    #[doc = "0x84 - The last transmit descriptor address of TX channel 0"]
    #[inline(always)]
    pub const fn out_dscr_bf0(&self) -> &OUT_DSCR_BF0 {
        &self.out_dscr_bf0
    }
    #[doc = "0x88 - The second-to-last transmit descriptor address of TX channel 0"]
    #[inline(always)]
    pub const fn out_dscr_bf1(&self) -> &OUT_DSCR_BF1 {
        &self.out_dscr_bf1
    }
    #[doc = "0x90 - Peripheral selection register of TX channel 0"]
    #[inline(always)]
    pub const fn out_peri_sel(&self) -> &OUT_PERI_SEL {
        &self.out_peri_sel
    }
}
#[doc = "IN_CONF0 (rw) register accessor: Configuration register 0 of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_conf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_conf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_conf0`] module"]
pub type IN_CONF0 = crate::Reg<in_conf0::IN_CONF0_SPEC>;
#[doc = "Configuration register 0 of RX channel 0"]
pub mod in_conf0;
#[doc = "IN_CONF1 (rw) register accessor: Configuration register 1 of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_conf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_conf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_conf1`] module"]
pub type IN_CONF1 = crate::Reg<in_conf1::IN_CONF1_SPEC>;
#[doc = "Configuration register 1 of RX channel 0"]
pub mod in_conf1;
#[doc = "INFIFO_STATUS (r) register accessor: Receive FIFO status of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`infifo_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@infifo_status`] module"]
pub type INFIFO_STATUS = crate::Reg<infifo_status::INFIFO_STATUS_SPEC>;
#[doc = "Receive FIFO status of RX channel 0"]
pub mod infifo_status;
#[doc = "IN_POP (rw) register accessor: Receive FIFO status of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_pop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_pop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_pop`] module"]
pub type IN_POP = crate::Reg<in_pop::IN_POP_SPEC>;
#[doc = "Receive FIFO status of RX channel 0"]
pub mod in_pop;
#[doc = "IN_LINK (rw) register accessor: Receive FIFO status of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_link`] module"]
pub type IN_LINK = crate::Reg<in_link::IN_LINK_SPEC>;
#[doc = "Receive FIFO status of RX channel 0"]
pub mod in_link;
#[doc = "IN_STATE (r) register accessor: Receive status of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_state`] module"]
pub type IN_STATE = crate::Reg<in_state::IN_STATE_SPEC>;
#[doc = "Receive status of RX channel 0"]
pub mod in_state;
#[doc = "IN_SUC_EOF_DES_ADDR (r) register accessor: Receive descriptor address when EOF occurs on RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_suc_eof_des_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_suc_eof_des_addr`] module"]
pub type IN_SUC_EOF_DES_ADDR = crate::Reg<in_suc_eof_des_addr::IN_SUC_EOF_DES_ADDR_SPEC>;
#[doc = "Receive descriptor address when EOF occurs on RX channel 0"]
pub mod in_suc_eof_des_addr;
#[doc = "IN_ERR_EOF_DES_ADDR (r) register accessor: Receive descriptor address when errors occur of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_err_eof_des_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_err_eof_des_addr`] module"]
pub type IN_ERR_EOF_DES_ADDR = crate::Reg<in_err_eof_des_addr::IN_ERR_EOF_DES_ADDR_SPEC>;
#[doc = "Receive descriptor address when errors occur of RX channel 0"]
pub mod in_err_eof_des_addr;
#[doc = "IN_DSCR (r) register accessor: Current receive descriptor address of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_dscr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr`] module"]
pub type IN_DSCR = crate::Reg<in_dscr::IN_DSCR_SPEC>;
#[doc = "Current receive descriptor address of RX channel 0"]
pub mod in_dscr;
#[doc = "IN_DSCR_BF0 (r) register accessor: The last receive descriptor address of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_dscr_bf0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_bf0`] module"]
pub type IN_DSCR_BF0 = crate::Reg<in_dscr_bf0::IN_DSCR_BF0_SPEC>;
#[doc = "The last receive descriptor address of RX channel 0"]
pub mod in_dscr_bf0;
#[doc = "IN_DSCR_BF1 (r) register accessor: The second-to-last receive descriptor address of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_dscr_bf1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_bf1`] module"]
pub type IN_DSCR_BF1 = crate::Reg<in_dscr_bf1::IN_DSCR_BF1_SPEC>;
#[doc = "The second-to-last receive descriptor address of RX channel 0"]
pub mod in_dscr_bf1;
#[doc = "IN_PERI_SEL (rw) register accessor: Peripheral selection register of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_peri_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_peri_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_peri_sel`] module"]
pub type IN_PERI_SEL = crate::Reg<in_peri_sel::IN_PERI_SEL_SPEC>;
#[doc = "Peripheral selection register of RX channel 0"]
pub mod in_peri_sel;
#[doc = "OUT_CONF0 (rw) register accessor: Configuration register 0 of TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_conf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_conf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_conf0`] module"]
pub type OUT_CONF0 = crate::Reg<out_conf0::OUT_CONF0_SPEC>;
#[doc = "Configuration register 0 of TX channel 0"]
pub mod out_conf0;
#[doc = "OUT_CONF1 (rw) register accessor: Configuration register 1 of TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_conf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_conf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_conf1`] module"]
pub type OUT_CONF1 = crate::Reg<out_conf1::OUT_CONF1_SPEC>;
#[doc = "Configuration register 1 of TX channel 0"]
pub mod out_conf1;
#[doc = "OUTFIFO_STATUS (r) register accessor: Receive FIFO status of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`outfifo_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outfifo_status`] module"]
pub type OUTFIFO_STATUS = crate::Reg<outfifo_status::OUTFIFO_STATUS_SPEC>;
#[doc = "Receive FIFO status of RX channel 0"]
pub mod outfifo_status;
#[doc = "OUT_PUSH (rw) register accessor: Push control register of TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_push::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_push::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_push`] module"]
pub type OUT_PUSH = crate::Reg<out_push::OUT_PUSH_SPEC>;
#[doc = "Push control register of TX channel 0"]
pub mod out_push;
#[doc = "OUT_LINK (rw) register accessor: Push control register of TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_link`] module"]
pub type OUT_LINK = crate::Reg<out_link::OUT_LINK_SPEC>;
#[doc = "Push control register of TX channel 0"]
pub mod out_link;
#[doc = "OUT_STATE (r) register accessor: Transmit status of TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_state`] module"]
pub type OUT_STATE = crate::Reg<out_state::OUT_STATE_SPEC>;
#[doc = "Transmit status of TX channel 0"]
pub mod out_state;
#[doc = "OUT_EOF_DES_ADDR (r) register accessor: Transmit descriptor address when EOF occurs on TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_eof_des_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_eof_des_addr`] module"]
pub type OUT_EOF_DES_ADDR = crate::Reg<out_eof_des_addr::OUT_EOF_DES_ADDR_SPEC>;
#[doc = "Transmit descriptor address when EOF occurs on TX channel 0"]
pub mod out_eof_des_addr;
#[doc = "OUT_EOF_BFR_DES_ADDR (r) register accessor: The last transmit descriptor address when EOF occurs on TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_eof_bfr_des_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_eof_bfr_des_addr`] module"]
pub type OUT_EOF_BFR_DES_ADDR = crate::Reg<out_eof_bfr_des_addr::OUT_EOF_BFR_DES_ADDR_SPEC>;
#[doc = "The last transmit descriptor address when EOF occurs on TX channel 0"]
pub mod out_eof_bfr_des_addr;
#[doc = "OUT_DSCR (r) register accessor: Current transmit descriptor address of TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_dscr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr`] module"]
pub type OUT_DSCR = crate::Reg<out_dscr::OUT_DSCR_SPEC>;
#[doc = "Current transmit descriptor address of TX channel 0"]
pub mod out_dscr;
#[doc = "OUT_DSCR_BF0 (r) register accessor: The last transmit descriptor address of TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_dscr_bf0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_bf0`] module"]
pub type OUT_DSCR_BF0 = crate::Reg<out_dscr_bf0::OUT_DSCR_BF0_SPEC>;
#[doc = "The last transmit descriptor address of TX channel 0"]
pub mod out_dscr_bf0;
#[doc = "OUT_DSCR_BF1 (r) register accessor: The second-to-last transmit descriptor address of TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_dscr_bf1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_bf1`] module"]
pub type OUT_DSCR_BF1 = crate::Reg<out_dscr_bf1::OUT_DSCR_BF1_SPEC>;
#[doc = "The second-to-last transmit descriptor address of TX channel 0"]
pub mod out_dscr_bf1;
#[doc = "OUT_PERI_SEL (rw) register accessor: Peripheral selection register of TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_peri_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_peri_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_peri_sel`] module"]
pub type OUT_PERI_SEL = crate::Reg<out_peri_sel::OUT_PERI_SEL_SPEC>;
#[doc = "Peripheral selection register of TX channel 0"]
pub mod out_peri_sel;
