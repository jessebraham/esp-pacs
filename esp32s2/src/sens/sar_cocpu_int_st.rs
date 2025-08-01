#[doc = "Register `SAR_COCPU_INT_ST` reader"]
pub type R = crate::R<SAR_COCPU_INT_ST_SPEC>;
#[doc = "Field `COCPU_TOUCH_DONE_INT_ST` reader - TOUCH_DONE_INT interrupt status bit"]
pub type COCPU_TOUCH_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `COCPU_TOUCH_INACTIVE_INT_ST` reader - TOUCH_INACTIVE_INT interrupt status bit"]
pub type COCPU_TOUCH_INACTIVE_INT_ST_R = crate::BitReader;
#[doc = "Field `COCPU_TOUCH_ACTIVE_INT_ST` reader - TOUCH_ACTIVE_INT interrupt status bit"]
pub type COCPU_TOUCH_ACTIVE_INT_ST_R = crate::BitReader;
#[doc = "Field `COCPU_SARADC1_INT_ST` reader - SARADC1_DONE_INT interrupt status bit"]
pub type COCPU_SARADC1_INT_ST_R = crate::BitReader;
#[doc = "Field `COCPU_SARADC2_INT_ST` reader - SARADC2_DONE_INT interrupt status bit"]
pub type COCPU_SARADC2_INT_ST_R = crate::BitReader;
#[doc = "Field `COCPU_TSENS_INT_ST` reader - TSENS_DONE_INT interrupt status bit"]
pub type COCPU_TSENS_INT_ST_R = crate::BitReader;
#[doc = "Field `COCPU_START_INT_ST` reader - RISCV_START_INT interrupt status bit"]
pub type COCPU_START_INT_ST_R = crate::BitReader;
#[doc = "Field `COCPU_SW_INT_ST` reader - SW_INT interrupt status bit"]
pub type COCPU_SW_INT_ST_R = crate::BitReader;
#[doc = "Field `COCPU_SWD_INT_ST` reader - SWD_INT interrupt status bit"]
pub type COCPU_SWD_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TOUCH_DONE_INT interrupt status bit"]
    #[inline(always)]
    pub fn cocpu_touch_done_int_st(&self) -> COCPU_TOUCH_DONE_INT_ST_R {
        COCPU_TOUCH_DONE_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TOUCH_INACTIVE_INT interrupt status bit"]
    #[inline(always)]
    pub fn cocpu_touch_inactive_int_st(&self) -> COCPU_TOUCH_INACTIVE_INT_ST_R {
        COCPU_TOUCH_INACTIVE_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TOUCH_ACTIVE_INT interrupt status bit"]
    #[inline(always)]
    pub fn cocpu_touch_active_int_st(&self) -> COCPU_TOUCH_ACTIVE_INT_ST_R {
        COCPU_TOUCH_ACTIVE_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SARADC1_DONE_INT interrupt status bit"]
    #[inline(always)]
    pub fn cocpu_saradc1_int_st(&self) -> COCPU_SARADC1_INT_ST_R {
        COCPU_SARADC1_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SARADC2_DONE_INT interrupt status bit"]
    #[inline(always)]
    pub fn cocpu_saradc2_int_st(&self) -> COCPU_SARADC2_INT_ST_R {
        COCPU_SARADC2_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TSENS_DONE_INT interrupt status bit"]
    #[inline(always)]
    pub fn cocpu_tsens_int_st(&self) -> COCPU_TSENS_INT_ST_R {
        COCPU_TSENS_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RISCV_START_INT interrupt status bit"]
    #[inline(always)]
    pub fn cocpu_start_int_st(&self) -> COCPU_START_INT_ST_R {
        COCPU_START_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SW_INT interrupt status bit"]
    #[inline(always)]
    pub fn cocpu_sw_int_st(&self) -> COCPU_SW_INT_ST_R {
        COCPU_SW_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SWD_INT interrupt status bit"]
    #[inline(always)]
    pub fn cocpu_swd_int_st(&self) -> COCPU_SWD_INT_ST_R {
        COCPU_SWD_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_COCPU_INT_ST")
            .field("cocpu_touch_done_int_st", &self.cocpu_touch_done_int_st())
            .field(
                "cocpu_touch_inactive_int_st",
                &self.cocpu_touch_inactive_int_st(),
            )
            .field(
                "cocpu_touch_active_int_st",
                &self.cocpu_touch_active_int_st(),
            )
            .field("cocpu_saradc1_int_st", &self.cocpu_saradc1_int_st())
            .field("cocpu_saradc2_int_st", &self.cocpu_saradc2_int_st())
            .field("cocpu_tsens_int_st", &self.cocpu_tsens_int_st())
            .field("cocpu_start_int_st", &self.cocpu_start_int_st())
            .field("cocpu_sw_int_st", &self.cocpu_sw_int_st())
            .field("cocpu_swd_int_st", &self.cocpu_swd_int_st())
            .finish()
    }
}
#[doc = "Interrupt status bit of ULP-RISCV\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_cocpu_int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_COCPU_INT_ST_SPEC;
impl crate::RegisterSpec for SAR_COCPU_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_cocpu_int_st::R`](R) reader structure"]
impl crate::Readable for SAR_COCPU_INT_ST_SPEC {}
#[doc = "`reset()` method sets SAR_COCPU_INT_ST to value 0"]
impl crate::Resettable for SAR_COCPU_INT_ST_SPEC {}
