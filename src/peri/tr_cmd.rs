#[doc = "Register `TR_CMD` reader"]
pub type R = crate::R<TR_CMD_SPEC>;
#[doc = "Register `TR_CMD` writer"]
pub type W = crate::W<TR_CMD_SPEC>;
#[doc = "Field `TR_SEL` reader - Specifies the activated trigger when ACTIVATE is '1'. If the specified trigger is not present, the trigger activation has no effect."]
pub type TR_SEL_R = crate::FieldReader;
#[doc = "Field `TR_SEL` writer - Specifies the activated trigger when ACTIVATE is '1'. If the specified trigger is not present, the trigger activation has no effect."]
pub type TR_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GROUP_SEL` reader - Specifies the trigger group: '0'-'15': trigger multiplexer groups. '16'-'31': trigger 1-to-1 groups."]
pub type GROUP_SEL_R = crate::FieldReader;
#[doc = "Field `GROUP_SEL` writer - Specifies the trigger group: '0'-'15': trigger multiplexer groups. '16'-'31': trigger 1-to-1 groups."]
pub type GROUP_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TR_EDGE` reader - Specifies if the activated trigger is treated as a level sensitive or edge sensitive trigger. '0': level sensitive. The trigger reflects TR_CMD.ACTIVATE. '1': edge sensitive trigger. The trigger is activated for two clk_peri cycles."]
pub type TR_EDGE_R = crate::BitReader;
#[doc = "Field `TR_EDGE` writer - Specifies if the activated trigger is treated as a level sensitive or edge sensitive trigger. '0': level sensitive. The trigger reflects TR_CMD.ACTIVATE. '1': edge sensitive trigger. The trigger is activated for two clk_peri cycles."]
pub type TR_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_SEL` reader - Specifies whether trigger activation is for a specific input or output trigger of the trigger multiplexer. Activation of a specific input trigger, will result in activation of all output triggers that have the specific input trigger selected through their TR_OUT_CTL.TR_SEL field. Activation of a specific output trigger, will result in activation of the specified TR_SEL output trigger only. '0': TR_SEL selection and trigger activation is for an input trigger to the trigger multiplexer. '1': TR_SEL selection and trigger activation is for an output trigger from the trigger multiplexer. Note: this field is not used for trigger 1-to-1 groups."]
pub type OUT_SEL_R = crate::BitReader;
#[doc = "Field `OUT_SEL` writer - Specifies whether trigger activation is for a specific input or output trigger of the trigger multiplexer. Activation of a specific input trigger, will result in activation of all output triggers that have the specific input trigger selected through their TR_OUT_CTL.TR_SEL field. Activation of a specific output trigger, will result in activation of the specified TR_SEL output trigger only. '0': TR_SEL selection and trigger activation is for an input trigger to the trigger multiplexer. '1': TR_SEL selection and trigger activation is for an output trigger from the trigger multiplexer. Note: this field is not used for trigger 1-to-1 groups."]
pub type OUT_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVATE` reader - SW sets this field to '1' to activate (set to '1') a trigger as identified by TR_SEL, TR_EDGE and OUT_SEL. HW sets this field to '0' for edge sensitive triggers AFTER the selected trigger is activated for two clk_peri cycles. Note: when ACTIVATE is '1', SW should not modify the other register fields."]
pub type ACTIVATE_R = crate::BitReader;
#[doc = "Field `ACTIVATE` writer - SW sets this field to '1' to activate (set to '1') a trigger as identified by TR_SEL, TR_EDGE and OUT_SEL. HW sets this field to '0' for edge sensitive triggers AFTER the selected trigger is activated for two clk_peri cycles. Note: when ACTIVATE is '1', SW should not modify the other register fields."]
pub type ACTIVATE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Specifies the activated trigger when ACTIVATE is '1'. If the specified trigger is not present, the trigger activation has no effect."]
    #[inline(always)]
    pub fn tr_sel(&self) -> TR_SEL_R {
        TR_SEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - Specifies the trigger group: '0'-'15': trigger multiplexer groups. '16'-'31': trigger 1-to-1 groups."]
    #[inline(always)]
    pub fn group_sel(&self) -> GROUP_SEL_R {
        GROUP_SEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - Specifies if the activated trigger is treated as a level sensitive or edge sensitive trigger. '0': level sensitive. The trigger reflects TR_CMD.ACTIVATE. '1': edge sensitive trigger. The trigger is activated for two clk_peri cycles."]
    #[inline(always)]
    pub fn tr_edge(&self) -> TR_EDGE_R {
        TR_EDGE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Specifies whether trigger activation is for a specific input or output trigger of the trigger multiplexer. Activation of a specific input trigger, will result in activation of all output triggers that have the specific input trigger selected through their TR_OUT_CTL.TR_SEL field. Activation of a specific output trigger, will result in activation of the specified TR_SEL output trigger only. '0': TR_SEL selection and trigger activation is for an input trigger to the trigger multiplexer. '1': TR_SEL selection and trigger activation is for an output trigger from the trigger multiplexer. Note: this field is not used for trigger 1-to-1 groups."]
    #[inline(always)]
    pub fn out_sel(&self) -> OUT_SEL_R {
        OUT_SEL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SW sets this field to '1' to activate (set to '1') a trigger as identified by TR_SEL, TR_EDGE and OUT_SEL. HW sets this field to '0' for edge sensitive triggers AFTER the selected trigger is activated for two clk_peri cycles. Note: when ACTIVATE is '1', SW should not modify the other register fields."]
    #[inline(always)]
    pub fn activate(&self) -> ACTIVATE_R {
        ACTIVATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Specifies the activated trigger when ACTIVATE is '1'. If the specified trigger is not present, the trigger activation has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn tr_sel(&mut self) -> TR_SEL_W<TR_CMD_SPEC> {
        TR_SEL_W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Specifies the trigger group: '0'-'15': trigger multiplexer groups. '16'-'31': trigger 1-to-1 groups."]
    #[inline(always)]
    #[must_use]
    pub fn group_sel(&mut self) -> GROUP_SEL_W<TR_CMD_SPEC> {
        GROUP_SEL_W::new(self, 8)
    }
    #[doc = "Bit 29 - Specifies if the activated trigger is treated as a level sensitive or edge sensitive trigger. '0': level sensitive. The trigger reflects TR_CMD.ACTIVATE. '1': edge sensitive trigger. The trigger is activated for two clk_peri cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tr_edge(&mut self) -> TR_EDGE_W<TR_CMD_SPEC> {
        TR_EDGE_W::new(self, 29)
    }
    #[doc = "Bit 30 - Specifies whether trigger activation is for a specific input or output trigger of the trigger multiplexer. Activation of a specific input trigger, will result in activation of all output triggers that have the specific input trigger selected through their TR_OUT_CTL.TR_SEL field. Activation of a specific output trigger, will result in activation of the specified TR_SEL output trigger only. '0': TR_SEL selection and trigger activation is for an input trigger to the trigger multiplexer. '1': TR_SEL selection and trigger activation is for an output trigger from the trigger multiplexer. Note: this field is not used for trigger 1-to-1 groups."]
    #[inline(always)]
    #[must_use]
    pub fn out_sel(&mut self) -> OUT_SEL_W<TR_CMD_SPEC> {
        OUT_SEL_W::new(self, 30)
    }
    #[doc = "Bit 31 - SW sets this field to '1' to activate (set to '1') a trigger as identified by TR_SEL, TR_EDGE and OUT_SEL. HW sets this field to '0' for edge sensitive triggers AFTER the selected trigger is activated for two clk_peri cycles. Note: when ACTIVATE is '1', SW should not modify the other register fields."]
    #[inline(always)]
    #[must_use]
    pub fn activate(&mut self) -> ACTIVATE_W<TR_CMD_SPEC> {
        ACTIVATE_W::new(self, 31)
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
#[doc = "Trigger command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tr_cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tr_cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TR_CMD_SPEC;
impl crate::RegisterSpec for TR_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr_cmd::R`](R) reader structure"]
impl crate::Readable for TR_CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tr_cmd::W`](W) writer structure"]
impl crate::Writable for TR_CMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TR_CMD to value 0"]
impl crate::Resettable for TR_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
