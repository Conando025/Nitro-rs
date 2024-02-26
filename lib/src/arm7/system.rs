#[repr(packed)]
pub struct PersonalData {
    ///	??? (0x05 0x00). (version according to gbatek)
    reserved_0: [u8; 2],

    ///The user's theme color (0-15).
    theme: u8,
    ///The user's birth month (1-12).
    birth_month: u8,    
    ///The user's birth day (1-31).
    birth_day: u8,      

    ///	???
    reserved_1: u8,

    ///The user's name in UTF-16 format.
    name: [i16; 10],
    ///The length of the user's name in characters.   
    name_length: u16,

    ///The user's message.
    message:[i16; 26],
    ///The length of the user's message in characters.
    message_length: u16,

    ///What hour the alarm clock is set to (0-23).
    alarm_hour: u8,
    ///What minute the alarm clock is set to (0-59).
    alarm_minute: u8,

    ///??? 0x02FFFCD4
    reserved_2: u64,	

    ///	Touchscreen calibration: first X touch
    calX1: u16 ,
    ///	Touchscreen calibration: first Y touch
	calY1: u16 ,
    ///	Touchscreen calibration: first X touch pixel
	calX1px: u8 ,
    ///	Touchscreen calibration: first X touch pixel
	calY1px: u8 ,

    ///	Touchscreen calibration: second X touch
    calX2: u16 ,
    ///	Touchscreen calibration: second Y touch
	calY2: u16 ,
    ///	Touchscreen calibration: second X touch pixel
	calX2px: u8 ,
    ///	Touchscreen calibration: second X touch pixel
	calY2px: u8 ,

    unkown: refector_this,

    ///???
    reserved_3: u16,
    ///Real Time Clock offset.
	rtcOffset: u32,		
	///???
    reserved_4: u32,
}

#[repr(packed)]
struct refector_this {
    language: usize,
    ///GBA screen selection (lower screen if set, otherwise upper screen).
    gba_screen: usize,
    ///Brightness level at power on, dslite.
    default_brightness: usize,
    ///The DS should boot from the DS cart or GBA cart automatically if one is inserted.
    auto_mode: usize,
    ///???
    reserved_5: usize,
    ///User Settings Lost (0=Normal, 1=Prompt/Settings Lost)
    settings_lost: usize,
    ///???
    reserved_3D6: usize,
}
