#[doc = "Register `LCD_FIRST_CMD_VAL` reader"]
pub type R = crate::R<LCD_FIRST_CMD_VAL_SPEC>;
#[doc = "Register `LCD_FIRST_CMD_VAL` writer"]
pub type W = crate::W<LCD_FIRST_CMD_VAL_SPEC>;
#[doc = "Field `LCD_FIRST_CMD_VALUE` reader - The LCD write command value of first cmd cycle."]
pub type LCD_FIRST_CMD_VALUE_R = crate::FieldReader<u32>;
#[doc = "Field `LCD_FIRST_CMD_VALUE` writer - The LCD write command value of first cmd cycle."]
pub type LCD_FIRST_CMD_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The LCD write command value of first cmd cycle."]
    #[inline(always)]
    pub fn lcd_first_cmd_value(&self) -> LCD_FIRST_CMD_VALUE_R {
        LCD_FIRST_CMD_VALUE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCD_FIRST_CMD_VAL")
            .field("lcd_first_cmd_value", &self.lcd_first_cmd_value())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - The LCD write command value of first cmd cycle."]
    #[inline(always)]
    pub fn lcd_first_cmd_value(&mut self) -> LCD_FIRST_CMD_VALUE_W<LCD_FIRST_CMD_VAL_SPEC> {
        LCD_FIRST_CMD_VALUE_W::new(self, 0)
    }
}
#[doc = "LCD config register.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_first_cmd_val::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_first_cmd_val::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_FIRST_CMD_VAL_SPEC;
impl crate::RegisterSpec for LCD_FIRST_CMD_VAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_first_cmd_val::R`](R) reader structure"]
impl crate::Readable for LCD_FIRST_CMD_VAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_first_cmd_val::W`](W) writer structure"]
impl crate::Writable for LCD_FIRST_CMD_VAL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD_FIRST_CMD_VAL to value 0"]
impl crate::Resettable for LCD_FIRST_CMD_VAL_SPEC {}
