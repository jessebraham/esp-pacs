#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `JTAG_IN_FLUSH_INT_ST` reader - The raw interrupt status bit for the USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT interrupt."]
pub type JTAG_IN_FLUSH_INT_ST_R = crate::BitReader;
#[doc = "Field `SOF_INT_ST` reader - The raw interrupt status bit for the USB_SERIAL_JTAG_SOF_INT interrupt."]
pub type SOF_INT_ST_R = crate::BitReader;
#[doc = "Field `SERIAL_OUT_RECV_PKT_INT_ST` reader - The raw interrupt status bit for the USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT interrupt."]
pub type SERIAL_OUT_RECV_PKT_INT_ST_R = crate::BitReader;
#[doc = "Field `SERIAL_IN_EMPTY_INT_ST` reader - The raw interrupt status bit for the USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT interrupt."]
pub type SERIAL_IN_EMPTY_INT_ST_R = crate::BitReader;
#[doc = "Field `PID_ERR_INT_ST` reader - The raw interrupt status bit for the USB_SERIAL_JTAG_PID_ERR_INT interrupt."]
pub type PID_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `CRC5_ERR_INT_ST` reader - The raw interrupt status bit for the USB_SERIAL_JTAG_CRC5_ERR_INT interrupt."]
pub type CRC5_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `CRC16_ERR_INT_ST` reader - The raw interrupt status bit for the USB_SERIAL_JTAG_CRC16_ERR_INT interrupt."]
pub type CRC16_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `STUFF_ERR_INT_ST` reader - The raw interrupt status bit for the USB_SERIAL_JTAG_STUFF_ERR_INT interrupt."]
pub type STUFF_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `IN_TOKEN_REC_IN_EP1_INT_ST` reader - The raw interrupt status bit for the USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT interrupt."]
pub type IN_TOKEN_REC_IN_EP1_INT_ST_R = crate::BitReader;
#[doc = "Field `USB_BUS_RESET_INT_ST` reader - The raw interrupt status bit for the USB_SERIAL_JTAG_USB_BUS_RESET_INT interrupt."]
pub type USB_BUS_RESET_INT_ST_R = crate::BitReader;
#[doc = "Field `OUT_EP1_ZERO_PAYLOAD_INT_ST` reader - The raw interrupt status bit for the USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT interrupt."]
pub type OUT_EP1_ZERO_PAYLOAD_INT_ST_R = crate::BitReader;
#[doc = "Field `OUT_EP2_ZERO_PAYLOAD_INT_ST` reader - The raw interrupt status bit for the USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT interrupt."]
pub type OUT_EP2_ZERO_PAYLOAD_INT_ST_R = crate::BitReader;
#[doc = "Field `RTS_CHG_INT_ST` reader - The raw interrupt status bit for the USB_SERIAL_JTAG_RTS_CHG_INT interrupt."]
pub type RTS_CHG_INT_ST_R = crate::BitReader;
#[doc = "Field `DTR_CHG_INT_ST` reader - The raw interrupt status bit for the USB_SERIAL_JTAG_DTR_CHG_INT interrupt."]
pub type DTR_CHG_INT_ST_R = crate::BitReader;
#[doc = "Field `GET_LINE_CODE_INT_ST` reader - The raw interrupt status bit for the USB_SERIAL_JTAG_GET_LINE_CODE_INT interrupt."]
pub type GET_LINE_CODE_INT_ST_R = crate::BitReader;
#[doc = "Field `SET_LINE_CODE_INT_ST` reader - The raw interrupt status bit for the USB_SERIAL_JTAG_SET_LINE_CODE_INT interrupt."]
pub type SET_LINE_CODE_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The raw interrupt status bit for the USB_SERIAL_JTAG_JTAG_IN_FLUSH_INT interrupt."]
    #[inline(always)]
    pub fn jtag_in_flush_int_st(&self) -> JTAG_IN_FLUSH_INT_ST_R {
        JTAG_IN_FLUSH_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status bit for the USB_SERIAL_JTAG_SOF_INT interrupt."]
    #[inline(always)]
    pub fn sof_int_st(&self) -> SOF_INT_ST_R {
        SOF_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status bit for the USB_SERIAL_JTAG_SERIAL_OUT_RECV_PKT_INT interrupt."]
    #[inline(always)]
    pub fn serial_out_recv_pkt_int_st(&self) -> SERIAL_OUT_RECV_PKT_INT_ST_R {
        SERIAL_OUT_RECV_PKT_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt status bit for the USB_SERIAL_JTAG_SERIAL_IN_EMPTY_INT interrupt."]
    #[inline(always)]
    pub fn serial_in_empty_int_st(&self) -> SERIAL_IN_EMPTY_INT_ST_R {
        SERIAL_IN_EMPTY_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt status bit for the USB_SERIAL_JTAG_PID_ERR_INT interrupt."]
    #[inline(always)]
    pub fn pid_err_int_st(&self) -> PID_ERR_INT_ST_R {
        PID_ERR_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt status bit for the USB_SERIAL_JTAG_CRC5_ERR_INT interrupt."]
    #[inline(always)]
    pub fn crc5_err_int_st(&self) -> CRC5_ERR_INT_ST_R {
        CRC5_ERR_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt status bit for the USB_SERIAL_JTAG_CRC16_ERR_INT interrupt."]
    #[inline(always)]
    pub fn crc16_err_int_st(&self) -> CRC16_ERR_INT_ST_R {
        CRC16_ERR_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw interrupt status bit for the USB_SERIAL_JTAG_STUFF_ERR_INT interrupt."]
    #[inline(always)]
    pub fn stuff_err_int_st(&self) -> STUFF_ERR_INT_ST_R {
        STUFF_ERR_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw interrupt status bit for the USB_SERIAL_JTAG_IN_TOKEN_REC_IN_EP1_INT interrupt."]
    #[inline(always)]
    pub fn in_token_rec_in_ep1_int_st(&self) -> IN_TOKEN_REC_IN_EP1_INT_ST_R {
        IN_TOKEN_REC_IN_EP1_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The raw interrupt status bit for the USB_SERIAL_JTAG_USB_BUS_RESET_INT interrupt."]
    #[inline(always)]
    pub fn usb_bus_reset_int_st(&self) -> USB_BUS_RESET_INT_ST_R {
        USB_BUS_RESET_INT_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The raw interrupt status bit for the USB_SERIAL_JTAG_OUT_EP1_ZERO_PAYLOAD_INT interrupt."]
    #[inline(always)]
    pub fn out_ep1_zero_payload_int_st(&self) -> OUT_EP1_ZERO_PAYLOAD_INT_ST_R {
        OUT_EP1_ZERO_PAYLOAD_INT_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The raw interrupt status bit for the USB_SERIAL_JTAG_OUT_EP2_ZERO_PAYLOAD_INT interrupt."]
    #[inline(always)]
    pub fn out_ep2_zero_payload_int_st(&self) -> OUT_EP2_ZERO_PAYLOAD_INT_ST_R {
        OUT_EP2_ZERO_PAYLOAD_INT_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The raw interrupt status bit for the USB_SERIAL_JTAG_RTS_CHG_INT interrupt."]
    #[inline(always)]
    pub fn rts_chg_int_st(&self) -> RTS_CHG_INT_ST_R {
        RTS_CHG_INT_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The raw interrupt status bit for the USB_SERIAL_JTAG_DTR_CHG_INT interrupt."]
    #[inline(always)]
    pub fn dtr_chg_int_st(&self) -> DTR_CHG_INT_ST_R {
        DTR_CHG_INT_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The raw interrupt status bit for the USB_SERIAL_JTAG_GET_LINE_CODE_INT interrupt."]
    #[inline(always)]
    pub fn get_line_code_int_st(&self) -> GET_LINE_CODE_INT_ST_R {
        GET_LINE_CODE_INT_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The raw interrupt status bit for the USB_SERIAL_JTAG_SET_LINE_CODE_INT interrupt."]
    #[inline(always)]
    pub fn set_line_code_int_st(&self) -> SET_LINE_CODE_INT_ST_R {
        SET_LINE_CODE_INT_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("jtag_in_flush_int_st", &self.jtag_in_flush_int_st())
            .field("sof_int_st", &self.sof_int_st())
            .field(
                "serial_out_recv_pkt_int_st",
                &self.serial_out_recv_pkt_int_st(),
            )
            .field("serial_in_empty_int_st", &self.serial_in_empty_int_st())
            .field("pid_err_int_st", &self.pid_err_int_st())
            .field("crc5_err_int_st", &self.crc5_err_int_st())
            .field("crc16_err_int_st", &self.crc16_err_int_st())
            .field("stuff_err_int_st", &self.stuff_err_int_st())
            .field(
                "in_token_rec_in_ep1_int_st",
                &self.in_token_rec_in_ep1_int_st(),
            )
            .field("usb_bus_reset_int_st", &self.usb_bus_reset_int_st())
            .field(
                "out_ep1_zero_payload_int_st",
                &self.out_ep1_zero_payload_int_st(),
            )
            .field(
                "out_ep2_zero_payload_int_st",
                &self.out_ep2_zero_payload_int_st(),
            )
            .field("rts_chg_int_st", &self.rts_chg_int_st())
            .field("dtr_chg_int_st", &self.dtr_chg_int_st())
            .field("get_line_code_int_st", &self.get_line_code_int_st())
            .field("set_line_code_int_st", &self.set_line_code_int_st())
            .finish()
    }
}
#[doc = "Interrupt status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {}
