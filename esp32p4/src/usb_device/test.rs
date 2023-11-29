#[doc = "Register `TEST` reader"]
pub type R = crate::R<TEST_SPEC>;
#[doc = "Register `TEST` writer"]
pub type W = crate::W<TEST_SPEC>;
#[doc = "Field `USB_SERIAL_JTAG_TEST_ENABLE` reader - Enable test of the USB pad"]
pub type USB_SERIAL_JTAG_TEST_ENABLE_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_TEST_ENABLE` writer - Enable test of the USB pad"]
pub type USB_SERIAL_JTAG_TEST_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_TEST_USB_OE` reader - USB pad oen in test"]
pub type USB_SERIAL_JTAG_TEST_USB_OE_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_TEST_USB_OE` writer - USB pad oen in test"]
pub type USB_SERIAL_JTAG_TEST_USB_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_TEST_TX_DP` reader - USB D+ tx value in test"]
pub type USB_SERIAL_JTAG_TEST_TX_DP_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_TEST_TX_DP` writer - USB D+ tx value in test"]
pub type USB_SERIAL_JTAG_TEST_TX_DP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_TEST_TX_DM` reader - USB D- tx value in test"]
pub type USB_SERIAL_JTAG_TEST_TX_DM_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_TEST_TX_DM` writer - USB D- tx value in test"]
pub type USB_SERIAL_JTAG_TEST_TX_DM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_TEST_RX_RCV` reader - USB RCV value in test"]
pub type USB_SERIAL_JTAG_TEST_RX_RCV_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_TEST_RX_DP` reader - USB D+ rx value in test"]
pub type USB_SERIAL_JTAG_TEST_RX_DP_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_TEST_RX_DM` reader - USB D- rx value in test"]
pub type USB_SERIAL_JTAG_TEST_RX_DM_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Enable test of the USB pad"]
    #[inline(always)]
    pub fn usb_serial_jtag_test_enable(&self) -> USB_SERIAL_JTAG_TEST_ENABLE_R {
        USB_SERIAL_JTAG_TEST_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB pad oen in test"]
    #[inline(always)]
    pub fn usb_serial_jtag_test_usb_oe(&self) -> USB_SERIAL_JTAG_TEST_USB_OE_R {
        USB_SERIAL_JTAG_TEST_USB_OE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB D+ tx value in test"]
    #[inline(always)]
    pub fn usb_serial_jtag_test_tx_dp(&self) -> USB_SERIAL_JTAG_TEST_TX_DP_R {
        USB_SERIAL_JTAG_TEST_TX_DP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USB D- tx value in test"]
    #[inline(always)]
    pub fn usb_serial_jtag_test_tx_dm(&self) -> USB_SERIAL_JTAG_TEST_TX_DM_R {
        USB_SERIAL_JTAG_TEST_TX_DM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USB RCV value in test"]
    #[inline(always)]
    pub fn usb_serial_jtag_test_rx_rcv(&self) -> USB_SERIAL_JTAG_TEST_RX_RCV_R {
        USB_SERIAL_JTAG_TEST_RX_RCV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USB D+ rx value in test"]
    #[inline(always)]
    pub fn usb_serial_jtag_test_rx_dp(&self) -> USB_SERIAL_JTAG_TEST_RX_DP_R {
        USB_SERIAL_JTAG_TEST_RX_DP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USB D- rx value in test"]
    #[inline(always)]
    pub fn usb_serial_jtag_test_rx_dm(&self) -> USB_SERIAL_JTAG_TEST_RX_DM_R {
        USB_SERIAL_JTAG_TEST_RX_DM_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEST")
            .field(
                "usb_serial_jtag_test_enable",
                &format_args!("{}", self.usb_serial_jtag_test_enable().bit()),
            )
            .field(
                "usb_serial_jtag_test_usb_oe",
                &format_args!("{}", self.usb_serial_jtag_test_usb_oe().bit()),
            )
            .field(
                "usb_serial_jtag_test_tx_dp",
                &format_args!("{}", self.usb_serial_jtag_test_tx_dp().bit()),
            )
            .field(
                "usb_serial_jtag_test_tx_dm",
                &format_args!("{}", self.usb_serial_jtag_test_tx_dm().bit()),
            )
            .field(
                "usb_serial_jtag_test_rx_rcv",
                &format_args!("{}", self.usb_serial_jtag_test_rx_rcv().bit()),
            )
            .field(
                "usb_serial_jtag_test_rx_dp",
                &format_args!("{}", self.usb_serial_jtag_test_rx_dp().bit()),
            )
            .field(
                "usb_serial_jtag_test_rx_dm",
                &format_args!("{}", self.usb_serial_jtag_test_rx_dm().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TEST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Enable test of the USB pad"]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_test_enable(&mut self) -> USB_SERIAL_JTAG_TEST_ENABLE_W<TEST_SPEC> {
        USB_SERIAL_JTAG_TEST_ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - USB pad oen in test"]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_test_usb_oe(&mut self) -> USB_SERIAL_JTAG_TEST_USB_OE_W<TEST_SPEC> {
        USB_SERIAL_JTAG_TEST_USB_OE_W::new(self, 1)
    }
    #[doc = "Bit 2 - USB D+ tx value in test"]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_test_tx_dp(&mut self) -> USB_SERIAL_JTAG_TEST_TX_DP_W<TEST_SPEC> {
        USB_SERIAL_JTAG_TEST_TX_DP_W::new(self, 2)
    }
    #[doc = "Bit 3 - USB D- tx value in test"]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_test_tx_dm(&mut self) -> USB_SERIAL_JTAG_TEST_TX_DM_W<TEST_SPEC> {
        USB_SERIAL_JTAG_TEST_TX_DM_W::new(self, 3)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Registers used for debugging the PHY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TEST_SPEC;
impl crate::RegisterSpec for TEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test::R`](R) reader structure"]
impl crate::Readable for TEST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`test::W`](W) writer structure"]
impl crate::Writable for TEST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEST to value 0x30"]
impl crate::Resettable for TEST_SPEC {
    const RESET_VALUE: Self::Ux = 0x30;
}