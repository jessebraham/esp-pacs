#[doc = "Register `SR` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Field `RESP_REC` reader - Represents the received ACK value in master mode or slave mode. 0: ACK, 1: NACK."]
pub type RESP_REC_R = crate::BitReader;
#[doc = "Field `ARB_LOST` reader - Represents whether the I2C controller loses control of SCL line. 0: No arbitration lost 1: Arbitration lost"]
pub type ARB_LOST_R = crate::BitReader;
#[doc = "Field `BUS_BUSY` reader - Represents the I2C bus state. 1: The I2C bus is busy transferring data, 0: The I2C bus is in idle state."]
pub type BUS_BUSY_R = crate::BitReader;
#[doc = "Field `RXFIFO_CNT` reader - Represents the number of data bytes to be sent."]
pub type RXFIFO_CNT_R = crate::FieldReader;
#[doc = "Field `TXFIFO_CNT` reader - Represents the number of data bytes received in RAM."]
pub type TXFIFO_CNT_R = crate::FieldReader;
#[doc = "Field `SCL_MAIN_STATE_LAST` reader - Represents the states of the I2C module state machine. 0: Idle, 1: Address shift, 2: ACK address, 3: Rx data, 4: Tx data, 5: Send ACK, 6: Wait ACK"]
pub type SCL_MAIN_STATE_LAST_R = crate::FieldReader;
#[doc = "Field `SCL_STATE_LAST` reader - Represents the states of the state machine used to produce SCL. 0: Idle, 1: Start, 2: Negative edge, 3: Low, 4: Positive edge, 5: High, 6: Stop"]
pub type SCL_STATE_LAST_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Represents the received ACK value in master mode or slave mode. 0: ACK, 1: NACK."]
    #[inline(always)]
    pub fn resp_rec(&self) -> RESP_REC_R {
        RESP_REC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Represents whether the I2C controller loses control of SCL line. 0: No arbitration lost 1: Arbitration lost"]
    #[inline(always)]
    pub fn arb_lost(&self) -> ARB_LOST_R {
        ARB_LOST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Represents the I2C bus state. 1: The I2C bus is busy transferring data, 0: The I2C bus is in idle state."]
    #[inline(always)]
    pub fn bus_busy(&self) -> BUS_BUSY_R {
        BUS_BUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Represents the number of data bytes to be sent."]
    #[inline(always)]
    pub fn rxfifo_cnt(&self) -> RXFIFO_CNT_R {
        RXFIFO_CNT_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 18:22 - Represents the number of data bytes received in RAM."]
    #[inline(always)]
    pub fn txfifo_cnt(&self) -> TXFIFO_CNT_R {
        TXFIFO_CNT_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26 - Represents the states of the I2C module state machine. 0: Idle, 1: Address shift, 2: ACK address, 3: Rx data, 4: Tx data, 5: Send ACK, 6: Wait ACK"]
    #[inline(always)]
    pub fn scl_main_state_last(&self) -> SCL_MAIN_STATE_LAST_R {
        SCL_MAIN_STATE_LAST_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Represents the states of the state machine used to produce SCL. 0: Idle, 1: Start, 2: Negative edge, 3: Low, 4: Positive edge, 5: High, 6: Stop"]
    #[inline(always)]
    pub fn scl_state_last(&self) -> SCL_STATE_LAST_R {
        SCL_STATE_LAST_R::new(((self.bits >> 28) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("resp_rec", &self.resp_rec())
            .field("arb_lost", &self.arb_lost())
            .field("bus_busy", &self.bus_busy())
            .field("rxfifo_cnt", &self.rxfifo_cnt())
            .field("txfifo_cnt", &self.txfifo_cnt())
            .field("scl_main_state_last", &self.scl_main_state_last())
            .field("scl_state_last", &self.scl_state_last())
            .finish()
    }
}
#[doc = "Describe I2C work status.\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SR_SPEC {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {}
