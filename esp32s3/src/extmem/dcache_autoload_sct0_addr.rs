#[doc = "Register `DCACHE_AUTOLOAD_SCT0_ADDR` reader"]
pub type R = crate::R<DCACHE_AUTOLOAD_SCT0_ADDR_SPEC>;
#[doc = "Register `DCACHE_AUTOLOAD_SCT0_ADDR` writer"]
pub type W = crate::W<DCACHE_AUTOLOAD_SCT0_ADDR_SPEC>;
#[doc = "Field `DCACHE_AUTOLOAD_SCT0_ADDR` reader - The bits are used to configure the start virtual address of the first section for autoload operation. It should be combined with dcache_autoload_sct0_ena."]
pub type DCACHE_AUTOLOAD_SCT0_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `DCACHE_AUTOLOAD_SCT0_ADDR` writer - The bits are used to configure the start virtual address of the first section for autoload operation. It should be combined with dcache_autoload_sct0_ena."]
pub type DCACHE_AUTOLOAD_SCT0_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The bits are used to configure the start virtual address of the first section for autoload operation. It should be combined with dcache_autoload_sct0_ena."]
    #[inline(always)]
    pub fn dcache_autoload_sct0_addr(&self) -> DCACHE_AUTOLOAD_SCT0_ADDR_R {
        DCACHE_AUTOLOAD_SCT0_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCACHE_AUTOLOAD_SCT0_ADDR")
            .field(
                "dcache_autoload_sct0_addr",
                &self.dcache_autoload_sct0_addr(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - The bits are used to configure the start virtual address of the first section for autoload operation. It should be combined with dcache_autoload_sct0_ena."]
    #[inline(always)]
    pub fn dcache_autoload_sct0_addr(
        &mut self,
    ) -> DCACHE_AUTOLOAD_SCT0_ADDR_W<DCACHE_AUTOLOAD_SCT0_ADDR_SPEC> {
        DCACHE_AUTOLOAD_SCT0_ADDR_W::new(self, 0)
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_autoload_sct0_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_autoload_sct0_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCACHE_AUTOLOAD_SCT0_ADDR_SPEC;
impl crate::RegisterSpec for DCACHE_AUTOLOAD_SCT0_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcache_autoload_sct0_addr::R`](R) reader structure"]
impl crate::Readable for DCACHE_AUTOLOAD_SCT0_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcache_autoload_sct0_addr::W`](W) writer structure"]
impl crate::Writable for DCACHE_AUTOLOAD_SCT0_ADDR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCACHE_AUTOLOAD_SCT0_ADDR to value 0"]
impl crate::Resettable for DCACHE_AUTOLOAD_SCT0_ADDR_SPEC {}
