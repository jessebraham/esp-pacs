#[doc = "Register `PMUP_BITMAP_HIGH2` reader"]
pub type R = crate::R<PMUP_BITMAP_HIGH2_SPEC>;
#[doc = "Register `PMUP_BITMAP_HIGH2` writer"]
pub type W = crate::W<PMUP_BITMAP_HIGH2_SPEC>;
#[doc = "Field `PUMP_BITMAP_HIGH2` reader - select valid high channel2"]
pub type PUMP_BITMAP_HIGH2_R = crate::FieldReader<u32>;
#[doc = "Field `PUMP_BITMAP_HIGH2` writer - select valid high channel2"]
pub type PUMP_BITMAP_HIGH2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - select valid high channel2"]
    #[inline(always)]
    pub fn pump_bitmap_high2(&self) -> PUMP_BITMAP_HIGH2_R {
        PUMP_BITMAP_HIGH2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMUP_BITMAP_HIGH2")
            .field("pump_bitmap_high2", &self.pump_bitmap_high2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - select valid high channel2"]
    #[inline(always)]
    pub fn pump_bitmap_high2(&mut self) -> PUMP_BITMAP_HIGH2_W<PMUP_BITMAP_HIGH2_SPEC> {
        PUMP_BITMAP_HIGH2_W::new(self, 0)
    }
}
#[doc = "select valid pvt channel\n\nYou can [`read`](crate::Reg::read) this register and get [`pmup_bitmap_high2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmup_bitmap_high2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMUP_BITMAP_HIGH2_SPEC;
impl crate::RegisterSpec for PMUP_BITMAP_HIGH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmup_bitmap_high2::R`](R) reader structure"]
impl crate::Readable for PMUP_BITMAP_HIGH2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmup_bitmap_high2::W`](W) writer structure"]
impl crate::Writable for PMUP_BITMAP_HIGH2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PMUP_BITMAP_HIGH2 to value 0"]
impl crate::Resettable for PMUP_BITMAP_HIGH2_SPEC {}
