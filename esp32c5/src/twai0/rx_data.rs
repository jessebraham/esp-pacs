#[doc = "Register `RX_DATA` reader"]
pub type R = crate::R<RX_DATA_SPEC>;
#[doc = "Field `RX_DATA` reader - RX buffer data at read pointer position in FIFO. By reading from this register, read pointer is auto- matically increased, as long as there is next data word stored in RX buffer. First stored word in the buffer is FRAME_FORMAT_W, next IDENTIFIER_W etc. This register shall be read by 32 bit access."]
pub type RX_DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RX buffer data at read pointer position in FIFO. By reading from this register, read pointer is auto- matically increased, as long as there is next data word stored in RX buffer. First stored word in the buffer is FRAME_FORMAT_W, next IDENTIFIER_W etc. This register shall be read by 32 bit access."]
    #[inline(always)]
    pub fn rx_data(&self) -> RX_DATA_R {
        RX_DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_DATA")
            .field("rx_data", &self.rx_data())
            .finish()
    }
}
#[doc = "TWAI FD received data register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_DATA_SPEC;
impl crate::RegisterSpec for RX_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_data::R`](R) reader structure"]
impl crate::Readable for RX_DATA_SPEC {}
#[doc = "`reset()` method sets RX_DATA to value 0"]
impl crate::Resettable for RX_DATA_SPEC {}
