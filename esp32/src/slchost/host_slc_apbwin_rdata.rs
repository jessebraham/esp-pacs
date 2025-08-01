#[doc = "Register `HOST_SLC_APBWIN_RDATA` reader"]
pub type R = crate::R<HOST_SLC_APBWIN_RDATA_SPEC>;
#[doc = "Field `HOST_SLC_APBWIN_RDATA` reader - "]
pub type HOST_SLC_APBWIN_RDATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn host_slc_apbwin_rdata(&self) -> HOST_SLC_APBWIN_RDATA_R {
        HOST_SLC_APBWIN_RDATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLC_APBWIN_RDATA")
            .field("host_slc_apbwin_rdata", &self.host_slc_apbwin_rdata())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`host_slc_apbwin_rdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_SLC_APBWIN_RDATA_SPEC;
impl crate::RegisterSpec for HOST_SLC_APBWIN_RDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_slc_apbwin_rdata::R`](R) reader structure"]
impl crate::Readable for HOST_SLC_APBWIN_RDATA_SPEC {}
#[doc = "`reset()` method sets HOST_SLC_APBWIN_RDATA to value 0"]
impl crate::Resettable for HOST_SLC_APBWIN_RDATA_SPEC {}
