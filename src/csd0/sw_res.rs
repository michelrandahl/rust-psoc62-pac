#[doc = "Register `SW_RES` reader"]
pub type R = crate::R<SW_RES_SPEC>;
#[doc = "Register `SW_RES` writer"]
pub type W = crate::W<SW_RES_SPEC>;
#[doc = "Field `RES_HCAV` reader - Select resistance or low EMI (slow ramp) for the HCAV switch"]
pub type RES_HCAV_R = crate::FieldReader<RES_HCAV_A>;
#[doc = "Select resistance or low EMI (slow ramp) for the HCAV switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RES_HCAV_A {
    #[doc = "0: Low"]
    LOW = 0,
    #[doc = "1: Medium"]
    MED = 1,
    #[doc = "2: High"]
    HIGH = 2,
    #[doc = "3: Low EMI (slow ramp: 3 switches closed by fixed delay line)"]
    LOWEMI = 3,
}
impl From<RES_HCAV_A> for u8 {
    #[inline(always)]
    fn from(variant: RES_HCAV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RES_HCAV_A {
    type Ux = u8;
}
impl RES_HCAV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RES_HCAV_A {
        match self.bits {
            0 => RES_HCAV_A::LOW,
            1 => RES_HCAV_A::MED,
            2 => RES_HCAV_A::HIGH,
            3 => RES_HCAV_A::LOWEMI,
            _ => unreachable!(),
        }
    }
    #[doc = "Low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == RES_HCAV_A::LOW
    }
    #[doc = "Medium"]
    #[inline(always)]
    pub fn is_med(&self) -> bool {
        *self == RES_HCAV_A::MED
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == RES_HCAV_A::HIGH
    }
    #[doc = "Low EMI (slow ramp: 3 switches closed by fixed delay line)"]
    #[inline(always)]
    pub fn is_lowemi(&self) -> bool {
        *self == RES_HCAV_A::LOWEMI
    }
}
#[doc = "Field `RES_HCAV` writer - Select resistance or low EMI (slow ramp) for the HCAV switch"]
pub type RES_HCAV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, RES_HCAV_A>;
impl<'a, REG> RES_HCAV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(RES_HCAV_A::LOW)
    }
    #[doc = "Medium"]
    #[inline(always)]
    pub fn med(self) -> &'a mut crate::W<REG> {
        self.variant(RES_HCAV_A::MED)
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(RES_HCAV_A::HIGH)
    }
    #[doc = "Low EMI (slow ramp: 3 switches closed by fixed delay line)"]
    #[inline(always)]
    pub fn lowemi(self) -> &'a mut crate::W<REG> {
        self.variant(RES_HCAV_A::LOWEMI)
    }
}
#[doc = "Field `RES_HCAG` reader - Select resistance or low EMI for the corresponding switch"]
pub type RES_HCAG_R = crate::FieldReader;
#[doc = "Field `RES_HCAG` writer - Select resistance or low EMI for the corresponding switch"]
pub type RES_HCAG_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RES_HCBV` reader - Select resistance or low EMI for the corresponding switch"]
pub type RES_HCBV_R = crate::FieldReader;
#[doc = "Field `RES_HCBV` writer - Select resistance or low EMI for the corresponding switch"]
pub type RES_HCBV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RES_HCBG` reader - Select resistance or low EMI for the corresponding switch"]
pub type RES_HCBG_R = crate::FieldReader;
#[doc = "Field `RES_HCBG` writer - Select resistance or low EMI for the corresponding switch"]
pub type RES_HCBG_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RES_F1PM` reader - Select resistance for the corresponding switch"]
pub type RES_F1PM_R = crate::FieldReader<RES_F1PM_A>;
#[doc = "Select resistance for the corresponding switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RES_F1PM_A {
    #[doc = "0: Low"]
    LOW = 0,
    #[doc = "1: Medium"]
    MED = 1,
    #[doc = "2: High"]
    HIGH = 2,
    #[doc = "3: N/A"]
    RSVD = 3,
}
impl From<RES_F1PM_A> for u8 {
    #[inline(always)]
    fn from(variant: RES_F1PM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RES_F1PM_A {
    type Ux = u8;
}
impl RES_F1PM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RES_F1PM_A {
        match self.bits {
            0 => RES_F1PM_A::LOW,
            1 => RES_F1PM_A::MED,
            2 => RES_F1PM_A::HIGH,
            3 => RES_F1PM_A::RSVD,
            _ => unreachable!(),
        }
    }
    #[doc = "Low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == RES_F1PM_A::LOW
    }
    #[doc = "Medium"]
    #[inline(always)]
    pub fn is_med(&self) -> bool {
        *self == RES_F1PM_A::MED
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == RES_F1PM_A::HIGH
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_rsvd(&self) -> bool {
        *self == RES_F1PM_A::RSVD
    }
}
#[doc = "Field `RES_F1PM` writer - Select resistance for the corresponding switch"]
pub type RES_F1PM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, RES_F1PM_A>;
impl<'a, REG> RES_F1PM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(RES_F1PM_A::LOW)
    }
    #[doc = "Medium"]
    #[inline(always)]
    pub fn med(self) -> &'a mut crate::W<REG> {
        self.variant(RES_F1PM_A::MED)
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(RES_F1PM_A::HIGH)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd(self) -> &'a mut crate::W<REG> {
        self.variant(RES_F1PM_A::RSVD)
    }
}
#[doc = "Field `RES_F2PT` reader - Select resistance for the corresponding switch"]
pub type RES_F2PT_R = crate::FieldReader;
#[doc = "Field `RES_F2PT` writer - Select resistance for the corresponding switch"]
pub type RES_F2PT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Select resistance or low EMI (slow ramp) for the HCAV switch"]
    #[inline(always)]
    pub fn res_hcav(&self) -> RES_HCAV_R {
        RES_HCAV_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Select resistance or low EMI for the corresponding switch"]
    #[inline(always)]
    pub fn res_hcag(&self) -> RES_HCAG_R {
        RES_HCAG_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Select resistance or low EMI for the corresponding switch"]
    #[inline(always)]
    pub fn res_hcbv(&self) -> RES_HCBV_R {
        RES_HCBV_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Select resistance or low EMI for the corresponding switch"]
    #[inline(always)]
    pub fn res_hcbg(&self) -> RES_HCBG_R {
        RES_HCBG_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Select resistance for the corresponding switch"]
    #[inline(always)]
    pub fn res_f1pm(&self) -> RES_F1PM_R {
        RES_F1PM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Select resistance for the corresponding switch"]
    #[inline(always)]
    pub fn res_f2pt(&self) -> RES_F2PT_R {
        RES_F2PT_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Select resistance or low EMI (slow ramp) for the HCAV switch"]
    #[inline(always)]
    #[must_use]
    pub fn res_hcav(&mut self) -> RES_HCAV_W<SW_RES_SPEC> {
        RES_HCAV_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Select resistance or low EMI for the corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn res_hcag(&mut self) -> RES_HCAG_W<SW_RES_SPEC> {
        RES_HCAG_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Select resistance or low EMI for the corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn res_hcbv(&mut self) -> RES_HCBV_W<SW_RES_SPEC> {
        RES_HCBV_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Select resistance or low EMI for the corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn res_hcbg(&mut self) -> RES_HCBG_W<SW_RES_SPEC> {
        RES_HCBG_W::new(self, 6)
    }
    #[doc = "Bits 16:17 - Select resistance for the corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn res_f1pm(&mut self) -> RES_F1PM_W<SW_RES_SPEC> {
        RES_F1PM_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Select resistance for the corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn res_f2pt(&mut self) -> RES_F2PT_W<SW_RES_SPEC> {
        RES_F2PT_W::new(self, 18)
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
#[doc = "Switch Resistance configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_res::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_res::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SW_RES_SPEC;
impl crate::RegisterSpec for SW_RES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sw_res::R`](R) reader structure"]
impl crate::Readable for SW_RES_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sw_res::W`](W) writer structure"]
impl crate::Writable for SW_RES_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SW_RES to value 0"]
impl crate::Resettable for SW_RES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
