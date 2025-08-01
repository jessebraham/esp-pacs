#[doc = "Register `CONF` reader"]
pub type R = crate::R<CONF_SPEC>;
#[doc = "Register `CONF` writer"]
pub type W = crate::W<CONF_SPEC>;
#[doc = "Field `APB_CLK_SEL` reader - This bit is used to set the frequency of slow_clk. 1'b1:80mhz 1'b0:8mhz"]
pub type APB_CLK_SEL_R = crate::BitReader;
#[doc = "Field `APB_CLK_SEL` writer - This bit is used to set the frequency of slow_clk. 1'b1:80mhz 1'b0:8mhz"]
pub type APB_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit is used to set the frequency of slow_clk. 1'b1:80mhz 1'b0:8mhz"]
    #[inline(always)]
    pub fn apb_clk_sel(&self) -> APB_CLK_SEL_R {
        APB_CLK_SEL_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF")
            .field("apb_clk_sel", &self.apb_clk_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to set the frequency of slow_clk. 1'b1:80mhz 1'b0:8mhz"]
    #[inline(always)]
    pub fn apb_clk_sel(&mut self) -> APB_CLK_SEL_W<CONF_SPEC> {
        APB_CLK_SEL_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf::R`](R) reader structure"]
impl crate::Readable for CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf::W`](W) writer structure"]
impl crate::Writable for CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONF to value 0"]
impl crate::Resettable for CONF_SPEC {}
