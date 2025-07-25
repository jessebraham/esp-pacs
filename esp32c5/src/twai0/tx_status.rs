#[doc = "Register `TX_STATUS` reader"]
pub type R = crate::R<TX_STATUS_SPEC>;
#[doc = "Field `TXTB0_STATE` reader - Status of TXT buffer 1. 0b0000 - TXT_NOT_EXIST - TXT buffer does not exist in the core (applies only to TXT buffers 3-8, when CTU CAN FD was synthesized with less than 8 TXT buffers). 0b0001 - TXT_RDY - TXT buffer is in \"Ready\" state, it is waiting for CTU CAN FD to start transmission from it. 0b0010 - TXT_TRAN - TXT buffer is in \"TX in progress\" state. CTU CAN FD is transmitting frame. 0b0011 - TXT_ABTP - TXT buffer is in \"Abort in progress\" state. 0b0100 - TXT_TOK - TXT buffer is in \"TX OK\" state. 0b0110 - TXT_ERR - TXT buffer is in \"Failed\" state. 0b0111 - TXT_ABT - TXT buffer is in \"Aborted\" state. 0b1000 - TXT_ETY - TXT buffer is in \"Empty\" state."]
pub type TXTB0_STATE_R = crate::FieldReader;
#[doc = "Field `TX2S` reader - Status of TXT buffer 2. Bit field meaning is analogous to TX1S."]
pub type TX2S_R = crate::FieldReader;
#[doc = "Field `TX3S` reader - Status of TXT buffer 3. Bit field meaning is analogous to TX1S."]
pub type TX3S_R = crate::FieldReader;
#[doc = "Field `TX4S` reader - Status of TXT buffer 4. Bit field meaning is analogous to TX1S."]
pub type TX4S_R = crate::FieldReader;
#[doc = "Field `TX5S` reader - Status of TXT buffer 5. Bit field meaning is analogous to TX1S."]
pub type TX5S_R = crate::FieldReader;
#[doc = "Field `TX6S` reader - Status of TXT buffer 6. Bit field meaning is analogous to TX1S."]
pub type TX6S_R = crate::FieldReader;
#[doc = "Field `TX7S` reader - Status of TXT buffer 7. Bit field meaning is analogous to TX1S."]
pub type TX7S_R = crate::FieldReader;
#[doc = "Field `TX8S` reader - Status of TXT buffer 8. Bit field meaning is analogous to TX1S."]
pub type TX8S_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Status of TXT buffer 1. 0b0000 - TXT_NOT_EXIST - TXT buffer does not exist in the core (applies only to TXT buffers 3-8, when CTU CAN FD was synthesized with less than 8 TXT buffers). 0b0001 - TXT_RDY - TXT buffer is in \"Ready\" state, it is waiting for CTU CAN FD to start transmission from it. 0b0010 - TXT_TRAN - TXT buffer is in \"TX in progress\" state. CTU CAN FD is transmitting frame. 0b0011 - TXT_ABTP - TXT buffer is in \"Abort in progress\" state. 0b0100 - TXT_TOK - TXT buffer is in \"TX OK\" state. 0b0110 - TXT_ERR - TXT buffer is in \"Failed\" state. 0b0111 - TXT_ABT - TXT buffer is in \"Aborted\" state. 0b1000 - TXT_ETY - TXT buffer is in \"Empty\" state."]
    #[inline(always)]
    pub fn txtb0_state(&self) -> TXTB0_STATE_R {
        TXTB0_STATE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Status of TXT buffer 2. Bit field meaning is analogous to TX1S."]
    #[inline(always)]
    pub fn tx2s(&self) -> TX2S_R {
        TX2S_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Status of TXT buffer 3. Bit field meaning is analogous to TX1S."]
    #[inline(always)]
    pub fn tx3s(&self) -> TX3S_R {
        TX3S_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Status of TXT buffer 4. Bit field meaning is analogous to TX1S."]
    #[inline(always)]
    pub fn tx4s(&self) -> TX4S_R {
        TX4S_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Status of TXT buffer 5. Bit field meaning is analogous to TX1S."]
    #[inline(always)]
    pub fn tx5s(&self) -> TX5S_R {
        TX5S_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Status of TXT buffer 6. Bit field meaning is analogous to TX1S."]
    #[inline(always)]
    pub fn tx6s(&self) -> TX6S_R {
        TX6S_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Status of TXT buffer 7. Bit field meaning is analogous to TX1S."]
    #[inline(always)]
    pub fn tx7s(&self) -> TX7S_R {
        TX7S_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Status of TXT buffer 8. Bit field meaning is analogous to TX1S."]
    #[inline(always)]
    pub fn tx8s(&self) -> TX8S_R {
        TX8S_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_STATUS")
            .field("txtb0_state", &self.txtb0_state())
            .field("tx2s", &self.tx2s())
            .field("tx3s", &self.tx3s())
            .field("tx4s", &self.tx4s())
            .field("tx5s", &self.tx5s())
            .field("tx6s", &self.tx6s())
            .field("tx7s", &self.tx7s())
            .field("tx8s", &self.tx8s())
            .finish()
    }
}
#[doc = "TWAI FD TX buffer status register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_STATUS_SPEC;
impl crate::RegisterSpec for TX_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_status::R`](R) reader structure"]
impl crate::Readable for TX_STATUS_SPEC {}
#[doc = "`reset()` method sets TX_STATUS to value 0x8888"]
impl crate::Resettable for TX_STATUS_SPEC {
    const RESET_VALUE: u32 = 0x8888;
}
