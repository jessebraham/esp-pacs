#[doc = "Register `SET_START_MODMULT` writer"]
pub type W = crate::W<SET_START_MODMULT_SPEC>;
#[doc = "Field `SET_START_MODMULT` writer - Configure whether or not to start the modular multiplication. 0: No effect 1: Start"]
pub type SET_START_MODMULT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SET_START_MODMULT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Configure whether or not to start the modular multiplication. 0: No effect 1: Start"]
    #[inline(always)]
    pub fn set_start_modmult(&mut self) -> SET_START_MODMULT_W<SET_START_MODMULT_SPEC> {
        SET_START_MODMULT_W::new(self, 0)
    }
}
#[doc = "Starts modular multiplication\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_start_modmult::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SET_START_MODMULT_SPEC;
impl crate::RegisterSpec for SET_START_MODMULT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`set_start_modmult::W`](W) writer structure"]
impl crate::Writable for SET_START_MODMULT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SET_START_MODMULT to value 0"]
impl crate::Resettable for SET_START_MODMULT_SPEC {}
