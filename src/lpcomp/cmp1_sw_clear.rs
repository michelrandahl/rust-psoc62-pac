#[doc = "Register `CMP1_SW_CLEAR` reader"]
pub type R = crate::R<CMP1_SW_CLEAR_SPEC>;
#[doc = "Register `CMP1_SW_CLEAR` writer"]
pub type W = crate::W<CMP1_SW_CLEAR_SPEC>;
#[doc = "Field `CMP1_IP1` reader - see corresponding bit in CMP1_SW"]
pub type CMP1_IP1_R = crate::BitReader;
#[doc = "Field `CMP1_IP1` writer - see corresponding bit in CMP1_SW"]
pub type CMP1_IP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1_AP1` reader - see corresponding bit in CMP1_SW"]
pub type CMP1_AP1_R = crate::BitReader;
#[doc = "Field `CMP1_AP1` writer - see corresponding bit in CMP1_SW"]
pub type CMP1_AP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1_BP1` reader - see corresponding bit in CMP1_SW"]
pub type CMP1_BP1_R = crate::BitReader;
#[doc = "Field `CMP1_BP1` writer - see corresponding bit in CMP1_SW"]
pub type CMP1_BP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1_IN1` reader - see corresponding bit in CMP1_SW"]
pub type CMP1_IN1_R = crate::BitReader;
#[doc = "Field `CMP1_IN1` writer - see corresponding bit in CMP1_SW"]
pub type CMP1_IN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1_AN1` reader - see corresponding bit in CMP1_SW"]
pub type CMP1_AN1_R = crate::BitReader;
#[doc = "Field `CMP1_AN1` writer - see corresponding bit in CMP1_SW"]
pub type CMP1_AN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1_BN1` reader - see corresponding bit in CMP1_SW"]
pub type CMP1_BN1_R = crate::BitReader;
#[doc = "Field `CMP1_BN1` writer - see corresponding bit in CMP1_SW"]
pub type CMP1_BN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1_VN1` reader - see corresponding bit in CMP1_SW"]
pub type CMP1_VN1_R = crate::BitReader;
#[doc = "Field `CMP1_VN1` writer - see corresponding bit in CMP1_SW"]
pub type CMP1_VN1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - see corresponding bit in CMP1_SW"]
    #[inline(always)]
    pub fn cmp1_ip1(&self) -> CMP1_IP1_R {
        CMP1_IP1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - see corresponding bit in CMP1_SW"]
    #[inline(always)]
    pub fn cmp1_ap1(&self) -> CMP1_AP1_R {
        CMP1_AP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - see corresponding bit in CMP1_SW"]
    #[inline(always)]
    pub fn cmp1_bp1(&self) -> CMP1_BP1_R {
        CMP1_BP1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - see corresponding bit in CMP1_SW"]
    #[inline(always)]
    pub fn cmp1_in1(&self) -> CMP1_IN1_R {
        CMP1_IN1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - see corresponding bit in CMP1_SW"]
    #[inline(always)]
    pub fn cmp1_an1(&self) -> CMP1_AN1_R {
        CMP1_AN1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - see corresponding bit in CMP1_SW"]
    #[inline(always)]
    pub fn cmp1_bn1(&self) -> CMP1_BN1_R {
        CMP1_BN1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - see corresponding bit in CMP1_SW"]
    #[inline(always)]
    pub fn cmp1_vn1(&self) -> CMP1_VN1_R {
        CMP1_VN1_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - see corresponding bit in CMP1_SW"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1_ip1(&mut self) -> CMP1_IP1_W<CMP1_SW_CLEAR_SPEC> {
        CMP1_IP1_W::new(self, 0)
    }
    #[doc = "Bit 1 - see corresponding bit in CMP1_SW"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1_ap1(&mut self) -> CMP1_AP1_W<CMP1_SW_CLEAR_SPEC> {
        CMP1_AP1_W::new(self, 1)
    }
    #[doc = "Bit 2 - see corresponding bit in CMP1_SW"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1_bp1(&mut self) -> CMP1_BP1_W<CMP1_SW_CLEAR_SPEC> {
        CMP1_BP1_W::new(self, 2)
    }
    #[doc = "Bit 4 - see corresponding bit in CMP1_SW"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1_in1(&mut self) -> CMP1_IN1_W<CMP1_SW_CLEAR_SPEC> {
        CMP1_IN1_W::new(self, 4)
    }
    #[doc = "Bit 5 - see corresponding bit in CMP1_SW"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1_an1(&mut self) -> CMP1_AN1_W<CMP1_SW_CLEAR_SPEC> {
        CMP1_AN1_W::new(self, 5)
    }
    #[doc = "Bit 6 - see corresponding bit in CMP1_SW"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1_bn1(&mut self) -> CMP1_BN1_W<CMP1_SW_CLEAR_SPEC> {
        CMP1_BN1_W::new(self, 6)
    }
    #[doc = "Bit 7 - see corresponding bit in CMP1_SW"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1_vn1(&mut self) -> CMP1_VN1_W<CMP1_SW_CLEAR_SPEC> {
        CMP1_VN1_W::new(self, 7)
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
#[doc = "Comparator 1 switch control clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp1_sw_clear::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp1_sw_clear::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMP1_SW_CLEAR_SPEC;
impl crate::RegisterSpec for CMP1_SW_CLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp1_sw_clear::R`](R) reader structure"]
impl crate::Readable for CMP1_SW_CLEAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmp1_sw_clear::W`](W) writer structure"]
impl crate::Writable for CMP1_SW_CLEAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMP1_SW_CLEAR to value 0"]
impl crate::Resettable for CMP1_SW_CLEAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
