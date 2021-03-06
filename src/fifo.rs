
const FIFO_CAPACITY :usize = 32;

pub struct Fifo {
    size: usize,
    read_idx: usize,
    buffer: [u8; FIFO_CAPACITY]

}

pub enum FifoError {
    	FifoFull,
		FifoEmpty,
}

impl Default for Fifo {
    fn default() -> Fifo {
        Fifo {
            size: 0,
            read_idx: 0,
            buffer: [0; FIFO_CAPACITY],
        }
    }
}


impl Fifo {
    pub fn new() -> Fifo {
       Self::default()
    }

//    pub fn put(&mut self, item: u8) -> Result<(), FifoError> {
//        if self.buffer_full() {
//           Err(FifoError::FifoFull)      
//        } else {
//            self.buffer[self.write_idx] = item;
//
//            self.write_idx = Fifo::increment_index(self.write_idx);
//            self.size += 1;
//
//            Ok(())
//        }
//    }

	fn wrap_around(idx: usize) -> usize {
    	idx % FIFO_CAPACITY
	}
    
	pub fn put(&mut self, item: u8) -> Result<(), FifoError> {
        if self.is_buffer_full() {
           Err(FifoError::FifoFull)      
        } else {
			let write_idx = Self::wrap_around(self.read_idx + self.size);
	        self.buffer[write_idx] = item;
    	    self.size += self.size;
        	Ok(())
        }
    }

//bool Fifo::put(uint8_t bajt){
//	uint32_t tmpIn = in;
//	uint32_t tmpOut = out;
//	if ((tmpIn + 1) == tmpOut) return false;	//bufor pelny, bo ! Trzeba co najmniej 1 wolny bajt w buforze
//	if ((tmpOut == 0)&&(tmpIn == (bufSize-1))) return false; // bufor pelny, indeksy na koncu i poczatku tablicy
//	bufor[tmpIn++] = bajt;		// zapis pod indeks in (pierwsze wolne) i inkrementacja indeksu
//	in = (tmpIn >= (bufSize))? 0 : tmpIn;	//korekcja indeksu w strukturze
//	return true;
//}

    pub fn get(&mut self) -> Option<u8> {
        if self.size == 0 {
            None
        } else {
            let result = self.buffer[self.read_idx];
            self.read_idx = Fifo::wrap_around(self.read_idx + 1);
            self.size -= 1;
            Some(result)        
        }
    }

    pub fn is_buffer_full(&self) -> bool {
        self.size >= (FIFO_CAPACITY + 1)
    }

   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pop_item_that_was_pushed_to_buffer() {
        let mut buffer = Fifo::new();
        assert!(buffer.put(5).is_ok());
        let pop_result = buffer.get();
        assert_eq!(Some(5), pop_result);
    }

    #[test]
    fn popping_empty_buffer_returns_none() {
        let mut buffer = Fifo::new();
        assert_eq!(None, buffer.get());
    }

    #[test]
    fn popping_returns_first_pushed_first() {
        let mut buffer = Fifo::new();
        assert!(buffer.put(1).is_ok());
        assert!(buffer.put(2).is_ok());

        assert_eq!(Some(1), buffer.get());
        assert_eq!(Some(2), buffer.get());
    }

    #[test]
    fn pop_beyond_write_index_returns_none() {
        let mut buffer = Fifo::new();
        assert!(buffer.put(1).is_ok());
        assert_eq!(Some(1), buffer.get());
        assert_eq!(None, buffer.get());
    }

    #[test]
    fn pop_beyond_write_index_continuing_on_works() {
        let mut buffer = Fifo::new();
        assert!(buffer.put(1).is_ok());
        assert_eq!(Some(1), buffer.get());
        assert_eq!(None, buffer.get());

        assert!(buffer.put(2).is_ok());
        assert_eq!(Some(2), buffer.get());
    }

    #[test]
    fn buffer_wraps_around() {
        let mut buffer = Fifo::new();
        let capacity = FIFO_CAPACITY as u8;
        for x in 0..=(capacity * 3) {
            assert!(buffer.put(x).is_ok());
            assert_eq!(Some(x), buffer.get());
        }
    }

    #[test]
    fn push_fails_if_buffer_is_full() {
        let mut buffer = Fifo::new();
        let capacity = FIFO_CAPACITY as u8;

        for x in 0..(capacity) {
            assert!(buffer.put(x).is_ok());
        }

        assert!(buffer.put(capacity + 1).is_err())
    }
}

