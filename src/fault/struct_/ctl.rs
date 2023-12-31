#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `TR_EN` reader - Trigger output enable: '0': Disabled. The trigger output 'tr_fault' is '0'. '1': Enabled. The trigger output 'tr_fault' reflects STATUS.VALID. The trigger can be used to initiate a Datawire transfer of the FAULT data (FAULT_DATA0 through FAULT_DATA3)."]
pub type TR_EN_R = crate::BitReader;
#[doc = "Field `TR_EN` writer - Trigger output enable: '0': Disabled. The trigger output 'tr_fault' is '0'. '1': Enabled. The trigger output 'tr_fault' reflects STATUS.VALID. The trigger can be used to initiate a Datawire transfer of the FAULT data (FAULT_DATA0 through FAULT_DATA3)."]
pub type TR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EN` reader - IO output signal enable: '0': Disabled. The IO output signal 'fault_out' is '0'. The IO output enable signal 'fault_out_en' is '0'. '1': Enabled. The IO output signal 'fault_out' reflects STATUS.VALID. The IO output enable signal 'fault_out_en' is '1'."]
pub type OUT_EN_R = crate::BitReader;
#[doc = "Field `OUT_EN` writer - IO output signal enable: '0': Disabled. The IO output signal 'fault_out' is '0'. The IO output enable signal 'fault_out_en' is '0'. '1': Enabled. The IO output signal 'fault_out' reflects STATUS.VALID. The IO output enable signal 'fault_out_en' is '1'."]
pub type OUT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_REQ_EN` reader - Reset request enable: '0': Disabled. '1': Enabled. The output reset request signal 'fault_reset_req' reflects STATUS.VALID. This reset causes a warm/soft/core reset. This warm/soft/core reset does not affect the fault logic STATUS, DATA0, ..., DATA3 registers (allowing for post soft reset failure analysis). The 'fault_reset_req' signals of the individual fault report structures are combined (logically OR'd) into a single SRSS 'fault_reset_req' signal."]
pub type RESET_REQ_EN_R = crate::BitReader;
#[doc = "Field `RESET_REQ_EN` writer - Reset request enable: '0': Disabled. '1': Enabled. The output reset request signal 'fault_reset_req' reflects STATUS.VALID. This reset causes a warm/soft/core reset. This warm/soft/core reset does not affect the fault logic STATUS, DATA0, ..., DATA3 registers (allowing for post soft reset failure analysis). The 'fault_reset_req' signals of the individual fault report structures are combined (logically OR'd) into a single SRSS 'fault_reset_req' signal."]
pub type RESET_REQ_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Trigger output enable: '0': Disabled. The trigger output 'tr_fault' is '0'. '1': Enabled. The trigger output 'tr_fault' reflects STATUS.VALID. The trigger can be used to initiate a Datawire transfer of the FAULT data (FAULT_DATA0 through FAULT_DATA3)."]
    #[inline(always)]
    pub fn tr_en(&self) -> TR_EN_R {
        TR_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO output signal enable: '0': Disabled. The IO output signal 'fault_out' is '0'. The IO output enable signal 'fault_out_en' is '0'. '1': Enabled. The IO output signal 'fault_out' reflects STATUS.VALID. The IO output enable signal 'fault_out_en' is '1'."]
    #[inline(always)]
    pub fn out_en(&self) -> OUT_EN_R {
        OUT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset request enable: '0': Disabled. '1': Enabled. The output reset request signal 'fault_reset_req' reflects STATUS.VALID. This reset causes a warm/soft/core reset. This warm/soft/core reset does not affect the fault logic STATUS, DATA0, ..., DATA3 registers (allowing for post soft reset failure analysis). The 'fault_reset_req' signals of the individual fault report structures are combined (logically OR'd) into a single SRSS 'fault_reset_req' signal."]
    #[inline(always)]
    pub fn reset_req_en(&self) -> RESET_REQ_EN_R {
        RESET_REQ_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger output enable: '0': Disabled. The trigger output 'tr_fault' is '0'. '1': Enabled. The trigger output 'tr_fault' reflects STATUS.VALID. The trigger can be used to initiate a Datawire transfer of the FAULT data (FAULT_DATA0 through FAULT_DATA3)."]
    #[inline(always)]
    #[must_use]
    pub fn tr_en(&mut self) -> TR_EN_W<CTL_SPEC> {
        TR_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - IO output signal enable: '0': Disabled. The IO output signal 'fault_out' is '0'. The IO output enable signal 'fault_out_en' is '0'. '1': Enabled. The IO output signal 'fault_out' reflects STATUS.VALID. The IO output enable signal 'fault_out_en' is '1'."]
    #[inline(always)]
    #[must_use]
    pub fn out_en(&mut self) -> OUT_EN_W<CTL_SPEC> {
        OUT_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Reset request enable: '0': Disabled. '1': Enabled. The output reset request signal 'fault_reset_req' reflects STATUS.VALID. This reset causes a warm/soft/core reset. This warm/soft/core reset does not affect the fault logic STATUS, DATA0, ..., DATA3 registers (allowing for post soft reset failure analysis). The 'fault_reset_req' signals of the individual fault report structures are combined (logically OR'd) into a single SRSS 'fault_reset_req' signal."]
    #[inline(always)]
    #[must_use]
    pub fn reset_req_en(&mut self) -> RESET_REQ_EN_W<CTL_SPEC> {
        RESET_REQ_EN_W::new(self, 2)
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
#[doc = "Fault control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
