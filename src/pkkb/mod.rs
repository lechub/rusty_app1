
mod pkkbL1;

pub const PKKB_MASTER_ADDRESS: u8 = 0x00;
pub const PKKB_BROADCAST_ADDRESS: u8 = 0xFF;
pub const PKKB_UNUSED_ADDRESS: u8 = 0xFE;


//pub enum Consts{
//	PkkbMasterAdress = 0x00,
//	PkkbBroadcastAdress = 0xFF,
//	PkkbUnusedAdress = 0xFE,
//}


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


