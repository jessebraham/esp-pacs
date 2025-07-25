#[doc = "Register `M0_STATUS_CLR` writer"]
pub type W = crate::W<M0_STATUS_CLR_SPEC>;
#[doc = "Field `M0_REGION_STATUS_CLR` writer - Clear exception status"]
pub type M0_REGION_STATUS_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<M0_STATUS_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Clear exception status"]
    #[inline(always)]
    pub fn m0_region_status_clr(&mut self) -> M0_REGION_STATUS_CLR_W<M0_STATUS_CLR_SPEC> {
        M0_REGION_STATUS_CLR_W::new(self, 0)
    }
}
#[doc = "M0 status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m0_status_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M0_STATUS_CLR_SPEC;
impl crate::RegisterSpec for M0_STATUS_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`m0_status_clr::W`](W) writer structure"]
impl crate::Writable for M0_STATUS_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets M0_STATUS_CLR to value 0"]
impl crate::Resettable for M0_STATUS_CLR_SPEC {}
