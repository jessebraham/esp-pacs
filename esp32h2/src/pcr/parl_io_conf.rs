#[doc = "Register `PARL_IO_CONF` reader"]
pub struct R(crate::R<PARL_IO_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PARL_IO_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PARL_IO_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PARL_IO_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PARL_IO_CONF` writer"]
pub struct W(crate::W<PARL_IO_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PARL_IO_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PARL_IO_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PARL_IO_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PARL_CLK_EN` reader - Set 1 to enable parl apb clock"]
pub type PARL_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `PARL_CLK_EN` writer - Set 1 to enable parl apb clock"]
pub type PARL_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PARL_IO_CONF_SPEC, bool, O>;
#[doc = "Field `PARL_RST_EN` reader - Set 0 to reset parl apb reg"]
pub type PARL_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `PARL_RST_EN` writer - Set 0 to reset parl apb reg"]
pub type PARL_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PARL_IO_CONF_SPEC, bool, O>;
#[doc = "Field `PARL_READY` reader - Query this field after reset parl module"]
pub type PARL_READY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Set 1 to enable parl apb clock"]
    #[inline(always)]
    pub fn parl_clk_en(&self) -> PARL_CLK_EN_R {
        PARL_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset parl apb reg"]
    #[inline(always)]
    pub fn parl_rst_en(&self) -> PARL_RST_EN_R {
        PARL_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Query this field after reset parl module"]
    #[inline(always)]
    pub fn parl_ready(&self) -> PARL_READY_R {
        PARL_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable parl apb clock"]
    #[inline(always)]
    #[must_use]
    pub fn parl_clk_en(&mut self) -> PARL_CLK_EN_W<0> {
        PARL_CLK_EN_W::new(self)
    }
    #[doc = "Bit 1 - Set 0 to reset parl apb reg"]
    #[inline(always)]
    #[must_use]
    pub fn parl_rst_en(&mut self) -> PARL_RST_EN_W<1> {
        PARL_RST_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PARL_IO configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [parl_io_conf](index.html) module"]
pub struct PARL_IO_CONF_SPEC;
impl crate::RegisterSpec for PARL_IO_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [parl_io_conf::R](R) reader structure"]
impl crate::Readable for PARL_IO_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [parl_io_conf::W](W) writer structure"]
impl crate::Writable for PARL_IO_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PARL_IO_CONF to value 0x05"]
impl crate::Resettable for PARL_IO_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}