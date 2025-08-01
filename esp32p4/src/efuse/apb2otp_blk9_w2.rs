#[doc = "Register `APB2OTP_BLK9_W2` reader"]
pub type R = crate::R<APB2OTP_BLK9_W2_SPEC>;
#[doc = "Field `APB2OTP_BLOCK9_W2` reader - Otp block9 word2 data."]
pub type APB2OTP_BLOCK9_W2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block9 word2 data."]
    #[inline(always)]
    pub fn apb2otp_block9_w2(&self) -> APB2OTP_BLOCK9_W2_R {
        APB2OTP_BLOCK9_W2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2OTP_BLK9_W2")
            .field("apb2otp_block9_w2", &self.apb2otp_block9_w2())
            .finish()
    }
}
#[doc = "eFuse apb2otp block9 data register2.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk9_w2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2OTP_BLK9_W2_SPEC;
impl crate::RegisterSpec for APB2OTP_BLK9_W2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2otp_blk9_w2::R`](R) reader structure"]
impl crate::Readable for APB2OTP_BLK9_W2_SPEC {}
#[doc = "`reset()` method sets APB2OTP_BLK9_W2 to value 0"]
impl crate::Resettable for APB2OTP_BLK9_W2_SPEC {}
