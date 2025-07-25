#[doc = "Register `BUFFER_%s` writer"]
pub type W = crate::W<BUFFER__SPEC>;
#[doc = "Field `BUFFER` writer - Data buffers for encryption."]
pub type BUFFER_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BUFFER__SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - Data buffers for encryption."]
    #[inline(always)]
    pub fn buffer(&mut self) -> BUFFER_W<BUFFER__SPEC> {
        BUFFER_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buffer_::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUFFER__SPEC;
impl crate::RegisterSpec for BUFFER__SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`buffer_::W`](W) writer structure"]
impl crate::Writable for BUFFER__SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUFFER_%s to value 0"]
impl crate::Resettable for BUFFER__SPEC {}
