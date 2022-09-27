#[doc = "Register `INT_ST` reader"]
pub struct R(crate::R<INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `THRES1_LOW_INT_ST` reader - Need add description"]
pub type THRES1_LOW_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `THRES0_LOW_INT_ST` reader - Need add description"]
pub type THRES0_LOW_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `THRES1_HIGH_INT_ST` reader - Need add description"]
pub type THRES1_HIGH_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `THRES0_HIGH_INT_ST` reader - Need add description"]
pub type THRES0_HIGH_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `APB_SARADC2_DONE_INT_ST` reader - Need add description"]
pub type APB_SARADC2_DONE_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `APB_SARADC1_DONE_INT_ST` reader - Need add description"]
pub type APB_SARADC1_DONE_INT_ST_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 26 - Need add description"]
    #[inline(always)]
    pub fn thres1_low_int_st(&self) -> THRES1_LOW_INT_ST_R {
        THRES1_LOW_INT_ST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Need add description"]
    #[inline(always)]
    pub fn thres0_low_int_st(&self) -> THRES0_LOW_INT_ST_R {
        THRES0_LOW_INT_ST_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Need add description"]
    #[inline(always)]
    pub fn thres1_high_int_st(&self) -> THRES1_HIGH_INT_ST_R {
        THRES1_HIGH_INT_ST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Need add description"]
    #[inline(always)]
    pub fn thres0_high_int_st(&self) -> THRES0_HIGH_INT_ST_R {
        THRES0_HIGH_INT_ST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Need add description"]
    #[inline(always)]
    pub fn apb_saradc2_done_int_st(&self) -> APB_SARADC2_DONE_INT_ST_R {
        APB_SARADC2_DONE_INT_ST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Need add description"]
    #[inline(always)]
    pub fn apb_saradc1_done_int_st(&self) -> APB_SARADC1_DONE_INT_ST_R {
        APB_SARADC1_DONE_INT_ST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st](index.html) module"]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_st::R](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}