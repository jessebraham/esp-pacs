#[doc = "Register `CONTINUE` writer"]
pub type W = crate::W<CONTINUE_SPEC>;
#[doc = "Field `CONTINUE` writer - reserved."]
pub type CONTINUE_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONTINUE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 1:31 - reserved."]
    #[inline(always)]
    pub fn continue_(&mut self) -> CONTINUE_W<CONTINUE_SPEC> {
        CONTINUE_W::new(self, 1)
    }
}
#[doc = "Typical SHA configuration register 1.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`continue_::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONTINUE_SPEC;
impl crate::RegisterSpec for CONTINUE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`continue_::W`](W) writer structure"]
impl crate::Writable for CONTINUE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONTINUE to value 0"]
impl crate::Resettable for CONTINUE_SPEC {}
