#[doc = "Register `CM4_NMI_CTL[%s]` reader"]
pub type R = crate::R<CM4_NMI_CTL_SPEC>;
#[doc = "Register `CM4_NMI_CTL[%s]` writer"]
pub type W = crate::W<CM4_NMI_CTL_SPEC>;
#[doc = "Field `SYSTEM_INT_IDX` reader - System interrupt select for CPU NMI. The reset value ('1023') ensures that the CPU NMI is NOT connected to any system interrupt after DeepSleep reset."]
pub type SYSTEM_INT_IDX_R = crate::FieldReader<u16>;
#[doc = "Field `SYSTEM_INT_IDX` writer - System interrupt select for CPU NMI. The reset value ('1023') ensures that the CPU NMI is NOT connected to any system interrupt after DeepSleep reset."]
pub type SYSTEM_INT_IDX_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - System interrupt select for CPU NMI. The reset value ('1023') ensures that the CPU NMI is NOT connected to any system interrupt after DeepSleep reset."]
    #[inline(always)]
    pub fn system_int_idx(&self) -> SYSTEM_INT_IDX_R {
        SYSTEM_INT_IDX_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - System interrupt select for CPU NMI. The reset value ('1023') ensures that the CPU NMI is NOT connected to any system interrupt after DeepSleep reset."]
    #[inline(always)]
    #[must_use]
    pub fn system_int_idx(&mut self) -> SYSTEM_INT_IDX_W<CM4_NMI_CTL_SPEC> {
        SYSTEM_INT_IDX_W::new(self, 0)
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
#[doc = "CM4 NMI control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm4_nmi_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm4_nmi_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CM4_NMI_CTL_SPEC;
impl crate::RegisterSpec for CM4_NMI_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm4_nmi_ctl::R`](R) reader structure"]
impl crate::Readable for CM4_NMI_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cm4_nmi_ctl::W`](W) writer structure"]
impl crate::Writable for CM4_NMI_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CM4_NMI_CTL[%s]
to value 0x03ff"]
impl crate::Resettable for CM4_NMI_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x03ff;
}
