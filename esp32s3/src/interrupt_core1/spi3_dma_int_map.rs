#[doc = "Register `SPI3_DMA_INT_MAP` reader"]
pub struct R(crate::R<SPI3_DMA_INT_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI3_DMA_INT_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI3_DMA_INT_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI3_DMA_INT_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI3_DMA_INT_MAP` writer"]
pub struct W(crate::W<SPI3_DMA_INT_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI3_DMA_INT_MAP_SPEC>;
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
impl From<crate::W<SPI3_DMA_INT_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI3_DMA_INT_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI3_DMA_INT_MAP` reader - this register used to map spi3_dma interrupt to one of core1's external interrupt"]
pub type SPI3_DMA_INT_MAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI3_DMA_INT_MAP` writer - this register used to map spi3_dma interrupt to one of core1's external interrupt"]
pub type SPI3_DMA_INT_MAP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPI3_DMA_INT_MAP_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - this register used to map spi3_dma interrupt to one of core1's external interrupt"]
    #[inline(always)]
    pub fn spi3_dma_int_map(&self) -> SPI3_DMA_INT_MAP_R {
        SPI3_DMA_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - this register used to map spi3_dma interrupt to one of core1's external interrupt"]
    #[inline(always)]
    pub fn spi3_dma_int_map(&mut self) -> SPI3_DMA_INT_MAP_W<0> {
        SPI3_DMA_INT_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "spi3_dma interrupt configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi3_dma_int_map](index.html) module"]
pub struct SPI3_DMA_INT_MAP_SPEC;
impl crate::RegisterSpec for SPI3_DMA_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi3_dma_int_map::R](R) reader structure"]
impl crate::Readable for SPI3_DMA_INT_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi3_dma_int_map::W](W) writer structure"]
impl crate::Writable for SPI3_DMA_INT_MAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI3_DMA_INT_MAP to value 0x10"]
impl crate::Resettable for SPI3_DMA_INT_MAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}