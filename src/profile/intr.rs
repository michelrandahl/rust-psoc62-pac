#[doc = "Register `INTR` reader"]
pub type R = crate::R<INTR_SPEC>;
#[doc = "Register `INTR` writer"]
pub type W = crate::W<INTR_SPEC>;
#[doc = "Field `CNT_OVFLW` reader - This interrupt cause field is activated (HW sets the field to '1') when an profiling counter overflow (from 0xFFFFFFFF to 0x00000000) is captured. There is one bit per profling counter. SW writes a '1' to a bit of this field to clear this bit to '0' (writing 0xFFFFFFFF clears all interrupt causes to '0')."]
pub type CNT_OVFLW_R = crate::FieldReader<u32>;
#[doc = "Field `CNT_OVFLW` writer - This interrupt cause field is activated (HW sets the field to '1') when an profiling counter overflow (from 0xFFFFFFFF to 0x00000000) is captured. There is one bit per profling counter. SW writes a '1' to a bit of this field to clear this bit to '0' (writing 0xFFFFFFFF clears all interrupt causes to '0')."]
pub type CNT_OVFLW_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This interrupt cause field is activated (HW sets the field to '1') when an profiling counter overflow (from 0xFFFFFFFF to 0x00000000) is captured. There is one bit per profling counter. SW writes a '1' to a bit of this field to clear this bit to '0' (writing 0xFFFFFFFF clears all interrupt causes to '0')."]
    #[inline(always)]
    pub fn cnt_ovflw(&self) -> CNT_OVFLW_R {
        CNT_OVFLW_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This interrupt cause field is activated (HW sets the field to '1') when an profiling counter overflow (from 0xFFFFFFFF to 0x00000000) is captured. There is one bit per profling counter. SW writes a '1' to a bit of this field to clear this bit to '0' (writing 0xFFFFFFFF clears all interrupt causes to '0')."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_ovflw(&mut self) -> CNT_OVFLW_W<INTR_SPEC> {
        CNT_OVFLW_W::new(self, 0)
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
#[doc = "Profile interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr::R`](R) reader structure"]
impl crate::Readable for INTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr::W`](W) writer structure"]
impl crate::Writable for INTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for INTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
