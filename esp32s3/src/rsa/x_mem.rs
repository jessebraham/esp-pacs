#[doc = "Register `X_MEM[%s]` writer"]
pub type W = crate::W<X_MEM_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<X_MEM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Memory X\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`x_mem::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct X_MEM_SPEC;
impl crate::RegisterSpec for X_MEM_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`x_mem::W`](W) writer structure"]
impl crate::Writable for X_MEM_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets X_MEM[%s] to value 0"]
impl crate::Resettable for X_MEM_SPEC {}
