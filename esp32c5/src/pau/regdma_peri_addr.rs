#[doc = "Register `REGDMA_PERI_ADDR` reader"]
pub type R = crate::R<REGDMA_PERI_ADDR_SPEC>;
#[doc = "Field `PERI_ADDR` reader - peri addr reg"]
pub type PERI_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - peri addr reg"]
    #[inline(always)]
    pub fn peri_addr(&self) -> PERI_ADDR_R {
        PERI_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGDMA_PERI_ADDR")
            .field("peri_addr", &self.peri_addr())
            .finish()
    }
}
#[doc = "Backup addr\n\nYou can [`read`](crate::Reg::read) this register and get [`regdma_peri_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGDMA_PERI_ADDR_SPEC;
impl crate::RegisterSpec for REGDMA_PERI_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regdma_peri_addr::R`](R) reader structure"]
impl crate::Readable for REGDMA_PERI_ADDR_SPEC {}
#[doc = "`reset()` method sets REGDMA_PERI_ADDR to value 0"]
impl crate::Resettable for REGDMA_PERI_ADDR_SPEC {}
