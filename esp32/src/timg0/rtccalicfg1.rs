#[doc = "Register `RTCCALICFG1` reader"]
pub type R = crate::R<RTCCALICFG1_SPEC>;
#[doc = "Field `RTC_CALI_VALUE` reader - "]
pub type RTC_CALI_VALUE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 7:31"]
    #[inline(always)]
    pub fn rtc_cali_value(&self) -> RTC_CALI_VALUE_R {
        RTC_CALI_VALUE_R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTCCALICFG1")
            .field("rtc_cali_value", &self.rtc_cali_value())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rtccalicfg1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTCCALICFG1_SPEC;
impl crate::RegisterSpec for RTCCALICFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtccalicfg1::R`](R) reader structure"]
impl crate::Readable for RTCCALICFG1_SPEC {}
#[doc = "`reset()` method sets RTCCALICFG1 to value 0"]
impl crate::Resettable for RTCCALICFG1_SPEC {}
