#[doc = "Register `PERI_CLK_CTRL115` reader"]
pub type R = crate::R<PERI_CLK_CTRL115_SPEC>;
#[doc = "Register `PERI_CLK_CTRL115` writer"]
pub type W = crate::W<PERI_CLK_CTRL115_SPEC>;
#[doc = "Field `UART4_SCLK_DIV_NUM` reader - Reserved"]
pub type UART4_SCLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `UART4_SCLK_DIV_NUM` writer - Reserved"]
pub type UART4_SCLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `UART4_SCLK_DIV_NUMERATOR` reader - Reserved"]
pub type UART4_SCLK_DIV_NUMERATOR_R = crate::FieldReader;
#[doc = "Field `UART4_SCLK_DIV_NUMERATOR` writer - Reserved"]
pub type UART4_SCLK_DIV_NUMERATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `UART4_SCLK_DIV_DENOMINATOR` reader - Reserved"]
pub type UART4_SCLK_DIV_DENOMINATOR_R = crate::FieldReader;
#[doc = "Field `UART4_SCLK_DIV_DENOMINATOR` writer - Reserved"]
pub type UART4_SCLK_DIV_DENOMINATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TWAI0_CLK_SRC_SEL` reader - Reserved"]
pub type TWAI0_CLK_SRC_SEL_R = crate::BitReader;
#[doc = "Field `TWAI0_CLK_SRC_SEL` writer - Reserved"]
pub type TWAI0_CLK_SRC_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWAI0_CLK_EN` reader - Reserved"]
pub type TWAI0_CLK_EN_R = crate::BitReader;
#[doc = "Field `TWAI0_CLK_EN` writer - Reserved"]
pub type TWAI0_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWAI1_CLK_SRC_SEL` reader - Reserved"]
pub type TWAI1_CLK_SRC_SEL_R = crate::BitReader;
#[doc = "Field `TWAI1_CLK_SRC_SEL` writer - Reserved"]
pub type TWAI1_CLK_SRC_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWAI1_CLK_EN` reader - Reserved"]
pub type TWAI1_CLK_EN_R = crate::BitReader;
#[doc = "Field `TWAI1_CLK_EN` writer - Reserved"]
pub type TWAI1_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWAI2_CLK_SRC_SEL` reader - Reserved"]
pub type TWAI2_CLK_SRC_SEL_R = crate::BitReader;
#[doc = "Field `TWAI2_CLK_SRC_SEL` writer - Reserved"]
pub type TWAI2_CLK_SRC_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWAI2_CLK_EN` reader - Reserved"]
pub type TWAI2_CLK_EN_R = crate::BitReader;
#[doc = "Field `TWAI2_CLK_EN` writer - Reserved"]
pub type TWAI2_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn uart4_sclk_div_num(&self) -> UART4_SCLK_DIV_NUM_R {
        UART4_SCLK_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn uart4_sclk_div_numerator(&self) -> UART4_SCLK_DIV_NUMERATOR_R {
        UART4_SCLK_DIV_NUMERATOR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    pub fn uart4_sclk_div_denominator(&self) -> UART4_SCLK_DIV_DENOMINATOR_R {
        UART4_SCLK_DIV_DENOMINATOR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    pub fn twai0_clk_src_sel(&self) -> TWAI0_CLK_SRC_SEL_R {
        TWAI0_CLK_SRC_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn twai0_clk_en(&self) -> TWAI0_CLK_EN_R {
        TWAI0_CLK_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Reserved"]
    #[inline(always)]
    pub fn twai1_clk_src_sel(&self) -> TWAI1_CLK_SRC_SEL_R {
        TWAI1_CLK_SRC_SEL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn twai1_clk_en(&self) -> TWAI1_CLK_EN_R {
        TWAI1_CLK_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn twai2_clk_src_sel(&self) -> TWAI2_CLK_SRC_SEL_R {
        TWAI2_CLK_SRC_SEL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Reserved"]
    #[inline(always)]
    pub fn twai2_clk_en(&self) -> TWAI2_CLK_EN_R {
        TWAI2_CLK_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_CLK_CTRL115")
            .field("uart4_sclk_div_num", &self.uart4_sclk_div_num())
            .field("uart4_sclk_div_numerator", &self.uart4_sclk_div_numerator())
            .field(
                "uart4_sclk_div_denominator",
                &self.uart4_sclk_div_denominator(),
            )
            .field("twai0_clk_src_sel", &self.twai0_clk_src_sel())
            .field("twai0_clk_en", &self.twai0_clk_en())
            .field("twai1_clk_src_sel", &self.twai1_clk_src_sel())
            .field("twai1_clk_en", &self.twai1_clk_en())
            .field("twai2_clk_src_sel", &self.twai2_clk_src_sel())
            .field("twai2_clk_en", &self.twai2_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn uart4_sclk_div_num(&mut self) -> UART4_SCLK_DIV_NUM_W<PERI_CLK_CTRL115_SPEC> {
        UART4_SCLK_DIV_NUM_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn uart4_sclk_div_numerator(
        &mut self,
    ) -> UART4_SCLK_DIV_NUMERATOR_W<PERI_CLK_CTRL115_SPEC> {
        UART4_SCLK_DIV_NUMERATOR_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    pub fn uart4_sclk_div_denominator(
        &mut self,
    ) -> UART4_SCLK_DIV_DENOMINATOR_W<PERI_CLK_CTRL115_SPEC> {
        UART4_SCLK_DIV_DENOMINATOR_W::new(self, 16)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    pub fn twai0_clk_src_sel(&mut self) -> TWAI0_CLK_SRC_SEL_W<PERI_CLK_CTRL115_SPEC> {
        TWAI0_CLK_SRC_SEL_W::new(self, 24)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn twai0_clk_en(&mut self) -> TWAI0_CLK_EN_W<PERI_CLK_CTRL115_SPEC> {
        TWAI0_CLK_EN_W::new(self, 25)
    }
    #[doc = "Bit 26 - Reserved"]
    #[inline(always)]
    pub fn twai1_clk_src_sel(&mut self) -> TWAI1_CLK_SRC_SEL_W<PERI_CLK_CTRL115_SPEC> {
        TWAI1_CLK_SRC_SEL_W::new(self, 26)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn twai1_clk_en(&mut self) -> TWAI1_CLK_EN_W<PERI_CLK_CTRL115_SPEC> {
        TWAI1_CLK_EN_W::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn twai2_clk_src_sel(&mut self) -> TWAI2_CLK_SRC_SEL_W<PERI_CLK_CTRL115_SPEC> {
        TWAI2_CLK_SRC_SEL_W::new(self, 28)
    }
    #[doc = "Bit 29 - Reserved"]
    #[inline(always)]
    pub fn twai2_clk_en(&mut self) -> TWAI2_CLK_EN_W<PERI_CLK_CTRL115_SPEC> {
        TWAI2_CLK_EN_W::new(self, 29)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl115::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl115::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_CLK_CTRL115_SPEC;
impl crate::RegisterSpec for PERI_CLK_CTRL115_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl115::R`](R) reader structure"]
impl crate::Readable for PERI_CLK_CTRL115_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl115::W`](W) writer structure"]
impl crate::Writable for PERI_CLK_CTRL115_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL115 to value 0"]
impl crate::Resettable for PERI_CLK_CTRL115_SPEC {}
