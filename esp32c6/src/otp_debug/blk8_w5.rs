#[doc = "Register `BLK8_W5` reader"]
pub struct R(crate::R<BLK8_W5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK8_W5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK8_W5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK8_W5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BLOCK8_W5` reader - Otp block8 word5 data."]
pub type BLOCK8_W5_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block8 word5 data."]
    #[inline(always)]
    pub fn block8_w5(&self) -> BLOCK8_W5_R {
        BLOCK8_W5_R::new(self.bits)
    }
}
#[doc = "Otp debuger block8 data register5.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk8_w5](index.html) module"]
pub struct BLK8_W5_SPEC;
impl crate::RegisterSpec for BLK8_W5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk8_w5::R](R) reader structure"]
impl crate::Readable for BLK8_W5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK8_W5 to value 0"]
impl crate::Resettable for BLK8_W5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}