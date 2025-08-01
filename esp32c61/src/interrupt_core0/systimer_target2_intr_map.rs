#[doc = "Register `SYSTIMER_TARGET2_INTR_MAP` reader"]
pub type R = crate::R<SYSTIMER_TARGET2_INTR_MAP_SPEC>;
#[doc = "Register `SYSTIMER_TARGET2_INTR_MAP` writer"]
pub type W = crate::W<SYSTIMER_TARGET2_INTR_MAP_SPEC>;
#[doc = "Field `SYSTIMER_TARGET2_INTR_MAP` reader - Configures the interrupt source into one CPU interrupt."]
pub type SYSTIMER_TARGET2_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `SYSTIMER_TARGET2_INTR_MAP` writer - Configures the interrupt source into one CPU interrupt."]
pub type SYSTIMER_TARGET2_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SYSTIMER_TARGET2_INTR_PASS_IN_SEC` reader - Configures the PASS_IN_SEC flag of the interrupt source."]
pub type SYSTIMER_TARGET2_INTR_PASS_IN_SEC_R = crate::BitReader;
#[doc = "Field `SYSTIMER_TARGET2_INTR_PASS_IN_SEC` writer - Configures the PASS_IN_SEC flag of the interrupt source."]
pub type SYSTIMER_TARGET2_INTR_PASS_IN_SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Configures the interrupt source into one CPU interrupt."]
    #[inline(always)]
    pub fn systimer_target2_intr_map(&self) -> SYSTIMER_TARGET2_INTR_MAP_R {
        SYSTIMER_TARGET2_INTR_MAP_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - Configures the PASS_IN_SEC flag of the interrupt source."]
    #[inline(always)]
    pub fn systimer_target2_intr_pass_in_sec(&self) -> SYSTIMER_TARGET2_INTR_PASS_IN_SEC_R {
        SYSTIMER_TARGET2_INTR_PASS_IN_SEC_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSTIMER_TARGET2_INTR_MAP")
            .field(
                "systimer_target2_intr_map",
                &self.systimer_target2_intr_map(),
            )
            .field(
                "systimer_target2_intr_pass_in_sec",
                &self.systimer_target2_intr_pass_in_sec(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Configures the interrupt source into one CPU interrupt."]
    #[inline(always)]
    pub fn systimer_target2_intr_map(
        &mut self,
    ) -> SYSTIMER_TARGET2_INTR_MAP_W<SYSTIMER_TARGET2_INTR_MAP_SPEC> {
        SYSTIMER_TARGET2_INTR_MAP_W::new(self, 0)
    }
    #[doc = "Bit 8 - Configures the PASS_IN_SEC flag of the interrupt source."]
    #[inline(always)]
    pub fn systimer_target2_intr_pass_in_sec(
        &mut self,
    ) -> SYSTIMER_TARGET2_INTR_PASS_IN_SEC_W<SYSTIMER_TARGET2_INTR_MAP_SPEC> {
        SYSTIMER_TARGET2_INTR_PASS_IN_SEC_W::new(self, 8)
    }
}
#[doc = "SYSTIMER_TARGET2_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`systimer_target2_intr_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systimer_target2_intr_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSTIMER_TARGET2_INTR_MAP_SPEC;
impl crate::RegisterSpec for SYSTIMER_TARGET2_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`systimer_target2_intr_map::R`](R) reader structure"]
impl crate::Readable for SYSTIMER_TARGET2_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`systimer_target2_intr_map::W`](W) writer structure"]
impl crate::Writable for SYSTIMER_TARGET2_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSTIMER_TARGET2_INTR_MAP to value 0"]
impl crate::Resettable for SYSTIMER_TARGET2_INTR_MAP_SPEC {}
