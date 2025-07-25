#[doc = "Register `SAR_TOUCH_OUT1` reader"]
pub type R = crate::R<SAR_TOUCH_OUT1_SPEC>;
#[doc = "Field `TOUCH_MEAS_OUT1` reader - the counter for touch pad 1"]
pub type TOUCH_MEAS_OUT1_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_MEAS_OUT0` reader - the counter for touch pad 0"]
pub type TOUCH_MEAS_OUT0_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - the counter for touch pad 1"]
    #[inline(always)]
    pub fn touch_meas_out1(&self) -> TOUCH_MEAS_OUT1_R {
        TOUCH_MEAS_OUT1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - the counter for touch pad 0"]
    #[inline(always)]
    pub fn touch_meas_out0(&self) -> TOUCH_MEAS_OUT0_R {
        TOUCH_MEAS_OUT0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_OUT1")
            .field("touch_meas_out1", &self.touch_meas_out1())
            .field("touch_meas_out0", &self.touch_meas_out0())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_out1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_TOUCH_OUT1_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_OUT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_touch_out1::R`](R) reader structure"]
impl crate::Readable for SAR_TOUCH_OUT1_SPEC {}
#[doc = "`reset()` method sets SAR_TOUCH_OUT1 to value 0"]
impl crate::Resettable for SAR_TOUCH_OUT1_SPEC {}
