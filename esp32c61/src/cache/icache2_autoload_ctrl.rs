#[doc = "Register `ICACHE2_AUTOLOAD_CTRL` reader"]
pub type R = crate::R<ICACHE2_AUTOLOAD_CTRL_SPEC>;
#[doc = "Field `ICACHE2_AUTOLOAD_ENA` reader - The bit is used to enable and disable autoload operation on L1-ICache2. 1: enable, 0: disable."]
pub type ICACHE2_AUTOLOAD_ENA_R = crate::BitReader;
#[doc = "Field `ICACHE2_AUTOLOAD_DONE` reader - The bit is used to indicate whether autoload operation on L1-ICache2 is finished or not. 0: not finished. 1: finished."]
pub type ICACHE2_AUTOLOAD_DONE_R = crate::BitReader;
#[doc = "Field `ICACHE2_AUTOLOAD_ORDER` reader - The bit is used to configure the direction of autoload operation on L1-ICache2. 0: ascending. 1: descending."]
pub type ICACHE2_AUTOLOAD_ORDER_R = crate::BitReader;
#[doc = "Field `ICACHE2_AUTOLOAD_TRIGGER_MODE` reader - The field is used to configure trigger mode of autoload operation on L1-ICache2. 0/3: miss-trigger, 1: hit-trigger, 2: miss-hit-trigger."]
pub type ICACHE2_AUTOLOAD_TRIGGER_MODE_R = crate::FieldReader;
#[doc = "Field `ICACHE2_AUTOLOAD_SCT0_ENA` reader - The bit is used to enable the first section for autoload operation on L1-ICache2."]
pub type ICACHE2_AUTOLOAD_SCT0_ENA_R = crate::BitReader;
#[doc = "Field `ICACHE2_AUTOLOAD_SCT1_ENA` reader - The bit is used to enable the second section for autoload operation on L1-ICache2."]
pub type ICACHE2_AUTOLOAD_SCT1_ENA_R = crate::BitReader;
#[doc = "Field `ICACHE2_AUTOLOAD_RGID` reader - The bit is used to set the gid of l1 icache2 autoload."]
pub type ICACHE2_AUTOLOAD_RGID_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - The bit is used to enable and disable autoload operation on L1-ICache2. 1: enable, 0: disable."]
    #[inline(always)]
    pub fn icache2_autoload_ena(&self) -> ICACHE2_AUTOLOAD_ENA_R {
        ICACHE2_AUTOLOAD_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to indicate whether autoload operation on L1-ICache2 is finished or not. 0: not finished. 1: finished."]
    #[inline(always)]
    pub fn icache2_autoload_done(&self) -> ICACHE2_AUTOLOAD_DONE_R {
        ICACHE2_AUTOLOAD_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to configure the direction of autoload operation on L1-ICache2. 0: ascending. 1: descending."]
    #[inline(always)]
    pub fn icache2_autoload_order(&self) -> ICACHE2_AUTOLOAD_ORDER_R {
        ICACHE2_AUTOLOAD_ORDER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - The field is used to configure trigger mode of autoload operation on L1-ICache2. 0/3: miss-trigger, 1: hit-trigger, 2: miss-hit-trigger."]
    #[inline(always)]
    pub fn icache2_autoload_trigger_mode(&self) -> ICACHE2_AUTOLOAD_TRIGGER_MODE_R {
        ICACHE2_AUTOLOAD_TRIGGER_MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 8 - The bit is used to enable the first section for autoload operation on L1-ICache2."]
    #[inline(always)]
    pub fn icache2_autoload_sct0_ena(&self) -> ICACHE2_AUTOLOAD_SCT0_ENA_R {
        ICACHE2_AUTOLOAD_SCT0_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The bit is used to enable the second section for autoload operation on L1-ICache2."]
    #[inline(always)]
    pub fn icache2_autoload_sct1_ena(&self) -> ICACHE2_AUTOLOAD_SCT1_ENA_R {
        ICACHE2_AUTOLOAD_SCT1_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - The bit is used to set the gid of l1 icache2 autoload."]
    #[inline(always)]
    pub fn icache2_autoload_rgid(&self) -> ICACHE2_AUTOLOAD_RGID_R {
        ICACHE2_AUTOLOAD_RGID_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACHE2_AUTOLOAD_CTRL")
            .field("icache2_autoload_ena", &self.icache2_autoload_ena())
            .field("icache2_autoload_done", &self.icache2_autoload_done())
            .field("icache2_autoload_order", &self.icache2_autoload_order())
            .field(
                "icache2_autoload_trigger_mode",
                &self.icache2_autoload_trigger_mode(),
            )
            .field(
                "icache2_autoload_sct0_ena",
                &self.icache2_autoload_sct0_ena(),
            )
            .field(
                "icache2_autoload_sct1_ena",
                &self.icache2_autoload_sct1_ena(),
            )
            .field("icache2_autoload_rgid", &self.icache2_autoload_rgid())
            .finish()
    }
}
#[doc = "L1 instruction Cache 2 autoload-operation control register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache2_autoload_ctrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICACHE2_AUTOLOAD_CTRL_SPEC;
impl crate::RegisterSpec for ICACHE2_AUTOLOAD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icache2_autoload_ctrl::R`](R) reader structure"]
impl crate::Readable for ICACHE2_AUTOLOAD_CTRL_SPEC {}
#[doc = "`reset()` method sets ICACHE2_AUTOLOAD_CTRL to value 0x02"]
impl crate::Resettable for ICACHE2_AUTOLOAD_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
