#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Field `PREP_DONE_INT_RAW` reader - The raw interrupt status bit for the ecdsa_prep_done_int interrupt"]
pub type PREP_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `PROC_DONE_INT_RAW` reader - The raw interrupt status bit for the ecdsa_proc_done_int interrupt"]
pub type PROC_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `POST_DONE_INT_RAW` reader - The raw interrupt status bit for the ecdsa_post_done_int interrupt"]
pub type POST_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `SHA_RELEASE_INT_RAW` reader - The raw interrupt status bit for the ecdsa_sha_release_int interrupt"]
pub type SHA_RELEASE_INT_RAW_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The raw interrupt status bit for the ecdsa_prep_done_int interrupt"]
    #[inline(always)]
    pub fn prep_done_int_raw(&self) -> PREP_DONE_INT_RAW_R {
        PREP_DONE_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status bit for the ecdsa_proc_done_int interrupt"]
    #[inline(always)]
    pub fn proc_done_int_raw(&self) -> PROC_DONE_INT_RAW_R {
        PROC_DONE_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status bit for the ecdsa_post_done_int interrupt"]
    #[inline(always)]
    pub fn post_done_int_raw(&self) -> POST_DONE_INT_RAW_R {
        POST_DONE_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt status bit for the ecdsa_sha_release_int interrupt"]
    #[inline(always)]
    pub fn sha_release_int_raw(&self) -> SHA_RELEASE_INT_RAW_R {
        SHA_RELEASE_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("prep_done_int_raw", &self.prep_done_int_raw())
            .field("proc_done_int_raw", &self.proc_done_int_raw())
            .field("post_done_int_raw", &self.post_done_int_raw())
            .field("sha_release_int_raw", &self.sha_release_int_raw())
            .finish()
    }
}
#[doc = "ECDSA interrupt raw register, valid in level.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {}
