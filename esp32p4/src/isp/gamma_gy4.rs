#[doc = "Register `GAMMA_GY4` reader"]
pub type R = crate::R<GAMMA_GY4_SPEC>;
#[doc = "Register `GAMMA_GY4` writer"]
pub type W = crate::W<GAMMA_GY4_SPEC>;
#[doc = "Field `GAMMA_G_Y0F` reader - this field configures the point 15 of Y-axis of g channel gamma curve"]
pub type GAMMA_G_Y0F_R = crate::FieldReader;
#[doc = "Field `GAMMA_G_Y0F` writer - this field configures the point 15 of Y-axis of g channel gamma curve"]
pub type GAMMA_G_Y0F_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GAMMA_G_Y0E` reader - this field configures the point 14 of Y-axis of g channel gamma curve"]
pub type GAMMA_G_Y0E_R = crate::FieldReader;
#[doc = "Field `GAMMA_G_Y0E` writer - this field configures the point 14 of Y-axis of g channel gamma curve"]
pub type GAMMA_G_Y0E_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GAMMA_G_Y0D` reader - this field configures the point 13 of Y-axis of g channel gamma curve"]
pub type GAMMA_G_Y0D_R = crate::FieldReader;
#[doc = "Field `GAMMA_G_Y0D` writer - this field configures the point 13 of Y-axis of g channel gamma curve"]
pub type GAMMA_G_Y0D_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GAMMA_G_Y0C` reader - this field configures the point 12 of Y-axis of g channel gamma curve"]
pub type GAMMA_G_Y0C_R = crate::FieldReader;
#[doc = "Field `GAMMA_G_Y0C` writer - this field configures the point 12 of Y-axis of g channel gamma curve"]
pub type GAMMA_G_Y0C_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - this field configures the point 15 of Y-axis of g channel gamma curve"]
    #[inline(always)]
    pub fn gamma_g_y0f(&self) -> GAMMA_G_Y0F_R {
        GAMMA_G_Y0F_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures the point 14 of Y-axis of g channel gamma curve"]
    #[inline(always)]
    pub fn gamma_g_y0e(&self) -> GAMMA_G_Y0E_R {
        GAMMA_G_Y0E_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures the point 13 of Y-axis of g channel gamma curve"]
    #[inline(always)]
    pub fn gamma_g_y0d(&self) -> GAMMA_G_Y0D_R {
        GAMMA_G_Y0D_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - this field configures the point 12 of Y-axis of g channel gamma curve"]
    #[inline(always)]
    pub fn gamma_g_y0c(&self) -> GAMMA_G_Y0C_R {
        GAMMA_G_Y0C_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GAMMA_GY4")
            .field("gamma_g_y0f", &self.gamma_g_y0f())
            .field("gamma_g_y0e", &self.gamma_g_y0e())
            .field("gamma_g_y0d", &self.gamma_g_y0d())
            .field("gamma_g_y0c", &self.gamma_g_y0c())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures the point 15 of Y-axis of g channel gamma curve"]
    #[inline(always)]
    pub fn gamma_g_y0f(&mut self) -> GAMMA_G_Y0F_W<GAMMA_GY4_SPEC> {
        GAMMA_G_Y0F_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - this field configures the point 14 of Y-axis of g channel gamma curve"]
    #[inline(always)]
    pub fn gamma_g_y0e(&mut self) -> GAMMA_G_Y0E_W<GAMMA_GY4_SPEC> {
        GAMMA_G_Y0E_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures the point 13 of Y-axis of g channel gamma curve"]
    #[inline(always)]
    pub fn gamma_g_y0d(&mut self) -> GAMMA_G_Y0D_W<GAMMA_GY4_SPEC> {
        GAMMA_G_Y0D_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - this field configures the point 12 of Y-axis of g channel gamma curve"]
    #[inline(always)]
    pub fn gamma_g_y0c(&mut self) -> GAMMA_G_Y0C_W<GAMMA_GY4_SPEC> {
        GAMMA_G_Y0C_W::new(self, 24)
    }
}
#[doc = "point of Y-axis of g channel gamma curve register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`gamma_gy4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gamma_gy4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GAMMA_GY4_SPEC;
impl crate::RegisterSpec for GAMMA_GY4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gamma_gy4::R`](R) reader structure"]
impl crate::Readable for GAMMA_GY4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gamma_gy4::W`](W) writer structure"]
impl crate::Writable for GAMMA_GY4_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GAMMA_GY4 to value 0xd0e0_f0ff"]
impl crate::Resettable for GAMMA_GY4_SPEC {
    const RESET_VALUE: u32 = 0xd0e0_f0ff;
}
