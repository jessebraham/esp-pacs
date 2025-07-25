#[doc = "Register `STATUS0` reader"]
pub type R = crate::R<STATUS0_SPEC>;
#[doc = "Field `CH1_CMPLTD_BLK_TFR_SIZE` reader - NA"]
pub type CH1_CMPLTD_BLK_TFR_SIZE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:21 - NA"]
    #[inline(always)]
    pub fn ch1_cmpltd_blk_tfr_size(&self) -> CH1_CMPLTD_BLK_TFR_SIZE_R {
        CH1_CMPLTD_BLK_TFR_SIZE_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS0")
            .field("ch1_cmpltd_blk_tfr_size", &self.ch1_cmpltd_blk_tfr_size())
            .finish()
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`status0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS0_SPEC;
impl crate::RegisterSpec for STATUS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status0::R`](R) reader structure"]
impl crate::Readable for STATUS0_SPEC {}
#[doc = "`reset()` method sets STATUS0 to value 0"]
impl crate::Resettable for STATUS0_SPEC {}
