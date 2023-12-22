#[doc = "Register `CLK_ROOT_SELECT[%s]` reader"]
pub type R = crate::R<CLK_ROOT_SELECT_SPEC>;
#[doc = "Register `CLK_ROOT_SELECT[%s]` writer"]
pub type W = crate::W<CLK_ROOT_SELECT_SPEC>;
#[doc = "Field `ROOT_MUX` reader - Selects a clock path as the root of HFCLK&lt;k> and for SRSS DSI input &lt;k>. Use CLK_SELECT_PATH\\[i\\]
to configure the desired path. Some paths may have FLL or PLL available (product-specific), and the control and bypass mux selections of these are in other registers. Configure the FLL using CLK_FLL_CONFIG register. Configure a PLL using the related CLK_PLL_CONFIG\\[k\\]
register. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior."]
pub type ROOT_MUX_R = crate::FieldReader<ROOT_MUX_A>;
#[doc = "Selects a clock path as the root of HFCLK&lt;k> and for SRSS DSI input &lt;k>. Use CLK_SELECT_PATH\\[i\\]
to configure the desired path. Some paths may have FLL or PLL available (product-specific), and the control and bypass mux selections of these are in other registers. Configure the FLL using CLK_FLL_CONFIG register. Configure a PLL using the related CLK_PLL_CONFIG\\[k\\]
register. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ROOT_MUX_A {
    #[doc = "0: Select PATH0 (can be configured for FLL)"]
    PATH0 = 0,
    #[doc = "1: Select PATH1 (can be configured for PLL0, if available in the product)"]
    PATH1 = 1,
    #[doc = "2: Select PATH2 (can be configured for PLL1, if available in the product)"]
    PATH2 = 2,
    #[doc = "3: Select PATH3 (can be configured for PLL2, if available in the product)"]
    PATH3 = 3,
    #[doc = "4: Select PATH4 (can be configured for PLL3, if available in the product)"]
    PATH4 = 4,
    #[doc = "5: Select PATH5 (can be configured for PLL4, if available in the product)"]
    PATH5 = 5,
    #[doc = "6: Select PATH6 (can be configured for PLL5, if available in the product)"]
    PATH6 = 6,
    #[doc = "7: Select PATH7 (can be configured for PLL6, if available in the product)"]
    PATH7 = 7,
    #[doc = "8: Select PATH8 (can be configured for PLL7, if available in the product)"]
    PATH8 = 8,
    #[doc = "9: Select PATH9 (can be configured for PLL8, if available in the product)"]
    PATH9 = 9,
    #[doc = "10: Select PATH10 (can be configured for PLL9, if available in the product)"]
    PATH10 = 10,
    #[doc = "11: Select PATH11 (can be configured for PLL10, if available in the product)"]
    PATH11 = 11,
    #[doc = "12: Select PATH12 (can be configured for PLL11, if available in the product)"]
    PATH12 = 12,
    #[doc = "13: Select PATH13 (can be configured for PLL12, if available in the product)"]
    PATH13 = 13,
    #[doc = "14: Select PATH14 (can be configured for PLL13, if available in the product)"]
    PATH14 = 14,
    #[doc = "15: Select PATH15 (can be configured for PLL14, if available in the product)"]
    PATH15 = 15,
}
impl From<ROOT_MUX_A> for u8 {
    #[inline(always)]
    fn from(variant: ROOT_MUX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ROOT_MUX_A {
    type Ux = u8;
}
impl ROOT_MUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ROOT_MUX_A {
        match self.bits {
            0 => ROOT_MUX_A::PATH0,
            1 => ROOT_MUX_A::PATH1,
            2 => ROOT_MUX_A::PATH2,
            3 => ROOT_MUX_A::PATH3,
            4 => ROOT_MUX_A::PATH4,
            5 => ROOT_MUX_A::PATH5,
            6 => ROOT_MUX_A::PATH6,
            7 => ROOT_MUX_A::PATH7,
            8 => ROOT_MUX_A::PATH8,
            9 => ROOT_MUX_A::PATH9,
            10 => ROOT_MUX_A::PATH10,
            11 => ROOT_MUX_A::PATH11,
            12 => ROOT_MUX_A::PATH12,
            13 => ROOT_MUX_A::PATH13,
            14 => ROOT_MUX_A::PATH14,
            15 => ROOT_MUX_A::PATH15,
            _ => unreachable!(),
        }
    }
    #[doc = "Select PATH0 (can be configured for FLL)"]
    #[inline(always)]
    pub fn is_path0(&self) -> bool {
        *self == ROOT_MUX_A::PATH0
    }
    #[doc = "Select PATH1 (can be configured for PLL0, if available in the product)"]
    #[inline(always)]
    pub fn is_path1(&self) -> bool {
        *self == ROOT_MUX_A::PATH1
    }
    #[doc = "Select PATH2 (can be configured for PLL1, if available in the product)"]
    #[inline(always)]
    pub fn is_path2(&self) -> bool {
        *self == ROOT_MUX_A::PATH2
    }
    #[doc = "Select PATH3 (can be configured for PLL2, if available in the product)"]
    #[inline(always)]
    pub fn is_path3(&self) -> bool {
        *self == ROOT_MUX_A::PATH3
    }
    #[doc = "Select PATH4 (can be configured for PLL3, if available in the product)"]
    #[inline(always)]
    pub fn is_path4(&self) -> bool {
        *self == ROOT_MUX_A::PATH4
    }
    #[doc = "Select PATH5 (can be configured for PLL4, if available in the product)"]
    #[inline(always)]
    pub fn is_path5(&self) -> bool {
        *self == ROOT_MUX_A::PATH5
    }
    #[doc = "Select PATH6 (can be configured for PLL5, if available in the product)"]
    #[inline(always)]
    pub fn is_path6(&self) -> bool {
        *self == ROOT_MUX_A::PATH6
    }
    #[doc = "Select PATH7 (can be configured for PLL6, if available in the product)"]
    #[inline(always)]
    pub fn is_path7(&self) -> bool {
        *self == ROOT_MUX_A::PATH7
    }
    #[doc = "Select PATH8 (can be configured for PLL7, if available in the product)"]
    #[inline(always)]
    pub fn is_path8(&self) -> bool {
        *self == ROOT_MUX_A::PATH8
    }
    #[doc = "Select PATH9 (can be configured for PLL8, if available in the product)"]
    #[inline(always)]
    pub fn is_path9(&self) -> bool {
        *self == ROOT_MUX_A::PATH9
    }
    #[doc = "Select PATH10 (can be configured for PLL9, if available in the product)"]
    #[inline(always)]
    pub fn is_path10(&self) -> bool {
        *self == ROOT_MUX_A::PATH10
    }
    #[doc = "Select PATH11 (can be configured for PLL10, if available in the product)"]
    #[inline(always)]
    pub fn is_path11(&self) -> bool {
        *self == ROOT_MUX_A::PATH11
    }
    #[doc = "Select PATH12 (can be configured for PLL11, if available in the product)"]
    #[inline(always)]
    pub fn is_path12(&self) -> bool {
        *self == ROOT_MUX_A::PATH12
    }
    #[doc = "Select PATH13 (can be configured for PLL12, if available in the product)"]
    #[inline(always)]
    pub fn is_path13(&self) -> bool {
        *self == ROOT_MUX_A::PATH13
    }
    #[doc = "Select PATH14 (can be configured for PLL13, if available in the product)"]
    #[inline(always)]
    pub fn is_path14(&self) -> bool {
        *self == ROOT_MUX_A::PATH14
    }
    #[doc = "Select PATH15 (can be configured for PLL14, if available in the product)"]
    #[inline(always)]
    pub fn is_path15(&self) -> bool {
        *self == ROOT_MUX_A::PATH15
    }
}
#[doc = "Field `ROOT_MUX` writer - Selects a clock path as the root of HFCLK&lt;k> and for SRSS DSI input &lt;k>. Use CLK_SELECT_PATH\\[i\\]
to configure the desired path. Some paths may have FLL or PLL available (product-specific), and the control and bypass mux selections of these are in other registers. Configure the FLL using CLK_FLL_CONFIG register. Configure a PLL using the related CLK_PLL_CONFIG\\[k\\]
register. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior."]
pub type ROOT_MUX_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, ROOT_MUX_A>;
impl<'a, REG> ROOT_MUX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select PATH0 (can be configured for FLL)"]
    #[inline(always)]
    pub fn path0(self) -> &'a mut crate::W<REG> {
        self.variant(ROOT_MUX_A::PATH0)
    }
    #[doc = "Select PATH1 (can be configured for PLL0, if available in the product)"]
    #[inline(always)]
    pub fn path1(self) -> &'a mut crate::W<REG> {
        self.variant(ROOT_MUX_A::PATH1)
    }
    #[doc = "Select PATH2 (can be configured for PLL1, if available in the product)"]
    #[inline(always)]
    pub fn path2(self) -> &'a mut crate::W<REG> {
        self.variant(ROOT_MUX_A::PATH2)
    }
    #[doc = "Select PATH3 (can be configured for PLL2, if available in the product)"]
    #[inline(always)]
    pub fn path3(self) -> &'a mut crate::W<REG> {
        self.variant(ROOT_MUX_A::PATH3)
    }
    #[doc = "Select PATH4 (can be configured for PLL3, if available in the product)"]
    #[inline(always)]
    pub fn path4(self) -> &'a mut crate::W<REG> {
        self.variant(ROOT_MUX_A::PATH4)
    }
    #[doc = "Select PATH5 (can be configured for PLL4, if available in the product)"]
    #[inline(always)]
    pub fn path5(self) -> &'a mut crate::W<REG> {
        self.variant(ROOT_MUX_A::PATH5)
    }
    #[doc = "Select PATH6 (can be configured for PLL5, if available in the product)"]
    #[inline(always)]
    pub fn path6(self) -> &'a mut crate::W<REG> {
        self.variant(ROOT_MUX_A::PATH6)
    }
    #[doc = "Select PATH7 (can be configured for PLL6, if available in the product)"]
    #[inline(always)]
    pub fn path7(self) -> &'a mut crate::W<REG> {
        self.variant(ROOT_MUX_A::PATH7)
    }
    #[doc = "Select PATH8 (can be configured for PLL7, if available in the product)"]
    #[inline(always)]
    pub fn path8(self) -> &'a mut crate::W<REG> {
        self.variant(ROOT_MUX_A::PATH8)
    }
    #[doc = "Select PATH9 (can be configured for PLL8, if available in the product)"]
    #[inline(always)]
    pub fn path9(self) -> &'a mut crate::W<REG> {
        self.variant(ROOT_MUX_A::PATH9)
    }
    #[doc = "Select PATH10 (can be configured for PLL9, if available in the product)"]
    #[inline(always)]
    pub fn path10(self) -> &'a mut crate::W<REG> {
        self.variant(ROOT_MUX_A::PATH10)
    }
    #[doc = "Select PATH11 (can be configured for PLL10, if available in the product)"]
    #[inline(always)]
    pub fn path11(self) -> &'a mut crate::W<REG> {
        self.variant(ROOT_MUX_A::PATH11)
    }
    #[doc = "Select PATH12 (can be configured for PLL11, if available in the product)"]
    #[inline(always)]
    pub fn path12(self) -> &'a mut crate::W<REG> {
        self.variant(ROOT_MUX_A::PATH12)
    }
    #[doc = "Select PATH13 (can be configured for PLL12, if available in the product)"]
    #[inline(always)]
    pub fn path13(self) -> &'a mut crate::W<REG> {
        self.variant(ROOT_MUX_A::PATH13)
    }
    #[doc = "Select PATH14 (can be configured for PLL13, if available in the product)"]
    #[inline(always)]
    pub fn path14(self) -> &'a mut crate::W<REG> {
        self.variant(ROOT_MUX_A::PATH14)
    }
    #[doc = "Select PATH15 (can be configured for PLL14, if available in the product)"]
    #[inline(always)]
    pub fn path15(self) -> &'a mut crate::W<REG> {
        self.variant(ROOT_MUX_A::PATH15)
    }
}
#[doc = "Field `ROOT_DIV` reader - Selects predivider value for this clock root and DSI input."]
pub type ROOT_DIV_R = crate::FieldReader<ROOT_DIV_A>;
#[doc = "Selects predivider value for this clock root and DSI input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ROOT_DIV_A {
    #[doc = "0: Transparent mode, feed through selected clock source w/o dividing."]
    NO_DIV = 0,
    #[doc = "1: Divide selected clock source by 2"]
    DIV_BY_2 = 1,
    #[doc = "2: Divide selected clock source by 4"]
    DIV_BY_4 = 2,
    #[doc = "3: Divide selected clock source by 8"]
    DIV_BY_8 = 3,
}
impl From<ROOT_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: ROOT_DIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ROOT_DIV_A {
    type Ux = u8;
}
impl ROOT_DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ROOT_DIV_A {
        match self.bits {
            0 => ROOT_DIV_A::NO_DIV,
            1 => ROOT_DIV_A::DIV_BY_2,
            2 => ROOT_DIV_A::DIV_BY_4,
            3 => ROOT_DIV_A::DIV_BY_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Transparent mode, feed through selected clock source w/o dividing."]
    #[inline(always)]
    pub fn is_no_div(&self) -> bool {
        *self == ROOT_DIV_A::NO_DIV
    }
    #[doc = "Divide selected clock source by 2"]
    #[inline(always)]
    pub fn is_div_by_2(&self) -> bool {
        *self == ROOT_DIV_A::DIV_BY_2
    }
    #[doc = "Divide selected clock source by 4"]
    #[inline(always)]
    pub fn is_div_by_4(&self) -> bool {
        *self == ROOT_DIV_A::DIV_BY_4
    }
    #[doc = "Divide selected clock source by 8"]
    #[inline(always)]
    pub fn is_div_by_8(&self) -> bool {
        *self == ROOT_DIV_A::DIV_BY_8
    }
}
#[doc = "Field `ROOT_DIV` writer - Selects predivider value for this clock root and DSI input."]
pub type ROOT_DIV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ROOT_DIV_A>;
impl<'a, REG> ROOT_DIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Transparent mode, feed through selected clock source w/o dividing."]
    #[inline(always)]
    pub fn no_div(self) -> &'a mut crate::W<REG> {
        self.variant(ROOT_DIV_A::NO_DIV)
    }
    #[doc = "Divide selected clock source by 2"]
    #[inline(always)]
    pub fn div_by_2(self) -> &'a mut crate::W<REG> {
        self.variant(ROOT_DIV_A::DIV_BY_2)
    }
    #[doc = "Divide selected clock source by 4"]
    #[inline(always)]
    pub fn div_by_4(self) -> &'a mut crate::W<REG> {
        self.variant(ROOT_DIV_A::DIV_BY_4)
    }
    #[doc = "Divide selected clock source by 8"]
    #[inline(always)]
    pub fn div_by_8(self) -> &'a mut crate::W<REG> {
        self.variant(ROOT_DIV_A::DIV_BY_8)
    }
}
#[doc = "Field `ENABLE` reader - Enable for this clock root. All clock roots default to disabled (ENABLE==0) except HFCLK0, which cannot be disabled."]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable for this clock root. All clock roots default to disabled (ENABLE==0) except HFCLK0, which cannot be disabled."]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Selects a clock path as the root of HFCLK&lt;k> and for SRSS DSI input &lt;k>. Use CLK_SELECT_PATH\\[i\\]
to configure the desired path. Some paths may have FLL or PLL available (product-specific), and the control and bypass mux selections of these are in other registers. Configure the FLL using CLK_FLL_CONFIG register. Configure a PLL using the related CLK_PLL_CONFIG\\[k\\]
register. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior."]
    #[inline(always)]
    pub fn root_mux(&self) -> ROOT_MUX_R {
        ROOT_MUX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Selects predivider value for this clock root and DSI input."]
    #[inline(always)]
    pub fn root_div(&self) -> ROOT_DIV_R {
        ROOT_DIV_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 31 - Enable for this clock root. All clock roots default to disabled (ENABLE==0) except HFCLK0, which cannot be disabled."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Selects a clock path as the root of HFCLK&lt;k> and for SRSS DSI input &lt;k>. Use CLK_SELECT_PATH\\[i\\]
to configure the desired path. Some paths may have FLL or PLL available (product-specific), and the control and bypass mux selections of these are in other registers. Configure the FLL using CLK_FLL_CONFIG register. Configure a PLL using the related CLK_PLL_CONFIG\\[k\\]
register. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn root_mux(&mut self) -> ROOT_MUX_W<CLK_ROOT_SELECT_SPEC> {
        ROOT_MUX_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Selects predivider value for this clock root and DSI input."]
    #[inline(always)]
    #[must_use]
    pub fn root_div(&mut self) -> ROOT_DIV_W<CLK_ROOT_SELECT_SPEC> {
        ROOT_DIV_W::new(self, 4)
    }
    #[doc = "Bit 31 - Enable for this clock root. All clock roots default to disabled (ENABLE==0) except HFCLK0, which cannot be disabled."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CLK_ROOT_SELECT_SPEC> {
        ENABLE_W::new(self, 31)
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
#[doc = "Clock Root Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_root_select::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_root_select::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_ROOT_SELECT_SPEC;
impl crate::RegisterSpec for CLK_ROOT_SELECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_root_select::R`](R) reader structure"]
impl crate::Readable for CLK_ROOT_SELECT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_root_select::W`](W) writer structure"]
impl crate::Writable for CLK_ROOT_SELECT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_ROOT_SELECT[%s]
to value 0"]
impl crate::Resettable for CLK_ROOT_SELECT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
