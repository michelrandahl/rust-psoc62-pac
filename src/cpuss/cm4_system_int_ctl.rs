#[doc = "Register `CM4_SYSTEM_INT_CTL[%s]` reader"]
pub type R = crate::R<CM4_SYSTEM_INT_CTL_SPEC>;
#[doc = "Register `CM4_SYSTEM_INT_CTL[%s]` writer"]
pub type W = crate::W<CM4_SYSTEM_INT_CTL_SPEC>;
#[doc = "Field `CPU_INT_IDX` reader - N/A"]
pub type CPU_INT_IDX_R = crate::FieldReader;
#[doc = "Field `CPU_INT_IDX` writer - N/A"]
pub type CPU_INT_IDX_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CPU_INT_VALID` reader - N/A"]
pub type CPU_INT_VALID_R = crate::BitReader;
#[doc = "Field `CPU_INT_VALID` writer - N/A"]
pub type CPU_INT_VALID_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - N/A"]
    #[inline(always)]
    pub fn cpu_int_idx(&self) -> CPU_INT_IDX_R {
        CPU_INT_IDX_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 31 - N/A"]
    #[inline(always)]
    pub fn cpu_int_valid(&self) -> CPU_INT_VALID_R {
        CPU_INT_VALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_int_idx(&mut self) -> CPU_INT_IDX_W<CM4_SYSTEM_INT_CTL_SPEC> {
        CPU_INT_IDX_W::new(self, 0)
    }
    #[doc = "Bit 31 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_int_valid(&mut self) -> CPU_INT_VALID_W<CM4_SYSTEM_INT_CTL_SPEC> {
        CPU_INT_VALID_W::new(self, 31)
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
#[doc = "CM4 system interrupt control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm4_system_int_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm4_system_int_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CM4_SYSTEM_INT_CTL_SPEC;
impl crate::RegisterSpec for CM4_SYSTEM_INT_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm4_system_int_ctl::R`](R) reader structure"]
impl crate::Readable for CM4_SYSTEM_INT_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cm4_system_int_ctl::W`](W) writer structure"]
impl crate::Writable for CM4_SYSTEM_INT_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CM4_SYSTEM_INT_CTL[%s]
to value 0"]
impl crate::Resettable for CM4_SYSTEM_INT_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
