#[doc = "Register `CH_CURR_PTR` reader"]
pub type R = crate::R<CH_CURR_PTR_SPEC>;
#[doc = "Register `CH_CURR_PTR` writer"]
pub type W = crate::W<CH_CURR_PTR_SPEC>;
#[doc = "Field `ADDR` reader - Address of current descriptor. When this field is '0', there is no valid descriptor. Note: HW updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: Typically, when SW updates the current descriptor pointer CH_CURR_PTR, it also sets CH_IDX.X_IDX and CH_IDX.Y_IDX to '0'."]
pub type ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Address of current descriptor. When this field is '0', there is no valid descriptor. Note: HW updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: Typically, when SW updates the current descriptor pointer CH_CURR_PTR, it also sets CH_IDX.X_IDX and CH_IDX.Y_IDX to '0'."]
pub type ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Address of current descriptor. When this field is '0', there is no valid descriptor. Note: HW updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: Typically, when SW updates the current descriptor pointer CH_CURR_PTR, it also sets CH_IDX.X_IDX and CH_IDX.Y_IDX to '0'."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Address of current descriptor. When this field is '0', there is no valid descriptor. Note: HW updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: Typically, when SW updates the current descriptor pointer CH_CURR_PTR, it also sets CH_IDX.X_IDX and CH_IDX.Y_IDX to '0'."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<CH_CURR_PTR_SPEC> {
        ADDR_W::new(self, 2)
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
#[doc = "Channel current descriptor pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_curr_ptr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_curr_ptr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_CURR_PTR_SPEC;
impl crate::RegisterSpec for CH_CURR_PTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_curr_ptr::R`](R) reader structure"]
impl crate::Readable for CH_CURR_PTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch_curr_ptr::W`](W) writer structure"]
impl crate::Writable for CH_CURR_PTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH_CURR_PTR to value 0"]
impl crate::Resettable for CH_CURR_PTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
