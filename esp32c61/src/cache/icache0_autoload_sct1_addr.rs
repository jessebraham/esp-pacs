#[doc = "Register `ICACHE0_AUTOLOAD_SCT1_ADDR` reader"]
pub type R = crate::R<ICACHE0_AUTOLOAD_SCT1_ADDR_SPEC>;
#[doc = "Field `ICACHE0_AUTOLOAD_SCT1_ADDR` reader - Those bits are used to configure the start address of the second section for autoload operation on L1-ICache0. Note that it should be used together with L1_ICACHE0_AUTOLOAD_SCT1_SIZE and L1_ICACHE_AUTOLOAD_SCT1_ENA."]
pub type ICACHE0_AUTOLOAD_SCT1_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Those bits are used to configure the start address of the second section for autoload operation on L1-ICache0. Note that it should be used together with L1_ICACHE0_AUTOLOAD_SCT1_SIZE and L1_ICACHE_AUTOLOAD_SCT1_ENA."]
    #[inline(always)]
    pub fn icache0_autoload_sct1_addr(&self) -> ICACHE0_AUTOLOAD_SCT1_ADDR_R {
        ICACHE0_AUTOLOAD_SCT1_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACHE0_AUTOLOAD_SCT1_ADDR")
            .field(
                "icache0_autoload_sct1_addr",
                &self.icache0_autoload_sct1_addr(),
            )
            .finish()
    }
}
#[doc = "L1 instruction Cache 0 autoload section 1 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache0_autoload_sct1_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICACHE0_AUTOLOAD_SCT1_ADDR_SPEC;
impl crate::RegisterSpec for ICACHE0_AUTOLOAD_SCT1_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icache0_autoload_sct1_addr::R`](R) reader structure"]
impl crate::Readable for ICACHE0_AUTOLOAD_SCT1_ADDR_SPEC {}
#[doc = "`reset()` method sets ICACHE0_AUTOLOAD_SCT1_ADDR to value 0"]
impl crate::Resettable for ICACHE0_AUTOLOAD_SCT1_ADDR_SPEC {}
