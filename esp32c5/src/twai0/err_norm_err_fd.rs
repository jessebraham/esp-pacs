#[doc = "Register `ERR_NORM_ERR_FD` reader"]
pub type R = crate::R<ERR_NORM_ERR_FD_SPEC>;
#[doc = "Field `ERR_NORM_VAL` reader - Represents the number of error in the nominal bit time."]
pub type ERR_NORM_VAL_R = crate::FieldReader<u16>;
#[doc = "Field `ERR_FD_VAL` reader - Represents the number of error in the data bit time."]
pub type ERR_FD_VAL_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Represents the number of error in the nominal bit time."]
    #[inline(always)]
    pub fn err_norm_val(&self) -> ERR_NORM_VAL_R {
        ERR_NORM_VAL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Represents the number of error in the data bit time."]
    #[inline(always)]
    pub fn err_fd_val(&self) -> ERR_FD_VAL_R {
        ERR_FD_VAL_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ERR_NORM_ERR_FD")
            .field("err_norm_val", &self.err_norm_val())
            .field("err_fd_val", &self.err_fd_val())
            .finish()
    }
}
#[doc = "TWAI FD special error counters status register\n\nYou can [`read`](crate::Reg::read) this register and get [`err_norm_err_fd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ERR_NORM_ERR_FD_SPEC;
impl crate::RegisterSpec for ERR_NORM_ERR_FD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err_norm_err_fd::R`](R) reader structure"]
impl crate::Readable for ERR_NORM_ERR_FD_SPEC {}
#[doc = "`reset()` method sets ERR_NORM_ERR_FD to value 0"]
impl crate::Resettable for ERR_NORM_ERR_FD_SPEC {}
