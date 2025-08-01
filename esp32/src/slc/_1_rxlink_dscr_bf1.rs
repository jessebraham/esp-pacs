#[doc = "Register `_1_RXLINK_DSCR_BF1` reader"]
pub type R = crate::R<_1_RXLINK_DSCR_BF1_SPEC>;
#[doc = "Field `SLC1_RXLINK_DSCR_BF1` reader - "]
pub type SLC1_RXLINK_DSCR_BF1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn slc1_rxlink_dscr_bf1(&self) -> SLC1_RXLINK_DSCR_BF1_R {
        SLC1_RXLINK_DSCR_BF1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_1_RXLINK_DSCR_BF1")
            .field("slc1_rxlink_dscr_bf1", &self.slc1_rxlink_dscr_bf1())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`_1_rxlink_dscr_bf1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _1_RXLINK_DSCR_BF1_SPEC;
impl crate::RegisterSpec for _1_RXLINK_DSCR_BF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_1_rxlink_dscr_bf1::R`](R) reader structure"]
impl crate::Readable for _1_RXLINK_DSCR_BF1_SPEC {}
#[doc = "`reset()` method sets _1_RXLINK_DSCR_BF1 to value 0"]
impl crate::Resettable for _1_RXLINK_DSCR_BF1_SPEC {}
