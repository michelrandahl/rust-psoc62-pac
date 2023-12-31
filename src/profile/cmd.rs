#[doc = "Register `CMD` reader"]
pub type R = crate::R<CMD_SPEC>;
#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMD_SPEC>;
#[doc = "Field `START_TR` reader - Software start trigger for the profiling time window. When written with '1', the profiling time window is started. Can only be used in start / stop mode (PROFILE_WIN_MODE=0). Has no effect in enable mode (PROFILE_WIN_MODE=1)."]
pub type START_TR_R = crate::BitReader;
#[doc = "Field `START_TR` writer - Software start trigger for the profiling time window. When written with '1', the profiling time window is started. Can only be used in start / stop mode (PROFILE_WIN_MODE=0). Has no effect in enable mode (PROFILE_WIN_MODE=1)."]
pub type START_TR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP_TR` reader - Software stop trigger for the profiling time window. When written with '1', the profiling time window is stopped. Can only be used in start / stop mode (PROFILE_WIN_MODE=0). Has no effect in enable mode (PROFILE_WIN_MODE=1)."]
pub type STOP_TR_R = crate::BitReader;
#[doc = "Field `STOP_TR` writer - Software stop trigger for the profiling time window. When written with '1', the profiling time window is stopped. Can only be used in start / stop mode (PROFILE_WIN_MODE=0). Has no effect in enable mode (PROFILE_WIN_MODE=1)."]
pub type STOP_TR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_ALL_CNT` reader - Counter clear. When written with '1', all profiling counter registers are cleared to 0x00."]
pub type CLR_ALL_CNT_R = crate::BitReader;
#[doc = "Field `CLR_ALL_CNT` writer - Counter clear. When written with '1', all profiling counter registers are cleared to 0x00."]
pub type CLR_ALL_CNT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software start trigger for the profiling time window. When written with '1', the profiling time window is started. Can only be used in start / stop mode (PROFILE_WIN_MODE=0). Has no effect in enable mode (PROFILE_WIN_MODE=1)."]
    #[inline(always)]
    pub fn start_tr(&self) -> START_TR_R {
        START_TR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software stop trigger for the profiling time window. When written with '1', the profiling time window is stopped. Can only be used in start / stop mode (PROFILE_WIN_MODE=0). Has no effect in enable mode (PROFILE_WIN_MODE=1)."]
    #[inline(always)]
    pub fn stop_tr(&self) -> STOP_TR_R {
        STOP_TR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Counter clear. When written with '1', all profiling counter registers are cleared to 0x00."]
    #[inline(always)]
    pub fn clr_all_cnt(&self) -> CLR_ALL_CNT_R {
        CLR_ALL_CNT_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software start trigger for the profiling time window. When written with '1', the profiling time window is started. Can only be used in start / stop mode (PROFILE_WIN_MODE=0). Has no effect in enable mode (PROFILE_WIN_MODE=1)."]
    #[inline(always)]
    #[must_use]
    pub fn start_tr(&mut self) -> START_TR_W<CMD_SPEC> {
        START_TR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Software stop trigger for the profiling time window. When written with '1', the profiling time window is stopped. Can only be used in start / stop mode (PROFILE_WIN_MODE=0). Has no effect in enable mode (PROFILE_WIN_MODE=1)."]
    #[inline(always)]
    #[must_use]
    pub fn stop_tr(&mut self) -> STOP_TR_W<CMD_SPEC> {
        STOP_TR_W::new(self, 1)
    }
    #[doc = "Bit 8 - Counter clear. When written with '1', all profiling counter registers are cleared to 0x00."]
    #[inline(always)]
    #[must_use]
    pub fn clr_all_cnt(&mut self) -> CLR_ALL_CNT_W<CMD_SPEC> {
        CLR_ALL_CNT_W::new(self, 8)
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
#[doc = "Profile command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd::R`](R) reader structure"]
impl crate::Readable for CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
