#[doc = "Register `HIST_BIN14` reader"]
pub type R = crate::R<HIST_BIN14_SPEC>;
#[doc = "Field `HIST_BIN_14` reader - this field represents result of histogram bin 14"]
pub type HIST_BIN_14_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:16 - this field represents result of histogram bin 14"]
    #[inline(always)]
    pub fn hist_bin_14(&self) -> HIST_BIN_14_R {
        HIST_BIN_14_R::new(self.bits & 0x0001_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HIST_BIN14")
            .field("hist_bin_14", &self.hist_bin_14())
            .finish()
    }
}
#[doc = "result of histogram bin 14\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_bin14::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HIST_BIN14_SPEC;
impl crate::RegisterSpec for HIST_BIN14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_bin14::R`](R) reader structure"]
impl crate::Readable for HIST_BIN14_SPEC {}
#[doc = "`reset()` method sets HIST_BIN14 to value 0"]
impl crate::Resettable for HIST_BIN14_SPEC {}
