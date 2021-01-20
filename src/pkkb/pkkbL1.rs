
mod super::fifo;


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
            Self::SOF => Self::SOF_VALUE,
            Self::EOF => Self::EOF_VALUE,
			Self::ESC => Self::ESC_VALUE,
        }
    }
	pub fn getEnum(data :u8) -> Option<CtrlChar> {
		match data {
			Self::SOF_VALUE => Some(Self::SOF),
			Self::EOF_VALUE => Some(Self::EOF),
			Self::ESC_VALUE => Some(Self::ESC),
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
            Self::ESC_SOF => Self::ESC_SOF_VALUE,
            Self::ESC_EOF => Self::ESC_EOF_VALUE,
			Self::ESC_ESC => Self::ESC_ESC_VALUE,
        }
    }
	pub fn getEnum(data :u8) -> Option<EncodedChar> {
		match data{
			Self::ESC_SOF_VALUE => Some(Self::ESC_SOF),
			Self::ESC_EOF_VALUE => Some(Self::ESC_EOF),
			Self::ESC_ESC_VALUE => Some(Self::ESC_ESC),
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

pub fn is_ctrl_code(data :u8) -> bool {
	match data{
		
	}
		return ((data == PKKB_SOF) || (data == PKKB_EOF) || (data == PKKB_ESC));
	}

	


fn encodeFrame(frameIn :&mut Fifo, frameOut :&mut Fifo) -> Result<(),bool>{
		uint8_t znak;
		if (!frameOut->put(PKKB_CtrlChar::PKKB_SOF))
			return false;
		while (frameIn->isNotEmpty()) {
			znak = (uint8_t)frameIn->get();
			if (is_ctrl_code(znak)) {
				if (!frameOut->put(PKKB_CtrlChar::PKKB_ESC))
					return false;
				znak = esc_encode(znak);
			}
			if (!frameOut->put(znak))
				return false;
		}
		if (!frameOut->put(PKKB_CtrlChar::PKKB_EOF))
			return false;
		return true;
	} 



