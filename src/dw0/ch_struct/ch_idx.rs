#[doc = "Register `CH_IDX` reader"]
pub type R = crate::R<CH_IDX_SPEC>;
#[doc = "Register `CH_IDX` writer"]
pub type W = crate::W<CH_IDX_SPEC>;
#[doc = "Field `X_IDX` reader - Specifies the X loop index. In the range of \\[0, X_COUNT\\], with X_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: SW should set this field to '0' when it updates CH_CURR_PTR."]
pub type X_IDX_R = crate::FieldReader;
#[doc = "Field `X_IDX` writer - Specifies the X loop index. In the range of \\[0, X_COUNT\\], with X_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: SW should set this field to '0' when it updates CH_CURR_PTR."]
pub type X_IDX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Y_IDX` reader - Specifies the Y loop index, with X_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: SW should set this field to '0' when it updates CH_CURR_PTR."]
pub type Y_IDX_R = crate::FieldReader;
#[doc = "Field `Y_IDX` writer - Specifies the Y loop index, with X_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: SW should set this field to '0' when it updates CH_CURR_PTR."]
pub type Y_IDX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Specifies the X loop index. In the range of \\[0, X_COUNT\\], with X_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: SW should set this field to '0' when it updates CH_CURR_PTR."]
    #[inline(always)]
    pub fn x_idx(&self) -> X_IDX_R {
        X_IDX_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Specifies the Y loop index, with X_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: SW should set this field to '0' when it updates CH_CURR_PTR."]
    #[inline(always)]
    pub fn y_idx(&self) -> Y_IDX_R {
        Y_IDX_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Specifies the X loop index. In the range of \\[0, X_COUNT\\], with X_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: SW should set this field to '0' when it updates CH_CURR_PTR."]
    #[inline(always)]
    #[must_use]
    pub fn x_idx(&mut self) -> X_IDX_W<CH_IDX_SPEC> {
        X_IDX_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Specifies the Y loop index, with X_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: SW should set this field to '0' when it updates CH_CURR_PTR."]
    #[inline(always)]
    #[must_use]
    pub fn y_idx(&mut self) -> Y_IDX_W<CH_IDX_SPEC> {
        Y_IDX_W::new(self, 8)
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
#[doc = "Channel current indices\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_idx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_idx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_IDX_SPEC;
impl crate::RegisterSpec for CH_IDX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_idx::R`](R) reader structure"]
impl crate::Readable for CH_IDX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch_idx::W`](W) writer structure"]
impl crate::Writable for CH_IDX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH_IDX to value 0"]
impl crate::Resettable for CH_IDX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
