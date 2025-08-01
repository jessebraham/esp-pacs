#[doc = "Register `SWFC_CONF0_SYNC` reader"]
pub type R = crate::R<SWFC_CONF0_SYNC_SPEC>;
#[doc = "Register `SWFC_CONF0_SYNC` writer"]
pub type W = crate::W<SWFC_CONF0_SYNC_SPEC>;
#[doc = "Field `XON_CHAR` reader - Configures the XON character for flow control."]
pub type XON_CHAR_R = crate::FieldReader;
#[doc = "Field `XON_CHAR` writer - Configures the XON character for flow control."]
pub type XON_CHAR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `XOFF_CHAR` reader - Configures the XOFF character for flow control."]
pub type XOFF_CHAR_R = crate::FieldReader;
#[doc = "Field `XOFF_CHAR` writer - Configures the XOFF character for flow control."]
pub type XOFF_CHAR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `XON_XOFF_STILL_SEND` reader - Configures whether the UART transmitter can send XON or XOFF characters when it is disabled.\\\\ 0: Cannot send\\\\ 1: Can send\\\\"]
pub type XON_XOFF_STILL_SEND_R = crate::BitReader;
#[doc = "Field `XON_XOFF_STILL_SEND` writer - Configures whether the UART transmitter can send XON or XOFF characters when it is disabled.\\\\ 0: Cannot send\\\\ 1: Can send\\\\"]
pub type XON_XOFF_STILL_SEND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_FLOW_CON_EN` reader - Configures whether or not to enable software flow control.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type SW_FLOW_CON_EN_R = crate::BitReader;
#[doc = "Field `SW_FLOW_CON_EN` writer - Configures whether or not to enable software flow control.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type SW_FLOW_CON_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XONOFF_DEL` reader - Configures whether or not to remove flow control characters from the received data.\\\\ 0: Not move\\\\ 1: Move\\\\"]
pub type XONOFF_DEL_R = crate::BitReader;
#[doc = "Field `XONOFF_DEL` writer - Configures whether or not to remove flow control characters from the received data.\\\\ 0: Not move\\\\ 1: Move\\\\"]
pub type XONOFF_DEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_XON` reader - Configures whether the transmitter continues to sending data.\\\\ 0: Not send\\\\ 1: Send\\\\"]
pub type FORCE_XON_R = crate::BitReader;
#[doc = "Field `FORCE_XON` writer - Configures whether the transmitter continues to sending data.\\\\ 0: Not send\\\\ 1: Send\\\\"]
pub type FORCE_XON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_XOFF` reader - Configures whether or not to stop the transmitter from sending data.\\\\ 0: Not stop\\\\ 1: Stop\\\\"]
pub type FORCE_XOFF_R = crate::BitReader;
#[doc = "Field `FORCE_XOFF` writer - Configures whether or not to stop the transmitter from sending data.\\\\ 0: Not stop\\\\ 1: Stop\\\\"]
pub type FORCE_XOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEND_XON` reader - Configures whether or not to send XON characters.\\\\ 0: Not send\\\\ 1: Send\\\\"]
pub type SEND_XON_R = crate::BitReader;
#[doc = "Field `SEND_XON` writer - Configures whether or not to send XON characters.\\\\ 0: Not send\\\\ 1: Send\\\\"]
pub type SEND_XON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEND_XOFF` reader - Configures whether or not to send XOFF characters.\\\\ 0: Not send\\\\ 1: Send\\\\"]
pub type SEND_XOFF_R = crate::BitReader;
#[doc = "Field `SEND_XOFF` writer - Configures whether or not to send XOFF characters.\\\\ 0: Not send\\\\ 1: Send\\\\"]
pub type SEND_XOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Configures the XON character for flow control."]
    #[inline(always)]
    pub fn xon_char(&self) -> XON_CHAR_R {
        XON_CHAR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Configures the XOFF character for flow control."]
    #[inline(always)]
    pub fn xoff_char(&self) -> XOFF_CHAR_R {
        XOFF_CHAR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Configures whether the UART transmitter can send XON or XOFF characters when it is disabled.\\\\ 0: Cannot send\\\\ 1: Can send\\\\"]
    #[inline(always)]
    pub fn xon_xoff_still_send(&self) -> XON_XOFF_STILL_SEND_R {
        XON_XOFF_STILL_SEND_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Configures whether or not to enable software flow control.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn sw_flow_con_en(&self) -> SW_FLOW_CON_EN_R {
        SW_FLOW_CON_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Configures whether or not to remove flow control characters from the received data.\\\\ 0: Not move\\\\ 1: Move\\\\"]
    #[inline(always)]
    pub fn xonoff_del(&self) -> XONOFF_DEL_R {
        XONOFF_DEL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Configures whether the transmitter continues to sending data.\\\\ 0: Not send\\\\ 1: Send\\\\"]
    #[inline(always)]
    pub fn force_xon(&self) -> FORCE_XON_R {
        FORCE_XON_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Configures whether or not to stop the transmitter from sending data.\\\\ 0: Not stop\\\\ 1: Stop\\\\"]
    #[inline(always)]
    pub fn force_xoff(&self) -> FORCE_XOFF_R {
        FORCE_XOFF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Configures whether or not to send XON characters.\\\\ 0: Not send\\\\ 1: Send\\\\"]
    #[inline(always)]
    pub fn send_xon(&self) -> SEND_XON_R {
        SEND_XON_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Configures whether or not to send XOFF characters.\\\\ 0: Not send\\\\ 1: Send\\\\"]
    #[inline(always)]
    pub fn send_xoff(&self) -> SEND_XOFF_R {
        SEND_XOFF_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWFC_CONF0_SYNC")
            .field("xon_char", &self.xon_char())
            .field("xoff_char", &self.xoff_char())
            .field("xon_xoff_still_send", &self.xon_xoff_still_send())
            .field("sw_flow_con_en", &self.sw_flow_con_en())
            .field("xonoff_del", &self.xonoff_del())
            .field("force_xon", &self.force_xon())
            .field("force_xoff", &self.force_xoff())
            .field("send_xon", &self.send_xon())
            .field("send_xoff", &self.send_xoff())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Configures the XON character for flow control."]
    #[inline(always)]
    pub fn xon_char(&mut self) -> XON_CHAR_W<SWFC_CONF0_SYNC_SPEC> {
        XON_CHAR_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Configures the XOFF character for flow control."]
    #[inline(always)]
    pub fn xoff_char(&mut self) -> XOFF_CHAR_W<SWFC_CONF0_SYNC_SPEC> {
        XOFF_CHAR_W::new(self, 8)
    }
    #[doc = "Bit 16 - Configures whether the UART transmitter can send XON or XOFF characters when it is disabled.\\\\ 0: Cannot send\\\\ 1: Can send\\\\"]
    #[inline(always)]
    pub fn xon_xoff_still_send(&mut self) -> XON_XOFF_STILL_SEND_W<SWFC_CONF0_SYNC_SPEC> {
        XON_XOFF_STILL_SEND_W::new(self, 16)
    }
    #[doc = "Bit 17 - Configures whether or not to enable software flow control.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn sw_flow_con_en(&mut self) -> SW_FLOW_CON_EN_W<SWFC_CONF0_SYNC_SPEC> {
        SW_FLOW_CON_EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - Configures whether or not to remove flow control characters from the received data.\\\\ 0: Not move\\\\ 1: Move\\\\"]
    #[inline(always)]
    pub fn xonoff_del(&mut self) -> XONOFF_DEL_W<SWFC_CONF0_SYNC_SPEC> {
        XONOFF_DEL_W::new(self, 18)
    }
    #[doc = "Bit 19 - Configures whether the transmitter continues to sending data.\\\\ 0: Not send\\\\ 1: Send\\\\"]
    #[inline(always)]
    pub fn force_xon(&mut self) -> FORCE_XON_W<SWFC_CONF0_SYNC_SPEC> {
        FORCE_XON_W::new(self, 19)
    }
    #[doc = "Bit 20 - Configures whether or not to stop the transmitter from sending data.\\\\ 0: Not stop\\\\ 1: Stop\\\\"]
    #[inline(always)]
    pub fn force_xoff(&mut self) -> FORCE_XOFF_W<SWFC_CONF0_SYNC_SPEC> {
        FORCE_XOFF_W::new(self, 20)
    }
    #[doc = "Bit 21 - Configures whether or not to send XON characters.\\\\ 0: Not send\\\\ 1: Send\\\\"]
    #[inline(always)]
    pub fn send_xon(&mut self) -> SEND_XON_W<SWFC_CONF0_SYNC_SPEC> {
        SEND_XON_W::new(self, 21)
    }
    #[doc = "Bit 22 - Configures whether or not to send XOFF characters.\\\\ 0: Not send\\\\ 1: Send\\\\"]
    #[inline(always)]
    pub fn send_xoff(&mut self) -> SEND_XOFF_W<SWFC_CONF0_SYNC_SPEC> {
        SEND_XOFF_W::new(self, 22)
    }
}
#[doc = "Software flow control character configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`swfc_conf0_sync::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swfc_conf0_sync::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWFC_CONF0_SYNC_SPEC;
impl crate::RegisterSpec for SWFC_CONF0_SYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swfc_conf0_sync::R`](R) reader structure"]
impl crate::Readable for SWFC_CONF0_SYNC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`swfc_conf0_sync::W`](W) writer structure"]
impl crate::Writable for SWFC_CONF0_SYNC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWFC_CONF0_SYNC to value 0x1311"]
impl crate::Resettable for SWFC_CONF0_SYNC_SPEC {
    const RESET_VALUE: u32 = 0x1311;
}
