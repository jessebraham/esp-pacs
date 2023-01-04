#[doc = "Register `ENABLE1_W1TC` writer"]
pub struct W(crate::W<ENABLE1_W1TC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENABLE1_W1TC_SPEC>;
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
impl From<crate::W<ENABLE1_W1TC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENABLE1_W1TC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE1_W1TC` writer - GPIO output enable clear register for GPIO32-34"]
pub type ENABLE1_W1TC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ENABLE1_W1TC_SPEC, u8, u8, 3, O>;
impl W {
    #[doc = "Bits 0:2 - GPIO output enable clear register for GPIO32-34"]
    #[inline(always)]
    #[must_use]
    pub fn enable1_w1tc(&mut self) -> ENABLE1_W1TC_W<0> {
        ENABLE1_W1TC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO output enable clear register for GPIO32-34\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enable1_w1tc](index.html) module"]
pub struct ENABLE1_W1TC_SPEC;
impl crate::RegisterSpec for ENABLE1_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [enable1_w1tc::W](W) writer structure"]
impl crate::Writable for ENABLE1_W1TC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ENABLE1_W1TC to value 0"]
impl crate::Resettable for ENABLE1_W1TC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}