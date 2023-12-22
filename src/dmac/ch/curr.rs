#[doc = "Register `CURR` reader"]
pub type R = crate::R<CURR_SPEC>;
#[doc = "Register `CURR` writer"]
pub type W = crate::W<CURR_SPEC>;
#[doc = "Field `PTR` reader - Address of current descriptor. When this field is '0', there is no valid descriptor. Note: HW updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor."]
pub type PTR_R = crate::FieldReader<u32>;
#[doc = "Field `PTR` writer - Address of current descriptor. When this field is '0', there is no valid descriptor. Note: HW updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor."]
pub type PTR_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Address of current descriptor. When this field is '0', there is no valid descriptor. Note: HW updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor."]
    #[inline(always)]
    pub fn ptr(&self) -> PTR_R {
        PTR_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Address of current descriptor. When this field is '0', there is no valid descriptor. Note: HW updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor."]
    #[inline(always)]
    #[must_use]
    pub fn ptr(&mut self) -> PTR_W<CURR_SPEC> {
        PTR_W::new(self, 2)
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
#[doc = "Channel current descriptor pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`curr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`curr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CURR_SPEC;
impl crate::RegisterSpec for CURR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`curr::R`](R) reader structure"]
impl crate::Readable for CURR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`curr::W`](W) writer structure"]
impl crate::Writable for CURR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CURR to value 0"]
impl crate::Resettable for CURR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
