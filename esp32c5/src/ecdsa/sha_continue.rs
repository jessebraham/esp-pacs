#[doc = "Register `SHA_CONTINUE` writer"]
pub type W = crate::W<SHA_CONTINUE_SPEC>;
#[doc = "Field `SHA_CONTINUE` writer - Write 1 to start the latter caculation of SHA Calculator in ECDSA Accelerator. This bit will be self-cleared after configuration."]
pub type SHA_CONTINUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SHA_CONTINUE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to start the latter caculation of SHA Calculator in ECDSA Accelerator. This bit will be self-cleared after configuration."]
    #[inline(always)]
    pub fn sha_continue(&mut self) -> SHA_CONTINUE_W<SHA_CONTINUE_SPEC> {
        SHA_CONTINUE_W::new(self, 0)
    }
}
#[doc = "ECDSA control SHA register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha_continue::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHA_CONTINUE_SPEC;
impl crate::RegisterSpec for SHA_CONTINUE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sha_continue::W`](W) writer structure"]
impl crate::Writable for SHA_CONTINUE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHA_CONTINUE to value 0"]
impl crate::Resettable for SHA_CONTINUE_SPEC {}
