#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `AUTO_RELOAD_CC` reader - Specifies switching of the CC and buffered CC values. This field has a function in TIMER, PWM, PWM_DT and PWM_PR modes. Timer mode: '0': never switch. '1': switch on a compare match event. PWM, PWM_DT, PWM_PR modes: '0: never switch. '1': switch on a terminal count event with an actively pending switch event."]
pub type AUTO_RELOAD_CC_R = crate::BitReader;
#[doc = "Field `AUTO_RELOAD_CC` writer - Specifies switching of the CC and buffered CC values. This field has a function in TIMER, PWM, PWM_DT and PWM_PR modes. Timer mode: '0': never switch. '1': switch on a compare match event. PWM, PWM_DT, PWM_PR modes: '0: never switch. '1': switch on a terminal count event with an actively pending switch event."]
pub type AUTO_RELOAD_CC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_RELOAD_PERIOD` reader - Specifies switching of the PERIOD and buffered PERIOD values. This field has a function in PWM, PWM_DT and PWM_PR modes. '0': never switch. '1': switch on a terminal count event with and actively pending switch event."]
pub type AUTO_RELOAD_PERIOD_R = crate::BitReader;
#[doc = "Field `AUTO_RELOAD_PERIOD` writer - Specifies switching of the PERIOD and buffered PERIOD values. This field has a function in PWM, PWM_DT and PWM_PR modes. '0': never switch. '1': switch on a terminal count event with and actively pending switch event."]
pub type AUTO_RELOAD_PERIOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM_SYNC_KILL` reader - Specifies asynchronous/synchronous kill behavior: '1': synchronous kill mode: the kill event disables the 'dt_line_out' and 'dt_line_compl_out' signals till the next terminal count event (synchronous kill). In synchronous kill mode, STOP_EDGE should be RISING_EDGE. '0': asynchronous kill mode: the kill event only disables the 'dt_line_out' and 'dt_line_compl_out' signals when present. In asynchronous kill mode, STOP_EDGE should be NO_EDGE_DET. This field has a function in PWM and PWM_DT modes only. This field is only used when PWM_STOP_ON_KILL is '0'."]
pub type PWM_SYNC_KILL_R = crate::BitReader;
#[doc = "Field `PWM_SYNC_KILL` writer - Specifies asynchronous/synchronous kill behavior: '1': synchronous kill mode: the kill event disables the 'dt_line_out' and 'dt_line_compl_out' signals till the next terminal count event (synchronous kill). In synchronous kill mode, STOP_EDGE should be RISING_EDGE. '0': asynchronous kill mode: the kill event only disables the 'dt_line_out' and 'dt_line_compl_out' signals when present. In asynchronous kill mode, STOP_EDGE should be NO_EDGE_DET. This field has a function in PWM and PWM_DT modes only. This field is only used when PWM_STOP_ON_KILL is '0'."]
pub type PWM_SYNC_KILL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM_STOP_ON_KILL` reader - Specifies whether the counter stops on a kill events: '0': kill event does NOT stop counter. '1': kill event stops counter. This field has a function in PWM, PWM_DT and PWM_PR modes only."]
pub type PWM_STOP_ON_KILL_R = crate::BitReader;
#[doc = "Field `PWM_STOP_ON_KILL` writer - Specifies whether the counter stops on a kill events: '0': kill event does NOT stop counter. '1': kill event stops counter. This field has a function in PWM, PWM_DT and PWM_PR modes only."]
pub type PWM_STOP_ON_KILL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GENERIC` reader - Generic 8-bit control field. In PWM_DT mode, this field is used to determine the dead time: amount of dead time cycles in the counter clock domain. In all other modes, the lower 3 bits of this field determine pre-scaling of the selected counter clock."]
pub type GENERIC_R = crate::FieldReader<GENERIC_A>;
#[doc = "Generic 8-bit control field. In PWM_DT mode, this field is used to determine the dead time: amount of dead time cycles in the counter clock domain. In all other modes, the lower 3 bits of this field determine pre-scaling of the selected counter clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GENERIC_A {
    #[doc = "0: Divide by 1 (other-than-PWM_DT mode)"]
    DIVBY1 = 0,
    #[doc = "1: Divide by 2 (other-than-PWM_DT mode)"]
    DIVBY2 = 1,
    #[doc = "2: Divide by 4 (other-than-PWM_DT mode)"]
    DIVBY4 = 2,
    #[doc = "3: Divide by 8 (other-than-PWM_DT mode)"]
    DIVBY8 = 3,
    #[doc = "4: Divide by 16 (other-than-PWM_DT mode)"]
    DIVBY16 = 4,
    #[doc = "5: Divide by 32 (other-than-PWM_DT mode)"]
    DIVBY32 = 5,
    #[doc = "6: Divide by 64 (other-than-PWM_DT mode)"]
    DIVBY64 = 6,
    #[doc = "7: Divide by 128 (other-than-PWM_DT mode)"]
    DIVBY128 = 7,
}
impl From<GENERIC_A> for u8 {
    #[inline(always)]
    fn from(variant: GENERIC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GENERIC_A {
    type Ux = u8;
}
impl GENERIC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GENERIC_A> {
        match self.bits {
            0 => Some(GENERIC_A::DIVBY1),
            1 => Some(GENERIC_A::DIVBY2),
            2 => Some(GENERIC_A::DIVBY4),
            3 => Some(GENERIC_A::DIVBY8),
            4 => Some(GENERIC_A::DIVBY16),
            5 => Some(GENERIC_A::DIVBY32),
            6 => Some(GENERIC_A::DIVBY64),
            7 => Some(GENERIC_A::DIVBY128),
            _ => None,
        }
    }
    #[doc = "Divide by 1 (other-than-PWM_DT mode)"]
    #[inline(always)]
    pub fn is_divby1(&self) -> bool {
        *self == GENERIC_A::DIVBY1
    }
    #[doc = "Divide by 2 (other-than-PWM_DT mode)"]
    #[inline(always)]
    pub fn is_divby2(&self) -> bool {
        *self == GENERIC_A::DIVBY2
    }
    #[doc = "Divide by 4 (other-than-PWM_DT mode)"]
    #[inline(always)]
    pub fn is_divby4(&self) -> bool {
        *self == GENERIC_A::DIVBY4
    }
    #[doc = "Divide by 8 (other-than-PWM_DT mode)"]
    #[inline(always)]
    pub fn is_divby8(&self) -> bool {
        *self == GENERIC_A::DIVBY8
    }
    #[doc = "Divide by 16 (other-than-PWM_DT mode)"]
    #[inline(always)]
    pub fn is_divby16(&self) -> bool {
        *self == GENERIC_A::DIVBY16
    }
    #[doc = "Divide by 32 (other-than-PWM_DT mode)"]
    #[inline(always)]
    pub fn is_divby32(&self) -> bool {
        *self == GENERIC_A::DIVBY32
    }
    #[doc = "Divide by 64 (other-than-PWM_DT mode)"]
    #[inline(always)]
    pub fn is_divby64(&self) -> bool {
        *self == GENERIC_A::DIVBY64
    }
    #[doc = "Divide by 128 (other-than-PWM_DT mode)"]
    #[inline(always)]
    pub fn is_divby128(&self) -> bool {
        *self == GENERIC_A::DIVBY128
    }
}
#[doc = "Field `GENERIC` writer - Generic 8-bit control field. In PWM_DT mode, this field is used to determine the dead time: amount of dead time cycles in the counter clock domain. In all other modes, the lower 3 bits of this field determine pre-scaling of the selected counter clock."]
pub type GENERIC_W<'a, REG> = crate::FieldWriter<'a, REG, 8, GENERIC_A>;
impl<'a, REG> GENERIC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1 (other-than-PWM_DT mode)"]
    #[inline(always)]
    pub fn divby1(self) -> &'a mut crate::W<REG> {
        self.variant(GENERIC_A::DIVBY1)
    }
    #[doc = "Divide by 2 (other-than-PWM_DT mode)"]
    #[inline(always)]
    pub fn divby2(self) -> &'a mut crate::W<REG> {
        self.variant(GENERIC_A::DIVBY2)
    }
    #[doc = "Divide by 4 (other-than-PWM_DT mode)"]
    #[inline(always)]
    pub fn divby4(self) -> &'a mut crate::W<REG> {
        self.variant(GENERIC_A::DIVBY4)
    }
    #[doc = "Divide by 8 (other-than-PWM_DT mode)"]
    #[inline(always)]
    pub fn divby8(self) -> &'a mut crate::W<REG> {
        self.variant(GENERIC_A::DIVBY8)
    }
    #[doc = "Divide by 16 (other-than-PWM_DT mode)"]
    #[inline(always)]
    pub fn divby16(self) -> &'a mut crate::W<REG> {
        self.variant(GENERIC_A::DIVBY16)
    }
    #[doc = "Divide by 32 (other-than-PWM_DT mode)"]
    #[inline(always)]
    pub fn divby32(self) -> &'a mut crate::W<REG> {
        self.variant(GENERIC_A::DIVBY32)
    }
    #[doc = "Divide by 64 (other-than-PWM_DT mode)"]
    #[inline(always)]
    pub fn divby64(self) -> &'a mut crate::W<REG> {
        self.variant(GENERIC_A::DIVBY64)
    }
    #[doc = "Divide by 128 (other-than-PWM_DT mode)"]
    #[inline(always)]
    pub fn divby128(self) -> &'a mut crate::W<REG> {
        self.variant(GENERIC_A::DIVBY128)
    }
}
#[doc = "Field `UP_DOWN_MODE` reader - Determines counter direction."]
pub type UP_DOWN_MODE_R = crate::FieldReader<UP_DOWN_MODE_A>;
#[doc = "Determines counter direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UP_DOWN_MODE_A {
    #[doc = "0: Count up (to PERIOD). An overflow event is generated when the counter changes from a state in which COUNTER equals PERIOD. A terminal count event is generated when the counter changes from a state in which COUNTER equals PERIOD."]
    COUNT_UP = 0,
    #[doc = "1: Count down (to '0'). An underflow event is generated when the counter changes from a state in which COUNTER equals '0'. A terminal count event is generated when the counter changes from a state in which COUNTER equals '0'."]
    COUNT_DOWN = 1,
    #[doc = "2: Count up (to PERIOD), then count down (to '0'). An overflow event is generated when the counter changes from a state in which COUNTER equals PERIOD. An underflow event is generated when the counter changes from a state in which COUNTER equals '0'. A terminal count event is generated when the counter changes from a state in which COUNTER equals '0'."]
    COUNT_UPDN1 = 2,
    #[doc = "3: Count up (to PERIOD), then count down (to '0'). An overflow event is generated when the counter changes from a state in which COUNTER equals PERIOD. An underflow event is generated when the counter changes from a state in which COUNTER equals '0'. A terminal count event is generated when the counter changes from a state in which COUNTER equals '0' AND when the counter changes from a state in which COUNTER equals PERIOD (this counter direction can be used for PWM functionality with asymmetrical updates)."]
    COUNT_UPDN2 = 3,
}
impl From<UP_DOWN_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: UP_DOWN_MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UP_DOWN_MODE_A {
    type Ux = u8;
}
impl UP_DOWN_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UP_DOWN_MODE_A {
        match self.bits {
            0 => UP_DOWN_MODE_A::COUNT_UP,
            1 => UP_DOWN_MODE_A::COUNT_DOWN,
            2 => UP_DOWN_MODE_A::COUNT_UPDN1,
            3 => UP_DOWN_MODE_A::COUNT_UPDN2,
            _ => unreachable!(),
        }
    }
    #[doc = "Count up (to PERIOD). An overflow event is generated when the counter changes from a state in which COUNTER equals PERIOD. A terminal count event is generated when the counter changes from a state in which COUNTER equals PERIOD."]
    #[inline(always)]
    pub fn is_count_up(&self) -> bool {
        *self == UP_DOWN_MODE_A::COUNT_UP
    }
    #[doc = "Count down (to '0'). An underflow event is generated when the counter changes from a state in which COUNTER equals '0'. A terminal count event is generated when the counter changes from a state in which COUNTER equals '0'."]
    #[inline(always)]
    pub fn is_count_down(&self) -> bool {
        *self == UP_DOWN_MODE_A::COUNT_DOWN
    }
    #[doc = "Count up (to PERIOD), then count down (to '0'). An overflow event is generated when the counter changes from a state in which COUNTER equals PERIOD. An underflow event is generated when the counter changes from a state in which COUNTER equals '0'. A terminal count event is generated when the counter changes from a state in which COUNTER equals '0'."]
    #[inline(always)]
    pub fn is_count_updn1(&self) -> bool {
        *self == UP_DOWN_MODE_A::COUNT_UPDN1
    }
    #[doc = "Count up (to PERIOD), then count down (to '0'). An overflow event is generated when the counter changes from a state in which COUNTER equals PERIOD. An underflow event is generated when the counter changes from a state in which COUNTER equals '0'. A terminal count event is generated when the counter changes from a state in which COUNTER equals '0' AND when the counter changes from a state in which COUNTER equals PERIOD (this counter direction can be used for PWM functionality with asymmetrical updates)."]
    #[inline(always)]
    pub fn is_count_updn2(&self) -> bool {
        *self == UP_DOWN_MODE_A::COUNT_UPDN2
    }
}
#[doc = "Field `UP_DOWN_MODE` writer - Determines counter direction."]
pub type UP_DOWN_MODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, UP_DOWN_MODE_A>;
impl<'a, REG> UP_DOWN_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Count up (to PERIOD). An overflow event is generated when the counter changes from a state in which COUNTER equals PERIOD. A terminal count event is generated when the counter changes from a state in which COUNTER equals PERIOD."]
    #[inline(always)]
    pub fn count_up(self) -> &'a mut crate::W<REG> {
        self.variant(UP_DOWN_MODE_A::COUNT_UP)
    }
    #[doc = "Count down (to '0'). An underflow event is generated when the counter changes from a state in which COUNTER equals '0'. A terminal count event is generated when the counter changes from a state in which COUNTER equals '0'."]
    #[inline(always)]
    pub fn count_down(self) -> &'a mut crate::W<REG> {
        self.variant(UP_DOWN_MODE_A::COUNT_DOWN)
    }
    #[doc = "Count up (to PERIOD), then count down (to '0'). An overflow event is generated when the counter changes from a state in which COUNTER equals PERIOD. An underflow event is generated when the counter changes from a state in which COUNTER equals '0'. A terminal count event is generated when the counter changes from a state in which COUNTER equals '0'."]
    #[inline(always)]
    pub fn count_updn1(self) -> &'a mut crate::W<REG> {
        self.variant(UP_DOWN_MODE_A::COUNT_UPDN1)
    }
    #[doc = "Count up (to PERIOD), then count down (to '0'). An overflow event is generated when the counter changes from a state in which COUNTER equals PERIOD. An underflow event is generated when the counter changes from a state in which COUNTER equals '0'. A terminal count event is generated when the counter changes from a state in which COUNTER equals '0' AND when the counter changes from a state in which COUNTER equals PERIOD (this counter direction can be used for PWM functionality with asymmetrical updates)."]
    #[inline(always)]
    pub fn count_updn2(self) -> &'a mut crate::W<REG> {
        self.variant(UP_DOWN_MODE_A::COUNT_UPDN2)
    }
}
#[doc = "Field `ONE_SHOT` reader - When '0', counter runs continuous. When '1', counter is turned off by hardware when a terminal count event is generated."]
pub type ONE_SHOT_R = crate::BitReader;
#[doc = "Field `ONE_SHOT` writer - When '0', counter runs continuous. When '1', counter is turned off by hardware when a terminal count event is generated."]
pub type ONE_SHOT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QUADRATURE_MODE` reader - In QUAD mode selects quadrature encoding mode (X1/X2/X4). In PWM, PWM_DT and PWM_PR modes, these two bits can be used to invert 'dt_line_out' and 'dt_line_compl_out'. Inversion is the last step in generation of 'dt_line_out' and 'dt_line_compl_out'; i.e. a disabled output line 'dt_line_out' has the value QUADRATURE_MODE\\[0\\]
and a disabled output line 'dt_line_compl_out' has the value QUADRATURE_MODE\\[1\\]."]
pub type QUADRATURE_MODE_R = crate::FieldReader<QUADRATURE_MODE_A>;
#[doc = "In QUAD mode selects quadrature encoding mode (X1/X2/X4). In PWM, PWM_DT and PWM_PR modes, these two bits can be used to invert 'dt_line_out' and 'dt_line_compl_out'. Inversion is the last step in generation of 'dt_line_out' and 'dt_line_compl_out'; i.e. a disabled output line 'dt_line_out' has the value QUADRATURE_MODE\\[0\\]
and a disabled output line 'dt_line_compl_out' has the value QUADRATURE_MODE\\[1\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum QUADRATURE_MODE_A {
    #[doc = "0: X1 encoding (QUAD mode)"]
    X1 = 0,
    #[doc = "1: X2 encoding (QUAD mode)"]
    X2 = 1,
    #[doc = "2: X4 encoding (QUAD mode)"]
    X4 = 2,
}
impl From<QUADRATURE_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: QUADRATURE_MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for QUADRATURE_MODE_A {
    type Ux = u8;
}
impl QUADRATURE_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<QUADRATURE_MODE_A> {
        match self.bits {
            0 => Some(QUADRATURE_MODE_A::X1),
            1 => Some(QUADRATURE_MODE_A::X2),
            2 => Some(QUADRATURE_MODE_A::X4),
            _ => None,
        }
    }
    #[doc = "X1 encoding (QUAD mode)"]
    #[inline(always)]
    pub fn is_x1(&self) -> bool {
        *self == QUADRATURE_MODE_A::X1
    }
    #[doc = "X2 encoding (QUAD mode)"]
    #[inline(always)]
    pub fn is_x2(&self) -> bool {
        *self == QUADRATURE_MODE_A::X2
    }
    #[doc = "X4 encoding (QUAD mode)"]
    #[inline(always)]
    pub fn is_x4(&self) -> bool {
        *self == QUADRATURE_MODE_A::X4
    }
}
#[doc = "Field `QUADRATURE_MODE` writer - In QUAD mode selects quadrature encoding mode (X1/X2/X4). In PWM, PWM_DT and PWM_PR modes, these two bits can be used to invert 'dt_line_out' and 'dt_line_compl_out'. Inversion is the last step in generation of 'dt_line_out' and 'dt_line_compl_out'; i.e. a disabled output line 'dt_line_out' has the value QUADRATURE_MODE\\[0\\]
and a disabled output line 'dt_line_compl_out' has the value QUADRATURE_MODE\\[1\\]."]
pub type QUADRATURE_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, QUADRATURE_MODE_A>;
impl<'a, REG> QUADRATURE_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "X1 encoding (QUAD mode)"]
    #[inline(always)]
    pub fn x1(self) -> &'a mut crate::W<REG> {
        self.variant(QUADRATURE_MODE_A::X1)
    }
    #[doc = "X2 encoding (QUAD mode)"]
    #[inline(always)]
    pub fn x2(self) -> &'a mut crate::W<REG> {
        self.variant(QUADRATURE_MODE_A::X2)
    }
    #[doc = "X4 encoding (QUAD mode)"]
    #[inline(always)]
    pub fn x4(self) -> &'a mut crate::W<REG> {
        self.variant(QUADRATURE_MODE_A::X4)
    }
}
#[doc = "Field `MODE` reader - Counter mode."]
pub type MODE_R = crate::FieldReader<MODE_A>;
#[doc = "Counter mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Timer mode"]
    TIMER = 0,
    #[doc = "2: Capture mode"]
    CAPTURE = 2,
    #[doc = "3: Quadrature encoding mode"]
    QUAD = 3,
    #[doc = "4: Pulse width modulation (PWM) mode"]
    PWM = 4,
    #[doc = "5: PWM with deadtime insertion mode"]
    PWM_DT = 5,
    #[doc = "6: Pseudo random pulse width modulation"]
    PWM_PR = 6,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE_A {
    type Ux = u8;
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::TIMER),
            2 => Some(MODE_A::CAPTURE),
            3 => Some(MODE_A::QUAD),
            4 => Some(MODE_A::PWM),
            5 => Some(MODE_A::PWM_DT),
            6 => Some(MODE_A::PWM_PR),
            _ => None,
        }
    }
    #[doc = "Timer mode"]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == MODE_A::TIMER
    }
    #[doc = "Capture mode"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == MODE_A::CAPTURE
    }
    #[doc = "Quadrature encoding mode"]
    #[inline(always)]
    pub fn is_quad(&self) -> bool {
        *self == MODE_A::QUAD
    }
    #[doc = "Pulse width modulation (PWM) mode"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == MODE_A::PWM
    }
    #[doc = "PWM with deadtime insertion mode"]
    #[inline(always)]
    pub fn is_pwm_dt(&self) -> bool {
        *self == MODE_A::PWM_DT
    }
    #[doc = "Pseudo random pulse width modulation"]
    #[inline(always)]
    pub fn is_pwm_pr(&self) -> bool {
        *self == MODE_A::PWM_PR
    }
}
#[doc = "Field `MODE` writer - Counter mode."]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MODE_A>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer mode"]
    #[inline(always)]
    pub fn timer(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::TIMER)
    }
    #[doc = "Capture mode"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::CAPTURE)
    }
    #[doc = "Quadrature encoding mode"]
    #[inline(always)]
    pub fn quad(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::QUAD)
    }
    #[doc = "Pulse width modulation (PWM) mode"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::PWM)
    }
    #[doc = "PWM with deadtime insertion mode"]
    #[inline(always)]
    pub fn pwm_dt(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::PWM_DT)
    }
    #[doc = "Pseudo random pulse width modulation"]
    #[inline(always)]
    pub fn pwm_pr(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::PWM_PR)
    }
}
impl R {
    #[doc = "Bit 0 - Specifies switching of the CC and buffered CC values. This field has a function in TIMER, PWM, PWM_DT and PWM_PR modes. Timer mode: '0': never switch. '1': switch on a compare match event. PWM, PWM_DT, PWM_PR modes: '0: never switch. '1': switch on a terminal count event with an actively pending switch event."]
    #[inline(always)]
    pub fn auto_reload_cc(&self) -> AUTO_RELOAD_CC_R {
        AUTO_RELOAD_CC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Specifies switching of the PERIOD and buffered PERIOD values. This field has a function in PWM, PWM_DT and PWM_PR modes. '0': never switch. '1': switch on a terminal count event with and actively pending switch event."]
    #[inline(always)]
    pub fn auto_reload_period(&self) -> AUTO_RELOAD_PERIOD_R {
        AUTO_RELOAD_PERIOD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Specifies asynchronous/synchronous kill behavior: '1': synchronous kill mode: the kill event disables the 'dt_line_out' and 'dt_line_compl_out' signals till the next terminal count event (synchronous kill). In synchronous kill mode, STOP_EDGE should be RISING_EDGE. '0': asynchronous kill mode: the kill event only disables the 'dt_line_out' and 'dt_line_compl_out' signals when present. In asynchronous kill mode, STOP_EDGE should be NO_EDGE_DET. This field has a function in PWM and PWM_DT modes only. This field is only used when PWM_STOP_ON_KILL is '0'."]
    #[inline(always)]
    pub fn pwm_sync_kill(&self) -> PWM_SYNC_KILL_R {
        PWM_SYNC_KILL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Specifies whether the counter stops on a kill events: '0': kill event does NOT stop counter. '1': kill event stops counter. This field has a function in PWM, PWM_DT and PWM_PR modes only."]
    #[inline(always)]
    pub fn pwm_stop_on_kill(&self) -> PWM_STOP_ON_KILL_R {
        PWM_STOP_ON_KILL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Generic 8-bit control field. In PWM_DT mode, this field is used to determine the dead time: amount of dead time cycles in the counter clock domain. In all other modes, the lower 3 bits of this field determine pre-scaling of the selected counter clock."]
    #[inline(always)]
    pub fn generic(&self) -> GENERIC_R {
        GENERIC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Determines counter direction."]
    #[inline(always)]
    pub fn up_down_mode(&self) -> UP_DOWN_MODE_R {
        UP_DOWN_MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - When '0', counter runs continuous. When '1', counter is turned off by hardware when a terminal count event is generated."]
    #[inline(always)]
    pub fn one_shot(&self) -> ONE_SHOT_R {
        ONE_SHOT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:21 - In QUAD mode selects quadrature encoding mode (X1/X2/X4). In PWM, PWM_DT and PWM_PR modes, these two bits can be used to invert 'dt_line_out' and 'dt_line_compl_out'. Inversion is the last step in generation of 'dt_line_out' and 'dt_line_compl_out'; i.e. a disabled output line 'dt_line_out' has the value QUADRATURE_MODE\\[0\\]
and a disabled output line 'dt_line_compl_out' has the value QUADRATURE_MODE\\[1\\]."]
    #[inline(always)]
    pub fn quadrature_mode(&self) -> QUADRATURE_MODE_R {
        QUADRATURE_MODE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Counter mode."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Specifies switching of the CC and buffered CC values. This field has a function in TIMER, PWM, PWM_DT and PWM_PR modes. Timer mode: '0': never switch. '1': switch on a compare match event. PWM, PWM_DT, PWM_PR modes: '0: never switch. '1': switch on a terminal count event with an actively pending switch event."]
    #[inline(always)]
    #[must_use]
    pub fn auto_reload_cc(&mut self) -> AUTO_RELOAD_CC_W<CTRL_SPEC> {
        AUTO_RELOAD_CC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Specifies switching of the PERIOD and buffered PERIOD values. This field has a function in PWM, PWM_DT and PWM_PR modes. '0': never switch. '1': switch on a terminal count event with and actively pending switch event."]
    #[inline(always)]
    #[must_use]
    pub fn auto_reload_period(&mut self) -> AUTO_RELOAD_PERIOD_W<CTRL_SPEC> {
        AUTO_RELOAD_PERIOD_W::new(self, 1)
    }
    #[doc = "Bit 2 - Specifies asynchronous/synchronous kill behavior: '1': synchronous kill mode: the kill event disables the 'dt_line_out' and 'dt_line_compl_out' signals till the next terminal count event (synchronous kill). In synchronous kill mode, STOP_EDGE should be RISING_EDGE. '0': asynchronous kill mode: the kill event only disables the 'dt_line_out' and 'dt_line_compl_out' signals when present. In asynchronous kill mode, STOP_EDGE should be NO_EDGE_DET. This field has a function in PWM and PWM_DT modes only. This field is only used when PWM_STOP_ON_KILL is '0'."]
    #[inline(always)]
    #[must_use]
    pub fn pwm_sync_kill(&mut self) -> PWM_SYNC_KILL_W<CTRL_SPEC> {
        PWM_SYNC_KILL_W::new(self, 2)
    }
    #[doc = "Bit 3 - Specifies whether the counter stops on a kill events: '0': kill event does NOT stop counter. '1': kill event stops counter. This field has a function in PWM, PWM_DT and PWM_PR modes only."]
    #[inline(always)]
    #[must_use]
    pub fn pwm_stop_on_kill(&mut self) -> PWM_STOP_ON_KILL_W<CTRL_SPEC> {
        PWM_STOP_ON_KILL_W::new(self, 3)
    }
    #[doc = "Bits 8:15 - Generic 8-bit control field. In PWM_DT mode, this field is used to determine the dead time: amount of dead time cycles in the counter clock domain. In all other modes, the lower 3 bits of this field determine pre-scaling of the selected counter clock."]
    #[inline(always)]
    #[must_use]
    pub fn generic(&mut self) -> GENERIC_W<CTRL_SPEC> {
        GENERIC_W::new(self, 8)
    }
    #[doc = "Bits 16:17 - Determines counter direction."]
    #[inline(always)]
    #[must_use]
    pub fn up_down_mode(&mut self) -> UP_DOWN_MODE_W<CTRL_SPEC> {
        UP_DOWN_MODE_W::new(self, 16)
    }
    #[doc = "Bit 18 - When '0', counter runs continuous. When '1', counter is turned off by hardware when a terminal count event is generated."]
    #[inline(always)]
    #[must_use]
    pub fn one_shot(&mut self) -> ONE_SHOT_W<CTRL_SPEC> {
        ONE_SHOT_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - In QUAD mode selects quadrature encoding mode (X1/X2/X4). In PWM, PWM_DT and PWM_PR modes, these two bits can be used to invert 'dt_line_out' and 'dt_line_compl_out'. Inversion is the last step in generation of 'dt_line_out' and 'dt_line_compl_out'; i.e. a disabled output line 'dt_line_out' has the value QUADRATURE_MODE\\[0\\]
and a disabled output line 'dt_line_compl_out' has the value QUADRATURE_MODE\\[1\\]."]
    #[inline(always)]
    #[must_use]
    pub fn quadrature_mode(&mut self) -> QUADRATURE_MODE_W<CTRL_SPEC> {
        QUADRATURE_MODE_W::new(self, 20)
    }
    #[doc = "Bits 24:26 - Counter mode."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CTRL_SPEC> {
        MODE_W::new(self, 24)
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
#[doc = "Counter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
