#[doc = "Register `MS_ATT1` reader"]
pub type R = crate::R<MS_ATT1_SPEC>;
#[doc = "Register `MS_ATT1` writer"]
pub type W = crate::W<MS_ATT1_SPEC>;
#[doc = "Field `PC4_UR` reader - Protection context 4, user read enable."]
pub type PC4_UR_R = crate::BitReader;
#[doc = "Field `PC4_UW` reader - Protection context 4, user write enable."]
pub type PC4_UW_R = crate::BitReader;
#[doc = "Field `PC4_UW` writer - Protection context 4, user write enable."]
pub type PC4_UW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC4_PR` reader - Protection context 4, privileged read enable."]
pub type PC4_PR_R = crate::BitReader;
#[doc = "Field `PC4_PW` reader - Protection context 4, privileged write enable."]
pub type PC4_PW_R = crate::BitReader;
#[doc = "Field `PC4_PW` writer - Protection context 4, privileged write enable."]
pub type PC4_PW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC4_NS` reader - Protection context 4, non-secure."]
pub type PC4_NS_R = crate::BitReader;
#[doc = "Field `PC4_NS` writer - Protection context 4, non-secure."]
pub type PC4_NS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC5_UR` reader - Protection context 5, user read enable."]
pub type PC5_UR_R = crate::BitReader;
#[doc = "Field `PC5_UW` reader - Protection context 5, user write enable."]
pub type PC5_UW_R = crate::BitReader;
#[doc = "Field `PC5_UW` writer - Protection context 5, user write enable."]
pub type PC5_UW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC5_PR` reader - Protection context 5, privileged read enable."]
pub type PC5_PR_R = crate::BitReader;
#[doc = "Field `PC5_PW` reader - Protection context 5, privileged write enable."]
pub type PC5_PW_R = crate::BitReader;
#[doc = "Field `PC5_PW` writer - Protection context 5, privileged write enable."]
pub type PC5_PW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC5_NS` reader - Protection context 5, non-secure."]
pub type PC5_NS_R = crate::BitReader;
#[doc = "Field `PC5_NS` writer - Protection context 5, non-secure."]
pub type PC5_NS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC6_UR` reader - Protection context 6, user read enable."]
pub type PC6_UR_R = crate::BitReader;
#[doc = "Field `PC6_UW` reader - Protection context 6, user write enable."]
pub type PC6_UW_R = crate::BitReader;
#[doc = "Field `PC6_UW` writer - Protection context 6, user write enable."]
pub type PC6_UW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC6_PR` reader - Protection context 6, privileged read enable."]
pub type PC6_PR_R = crate::BitReader;
#[doc = "Field `PC6_PW` reader - Protection context 6, privileged write enable."]
pub type PC6_PW_R = crate::BitReader;
#[doc = "Field `PC6_PW` writer - Protection context 6, privileged write enable."]
pub type PC6_PW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC6_NS` reader - Protection context 6, non-secure."]
pub type PC6_NS_R = crate::BitReader;
#[doc = "Field `PC6_NS` writer - Protection context 6, non-secure."]
pub type PC6_NS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC7_UR` reader - Protection context 7, user read enable."]
pub type PC7_UR_R = crate::BitReader;
#[doc = "Field `PC7_UW` reader - Protection context 7, user write enable."]
pub type PC7_UW_R = crate::BitReader;
#[doc = "Field `PC7_UW` writer - Protection context 7, user write enable."]
pub type PC7_UW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC7_PR` reader - Protection context 7, privileged read enable."]
pub type PC7_PR_R = crate::BitReader;
#[doc = "Field `PC7_PW` reader - Protection context 7, privileged write enable."]
pub type PC7_PW_R = crate::BitReader;
#[doc = "Field `PC7_PW` writer - Protection context 7, privileged write enable."]
pub type PC7_PW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC7_NS` reader - Protection context 7, non-secure."]
pub type PC7_NS_R = crate::BitReader;
#[doc = "Field `PC7_NS` writer - Protection context 7, non-secure."]
pub type PC7_NS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Protection context 4, user read enable."]
    #[inline(always)]
    pub fn pc4_ur(&self) -> PC4_UR_R {
        PC4_UR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Protection context 4, user write enable."]
    #[inline(always)]
    pub fn pc4_uw(&self) -> PC4_UW_R {
        PC4_UW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Protection context 4, privileged read enable."]
    #[inline(always)]
    pub fn pc4_pr(&self) -> PC4_PR_R {
        PC4_PR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Protection context 4, privileged write enable."]
    #[inline(always)]
    pub fn pc4_pw(&self) -> PC4_PW_R {
        PC4_PW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Protection context 4, non-secure."]
    #[inline(always)]
    pub fn pc4_ns(&self) -> PC4_NS_R {
        PC4_NS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Protection context 5, user read enable."]
    #[inline(always)]
    pub fn pc5_ur(&self) -> PC5_UR_R {
        PC5_UR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Protection context 5, user write enable."]
    #[inline(always)]
    pub fn pc5_uw(&self) -> PC5_UW_R {
        PC5_UW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Protection context 5, privileged read enable."]
    #[inline(always)]
    pub fn pc5_pr(&self) -> PC5_PR_R {
        PC5_PR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Protection context 5, privileged write enable."]
    #[inline(always)]
    pub fn pc5_pw(&self) -> PC5_PW_R {
        PC5_PW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Protection context 5, non-secure."]
    #[inline(always)]
    pub fn pc5_ns(&self) -> PC5_NS_R {
        PC5_NS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Protection context 6, user read enable."]
    #[inline(always)]
    pub fn pc6_ur(&self) -> PC6_UR_R {
        PC6_UR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Protection context 6, user write enable."]
    #[inline(always)]
    pub fn pc6_uw(&self) -> PC6_UW_R {
        PC6_UW_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Protection context 6, privileged read enable."]
    #[inline(always)]
    pub fn pc6_pr(&self) -> PC6_PR_R {
        PC6_PR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Protection context 6, privileged write enable."]
    #[inline(always)]
    pub fn pc6_pw(&self) -> PC6_PW_R {
        PC6_PW_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Protection context 6, non-secure."]
    #[inline(always)]
    pub fn pc6_ns(&self) -> PC6_NS_R {
        PC6_NS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Protection context 7, user read enable."]
    #[inline(always)]
    pub fn pc7_ur(&self) -> PC7_UR_R {
        PC7_UR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Protection context 7, user write enable."]
    #[inline(always)]
    pub fn pc7_uw(&self) -> PC7_UW_R {
        PC7_UW_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Protection context 7, privileged read enable."]
    #[inline(always)]
    pub fn pc7_pr(&self) -> PC7_PR_R {
        PC7_PR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Protection context 7, privileged write enable."]
    #[inline(always)]
    pub fn pc7_pw(&self) -> PC7_PW_R {
        PC7_PW_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Protection context 7, non-secure."]
    #[inline(always)]
    pub fn pc7_ns(&self) -> PC7_NS_R {
        PC7_NS_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Protection context 4, user write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc4_uw(&mut self) -> PC4_UW_W<MS_ATT1_SPEC> {
        PC4_UW_W::new(self, 1)
    }
    #[doc = "Bit 3 - Protection context 4, privileged write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc4_pw(&mut self) -> PC4_PW_W<MS_ATT1_SPEC> {
        PC4_PW_W::new(self, 3)
    }
    #[doc = "Bit 4 - Protection context 4, non-secure."]
    #[inline(always)]
    #[must_use]
    pub fn pc4_ns(&mut self) -> PC4_NS_W<MS_ATT1_SPEC> {
        PC4_NS_W::new(self, 4)
    }
    #[doc = "Bit 9 - Protection context 5, user write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc5_uw(&mut self) -> PC5_UW_W<MS_ATT1_SPEC> {
        PC5_UW_W::new(self, 9)
    }
    #[doc = "Bit 11 - Protection context 5, privileged write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc5_pw(&mut self) -> PC5_PW_W<MS_ATT1_SPEC> {
        PC5_PW_W::new(self, 11)
    }
    #[doc = "Bit 12 - Protection context 5, non-secure."]
    #[inline(always)]
    #[must_use]
    pub fn pc5_ns(&mut self) -> PC5_NS_W<MS_ATT1_SPEC> {
        PC5_NS_W::new(self, 12)
    }
    #[doc = "Bit 17 - Protection context 6, user write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc6_uw(&mut self) -> PC6_UW_W<MS_ATT1_SPEC> {
        PC6_UW_W::new(self, 17)
    }
    #[doc = "Bit 19 - Protection context 6, privileged write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc6_pw(&mut self) -> PC6_PW_W<MS_ATT1_SPEC> {
        PC6_PW_W::new(self, 19)
    }
    #[doc = "Bit 20 - Protection context 6, non-secure."]
    #[inline(always)]
    #[must_use]
    pub fn pc6_ns(&mut self) -> PC6_NS_W<MS_ATT1_SPEC> {
        PC6_NS_W::new(self, 20)
    }
    #[doc = "Bit 25 - Protection context 7, user write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc7_uw(&mut self) -> PC7_UW_W<MS_ATT1_SPEC> {
        PC7_UW_W::new(self, 25)
    }
    #[doc = "Bit 27 - Protection context 7, privileged write enable."]
    #[inline(always)]
    #[must_use]
    pub fn pc7_pw(&mut self) -> PC7_PW_W<MS_ATT1_SPEC> {
        PC7_PW_W::new(self, 27)
    }
    #[doc = "Bit 28 - Protection context 7, non-secure."]
    #[inline(always)]
    #[must_use]
    pub fn pc7_ns(&mut self) -> PC7_NS_W<MS_ATT1_SPEC> {
        PC7_NS_W::new(self, 28)
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
#[doc = "Master attributes 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ms_att1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ms_att1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MS_ATT1_SPEC;
impl crate::RegisterSpec for MS_ATT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ms_att1::R`](R) reader structure"]
impl crate::Readable for MS_ATT1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ms_att1::W`](W) writer structure"]
impl crate::Writable for MS_ATT1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MS_ATT1 to value 0x1f1f_1f1f"]
impl crate::Resettable for MS_ATT1_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f1f_1f1f;
}
