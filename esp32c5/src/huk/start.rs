#[doc = "Register `START` writer"]
pub type W = crate::W<START_SPEC>;
#[doc = "Field `START` writer - Write 1 to continue HUK Generator operation at LOAD/GAIN state."]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONTINUE` writer - Write 1 to start HUK Generator at IDLE state."]
pub type CONTINUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<START_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to continue HUK Generator operation at LOAD/GAIN state."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<START_SPEC> {
        START_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to start HUK Generator at IDLE state."]
    #[inline(always)]
    pub fn continue_(&mut self) -> CONTINUE_W<START_SPEC> {
        CONTINUE_W::new(self, 1)
    }
}
#[doc = "HUK Generator control register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`start::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct START_SPEC;
impl crate::RegisterSpec for START_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`start::W`](W) writer structure"]
impl crate::Writable for START_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets START to value 0"]
impl crate::Resettable for START_SPEC {}
