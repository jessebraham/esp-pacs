#[doc = "Register `PARL_CLK_TX_CONF` reader"]
pub type R = crate::R<PARL_CLK_TX_CONF_SPEC>;
#[doc = "Register `PARL_CLK_TX_CONF` writer"]
pub type W = crate::W<PARL_CLK_TX_CONF_SPEC>;
#[doc = "Field `PARL_CLK_TX_DIV_NUM` reader - The integral part of the frequency divider factor of the parl tx clock."]
pub type PARL_CLK_TX_DIV_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `PARL_CLK_TX_DIV_NUM` writer - The integral part of the frequency divider factor of the parl tx clock."]
pub type PARL_CLK_TX_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PARL_CLK_TX_SEL` reader - Configures the clock source of Paraller IO RX\\\\ 0 (default): XTAL_CLK\\\\ 1: RC_FAST_CLK\\\\ 2: PLL_F240M_CLK\\\\ 3: Use the clock from chip pin\\\\"]
pub type PARL_CLK_TX_SEL_R = crate::FieldReader;
#[doc = "Field `PARL_CLK_TX_SEL` writer - Configures the clock source of Paraller IO RX\\\\ 0 (default): XTAL_CLK\\\\ 1: RC_FAST_CLK\\\\ 2: PLL_F240M_CLK\\\\ 3: Use the clock from chip pin\\\\"]
pub type PARL_CLK_TX_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PARL_CLK_TX_EN` reader - Set 1 to enable parl tx clock"]
pub type PARL_CLK_TX_EN_R = crate::BitReader;
#[doc = "Field `PARL_CLK_TX_EN` writer - Set 1 to enable parl tx clock"]
pub type PARL_CLK_TX_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARL_TX_RST_EN` reader - Set 0 to reset parl tx module"]
pub type PARL_TX_RST_EN_R = crate::BitReader;
#[doc = "Field `PARL_TX_RST_EN` writer - Set 0 to reset parl tx module"]
pub type PARL_TX_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - The integral part of the frequency divider factor of the parl tx clock."]
    #[inline(always)]
    pub fn parl_clk_tx_div_num(&self) -> PARL_CLK_TX_DIV_NUM_R {
        PARL_CLK_TX_DIV_NUM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - Configures the clock source of Paraller IO RX\\\\ 0 (default): XTAL_CLK\\\\ 1: RC_FAST_CLK\\\\ 2: PLL_F240M_CLK\\\\ 3: Use the clock from chip pin\\\\"]
    #[inline(always)]
    pub fn parl_clk_tx_sel(&self) -> PARL_CLK_TX_SEL_R {
        PARL_CLK_TX_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Set 1 to enable parl tx clock"]
    #[inline(always)]
    pub fn parl_clk_tx_en(&self) -> PARL_CLK_TX_EN_R {
        PARL_CLK_TX_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Set 0 to reset parl tx module"]
    #[inline(always)]
    pub fn parl_tx_rst_en(&self) -> PARL_TX_RST_EN_R {
        PARL_TX_RST_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PARL_CLK_TX_CONF")
            .field("parl_clk_tx_div_num", &self.parl_clk_tx_div_num())
            .field("parl_clk_tx_sel", &self.parl_clk_tx_sel())
            .field("parl_clk_tx_en", &self.parl_clk_tx_en())
            .field("parl_tx_rst_en", &self.parl_tx_rst_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - The integral part of the frequency divider factor of the parl tx clock."]
    #[inline(always)]
    pub fn parl_clk_tx_div_num(&mut self) -> PARL_CLK_TX_DIV_NUM_W<PARL_CLK_TX_CONF_SPEC> {
        PARL_CLK_TX_DIV_NUM_W::new(self, 0)
    }
    #[doc = "Bits 16:17 - Configures the clock source of Paraller IO RX\\\\ 0 (default): XTAL_CLK\\\\ 1: RC_FAST_CLK\\\\ 2: PLL_F240M_CLK\\\\ 3: Use the clock from chip pin\\\\"]
    #[inline(always)]
    pub fn parl_clk_tx_sel(&mut self) -> PARL_CLK_TX_SEL_W<PARL_CLK_TX_CONF_SPEC> {
        PARL_CLK_TX_SEL_W::new(self, 16)
    }
    #[doc = "Bit 18 - Set 1 to enable parl tx clock"]
    #[inline(always)]
    pub fn parl_clk_tx_en(&mut self) -> PARL_CLK_TX_EN_W<PARL_CLK_TX_CONF_SPEC> {
        PARL_CLK_TX_EN_W::new(self, 18)
    }
    #[doc = "Bit 19 - Set 0 to reset parl tx module"]
    #[inline(always)]
    pub fn parl_tx_rst_en(&mut self) -> PARL_TX_RST_EN_W<PARL_CLK_TX_CONF_SPEC> {
        PARL_TX_RST_EN_W::new(self, 19)
    }
}
#[doc = "PARL_CLK_TX configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`parl_clk_tx_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`parl_clk_tx_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PARL_CLK_TX_CONF_SPEC;
impl crate::RegisterSpec for PARL_CLK_TX_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`parl_clk_tx_conf::R`](R) reader structure"]
impl crate::Readable for PARL_CLK_TX_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`parl_clk_tx_conf::W`](W) writer structure"]
impl crate::Writable for PARL_CLK_TX_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PARL_CLK_TX_CONF to value 0"]
impl crate::Resettable for PARL_CLK_TX_CONF_SPEC {}
