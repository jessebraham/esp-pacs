#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "11 - I2C"]
    I2C = 11,
    #[doc = "15 - UHCI0"]
    UHCI0 = 15,
    #[doc = "16 - GPIO"]
    GPIO = 16,
    #[doc = "17 - GPIO_NMI"]
    GPIO_NMI = 17,
    #[doc = "18 - SPI1"]
    SPI1 = 18,
    #[doc = "19 - SPI2"]
    SPI2 = 19,
    #[doc = "20 - I2S1"]
    I2S1 = 20,
    #[doc = "21 - UART0"]
    UART0 = 21,
    #[doc = "22 - UART1"]
    UART1 = 22,
    #[doc = "23 - LEDC"]
    LEDC = 23,
    #[doc = "24 - EFUSE"]
    EFUSE = 24,
    #[doc = "25 - TWAI"]
    TWAI = 25,
    #[doc = "26 - USB"]
    USB = 26,
    #[doc = "28 - RMT"]
    RMT = 28,
    #[doc = "29 - I2C_EXT0"]
    I2C_EXT0 = 29,
    #[doc = "32 - TG0_T0_LEVEL"]
    TG0_T0_LEVEL = 32,
    #[doc = "33 - TG0_WDT_LEVEL"]
    TG0_WDT_LEVEL = 33,
    #[doc = "34 - TG1_T0_LEVEL"]
    TG1_T0_LEVEL = 34,
    #[doc = "35 - TG1_WDT_LEVEL"]
    TG1_WDT_LEVEL = 35,
    #[doc = "37 - SYSTIMER_TARGET0_EDGE"]
    SYSTIMER_TARGET0_EDGE = 37,
    #[doc = "38 - SYSTIMER_TARGET1_EDGE"]
    SYSTIMER_TARGET1_EDGE = 38,
    #[doc = "39 - SYSTIMER_TARGET2_EDGE"]
    SYSTIMER_TARGET2_EDGE = 39,
    #[doc = "40 - SPI_MEM_REJECT_CACHE"]
    SPI_MEM_REJECT_CACHE = 40,
    #[doc = "43 - APB_ADC"]
    APB_ADC = 43,
    #[doc = "44 - DMA_CH0"]
    DMA_CH0 = 44,
    #[doc = "45 - DMA_CH1"]
    DMA_CH1 = 45,
    #[doc = "47 - RSA"]
    RSA = 47,
    #[doc = "48 - AES"]
    AES = 48,
    #[doc = "49 - SHA"]
    SHA = 49,
}
#[doc = r" TryFromInterruptError"]
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
    #[doc = r" Attempt to convert a given value into an `Interrupt`"]
    #[inline]
    pub fn try_from(value: u8) -> Result<Self, TryFromInterruptError> {
        match value {
            11 => Ok(Interrupt::I2C),
            15 => Ok(Interrupt::UHCI0),
            16 => Ok(Interrupt::GPIO),
            17 => Ok(Interrupt::GPIO_NMI),
            18 => Ok(Interrupt::SPI1),
            19 => Ok(Interrupt::SPI2),
            20 => Ok(Interrupt::I2S1),
            21 => Ok(Interrupt::UART0),
            22 => Ok(Interrupt::UART1),
            23 => Ok(Interrupt::LEDC),
            24 => Ok(Interrupt::EFUSE),
            25 => Ok(Interrupt::TWAI),
            26 => Ok(Interrupt::USB),
            28 => Ok(Interrupt::RMT),
            29 => Ok(Interrupt::I2C_EXT0),
            32 => Ok(Interrupt::TG0_T0_LEVEL),
            33 => Ok(Interrupt::TG0_WDT_LEVEL),
            34 => Ok(Interrupt::TG1_T0_LEVEL),
            35 => Ok(Interrupt::TG1_WDT_LEVEL),
            37 => Ok(Interrupt::SYSTIMER_TARGET0_EDGE),
            38 => Ok(Interrupt::SYSTIMER_TARGET1_EDGE),
            39 => Ok(Interrupt::SYSTIMER_TARGET2_EDGE),
            40 => Ok(Interrupt::SPI_MEM_REJECT_CACHE),
            43 => Ok(Interrupt::APB_ADC),
            44 => Ok(Interrupt::DMA_CH0),
            45 => Ok(Interrupt::DMA_CH1),
            47 => Ok(Interrupt::RSA),
            48 => Ok(Interrupt::AES),
            49 => Ok(Interrupt::SHA),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
#[cfg(feature = "rt")]
#[macro_export]
#[doc = r" Assigns a handler to an interrupt"]
#[doc = r""]
#[doc = r" This macro takes two arguments: the name of an interrupt and the path to the"]
#[doc = r" function that will be used as the handler of that interrupt. That function"]
#[doc = r" must have signature `fn()`."]
#[doc = r""]
#[doc = r" Optionally, a third argument may be used to declare interrupt local data."]
#[doc = r" The handler will have exclusive access to these *local* variables on each"]
#[doc = r" invocation. If the third argument is used then the signature of the handler"]
#[doc = r" function must be `fn(&mut $NAME::Locals)` where `$NAME` is the first argument"]
#[doc = r" passed to the macro."]
#[doc = r""]
#[doc = r" # Example"]
#[doc = r""]
#[doc = r" ``` ignore"]
#[doc = r" interrupt!(TIM2, periodic);"]
#[doc = r""]
#[doc = r" fn periodic() {"]
#[doc = r#"     print!(".");"#]
#[doc = r" }"]
#[doc = r""]
#[doc = r" interrupt!(TIM3, tick, locals: {"]
#[doc = r"     tick: bool = false;"]
#[doc = r" });"]
#[doc = r""]
#[doc = r" fn tick(locals: &mut TIM3::Locals) {"]
#[doc = r"     locals.tick = !locals.tick;"]
#[doc = r""]
#[doc = r"     if locals.tick {"]
#[doc = r#"         println!("Tick");"#]
#[doc = r"     } else {"]
#[doc = r#"         println!("Tock");"#]
#[doc = r"     }"]
#[doc = r" }"]
#[doc = r" ```"]
macro_rules ! interrupt { ($ NAME : ident , $ path : path , locals : { $ ($ lvar : ident : $ lty : ty = $ lval : expr ;) * }) => { # [allow (non_snake_case)] mod $ NAME { pub struct Locals { $ (pub $ lvar : $ lty ,) * } } # [allow (non_snake_case)] # [no_mangle] pub extern "C" fn $ NAME () { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; static mut LOCALS : self :: $ NAME :: Locals = self :: $ NAME :: Locals { $ ($ lvar : $ lval ,) * } ; let f : fn (& mut self :: $ NAME :: Locals) = $ path ; f (unsafe { & mut LOCALS }) ; } } ; ($ NAME : ident , $ path : path) => { # [allow (non_snake_case)] # [no_mangle] pub extern "C" fn $ NAME () { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; let f : fn () = $ path ; f () ; } } }