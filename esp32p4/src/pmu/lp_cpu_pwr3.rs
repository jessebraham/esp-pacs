#[doc = "Register `LP_CPU_PWR3` reader"]
pub type R = crate::R<LP_CPU_PWR3_SPEC>;
#[doc = "Field `LP_CPU_WAKEUP_CAUSE` reader - need_des"]
pub type LP_CPU_WAKEUP_CAUSE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:30 - need_des"]
    #[inline(always)]
    pub fn lp_cpu_wakeup_cause(&self) -> LP_CPU_WAKEUP_CAUSE_R {
        LP_CPU_WAKEUP_CAUSE_R::new(self.bits & 0x7fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_CPU_PWR3")
            .field("lp_cpu_wakeup_cause", &self.lp_cpu_wakeup_cause())
            .finish()
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_cpu_pwr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_CPU_PWR3_SPEC;
impl crate::RegisterSpec for LP_CPU_PWR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_cpu_pwr3::R`](R) reader structure"]
impl crate::Readable for LP_CPU_PWR3_SPEC {}
#[doc = "`reset()` method sets LP_CPU_PWR3 to value 0"]
impl crate::Resettable for LP_CPU_PWR3_SPEC {}
