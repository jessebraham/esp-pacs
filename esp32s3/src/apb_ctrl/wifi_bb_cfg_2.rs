#[doc = "Register `WIFI_BB_CFG_2` reader"]
pub struct R(crate::R<WIFI_BB_CFG_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WIFI_BB_CFG_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WIFI_BB_CFG_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WIFI_BB_CFG_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WIFI_BB_CFG_2` writer"]
pub struct W(crate::W<WIFI_BB_CFG_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WIFI_BB_CFG_2_SPEC>;
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
impl From<crate::W<WIFI_BB_CFG_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WIFI_BB_CFG_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WIFI_BB_CFG_2` reader - ******* Description ***********"]
pub type WIFI_BB_CFG_2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WIFI_BB_CFG_2` writer - ******* Description ***********"]
pub type WIFI_BB_CFG_2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WIFI_BB_CFG_2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - ******* Description ***********"]
    #[inline(always)]
    pub fn wifi_bb_cfg_2(&self) -> WIFI_BB_CFG_2_R {
        WIFI_BB_CFG_2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ******* Description ***********"]
    #[inline(always)]
    pub fn wifi_bb_cfg_2(&mut self) -> WIFI_BB_CFG_2_W<0> {
        WIFI_BB_CFG_2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wifi_bb_cfg_2](index.html) module"]
pub struct WIFI_BB_CFG_2_SPEC;
impl crate::RegisterSpec for WIFI_BB_CFG_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wifi_bb_cfg_2::R](R) reader structure"]
impl crate::Readable for WIFI_BB_CFG_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wifi_bb_cfg_2::W](W) writer structure"]
impl crate::Writable for WIFI_BB_CFG_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WIFI_BB_CFG_2 to value 0"]
impl crate::Resettable for WIFI_BB_CFG_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}