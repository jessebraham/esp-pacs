#[doc = "Register `PERI_CLK_CTRL15` reader"]
pub type R = crate::R<PERI_CLK_CTRL15_SPEC>;
#[doc = "Register `PERI_CLK_CTRL15` writer"]
pub type W = crate::W<PERI_CLK_CTRL15_SPEC>;
#[doc = "Field `I2S1_RX_DIV_X` reader - Reserved"]
pub type I2S1_RX_DIV_X_R = crate::FieldReader<u16>;
#[doc = "Field `I2S1_RX_DIV_X` writer - Reserved"]
pub type I2S1_RX_DIV_X_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `I2S1_RX_DIV_Y` reader - Reserved"]
pub type I2S1_RX_DIV_Y_R = crate::FieldReader<u16>;
#[doc = "Field `I2S1_RX_DIV_Y` writer - Reserved"]
pub type I2S1_RX_DIV_Y_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `I2S1_RX_DIV_Z` reader - Reserved"]
pub type I2S1_RX_DIV_Z_R = crate::FieldReader<u16>;
#[doc = "Field `I2S1_RX_DIV_Z` writer - Reserved"]
pub type I2S1_RX_DIV_Z_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `I2S1_RX_DIV_YN1` reader - Reserved"]
pub type I2S1_RX_DIV_YN1_R = crate::BitReader;
#[doc = "Field `I2S1_RX_DIV_YN1` writer - Reserved"]
pub type I2S1_RX_DIV_YN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S1_TX_CLK_EN` reader - Reserved"]
pub type I2S1_TX_CLK_EN_R = crate::BitReader;
#[doc = "Field `I2S1_TX_CLK_EN` writer - Reserved"]
pub type I2S1_TX_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S1_TX_CLK_SRC_SEL` reader - Reserved"]
pub type I2S1_TX_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `I2S1_TX_CLK_SRC_SEL` writer - Reserved"]
pub type I2S1_TX_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:8 - Reserved"]
    #[inline(always)]
    pub fn i2s1_rx_div_x(&self) -> I2S1_RX_DIV_X_R {
        I2S1_RX_DIV_X_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17 - Reserved"]
    #[inline(always)]
    pub fn i2s1_rx_div_y(&self) -> I2S1_RX_DIV_Y_R {
        I2S1_RX_DIV_Y_R::new(((self.bits >> 9) & 0x01ff) as u16)
    }
    #[doc = "Bits 18:26 - Reserved"]
    #[inline(always)]
    pub fn i2s1_rx_div_z(&self) -> I2S1_RX_DIV_Z_R {
        I2S1_RX_DIV_Z_R::new(((self.bits >> 18) & 0x01ff) as u16)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn i2s1_rx_div_yn1(&self) -> I2S1_RX_DIV_YN1_R {
        I2S1_RX_DIV_YN1_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn i2s1_tx_clk_en(&self) -> I2S1_TX_CLK_EN_R {
        I2S1_TX_CLK_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - Reserved"]
    #[inline(always)]
    pub fn i2s1_tx_clk_src_sel(&self) -> I2S1_TX_CLK_SRC_SEL_R {
        I2S1_TX_CLK_SRC_SEL_R::new(((self.bits >> 29) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_CLK_CTRL15")
            .field("i2s1_rx_div_x", &self.i2s1_rx_div_x())
            .field("i2s1_rx_div_y", &self.i2s1_rx_div_y())
            .field("i2s1_rx_div_z", &self.i2s1_rx_div_z())
            .field("i2s1_rx_div_yn1", &self.i2s1_rx_div_yn1())
            .field("i2s1_tx_clk_en", &self.i2s1_tx_clk_en())
            .field("i2s1_tx_clk_src_sel", &self.i2s1_tx_clk_src_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - Reserved"]
    #[inline(always)]
    pub fn i2s1_rx_div_x(&mut self) -> I2S1_RX_DIV_X_W<PERI_CLK_CTRL15_SPEC> {
        I2S1_RX_DIV_X_W::new(self, 0)
    }
    #[doc = "Bits 9:17 - Reserved"]
    #[inline(always)]
    pub fn i2s1_rx_div_y(&mut self) -> I2S1_RX_DIV_Y_W<PERI_CLK_CTRL15_SPEC> {
        I2S1_RX_DIV_Y_W::new(self, 9)
    }
    #[doc = "Bits 18:26 - Reserved"]
    #[inline(always)]
    pub fn i2s1_rx_div_z(&mut self) -> I2S1_RX_DIV_Z_W<PERI_CLK_CTRL15_SPEC> {
        I2S1_RX_DIV_Z_W::new(self, 18)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn i2s1_rx_div_yn1(&mut self) -> I2S1_RX_DIV_YN1_W<PERI_CLK_CTRL15_SPEC> {
        I2S1_RX_DIV_YN1_W::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn i2s1_tx_clk_en(&mut self) -> I2S1_TX_CLK_EN_W<PERI_CLK_CTRL15_SPEC> {
        I2S1_TX_CLK_EN_W::new(self, 28)
    }
    #[doc = "Bits 29:30 - Reserved"]
    #[inline(always)]
    pub fn i2s1_tx_clk_src_sel(&mut self) -> I2S1_TX_CLK_SRC_SEL_W<PERI_CLK_CTRL15_SPEC> {
        I2S1_TX_CLK_SRC_SEL_W::new(self, 29)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_CLK_CTRL15_SPEC;
impl crate::RegisterSpec for PERI_CLK_CTRL15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl15::R`](R) reader structure"]
impl crate::Readable for PERI_CLK_CTRL15_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl15::W`](W) writer structure"]
impl crate::Writable for PERI_CLK_CTRL15_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL15 to value 0"]
impl crate::Resettable for PERI_CLK_CTRL15_SPEC {}
