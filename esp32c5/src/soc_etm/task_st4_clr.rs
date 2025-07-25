#[doc = "Register `TASK_ST4_CLR` writer"]
pub type W = crate::W<TASK_ST4_CLR_SPEC>;
#[doc = "Field `REGDMA_TASK_START2_ST_CLR` writer - Configures whether or not to clear REGDMA_task_start2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type REGDMA_TASK_START2_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGDMA_TASK_START3_ST_CLR` writer - Configures whether or not to clear REGDMA_task_start3 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type REGDMA_TASK_START3_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMPSNSR_TASK_START_SAMPLE_ST_CLR` writer - Configures whether or not to clear TMPSNSR_task_start_sample trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type TMPSNSR_TASK_START_SAMPLE_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMPSNSR_TASK_STOP_SAMPLE_ST_CLR` writer - Configures whether or not to clear TMPSNSR_task_stop_sample trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type TMPSNSR_TASK_STOP_SAMPLE_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S0_TASK_START_RX_ST_CLR` writer - Configures whether or not to clear I2S0_task_start_rx trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type I2S0_TASK_START_RX_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S0_TASK_START_TX_ST_CLR` writer - Configures whether or not to clear I2S0_task_start_tx trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type I2S0_TASK_START_TX_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S0_TASK_STOP_RX_ST_CLR` writer - Configures whether or not to clear I2S0_task_stop_rx trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type I2S0_TASK_STOP_RX_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S0_TASK_STOP_TX_ST_CLR` writer - Configures whether or not to clear I2S0_task_stop_tx trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type I2S0_TASK_STOP_TX_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULP_TASK_WAKEUP_CPU_ST_CLR` writer - Configures whether or not to clear ULP_task_wakeup_cpu trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type ULP_TASK_WAKEUP_CPU_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULP_TASK_INT_CPU_ST_CLR` writer - Configures whether or not to clear ULP_task_int_cpu trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type ULP_TASK_INT_CPU_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_TASK_START_ST_CLR` writer - Configures whether or not to clear RTC_task_start trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type RTC_TASK_START_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_TASK_STOP_ST_CLR` writer - Configures whether or not to clear RTC_task_stop trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type RTC_TASK_STOP_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_TASK_CLR_ST_CLR` writer - Configures whether or not to clear RTC_task_clr trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type RTC_TASK_CLR_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_TASK_TRIGGERFLW_ST_CLR` writer - Configures whether or not to clear RTC_task_triggerflw trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type RTC_TASK_TRIGGERFLW_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_TASK_IN_START_CH0_ST_CLR` writer - Configures whether or not to clear GDMA_task_in_start_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type GDMA_TASK_IN_START_CH0_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_TASK_IN_START_CH1_ST_CLR` writer - Configures whether or not to clear GDMA_task_in_start_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type GDMA_TASK_IN_START_CH1_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_TASK_IN_START_CH2_ST_CLR` writer - Configures whether or not to clear GDMA_task_in_start_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type GDMA_TASK_IN_START_CH2_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_TASK_OUT_START_CH0_ST_CLR` writer - Configures whether or not to clear GDMA_task_out_start_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type GDMA_TASK_OUT_START_CH0_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_TASK_OUT_START_CH1_ST_CLR` writer - Configures whether or not to clear GDMA_task_out_start_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type GDMA_TASK_OUT_START_CH1_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_TASK_OUT_START_CH2_ST_CLR` writer - Configures whether or not to clear GDMA_task_out_start_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type GDMA_TASK_OUT_START_CH2_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMU_TASK_SLEEP_REQ_ST_CLR` writer - Configures whether or not to clear PMU_task_sleep_req trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PMU_TASK_SLEEP_REQ_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TASK_ST4_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to clear REGDMA_task_start2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn regdma_task_start2_st_clr(&mut self) -> REGDMA_TASK_START2_ST_CLR_W<TASK_ST4_CLR_SPEC> {
        REGDMA_TASK_START2_ST_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not to clear REGDMA_task_start3 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn regdma_task_start3_st_clr(&mut self) -> REGDMA_TASK_START3_ST_CLR_W<TASK_ST4_CLR_SPEC> {
        REGDMA_TASK_START3_ST_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether or not to clear TMPSNSR_task_start_sample trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn tmpsnsr_task_start_sample_st_clr(
        &mut self,
    ) -> TMPSNSR_TASK_START_SAMPLE_ST_CLR_W<TASK_ST4_CLR_SPEC> {
        TMPSNSR_TASK_START_SAMPLE_ST_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures whether or not to clear TMPSNSR_task_stop_sample trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn tmpsnsr_task_stop_sample_st_clr(
        &mut self,
    ) -> TMPSNSR_TASK_STOP_SAMPLE_ST_CLR_W<TASK_ST4_CLR_SPEC> {
        TMPSNSR_TASK_STOP_SAMPLE_ST_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether or not to clear I2S0_task_start_rx trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn i2s0_task_start_rx_st_clr(&mut self) -> I2S0_TASK_START_RX_ST_CLR_W<TASK_ST4_CLR_SPEC> {
        I2S0_TASK_START_RX_ST_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures whether or not to clear I2S0_task_start_tx trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn i2s0_task_start_tx_st_clr(&mut self) -> I2S0_TASK_START_TX_ST_CLR_W<TASK_ST4_CLR_SPEC> {
        I2S0_TASK_START_TX_ST_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures whether or not to clear I2S0_task_stop_rx trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn i2s0_task_stop_rx_st_clr(&mut self) -> I2S0_TASK_STOP_RX_ST_CLR_W<TASK_ST4_CLR_SPEC> {
        I2S0_TASK_STOP_RX_ST_CLR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures whether or not to clear I2S0_task_stop_tx trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn i2s0_task_stop_tx_st_clr(&mut self) -> I2S0_TASK_STOP_TX_ST_CLR_W<TASK_ST4_CLR_SPEC> {
        I2S0_TASK_STOP_TX_ST_CLR_W::new(self, 7)
    }
    #[doc = "Bit 8 - Configures whether or not to clear ULP_task_wakeup_cpu trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ulp_task_wakeup_cpu_st_clr(
        &mut self,
    ) -> ULP_TASK_WAKEUP_CPU_ST_CLR_W<TASK_ST4_CLR_SPEC> {
        ULP_TASK_WAKEUP_CPU_ST_CLR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Configures whether or not to clear ULP_task_int_cpu trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ulp_task_int_cpu_st_clr(&mut self) -> ULP_TASK_INT_CPU_ST_CLR_W<TASK_ST4_CLR_SPEC> {
        ULP_TASK_INT_CPU_ST_CLR_W::new(self, 9)
    }
    #[doc = "Bit 10 - Configures whether or not to clear RTC_task_start trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn rtc_task_start_st_clr(&mut self) -> RTC_TASK_START_ST_CLR_W<TASK_ST4_CLR_SPEC> {
        RTC_TASK_START_ST_CLR_W::new(self, 10)
    }
    #[doc = "Bit 11 - Configures whether or not to clear RTC_task_stop trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn rtc_task_stop_st_clr(&mut self) -> RTC_TASK_STOP_ST_CLR_W<TASK_ST4_CLR_SPEC> {
        RTC_TASK_STOP_ST_CLR_W::new(self, 11)
    }
    #[doc = "Bit 12 - Configures whether or not to clear RTC_task_clr trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn rtc_task_clr_st_clr(&mut self) -> RTC_TASK_CLR_ST_CLR_W<TASK_ST4_CLR_SPEC> {
        RTC_TASK_CLR_ST_CLR_W::new(self, 12)
    }
    #[doc = "Bit 13 - Configures whether or not to clear RTC_task_triggerflw trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn rtc_task_triggerflw_st_clr(
        &mut self,
    ) -> RTC_TASK_TRIGGERFLW_ST_CLR_W<TASK_ST4_CLR_SPEC> {
        RTC_TASK_TRIGGERFLW_ST_CLR_W::new(self, 13)
    }
    #[doc = "Bit 14 - Configures whether or not to clear GDMA_task_in_start_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn gdma_task_in_start_ch0_st_clr(
        &mut self,
    ) -> GDMA_TASK_IN_START_CH0_ST_CLR_W<TASK_ST4_CLR_SPEC> {
        GDMA_TASK_IN_START_CH0_ST_CLR_W::new(self, 14)
    }
    #[doc = "Bit 15 - Configures whether or not to clear GDMA_task_in_start_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn gdma_task_in_start_ch1_st_clr(
        &mut self,
    ) -> GDMA_TASK_IN_START_CH1_ST_CLR_W<TASK_ST4_CLR_SPEC> {
        GDMA_TASK_IN_START_CH1_ST_CLR_W::new(self, 15)
    }
    #[doc = "Bit 16 - Configures whether or not to clear GDMA_task_in_start_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn gdma_task_in_start_ch2_st_clr(
        &mut self,
    ) -> GDMA_TASK_IN_START_CH2_ST_CLR_W<TASK_ST4_CLR_SPEC> {
        GDMA_TASK_IN_START_CH2_ST_CLR_W::new(self, 16)
    }
    #[doc = "Bit 17 - Configures whether or not to clear GDMA_task_out_start_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn gdma_task_out_start_ch0_st_clr(
        &mut self,
    ) -> GDMA_TASK_OUT_START_CH0_ST_CLR_W<TASK_ST4_CLR_SPEC> {
        GDMA_TASK_OUT_START_CH0_ST_CLR_W::new(self, 17)
    }
    #[doc = "Bit 18 - Configures whether or not to clear GDMA_task_out_start_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn gdma_task_out_start_ch1_st_clr(
        &mut self,
    ) -> GDMA_TASK_OUT_START_CH1_ST_CLR_W<TASK_ST4_CLR_SPEC> {
        GDMA_TASK_OUT_START_CH1_ST_CLR_W::new(self, 18)
    }
    #[doc = "Bit 19 - Configures whether or not to clear GDMA_task_out_start_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn gdma_task_out_start_ch2_st_clr(
        &mut self,
    ) -> GDMA_TASK_OUT_START_CH2_ST_CLR_W<TASK_ST4_CLR_SPEC> {
        GDMA_TASK_OUT_START_CH2_ST_CLR_W::new(self, 19)
    }
    #[doc = "Bit 20 - Configures whether or not to clear PMU_task_sleep_req trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pmu_task_sleep_req_st_clr(&mut self) -> PMU_TASK_SLEEP_REQ_ST_CLR_W<TASK_ST4_CLR_SPEC> {
        PMU_TASK_SLEEP_REQ_ST_CLR_W::new(self, 20)
    }
}
#[doc = "Tasks trigger status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_st4_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TASK_ST4_CLR_SPEC;
impl crate::RegisterSpec for TASK_ST4_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`task_st4_clr::W`](W) writer structure"]
impl crate::Writable for TASK_ST4_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASK_ST4_CLR to value 0"]
impl crate::Resettable for TASK_ST4_CLR_SPEC {}
