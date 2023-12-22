#[doc = "Register `CFG_OUT` reader"]
pub type R = crate::R<CFG_OUT_SPEC>;
#[doc = "Register `CFG_OUT` writer"]
pub type W = crate::W<CFG_OUT_SPEC>;
#[doc = "Field `SLOW0` reader - Enables slow slew rate for IO pin 0 '0': Fast slew rate '1': Slow slew rate"]
pub type SLOW0_R = crate::BitReader;
#[doc = "Field `SLOW0` writer - Enables slow slew rate for IO pin 0 '0': Fast slew rate '1': Slow slew rate"]
pub type SLOW0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOW1` reader - Enables slow slew rate for IO pin 1"]
pub type SLOW1_R = crate::BitReader;
#[doc = "Field `SLOW1` writer - Enables slow slew rate for IO pin 1"]
pub type SLOW1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOW2` reader - Enables slow slew rate for IO pin 2"]
pub type SLOW2_R = crate::BitReader;
#[doc = "Field `SLOW2` writer - Enables slow slew rate for IO pin 2"]
pub type SLOW2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOW3` reader - Enables slow slew rate for IO pin 3"]
pub type SLOW3_R = crate::BitReader;
#[doc = "Field `SLOW3` writer - Enables slow slew rate for IO pin 3"]
pub type SLOW3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOW4` reader - Enables slow slew rate for IO pin 4"]
pub type SLOW4_R = crate::BitReader;
#[doc = "Field `SLOW4` writer - Enables slow slew rate for IO pin 4"]
pub type SLOW4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOW5` reader - Enables slow slew rate for IO pin 5"]
pub type SLOW5_R = crate::BitReader;
#[doc = "Field `SLOW5` writer - Enables slow slew rate for IO pin 5"]
pub type SLOW5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOW6` reader - Enables slow slew rate for IO pin 6"]
pub type SLOW6_R = crate::BitReader;
#[doc = "Field `SLOW6` writer - Enables slow slew rate for IO pin 6"]
pub type SLOW6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOW7` reader - Enables slow slew rate for IO pin 7"]
pub type SLOW7_R = crate::BitReader;
#[doc = "Field `SLOW7` writer - Enables slow slew rate for IO pin 7"]
pub type SLOW7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRIVE_SEL0` reader - Sets the GPIO drive strength for IO pin 0"]
pub type DRIVE_SEL0_R = crate::FieldReader<DRIVE_SEL0_A>;
#[doc = "Sets the GPIO drive strength for IO pin 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DRIVE_SEL0_A {
    #[doc = "0: N/A"]
    DRIVE_SEL_ZERO = 0,
    #[doc = "1: N/A"]
    DRIVE_SEL_ONE = 1,
    #[doc = "2: N/A"]
    DRIVE_SEL_TWO = 2,
    #[doc = "3: N/A"]
    DRIVE_SEL_THREE = 3,
}
impl From<DRIVE_SEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: DRIVE_SEL0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DRIVE_SEL0_A {
    type Ux = u8;
}
impl DRIVE_SEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DRIVE_SEL0_A {
        match self.bits {
            0 => DRIVE_SEL0_A::DRIVE_SEL_ZERO,
            1 => DRIVE_SEL0_A::DRIVE_SEL_ONE,
            2 => DRIVE_SEL0_A::DRIVE_SEL_TWO,
            3 => DRIVE_SEL0_A::DRIVE_SEL_THREE,
            _ => unreachable!(),
        }
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_drive_sel_zero(&self) -> bool {
        *self == DRIVE_SEL0_A::DRIVE_SEL_ZERO
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_drive_sel_one(&self) -> bool {
        *self == DRIVE_SEL0_A::DRIVE_SEL_ONE
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_drive_sel_two(&self) -> bool {
        *self == DRIVE_SEL0_A::DRIVE_SEL_TWO
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_drive_sel_three(&self) -> bool {
        *self == DRIVE_SEL0_A::DRIVE_SEL_THREE
    }
}
#[doc = "Field `DRIVE_SEL0` writer - Sets the GPIO drive strength for IO pin 0"]
pub type DRIVE_SEL0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DRIVE_SEL0_A>;
impl<'a, REG> DRIVE_SEL0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "N/A"]
    #[inline(always)]
    pub fn drive_sel_zero(self) -> &'a mut crate::W<REG> {
        self.variant(DRIVE_SEL0_A::DRIVE_SEL_ZERO)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn drive_sel_one(self) -> &'a mut crate::W<REG> {
        self.variant(DRIVE_SEL0_A::DRIVE_SEL_ONE)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn drive_sel_two(self) -> &'a mut crate::W<REG> {
        self.variant(DRIVE_SEL0_A::DRIVE_SEL_TWO)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn drive_sel_three(self) -> &'a mut crate::W<REG> {
        self.variant(DRIVE_SEL0_A::DRIVE_SEL_THREE)
    }
}
#[doc = "Field `DRIVE_SEL1` reader - Sets the GPIO drive strength for IO pin 1"]
pub type DRIVE_SEL1_R = crate::FieldReader;
#[doc = "Field `DRIVE_SEL1` writer - Sets the GPIO drive strength for IO pin 1"]
pub type DRIVE_SEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DRIVE_SEL2` reader - Sets the GPIO drive strength for IO pin 2"]
pub type DRIVE_SEL2_R = crate::FieldReader;
#[doc = "Field `DRIVE_SEL2` writer - Sets the GPIO drive strength for IO pin 2"]
pub type DRIVE_SEL2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DRIVE_SEL3` reader - Sets the GPIO drive strength for IO pin 3"]
pub type DRIVE_SEL3_R = crate::FieldReader;
#[doc = "Field `DRIVE_SEL3` writer - Sets the GPIO drive strength for IO pin 3"]
pub type DRIVE_SEL3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DRIVE_SEL4` reader - Sets the GPIO drive strength for IO pin 4"]
pub type DRIVE_SEL4_R = crate::FieldReader;
#[doc = "Field `DRIVE_SEL4` writer - Sets the GPIO drive strength for IO pin 4"]
pub type DRIVE_SEL4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DRIVE_SEL5` reader - Sets the GPIO drive strength for IO pin 5"]
pub type DRIVE_SEL5_R = crate::FieldReader;
#[doc = "Field `DRIVE_SEL5` writer - Sets the GPIO drive strength for IO pin 5"]
pub type DRIVE_SEL5_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DRIVE_SEL6` reader - Sets the GPIO drive strength for IO pin 6"]
pub type DRIVE_SEL6_R = crate::FieldReader;
#[doc = "Field `DRIVE_SEL6` writer - Sets the GPIO drive strength for IO pin 6"]
pub type DRIVE_SEL6_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DRIVE_SEL7` reader - Sets the GPIO drive strength for IO pin 7"]
pub type DRIVE_SEL7_R = crate::FieldReader;
#[doc = "Field `DRIVE_SEL7` writer - Sets the GPIO drive strength for IO pin 7"]
pub type DRIVE_SEL7_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Enables slow slew rate for IO pin 0 '0': Fast slew rate '1': Slow slew rate"]
    #[inline(always)]
    pub fn slow0(&self) -> SLOW0_R {
        SLOW0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables slow slew rate for IO pin 1"]
    #[inline(always)]
    pub fn slow1(&self) -> SLOW1_R {
        SLOW1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables slow slew rate for IO pin 2"]
    #[inline(always)]
    pub fn slow2(&self) -> SLOW2_R {
        SLOW2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables slow slew rate for IO pin 3"]
    #[inline(always)]
    pub fn slow3(&self) -> SLOW3_R {
        SLOW3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables slow slew rate for IO pin 4"]
    #[inline(always)]
    pub fn slow4(&self) -> SLOW4_R {
        SLOW4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enables slow slew rate for IO pin 5"]
    #[inline(always)]
    pub fn slow5(&self) -> SLOW5_R {
        SLOW5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enables slow slew rate for IO pin 6"]
    #[inline(always)]
    pub fn slow6(&self) -> SLOW6_R {
        SLOW6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enables slow slew rate for IO pin 7"]
    #[inline(always)]
    pub fn slow7(&self) -> SLOW7_R {
        SLOW7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Sets the GPIO drive strength for IO pin 0"]
    #[inline(always)]
    pub fn drive_sel0(&self) -> DRIVE_SEL0_R {
        DRIVE_SEL0_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Sets the GPIO drive strength for IO pin 1"]
    #[inline(always)]
    pub fn drive_sel1(&self) -> DRIVE_SEL1_R {
        DRIVE_SEL1_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Sets the GPIO drive strength for IO pin 2"]
    #[inline(always)]
    pub fn drive_sel2(&self) -> DRIVE_SEL2_R {
        DRIVE_SEL2_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Sets the GPIO drive strength for IO pin 3"]
    #[inline(always)]
    pub fn drive_sel3(&self) -> DRIVE_SEL3_R {
        DRIVE_SEL3_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Sets the GPIO drive strength for IO pin 4"]
    #[inline(always)]
    pub fn drive_sel4(&self) -> DRIVE_SEL4_R {
        DRIVE_SEL4_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Sets the GPIO drive strength for IO pin 5"]
    #[inline(always)]
    pub fn drive_sel5(&self) -> DRIVE_SEL5_R {
        DRIVE_SEL5_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Sets the GPIO drive strength for IO pin 6"]
    #[inline(always)]
    pub fn drive_sel6(&self) -> DRIVE_SEL6_R {
        DRIVE_SEL6_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Sets the GPIO drive strength for IO pin 7"]
    #[inline(always)]
    pub fn drive_sel7(&self) -> DRIVE_SEL7_R {
        DRIVE_SEL7_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enables slow slew rate for IO pin 0 '0': Fast slew rate '1': Slow slew rate"]
    #[inline(always)]
    #[must_use]
    pub fn slow0(&mut self) -> SLOW0_W<CFG_OUT_SPEC> {
        SLOW0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enables slow slew rate for IO pin 1"]
    #[inline(always)]
    #[must_use]
    pub fn slow1(&mut self) -> SLOW1_W<CFG_OUT_SPEC> {
        SLOW1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enables slow slew rate for IO pin 2"]
    #[inline(always)]
    #[must_use]
    pub fn slow2(&mut self) -> SLOW2_W<CFG_OUT_SPEC> {
        SLOW2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Enables slow slew rate for IO pin 3"]
    #[inline(always)]
    #[must_use]
    pub fn slow3(&mut self) -> SLOW3_W<CFG_OUT_SPEC> {
        SLOW3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Enables slow slew rate for IO pin 4"]
    #[inline(always)]
    #[must_use]
    pub fn slow4(&mut self) -> SLOW4_W<CFG_OUT_SPEC> {
        SLOW4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Enables slow slew rate for IO pin 5"]
    #[inline(always)]
    #[must_use]
    pub fn slow5(&mut self) -> SLOW5_W<CFG_OUT_SPEC> {
        SLOW5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Enables slow slew rate for IO pin 6"]
    #[inline(always)]
    #[must_use]
    pub fn slow6(&mut self) -> SLOW6_W<CFG_OUT_SPEC> {
        SLOW6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Enables slow slew rate for IO pin 7"]
    #[inline(always)]
    #[must_use]
    pub fn slow7(&mut self) -> SLOW7_W<CFG_OUT_SPEC> {
        SLOW7_W::new(self, 7)
    }
    #[doc = "Bits 16:17 - Sets the GPIO drive strength for IO pin 0"]
    #[inline(always)]
    #[must_use]
    pub fn drive_sel0(&mut self) -> DRIVE_SEL0_W<CFG_OUT_SPEC> {
        DRIVE_SEL0_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Sets the GPIO drive strength for IO pin 1"]
    #[inline(always)]
    #[must_use]
    pub fn drive_sel1(&mut self) -> DRIVE_SEL1_W<CFG_OUT_SPEC> {
        DRIVE_SEL1_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Sets the GPIO drive strength for IO pin 2"]
    #[inline(always)]
    #[must_use]
    pub fn drive_sel2(&mut self) -> DRIVE_SEL2_W<CFG_OUT_SPEC> {
        DRIVE_SEL2_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Sets the GPIO drive strength for IO pin 3"]
    #[inline(always)]
    #[must_use]
    pub fn drive_sel3(&mut self) -> DRIVE_SEL3_W<CFG_OUT_SPEC> {
        DRIVE_SEL3_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Sets the GPIO drive strength for IO pin 4"]
    #[inline(always)]
    #[must_use]
    pub fn drive_sel4(&mut self) -> DRIVE_SEL4_W<CFG_OUT_SPEC> {
        DRIVE_SEL4_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Sets the GPIO drive strength for IO pin 5"]
    #[inline(always)]
    #[must_use]
    pub fn drive_sel5(&mut self) -> DRIVE_SEL5_W<CFG_OUT_SPEC> {
        DRIVE_SEL5_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Sets the GPIO drive strength for IO pin 6"]
    #[inline(always)]
    #[must_use]
    pub fn drive_sel6(&mut self) -> DRIVE_SEL6_W<CFG_OUT_SPEC> {
        DRIVE_SEL6_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Sets the GPIO drive strength for IO pin 7"]
    #[inline(always)]
    #[must_use]
    pub fn drive_sel7(&mut self) -> DRIVE_SEL7_W<CFG_OUT_SPEC> {
        DRIVE_SEL7_W::new(self, 30)
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
#[doc = "Port output buffer configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_out::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_out::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_OUT_SPEC;
impl crate::RegisterSpec for CFG_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_out::R`](R) reader structure"]
impl crate::Readable for CFG_OUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg_out::W`](W) writer structure"]
impl crate::Writable for CFG_OUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG_OUT to value 0"]
impl crate::Resettable for CFG_OUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
