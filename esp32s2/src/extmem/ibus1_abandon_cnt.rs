#[doc = "Register `IBUS1_ABANDON_CNT` reader"]
pub type R = crate::R<IBUS1_ABANDON_CNT_SPEC>;
#[doc = "Field `IBUS1_ABANDON_CNT` reader - The bits are used to count the number of the abandoned ibus1 access."]
pub type IBUS1_ABANDON_CNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - The bits are used to count the number of the abandoned ibus1 access."]
    #[inline(always)]
    pub fn ibus1_abandon_cnt(&self) -> IBUS1_ABANDON_CNT_R {
        IBUS1_ABANDON_CNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IBUS1_ABANDON_CNT")
            .field("ibus1_abandon_cnt", &self.ibus1_abandon_cnt())
            .finish()
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus1_abandon_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IBUS1_ABANDON_CNT_SPEC;
impl crate::RegisterSpec for IBUS1_ABANDON_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ibus1_abandon_cnt::R`](R) reader structure"]
impl crate::Readable for IBUS1_ABANDON_CNT_SPEC {}
#[doc = "`reset()` method sets IBUS1_ABANDON_CNT to value 0"]
impl crate::Resettable for IBUS1_ABANDON_CNT_SPEC {}
