#[doc = "Register `REDUNDANCY_SIG4` reader"]
pub type R = crate::R<REDUNDANCY_SIG4_SPEC>;
#[doc = "Field `REDCY_SIG4` reader - Those bits are prepared for ECO."]
pub type REDCY_SIG4_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Those bits are prepared for ECO."]
    #[inline(always)]
    pub fn redcy_sig4(&self) -> REDCY_SIG4_R {
        REDCY_SIG4_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REDUNDANCY_SIG4")
            .field("redcy_sig4", &self.redcy_sig4())
            .finish()
    }
}
#[doc = "Cache redundancy signal 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`redundancy_sig4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REDUNDANCY_SIG4_SPEC;
impl crate::RegisterSpec for REDUNDANCY_SIG4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`redundancy_sig4::R`](R) reader structure"]
impl crate::Readable for REDUNDANCY_SIG4_SPEC {}
#[doc = "`reset()` method sets REDUNDANCY_SIG4 to value 0"]
impl crate::Resettable for REDUNDANCY_SIG4_SPEC {}
