#[doc = "Register `RNG_DATA_SYNC` reader"]
pub type R = crate::R<RNG_DATA_SYNC_SPEC>;
#[doc = "Field `RND_SYNC_DATA` reader - need_des"]
pub type RND_SYNC_DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn rnd_sync_data(&self) -> RND_SYNC_DATA_R {
        RND_SYNC_DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RNG_DATA_SYNC")
            .field("rnd_sync_data", &self.rnd_sync_data())
            .finish()
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_data_sync::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RNG_DATA_SYNC_SPEC;
impl crate::RegisterSpec for RNG_DATA_SYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_data_sync::R`](R) reader structure"]
impl crate::Readable for RNG_DATA_SYNC_SPEC {}
#[doc = "`reset()` method sets RNG_DATA_SYNC to value 0"]
impl crate::Resettable for RNG_DATA_SYNC_SPEC {}
