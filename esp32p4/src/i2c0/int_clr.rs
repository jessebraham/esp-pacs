#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `RXFIFO_WM` writer - Write 1 to clear I2C_RXFIFO_WM_INT interrupt."]
pub type RXFIFO_WM_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TXFIFO_WM` writer - Write 1 to clear I2C_TXFIFO_WM_INT interrupt."]
pub type TXFIFO_WM_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RXFIFO_OVF` writer - Write 1 to clear I2C_RXFIFO_OVF_INT interrupt."]
pub type RXFIFO_OVF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `END_DETECT` writer - Write 1 to clear the I2C_END_DETECT_INT interrupt."]
pub type END_DETECT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BYTE_TRANS_DONE` writer - Write 1 to clear the I2C_END_DETECT_INT interrupt."]
pub type BYTE_TRANS_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ARBITRATION_LOST` writer - Write 1 to clear the I2C_ARBITRATION_LOST_INT interrupt."]
pub type ARBITRATION_LOST_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `MST_TXFIFO_UDF` writer - Write 1 to clear I2C_TRANS_COMPLETE_INT interrupt."]
pub type MST_TXFIFO_UDF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TRANS_COMPLETE` writer - Write 1 to clear the I2C_TRANS_COMPLETE_INT interrupt."]
pub type TRANS_COMPLETE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TIME_OUT` writer - Write 1 to clear the I2C_TIME_OUT_INT interrupt."]
pub type TIME_OUT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TRANS_START` writer - Write 1 to clear the I2C_TRANS_START_INT interrupt."]
pub type TRANS_START_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `NACK` writer - Write 1 to clear I2C_SLAVE_STRETCH_INT interrupt."]
pub type NACK_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TXFIFO_OVF` writer - Write 1 to clear I2C_TXFIFO_OVF_INT interrupt."]
pub type TXFIFO_OVF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RXFIFO_UDF` writer - Write 1 to clear I2C_RXFIFO_UDF_INT interrupt."]
pub type RXFIFO_UDF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SCL_ST_TO` writer - Write 1 to clear I2C_SCL_ST_TO_INT interrupt."]
pub type SCL_ST_TO_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SCL_MAIN_ST_TO` writer - Write 1 to clear I2C_SCL_MAIN_ST_TO_INT interrupt."]
pub type SCL_MAIN_ST_TO_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DET_START` writer - Write 1 to clear I2C_DET_START_INT interrupt."]
pub type DET_START_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SLAVE_STRETCH` writer - Write 1 to clear I2C_SLAVE_STRETCH_INT interrupt."]
pub type SLAVE_STRETCH_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `GENERAL_CALL` writer - Write 1 to clear I2C_GENARAL_CALL_INT interrupt."]
pub type GENERAL_CALL_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SLAVE_ADDR_UNMATCH` writer - Write 1 to clear I2C_SLAVE_ADDR_UNMATCH_INT_RAW interrupt."]
pub type SLAVE_ADDR_UNMATCH_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to clear I2C_RXFIFO_WM_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_wm(&mut self) -> RXFIFO_WM_W<INT_CLR_SPEC> {
        RXFIFO_WM_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to clear I2C_TXFIFO_WM_INT interrupt."]
    #[inline(always)]
    pub fn txfifo_wm(&mut self) -> TXFIFO_WM_W<INT_CLR_SPEC> {
        TXFIFO_WM_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to clear I2C_RXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_ovf(&mut self) -> RXFIFO_OVF_W<INT_CLR_SPEC> {
        RXFIFO_OVF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 to clear the I2C_END_DETECT_INT interrupt."]
    #[inline(always)]
    pub fn end_detect(&mut self) -> END_DETECT_W<INT_CLR_SPEC> {
        END_DETECT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1 to clear the I2C_END_DETECT_INT interrupt."]
    #[inline(always)]
    pub fn byte_trans_done(&mut self) -> BYTE_TRANS_DONE_W<INT_CLR_SPEC> {
        BYTE_TRANS_DONE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Write 1 to clear the I2C_ARBITRATION_LOST_INT interrupt."]
    #[inline(always)]
    pub fn arbitration_lost(&mut self) -> ARBITRATION_LOST_W<INT_CLR_SPEC> {
        ARBITRATION_LOST_W::new(self, 5)
    }
    #[doc = "Bit 6 - Write 1 to clear I2C_TRANS_COMPLETE_INT interrupt."]
    #[inline(always)]
    pub fn mst_txfifo_udf(&mut self) -> MST_TXFIFO_UDF_W<INT_CLR_SPEC> {
        MST_TXFIFO_UDF_W::new(self, 6)
    }
    #[doc = "Bit 7 - Write 1 to clear the I2C_TRANS_COMPLETE_INT interrupt."]
    #[inline(always)]
    pub fn trans_complete(&mut self) -> TRANS_COMPLETE_W<INT_CLR_SPEC> {
        TRANS_COMPLETE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Write 1 to clear the I2C_TIME_OUT_INT interrupt."]
    #[inline(always)]
    pub fn time_out(&mut self) -> TIME_OUT_W<INT_CLR_SPEC> {
        TIME_OUT_W::new(self, 8)
    }
    #[doc = "Bit 9 - Write 1 to clear the I2C_TRANS_START_INT interrupt."]
    #[inline(always)]
    pub fn trans_start(&mut self) -> TRANS_START_W<INT_CLR_SPEC> {
        TRANS_START_W::new(self, 9)
    }
    #[doc = "Bit 10 - Write 1 to clear I2C_SLAVE_STRETCH_INT interrupt."]
    #[inline(always)]
    pub fn nack(&mut self) -> NACK_W<INT_CLR_SPEC> {
        NACK_W::new(self, 10)
    }
    #[doc = "Bit 11 - Write 1 to clear I2C_TXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn txfifo_ovf(&mut self) -> TXFIFO_OVF_W<INT_CLR_SPEC> {
        TXFIFO_OVF_W::new(self, 11)
    }
    #[doc = "Bit 12 - Write 1 to clear I2C_RXFIFO_UDF_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_udf(&mut self) -> RXFIFO_UDF_W<INT_CLR_SPEC> {
        RXFIFO_UDF_W::new(self, 12)
    }
    #[doc = "Bit 13 - Write 1 to clear I2C_SCL_ST_TO_INT interrupt."]
    #[inline(always)]
    pub fn scl_st_to(&mut self) -> SCL_ST_TO_W<INT_CLR_SPEC> {
        SCL_ST_TO_W::new(self, 13)
    }
    #[doc = "Bit 14 - Write 1 to clear I2C_SCL_MAIN_ST_TO_INT interrupt."]
    #[inline(always)]
    pub fn scl_main_st_to(&mut self) -> SCL_MAIN_ST_TO_W<INT_CLR_SPEC> {
        SCL_MAIN_ST_TO_W::new(self, 14)
    }
    #[doc = "Bit 15 - Write 1 to clear I2C_DET_START_INT interrupt."]
    #[inline(always)]
    pub fn det_start(&mut self) -> DET_START_W<INT_CLR_SPEC> {
        DET_START_W::new(self, 15)
    }
    #[doc = "Bit 16 - Write 1 to clear I2C_SLAVE_STRETCH_INT interrupt."]
    #[inline(always)]
    pub fn slave_stretch(&mut self) -> SLAVE_STRETCH_W<INT_CLR_SPEC> {
        SLAVE_STRETCH_W::new(self, 16)
    }
    #[doc = "Bit 17 - Write 1 to clear I2C_GENARAL_CALL_INT interrupt."]
    #[inline(always)]
    pub fn general_call(&mut self) -> GENERAL_CALL_W<INT_CLR_SPEC> {
        GENERAL_CALL_W::new(self, 17)
    }
    #[doc = "Bit 18 - Write 1 to clear I2C_SLAVE_ADDR_UNMATCH_INT_RAW interrupt."]
    #[inline(always)]
    pub fn slave_addr_unmatch(&mut self) -> SLAVE_ADDR_UNMATCH_W<INT_CLR_SPEC> {
        SLAVE_ADDR_UNMATCH_W::new(self, 18)
    }
}
#[doc = "Interrupt clear bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0007_ffff;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {}
