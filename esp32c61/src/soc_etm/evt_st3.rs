#[doc = "Register `EVT_ST3` reader"]
pub type R = crate::R<EVT_ST3_SPEC>;
#[doc = "Register `EVT_ST3` writer"]
pub type W = crate::W<EVT_ST3_SPEC>;
#[doc = "Field `GDMA_AHB_EVT_IN_FIFO_FULL_CH1_ST` reader - Represents GDMA_AHB_evt_in_fifo_full_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_AHB_EVT_IN_FIFO_FULL_CH1_ST_R = crate::BitReader;
#[doc = "Field `GDMA_AHB_EVT_IN_FIFO_FULL_CH1_ST` writer - Represents GDMA_AHB_evt_in_fifo_full_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_AHB_EVT_IN_FIFO_FULL_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_AHB_EVT_IN_FIFO_FULL_CH2_ST` reader - Represents GDMA_AHB_evt_in_fifo_full_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_AHB_EVT_IN_FIFO_FULL_CH2_ST_R = crate::BitReader;
#[doc = "Field `GDMA_AHB_EVT_IN_FIFO_FULL_CH2_ST` writer - Represents GDMA_AHB_evt_in_fifo_full_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_AHB_EVT_IN_FIFO_FULL_CH2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_AHB_EVT_OUT_DONE_CH0_ST` reader - Represents GDMA_AHB_evt_out_done_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_AHB_EVT_OUT_DONE_CH0_ST_R = crate::BitReader;
#[doc = "Field `GDMA_AHB_EVT_OUT_DONE_CH0_ST` writer - Represents GDMA_AHB_evt_out_done_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_AHB_EVT_OUT_DONE_CH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_AHB_EVT_OUT_DONE_CH1_ST` reader - Represents GDMA_AHB_evt_out_done_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_AHB_EVT_OUT_DONE_CH1_ST_R = crate::BitReader;
#[doc = "Field `GDMA_AHB_EVT_OUT_DONE_CH1_ST` writer - Represents GDMA_AHB_evt_out_done_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_AHB_EVT_OUT_DONE_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_AHB_EVT_OUT_DONE_CH2_ST` reader - Represents GDMA_AHB_evt_out_done_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_AHB_EVT_OUT_DONE_CH2_ST_R = crate::BitReader;
#[doc = "Field `GDMA_AHB_EVT_OUT_DONE_CH2_ST` writer - Represents GDMA_AHB_evt_out_done_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_AHB_EVT_OUT_DONE_CH2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_AHB_EVT_OUT_EOF_CH0_ST` reader - Represents GDMA_AHB_evt_out_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_AHB_EVT_OUT_EOF_CH0_ST_R = crate::BitReader;
#[doc = "Field `GDMA_AHB_EVT_OUT_EOF_CH0_ST` writer - Represents GDMA_AHB_evt_out_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_AHB_EVT_OUT_EOF_CH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_AHB_EVT_OUT_EOF_CH1_ST` reader - Represents GDMA_AHB_evt_out_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_AHB_EVT_OUT_EOF_CH1_ST_R = crate::BitReader;
#[doc = "Field `GDMA_AHB_EVT_OUT_EOF_CH1_ST` writer - Represents GDMA_AHB_evt_out_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_AHB_EVT_OUT_EOF_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_AHB_EVT_OUT_EOF_CH2_ST` reader - Represents GDMA_AHB_evt_out_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_AHB_EVT_OUT_EOF_CH2_ST_R = crate::BitReader;
#[doc = "Field `GDMA_AHB_EVT_OUT_EOF_CH2_ST` writer - Represents GDMA_AHB_evt_out_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_AHB_EVT_OUT_EOF_CH2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_AHB_EVT_OUT_TOTAL_EOF_CH0_ST` reader - Represents GDMA_AHB_evt_out_total_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_AHB_EVT_OUT_TOTAL_EOF_CH0_ST_R = crate::BitReader;
#[doc = "Field `GDMA_AHB_EVT_OUT_TOTAL_EOF_CH0_ST` writer - Represents GDMA_AHB_evt_out_total_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_AHB_EVT_OUT_TOTAL_EOF_CH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_AHB_EVT_OUT_TOTAL_EOF_CH1_ST` reader - Represents GDMA_AHB_evt_out_total_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_AHB_EVT_OUT_TOTAL_EOF_CH1_ST_R = crate::BitReader;
#[doc = "Field `GDMA_AHB_EVT_OUT_TOTAL_EOF_CH1_ST` writer - Represents GDMA_AHB_evt_out_total_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_AHB_EVT_OUT_TOTAL_EOF_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_AHB_EVT_OUT_TOTAL_EOF_CH2_ST` reader - Represents GDMA_AHB_evt_out_total_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_AHB_EVT_OUT_TOTAL_EOF_CH2_ST_R = crate::BitReader;
#[doc = "Field `GDMA_AHB_EVT_OUT_TOTAL_EOF_CH2_ST` writer - Represents GDMA_AHB_evt_out_total_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_AHB_EVT_OUT_TOTAL_EOF_CH2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_AHB_EVT_OUT_FIFO_EMPTY_CH0_ST` reader - Represents GDMA_AHB_evt_out_fifo_empty_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_AHB_EVT_OUT_FIFO_EMPTY_CH0_ST_R = crate::BitReader;
#[doc = "Field `GDMA_AHB_EVT_OUT_FIFO_EMPTY_CH0_ST` writer - Represents GDMA_AHB_evt_out_fifo_empty_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_AHB_EVT_OUT_FIFO_EMPTY_CH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_AHB_EVT_OUT_FIFO_EMPTY_CH1_ST` reader - Represents GDMA_AHB_evt_out_fifo_empty_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_AHB_EVT_OUT_FIFO_EMPTY_CH1_ST_R = crate::BitReader;
#[doc = "Field `GDMA_AHB_EVT_OUT_FIFO_EMPTY_CH1_ST` writer - Represents GDMA_AHB_evt_out_fifo_empty_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_AHB_EVT_OUT_FIFO_EMPTY_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_AHB_EVT_OUT_FIFO_EMPTY_CH2_ST` reader - Represents GDMA_AHB_evt_out_fifo_empty_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_AHB_EVT_OUT_FIFO_EMPTY_CH2_ST_R = crate::BitReader;
#[doc = "Field `GDMA_AHB_EVT_OUT_FIFO_EMPTY_CH2_ST` writer - Represents GDMA_AHB_evt_out_fifo_empty_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_AHB_EVT_OUT_FIFO_EMPTY_CH2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_AHB_EVT_OUT_FIFO_FULL_CH0_ST` reader - Represents GDMA_AHB_evt_out_fifo_full_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_AHB_EVT_OUT_FIFO_FULL_CH0_ST_R = crate::BitReader;
#[doc = "Field `GDMA_AHB_EVT_OUT_FIFO_FULL_CH0_ST` writer - Represents GDMA_AHB_evt_out_fifo_full_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_AHB_EVT_OUT_FIFO_FULL_CH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_AHB_EVT_OUT_FIFO_FULL_CH1_ST` reader - Represents GDMA_AHB_evt_out_fifo_full_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_AHB_EVT_OUT_FIFO_FULL_CH1_ST_R = crate::BitReader;
#[doc = "Field `GDMA_AHB_EVT_OUT_FIFO_FULL_CH1_ST` writer - Represents GDMA_AHB_evt_out_fifo_full_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_AHB_EVT_OUT_FIFO_FULL_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_AHB_EVT_OUT_FIFO_FULL_CH2_ST` reader - Represents GDMA_AHB_evt_out_fifo_full_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_AHB_EVT_OUT_FIFO_FULL_CH2_ST_R = crate::BitReader;
#[doc = "Field `GDMA_AHB_EVT_OUT_FIFO_FULL_CH2_ST` writer - Represents GDMA_AHB_evt_out_fifo_full_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_AHB_EVT_OUT_FIFO_FULL_CH2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMU_EVT_SLEEP_WEEKUP_ST` reader - Represents PMU_evt_sleep_weekup trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PMU_EVT_SLEEP_WEEKUP_ST_R = crate::BitReader;
#[doc = "Field `PMU_EVT_SLEEP_WEEKUP_ST` writer - Represents PMU_evt_sleep_weekup trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PMU_EVT_SLEEP_WEEKUP_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Represents GDMA_AHB_evt_in_fifo_full_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_ahb_evt_in_fifo_full_ch1_st(&self) -> GDMA_AHB_EVT_IN_FIFO_FULL_CH1_ST_R {
        GDMA_AHB_EVT_IN_FIFO_FULL_CH1_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents GDMA_AHB_evt_in_fifo_full_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_ahb_evt_in_fifo_full_ch2_st(&self) -> GDMA_AHB_EVT_IN_FIFO_FULL_CH2_ST_R {
        GDMA_AHB_EVT_IN_FIFO_FULL_CH2_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Represents GDMA_AHB_evt_out_done_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_ahb_evt_out_done_ch0_st(&self) -> GDMA_AHB_EVT_OUT_DONE_CH0_ST_R {
        GDMA_AHB_EVT_OUT_DONE_CH0_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Represents GDMA_AHB_evt_out_done_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_ahb_evt_out_done_ch1_st(&self) -> GDMA_AHB_EVT_OUT_DONE_CH1_ST_R {
        GDMA_AHB_EVT_OUT_DONE_CH1_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Represents GDMA_AHB_evt_out_done_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_ahb_evt_out_done_ch2_st(&self) -> GDMA_AHB_EVT_OUT_DONE_CH2_ST_R {
        GDMA_AHB_EVT_OUT_DONE_CH2_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Represents GDMA_AHB_evt_out_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_ahb_evt_out_eof_ch0_st(&self) -> GDMA_AHB_EVT_OUT_EOF_CH0_ST_R {
        GDMA_AHB_EVT_OUT_EOF_CH0_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Represents GDMA_AHB_evt_out_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_ahb_evt_out_eof_ch1_st(&self) -> GDMA_AHB_EVT_OUT_EOF_CH1_ST_R {
        GDMA_AHB_EVT_OUT_EOF_CH1_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Represents GDMA_AHB_evt_out_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_ahb_evt_out_eof_ch2_st(&self) -> GDMA_AHB_EVT_OUT_EOF_CH2_ST_R {
        GDMA_AHB_EVT_OUT_EOF_CH2_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Represents GDMA_AHB_evt_out_total_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_ahb_evt_out_total_eof_ch0_st(&self) -> GDMA_AHB_EVT_OUT_TOTAL_EOF_CH0_ST_R {
        GDMA_AHB_EVT_OUT_TOTAL_EOF_CH0_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Represents GDMA_AHB_evt_out_total_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_ahb_evt_out_total_eof_ch1_st(&self) -> GDMA_AHB_EVT_OUT_TOTAL_EOF_CH1_ST_R {
        GDMA_AHB_EVT_OUT_TOTAL_EOF_CH1_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Represents GDMA_AHB_evt_out_total_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_ahb_evt_out_total_eof_ch2_st(&self) -> GDMA_AHB_EVT_OUT_TOTAL_EOF_CH2_ST_R {
        GDMA_AHB_EVT_OUT_TOTAL_EOF_CH2_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Represents GDMA_AHB_evt_out_fifo_empty_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_ahb_evt_out_fifo_empty_ch0_st(&self) -> GDMA_AHB_EVT_OUT_FIFO_EMPTY_CH0_ST_R {
        GDMA_AHB_EVT_OUT_FIFO_EMPTY_CH0_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Represents GDMA_AHB_evt_out_fifo_empty_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_ahb_evt_out_fifo_empty_ch1_st(&self) -> GDMA_AHB_EVT_OUT_FIFO_EMPTY_CH1_ST_R {
        GDMA_AHB_EVT_OUT_FIFO_EMPTY_CH1_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Represents GDMA_AHB_evt_out_fifo_empty_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_ahb_evt_out_fifo_empty_ch2_st(&self) -> GDMA_AHB_EVT_OUT_FIFO_EMPTY_CH2_ST_R {
        GDMA_AHB_EVT_OUT_FIFO_EMPTY_CH2_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Represents GDMA_AHB_evt_out_fifo_full_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_ahb_evt_out_fifo_full_ch0_st(&self) -> GDMA_AHB_EVT_OUT_FIFO_FULL_CH0_ST_R {
        GDMA_AHB_EVT_OUT_FIFO_FULL_CH0_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Represents GDMA_AHB_evt_out_fifo_full_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_ahb_evt_out_fifo_full_ch1_st(&self) -> GDMA_AHB_EVT_OUT_FIFO_FULL_CH1_ST_R {
        GDMA_AHB_EVT_OUT_FIFO_FULL_CH1_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Represents GDMA_AHB_evt_out_fifo_full_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_ahb_evt_out_fifo_full_ch2_st(&self) -> GDMA_AHB_EVT_OUT_FIFO_FULL_CH2_ST_R {
        GDMA_AHB_EVT_OUT_FIFO_FULL_CH2_ST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Represents PMU_evt_sleep_weekup trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pmu_evt_sleep_weekup_st(&self) -> PMU_EVT_SLEEP_WEEKUP_ST_R {
        PMU_EVT_SLEEP_WEEKUP_ST_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVT_ST3")
            .field(
                "gdma_ahb_evt_in_fifo_full_ch1_st",
                &self.gdma_ahb_evt_in_fifo_full_ch1_st(),
            )
            .field(
                "gdma_ahb_evt_in_fifo_full_ch2_st",
                &self.gdma_ahb_evt_in_fifo_full_ch2_st(),
            )
            .field(
                "gdma_ahb_evt_out_done_ch0_st",
                &self.gdma_ahb_evt_out_done_ch0_st(),
            )
            .field(
                "gdma_ahb_evt_out_done_ch1_st",
                &self.gdma_ahb_evt_out_done_ch1_st(),
            )
            .field(
                "gdma_ahb_evt_out_done_ch2_st",
                &self.gdma_ahb_evt_out_done_ch2_st(),
            )
            .field(
                "gdma_ahb_evt_out_eof_ch0_st",
                &self.gdma_ahb_evt_out_eof_ch0_st(),
            )
            .field(
                "gdma_ahb_evt_out_eof_ch1_st",
                &self.gdma_ahb_evt_out_eof_ch1_st(),
            )
            .field(
                "gdma_ahb_evt_out_eof_ch2_st",
                &self.gdma_ahb_evt_out_eof_ch2_st(),
            )
            .field(
                "gdma_ahb_evt_out_total_eof_ch0_st",
                &self.gdma_ahb_evt_out_total_eof_ch0_st(),
            )
            .field(
                "gdma_ahb_evt_out_total_eof_ch1_st",
                &self.gdma_ahb_evt_out_total_eof_ch1_st(),
            )
            .field(
                "gdma_ahb_evt_out_total_eof_ch2_st",
                &self.gdma_ahb_evt_out_total_eof_ch2_st(),
            )
            .field(
                "gdma_ahb_evt_out_fifo_empty_ch0_st",
                &self.gdma_ahb_evt_out_fifo_empty_ch0_st(),
            )
            .field(
                "gdma_ahb_evt_out_fifo_empty_ch1_st",
                &self.gdma_ahb_evt_out_fifo_empty_ch1_st(),
            )
            .field(
                "gdma_ahb_evt_out_fifo_empty_ch2_st",
                &self.gdma_ahb_evt_out_fifo_empty_ch2_st(),
            )
            .field(
                "gdma_ahb_evt_out_fifo_full_ch0_st",
                &self.gdma_ahb_evt_out_fifo_full_ch0_st(),
            )
            .field(
                "gdma_ahb_evt_out_fifo_full_ch1_st",
                &self.gdma_ahb_evt_out_fifo_full_ch1_st(),
            )
            .field(
                "gdma_ahb_evt_out_fifo_full_ch2_st",
                &self.gdma_ahb_evt_out_fifo_full_ch2_st(),
            )
            .field("pmu_evt_sleep_weekup_st", &self.pmu_evt_sleep_weekup_st())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Represents GDMA_AHB_evt_in_fifo_full_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_ahb_evt_in_fifo_full_ch1_st(
        &mut self,
    ) -> GDMA_AHB_EVT_IN_FIFO_FULL_CH1_ST_W<EVT_ST3_SPEC> {
        GDMA_AHB_EVT_IN_FIFO_FULL_CH1_ST_W::new(self, 0)
    }
    #[doc = "Bit 1 - Represents GDMA_AHB_evt_in_fifo_full_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_ahb_evt_in_fifo_full_ch2_st(
        &mut self,
    ) -> GDMA_AHB_EVT_IN_FIFO_FULL_CH2_ST_W<EVT_ST3_SPEC> {
        GDMA_AHB_EVT_IN_FIFO_FULL_CH2_ST_W::new(self, 1)
    }
    #[doc = "Bit 2 - Represents GDMA_AHB_evt_out_done_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_ahb_evt_out_done_ch0_st(&mut self) -> GDMA_AHB_EVT_OUT_DONE_CH0_ST_W<EVT_ST3_SPEC> {
        GDMA_AHB_EVT_OUT_DONE_CH0_ST_W::new(self, 2)
    }
    #[doc = "Bit 3 - Represents GDMA_AHB_evt_out_done_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_ahb_evt_out_done_ch1_st(&mut self) -> GDMA_AHB_EVT_OUT_DONE_CH1_ST_W<EVT_ST3_SPEC> {
        GDMA_AHB_EVT_OUT_DONE_CH1_ST_W::new(self, 3)
    }
    #[doc = "Bit 4 - Represents GDMA_AHB_evt_out_done_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_ahb_evt_out_done_ch2_st(&mut self) -> GDMA_AHB_EVT_OUT_DONE_CH2_ST_W<EVT_ST3_SPEC> {
        GDMA_AHB_EVT_OUT_DONE_CH2_ST_W::new(self, 4)
    }
    #[doc = "Bit 5 - Represents GDMA_AHB_evt_out_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_ahb_evt_out_eof_ch0_st(&mut self) -> GDMA_AHB_EVT_OUT_EOF_CH0_ST_W<EVT_ST3_SPEC> {
        GDMA_AHB_EVT_OUT_EOF_CH0_ST_W::new(self, 5)
    }
    #[doc = "Bit 6 - Represents GDMA_AHB_evt_out_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_ahb_evt_out_eof_ch1_st(&mut self) -> GDMA_AHB_EVT_OUT_EOF_CH1_ST_W<EVT_ST3_SPEC> {
        GDMA_AHB_EVT_OUT_EOF_CH1_ST_W::new(self, 6)
    }
    #[doc = "Bit 7 - Represents GDMA_AHB_evt_out_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_ahb_evt_out_eof_ch2_st(&mut self) -> GDMA_AHB_EVT_OUT_EOF_CH2_ST_W<EVT_ST3_SPEC> {
        GDMA_AHB_EVT_OUT_EOF_CH2_ST_W::new(self, 7)
    }
    #[doc = "Bit 8 - Represents GDMA_AHB_evt_out_total_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_ahb_evt_out_total_eof_ch0_st(
        &mut self,
    ) -> GDMA_AHB_EVT_OUT_TOTAL_EOF_CH0_ST_W<EVT_ST3_SPEC> {
        GDMA_AHB_EVT_OUT_TOTAL_EOF_CH0_ST_W::new(self, 8)
    }
    #[doc = "Bit 9 - Represents GDMA_AHB_evt_out_total_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_ahb_evt_out_total_eof_ch1_st(
        &mut self,
    ) -> GDMA_AHB_EVT_OUT_TOTAL_EOF_CH1_ST_W<EVT_ST3_SPEC> {
        GDMA_AHB_EVT_OUT_TOTAL_EOF_CH1_ST_W::new(self, 9)
    }
    #[doc = "Bit 10 - Represents GDMA_AHB_evt_out_total_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_ahb_evt_out_total_eof_ch2_st(
        &mut self,
    ) -> GDMA_AHB_EVT_OUT_TOTAL_EOF_CH2_ST_W<EVT_ST3_SPEC> {
        GDMA_AHB_EVT_OUT_TOTAL_EOF_CH2_ST_W::new(self, 10)
    }
    #[doc = "Bit 11 - Represents GDMA_AHB_evt_out_fifo_empty_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_ahb_evt_out_fifo_empty_ch0_st(
        &mut self,
    ) -> GDMA_AHB_EVT_OUT_FIFO_EMPTY_CH0_ST_W<EVT_ST3_SPEC> {
        GDMA_AHB_EVT_OUT_FIFO_EMPTY_CH0_ST_W::new(self, 11)
    }
    #[doc = "Bit 12 - Represents GDMA_AHB_evt_out_fifo_empty_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_ahb_evt_out_fifo_empty_ch1_st(
        &mut self,
    ) -> GDMA_AHB_EVT_OUT_FIFO_EMPTY_CH1_ST_W<EVT_ST3_SPEC> {
        GDMA_AHB_EVT_OUT_FIFO_EMPTY_CH1_ST_W::new(self, 12)
    }
    #[doc = "Bit 13 - Represents GDMA_AHB_evt_out_fifo_empty_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_ahb_evt_out_fifo_empty_ch2_st(
        &mut self,
    ) -> GDMA_AHB_EVT_OUT_FIFO_EMPTY_CH2_ST_W<EVT_ST3_SPEC> {
        GDMA_AHB_EVT_OUT_FIFO_EMPTY_CH2_ST_W::new(self, 13)
    }
    #[doc = "Bit 14 - Represents GDMA_AHB_evt_out_fifo_full_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_ahb_evt_out_fifo_full_ch0_st(
        &mut self,
    ) -> GDMA_AHB_EVT_OUT_FIFO_FULL_CH0_ST_W<EVT_ST3_SPEC> {
        GDMA_AHB_EVT_OUT_FIFO_FULL_CH0_ST_W::new(self, 14)
    }
    #[doc = "Bit 15 - Represents GDMA_AHB_evt_out_fifo_full_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_ahb_evt_out_fifo_full_ch1_st(
        &mut self,
    ) -> GDMA_AHB_EVT_OUT_FIFO_FULL_CH1_ST_W<EVT_ST3_SPEC> {
        GDMA_AHB_EVT_OUT_FIFO_FULL_CH1_ST_W::new(self, 15)
    }
    #[doc = "Bit 16 - Represents GDMA_AHB_evt_out_fifo_full_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_ahb_evt_out_fifo_full_ch2_st(
        &mut self,
    ) -> GDMA_AHB_EVT_OUT_FIFO_FULL_CH2_ST_W<EVT_ST3_SPEC> {
        GDMA_AHB_EVT_OUT_FIFO_FULL_CH2_ST_W::new(self, 16)
    }
    #[doc = "Bit 17 - Represents PMU_evt_sleep_weekup trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pmu_evt_sleep_weekup_st(&mut self) -> PMU_EVT_SLEEP_WEEKUP_ST_W<EVT_ST3_SPEC> {
        PMU_EVT_SLEEP_WEEKUP_ST_W::new(self, 17)
    }
}
#[doc = "Events trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_st3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVT_ST3_SPEC;
impl crate::RegisterSpec for EVT_ST3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evt_st3::R`](R) reader structure"]
impl crate::Readable for EVT_ST3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`evt_st3::W`](W) writer structure"]
impl crate::Writable for EVT_ST3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVT_ST3 to value 0"]
impl crate::Resettable for EVT_ST3_SPEC {}
