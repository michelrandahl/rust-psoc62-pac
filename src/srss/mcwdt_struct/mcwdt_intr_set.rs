#[doc = "Register `MCWDT_INTR_SET` reader"]
pub type R = crate::R<MCWDT_INTR_SET_SPEC>;
#[doc = "Register `MCWDT_INTR_SET` writer"]
pub type W = crate::W<MCWDT_INTR_SET_SPEC>;
#[doc = "Field `MCWDT_INT0` reader - Set interrupt for MCWDT_INT0"]
pub type MCWDT_INT0_R = crate::BitReader;
#[doc = "Field `MCWDT_INT0` writer - Set interrupt for MCWDT_INT0"]
pub type MCWDT_INT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCWDT_INT1` reader - Set interrupt for MCWDT_INT1"]
pub type MCWDT_INT1_R = crate::BitReader;
#[doc = "Field `MCWDT_INT1` writer - Set interrupt for MCWDT_INT1"]
pub type MCWDT_INT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCWDT_INT2` reader - Set interrupt for MCWDT_INT2"]
pub type MCWDT_INT2_R = crate::BitReader;
#[doc = "Field `MCWDT_INT2` writer - Set interrupt for MCWDT_INT2"]
pub type MCWDT_INT2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set interrupt for MCWDT_INT0"]
    #[inline(always)]
    pub fn mcwdt_int0(&self) -> MCWDT_INT0_R {
        MCWDT_INT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set interrupt for MCWDT_INT1"]
    #[inline(always)]
    pub fn mcwdt_int1(&self) -> MCWDT_INT1_R {
        MCWDT_INT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set interrupt for MCWDT_INT2"]
    #[inline(always)]
    pub fn mcwdt_int2(&self) -> MCWDT_INT2_R {
        MCWDT_INT2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set interrupt for MCWDT_INT0"]
    #[inline(always)]
    #[must_use]
    pub fn mcwdt_int0(&mut self) -> MCWDT_INT0_W<MCWDT_INTR_SET_SPEC> {
        MCWDT_INT0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set interrupt for MCWDT_INT1"]
    #[inline(always)]
    #[must_use]
    pub fn mcwdt_int1(&mut self) -> MCWDT_INT1_W<MCWDT_INTR_SET_SPEC> {
        MCWDT_INT1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set interrupt for MCWDT_INT2"]
    #[inline(always)]
    #[must_use]
    pub fn mcwdt_int2(&mut self) -> MCWDT_INT2_W<MCWDT_INTR_SET_SPEC> {
        MCWDT_INT2_W::new(self, 2)
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
#[doc = "Multi-Counter Watchdog Counter Interrupt Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcwdt_intr_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcwdt_intr_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCWDT_INTR_SET_SPEC;
impl crate::RegisterSpec for MCWDT_INTR_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcwdt_intr_set::R`](R) reader structure"]
impl crate::Readable for MCWDT_INTR_SET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcwdt_intr_set::W`](W) writer structure"]
impl crate::Writable for MCWDT_INTR_SET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCWDT_INTR_SET to value 0"]
impl crate::Resettable for MCWDT_INTR_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
