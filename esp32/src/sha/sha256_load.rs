#[doc = "Register `SHA256_LOAD` writer"]
pub type W = crate::W<SHA256_LOAD_SPEC>;
#[doc = "Field `SHA256_LOAD` writer - Write 1 to finish the SHA-256 operation to calculate the final message hash."]
pub type SHA256_LOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SHA256_LOAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to finish the SHA-256 operation to calculate the final message hash."]
    #[inline(always)]
    pub fn sha256_load(&mut self) -> SHA256_LOAD_W<SHA256_LOAD_SPEC> {
        SHA256_LOAD_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha256_load::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHA256_LOAD_SPEC;
impl crate::RegisterSpec for SHA256_LOAD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sha256_load::W`](W) writer structure"]
impl crate::Writable for SHA256_LOAD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHA256_LOAD to value 0"]
impl crate::Resettable for SHA256_LOAD_SPEC {}
