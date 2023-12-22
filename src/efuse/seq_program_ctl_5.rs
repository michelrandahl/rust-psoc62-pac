#[doc = "Register `SEQ_PROGRAM_CTL_5` reader"]
pub type R = crate::R<SEQ_PROGRAM_CTL_5_SPEC>;
#[doc = "Register `SEQ_PROGRAM_CTL_5` writer"]
pub type W = crate::W<SEQ_PROGRAM_CTL_5_SPEC>;
#[doc = "Field `CYCLES` reader - Number of IP clock cycles (minus 1). This field is in the range of \\[0, 1023\\], allowing for a time of \\[1, 1024\\]
IP clock cycles."]
pub type CYCLES_R = crate::FieldReader<u16>;
#[doc = "Field `CYCLES` writer - Number of IP clock cycles (minus 1). This field is in the range of \\[0, 1023\\], allowing for a time of \\[1, 1024\\]
IP clock cycles."]
pub type CYCLES_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `STROBE_A` reader - Specifies value of eFUSE control signal strobe_a"]
pub type STROBE_A_R = crate::BitReader;
#[doc = "Field `STROBE_A` writer - Specifies value of eFUSE control signal strobe_a"]
pub type STROBE_A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STROBE_B` reader - Specifies value of eFUSEcontrol signal strobe_b"]
pub type STROBE_B_R = crate::BitReader;
#[doc = "Field `STROBE_B` writer - Specifies value of eFUSEcontrol signal strobe_b"]
pub type STROBE_B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STROBE_C` reader - Specifies value of eFUSE control signal strobe_c"]
pub type STROBE_C_R = crate::BitReader;
#[doc = "Field `STROBE_C` writer - Specifies value of eFUSE control signal strobe_c"]
pub type STROBE_C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STROBE_D` reader - Specifies value of eFUSE control signal strobe_d"]
pub type STROBE_D_R = crate::BitReader;
#[doc = "Field `STROBE_D` writer - Specifies value of eFUSE control signal strobe_d"]
pub type STROBE_D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STROBE_E` reader - Specifies value of eFUSE control signal strobe_e"]
pub type STROBE_E_R = crate::BitReader;
#[doc = "Field `STROBE_E` writer - Specifies value of eFUSE control signal strobe_e"]
pub type STROBE_E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STROBE_F` reader - Specifies value of eFUSE control signal strobe_f"]
pub type STROBE_F_R = crate::BitReader;
#[doc = "Field `STROBE_F` writer - Specifies value of eFUSE control signal strobe_f"]
pub type STROBE_F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STROBE_G` reader - Specifies value of eFUSE control signal strobe_g"]
pub type STROBE_G_R = crate::BitReader;
#[doc = "Field `STROBE_G` writer - Specifies value of eFUSE control signal strobe_g"]
pub type STROBE_G_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE` reader - When set to 1 indicates that the Read cycle ends when the current cycle count reaches 0."]
pub type DONE_R = crate::BitReader;
#[doc = "Field `DONE` writer - When set to 1 indicates that the Read cycle ends when the current cycle count reaches 0."]
pub type DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Number of IP clock cycles (minus 1). This field is in the range of \\[0, 1023\\], allowing for a time of \\[1, 1024\\]
IP clock cycles."]
    #[inline(always)]
    pub fn cycles(&self) -> CYCLES_R {
        CYCLES_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 16 - Specifies value of eFUSE control signal strobe_a"]
    #[inline(always)]
    pub fn strobe_a(&self) -> STROBE_A_R {
        STROBE_A_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Specifies value of eFUSEcontrol signal strobe_b"]
    #[inline(always)]
    pub fn strobe_b(&self) -> STROBE_B_R {
        STROBE_B_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Specifies value of eFUSE control signal strobe_c"]
    #[inline(always)]
    pub fn strobe_c(&self) -> STROBE_C_R {
        STROBE_C_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Specifies value of eFUSE control signal strobe_d"]
    #[inline(always)]
    pub fn strobe_d(&self) -> STROBE_D_R {
        STROBE_D_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Specifies value of eFUSE control signal strobe_e"]
    #[inline(always)]
    pub fn strobe_e(&self) -> STROBE_E_R {
        STROBE_E_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub fn strobe_f(&self) -> STROBE_F_R {
        STROBE_F_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Specifies value of eFUSE control signal strobe_g"]
    #[inline(always)]
    pub fn strobe_g(&self) -> STROBE_G_R {
        STROBE_G_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 31 - When set to 1 indicates that the Read cycle ends when the current cycle count reaches 0."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Number of IP clock cycles (minus 1). This field is in the range of \\[0, 1023\\], allowing for a time of \\[1, 1024\\]
IP clock cycles."]
    #[inline(always)]
    #[must_use]
    pub fn cycles(&mut self) -> CYCLES_W<SEQ_PROGRAM_CTL_5_SPEC> {
        CYCLES_W::new(self, 0)
    }
    #[doc = "Bit 16 - Specifies value of eFUSE control signal strobe_a"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_a(&mut self) -> STROBE_A_W<SEQ_PROGRAM_CTL_5_SPEC> {
        STROBE_A_W::new(self, 16)
    }
    #[doc = "Bit 17 - Specifies value of eFUSEcontrol signal strobe_b"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_b(&mut self) -> STROBE_B_W<SEQ_PROGRAM_CTL_5_SPEC> {
        STROBE_B_W::new(self, 17)
    }
    #[doc = "Bit 18 - Specifies value of eFUSE control signal strobe_c"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_c(&mut self) -> STROBE_C_W<SEQ_PROGRAM_CTL_5_SPEC> {
        STROBE_C_W::new(self, 18)
    }
    #[doc = "Bit 19 - Specifies value of eFUSE control signal strobe_d"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_d(&mut self) -> STROBE_D_W<SEQ_PROGRAM_CTL_5_SPEC> {
        STROBE_D_W::new(self, 19)
    }
    #[doc = "Bit 20 - Specifies value of eFUSE control signal strobe_e"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_e(&mut self) -> STROBE_E_W<SEQ_PROGRAM_CTL_5_SPEC> {
        STROBE_E_W::new(self, 20)
    }
    #[doc = "Bit 21 - Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_f(&mut self) -> STROBE_F_W<SEQ_PROGRAM_CTL_5_SPEC> {
        STROBE_F_W::new(self, 21)
    }
    #[doc = "Bit 22 - Specifies value of eFUSE control signal strobe_g"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_g(&mut self) -> STROBE_G_W<SEQ_PROGRAM_CTL_5_SPEC> {
        STROBE_G_W::new(self, 22)
    }
    #[doc = "Bit 31 - When set to 1 indicates that the Read cycle ends when the current cycle count reaches 0."]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DONE_W<SEQ_PROGRAM_CTL_5_SPEC> {
        DONE_W::new(self, 31)
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
#[doc = "Sequencer program control 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq_program_ctl_5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq_program_ctl_5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEQ_PROGRAM_CTL_5_SPEC;
impl crate::RegisterSpec for SEQ_PROGRAM_CTL_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seq_program_ctl_5::R`](R) reader structure"]
impl crate::Readable for SEQ_PROGRAM_CTL_5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`seq_program_ctl_5::W`](W) writer structure"]
impl crate::Writable for SEQ_PROGRAM_CTL_5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEQ_PROGRAM_CTL_5 to value 0x803d_0019"]
impl crate::Resettable for SEQ_PROGRAM_CTL_5_SPEC {
    const RESET_VALUE: Self::Ux = 0x803d_0019;
}
