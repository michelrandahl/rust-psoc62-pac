#[doc = "Register `CMP0_SW_CLEAR` reader"]
pub type R = crate::R<CMP0_SW_CLEAR_SPEC>;
#[doc = "Register `CMP0_SW_CLEAR` writer"]
pub type W = crate::W<CMP0_SW_CLEAR_SPEC>;
#[doc = "Field `CMP0_IP0` reader - see corresponding bit in CMP0_SW"]
pub type CMP0_IP0_R = crate::BitReader;
#[doc = "Field `CMP0_IP0` writer - see corresponding bit in CMP0_SW"]
pub type CMP0_IP0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP0_AP0` reader - see corresponding bit in CMP0_SW"]
pub type CMP0_AP0_R = crate::BitReader;
#[doc = "Field `CMP0_AP0` writer - see corresponding bit in CMP0_SW"]
pub type CMP0_AP0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP0_BP0` reader - see corresponding bit in CMP0_SW"]
pub type CMP0_BP0_R = crate::BitReader;
#[doc = "Field `CMP0_BP0` writer - see corresponding bit in CMP0_SW"]
pub type CMP0_BP0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP0_IN0` reader - see corresponding bit in CMP0_SW"]
pub type CMP0_IN0_R = crate::BitReader;
#[doc = "Field `CMP0_IN0` writer - see corresponding bit in CMP0_SW"]
pub type CMP0_IN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP0_AN0` reader - see corresponding bit in CMP0_SW"]
pub type CMP0_AN0_R = crate::BitReader;
#[doc = "Field `CMP0_AN0` writer - see corresponding bit in CMP0_SW"]
pub type CMP0_AN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP0_BN0` reader - see corresponding bit in CMP0_SW"]
pub type CMP0_BN0_R = crate::BitReader;
#[doc = "Field `CMP0_BN0` writer - see corresponding bit in CMP0_SW"]
pub type CMP0_BN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP0_VN0` reader - see corresponding bit in CMP0_SW"]
pub type CMP0_VN0_R = crate::BitReader;
#[doc = "Field `CMP0_VN0` writer - see corresponding bit in CMP0_SW"]
pub type CMP0_VN0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub fn cmp0_ip0(&self) -> CMP0_IP0_R {
        CMP0_IP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub fn cmp0_ap0(&self) -> CMP0_AP0_R {
        CMP0_AP0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub fn cmp0_bp0(&self) -> CMP0_BP0_R {
        CMP0_BP0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub fn cmp0_in0(&self) -> CMP0_IN0_R {
        CMP0_IN0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub fn cmp0_an0(&self) -> CMP0_AN0_R {
        CMP0_AN0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub fn cmp0_bn0(&self) -> CMP0_BN0_R {
        CMP0_BN0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub fn cmp0_vn0(&self) -> CMP0_VN0_R {
        CMP0_VN0_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0_ip0(&mut self) -> CMP0_IP0_W<CMP0_SW_CLEAR_SPEC> {
        CMP0_IP0_W::new(self, 0)
    }
    #[doc = "Bit 1 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0_ap0(&mut self) -> CMP0_AP0_W<CMP0_SW_CLEAR_SPEC> {
        CMP0_AP0_W::new(self, 1)
    }
    #[doc = "Bit 2 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0_bp0(&mut self) -> CMP0_BP0_W<CMP0_SW_CLEAR_SPEC> {
        CMP0_BP0_W::new(self, 2)
    }
    #[doc = "Bit 4 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0_in0(&mut self) -> CMP0_IN0_W<CMP0_SW_CLEAR_SPEC> {
        CMP0_IN0_W::new(self, 4)
    }
    #[doc = "Bit 5 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0_an0(&mut self) -> CMP0_AN0_W<CMP0_SW_CLEAR_SPEC> {
        CMP0_AN0_W::new(self, 5)
    }
    #[doc = "Bit 6 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0_bn0(&mut self) -> CMP0_BN0_W<CMP0_SW_CLEAR_SPEC> {
        CMP0_BN0_W::new(self, 6)
    }
    #[doc = "Bit 7 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0_vn0(&mut self) -> CMP0_VN0_W<CMP0_SW_CLEAR_SPEC> {
        CMP0_VN0_W::new(self, 7)
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
#[doc = "Comparator 0 switch control clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp0_sw_clear::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp0_sw_clear::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMP0_SW_CLEAR_SPEC;
impl crate::RegisterSpec for CMP0_SW_CLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp0_sw_clear::R`](R) reader structure"]
impl crate::Readable for CMP0_SW_CLEAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmp0_sw_clear::W`](W) writer structure"]
impl crate::Writable for CMP0_SW_CLEAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMP0_SW_CLEAR to value 0"]
impl crate::Resettable for CMP0_SW_CLEAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
