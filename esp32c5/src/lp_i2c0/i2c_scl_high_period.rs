#[doc = "Register `I2C_SCL_HIGH_PERIOD` reader"]
pub type R = crate::R<I2C_SCL_HIGH_PERIOD_SPEC>;
#[doc = "Register `I2C_SCL_HIGH_PERIOD` writer"]
pub type W = crate::W<I2C_SCL_HIGH_PERIOD_SPEC>;
#[doc = "Field `I2C_SCL_HIGH_PERIOD` reader - This register is used to configure for how long SCL setup to high level and remains high in master mode, in I2C module clock cycles."]
pub type I2C_SCL_HIGH_PERIOD_R = crate::FieldReader<u16>;
#[doc = "Field `I2C_SCL_HIGH_PERIOD` writer - This register is used to configure for how long SCL setup to high level and remains high in master mode, in I2C module clock cycles."]
pub type I2C_SCL_HIGH_PERIOD_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `I2C_SCL_WAIT_HIGH_PERIOD` reader - This register is used to configure for the SCL_FSM's waiting period for SCL high level in master mode, in I2C module clock cycles."]
pub type I2C_SCL_WAIT_HIGH_PERIOD_R = crate::FieldReader;
#[doc = "Field `I2C_SCL_WAIT_HIGH_PERIOD` writer - This register is used to configure for the SCL_FSM's waiting period for SCL high level in master mode, in I2C module clock cycles."]
pub type I2C_SCL_WAIT_HIGH_PERIOD_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:8 - This register is used to configure for how long SCL setup to high level and remains high in master mode, in I2C module clock cycles."]
    #[inline(always)]
    pub fn i2c_scl_high_period(&self) -> I2C_SCL_HIGH_PERIOD_R {
        I2C_SCL_HIGH_PERIOD_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:15 - This register is used to configure for the SCL_FSM's waiting period for SCL high level in master mode, in I2C module clock cycles."]
    #[inline(always)]
    pub fn i2c_scl_wait_high_period(&self) -> I2C_SCL_WAIT_HIGH_PERIOD_R {
        I2C_SCL_WAIT_HIGH_PERIOD_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_SCL_HIGH_PERIOD")
            .field("i2c_scl_high_period", &self.i2c_scl_high_period())
            .field("i2c_scl_wait_high_period", &self.i2c_scl_wait_high_period())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - This register is used to configure for how long SCL setup to high level and remains high in master mode, in I2C module clock cycles."]
    #[inline(always)]
    pub fn i2c_scl_high_period(&mut self) -> I2C_SCL_HIGH_PERIOD_W<I2C_SCL_HIGH_PERIOD_SPEC> {
        I2C_SCL_HIGH_PERIOD_W::new(self, 0)
    }
    #[doc = "Bits 9:15 - This register is used to configure for the SCL_FSM's waiting period for SCL high level in master mode, in I2C module clock cycles."]
    #[inline(always)]
    pub fn i2c_scl_wait_high_period(
        &mut self,
    ) -> I2C_SCL_WAIT_HIGH_PERIOD_W<I2C_SCL_HIGH_PERIOD_SPEC> {
        I2C_SCL_WAIT_HIGH_PERIOD_W::new(self, 9)
    }
}
#[doc = "Configures the high level width of SCL\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_scl_high_period::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_scl_high_period::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_SCL_HIGH_PERIOD_SPEC;
impl crate::RegisterSpec for I2C_SCL_HIGH_PERIOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_scl_high_period::R`](R) reader structure"]
impl crate::Readable for I2C_SCL_HIGH_PERIOD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c_scl_high_period::W`](W) writer structure"]
impl crate::Writable for I2C_SCL_HIGH_PERIOD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_SCL_HIGH_PERIOD to value 0"]
impl crate::Resettable for I2C_SCL_HIGH_PERIOD_SPEC {}
