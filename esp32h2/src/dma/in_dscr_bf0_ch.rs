#[doc = "Register `IN_DSCR_BF0_CH%s` reader"]
pub struct R(crate::R<IN_DSCR_BF0_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_DSCR_BF0_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_DSCR_BF0_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_DSCR_BF0_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INLINK_DSCR_BF0` reader - The address of the last inlink descriptor x-1."]
pub type INLINK_DSCR_BF0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The address of the last inlink descriptor x-1."]
    #[inline(always)]
    pub fn inlink_dscr_bf0(&self) -> INLINK_DSCR_BF0_R {
        INLINK_DSCR_BF0_R::new(self.bits)
    }
}
#[doc = "The last inlink descriptor address of Rx channel 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_dscr_bf0_ch](index.html) module"]
pub struct IN_DSCR_BF0_CH_SPEC;
impl crate::RegisterSpec for IN_DSCR_BF0_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_dscr_bf0_ch::R](R) reader structure"]
impl crate::Readable for IN_DSCR_BF0_CH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IN_DSCR_BF0_CH%s to value 0"]
impl crate::Resettable for IN_DSCR_BF0_CH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}