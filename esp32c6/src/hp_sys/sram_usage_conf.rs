#[doc = "Register `SRAM_USAGE_CONF` reader"]
pub struct R(crate::R<SRAM_USAGE_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAM_USAGE_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAM_USAGE_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAM_USAGE_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRAM_USAGE_CONF` writer"]
pub struct W(crate::W<SRAM_USAGE_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAM_USAGE_CONF_SPEC>;
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
impl From<crate::W<SRAM_USAGE_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAM_USAGE_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CACHE_USAGE` reader - reserved"]
pub type CACHE_USAGE_R = crate::BitReader<bool>;
#[doc = "Field `SRAM_USAGE` reader - 0: cpu use hp-memory. 1:mac-dump accessing hp-memory."]
pub type SRAM_USAGE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SRAM_USAGE` writer - 0: cpu use hp-memory. 1:mac-dump accessing hp-memory."]
pub type SRAM_USAGE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SRAM_USAGE_CONF_SPEC, u8, u8, 4, O>;
#[doc = "Field `MAC_DUMP_ALLOC` reader - Set this bit as 1 to add an offset (64KB) when mac-dump accessing hp-memory."]
pub type MAC_DUMP_ALLOC_R = crate::BitReader<bool>;
#[doc = "Field `MAC_DUMP_ALLOC` writer - Set this bit as 1 to add an offset (64KB) when mac-dump accessing hp-memory."]
pub type MAC_DUMP_ALLOC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SRAM_USAGE_CONF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - reserved"]
    #[inline(always)]
    pub fn cache_usage(&self) -> CACHE_USAGE_R {
        CACHE_USAGE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - 0: cpu use hp-memory. 1:mac-dump accessing hp-memory."]
    #[inline(always)]
    pub fn sram_usage(&self) -> SRAM_USAGE_R {
        SRAM_USAGE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Set this bit as 1 to add an offset (64KB) when mac-dump accessing hp-memory."]
    #[inline(always)]
    pub fn mac_dump_alloc(&self) -> MAC_DUMP_ALLOC_R {
        MAC_DUMP_ALLOC_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:11 - 0: cpu use hp-memory. 1:mac-dump accessing hp-memory."]
    #[inline(always)]
    #[must_use]
    pub fn sram_usage(&mut self) -> SRAM_USAGE_W<8> {
        SRAM_USAGE_W::new(self)
    }
    #[doc = "Bit 16 - Set this bit as 1 to add an offset (64KB) when mac-dump accessing hp-memory."]
    #[inline(always)]
    #[must_use]
    pub fn mac_dump_alloc(&mut self) -> MAC_DUMP_ALLOC_W<16> {
        MAC_DUMP_ALLOC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HP memory usage configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sram_usage_conf](index.html) module"]
pub struct SRAM_USAGE_CONF_SPEC;
impl crate::RegisterSpec for SRAM_USAGE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sram_usage_conf::R](R) reader structure"]
impl crate::Readable for SRAM_USAGE_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sram_usage_conf::W](W) writer structure"]
impl crate::Writable for SRAM_USAGE_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRAM_USAGE_CONF to value 0"]
impl crate::Resettable for SRAM_USAGE_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}