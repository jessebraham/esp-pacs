#[doc = "Register `AF_SUM_A` reader"]
pub type R = crate::R<AF_SUM_A_SPEC>;
#[doc = "Field `AF_SUMA` reader - this field represents the result of accumulation of pix grad of focus window a"]
pub type AF_SUMA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:29 - this field represents the result of accumulation of pix grad of focus window a"]
    #[inline(always)]
    pub fn af_suma(&self) -> AF_SUMA_R {
        AF_SUMA_R::new(self.bits & 0x3fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AF_SUM_A")
            .field("af_suma", &format_args!("{}", self.af_suma().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<AF_SUM_A_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "result of sum of af window a\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`af_sum_a::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AF_SUM_A_SPEC;
impl crate::RegisterSpec for AF_SUM_A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af_sum_a::R`](R) reader structure"]
impl crate::Readable for AF_SUM_A_SPEC {}
#[doc = "`reset()` method sets AF_SUM_A to value 0"]
impl crate::Resettable for AF_SUM_A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}