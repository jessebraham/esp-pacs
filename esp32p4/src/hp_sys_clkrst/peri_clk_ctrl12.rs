#[doc = "Register `PERI_CLK_CTRL12` reader"]
pub type R = crate::R<PERI_CLK_CTRL12_SPEC>;
#[doc = "Register `PERI_CLK_CTRL12` writer"]
pub type W = crate::W<PERI_CLK_CTRL12_SPEC>;
#[doc = "Field `REG_I2S0_RX_DIV_N` reader - Reserved"]
pub type REG_I2S0_RX_DIV_N_R = crate::FieldReader;
#[doc = "Field `REG_I2S0_RX_DIV_N` writer - Reserved"]
pub type REG_I2S0_RX_DIV_N_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_I2S0_RX_DIV_X` reader - Reserved"]
pub type REG_I2S0_RX_DIV_X_R = crate::FieldReader<u16>;
#[doc = "Field `REG_I2S0_RX_DIV_X` writer - Reserved"]
pub type REG_I2S0_RX_DIV_X_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `REG_I2S0_RX_DIV_Y` reader - Reserved"]
pub type REG_I2S0_RX_DIV_Y_R = crate::FieldReader<u16>;
#[doc = "Field `REG_I2S0_RX_DIV_Y` writer - Reserved"]
pub type REG_I2S0_RX_DIV_Y_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn reg_i2s0_rx_div_n(&self) -> REG_I2S0_RX_DIV_N_R {
        REG_I2S0_RX_DIV_N_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:16 - Reserved"]
    #[inline(always)]
    pub fn reg_i2s0_rx_div_x(&self) -> REG_I2S0_RX_DIV_X_R {
        REG_I2S0_RX_DIV_X_R::new(((self.bits >> 8) & 0x01ff) as u16)
    }
    #[doc = "Bits 17:25 - Reserved"]
    #[inline(always)]
    pub fn reg_i2s0_rx_div_y(&self) -> REG_I2S0_RX_DIV_Y_R {
        REG_I2S0_RX_DIV_Y_R::new(((self.bits >> 17) & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_CLK_CTRL12")
            .field(
                "reg_i2s0_rx_div_n",
                &format_args!("{}", self.reg_i2s0_rx_div_n().bits()),
            )
            .field(
                "reg_i2s0_rx_div_x",
                &format_args!("{}", self.reg_i2s0_rx_div_x().bits()),
            )
            .field(
                "reg_i2s0_rx_div_y",
                &format_args!("{}", self.reg_i2s0_rx_div_y().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PERI_CLK_CTRL12_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_i2s0_rx_div_n(&mut self) -> REG_I2S0_RX_DIV_N_W<PERI_CLK_CTRL12_SPEC> {
        REG_I2S0_RX_DIV_N_W::new(self, 0)
    }
    #[doc = "Bits 8:16 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_i2s0_rx_div_x(&mut self) -> REG_I2S0_RX_DIV_X_W<PERI_CLK_CTRL12_SPEC> {
        REG_I2S0_RX_DIV_X_W::new(self, 8)
    }
    #[doc = "Bits 17:25 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_i2s0_rx_div_y(&mut self) -> REG_I2S0_RX_DIV_Y_W<PERI_CLK_CTRL12_SPEC> {
        REG_I2S0_RX_DIV_Y_W::new(self, 17)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peri_clk_ctrl12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peri_clk_ctrl12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_CLK_CTRL12_SPEC;
impl crate::RegisterSpec for PERI_CLK_CTRL12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl12::R`](R) reader structure"]
impl crate::Readable for PERI_CLK_CTRL12_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl12::W`](W) writer structure"]
impl crate::Writable for PERI_CLK_CTRL12_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL12 to value 0"]
impl crate::Resettable for PERI_CLK_CTRL12_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}