#[doc = "Register `MODEXP_START` writer"]
pub type W = crate::W<MODEXP_START_SPEC>;
#[doc = "Field `MODEXP_START` writer - Set this bit to 1 to start the modular exponentiation."]
pub type MODEXP_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MODEXP_START_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to 1 to start the modular exponentiation."]
    #[inline(always)]
    pub fn modexp_start(&mut self) -> MODEXP_START_W<MODEXP_START_SPEC> {
        MODEXP_START_W::new(self, 0)
    }
}
#[doc = "Modular exponentiation trigger register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modexp_start::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODEXP_START_SPEC;
impl crate::RegisterSpec for MODEXP_START_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`modexp_start::W`](W) writer structure"]
impl crate::Writable for MODEXP_START_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODEXP_START to value 0"]
impl crate::Resettable for MODEXP_START_SPEC {}
