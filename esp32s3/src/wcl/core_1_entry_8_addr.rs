#[doc = "Register `Core_1_ENTRY_8_ADDR` reader"]
pub type R = crate::R<CORE_1_ENTRY_8_ADDR_SPEC>;
#[doc = "Register `Core_1_ENTRY_8_ADDR` writer"]
pub type W = crate::W<CORE_1_ENTRY_8_ADDR_SPEC>;
#[doc = "Field `CORE_1_ENTRY_8_ADDR` reader - Core_1 Entry 8 address from WORLD1 to WORLD0"]
pub type CORE_1_ENTRY_8_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `CORE_1_ENTRY_8_ADDR` writer - Core_1 Entry 8 address from WORLD1 to WORLD0"]
pub type CORE_1_ENTRY_8_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Core_1 Entry 8 address from WORLD1 to WORLD0"]
    #[inline(always)]
    pub fn core_1_entry_8_addr(&self) -> CORE_1_ENTRY_8_ADDR_R {
        CORE_1_ENTRY_8_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Core_1_ENTRY_8_ADDR")
            .field("core_1_entry_8_addr", &self.core_1_entry_8_addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Core_1 Entry 8 address from WORLD1 to WORLD0"]
    #[inline(always)]
    pub fn core_1_entry_8_addr(&mut self) -> CORE_1_ENTRY_8_ADDR_W<CORE_1_ENTRY_8_ADDR_SPEC> {
        CORE_1_ENTRY_8_ADDR_W::new(self, 0)
    }
}
#[doc = "Core_1 Entry 8 address configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_entry_8_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_1_entry_8_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_ENTRY_8_ADDR_SPEC;
impl crate::RegisterSpec for CORE_1_ENTRY_8_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_entry_8_addr::R`](R) reader structure"]
impl crate::Readable for CORE_1_ENTRY_8_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_1_entry_8_addr::W`](W) writer structure"]
impl crate::Writable for CORE_1_ENTRY_8_ADDR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Core_1_ENTRY_8_ADDR to value 0"]
impl crate::Resettable for CORE_1_ENTRY_8_ADDR_SPEC {}
