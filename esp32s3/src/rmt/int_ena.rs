#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `CH_TX_END(0-3)` reader - The interrupt enable bit for CH%s_TX_END_INT."]
pub type CH_TX_END_R = crate::BitReader;
#[doc = "Field `CH_TX_END(0-3)` writer - The interrupt enable bit for CH%s_TX_END_INT."]
pub type CH_TX_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_TX_ERR(0-3)` reader - The interrupt enable bit for CH%s_ERR_INT."]
pub type CH_TX_ERR_R = crate::BitReader;
#[doc = "Field `CH_TX_ERR(0-3)` writer - The interrupt enable bit for CH%s_ERR_INT."]
pub type CH_TX_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_TX_THR_EVENT(0-3)` reader - The interrupt enable bit for CH%s_TX_THR_EVENT_INT."]
pub type CH_TX_THR_EVENT_R = crate::BitReader;
#[doc = "Field `CH_TX_THR_EVENT(0-3)` writer - The interrupt enable bit for CH%s_TX_THR_EVENT_INT."]
pub type CH_TX_THR_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_TX_LOOP(0-3)` reader - The interrupt enable bit for CH%s_TX_LOOP_INT."]
pub type CH_TX_LOOP_R = crate::BitReader;
#[doc = "Field `CH_TX_LOOP(0-3)` writer - The interrupt enable bit for CH%s_TX_LOOP_INT."]
pub type CH_TX_LOOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_RX_END(4-7)` reader - The interrupt enable bit for CH%s_RX_END_INT."]
pub type CH_RX_END_R = crate::BitReader;
#[doc = "Field `CH_RX_END(4-7)` writer - The interrupt enable bit for CH%s_RX_END_INT."]
pub type CH_RX_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_RX_ERR(4-7)` reader - The interrupt enable bit for CH%s_ERR_INT."]
pub type CH_RX_ERR_R = crate::BitReader;
#[doc = "Field `CH_RX_ERR(4-7)` writer - The interrupt enable bit for CH%s_ERR_INT."]
pub type CH_RX_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_RX_THR_EVENT(4-7)` reader - The interrupt enable bit for CH%s_RX_THR_EVENT_INT."]
pub type CH_RX_THR_EVENT_R = crate::BitReader;
#[doc = "Field `CH_RX_THR_EVENT(4-7)` writer - The interrupt enable bit for CH%s_RX_THR_EVENT_INT."]
pub type CH_RX_THR_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CH3_DMA_ACCESS_FAIL` reader - The interrupt enable bit for CH3_DMA_ACCESS_FAIL_INT."]
pub type TX_CH3_DMA_ACCESS_FAIL_R = crate::BitReader;
#[doc = "Field `TX_CH3_DMA_ACCESS_FAIL` writer - The interrupt enable bit for CH3_DMA_ACCESS_FAIL_INT."]
pub type TX_CH3_DMA_ACCESS_FAIL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_CH7_DMA_ACCESS_FAIL` reader - The interrupt enable bit for CH7_DMA_ACCESS_FAIL_INT."]
pub type RX_CH7_DMA_ACCESS_FAIL_R = crate::BitReader;
#[doc = "Field `RX_CH7_DMA_ACCESS_FAIL` writer - The interrupt enable bit for CH7_DMA_ACCESS_FAIL_INT."]
pub type RX_CH7_DMA_ACCESS_FAIL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "The interrupt enable bit for CH(0-3)_TX_END_INT."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH0_TX_END` field.</div>"]
    #[inline(always)]
    pub fn ch_tx_end(&self, n: u8) -> CH_TX_END_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_TX_END_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The interrupt enable bit for CH(0-3)_TX_END_INT."]
    #[inline(always)]
    pub fn ch_tx_end_iter(&self) -> impl Iterator<Item = CH_TX_END_R> + '_ {
        (0..4).map(move |n| CH_TX_END_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - The interrupt enable bit for CH0_TX_END_INT."]
    #[inline(always)]
    pub fn ch0_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for CH1_TX_END_INT."]
    #[inline(always)]
    pub fn ch1_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for CH2_TX_END_INT."]
    #[inline(always)]
    pub fn ch2_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for CH3_TX_END_INT."]
    #[inline(always)]
    pub fn ch3_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "The interrupt enable bit for CH(0-3)_ERR_INT."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH0_TX_ERR` field.</div>"]
    #[inline(always)]
    pub fn ch_tx_err(&self, n: u8) -> CH_TX_ERR_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_TX_ERR_R::new(((self.bits >> (n + 4)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The interrupt enable bit for CH(0-3)_ERR_INT."]
    #[inline(always)]
    pub fn ch_tx_err_iter(&self) -> impl Iterator<Item = CH_TX_ERR_R> + '_ {
        (0..4).map(move |n| CH_TX_ERR_R::new(((self.bits >> (n + 4)) & 1) != 0))
    }
    #[doc = "Bit 4 - The interrupt enable bit for CH0_ERR_INT."]
    #[inline(always)]
    pub fn ch0_tx_err(&self) -> CH_TX_ERR_R {
        CH_TX_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt enable bit for CH1_ERR_INT."]
    #[inline(always)]
    pub fn ch1_tx_err(&self) -> CH_TX_ERR_R {
        CH_TX_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt enable bit for CH2_ERR_INT."]
    #[inline(always)]
    pub fn ch2_tx_err(&self) -> CH_TX_ERR_R {
        CH_TX_ERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt enable bit for CH3_ERR_INT."]
    #[inline(always)]
    pub fn ch3_tx_err(&self) -> CH_TX_ERR_R {
        CH_TX_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "The interrupt enable bit for CH(0-3)_TX_THR_EVENT_INT."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH0_TX_THR_EVENT` field.</div>"]
    #[inline(always)]
    pub fn ch_tx_thr_event(&self, n: u8) -> CH_TX_THR_EVENT_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_TX_THR_EVENT_R::new(((self.bits >> (n + 8)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The interrupt enable bit for CH(0-3)_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch_tx_thr_event_iter(&self) -> impl Iterator<Item = CH_TX_THR_EVENT_R> + '_ {
        (0..4).map(move |n| CH_TX_THR_EVENT_R::new(((self.bits >> (n + 8)) & 1) != 0))
    }
    #[doc = "Bit 8 - The interrupt enable bit for CH0_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch0_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt enable bit for CH1_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch1_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The interrupt enable bit for CH2_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch2_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The interrupt enable bit for CH3_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch3_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "The interrupt enable bit for CH(0-3)_TX_LOOP_INT."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH0_TX_LOOP` field.</div>"]
    #[inline(always)]
    pub fn ch_tx_loop(&self, n: u8) -> CH_TX_LOOP_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_TX_LOOP_R::new(((self.bits >> (n + 12)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The interrupt enable bit for CH(0-3)_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch_tx_loop_iter(&self) -> impl Iterator<Item = CH_TX_LOOP_R> + '_ {
        (0..4).map(move |n| CH_TX_LOOP_R::new(((self.bits >> (n + 12)) & 1) != 0))
    }
    #[doc = "Bit 12 - The interrupt enable bit for CH0_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch0_tx_loop(&self) -> CH_TX_LOOP_R {
        CH_TX_LOOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The interrupt enable bit for CH1_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch1_tx_loop(&self) -> CH_TX_LOOP_R {
        CH_TX_LOOP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The interrupt enable bit for CH2_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch2_tx_loop(&self) -> CH_TX_LOOP_R {
        CH_TX_LOOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The interrupt enable bit for CH3_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch3_tx_loop(&self) -> CH_TX_LOOP_R {
        CH_TX_LOOP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "The interrupt enable bit for CH(4-7)_RX_END_INT."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH4_RX_END` field.</div>"]
    #[inline(always)]
    pub fn ch_rx_end(&self, n: u8) -> CH_RX_END_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_RX_END_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The interrupt enable bit for CH(4-7)_RX_END_INT."]
    #[inline(always)]
    pub fn ch_rx_end_iter(&self) -> impl Iterator<Item = CH_RX_END_R> + '_ {
        (0..4).map(move |n| CH_RX_END_R::new(((self.bits >> (n + 16)) & 1) != 0))
    }
    #[doc = "Bit 16 - The interrupt enable bit for CH4_RX_END_INT."]
    #[inline(always)]
    pub fn ch4_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The interrupt enable bit for CH5_RX_END_INT."]
    #[inline(always)]
    pub fn ch5_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The interrupt enable bit for CH6_RX_END_INT."]
    #[inline(always)]
    pub fn ch6_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The interrupt enable bit for CH7_RX_END_INT."]
    #[inline(always)]
    pub fn ch7_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "The interrupt enable bit for CH(4-7)_ERR_INT."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH4_RX_ERR` field.</div>"]
    #[inline(always)]
    pub fn ch_rx_err(&self, n: u8) -> CH_RX_ERR_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_RX_ERR_R::new(((self.bits >> (n + 20)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The interrupt enable bit for CH(4-7)_ERR_INT."]
    #[inline(always)]
    pub fn ch_rx_err_iter(&self) -> impl Iterator<Item = CH_RX_ERR_R> + '_ {
        (0..4).map(move |n| CH_RX_ERR_R::new(((self.bits >> (n + 20)) & 1) != 0))
    }
    #[doc = "Bit 20 - The interrupt enable bit for CH4_ERR_INT."]
    #[inline(always)]
    pub fn ch4_rx_err(&self) -> CH_RX_ERR_R {
        CH_RX_ERR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The interrupt enable bit for CH5_ERR_INT."]
    #[inline(always)]
    pub fn ch5_rx_err(&self) -> CH_RX_ERR_R {
        CH_RX_ERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - The interrupt enable bit for CH6_ERR_INT."]
    #[inline(always)]
    pub fn ch6_rx_err(&self) -> CH_RX_ERR_R {
        CH_RX_ERR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - The interrupt enable bit for CH7_ERR_INT."]
    #[inline(always)]
    pub fn ch7_rx_err(&self) -> CH_RX_ERR_R {
        CH_RX_ERR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "The interrupt enable bit for CH(4-7)_RX_THR_EVENT_INT."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH4_RX_THR_EVENT` field.</div>"]
    #[inline(always)]
    pub fn ch_rx_thr_event(&self, n: u8) -> CH_RX_THR_EVENT_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_RX_THR_EVENT_R::new(((self.bits >> (n + 24)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The interrupt enable bit for CH(4-7)_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch_rx_thr_event_iter(&self) -> impl Iterator<Item = CH_RX_THR_EVENT_R> + '_ {
        (0..4).map(move |n| CH_RX_THR_EVENT_R::new(((self.bits >> (n + 24)) & 1) != 0))
    }
    #[doc = "Bit 24 - The interrupt enable bit for CH4_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch4_rx_thr_event(&self) -> CH_RX_THR_EVENT_R {
        CH_RX_THR_EVENT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - The interrupt enable bit for CH5_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch5_rx_thr_event(&self) -> CH_RX_THR_EVENT_R {
        CH_RX_THR_EVENT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - The interrupt enable bit for CH6_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch6_rx_thr_event(&self) -> CH_RX_THR_EVENT_R {
        CH_RX_THR_EVENT_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - The interrupt enable bit for CH7_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch7_rx_thr_event(&self) -> CH_RX_THR_EVENT_R {
        CH_RX_THR_EVENT_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - The interrupt enable bit for CH3_DMA_ACCESS_FAIL_INT."]
    #[inline(always)]
    pub fn tx_ch3_dma_access_fail(&self) -> TX_CH3_DMA_ACCESS_FAIL_R {
        TX_CH3_DMA_ACCESS_FAIL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - The interrupt enable bit for CH7_DMA_ACCESS_FAIL_INT."]
    #[inline(always)]
    pub fn rx_ch7_dma_access_fail(&self) -> RX_CH7_DMA_ACCESS_FAIL_R {
        RX_CH7_DMA_ACCESS_FAIL_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("ch0_tx_end", &self.ch0_tx_end())
            .field("ch1_tx_end", &self.ch1_tx_end())
            .field("ch2_tx_end", &self.ch2_tx_end())
            .field("ch3_tx_end", &self.ch3_tx_end())
            .field("ch0_tx_err", &self.ch0_tx_err())
            .field("ch1_tx_err", &self.ch1_tx_err())
            .field("ch2_tx_err", &self.ch2_tx_err())
            .field("ch3_tx_err", &self.ch3_tx_err())
            .field("ch0_tx_thr_event", &self.ch0_tx_thr_event())
            .field("ch1_tx_thr_event", &self.ch1_tx_thr_event())
            .field("ch2_tx_thr_event", &self.ch2_tx_thr_event())
            .field("ch3_tx_thr_event", &self.ch3_tx_thr_event())
            .field("ch0_tx_loop", &self.ch0_tx_loop())
            .field("ch1_tx_loop", &self.ch1_tx_loop())
            .field("ch2_tx_loop", &self.ch2_tx_loop())
            .field("ch3_tx_loop", &self.ch3_tx_loop())
            .field("ch4_rx_end", &self.ch4_rx_end())
            .field("ch5_rx_end", &self.ch5_rx_end())
            .field("ch6_rx_end", &self.ch6_rx_end())
            .field("ch7_rx_end", &self.ch7_rx_end())
            .field("ch4_rx_err", &self.ch4_rx_err())
            .field("ch5_rx_err", &self.ch5_rx_err())
            .field("ch6_rx_err", &self.ch6_rx_err())
            .field("ch7_rx_err", &self.ch7_rx_err())
            .field("ch4_rx_thr_event", &self.ch4_rx_thr_event())
            .field("ch5_rx_thr_event", &self.ch5_rx_thr_event())
            .field("ch6_rx_thr_event", &self.ch6_rx_thr_event())
            .field("ch7_rx_thr_event", &self.ch7_rx_thr_event())
            .field("tx_ch3_dma_access_fail", &self.tx_ch3_dma_access_fail())
            .field("rx_ch7_dma_access_fail", &self.rx_ch7_dma_access_fail())
            .finish()
    }
}
impl W {
    #[doc = "The interrupt enable bit for CH(0-3)_TX_END_INT."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH0_TX_END` field.</div>"]
    #[inline(always)]
    pub fn ch_tx_end(&mut self, n: u8) -> CH_TX_END_W<INT_ENA_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_TX_END_W::new(self, n)
    }
    #[doc = "Bit 0 - The interrupt enable bit for CH0_TX_END_INT."]
    #[inline(always)]
    pub fn ch0_tx_end(&mut self) -> CH_TX_END_W<INT_ENA_SPEC> {
        CH_TX_END_W::new(self, 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for CH1_TX_END_INT."]
    #[inline(always)]
    pub fn ch1_tx_end(&mut self) -> CH_TX_END_W<INT_ENA_SPEC> {
        CH_TX_END_W::new(self, 1)
    }
    #[doc = "Bit 2 - The interrupt enable bit for CH2_TX_END_INT."]
    #[inline(always)]
    pub fn ch2_tx_end(&mut self) -> CH_TX_END_W<INT_ENA_SPEC> {
        CH_TX_END_W::new(self, 2)
    }
    #[doc = "Bit 3 - The interrupt enable bit for CH3_TX_END_INT."]
    #[inline(always)]
    pub fn ch3_tx_end(&mut self) -> CH_TX_END_W<INT_ENA_SPEC> {
        CH_TX_END_W::new(self, 3)
    }
    #[doc = "The interrupt enable bit for CH(0-3)_ERR_INT."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH0_TX_ERR` field.</div>"]
    #[inline(always)]
    pub fn ch_tx_err(&mut self, n: u8) -> CH_TX_ERR_W<INT_ENA_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_TX_ERR_W::new(self, n + 4)
    }
    #[doc = "Bit 4 - The interrupt enable bit for CH0_ERR_INT."]
    #[inline(always)]
    pub fn ch0_tx_err(&mut self) -> CH_TX_ERR_W<INT_ENA_SPEC> {
        CH_TX_ERR_W::new(self, 4)
    }
    #[doc = "Bit 5 - The interrupt enable bit for CH1_ERR_INT."]
    #[inline(always)]
    pub fn ch1_tx_err(&mut self) -> CH_TX_ERR_W<INT_ENA_SPEC> {
        CH_TX_ERR_W::new(self, 5)
    }
    #[doc = "Bit 6 - The interrupt enable bit for CH2_ERR_INT."]
    #[inline(always)]
    pub fn ch2_tx_err(&mut self) -> CH_TX_ERR_W<INT_ENA_SPEC> {
        CH_TX_ERR_W::new(self, 6)
    }
    #[doc = "Bit 7 - The interrupt enable bit for CH3_ERR_INT."]
    #[inline(always)]
    pub fn ch3_tx_err(&mut self) -> CH_TX_ERR_W<INT_ENA_SPEC> {
        CH_TX_ERR_W::new(self, 7)
    }
    #[doc = "The interrupt enable bit for CH(0-3)_TX_THR_EVENT_INT."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH0_TX_THR_EVENT` field.</div>"]
    #[inline(always)]
    pub fn ch_tx_thr_event(&mut self, n: u8) -> CH_TX_THR_EVENT_W<INT_ENA_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_TX_THR_EVENT_W::new(self, n + 8)
    }
    #[doc = "Bit 8 - The interrupt enable bit for CH0_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch0_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_ENA_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 8)
    }
    #[doc = "Bit 9 - The interrupt enable bit for CH1_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch1_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_ENA_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 9)
    }
    #[doc = "Bit 10 - The interrupt enable bit for CH2_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch2_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_ENA_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 10)
    }
    #[doc = "Bit 11 - The interrupt enable bit for CH3_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch3_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_ENA_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 11)
    }
    #[doc = "The interrupt enable bit for CH(0-3)_TX_LOOP_INT."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH0_TX_LOOP` field.</div>"]
    #[inline(always)]
    pub fn ch_tx_loop(&mut self, n: u8) -> CH_TX_LOOP_W<INT_ENA_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_TX_LOOP_W::new(self, n + 12)
    }
    #[doc = "Bit 12 - The interrupt enable bit for CH0_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch0_tx_loop(&mut self) -> CH_TX_LOOP_W<INT_ENA_SPEC> {
        CH_TX_LOOP_W::new(self, 12)
    }
    #[doc = "Bit 13 - The interrupt enable bit for CH1_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch1_tx_loop(&mut self) -> CH_TX_LOOP_W<INT_ENA_SPEC> {
        CH_TX_LOOP_W::new(self, 13)
    }
    #[doc = "Bit 14 - The interrupt enable bit for CH2_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch2_tx_loop(&mut self) -> CH_TX_LOOP_W<INT_ENA_SPEC> {
        CH_TX_LOOP_W::new(self, 14)
    }
    #[doc = "Bit 15 - The interrupt enable bit for CH3_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch3_tx_loop(&mut self) -> CH_TX_LOOP_W<INT_ENA_SPEC> {
        CH_TX_LOOP_W::new(self, 15)
    }
    #[doc = "The interrupt enable bit for CH(4-7)_RX_END_INT."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH4_RX_END` field.</div>"]
    #[inline(always)]
    pub fn ch_rx_end(&mut self, n: u8) -> CH_RX_END_W<INT_ENA_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_RX_END_W::new(self, n + 16)
    }
    #[doc = "Bit 16 - The interrupt enable bit for CH4_RX_END_INT."]
    #[inline(always)]
    pub fn ch4_rx_end(&mut self) -> CH_RX_END_W<INT_ENA_SPEC> {
        CH_RX_END_W::new(self, 16)
    }
    #[doc = "Bit 17 - The interrupt enable bit for CH5_RX_END_INT."]
    #[inline(always)]
    pub fn ch5_rx_end(&mut self) -> CH_RX_END_W<INT_ENA_SPEC> {
        CH_RX_END_W::new(self, 17)
    }
    #[doc = "Bit 18 - The interrupt enable bit for CH6_RX_END_INT."]
    #[inline(always)]
    pub fn ch6_rx_end(&mut self) -> CH_RX_END_W<INT_ENA_SPEC> {
        CH_RX_END_W::new(self, 18)
    }
    #[doc = "Bit 19 - The interrupt enable bit for CH7_RX_END_INT."]
    #[inline(always)]
    pub fn ch7_rx_end(&mut self) -> CH_RX_END_W<INT_ENA_SPEC> {
        CH_RX_END_W::new(self, 19)
    }
    #[doc = "The interrupt enable bit for CH(4-7)_ERR_INT."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH4_RX_ERR` field.</div>"]
    #[inline(always)]
    pub fn ch_rx_err(&mut self, n: u8) -> CH_RX_ERR_W<INT_ENA_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_RX_ERR_W::new(self, n + 20)
    }
    #[doc = "Bit 20 - The interrupt enable bit for CH4_ERR_INT."]
    #[inline(always)]
    pub fn ch4_rx_err(&mut self) -> CH_RX_ERR_W<INT_ENA_SPEC> {
        CH_RX_ERR_W::new(self, 20)
    }
    #[doc = "Bit 21 - The interrupt enable bit for CH5_ERR_INT."]
    #[inline(always)]
    pub fn ch5_rx_err(&mut self) -> CH_RX_ERR_W<INT_ENA_SPEC> {
        CH_RX_ERR_W::new(self, 21)
    }
    #[doc = "Bit 22 - The interrupt enable bit for CH6_ERR_INT."]
    #[inline(always)]
    pub fn ch6_rx_err(&mut self) -> CH_RX_ERR_W<INT_ENA_SPEC> {
        CH_RX_ERR_W::new(self, 22)
    }
    #[doc = "Bit 23 - The interrupt enable bit for CH7_ERR_INT."]
    #[inline(always)]
    pub fn ch7_rx_err(&mut self) -> CH_RX_ERR_W<INT_ENA_SPEC> {
        CH_RX_ERR_W::new(self, 23)
    }
    #[doc = "The interrupt enable bit for CH(4-7)_RX_THR_EVENT_INT."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH4_RX_THR_EVENT` field.</div>"]
    #[inline(always)]
    pub fn ch_rx_thr_event(&mut self, n: u8) -> CH_RX_THR_EVENT_W<INT_ENA_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_RX_THR_EVENT_W::new(self, n + 24)
    }
    #[doc = "Bit 24 - The interrupt enable bit for CH4_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch4_rx_thr_event(&mut self) -> CH_RX_THR_EVENT_W<INT_ENA_SPEC> {
        CH_RX_THR_EVENT_W::new(self, 24)
    }
    #[doc = "Bit 25 - The interrupt enable bit for CH5_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch5_rx_thr_event(&mut self) -> CH_RX_THR_EVENT_W<INT_ENA_SPEC> {
        CH_RX_THR_EVENT_W::new(self, 25)
    }
    #[doc = "Bit 26 - The interrupt enable bit for CH6_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch6_rx_thr_event(&mut self) -> CH_RX_THR_EVENT_W<INT_ENA_SPEC> {
        CH_RX_THR_EVENT_W::new(self, 26)
    }
    #[doc = "Bit 27 - The interrupt enable bit for CH7_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch7_rx_thr_event(&mut self) -> CH_RX_THR_EVENT_W<INT_ENA_SPEC> {
        CH_RX_THR_EVENT_W::new(self, 27)
    }
    #[doc = "Bit 28 - The interrupt enable bit for CH3_DMA_ACCESS_FAIL_INT."]
    #[inline(always)]
    pub fn tx_ch3_dma_access_fail(&mut self) -> TX_CH3_DMA_ACCESS_FAIL_W<INT_ENA_SPEC> {
        TX_CH3_DMA_ACCESS_FAIL_W::new(self, 28)
    }
    #[doc = "Bit 29 - The interrupt enable bit for CH7_DMA_ACCESS_FAIL_INT."]
    #[inline(always)]
    pub fn rx_ch7_dma_access_fail(&mut self) -> RX_CH7_DMA_ACCESS_FAIL_W<INT_ENA_SPEC> {
        RX_CH7_DMA_ACCESS_FAIL_W::new(self, 29)
    }
}
#[doc = "Interrupt enable bits\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {}
