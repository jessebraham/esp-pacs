#[doc = "Register `TIMERGROUP1_WDT_CLK_CONF` reader"]
pub type R = crate::R<TIMERGROUP1_WDT_CLK_CONF_SPEC>;
#[doc = "Register `TIMERGROUP1_WDT_CLK_CONF` writer"]
pub type W = crate::W<TIMERGROUP1_WDT_CLK_CONF_SPEC>;
#[doc = "Field `TG1_WDT_CLK_SEL` reader - Configures the clock source of WDT in Timer Group 1.\\\\ 0 (default): XTAL_CLK\\\\ 1: RC_FAST_CLK\\\\ 2: PLL_F80M_CLK\\\\"]
pub type TG1_WDT_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `TG1_WDT_CLK_SEL` writer - Configures the clock source of WDT in Timer Group 1.\\\\ 0 (default): XTAL_CLK\\\\ 1: RC_FAST_CLK\\\\ 2: PLL_F80M_CLK\\\\"]
pub type TG1_WDT_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TG1_WDT_CLK_EN` reader - Set 1 to enable timer_group0 wdt clock"]
pub type TG1_WDT_CLK_EN_R = crate::BitReader;
#[doc = "Field `TG1_WDT_CLK_EN` writer - Set 1 to enable timer_group0 wdt clock"]
pub type TG1_WDT_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 20:21 - Configures the clock source of WDT in Timer Group 1.\\\\ 0 (default): XTAL_CLK\\\\ 1: RC_FAST_CLK\\\\ 2: PLL_F80M_CLK\\\\"]
    #[inline(always)]
    pub fn tg1_wdt_clk_sel(&self) -> TG1_WDT_CLK_SEL_R {
        TG1_WDT_CLK_SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Set 1 to enable timer_group0 wdt clock"]
    #[inline(always)]
    pub fn tg1_wdt_clk_en(&self) -> TG1_WDT_CLK_EN_R {
        TG1_WDT_CLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMERGROUP1_WDT_CLK_CONF")
            .field("tg1_wdt_clk_sel", &self.tg1_wdt_clk_sel())
            .field("tg1_wdt_clk_en", &self.tg1_wdt_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 20:21 - Configures the clock source of WDT in Timer Group 1.\\\\ 0 (default): XTAL_CLK\\\\ 1: RC_FAST_CLK\\\\ 2: PLL_F80M_CLK\\\\"]
    #[inline(always)]
    pub fn tg1_wdt_clk_sel(&mut self) -> TG1_WDT_CLK_SEL_W<TIMERGROUP1_WDT_CLK_CONF_SPEC> {
        TG1_WDT_CLK_SEL_W::new(self, 20)
    }
    #[doc = "Bit 22 - Set 1 to enable timer_group0 wdt clock"]
    #[inline(always)]
    pub fn tg1_wdt_clk_en(&mut self) -> TG1_WDT_CLK_EN_W<TIMERGROUP1_WDT_CLK_CONF_SPEC> {
        TG1_WDT_CLK_EN_W::new(self, 22)
    }
}
#[doc = "TIMERGROUP1_WDT_CLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`timergroup1_wdt_clk_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timergroup1_wdt_clk_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMERGROUP1_WDT_CLK_CONF_SPEC;
impl crate::RegisterSpec for TIMERGROUP1_WDT_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timergroup1_wdt_clk_conf::R`](R) reader structure"]
impl crate::Readable for TIMERGROUP1_WDT_CLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timergroup1_wdt_clk_conf::W`](W) writer structure"]
impl crate::Writable for TIMERGROUP1_WDT_CLK_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMERGROUP1_WDT_CLK_CONF to value 0x0040_0000"]
impl crate::Resettable for TIMERGROUP1_WDT_CLK_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0040_0000;
}
