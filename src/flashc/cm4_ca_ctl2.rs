#[doc = "Register `CM4_CA_CTL2` reader"]
pub type R = crate::R<CM4_CA_CTL2_SPEC>;
#[doc = "Register `CM4_CA_CTL2` writer"]
pub type W = crate::W<CM4_CA_CTL2_SPEC>;
#[doc = "Field `PWRUP_DELAY` reader - Number clock cycles delay needed after power domain power up"]
pub type PWRUP_DELAY_R = crate::FieldReader<u16>;
#[doc = "Field `PWRUP_DELAY` writer - Number clock cycles delay needed after power domain power up"]
pub type PWRUP_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Number clock cycles delay needed after power domain power up"]
    #[inline(always)]
    pub fn pwrup_delay(&self) -> PWRUP_DELAY_R {
        PWRUP_DELAY_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Number clock cycles delay needed after power domain power up"]
    #[inline(always)]
    #[must_use]
    pub fn pwrup_delay(&mut self) -> PWRUP_DELAY_W<CM4_CA_CTL2_SPEC> {
        PWRUP_DELAY_W::new(self, 0)
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
#[doc = "CM4 cache control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm4_ca_ctl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm4_ca_ctl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CM4_CA_CTL2_SPEC;
impl crate::RegisterSpec for CM4_CA_CTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm4_ca_ctl2::R`](R) reader structure"]
impl crate::Readable for CM4_CA_CTL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cm4_ca_ctl2::W`](W) writer structure"]
impl crate::Writable for CM4_CA_CTL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CM4_CA_CTL2 to value 0x012c"]
impl crate::Resettable for CM4_CA_CTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0x012c;
}
