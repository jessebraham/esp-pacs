#[doc = "Register `DEV_CHAR_TABLE5_LOC1` reader"]
pub type R = crate::R<DEV_CHAR_TABLE5_LOC1_SPEC>;
#[doc = "Field `DCT_DEV5_LOC1` reader - NA"]
pub type DCT_DEV5_LOC1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn dct_dev5_loc1(&self) -> DCT_DEV5_LOC1_R {
        DCT_DEV5_LOC1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEV_CHAR_TABLE5_LOC1")
            .field("dct_dev5_loc1", &self.dct_dev5_loc1())
            .finish()
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_char_table5_loc1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEV_CHAR_TABLE5_LOC1_SPEC;
impl crate::RegisterSpec for DEV_CHAR_TABLE5_LOC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_char_table5_loc1::R`](R) reader structure"]
impl crate::Readable for DEV_CHAR_TABLE5_LOC1_SPEC {}
#[doc = "`reset()` method sets DEV_CHAR_TABLE5_LOC1 to value 0"]
impl crate::Resettable for DEV_CHAR_TABLE5_LOC1_SPEC {}
