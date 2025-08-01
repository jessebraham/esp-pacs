#[doc = "Register `CPU_INTR_FROM_CPU_2` reader"]
pub type R = crate::R<CPU_INTR_FROM_CPU_2_SPEC>;
#[doc = "Register `CPU_INTR_FROM_CPU_2` writer"]
pub type W = crate::W<CPU_INTR_FROM_CPU_2_SPEC>;
#[doc = "Field `CPU_INTR_FROM_CPU_2` reader - Set this bit to generate CPU interrupt 2. This bit needs to be reset by software in the ISR process."]
pub type CPU_INTR_FROM_CPU_2_R = crate::BitReader;
#[doc = "Field `CPU_INTR_FROM_CPU_2` writer - Set this bit to generate CPU interrupt 2. This bit needs to be reset by software in the ISR process."]
pub type CPU_INTR_FROM_CPU_2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to generate CPU interrupt 2. This bit needs to be reset by software in the ISR process."]
    #[inline(always)]
    pub fn cpu_intr_from_cpu_2(&self) -> CPU_INTR_FROM_CPU_2_R {
        CPU_INTR_FROM_CPU_2_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_INTR_FROM_CPU_2")
            .field("cpu_intr_from_cpu_2", &self.cpu_intr_from_cpu_2())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to generate CPU interrupt 2. This bit needs to be reset by software in the ISR process."]
    #[inline(always)]
    pub fn cpu_intr_from_cpu_2(&mut self) -> CPU_INTR_FROM_CPU_2_W<CPU_INTR_FROM_CPU_2_SPEC> {
        CPU_INTR_FROM_CPU_2_W::new(self, 0)
    }
}
#[doc = "CPU interrupt controlling register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_intr_from_cpu_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPU_INTR_FROM_CPU_2_SPEC;
impl crate::RegisterSpec for CPU_INTR_FROM_CPU_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_intr_from_cpu_2::R`](R) reader structure"]
impl crate::Readable for CPU_INTR_FROM_CPU_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpu_intr_from_cpu_2::W`](W) writer structure"]
impl crate::Writable for CPU_INTR_FROM_CPU_2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_INTR_FROM_CPU_2 to value 0"]
impl crate::Resettable for CPU_INTR_FROM_CPU_2_SPEC {}
