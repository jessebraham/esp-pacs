#[doc = "Register `SPI_EXT1` reader"]
pub struct R(crate::R<SPI_EXT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_EXT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_EXT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_EXT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_EXT1` writer"]
pub struct W(crate::W<SPI_EXT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_EXT1_SPEC>;
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
impl From<crate::W<SPI_EXT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_EXT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `erase_time` reader - "]
pub type ERASE_TIME_R = crate::FieldReader<u16, u16>;
#[doc = "Field `erase_time` writer - "]
pub type ERASE_TIME_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPI_EXT1_SPEC, u16, u16, 12, O>;
#[doc = "Field `erase_shift` reader - "]
pub type ERASE_SHIFT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `erase_shift` writer - "]
pub type ERASE_SHIFT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI_EXT1_SPEC, u8, u8, 4, O>;
#[doc = "Field `erase_enable` reader - "]
pub type ERASE_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `erase_enable` writer - "]
pub type ERASE_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_EXT1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn erase_time(&self) -> ERASE_TIME_R {
        ERASE_TIME_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn erase_shift(&self) -> ERASE_SHIFT_R {
        ERASE_SHIFT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn erase_enable(&self) -> ERASE_ENABLE_R {
        ERASE_ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn erase_time(&mut self) -> ERASE_TIME_W<0> {
        ERASE_TIME_W::new(self)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn erase_shift(&mut self) -> ERASE_SHIFT_W<16> {
        ERASE_SHIFT_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn erase_enable(&mut self) -> ERASE_ENABLE_W<31> {
        ERASE_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_ext1](index.html) module"]
pub struct SPI_EXT1_SPEC;
impl crate::RegisterSpec for SPI_EXT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_ext1::R](R) reader structure"]
impl crate::Readable for SPI_EXT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_ext1::W](W) writer structure"]
impl crate::Writable for SPI_EXT1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_EXT1 to value 0"]
impl crate::Resettable for SPI_EXT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}