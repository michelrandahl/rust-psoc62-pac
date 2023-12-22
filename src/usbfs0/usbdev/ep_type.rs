#[doc = "Register `EP_TYPE` reader"]
pub type R = crate::R<EP_TYPE_SPEC>;
#[doc = "Register `EP_TYPE` writer"]
pub type W = crate::W<EP_TYPE_SPEC>;
#[doc = "Field `EP1_TYP` reader - Endpoint Type Indication."]
pub type EP1_TYP_R = crate::BitReader<EP1_TYP_A>;
#[doc = "Endpoint Type Indication.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EP1_TYP_A {
    #[doc = "0: IN outpoint"]
    EP_IN = 0,
    #[doc = "1: OUT outpoint"]
    EP_OUT = 1,
}
impl From<EP1_TYP_A> for bool {
    #[inline(always)]
    fn from(variant: EP1_TYP_A) -> Self {
        variant as u8 != 0
    }
}
impl EP1_TYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EP1_TYP_A {
        match self.bits {
            false => EP1_TYP_A::EP_IN,
            true => EP1_TYP_A::EP_OUT,
        }
    }
    #[doc = "IN outpoint"]
    #[inline(always)]
    pub fn is_ep_in(&self) -> bool {
        *self == EP1_TYP_A::EP_IN
    }
    #[doc = "OUT outpoint"]
    #[inline(always)]
    pub fn is_ep_out(&self) -> bool {
        *self == EP1_TYP_A::EP_OUT
    }
}
#[doc = "Field `EP1_TYP` writer - Endpoint Type Indication."]
pub type EP1_TYP_W<'a, REG> = crate::BitWriter<'a, REG, EP1_TYP_A>;
impl<'a, REG> EP1_TYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IN outpoint"]
    #[inline(always)]
    pub fn ep_in(self) -> &'a mut crate::W<REG> {
        self.variant(EP1_TYP_A::EP_IN)
    }
    #[doc = "OUT outpoint"]
    #[inline(always)]
    pub fn ep_out(self) -> &'a mut crate::W<REG> {
        self.variant(EP1_TYP_A::EP_OUT)
    }
}
#[doc = "Field `EP2_TYP` reader - Endpoint Type Indication."]
pub type EP2_TYP_R = crate::BitReader<EP2_TYP_A>;
#[doc = "Endpoint Type Indication.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EP2_TYP_A {
    #[doc = "0: IN outpoint"]
    EP_IN = 0,
    #[doc = "1: OUT outpoint"]
    EP_OUT = 1,
}
impl From<EP2_TYP_A> for bool {
    #[inline(always)]
    fn from(variant: EP2_TYP_A) -> Self {
        variant as u8 != 0
    }
}
impl EP2_TYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EP2_TYP_A {
        match self.bits {
            false => EP2_TYP_A::EP_IN,
            true => EP2_TYP_A::EP_OUT,
        }
    }
    #[doc = "IN outpoint"]
    #[inline(always)]
    pub fn is_ep_in(&self) -> bool {
        *self == EP2_TYP_A::EP_IN
    }
    #[doc = "OUT outpoint"]
    #[inline(always)]
    pub fn is_ep_out(&self) -> bool {
        *self == EP2_TYP_A::EP_OUT
    }
}
#[doc = "Field `EP2_TYP` writer - Endpoint Type Indication."]
pub type EP2_TYP_W<'a, REG> = crate::BitWriter<'a, REG, EP2_TYP_A>;
impl<'a, REG> EP2_TYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IN outpoint"]
    #[inline(always)]
    pub fn ep_in(self) -> &'a mut crate::W<REG> {
        self.variant(EP2_TYP_A::EP_IN)
    }
    #[doc = "OUT outpoint"]
    #[inline(always)]
    pub fn ep_out(self) -> &'a mut crate::W<REG> {
        self.variant(EP2_TYP_A::EP_OUT)
    }
}
#[doc = "Field `EP3_TYP` reader - Endpoint Type Indication."]
pub type EP3_TYP_R = crate::BitReader<EP3_TYP_A>;
#[doc = "Endpoint Type Indication.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EP3_TYP_A {
    #[doc = "0: IN outpoint"]
    EP_IN = 0,
    #[doc = "1: OUT outpoint"]
    EP_OUT = 1,
}
impl From<EP3_TYP_A> for bool {
    #[inline(always)]
    fn from(variant: EP3_TYP_A) -> Self {
        variant as u8 != 0
    }
}
impl EP3_TYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EP3_TYP_A {
        match self.bits {
            false => EP3_TYP_A::EP_IN,
            true => EP3_TYP_A::EP_OUT,
        }
    }
    #[doc = "IN outpoint"]
    #[inline(always)]
    pub fn is_ep_in(&self) -> bool {
        *self == EP3_TYP_A::EP_IN
    }
    #[doc = "OUT outpoint"]
    #[inline(always)]
    pub fn is_ep_out(&self) -> bool {
        *self == EP3_TYP_A::EP_OUT
    }
}
#[doc = "Field `EP3_TYP` writer - Endpoint Type Indication."]
pub type EP3_TYP_W<'a, REG> = crate::BitWriter<'a, REG, EP3_TYP_A>;
impl<'a, REG> EP3_TYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IN outpoint"]
    #[inline(always)]
    pub fn ep_in(self) -> &'a mut crate::W<REG> {
        self.variant(EP3_TYP_A::EP_IN)
    }
    #[doc = "OUT outpoint"]
    #[inline(always)]
    pub fn ep_out(self) -> &'a mut crate::W<REG> {
        self.variant(EP3_TYP_A::EP_OUT)
    }
}
#[doc = "Field `EP4_TYP` reader - Endpoint Type Indication."]
pub type EP4_TYP_R = crate::BitReader<EP4_TYP_A>;
#[doc = "Endpoint Type Indication.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EP4_TYP_A {
    #[doc = "0: IN outpoint"]
    EP_IN = 0,
    #[doc = "1: OUT outpoint"]
    EP_OUT = 1,
}
impl From<EP4_TYP_A> for bool {
    #[inline(always)]
    fn from(variant: EP4_TYP_A) -> Self {
        variant as u8 != 0
    }
}
impl EP4_TYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EP4_TYP_A {
        match self.bits {
            false => EP4_TYP_A::EP_IN,
            true => EP4_TYP_A::EP_OUT,
        }
    }
    #[doc = "IN outpoint"]
    #[inline(always)]
    pub fn is_ep_in(&self) -> bool {
        *self == EP4_TYP_A::EP_IN
    }
    #[doc = "OUT outpoint"]
    #[inline(always)]
    pub fn is_ep_out(&self) -> bool {
        *self == EP4_TYP_A::EP_OUT
    }
}
#[doc = "Field `EP4_TYP` writer - Endpoint Type Indication."]
pub type EP4_TYP_W<'a, REG> = crate::BitWriter<'a, REG, EP4_TYP_A>;
impl<'a, REG> EP4_TYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IN outpoint"]
    #[inline(always)]
    pub fn ep_in(self) -> &'a mut crate::W<REG> {
        self.variant(EP4_TYP_A::EP_IN)
    }
    #[doc = "OUT outpoint"]
    #[inline(always)]
    pub fn ep_out(self) -> &'a mut crate::W<REG> {
        self.variant(EP4_TYP_A::EP_OUT)
    }
}
#[doc = "Field `EP5_TYP` reader - Endpoint Type Indication."]
pub type EP5_TYP_R = crate::BitReader<EP5_TYP_A>;
#[doc = "Endpoint Type Indication.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EP5_TYP_A {
    #[doc = "0: IN outpoint"]
    EP_IN = 0,
    #[doc = "1: OUT outpoint"]
    EP_OUT = 1,
}
impl From<EP5_TYP_A> for bool {
    #[inline(always)]
    fn from(variant: EP5_TYP_A) -> Self {
        variant as u8 != 0
    }
}
impl EP5_TYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EP5_TYP_A {
        match self.bits {
            false => EP5_TYP_A::EP_IN,
            true => EP5_TYP_A::EP_OUT,
        }
    }
    #[doc = "IN outpoint"]
    #[inline(always)]
    pub fn is_ep_in(&self) -> bool {
        *self == EP5_TYP_A::EP_IN
    }
    #[doc = "OUT outpoint"]
    #[inline(always)]
    pub fn is_ep_out(&self) -> bool {
        *self == EP5_TYP_A::EP_OUT
    }
}
#[doc = "Field `EP5_TYP` writer - Endpoint Type Indication."]
pub type EP5_TYP_W<'a, REG> = crate::BitWriter<'a, REG, EP5_TYP_A>;
impl<'a, REG> EP5_TYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IN outpoint"]
    #[inline(always)]
    pub fn ep_in(self) -> &'a mut crate::W<REG> {
        self.variant(EP5_TYP_A::EP_IN)
    }
    #[doc = "OUT outpoint"]
    #[inline(always)]
    pub fn ep_out(self) -> &'a mut crate::W<REG> {
        self.variant(EP5_TYP_A::EP_OUT)
    }
}
#[doc = "Field `EP6_TYP` reader - Endpoint Type Indication."]
pub type EP6_TYP_R = crate::BitReader<EP6_TYP_A>;
#[doc = "Endpoint Type Indication.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EP6_TYP_A {
    #[doc = "0: IN outpoint"]
    EP_IN = 0,
    #[doc = "1: OUT outpoint"]
    EP_OUT = 1,
}
impl From<EP6_TYP_A> for bool {
    #[inline(always)]
    fn from(variant: EP6_TYP_A) -> Self {
        variant as u8 != 0
    }
}
impl EP6_TYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EP6_TYP_A {
        match self.bits {
            false => EP6_TYP_A::EP_IN,
            true => EP6_TYP_A::EP_OUT,
        }
    }
    #[doc = "IN outpoint"]
    #[inline(always)]
    pub fn is_ep_in(&self) -> bool {
        *self == EP6_TYP_A::EP_IN
    }
    #[doc = "OUT outpoint"]
    #[inline(always)]
    pub fn is_ep_out(&self) -> bool {
        *self == EP6_TYP_A::EP_OUT
    }
}
#[doc = "Field `EP6_TYP` writer - Endpoint Type Indication."]
pub type EP6_TYP_W<'a, REG> = crate::BitWriter<'a, REG, EP6_TYP_A>;
impl<'a, REG> EP6_TYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IN outpoint"]
    #[inline(always)]
    pub fn ep_in(self) -> &'a mut crate::W<REG> {
        self.variant(EP6_TYP_A::EP_IN)
    }
    #[doc = "OUT outpoint"]
    #[inline(always)]
    pub fn ep_out(self) -> &'a mut crate::W<REG> {
        self.variant(EP6_TYP_A::EP_OUT)
    }
}
#[doc = "Field `EP7_TYP` reader - Endpoint Type Indication."]
pub type EP7_TYP_R = crate::BitReader<EP7_TYP_A>;
#[doc = "Endpoint Type Indication.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EP7_TYP_A {
    #[doc = "0: IN outpoint"]
    EP_IN = 0,
    #[doc = "1: OUT outpoint"]
    EP_OUT = 1,
}
impl From<EP7_TYP_A> for bool {
    #[inline(always)]
    fn from(variant: EP7_TYP_A) -> Self {
        variant as u8 != 0
    }
}
impl EP7_TYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EP7_TYP_A {
        match self.bits {
            false => EP7_TYP_A::EP_IN,
            true => EP7_TYP_A::EP_OUT,
        }
    }
    #[doc = "IN outpoint"]
    #[inline(always)]
    pub fn is_ep_in(&self) -> bool {
        *self == EP7_TYP_A::EP_IN
    }
    #[doc = "OUT outpoint"]
    #[inline(always)]
    pub fn is_ep_out(&self) -> bool {
        *self == EP7_TYP_A::EP_OUT
    }
}
#[doc = "Field `EP7_TYP` writer - Endpoint Type Indication."]
pub type EP7_TYP_W<'a, REG> = crate::BitWriter<'a, REG, EP7_TYP_A>;
impl<'a, REG> EP7_TYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IN outpoint"]
    #[inline(always)]
    pub fn ep_in(self) -> &'a mut crate::W<REG> {
        self.variant(EP7_TYP_A::EP_IN)
    }
    #[doc = "OUT outpoint"]
    #[inline(always)]
    pub fn ep_out(self) -> &'a mut crate::W<REG> {
        self.variant(EP7_TYP_A::EP_OUT)
    }
}
#[doc = "Field `EP8_TYP` reader - Endpoint Type Indication."]
pub type EP8_TYP_R = crate::BitReader<EP8_TYP_A>;
#[doc = "Endpoint Type Indication.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EP8_TYP_A {
    #[doc = "0: IN outpoint"]
    EP_IN = 0,
    #[doc = "1: OUT outpoint"]
    EP_OUT = 1,
}
impl From<EP8_TYP_A> for bool {
    #[inline(always)]
    fn from(variant: EP8_TYP_A) -> Self {
        variant as u8 != 0
    }
}
impl EP8_TYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EP8_TYP_A {
        match self.bits {
            false => EP8_TYP_A::EP_IN,
            true => EP8_TYP_A::EP_OUT,
        }
    }
    #[doc = "IN outpoint"]
    #[inline(always)]
    pub fn is_ep_in(&self) -> bool {
        *self == EP8_TYP_A::EP_IN
    }
    #[doc = "OUT outpoint"]
    #[inline(always)]
    pub fn is_ep_out(&self) -> bool {
        *self == EP8_TYP_A::EP_OUT
    }
}
#[doc = "Field `EP8_TYP` writer - Endpoint Type Indication."]
pub type EP8_TYP_W<'a, REG> = crate::BitWriter<'a, REG, EP8_TYP_A>;
impl<'a, REG> EP8_TYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IN outpoint"]
    #[inline(always)]
    pub fn ep_in(self) -> &'a mut crate::W<REG> {
        self.variant(EP8_TYP_A::EP_IN)
    }
    #[doc = "OUT outpoint"]
    #[inline(always)]
    pub fn ep_out(self) -> &'a mut crate::W<REG> {
        self.variant(EP8_TYP_A::EP_OUT)
    }
}
impl R {
    #[doc = "Bit 0 - Endpoint Type Indication."]
    #[inline(always)]
    pub fn ep1_typ(&self) -> EP1_TYP_R {
        EP1_TYP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint Type Indication."]
    #[inline(always)]
    pub fn ep2_typ(&self) -> EP2_TYP_R {
        EP2_TYP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Endpoint Type Indication."]
    #[inline(always)]
    pub fn ep3_typ(&self) -> EP3_TYP_R {
        EP3_TYP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Endpoint Type Indication."]
    #[inline(always)]
    pub fn ep4_typ(&self) -> EP4_TYP_R {
        EP4_TYP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Endpoint Type Indication."]
    #[inline(always)]
    pub fn ep5_typ(&self) -> EP5_TYP_R {
        EP5_TYP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Endpoint Type Indication."]
    #[inline(always)]
    pub fn ep6_typ(&self) -> EP6_TYP_R {
        EP6_TYP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Endpoint Type Indication."]
    #[inline(always)]
    pub fn ep7_typ(&self) -> EP7_TYP_R {
        EP7_TYP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Endpoint Type Indication."]
    #[inline(always)]
    pub fn ep8_typ(&self) -> EP8_TYP_R {
        EP8_TYP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Endpoint Type Indication."]
    #[inline(always)]
    #[must_use]
    pub fn ep1_typ(&mut self) -> EP1_TYP_W<EP_TYPE_SPEC> {
        EP1_TYP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint Type Indication."]
    #[inline(always)]
    #[must_use]
    pub fn ep2_typ(&mut self) -> EP2_TYP_W<EP_TYPE_SPEC> {
        EP2_TYP_W::new(self, 1)
    }
    #[doc = "Bit 2 - Endpoint Type Indication."]
    #[inline(always)]
    #[must_use]
    pub fn ep3_typ(&mut self) -> EP3_TYP_W<EP_TYPE_SPEC> {
        EP3_TYP_W::new(self, 2)
    }
    #[doc = "Bit 3 - Endpoint Type Indication."]
    #[inline(always)]
    #[must_use]
    pub fn ep4_typ(&mut self) -> EP4_TYP_W<EP_TYPE_SPEC> {
        EP4_TYP_W::new(self, 3)
    }
    #[doc = "Bit 4 - Endpoint Type Indication."]
    #[inline(always)]
    #[must_use]
    pub fn ep5_typ(&mut self) -> EP5_TYP_W<EP_TYPE_SPEC> {
        EP5_TYP_W::new(self, 4)
    }
    #[doc = "Bit 5 - Endpoint Type Indication."]
    #[inline(always)]
    #[must_use]
    pub fn ep6_typ(&mut self) -> EP6_TYP_W<EP_TYPE_SPEC> {
        EP6_TYP_W::new(self, 5)
    }
    #[doc = "Bit 6 - Endpoint Type Indication."]
    #[inline(always)]
    #[must_use]
    pub fn ep7_typ(&mut self) -> EP7_TYP_W<EP_TYPE_SPEC> {
        EP7_TYP_W::new(self, 6)
    }
    #[doc = "Bit 7 - Endpoint Type Indication."]
    #[inline(always)]
    #[must_use]
    pub fn ep8_typ(&mut self) -> EP8_TYP_W<EP_TYPE_SPEC> {
        EP8_TYP_W::new(self, 7)
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
#[doc = "Endpoint Type (IN/OUT) Indication *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep_type::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_type::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EP_TYPE_SPEC;
impl crate::RegisterSpec for EP_TYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ep_type::R`](R) reader structure"]
impl crate::Readable for EP_TYPE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ep_type::W`](W) writer structure"]
impl crate::Writable for EP_TYPE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EP_TYPE to value 0"]
impl crate::Resettable for EP_TYPE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
