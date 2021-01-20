

//impl MyStruct {
//    const MY_STATIC: i32 = 123;
//}

//struct CTRL_CHAR{
//	const SOF
//}

pub enum CtrlChar{
		SOF, /* Znacznik poczatku transmisji, tzn. poczatku ramki*/
		EOF, /* Znacznik konca transmisji */
		ESC, /* Znacznik podmiany dla nastepnego znaku */
	}
	
impl CtrlChar {
	const SOF_VALUE: u8 = 0xAA;
	const EOF_VALUE: u8 = 0xAB;
	const ESC_VALUE: u8 = 0xAC;
    pub fn value(&self) -> u8 {
        match *self {
            CtrlChar::SOF => CtrlChar::SOF_VALUE,
            CtrlChar::EOF => CtrlChar::EOF_VALUE,
			CtrlChar::ESC => CtrlChar::ESC_VALUE,
        }
    }
	pub fn getEnum(data :u8) -> Option<CtrlChar> {
		match data {
			CtrlChar::SOF_VALUE => Some(CtrlChar::SOF),
			CtrlChar::EOF_VALUE => Some(CtrlChar::EOF),
			CtrlChar::ESC_VALUE => Some(CtrlChar::ESC),
			_ => None
		}
	}
}

pub enum EncodedChar{
		ESC_SOF, /* kod SOF dla zakodowanego bajtu */
		ESC_EOF, /* kod EOF dla zakodowanego bajtu */
		ESC_ESC, /* kod ESC dla zakodowanego bajtu */
}

impl EncodedChar {
	const ESC_SOF_VALUE: u8 = 0x01;
	const ESC_EOF_VALUE: u8 = 0x02;
	const ESC_ESC_VALUE: u8 = 0x03;
    pub fn value(&self) -> u8 {
        match *self {
            EncodedChar::ESC_SOF => EncodedChar::ESC_SOF_VALUE,
            EncodedChar::ESC_EOF => EncodedChar::ESC_EOF_VALUE,
			EncodedChar::ESC_ESC => EncodedChar::ESC_ESC_VALUE,
        }
    }
	pub fn getEnum(data :u8) -> Option<EncodedChar> {
		match data{
			EncodedChar::ESC_SOF_VALUE => Some(EncodedChar::ESC_SOF),
			EncodedChar::ESC_EOF_VALUE => Some(EncodedChar::ESC_EOF),
			EncodedChar::ESC_ESC_VALUE => Some(EncodedChar::ESC_ESC),
			_ => None	
		}
	}
}


pub enum PkkbCommand{
		READ = 0x01,
		WRITE = 0x02,		//	- zapis 1 bajtu
		READBLOCK = 0x03,			//	- odczyt bloku danych
		WRITEBLOCK = 0x04,			//- zapis bloku danych
		ANSWER = 0x05,		
		READSTRING = 0x06,		// - odczyt tekstu: READSTRING, INDEX
		WRITESTRING = 0x07,		// - zapis tekstu: WRITESTRING, INDEX
		ASSIGN = 0x10,		//wejscie w procedure nadawania adresu
}
	
pub enum PkkbAssignCommand{
		CLEAR = 0x22,
		START = 0x24,		//	-
		STOP = 0x25,			//	- 
		SET = 0x26,			//-
		GET = 0x27,		
}	



/// Procedurka zamienia zakodowane po ESC znaki na prawidlowe.
/// @param data Bajt do odkodowania
/// @return Bajt po odkodowaniu
pub fn esc_decode(data: u8) -> u8 {
	match EncodedChar::getEnum(data){
		Some(x) => x.value(),
		None => data
	}
}


	


 



