#[doc = "Register `BLK3_RDATA7` reader"]
pub type R = crate::R<BLK3_RDATA7_SPEC>;
#[doc = "Field `RD_BLK3_RESERVED_7` reader - "]
pub type RD_BLK3_RESERVED_7_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rd_blk3_reserved_7(&self) -> RD_BLK3_RESERVED_7_R {
        RD_BLK3_RESERVED_7_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK3_RDATA7")
            .field("rd_blk3_reserved_7", &self.rd_blk3_reserved_7())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`blk3_rdata7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK3_RDATA7_SPEC;
impl crate::RegisterSpec for BLK3_RDATA7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk3_rdata7::R`](R) reader structure"]
impl crate::Readable for BLK3_RDATA7_SPEC {}
#[doc = "`reset()` method sets BLK3_RDATA7 to value 0"]
impl crate::Resettable for BLK3_RDATA7_SPEC {}
