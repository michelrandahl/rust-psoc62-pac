#[doc = "Register `SW_FW_MOD_SEL` reader"]
pub type R = crate::R<SW_FW_MOD_SEL_SPEC>;
#[doc = "Register `SW_FW_MOD_SEL` writer"]
pub type W = crate::W<SW_FW_MOD_SEL_SPEC>;
#[doc = "Field `SW_F1PM` reader - Set corresponding switch"]
pub type SW_F1PM_R = crate::BitReader;
#[doc = "Field `SW_F1PM` writer - Set corresponding switch"]
pub type SW_F1PM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_F1MA` reader - Select waveform for corresponding switch"]
pub type SW_F1MA_R = crate::FieldReader;
#[doc = "Field `SW_F1MA` writer - Select waveform for corresponding switch"]
pub type SW_F1MA_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SW_F1CA` reader - Select waveform for corresponding switch"]
pub type SW_F1CA_R = crate::FieldReader;
#[doc = "Field `SW_F1CA` writer - Select waveform for corresponding switch"]
pub type SW_F1CA_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SW_C1CC` reader - Set corresponding switch"]
pub type SW_C1CC_R = crate::BitReader;
#[doc = "Field `SW_C1CC` writer - Set corresponding switch"]
pub type SW_C1CC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_C1CD` reader - Set corresponding switch"]
pub type SW_C1CD_R = crate::BitReader;
#[doc = "Field `SW_C1CD` writer - Set corresponding switch"]
pub type SW_C1CD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_C1F1` reader - Set corresponding switch"]
pub type SW_C1F1_R = crate::BitReader;
#[doc = "Field `SW_C1F1` writer - Set corresponding switch"]
pub type SW_C1F1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_f1pm(&self) -> SW_F1PM_R {
        SW_F1PM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:10 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_f1ma(&self) -> SW_F1MA_R {
        SW_F1MA_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_f1ca(&self) -> SW_F1CA_R {
        SW_F1CA_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c1cc(&self) -> SW_C1CC_R {
        SW_C1CC_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c1cd(&self) -> SW_C1CD_R {
        SW_C1CD_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c1f1(&self) -> SW_C1F1_R {
        SW_C1F1_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_f1pm(&mut self) -> SW_F1PM_W<SW_FW_MOD_SEL_SPEC> {
        SW_F1PM_W::new(self, 0)
    }
    #[doc = "Bits 8:10 - Select waveform for corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_f1ma(&mut self) -> SW_F1MA_W<SW_FW_MOD_SEL_SPEC> {
        SW_F1MA_W::new(self, 8)
    }
    #[doc = "Bits 16:18 - Select waveform for corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_f1ca(&mut self) -> SW_F1CA_W<SW_FW_MOD_SEL_SPEC> {
        SW_F1CA_W::new(self, 16)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_c1cc(&mut self) -> SW_C1CC_W<SW_FW_MOD_SEL_SPEC> {
        SW_C1CC_W::new(self, 20)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_c1cd(&mut self) -> SW_C1CD_W<SW_FW_MOD_SEL_SPEC> {
        SW_C1CD_W::new(self, 24)
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_c1f1(&mut self) -> SW_C1F1_W<SW_FW_MOD_SEL_SPEC> {
        SW_C1F1_W::new(self, 28)
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
#[doc = "Full Wave Cmod Switch Waveform selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_fw_mod_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_fw_mod_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SW_FW_MOD_SEL_SPEC;
impl crate::RegisterSpec for SW_FW_MOD_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sw_fw_mod_sel::R`](R) reader structure"]
impl crate::Readable for SW_FW_MOD_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sw_fw_mod_sel::W`](W) writer structure"]
impl crate::Writable for SW_FW_MOD_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SW_FW_MOD_SEL to value 0"]
impl crate::Resettable for SW_FW_MOD_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
