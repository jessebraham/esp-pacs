#[doc = "Register `WDTCONFIG1` reader"]
pub type R = crate::R<WDTCONFIG1_SPEC>;
#[doc = "Register `WDTCONFIG1` writer"]
pub type W = crate::W<WDTCONFIG1_SPEC>;
#[doc = "Field `WDT_DIVCNT_RST` writer - Configures whether to reset WDT 's clock divider counter. \\\\ 0: No effect \\\\ 1: Reset \\\\"]
pub type WDT_DIVCNT_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_CLK_PRESCALE` reader - Configures MWDT clock prescaler value. Valid only when write protection is disabled. \\\\ MWDT clock period = MWDT's clock source period * TIMG_WDT_CLK_PRESCALE. \\\\"]
pub type WDT_CLK_PRESCALE_R = crate::FieldReader<u16>;
#[doc = "Field `WDT_CLK_PRESCALE` writer - Configures MWDT clock prescaler value. Valid only when write protection is disabled. \\\\ MWDT clock period = MWDT's clock source period * TIMG_WDT_CLK_PRESCALE. \\\\"]
pub type WDT_CLK_PRESCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 16:31 - Configures MWDT clock prescaler value. Valid only when write protection is disabled. \\\\ MWDT clock period = MWDT's clock source period * TIMG_WDT_CLK_PRESCALE. \\\\"]
    #[inline(always)]
    pub fn wdt_clk_prescale(&self) -> WDT_CLK_PRESCALE_R {
        WDT_CLK_PRESCALE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDTCONFIG1")
            .field("wdt_clk_prescale", &self.wdt_clk_prescale())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether to reset WDT 's clock divider counter. \\\\ 0: No effect \\\\ 1: Reset \\\\"]
    #[inline(always)]
    pub fn wdt_divcnt_rst(&mut self) -> WDT_DIVCNT_RST_W<WDTCONFIG1_SPEC> {
        WDT_DIVCNT_RST_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Configures MWDT clock prescaler value. Valid only when write protection is disabled. \\\\ MWDT clock period = MWDT's clock source period * TIMG_WDT_CLK_PRESCALE. \\\\"]
    #[inline(always)]
    pub fn wdt_clk_prescale(&mut self) -> WDT_CLK_PRESCALE_W<WDTCONFIG1_SPEC> {
        WDT_CLK_PRESCALE_W::new(self, 16)
    }
}
#[doc = "Watchdog timer prescaler register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtconfig1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtconfig1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDTCONFIG1_SPEC;
impl crate::RegisterSpec for WDTCONFIG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdtconfig1::R`](R) reader structure"]
impl crate::Readable for WDTCONFIG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdtconfig1::W`](W) writer structure"]
impl crate::Writable for WDTCONFIG1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDTCONFIG1 to value 0x0001_0000"]
impl crate::Resettable for WDTCONFIG1_SPEC {
    const RESET_VALUE: u32 = 0x0001_0000;
}
