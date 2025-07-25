#[doc = "Register `WDTCONFIG4` reader"]
pub type R = crate::R<WDTCONFIG4_SPEC>;
#[doc = "Register `WDTCONFIG4` writer"]
pub type W = crate::W<WDTCONFIG4_SPEC>;
#[doc = "Field `WDT_STG2_HOLD` reader - Configures the stage 2 timeout value. Valid only when write protection is disabled. \\\\ Measurement unit: mwdt_clk \\\\"]
pub type WDT_STG2_HOLD_R = crate::FieldReader<u32>;
#[doc = "Field `WDT_STG2_HOLD` writer - Configures the stage 2 timeout value. Valid only when write protection is disabled. \\\\ Measurement unit: mwdt_clk \\\\"]
pub type WDT_STG2_HOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Configures the stage 2 timeout value. Valid only when write protection is disabled. \\\\ Measurement unit: mwdt_clk \\\\"]
    #[inline(always)]
    pub fn wdt_stg2_hold(&self) -> WDT_STG2_HOLD_R {
        WDT_STG2_HOLD_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDTCONFIG4")
            .field("wdt_stg2_hold", &self.wdt_stg2_hold())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Configures the stage 2 timeout value. Valid only when write protection is disabled. \\\\ Measurement unit: mwdt_clk \\\\"]
    #[inline(always)]
    pub fn wdt_stg2_hold(&mut self) -> WDT_STG2_HOLD_W<WDTCONFIG4_SPEC> {
        WDT_STG2_HOLD_W::new(self, 0)
    }
}
#[doc = "Watchdog timer stage 2 timeout value\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtconfig4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtconfig4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDTCONFIG4_SPEC;
impl crate::RegisterSpec for WDTCONFIG4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdtconfig4::R`](R) reader structure"]
impl crate::Readable for WDTCONFIG4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdtconfig4::W`](W) writer structure"]
impl crate::Writable for WDTCONFIG4_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDTCONFIG4 to value 0x000f_ffff"]
impl crate::Resettable for WDTCONFIG4_SPEC {
    const RESET_VALUE: u32 = 0x000f_ffff;
}
