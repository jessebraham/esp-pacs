#[doc = "Register `LOAD` writer"]
pub type W = crate::W<LOAD_SPEC>;
#[doc = "Field `T0_LOAD` writer - Write any value to trigger a timer 0 time-base counter reload."]
pub type T0_LOAD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LOAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Write any value to trigger a timer 0 time-base counter reload."]
    #[inline(always)]
    pub fn t0_load(&mut self) -> T0_LOAD_W<LOAD_SPEC> {
        T0_LOAD_W::new(self, 0)
    }
}
#[doc = "Write to reload timer from TIMG_T0LOADLO_REG or TIMG_T0LOADHI_REG\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`load::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOAD_SPEC;
impl crate::RegisterSpec for LOAD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`load::W`](W) writer structure"]
impl crate::Writable for LOAD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LOAD to value 0"]
impl crate::Resettable for LOAD_SPEC {}
