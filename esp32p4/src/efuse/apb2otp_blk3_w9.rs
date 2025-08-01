#[doc = "Register `APB2OTP_BLK3_W9` reader"]
pub type R = crate::R<APB2OTP_BLK3_W9_SPEC>;
#[doc = "Field `APB2OTP_BLOCK3_W9` reader - Otp block3 word9 data."]
pub type APB2OTP_BLOCK3_W9_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block3 word9 data."]
    #[inline(always)]
    pub fn apb2otp_block3_w9(&self) -> APB2OTP_BLOCK3_W9_R {
        APB2OTP_BLOCK3_W9_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2OTP_BLK3_W9")
            .field("apb2otp_block3_w9", &self.apb2otp_block3_w9())
            .finish()
    }
}
#[doc = "eFuse apb2otp block3 data register9.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk3_w9::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2OTP_BLK3_W9_SPEC;
impl crate::RegisterSpec for APB2OTP_BLK3_W9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2otp_blk3_w9::R`](R) reader structure"]
impl crate::Readable for APB2OTP_BLK3_W9_SPEC {}
#[doc = "`reset()` method sets APB2OTP_BLK3_W9 to value 0"]
impl crate::Resettable for APB2OTP_BLK3_W9_SPEC {}
