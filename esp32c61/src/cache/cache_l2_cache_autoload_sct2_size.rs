#[doc = "Register `CACHE_L2_CACHE_AUTOLOAD_SCT2_SIZE` reader"]
pub type R = crate::R<CACHE_L2_CACHE_AUTOLOAD_SCT2_SIZE_SPEC>;
#[doc = "Field `CACHE_L2_CACHE_AUTOLOAD_SCT2_SIZE` reader - Those bits are used to configure the size of the third section for autoload operation on L2-Cache. Note that it should be used together with L2_CACHE_AUTOLOAD_SCT2_ADDR and L2_CACHE_AUTOLOAD_SCT2_ENA."]
pub type CACHE_L2_CACHE_AUTOLOAD_SCT2_SIZE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:27 - Those bits are used to configure the size of the third section for autoload operation on L2-Cache. Note that it should be used together with L2_CACHE_AUTOLOAD_SCT2_ADDR and L2_CACHE_AUTOLOAD_SCT2_ENA."]
    #[inline(always)]
    pub fn cache_l2_cache_autoload_sct2_size(&self) -> CACHE_L2_CACHE_AUTOLOAD_SCT2_SIZE_R {
        CACHE_L2_CACHE_AUTOLOAD_SCT2_SIZE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_L2_CACHE_AUTOLOAD_SCT2_SIZE")
            .field(
                "cache_l2_cache_autoload_sct2_size",
                &self.cache_l2_cache_autoload_sct2_size(),
            )
            .finish()
    }
}
#[doc = "L2 Cache autoload section 2 size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_autoload_sct2_size::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_L2_CACHE_AUTOLOAD_SCT2_SIZE_SPEC;
impl crate::RegisterSpec for CACHE_L2_CACHE_AUTOLOAD_SCT2_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_l2_cache_autoload_sct2_size::R`](R) reader structure"]
impl crate::Readable for CACHE_L2_CACHE_AUTOLOAD_SCT2_SIZE_SPEC {}
#[doc = "`reset()` method sets CACHE_L2_CACHE_AUTOLOAD_SCT2_SIZE to value 0"]
impl crate::Resettable for CACHE_L2_CACHE_AUTOLOAD_SCT2_SIZE_SPEC {}
