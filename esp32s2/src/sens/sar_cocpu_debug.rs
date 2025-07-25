#[doc = "Register `SAR_COCPU_DEBUG` reader"]
pub type R = crate::R<SAR_COCPU_DEBUG_SPEC>;
#[doc = "Field `COCPU_PC` reader - ULP-RISCV Program counter"]
pub type COCPU_PC_R = crate::FieldReader<u16>;
#[doc = "Field `COCPU_MEM_VLD` reader - ULP-RISCV memory valid output"]
pub type COCPU_MEM_VLD_R = crate::BitReader;
#[doc = "Field `COCPU_MEM_RDY` reader - ULP-RISCV memory ready input"]
pub type COCPU_MEM_RDY_R = crate::BitReader;
#[doc = "Field `COCPU_MEM_WEN` reader - ULP-RISCV memory write enable output"]
pub type COCPU_MEM_WEN_R = crate::FieldReader;
#[doc = "Field `COCPU_MEM_ADDR` reader - ULP-RISCV memory address output"]
pub type COCPU_MEM_ADDR_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:12 - ULP-RISCV Program counter"]
    #[inline(always)]
    pub fn cocpu_pc(&self) -> COCPU_PC_R {
        COCPU_PC_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 13 - ULP-RISCV memory valid output"]
    #[inline(always)]
    pub fn cocpu_mem_vld(&self) -> COCPU_MEM_VLD_R {
        COCPU_MEM_VLD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ULP-RISCV memory ready input"]
    #[inline(always)]
    pub fn cocpu_mem_rdy(&self) -> COCPU_MEM_RDY_R {
        COCPU_MEM_RDY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:18 - ULP-RISCV memory write enable output"]
    #[inline(always)]
    pub fn cocpu_mem_wen(&self) -> COCPU_MEM_WEN_R {
        COCPU_MEM_WEN_R::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bits 19:31 - ULP-RISCV memory address output"]
    #[inline(always)]
    pub fn cocpu_mem_addr(&self) -> COCPU_MEM_ADDR_R {
        COCPU_MEM_ADDR_R::new(((self.bits >> 19) & 0x1fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_COCPU_DEBUG")
            .field("cocpu_pc", &self.cocpu_pc())
            .field("cocpu_mem_vld", &self.cocpu_mem_vld())
            .field("cocpu_mem_rdy", &self.cocpu_mem_rdy())
            .field("cocpu_mem_wen", &self.cocpu_mem_wen())
            .field("cocpu_mem_addr", &self.cocpu_mem_addr())
            .finish()
    }
}
#[doc = "ULP-RISCV debug register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_cocpu_debug::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_COCPU_DEBUG_SPEC;
impl crate::RegisterSpec for SAR_COCPU_DEBUG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_cocpu_debug::R`](R) reader structure"]
impl crate::Readable for SAR_COCPU_DEBUG_SPEC {}
#[doc = "`reset()` method sets SAR_COCPU_DEBUG to value 0"]
impl crate::Resettable for SAR_COCPU_DEBUG_SPEC {}
