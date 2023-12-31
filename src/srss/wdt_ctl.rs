#[doc = "Register `WDT_CTL` reader"]
pub type R = crate::R<WDT_CTL_SPEC>;
#[doc = "Register `WDT_CTL` writer"]
pub type W = crate::W<WDT_CTL_SPEC>;
#[doc = "Field `WDT_EN` reader - Enable this watchdog timer. This field is retained during DEEPSLEEP and HIBERNATE modes."]
pub type WDT_EN_R = crate::BitReader;
#[doc = "Field `WDT_EN` writer - Enable this watchdog timer. This field is retained during DEEPSLEEP and HIBERNATE modes."]
pub type WDT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_LOCK` reader - Prohibits writing to WDT_*, CLK_ILO_CONFIG, CLK_SELECT.LFCLK_SEL, and CLK_TRIM_ILO_CTL registers when not equal 0. Requires at least two different writes to unlock. A change in WDT_LOCK takes effect beginning with the next write cycle. Note that this field is 2 bits to force multiple writes only. It represents only a single write protect signal protecting all those registers at the same time. WDT will lock on any reset. This field is not retained during DEEPSLEEP or HIBERNATE mode, so the WDT will be locked after wakeup from these modes."]
pub type WDT_LOCK_R = crate::FieldReader<WDT_LOCK_A>;
#[doc = "Prohibits writing to WDT_*, CLK_ILO_CONFIG, CLK_SELECT.LFCLK_SEL, and CLK_TRIM_ILO_CTL registers when not equal 0. Requires at least two different writes to unlock. A change in WDT_LOCK takes effect beginning with the next write cycle. Note that this field is 2 bits to force multiple writes only. It represents only a single write protect signal protecting all those registers at the same time. WDT will lock on any reset. This field is not retained during DEEPSLEEP or HIBERNATE mode, so the WDT will be locked after wakeup from these modes.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDT_LOCK_A {
    #[doc = "0: No effect"]
    NO_CHG = 0,
    #[doc = "1: Clears bit 0"]
    CLR0 = 1,
    #[doc = "2: Clears bit 1"]
    CLR1 = 2,
    #[doc = "3: Sets both bits 0 and 1"]
    SET01 = 3,
}
impl From<WDT_LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: WDT_LOCK_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WDT_LOCK_A {
    type Ux = u8;
}
impl WDT_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WDT_LOCK_A {
        match self.bits {
            0 => WDT_LOCK_A::NO_CHG,
            1 => WDT_LOCK_A::CLR0,
            2 => WDT_LOCK_A::CLR1,
            3 => WDT_LOCK_A::SET01,
            _ => unreachable!(),
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_chg(&self) -> bool {
        *self == WDT_LOCK_A::NO_CHG
    }
    #[doc = "Clears bit 0"]
    #[inline(always)]
    pub fn is_clr0(&self) -> bool {
        *self == WDT_LOCK_A::CLR0
    }
    #[doc = "Clears bit 1"]
    #[inline(always)]
    pub fn is_clr1(&self) -> bool {
        *self == WDT_LOCK_A::CLR1
    }
    #[doc = "Sets both bits 0 and 1"]
    #[inline(always)]
    pub fn is_set01(&self) -> bool {
        *self == WDT_LOCK_A::SET01
    }
}
#[doc = "Field `WDT_LOCK` writer - Prohibits writing to WDT_*, CLK_ILO_CONFIG, CLK_SELECT.LFCLK_SEL, and CLK_TRIM_ILO_CTL registers when not equal 0. Requires at least two different writes to unlock. A change in WDT_LOCK takes effect beginning with the next write cycle. Note that this field is 2 bits to force multiple writes only. It represents only a single write protect signal protecting all those registers at the same time. WDT will lock on any reset. This field is not retained during DEEPSLEEP or HIBERNATE mode, so the WDT will be locked after wakeup from these modes."]
pub type WDT_LOCK_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, WDT_LOCK_A>;
impl<'a, REG> WDT_LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_chg(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_LOCK_A::NO_CHG)
    }
    #[doc = "Clears bit 0"]
    #[inline(always)]
    pub fn clr0(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_LOCK_A::CLR0)
    }
    #[doc = "Clears bit 1"]
    #[inline(always)]
    pub fn clr1(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_LOCK_A::CLR1)
    }
    #[doc = "Sets both bits 0 and 1"]
    #[inline(always)]
    pub fn set01(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_LOCK_A::SET01)
    }
}
impl R {
    #[doc = "Bit 0 - Enable this watchdog timer. This field is retained during DEEPSLEEP and HIBERNATE modes."]
    #[inline(always)]
    pub fn wdt_en(&self) -> WDT_EN_R {
        WDT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 30:31 - Prohibits writing to WDT_*, CLK_ILO_CONFIG, CLK_SELECT.LFCLK_SEL, and CLK_TRIM_ILO_CTL registers when not equal 0. Requires at least two different writes to unlock. A change in WDT_LOCK takes effect beginning with the next write cycle. Note that this field is 2 bits to force multiple writes only. It represents only a single write protect signal protecting all those registers at the same time. WDT will lock on any reset. This field is not retained during DEEPSLEEP or HIBERNATE mode, so the WDT will be locked after wakeup from these modes."]
    #[inline(always)]
    pub fn wdt_lock(&self) -> WDT_LOCK_R {
        WDT_LOCK_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable this watchdog timer. This field is retained during DEEPSLEEP and HIBERNATE modes."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_en(&mut self) -> WDT_EN_W<WDT_CTL_SPEC> {
        WDT_EN_W::new(self, 0)
    }
    #[doc = "Bits 30:31 - Prohibits writing to WDT_*, CLK_ILO_CONFIG, CLK_SELECT.LFCLK_SEL, and CLK_TRIM_ILO_CTL registers when not equal 0. Requires at least two different writes to unlock. A change in WDT_LOCK takes effect beginning with the next write cycle. Note that this field is 2 bits to force multiple writes only. It represents only a single write protect signal protecting all those registers at the same time. WDT will lock on any reset. This field is not retained during DEEPSLEEP or HIBERNATE mode, so the WDT will be locked after wakeup from these modes."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_lock(&mut self) -> WDT_LOCK_W<WDT_CTL_SPEC> {
        WDT_LOCK_W::new(self, 30)
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
#[doc = "Watchdog Counter Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdt_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdt_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDT_CTL_SPEC;
impl crate::RegisterSpec for WDT_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt_ctl::R`](R) reader structure"]
impl crate::Readable for WDT_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdt_ctl::W`](W) writer structure"]
impl crate::Writable for WDT_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDT_CTL to value 0xc000_0001"]
impl crate::Resettable for WDT_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0xc000_0001;
}
