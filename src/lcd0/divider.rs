#[doc = "Register `DIVIDER` reader"]
pub type R = crate::R<DIVIDER_SPEC>;
#[doc = "Register `DIVIDER` writer"]
pub type W = crate::W<DIVIDER_SPEC>;
#[doc = "Field `SUBFR_DIV` reader - Input clock frequency divide value, to generate the 1/4 sub-frame period. The sub-frame period is 4*(SUBFR_DIV+1) cycles long."]
pub type SUBFR_DIV_R = crate::FieldReader<u16>;
#[doc = "Field `SUBFR_DIV` writer - Input clock frequency divide value, to generate the 1/4 sub-frame period. The sub-frame period is 4*(SUBFR_DIV+1) cycles long."]
pub type SUBFR_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DEAD_DIV` reader - Length of the dead time period in cycles. When set to zero, no dead time period exists."]
pub type DEAD_DIV_R = crate::FieldReader<u16>;
#[doc = "Field `DEAD_DIV` writer - Length of the dead time period in cycles. When set to zero, no dead time period exists."]
pub type DEAD_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input clock frequency divide value, to generate the 1/4 sub-frame period. The sub-frame period is 4*(SUBFR_DIV+1) cycles long."]
    #[inline(always)]
    pub fn subfr_div(&self) -> SUBFR_DIV_R {
        SUBFR_DIV_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Length of the dead time period in cycles. When set to zero, no dead time period exists."]
    #[inline(always)]
    pub fn dead_div(&self) -> DEAD_DIV_R {
        DEAD_DIV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Input clock frequency divide value, to generate the 1/4 sub-frame period. The sub-frame period is 4*(SUBFR_DIV+1) cycles long."]
    #[inline(always)]
    #[must_use]
    pub fn subfr_div(&mut self) -> SUBFR_DIV_W<DIVIDER_SPEC> {
        SUBFR_DIV_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Length of the dead time period in cycles. When set to zero, no dead time period exists."]
    #[inline(always)]
    #[must_use]
    pub fn dead_div(&mut self) -> DEAD_DIV_W<DIVIDER_SPEC> {
        DEAD_DIV_W::new(self, 16)
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
#[doc = "LCD Divider Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`divider::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`divider::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIVIDER_SPEC;
impl crate::RegisterSpec for DIVIDER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`divider::R`](R) reader structure"]
impl crate::Readable for DIVIDER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`divider::W`](W) writer structure"]
impl crate::Writable for DIVIDER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIVIDER to value 0"]
impl crate::Resettable for DIVIDER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
