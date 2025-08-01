#[doc = "Register `IBUS3_ACS_MISS_CNT` reader"]
pub type R = crate::R<IBUS3_ACS_MISS_CNT_SPEC>;
#[doc = "Field `IBUS3_MISS_CNT` reader - The register records the number of missing when bus3 accesses L1-ICache3."]
pub type IBUS3_MISS_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of missing when bus3 accesses L1-ICache3."]
    #[inline(always)]
    pub fn ibus3_miss_cnt(&self) -> IBUS3_MISS_CNT_R {
        IBUS3_MISS_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IBUS3_ACS_MISS_CNT")
            .field("ibus3_miss_cnt", &self.ibus3_miss_cnt())
            .finish()
    }
}
#[doc = "L1-ICache bus3 Miss-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus3_acs_miss_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IBUS3_ACS_MISS_CNT_SPEC;
impl crate::RegisterSpec for IBUS3_ACS_MISS_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ibus3_acs_miss_cnt::R`](R) reader structure"]
impl crate::Readable for IBUS3_ACS_MISS_CNT_SPEC {}
#[doc = "`reset()` method sets IBUS3_ACS_MISS_CNT to value 0"]
impl crate::Resettable for IBUS3_ACS_MISS_CNT_SPEC {}
