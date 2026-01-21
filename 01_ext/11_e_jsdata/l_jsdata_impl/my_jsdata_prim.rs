#[allow(unused_imports)]
use super::*;

impl T_JsData_ for bool {
    #[inline(always)]
    fn write_to_js(&self, writer: T_JsData_Write, _transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err> {
        let x = if *self { 1_u8 } else { 0_u8 };
        Ok(writer.write_u8(x).map_err(|_| L_JsData_Err::BufWriter)?)
    }

    #[inline(always)]
    fn read_from_js(reader: T_JsData_Read, _transfers: &mut VecDeque<wb::JsValue>) -> Result<bool, L_JsData_Err> {
        let x: u8 = <u8 as T_JsData_>::read_from_js(reader, _transfers)?;
        Ok(x == 1)
    }
}

impl T_JsData_ for i8 {
    #[inline(always)]
    fn write_to_js(&self, writer: T_JsData_Write, _transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err> {
        Ok(writer.write_i8(*self).map_err(|_| L_JsData_Err::BufWriter)?)
    }

    #[inline(always)]
    fn read_from_js(reader: T_JsData_Read, _transfers: &mut VecDeque<wb::JsValue>) -> Result<i8, L_JsData_Err> {
        reader.read_i8().map_err(|_| L_JsData_Err::BufReader)
    }
}

impl T_JsData_ for u8 {
    #[inline(always)]
    fn write_to_js(&self, writer: T_JsData_Write, _transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err> {
        Ok(writer.write_u8(*self).map_err(|_| L_JsData_Err::BufWriter)?)
    }

    #[inline(always)]
    fn read_from_js(reader: T_JsData_Read, _transfers: &mut VecDeque<wb::JsValue>) -> Result<u8, L_JsData_Err> {
        reader.read_u8().map_err(|_| L_JsData_Err::BufReader)
    }
}

impl T_JsData_ for AtomicU8 {
    #[inline(always)]
    fn write_to_js(&self, writer: T_JsData_Write, _transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err> {
        Ok(writer.write_u8(self.load(atomic::Ordering::Relaxed)).map_err(|_| L_JsData_Err::BufWriter)?)
    }

    #[inline(always)]
    fn read_from_js(reader: T_JsData_Read, _transfers: &mut VecDeque<wb::JsValue>) -> Result<Self, L_JsData_Err>
    where
        Self: Sized,
    {
        Ok(AtomicU8::new(reader.read_u8().map_err(|_| L_JsData_Err::BufReader)?))
    }
}

impl T_JsData_ for i16 {
    #[inline(always)]
    fn write_to_js(&self, writer: T_JsData_Write, _transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err> {
        Ok(writer.write_i16::<LittleEndian>(*self).map_err(|_| L_JsData_Err::BufWriter)?)
    }

    #[inline(always)]
    fn read_from_js(reader: T_JsData_Read, _transfers: &mut VecDeque<wb::JsValue>) -> Result<i16, L_JsData_Err> {
        reader.read_i16::<LittleEndian>().map_err(|_| L_JsData_Err::BufReader)
    }
}

impl T_JsData_ for u16 {
    #[inline(always)]
    fn write_to_js(&self, writer: T_JsData_Write, _transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err> {
        Ok(writer.write_u16::<LittleEndian>(*self).map_err(|_| L_JsData_Err::BufWriter)?)
    }

    #[inline(always)]
    fn read_from_js(reader: T_JsData_Read, _transfers: &mut VecDeque<wb::JsValue>) -> Result<u16, L_JsData_Err> {
        reader.read_u16::<LittleEndian>().map_err(|_| L_JsData_Err::BufReader)
    }
}

impl T_JsData_ for AtomicU16 {
    #[inline(always)]
    fn write_to_js(&self, writer: T_JsData_Write, _transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err> {
        Ok(writer
            .write_u16::<LittleEndian>(self.load(atomic::Ordering::Relaxed))
            .map_err(|_| L_JsData_Err::BufWriter)?)
    }

    #[inline(always)]
    fn read_from_js(reader: T_JsData_Read, _transfers: &mut VecDeque<wb::JsValue>) -> Result<Self, L_JsData_Err>
    where
        Self: Sized,
    {
        Ok(AtomicU16::new(reader.read_u16::<LittleEndian>().map_err(|_| L_JsData_Err::BufReader)?))
    }
}

impl T_JsData_ for i32 {
    #[inline(always)]
    fn write_to_js(&self, writer: T_JsData_Write, _transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err> {
        Ok(writer.write_i32::<LittleEndian>(*self).map_err(|_| L_JsData_Err::BufWriter)?)
    }

    #[inline(always)]
    fn read_from_js(reader: T_JsData_Read, _transfers: &mut VecDeque<wb::JsValue>) -> Result<i32, L_JsData_Err> {
        reader.read_i32::<LittleEndian>().map_err(|_| L_JsData_Err::BufReader)
    }
}

impl T_JsData_ for f32 {
    #[inline(always)]
    fn write_to_js(&self, writer: T_JsData_Write, _transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err> {
        Ok(writer.write_f32::<LittleEndian>(*self).map_err(|_| L_JsData_Err::BufWriter)?)
    }

    #[inline(always)]
    fn read_from_js(reader: T_JsData_Read, _transfers: &mut VecDeque<wb::JsValue>) -> Result<f32, L_JsData_Err> {
        reader.read_f32::<LittleEndian>().map_err(|_| L_JsData_Err::BufReader)
    }
}

impl T_JsData_ for u32 {
    #[inline(always)]
    fn write_to_js(&self, writer: T_JsData_Write, _transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err> {
        Ok(writer.write_u32::<LittleEndian>(*self).map_err(|_| L_JsData_Err::BufWriter)?)
    }

    #[inline(always)]
    fn read_from_js(reader: T_JsData_Read, _i_transfers: &mut VecDeque<wb::JsValue>) -> Result<u32, L_JsData_Err> {
        reader.read_u32::<LittleEndian>().map_err(|_| L_JsData_Err::BufReader)
    }
}

impl T_JsData_ for i64 {
    #[inline(always)]
    fn write_to_js(&self, writer: T_JsData_Write, _transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err> {
        Ok(writer.write_i64::<LittleEndian>(*self).map_err(|_| L_JsData_Err::BufWriter)?)
    }

    #[inline(always)]
    fn read_from_js(reader: T_JsData_Read, _transfers: &mut VecDeque<wb::JsValue>) -> Result<i64, L_JsData_Err> {
        reader.read_i64::<LittleEndian>().map_err(|_| L_JsData_Err::BufReader)
    }
}

impl T_JsData_ for u64 {
    #[inline(always)]
    fn write_to_js(&self, writer: T_JsData_Write, _transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err> {
        Ok(writer.write_u64::<LittleEndian>(*self).map_err(|_| L_JsData_Err::BufWriter)?)
    }

    #[inline(always)]
    fn read_from_js(reader: T_JsData_Read, _transfers: &mut VecDeque<wb::JsValue>) -> Result<u64, L_JsData_Err> {
        reader.read_u64::<LittleEndian>().map_err(|_| L_JsData_Err::BufReader)
    }
}

impl T_JsData_ for i128 {
    #[inline(always)]
    fn write_to_js(&self, writer: T_JsData_Write, _transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err> {
        Ok(writer.write_i128::<LittleEndian>(*self).map_err(|_| L_JsData_Err::BufWriter)?)
    }

    #[inline(always)]
    fn read_from_js(reader: T_JsData_Read, _transfers: &mut VecDeque<wb::JsValue>) -> Result<i128, L_JsData_Err> {
        reader.read_i128::<LittleEndian>().map_err(|_| L_JsData_Err::BufReader)
    }
}

impl T_JsData_ for f64 {
    #[inline(always)]
    fn write_to_js(&self, writer: T_JsData_Write, _transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err> {
        Ok(writer.write_f64::<LittleEndian>(*self).map_err(|_| L_JsData_Err::BufWriter)?)
    }

    #[inline(always)]
    fn read_from_js(reader: T_JsData_Read, _transfers: &mut VecDeque<wb::JsValue>) -> Result<f64, L_JsData_Err> {
        reader.read_f64::<LittleEndian>().map_err(|_| L_JsData_Err::BufReader)
    }
}

impl T_JsData_ for u128 {
    #[inline(always)]
    fn write_to_js(&self, writer: T_JsData_Write, _transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err> {
        Ok(writer.write_u128::<LittleEndian>(*self).map_err(|_| L_JsData_Err::BufWriter)?)
    }

    #[inline(always)]
    fn read_from_js(reader: T_JsData_Read, _transfers: &mut VecDeque<wb::JsValue>) -> Result<u128, L_JsData_Err> {
        reader.read_u128::<LittleEndian>().map_err(|_| L_JsData_Err::BufReader)
    }
}
