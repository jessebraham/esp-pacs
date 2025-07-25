#[doc = "Register `PERIP_CLK_EN` reader"]
pub type R = crate::R<PERIP_CLK_EN_SPEC>;
#[doc = "Register `PERIP_CLK_EN` writer"]
pub type W = crate::W<PERIP_CLK_EN_SPEC>;
#[doc = "Field `TIMERS_CLK_EN` reader - "]
pub type TIMERS_CLK_EN_R = crate::BitReader;
#[doc = "Field `TIMERS_CLK_EN` writer - "]
pub type TIMERS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI01_CLK_EN` reader - "]
pub type SPI01_CLK_EN_R = crate::BitReader;
#[doc = "Field `SPI01_CLK_EN` writer - "]
pub type SPI01_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_CLK_EN` reader - "]
pub type UART_CLK_EN_R = crate::BitReader;
#[doc = "Field `UART_CLK_EN` writer - "]
pub type UART_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDG_CLK_EN` reader - "]
pub type WDG_CLK_EN_R = crate::BitReader;
#[doc = "Field `WDG_CLK_EN` writer - "]
pub type WDG_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S0_CLK_EN` reader - "]
pub type I2S0_CLK_EN_R = crate::BitReader;
#[doc = "Field `I2S0_CLK_EN` writer - "]
pub type I2S0_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1_CLK_EN` reader - "]
pub type UART1_CLK_EN_R = crate::BitReader;
#[doc = "Field `UART1_CLK_EN` writer - "]
pub type UART1_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2_CLK_EN` reader - "]
pub type SPI2_CLK_EN_R = crate::BitReader;
#[doc = "Field `SPI2_CLK_EN` writer - "]
pub type SPI2_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_EXT0_CLK_EN` reader - "]
pub type I2C_EXT0_CLK_EN_R = crate::BitReader;
#[doc = "Field `I2C_EXT0_CLK_EN` writer - "]
pub type I2C_EXT0_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UHCI0_CLK_EN` reader - "]
pub type UHCI0_CLK_EN_R = crate::BitReader;
#[doc = "Field `UHCI0_CLK_EN` writer - "]
pub type UHCI0_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMT_CLK_EN` reader - "]
pub type RMT_CLK_EN_R = crate::BitReader;
#[doc = "Field `RMT_CLK_EN` writer - "]
pub type RMT_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCNT_CLK_EN` reader - "]
pub type PCNT_CLK_EN_R = crate::BitReader;
#[doc = "Field `PCNT_CLK_EN` writer - "]
pub type PCNT_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC_CLK_EN` reader - "]
pub type LEDC_CLK_EN_R = crate::BitReader;
#[doc = "Field `LEDC_CLK_EN` writer - "]
pub type LEDC_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UHCI1_CLK_EN` reader - "]
pub type UHCI1_CLK_EN_R = crate::BitReader;
#[doc = "Field `UHCI1_CLK_EN` writer - "]
pub type UHCI1_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMERGROUP_CLK_EN` reader - "]
pub type TIMERGROUP_CLK_EN_R = crate::BitReader;
#[doc = "Field `TIMERGROUP_CLK_EN` writer - "]
pub type TIMERGROUP_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFUSE_CLK_EN` reader - "]
pub type EFUSE_CLK_EN_R = crate::BitReader;
#[doc = "Field `EFUSE_CLK_EN` writer - "]
pub type EFUSE_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMERGROUP1_CLK_EN` reader - "]
pub type TIMERGROUP1_CLK_EN_R = crate::BitReader;
#[doc = "Field `TIMERGROUP1_CLK_EN` writer - "]
pub type TIMERGROUP1_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3_CLK_EN` reader - "]
pub type SPI3_CLK_EN_R = crate::BitReader;
#[doc = "Field `SPI3_CLK_EN` writer - "]
pub type SPI3_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM0_CLK_EN` reader - "]
pub type PWM0_CLK_EN_R = crate::BitReader;
#[doc = "Field `PWM0_CLK_EN` writer - "]
pub type PWM0_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_EXT1_CLK_EN` reader - "]
pub type I2C_EXT1_CLK_EN_R = crate::BitReader;
#[doc = "Field `I2C_EXT1_CLK_EN` writer - "]
pub type I2C_EXT1_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWAI_CLK_EN` reader - "]
pub type TWAI_CLK_EN_R = crate::BitReader;
#[doc = "Field `TWAI_CLK_EN` writer - "]
pub type TWAI_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM1_CLK_EN` reader - "]
pub type PWM1_CLK_EN_R = crate::BitReader;
#[doc = "Field `PWM1_CLK_EN` writer - "]
pub type PWM1_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S1_CLK_EN` reader - "]
pub type I2S1_CLK_EN_R = crate::BitReader;
#[doc = "Field `I2S1_CLK_EN` writer - "]
pub type I2S1_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_DMA_CLK_EN` reader - "]
pub type SPI_DMA_CLK_EN_R = crate::BitReader;
#[doc = "Field `SPI_DMA_CLK_EN` writer - "]
pub type SPI_DMA_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART2_CLK_EN` reader - "]
pub type UART2_CLK_EN_R = crate::BitReader;
#[doc = "Field `UART2_CLK_EN` writer - "]
pub type UART2_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_MEM_CLK_EN` reader - "]
pub type UART_MEM_CLK_EN_R = crate::BitReader;
#[doc = "Field `UART_MEM_CLK_EN` writer - "]
pub type UART_MEM_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM2_CLK_EN` reader - "]
pub type PWM2_CLK_EN_R = crate::BitReader;
#[doc = "Field `PWM2_CLK_EN` writer - "]
pub type PWM2_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM3_CLK_EN` reader - "]
pub type PWM3_CLK_EN_R = crate::BitReader;
#[doc = "Field `PWM3_CLK_EN` writer - "]
pub type PWM3_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timers_clk_en(&self) -> TIMERS_CLK_EN_R {
        TIMERS_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi01_clk_en(&self) -> SPI01_CLK_EN_R {
        SPI01_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn uart_clk_en(&self) -> UART_CLK_EN_R {
        UART_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn wdg_clk_en(&self) -> WDG_CLK_EN_R {
        WDG_CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn i2s0_clk_en(&self) -> I2S0_CLK_EN_R {
        I2S0_CLK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn uart1_clk_en(&self) -> UART1_CLK_EN_R {
        UART1_CLK_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi2_clk_en(&self) -> SPI2_CLK_EN_R {
        SPI2_CLK_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn i2c_ext0_clk_en(&self) -> I2C_EXT0_CLK_EN_R {
        I2C_EXT0_CLK_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn uhci0_clk_en(&self) -> UHCI0_CLK_EN_R {
        UHCI0_CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rmt_clk_en(&self) -> RMT_CLK_EN_R {
        RMT_CLK_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pcnt_clk_en(&self) -> PCNT_CLK_EN_R {
        PCNT_CLK_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ledc_clk_en(&self) -> LEDC_CLK_EN_R {
        LEDC_CLK_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn uhci1_clk_en(&self) -> UHCI1_CLK_EN_R {
        UHCI1_CLK_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn timergroup_clk_en(&self) -> TIMERGROUP_CLK_EN_R {
        TIMERGROUP_CLK_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn efuse_clk_en(&self) -> EFUSE_CLK_EN_R {
        EFUSE_CLK_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn timergroup1_clk_en(&self) -> TIMERGROUP1_CLK_EN_R {
        TIMERGROUP1_CLK_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn spi3_clk_en(&self) -> SPI3_CLK_EN_R {
        SPI3_CLK_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pwm0_clk_en(&self) -> PWM0_CLK_EN_R {
        PWM0_CLK_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn i2c_ext1_clk_en(&self) -> I2C_EXT1_CLK_EN_R {
        I2C_EXT1_CLK_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn twai_clk_en(&self) -> TWAI_CLK_EN_R {
        TWAI_CLK_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pwm1_clk_en(&self) -> PWM1_CLK_EN_R {
        PWM1_CLK_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn i2s1_clk_en(&self) -> I2S1_CLK_EN_R {
        I2S1_CLK_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn spi_dma_clk_en(&self) -> SPI_DMA_CLK_EN_R {
        SPI_DMA_CLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn uart2_clk_en(&self) -> UART2_CLK_EN_R {
        UART2_CLK_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn uart_mem_clk_en(&self) -> UART_MEM_CLK_EN_R {
        UART_MEM_CLK_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pwm2_clk_en(&self) -> PWM2_CLK_EN_R {
        PWM2_CLK_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn pwm3_clk_en(&self) -> PWM3_CLK_EN_R {
        PWM3_CLK_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERIP_CLK_EN")
            .field("timers_clk_en", &self.timers_clk_en())
            .field("spi01_clk_en", &self.spi01_clk_en())
            .field("uart_clk_en", &self.uart_clk_en())
            .field("wdg_clk_en", &self.wdg_clk_en())
            .field("i2s0_clk_en", &self.i2s0_clk_en())
            .field("uart1_clk_en", &self.uart1_clk_en())
            .field("spi2_clk_en", &self.spi2_clk_en())
            .field("i2c_ext0_clk_en", &self.i2c_ext0_clk_en())
            .field("uhci0_clk_en", &self.uhci0_clk_en())
            .field("rmt_clk_en", &self.rmt_clk_en())
            .field("pcnt_clk_en", &self.pcnt_clk_en())
            .field("ledc_clk_en", &self.ledc_clk_en())
            .field("uhci1_clk_en", &self.uhci1_clk_en())
            .field("timergroup_clk_en", &self.timergroup_clk_en())
            .field("efuse_clk_en", &self.efuse_clk_en())
            .field("timergroup1_clk_en", &self.timergroup1_clk_en())
            .field("spi3_clk_en", &self.spi3_clk_en())
            .field("pwm0_clk_en", &self.pwm0_clk_en())
            .field("i2c_ext1_clk_en", &self.i2c_ext1_clk_en())
            .field("twai_clk_en", &self.twai_clk_en())
            .field("pwm1_clk_en", &self.pwm1_clk_en())
            .field("i2s1_clk_en", &self.i2s1_clk_en())
            .field("spi_dma_clk_en", &self.spi_dma_clk_en())
            .field("uart2_clk_en", &self.uart2_clk_en())
            .field("uart_mem_clk_en", &self.uart_mem_clk_en())
            .field("pwm2_clk_en", &self.pwm2_clk_en())
            .field("pwm3_clk_en", &self.pwm3_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timers_clk_en(&mut self) -> TIMERS_CLK_EN_W<PERIP_CLK_EN_SPEC> {
        TIMERS_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi01_clk_en(&mut self) -> SPI01_CLK_EN_W<PERIP_CLK_EN_SPEC> {
        SPI01_CLK_EN_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn uart_clk_en(&mut self) -> UART_CLK_EN_W<PERIP_CLK_EN_SPEC> {
        UART_CLK_EN_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn wdg_clk_en(&mut self) -> WDG_CLK_EN_W<PERIP_CLK_EN_SPEC> {
        WDG_CLK_EN_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn i2s0_clk_en(&mut self) -> I2S0_CLK_EN_W<PERIP_CLK_EN_SPEC> {
        I2S0_CLK_EN_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn uart1_clk_en(&mut self) -> UART1_CLK_EN_W<PERIP_CLK_EN_SPEC> {
        UART1_CLK_EN_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi2_clk_en(&mut self) -> SPI2_CLK_EN_W<PERIP_CLK_EN_SPEC> {
        SPI2_CLK_EN_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn i2c_ext0_clk_en(&mut self) -> I2C_EXT0_CLK_EN_W<PERIP_CLK_EN_SPEC> {
        I2C_EXT0_CLK_EN_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn uhci0_clk_en(&mut self) -> UHCI0_CLK_EN_W<PERIP_CLK_EN_SPEC> {
        UHCI0_CLK_EN_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rmt_clk_en(&mut self) -> RMT_CLK_EN_W<PERIP_CLK_EN_SPEC> {
        RMT_CLK_EN_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pcnt_clk_en(&mut self) -> PCNT_CLK_EN_W<PERIP_CLK_EN_SPEC> {
        PCNT_CLK_EN_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ledc_clk_en(&mut self) -> LEDC_CLK_EN_W<PERIP_CLK_EN_SPEC> {
        LEDC_CLK_EN_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn uhci1_clk_en(&mut self) -> UHCI1_CLK_EN_W<PERIP_CLK_EN_SPEC> {
        UHCI1_CLK_EN_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn timergroup_clk_en(&mut self) -> TIMERGROUP_CLK_EN_W<PERIP_CLK_EN_SPEC> {
        TIMERGROUP_CLK_EN_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn efuse_clk_en(&mut self) -> EFUSE_CLK_EN_W<PERIP_CLK_EN_SPEC> {
        EFUSE_CLK_EN_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn timergroup1_clk_en(&mut self) -> TIMERGROUP1_CLK_EN_W<PERIP_CLK_EN_SPEC> {
        TIMERGROUP1_CLK_EN_W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn spi3_clk_en(&mut self) -> SPI3_CLK_EN_W<PERIP_CLK_EN_SPEC> {
        SPI3_CLK_EN_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pwm0_clk_en(&mut self) -> PWM0_CLK_EN_W<PERIP_CLK_EN_SPEC> {
        PWM0_CLK_EN_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn i2c_ext1_clk_en(&mut self) -> I2C_EXT1_CLK_EN_W<PERIP_CLK_EN_SPEC> {
        I2C_EXT1_CLK_EN_W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn twai_clk_en(&mut self) -> TWAI_CLK_EN_W<PERIP_CLK_EN_SPEC> {
        TWAI_CLK_EN_W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pwm1_clk_en(&mut self) -> PWM1_CLK_EN_W<PERIP_CLK_EN_SPEC> {
        PWM1_CLK_EN_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn i2s1_clk_en(&mut self) -> I2S1_CLK_EN_W<PERIP_CLK_EN_SPEC> {
        I2S1_CLK_EN_W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn spi_dma_clk_en(&mut self) -> SPI_DMA_CLK_EN_W<PERIP_CLK_EN_SPEC> {
        SPI_DMA_CLK_EN_W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn uart2_clk_en(&mut self) -> UART2_CLK_EN_W<PERIP_CLK_EN_SPEC> {
        UART2_CLK_EN_W::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn uart_mem_clk_en(&mut self) -> UART_MEM_CLK_EN_W<PERIP_CLK_EN_SPEC> {
        UART_MEM_CLK_EN_W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pwm2_clk_en(&mut self) -> PWM2_CLK_EN_W<PERIP_CLK_EN_SPEC> {
        PWM2_CLK_EN_W::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn pwm3_clk_en(&mut self) -> PWM3_CLK_EN_W<PERIP_CLK_EN_SPEC> {
        PWM3_CLK_EN_W::new(self, 26)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`perip_clk_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perip_clk_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERIP_CLK_EN_SPEC;
impl crate::RegisterSpec for PERIP_CLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perip_clk_en::R`](R) reader structure"]
impl crate::Readable for PERIP_CLK_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`perip_clk_en::W`](W) writer structure"]
impl crate::Writable for PERIP_CLK_EN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERIP_CLK_EN to value 0xf9c1_e06f"]
impl crate::Resettable for PERIP_CLK_EN_SPEC {
    const RESET_VALUE: u32 = 0xf9c1_e06f;
}
