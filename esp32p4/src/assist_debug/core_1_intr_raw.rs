#[doc = "Register `CORE_1_INTR_RAW` reader"]
pub type R = crate::R<CORE_1_INTR_RAW_SPEC>;
#[doc = "Field `CORE_1_AREA_DRAM0_0_RD_RAW` reader - Core1 dram0 area0 read monitor interrupt status"]
pub type CORE_1_AREA_DRAM0_0_RD_RAW_R = crate::BitReader;
#[doc = "Field `CORE_1_AREA_DRAM0_0_WR_RAW` reader - Core1 dram0 area0 write monitor interrupt status"]
pub type CORE_1_AREA_DRAM0_0_WR_RAW_R = crate::BitReader;
#[doc = "Field `CORE_1_AREA_DRAM0_1_RD_RAW` reader - Core1 dram0 area1 read monitor interrupt status"]
pub type CORE_1_AREA_DRAM0_1_RD_RAW_R = crate::BitReader;
#[doc = "Field `CORE_1_AREA_DRAM0_1_WR_RAW` reader - Core1 dram0 area1 write monitor interrupt status"]
pub type CORE_1_AREA_DRAM0_1_WR_RAW_R = crate::BitReader;
#[doc = "Field `CORE_1_AREA_PIF_0_RD_RAW` reader - Core1 PIF area0 read monitor interrupt status"]
pub type CORE_1_AREA_PIF_0_RD_RAW_R = crate::BitReader;
#[doc = "Field `CORE_1_AREA_PIF_0_WR_RAW` reader - Core1 PIF area0 write monitor interrupt status"]
pub type CORE_1_AREA_PIF_0_WR_RAW_R = crate::BitReader;
#[doc = "Field `CORE_1_AREA_PIF_1_RD_RAW` reader - Core1 PIF area1 read monitor interrupt status"]
pub type CORE_1_AREA_PIF_1_RD_RAW_R = crate::BitReader;
#[doc = "Field `CORE_1_AREA_PIF_1_WR_RAW` reader - Core1 PIF area1 write monitor interrupt status"]
pub type CORE_1_AREA_PIF_1_WR_RAW_R = crate::BitReader;
#[doc = "Field `CORE_1_SP_SPILL_MIN_RAW` reader - Core1 stackpoint underflow monitor interrupt status"]
pub type CORE_1_SP_SPILL_MIN_RAW_R = crate::BitReader;
#[doc = "Field `CORE_1_SP_SPILL_MAX_RAW` reader - Core1 stackpoint overflow monitor interrupt status"]
pub type CORE_1_SP_SPILL_MAX_RAW_R = crate::BitReader;
#[doc = "Field `CORE_1_IRAM0_EXCEPTION_MONITOR_RAW` reader - IBUS busy monitor interrupt status"]
pub type CORE_1_IRAM0_EXCEPTION_MONITOR_RAW_R = crate::BitReader;
#[doc = "Field `CORE_1_DRAM0_EXCEPTION_MONITOR_RAW` reader - DBUS busy monitor initerrupt status"]
pub type CORE_1_DRAM0_EXCEPTION_MONITOR_RAW_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Core1 dram0 area0 read monitor interrupt status"]
    #[inline(always)]
    pub fn core_1_area_dram0_0_rd_raw(&self) -> CORE_1_AREA_DRAM0_0_RD_RAW_R {
        CORE_1_AREA_DRAM0_0_RD_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Core1 dram0 area0 write monitor interrupt status"]
    #[inline(always)]
    pub fn core_1_area_dram0_0_wr_raw(&self) -> CORE_1_AREA_DRAM0_0_WR_RAW_R {
        CORE_1_AREA_DRAM0_0_WR_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Core1 dram0 area1 read monitor interrupt status"]
    #[inline(always)]
    pub fn core_1_area_dram0_1_rd_raw(&self) -> CORE_1_AREA_DRAM0_1_RD_RAW_R {
        CORE_1_AREA_DRAM0_1_RD_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Core1 dram0 area1 write monitor interrupt status"]
    #[inline(always)]
    pub fn core_1_area_dram0_1_wr_raw(&self) -> CORE_1_AREA_DRAM0_1_WR_RAW_R {
        CORE_1_AREA_DRAM0_1_WR_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Core1 PIF area0 read monitor interrupt status"]
    #[inline(always)]
    pub fn core_1_area_pif_0_rd_raw(&self) -> CORE_1_AREA_PIF_0_RD_RAW_R {
        CORE_1_AREA_PIF_0_RD_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Core1 PIF area0 write monitor interrupt status"]
    #[inline(always)]
    pub fn core_1_area_pif_0_wr_raw(&self) -> CORE_1_AREA_PIF_0_WR_RAW_R {
        CORE_1_AREA_PIF_0_WR_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Core1 PIF area1 read monitor interrupt status"]
    #[inline(always)]
    pub fn core_1_area_pif_1_rd_raw(&self) -> CORE_1_AREA_PIF_1_RD_RAW_R {
        CORE_1_AREA_PIF_1_RD_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Core1 PIF area1 write monitor interrupt status"]
    #[inline(always)]
    pub fn core_1_area_pif_1_wr_raw(&self) -> CORE_1_AREA_PIF_1_WR_RAW_R {
        CORE_1_AREA_PIF_1_WR_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Core1 stackpoint underflow monitor interrupt status"]
    #[inline(always)]
    pub fn core_1_sp_spill_min_raw(&self) -> CORE_1_SP_SPILL_MIN_RAW_R {
        CORE_1_SP_SPILL_MIN_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Core1 stackpoint overflow monitor interrupt status"]
    #[inline(always)]
    pub fn core_1_sp_spill_max_raw(&self) -> CORE_1_SP_SPILL_MAX_RAW_R {
        CORE_1_SP_SPILL_MAX_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IBUS busy monitor interrupt status"]
    #[inline(always)]
    pub fn core_1_iram0_exception_monitor_raw(&self) -> CORE_1_IRAM0_EXCEPTION_MONITOR_RAW_R {
        CORE_1_IRAM0_EXCEPTION_MONITOR_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DBUS busy monitor initerrupt status"]
    #[inline(always)]
    pub fn core_1_dram0_exception_monitor_raw(&self) -> CORE_1_DRAM0_EXCEPTION_MONITOR_RAW_R {
        CORE_1_DRAM0_EXCEPTION_MONITOR_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_INTR_RAW")
            .field(
                "core_1_area_dram0_0_rd_raw",
                &self.core_1_area_dram0_0_rd_raw(),
            )
            .field(
                "core_1_area_dram0_0_wr_raw",
                &self.core_1_area_dram0_0_wr_raw(),
            )
            .field(
                "core_1_area_dram0_1_rd_raw",
                &self.core_1_area_dram0_1_rd_raw(),
            )
            .field(
                "core_1_area_dram0_1_wr_raw",
                &self.core_1_area_dram0_1_wr_raw(),
            )
            .field("core_1_area_pif_0_rd_raw", &self.core_1_area_pif_0_rd_raw())
            .field("core_1_area_pif_0_wr_raw", &self.core_1_area_pif_0_wr_raw())
            .field("core_1_area_pif_1_rd_raw", &self.core_1_area_pif_1_rd_raw())
            .field("core_1_area_pif_1_wr_raw", &self.core_1_area_pif_1_wr_raw())
            .field("core_1_sp_spill_min_raw", &self.core_1_sp_spill_min_raw())
            .field("core_1_sp_spill_max_raw", &self.core_1_sp_spill_max_raw())
            .field(
                "core_1_iram0_exception_monitor_raw",
                &self.core_1_iram0_exception_monitor_raw(),
            )
            .field(
                "core_1_dram0_exception_monitor_raw",
                &self.core_1_dram0_exception_monitor_raw(),
            )
            .finish()
    }
}
#[doc = "core1 monitor interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_intr_raw::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_INTR_RAW_SPEC;
impl crate::RegisterSpec for CORE_1_INTR_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_intr_raw::R`](R) reader structure"]
impl crate::Readable for CORE_1_INTR_RAW_SPEC {}
#[doc = "`reset()` method sets CORE_1_INTR_RAW to value 0"]
impl crate::Resettable for CORE_1_INTR_RAW_SPEC {}
