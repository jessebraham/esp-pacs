#[doc = "Register `CORE_1_DRAM0_EXCEPTION_MONITOR_1` reader"]
pub type R = crate::R<CORE_1_DRAM0_EXCEPTION_MONITOR_1_SPEC>;
#[doc = "Field `CORE_1_DRAM0_RECORDING_BYTEEN_0` reader - The first dram0's byteen status when trigger DRAM busy interrupt"]
pub type CORE_1_DRAM0_RECORDING_BYTEEN_0_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - The first dram0's byteen status when trigger DRAM busy interrupt"]
    #[inline(always)]
    pub fn core_1_dram0_recording_byteen_0(&self) -> CORE_1_DRAM0_RECORDING_BYTEEN_0_R {
        CORE_1_DRAM0_RECORDING_BYTEEN_0_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_DRAM0_EXCEPTION_MONITOR_1")
            .field(
                "core_1_dram0_recording_byteen_0",
                &self.core_1_dram0_recording_byteen_0(),
            )
            .finish()
    }
}
#[doc = "Core1 bus busy status regsiter\n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_dram0_exception_monitor_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_DRAM0_EXCEPTION_MONITOR_1_SPEC;
impl crate::RegisterSpec for CORE_1_DRAM0_EXCEPTION_MONITOR_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_dram0_exception_monitor_1::R`](R) reader structure"]
impl crate::Readable for CORE_1_DRAM0_EXCEPTION_MONITOR_1_SPEC {}
#[doc = "`reset()` method sets CORE_1_DRAM0_EXCEPTION_MONITOR_1 to value 0"]
impl crate::Resettable for CORE_1_DRAM0_EXCEPTION_MONITOR_1_SPEC {}
