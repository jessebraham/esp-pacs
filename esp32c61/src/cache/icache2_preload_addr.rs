#[doc = "Register `ICACHE2_PRELOAD_ADDR` reader"]
pub type R = crate::R<ICACHE2_PRELOAD_ADDR_SPEC>;
#[doc = "Field `ICACHE2_PRELOAD_ADDR` reader - Those bits are used to configure the start address of preload on L1-ICache2, which should be used together with L1_ICACHE2_PRELOAD_SIZE_REG"]
pub type ICACHE2_PRELOAD_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Those bits are used to configure the start address of preload on L1-ICache2, which should be used together with L1_ICACHE2_PRELOAD_SIZE_REG"]
    #[inline(always)]
    pub fn icache2_preload_addr(&self) -> ICACHE2_PRELOAD_ADDR_R {
        ICACHE2_PRELOAD_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACHE2_PRELOAD_ADDR")
            .field("icache2_preload_addr", &self.icache2_preload_addr())
            .finish()
    }
}
#[doc = "L1 instruction Cache 2 preload address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache2_preload_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICACHE2_PRELOAD_ADDR_SPEC;
impl crate::RegisterSpec for ICACHE2_PRELOAD_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icache2_preload_addr::R`](R) reader structure"]
impl crate::Readable for ICACHE2_PRELOAD_ADDR_SPEC {}
#[doc = "`reset()` method sets ICACHE2_PRELOAD_ADDR to value 0"]
impl crate::Resettable for ICACHE2_PRELOAD_ADDR_SPEC {}
