#[doc = "Register `CM4_PWR_DELAY_CTL` reader"]
pub type R = crate::R<CM4_PWR_DELAY_CTL_SPEC>;
#[doc = "Register `CM4_PWR_DELAY_CTL` writer"]
pub type W = crate::W<CM4_PWR_DELAY_CTL_SPEC>;
#[doc = "Field `UP` reader - Number clock cycles delay needed after power domain power up"]
pub type UP_R = crate::FieldReader<u16>;
#[doc = "Field `UP` writer - Number clock cycles delay needed after power domain power up"]
pub type UP_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Number clock cycles delay needed after power domain power up"]
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Number clock cycles delay needed after power domain power up"]
    #[inline(always)]
    #[must_use]
    pub fn up(&mut self) -> UP_W<CM4_PWR_DELAY_CTL_SPEC> {
        UP_W::new(self, 0)
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
#[doc = "CM4 power control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm4_pwr_delay_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm4_pwr_delay_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CM4_PWR_DELAY_CTL_SPEC;
impl crate::RegisterSpec for CM4_PWR_DELAY_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm4_pwr_delay_ctl::R`](R) reader structure"]
impl crate::Readable for CM4_PWR_DELAY_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cm4_pwr_delay_ctl::W`](W) writer structure"]
impl crate::Writable for CM4_PWR_DELAY_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CM4_PWR_DELAY_CTL to value 0x012c"]
impl crate::Resettable for CM4_PWR_DELAY_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x012c;
}