//
// /**
// * @brief	Klasa realizujaca bufor typu fifo. Moze sluzyc tez do przekazywania pakietow danych i do konwersji typow.
// */
//
//class Fifo{
//	static const uint16_t	FIFO_DEFAULT_SIZE = 300;	// Domyslny rozmiar buforu. Efektywnie wykorzystuje sie 1 bajt mniej
//protected:
//	uint32_t 	bufSize;	///< Aktualny rozmiar buforu
//	volatile	uint32_t in;		///< Pierwszy wolny bajt do zapisu - indeks w buforze (0..bufSize-1)
//	volatile	uint32_t out;		///< Ostatni zapisany bajt i pierwszy do odczytu - indeks w buforze
//	volatile	uint8_t *bufor;	///< Przechowalnia bajtow, czyli bufor bajtow
//
//public:
//	static constexpr uint32_t BUFFER_EMPTY_FLAG_U32 = 0xffffff00;	///< flaga pustego bufora dla wartosci 16-bitowej
//	static constexpr uint16_t BUFFER_EMPTY_FLAG_U16 = 0xff00;		///< flaga pustego bufora dla wartosci 8-bitowej
//	Fifo(uint8_t *buforb, uint32_t size);				///< konstruktor musi otrzymac dane bufora
//
//	/** ***************************************************************
//	 * @brief  Dodaje 1 bajt do bufora fifo
//	 * @param  bajt	Bajt do umieszczenia w buforze
//	 * @return true jesli operacja sie powiodla, albo false, jesli bufor jest pelny
//	 */
//	bool put(uint8_t bajt);
//
//	/** ***************************************************************
//	 * @brief  Pobiera 1 bajt z bufora.
//	 * @return Zwraca znak (bajt) lub BUFFER_EMPTY_FLAG_U16, gdy pusty bufor
//	 */
//	uint16_t get();
//
//	/** ***************************************************************
//	 * @brief  Pobiera z bufora 2 znaki i interpretuje je jako liczby Hex w zapisie Ascii
//	 * @return Zwraca znak (bajt) lub BUFFER_EMPTY_FLAG_U16, gdy pusty bufor lub pobrane znaki nie sa liczbami Hex
//	 */
//	uint16_t getByteAs2HexAscii();
//
//	uint32_t get2BytesAs4HexAscii();
//
//	/** ***************************************************************
//	 * @brief  Pobiera 1 slowo 16-bitowe z bufora.
//	 * Czyli dwa kolejne bajty - pierwszy jako starszy bajt i drugi jako mlodszy
//	 * @return Zwraca slowo (16-bitowe) lub BUFFER_EMPTY_FLAG_U32, gdy pusty bufor
//	 */
//	uint32_t getUInt16();
//
//	/** ***************************************************************
//	 * @brief  Pobiera 1 slowo 32-bitowe z bufora.
//	 * Czyli 4 kolejne bajty - pierwszy jako najstarszy bajt
//	 * @param  result	Wskaznik do rezultatu operacji
//	 * @return Zwraca slowo (32-bitowe). Poprawna operacja zwraca w *result true, niepoprawna false
//	 */
//	uint32_t getUInt32(bool *result);
//
//	bool putAll(Fifo * src);
//
//	bool putByteAs2AsciiBCD(uint8_t inValue);
//	bool put2BytesAs4AsciiBCD(uint16_t inValue);
//	bool putString(const char * string);
//	bool putString(const char * string, uint32_t limit);
//	//inline bool putString(const char * string){return putString((char*)string);}
//
//	bool putUint16(uint16_t value);
//	bool putUint32(uint32_t value);
//	// -----------------------------------------------------------------
//	/**	Podaje ilosc bajtow czekajacych w buforze.
//	 */
//	uint32_t get_count();
//
//	// -----------------------------------------------------------------
//	/**	Podaje ilosc bajtow mozliwych do zapisania do bufora.
//	 */
//	uint32_t countFree(){return bufSize - 1 - get_count();}
//
//	// -----------------------------------------------------------------
//	/**	Zwraca !=0 jesli bufor jest pusty
//	 */
//	bool isEmpty(){ return (in == out); }
//
//	bool isNotEmpty(){return !isEmpty();}
//
//	bool isFull();
//	// -----------------------------------------------------------------
//	/**	Zeruje bufor - ustawia pusty.
//	 */
//	void flush(){out = in = 0;}
//
//	bool clone(Fifo * destination);
//
//	// zwraca bajt po konwersji z BCD na bin
//	// gdy blad to liczba wieksza od 0x0f
//	uint8_t get1BcdAsHalfByte(){ return AsciiBcdToValue((uint8_t)get()); }
//
//	// z 2 kolejunych znakow BCD sklada liczbe 1-bajtowa
//	// gdy blad to liczba wieksza od 0x0ff
//	uint16_t get2BcdAs1Byte(){
//		if (get_count() < 2) return 0xefff;
//		uint8_t znakHi = get1BcdAsHalfByte();
//		uint8_t znakLo = get1BcdAsHalfByte();
//		if ((znakHi > 0x0f)||(znakLo > 0x0f)) return 0xffff;
//		return (uint16_t)((znakHi << 4) | znakLo);
//	}
//
//	// z 4 kolejunych znakow BCD sklada liczbe 2-bajtowa
//	// gdy blad to liczba wieksza od 0x0ffff
//	uint32_t get4BcdAs2Bytes(){
//		if (get_count() < 2) return 0xefffffff;
//		uint16_t znakHi = get2BcdAs1Byte();
//		uint16_t znakLo = get2BcdAs1Byte();
//		if ((znakHi > 0xff)||(znakLo > 0xff)) return 0xffffffff;
//		return ((znakHi << 8) | znakLo);
//	}
//
//};
//
////#define FifoFixed(size)
//
//#endif /* FIFO_H_ */