#[doc = "Register `WDTCONFIG2` reader"]
pub type R = crate::R<WDTCONFIG2_SPEC>;
#[doc = "Register `WDTCONFIG2` writer"]
pub type W = crate::W<WDTCONFIG2_SPEC>;
#[doc = "Field `WDT_STG1_HOLD` reader - Configures the hold time of RTC watchdog at level 2."]
pub type WDT_STG1_HOLD_R = crate::FieldReader<u32>;
#[doc = "Field `WDT_STG1_HOLD` writer - Configures the hold time of RTC watchdog at level 2."]
pub type WDT_STG1_HOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Configures the hold time of RTC watchdog at level 2."]
    #[inline(always)]
    pub fn wdt_stg1_hold(&self) -> WDT_STG1_HOLD_R {
        WDT_STG1_HOLD_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDTCONFIG2")
            .field("wdt_stg1_hold", &self.wdt_stg1_hold())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Configures the hold time of RTC watchdog at level 2."]
    #[inline(always)]
    pub fn wdt_stg1_hold(&mut self) -> WDT_STG1_HOLD_W<WDTCONFIG2_SPEC> {
        WDT_STG1_HOLD_W::new(self, 0)
    }
}
#[doc = "Configures the hold time of RTC watchdog at level 2\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtconfig2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtconfig2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDTCONFIG2_SPEC;
impl crate::RegisterSpec for WDTCONFIG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdtconfig2::R`](R) reader structure"]
impl crate::Readable for WDTCONFIG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdtconfig2::W`](W) writer structure"]
impl crate::Writable for WDTCONFIG2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDTCONFIG2 to value 0x0001_3880"]
impl crate::Resettable for WDTCONFIG2_SPEC {
    const RESET_VALUE: u32 = 0x0001_3880;
}
