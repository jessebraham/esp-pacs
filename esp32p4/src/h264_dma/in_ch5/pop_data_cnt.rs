#[doc = "Register `POP_DATA_CNT` reader"]
pub type R = crate::R<POP_DATA_CNT_SPEC>;
#[doc = "Field `IN_CMDFIFO_POP_DATA_CNT` reader - only for debug"]
pub type IN_CMDFIFO_POP_DATA_CNT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - only for debug"]
    #[inline(always)]
    pub fn in_cmdfifo_pop_data_cnt(&self) -> IN_CMDFIFO_POP_DATA_CNT_R {
        IN_CMDFIFO_POP_DATA_CNT_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POP_DATA_CNT")
            .field("in_cmdfifo_pop_data_cnt", &self.in_cmdfifo_pop_data_cnt())
            .finish()
    }
}
#[doc = "rx CH5 pop data cnt register\n\nYou can [`read`](crate::Reg::read) this register and get [`pop_data_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POP_DATA_CNT_SPEC;
impl crate::RegisterSpec for POP_DATA_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pop_data_cnt::R`](R) reader structure"]
impl crate::Readable for POP_DATA_CNT_SPEC {}
#[doc = "`reset()` method sets POP_DATA_CNT to value 0xff"]
impl crate::Resettable for POP_DATA_CNT_SPEC {
    const RESET_VALUE: u32 = 0xff;
}
