#[doc = "Register `COMB_PVT_ERR_HVT_SITE0` reader"]
pub type R = crate::R<COMB_PVT_ERR_HVT_SITE0_SPEC>;
#[doc = "Field `COMB_TIMING_ERR_CNT_HVT_SITE0` reader - reg_comb_timing_err_cnt_hvt_site0"]
pub type COMB_TIMING_ERR_CNT_HVT_SITE0_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - reg_comb_timing_err_cnt_hvt_site0"]
    #[inline(always)]
    pub fn comb_timing_err_cnt_hvt_site0(&self) -> COMB_TIMING_ERR_CNT_HVT_SITE0_R {
        COMB_TIMING_ERR_CNT_HVT_SITE0_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMB_PVT_ERR_HVT_SITE0")
            .field(
                "comb_timing_err_cnt_hvt_site0",
                &self.comb_timing_err_cnt_hvt_site0(),
            )
            .finish()
    }
}
#[doc = "mem pvt register\n\nYou can [`read`](crate::Reg::read) this register and get [`comb_pvt_err_hvt_site0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMB_PVT_ERR_HVT_SITE0_SPEC;
impl crate::RegisterSpec for COMB_PVT_ERR_HVT_SITE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comb_pvt_err_hvt_site0::R`](R) reader structure"]
impl crate::Readable for COMB_PVT_ERR_HVT_SITE0_SPEC {}
#[doc = "`reset()` method sets COMB_PVT_ERR_HVT_SITE0 to value 0"]
impl crate::Resettable for COMB_PVT_ERR_HVT_SITE0_SPEC {}
