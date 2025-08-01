#[doc = "Register `L2_IBUS0_ACS_MISS_CNT` reader"]
pub type R = crate::R<L2_IBUS0_ACS_MISS_CNT_SPEC>;
#[doc = "Field `L2_IBUS0_MISS_CNT` reader - The register records the number of missing when L1-ICache0 accesses L2-Cache due to bus0 accessing L1-ICache0."]
pub type L2_IBUS0_MISS_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of missing when L1-ICache0 accesses L2-Cache due to bus0 accessing L1-ICache0."]
    #[inline(always)]
    pub fn l2_ibus0_miss_cnt(&self) -> L2_IBUS0_MISS_CNT_R {
        L2_IBUS0_MISS_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_IBUS0_ACS_MISS_CNT")
            .field("l2_ibus0_miss_cnt", &self.l2_ibus0_miss_cnt())
            .finish()
    }
}
#[doc = "L2-Cache bus0 Miss-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_ibus0_acs_miss_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_IBUS0_ACS_MISS_CNT_SPEC;
impl crate::RegisterSpec for L2_IBUS0_ACS_MISS_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_ibus0_acs_miss_cnt::R`](R) reader structure"]
impl crate::Readable for L2_IBUS0_ACS_MISS_CNT_SPEC {}
#[doc = "`reset()` method sets L2_IBUS0_ACS_MISS_CNT to value 0"]
impl crate::Resettable for L2_IBUS0_ACS_MISS_CNT_SPEC {}
