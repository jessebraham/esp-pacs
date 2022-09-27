#[doc = "Register `Q0_WORD0` reader"]
pub struct R(crate::R<Q0_WORD0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<Q0_WORD0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<Q0_WORD0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<Q0_WORD0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Q0_WORD0` writer"]
pub struct W(crate::W<Q0_WORD0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<Q0_WORD0_SPEC>;
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
impl From<crate::W<Q0_WORD0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<Q0_WORD0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEND_Q0_WORD0` reader - This register is used as a quick_sent register when mode is specified by UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM."]
pub type SEND_Q0_WORD0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SEND_Q0_WORD0` writer - This register is used as a quick_sent register when mode is specified by UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM."]
pub type SEND_Q0_WORD0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, Q0_WORD0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - This register is used as a quick_sent register when mode is specified by UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM."]
    #[inline(always)]
    pub fn send_q0_word0(&self) -> SEND_Q0_WORD0_R {
        SEND_Q0_WORD0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register is used as a quick_sent register when mode is specified by UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM."]
    #[inline(always)]
    pub fn send_q0_word0(&mut self) -> SEND_Q0_WORD0_W<0> {
        SEND_Q0_WORD0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Q0_WORD0 quick_sent register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [q0_word0](index.html) module"]
pub struct Q0_WORD0_SPEC;
impl crate::RegisterSpec for Q0_WORD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [q0_word0::R](R) reader structure"]
impl crate::Readable for Q0_WORD0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [q0_word0::W](W) writer structure"]
impl crate::Writable for Q0_WORD0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Q0_WORD0 to value 0"]
impl crate::Resettable for Q0_WORD0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}