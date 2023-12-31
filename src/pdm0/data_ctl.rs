#[doc = "Register `DATA_CTL` reader"]
pub type R = crate::R<DATA_CTL_SPEC>;
#[doc = "Register `DATA_CTL` writer"]
pub type W = crate::W<DATA_CTL_SPEC>;
#[doc = "Field `WORD_LEN` reader - PCM Word Length in number of bits: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PCM_IWL)"]
pub type WORD_LEN_R = crate::FieldReader<WORD_LEN_A>;
#[doc = "PCM Word Length in number of bits: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PCM_IWL)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WORD_LEN_A {
    #[doc = "0: 16-bit"]
    BIT_LEN16 = 0,
    #[doc = "1: 18-bit"]
    BIT_LEN18 = 1,
    #[doc = "2: 20-bit"]
    BIT_LEN20 = 2,
    #[doc = "3: 24-bit"]
    BIT_LEN24 = 3,
}
impl From<WORD_LEN_A> for u8 {
    #[inline(always)]
    fn from(variant: WORD_LEN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WORD_LEN_A {
    type Ux = u8;
}
impl WORD_LEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WORD_LEN_A {
        match self.bits {
            0 => WORD_LEN_A::BIT_LEN16,
            1 => WORD_LEN_A::BIT_LEN18,
            2 => WORD_LEN_A::BIT_LEN20,
            3 => WORD_LEN_A::BIT_LEN24,
            _ => unreachable!(),
        }
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn is_bit_len16(&self) -> bool {
        *self == WORD_LEN_A::BIT_LEN16
    }
    #[doc = "18-bit"]
    #[inline(always)]
    pub fn is_bit_len18(&self) -> bool {
        *self == WORD_LEN_A::BIT_LEN18
    }
    #[doc = "20-bit"]
    #[inline(always)]
    pub fn is_bit_len20(&self) -> bool {
        *self == WORD_LEN_A::BIT_LEN20
    }
    #[doc = "24-bit"]
    #[inline(always)]
    pub fn is_bit_len24(&self) -> bool {
        *self == WORD_LEN_A::BIT_LEN24
    }
}
#[doc = "Field `WORD_LEN` writer - PCM Word Length in number of bits: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PCM_IWL)"]
pub type WORD_LEN_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, WORD_LEN_A>;
impl<'a, REG> WORD_LEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn bit_len16(self) -> &'a mut crate::W<REG> {
        self.variant(WORD_LEN_A::BIT_LEN16)
    }
    #[doc = "18-bit"]
    #[inline(always)]
    pub fn bit_len18(self) -> &'a mut crate::W<REG> {
        self.variant(WORD_LEN_A::BIT_LEN18)
    }
    #[doc = "20-bit"]
    #[inline(always)]
    pub fn bit_len20(self) -> &'a mut crate::W<REG> {
        self.variant(WORD_LEN_A::BIT_LEN20)
    }
    #[doc = "24-bit"]
    #[inline(always)]
    pub fn bit_len24(self) -> &'a mut crate::W<REG> {
        self.variant(WORD_LEN_A::BIT_LEN24)
    }
}
#[doc = "Field `BIT_EXTENSION` reader - When reception word length is shorter than the word length of RX_FIFO_RD, extension mode of upper bit should be set. '0': Extended by '0' '1': Extended by sign bit (if MSB word is '1', then it is extended by '1', if MSB is '0' then it is extended by '0')"]
pub type BIT_EXTENSION_R = crate::BitReader;
#[doc = "Field `BIT_EXTENSION` writer - When reception word length is shorter than the word length of RX_FIFO_RD, extension mode of upper bit should be set. '0': Extended by '0' '1': Extended by sign bit (if MSB word is '1', then it is extended by '1', if MSB is '0' then it is extended by '0')"]
pub type BIT_EXTENSION_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - PCM Word Length in number of bits: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PCM_IWL)"]
    #[inline(always)]
    pub fn word_len(&self) -> WORD_LEN_R {
        WORD_LEN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - When reception word length is shorter than the word length of RX_FIFO_RD, extension mode of upper bit should be set. '0': Extended by '0' '1': Extended by sign bit (if MSB word is '1', then it is extended by '1', if MSB is '0' then it is extended by '0')"]
    #[inline(always)]
    pub fn bit_extension(&self) -> BIT_EXTENSION_R {
        BIT_EXTENSION_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - PCM Word Length in number of bits: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PCM_IWL)"]
    #[inline(always)]
    #[must_use]
    pub fn word_len(&mut self) -> WORD_LEN_W<DATA_CTL_SPEC> {
        WORD_LEN_W::new(self, 0)
    }
    #[doc = "Bit 8 - When reception word length is shorter than the word length of RX_FIFO_RD, extension mode of upper bit should be set. '0': Extended by '0' '1': Extended by sign bit (if MSB word is '1', then it is extended by '1', if MSB is '0' then it is extended by '0')"]
    #[inline(always)]
    #[must_use]
    pub fn bit_extension(&mut self) -> BIT_EXTENSION_W<DATA_CTL_SPEC> {
        BIT_EXTENSION_W::new(self, 8)
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
#[doc = "Data control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA_CTL_SPEC;
impl crate::RegisterSpec for DATA_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_ctl::R`](R) reader structure"]
impl crate::Readable for DATA_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data_ctl::W`](W) writer structure"]
impl crate::Writable for DATA_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA_CTL to value 0"]
impl crate::Resettable for DATA_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
