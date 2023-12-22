#[doc = "Register `CLK_PATH_SELECT[%s]` reader"]
pub type R = crate::R<CLK_PATH_SELECT_SPEC>;
#[doc = "Register `CLK_PATH_SELECT[%s]` writer"]
pub type W = crate::W<CLK_PATH_SELECT_SPEC>;
#[doc = "Field `PATH_MUX` reader - Selects a source for clock PATH&lt;i>. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior."]
pub type PATH_MUX_R = crate::FieldReader<PATH_MUX_A>;
#[doc = "Selects a source for clock PATH&lt;i>. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PATH_MUX_A {
    #[doc = "0: IMO - Internal R/C Oscillator"]
    IMO = 0,
    #[doc = "1: EXTCLK - External Clock Pin"]
    EXTCLK = 1,
    #[doc = "2: ECO - External-Crystal Oscillator"]
    ECO = 2,
    #[doc = "3: ALTHF - Alternate High-Frequency clock input (product-specific clock)"]
    ALTHF = 3,
    #[doc = "4: DSI_MUX - Output of DSI mux for this path. Using a DSI source directly as root of HFCLK will result in undefined behavior."]
    DSI_MUX = 4,
}
impl From<PATH_MUX_A> for u8 {
    #[inline(always)]
    fn from(variant: PATH_MUX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PATH_MUX_A {
    type Ux = u8;
}
impl PATH_MUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PATH_MUX_A> {
        match self.bits {
            0 => Some(PATH_MUX_A::IMO),
            1 => Some(PATH_MUX_A::EXTCLK),
            2 => Some(PATH_MUX_A::ECO),
            3 => Some(PATH_MUX_A::ALTHF),
            4 => Some(PATH_MUX_A::DSI_MUX),
            _ => None,
        }
    }
    #[doc = "IMO - Internal R/C Oscillator"]
    #[inline(always)]
    pub fn is_imo(&self) -> bool {
        *self == PATH_MUX_A::IMO
    }
    #[doc = "EXTCLK - External Clock Pin"]
    #[inline(always)]
    pub fn is_extclk(&self) -> bool {
        *self == PATH_MUX_A::EXTCLK
    }
    #[doc = "ECO - External-Crystal Oscillator"]
    #[inline(always)]
    pub fn is_eco(&self) -> bool {
        *self == PATH_MUX_A::ECO
    }
    #[doc = "ALTHF - Alternate High-Frequency clock input (product-specific clock)"]
    #[inline(always)]
    pub fn is_althf(&self) -> bool {
        *self == PATH_MUX_A::ALTHF
    }
    #[doc = "DSI_MUX - Output of DSI mux for this path. Using a DSI source directly as root of HFCLK will result in undefined behavior."]
    #[inline(always)]
    pub fn is_dsi_mux(&self) -> bool {
        *self == PATH_MUX_A::DSI_MUX
    }
}
#[doc = "Field `PATH_MUX` writer - Selects a source for clock PATH&lt;i>. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior."]
pub type PATH_MUX_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PATH_MUX_A>;
impl<'a, REG> PATH_MUX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IMO - Internal R/C Oscillator"]
    #[inline(always)]
    pub fn imo(self) -> &'a mut crate::W<REG> {
        self.variant(PATH_MUX_A::IMO)
    }
    #[doc = "EXTCLK - External Clock Pin"]
    #[inline(always)]
    pub fn extclk(self) -> &'a mut crate::W<REG> {
        self.variant(PATH_MUX_A::EXTCLK)
    }
    #[doc = "ECO - External-Crystal Oscillator"]
    #[inline(always)]
    pub fn eco(self) -> &'a mut crate::W<REG> {
        self.variant(PATH_MUX_A::ECO)
    }
    #[doc = "ALTHF - Alternate High-Frequency clock input (product-specific clock)"]
    #[inline(always)]
    pub fn althf(self) -> &'a mut crate::W<REG> {
        self.variant(PATH_MUX_A::ALTHF)
    }
    #[doc = "DSI_MUX - Output of DSI mux for this path. Using a DSI source directly as root of HFCLK will result in undefined behavior."]
    #[inline(always)]
    pub fn dsi_mux(self) -> &'a mut crate::W<REG> {
        self.variant(PATH_MUX_A::DSI_MUX)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects a source for clock PATH&lt;i>. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior."]
    #[inline(always)]
    pub fn path_mux(&self) -> PATH_MUX_R {
        PATH_MUX_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects a source for clock PATH&lt;i>. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn path_mux(&mut self) -> PATH_MUX_W<CLK_PATH_SELECT_SPEC> {
        PATH_MUX_W::new(self, 0)
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
#[doc = "Clock Path Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_path_select::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_path_select::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_PATH_SELECT_SPEC;
impl crate::RegisterSpec for CLK_PATH_SELECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_path_select::R`](R) reader structure"]
impl crate::Readable for CLK_PATH_SELECT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_path_select::W`](W) writer structure"]
impl crate::Writable for CLK_PATH_SELECT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_PATH_SELECT[%s]
to value 0"]
impl crate::Resettable for CLK_PATH_SELECT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
