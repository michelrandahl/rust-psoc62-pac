#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `COUNTER_ENABLED` reader - Counter enables for counters 0 up to CNT_NR-1. '0': counter disabled. '1': counter enabled. Counter static configuration information (e.g. CTRL.MODE, all TR_CTRL0, TR_CTRL1, and TR_CTRL2 register fields) should only be modified when the counter is disabled. When a counter is disabled, command and status information associated to the counter is cleared by HW, this includes: - the associated counter triggers in the CMD register are set to '0'. - the counter's interrupt cause fields in counter's INTR register. - the counter's status fields in counter's STATUS register.. - the counter's trigger outputs ('tr_overflow', 'tr_underflow' and 'tr_compare_match'). - the counter's line outputs ('line_out' and 'line_compl_out'). In multi-core environments, use the CTRL_SET/CTRL_CLR registers to avoid race-conditions on read-modify-write attempts to this register."]
pub type COUNTER_ENABLED_R = crate::FieldReader<u32>;
#[doc = "Field `COUNTER_ENABLED` writer - Counter enables for counters 0 up to CNT_NR-1. '0': counter disabled. '1': counter enabled. Counter static configuration information (e.g. CTRL.MODE, all TR_CTRL0, TR_CTRL1, and TR_CTRL2 register fields) should only be modified when the counter is disabled. When a counter is disabled, command and status information associated to the counter is cleared by HW, this includes: - the associated counter triggers in the CMD register are set to '0'. - the counter's interrupt cause fields in counter's INTR register. - the counter's status fields in counter's STATUS register.. - the counter's trigger outputs ('tr_overflow', 'tr_underflow' and 'tr_compare_match'). - the counter's line outputs ('line_out' and 'line_compl_out'). In multi-core environments, use the CTRL_SET/CTRL_CLR registers to avoid race-conditions on read-modify-write attempts to this register."]
pub type COUNTER_ENABLED_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Counter enables for counters 0 up to CNT_NR-1. '0': counter disabled. '1': counter enabled. Counter static configuration information (e.g. CTRL.MODE, all TR_CTRL0, TR_CTRL1, and TR_CTRL2 register fields) should only be modified when the counter is disabled. When a counter is disabled, command and status information associated to the counter is cleared by HW, this includes: - the associated counter triggers in the CMD register are set to '0'. - the counter's interrupt cause fields in counter's INTR register. - the counter's status fields in counter's STATUS register.. - the counter's trigger outputs ('tr_overflow', 'tr_underflow' and 'tr_compare_match'). - the counter's line outputs ('line_out' and 'line_compl_out'). In multi-core environments, use the CTRL_SET/CTRL_CLR registers to avoid race-conditions on read-modify-write attempts to this register."]
    #[inline(always)]
    pub fn counter_enabled(&self) -> COUNTER_ENABLED_R {
        COUNTER_ENABLED_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counter enables for counters 0 up to CNT_NR-1. '0': counter disabled. '1': counter enabled. Counter static configuration information (e.g. CTRL.MODE, all TR_CTRL0, TR_CTRL1, and TR_CTRL2 register fields) should only be modified when the counter is disabled. When a counter is disabled, command and status information associated to the counter is cleared by HW, this includes: - the associated counter triggers in the CMD register are set to '0'. - the counter's interrupt cause fields in counter's INTR register. - the counter's status fields in counter's STATUS register.. - the counter's trigger outputs ('tr_overflow', 'tr_underflow' and 'tr_compare_match'). - the counter's line outputs ('line_out' and 'line_compl_out'). In multi-core environments, use the CTRL_SET/CTRL_CLR registers to avoid race-conditions on read-modify-write attempts to this register."]
    #[inline(always)]
    #[must_use]
    pub fn counter_enabled(&mut self) -> COUNTER_ENABLED_W<CTRL_SPEC> {
        COUNTER_ENABLED_W::new(self, 0)
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
#[doc = "TCPWM control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
