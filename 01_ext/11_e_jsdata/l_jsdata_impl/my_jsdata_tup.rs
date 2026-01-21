#[allow(unused_imports)]
use super::*;

impl T_JsData_ for () {
    #[inline(always)]
    fn write_to_js(&self, _writer: T_JsData_Write, _transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err>
    where
        Self: Sized,
    {
        Ok(())
    }

    #[inline(always)]
    fn read_from_js(_reader: T_JsData_Read, _transfers: &mut VecDeque<wb::JsValue>) -> Result<Self, L_JsData_Err>
    where
        Self: Sized,
    {
        Ok(())
    }
}

impl<T> T_JsData_ for std::marker::PhantomData<T> {
    #[inline(always)]
    fn write_to_js(&self, _writer: T_JsData_Write, _transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err>
    where
        Self: Sized,
    {
        Ok(())
    }

    #[inline(always)]
    fn read_from_js(_reader: T_JsData_Read, _transfers: &mut VecDeque<wb::JsValue>) -> Result<Self, L_JsData_Err>
    where
        Self: Sized,
    {
        Ok(PhantomData::default())
    }
}

impl<T0: T_JsData_, T1: T_JsData_> T_JsData_ for (T0, T1) {
    #[inline(always)]
    fn write_to_js(&self, writer: T_JsData_Write, _transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err> {
        self.0.write_to_js(writer, _transfers)?;
        self.1.write_to_js(writer, _transfers)?;

        Ok(())
    }

    #[inline(always)]
    fn read_from_js(reader: T_JsData_Read, _transfers: &mut VecDeque<wb::JsValue>) -> Result<(T0, T1), L_JsData_Err> {
        let t0 = <T0 as T_JsData_>::read_from_js(reader, _transfers)?;
        let t1 = <T1 as T_JsData_>::read_from_js(reader, _transfers)?;
        Ok((t0, t1))
    }
}

impl<T0: T_JsData_, T1: T_JsData_, T2: T_JsData_> T_JsData_ for (T0, T1, T2) {
    #[inline(always)]
    fn write_to_js(&self, writer: T_JsData_Write, _transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err> {
        self.0.write_to_js(writer, _transfers)?;
        self.1.write_to_js(writer, _transfers)?;
        self.2.write_to_js(writer, _transfers)?;
        Ok(())
    }

    #[inline(always)]
    fn read_from_js(reader: T_JsData_Read, _transfers: &mut VecDeque<wb::JsValue>) -> Result<(T0, T1, T2), L_JsData_Err> {
        let t0 = <T0 as T_JsData_>::read_from_js(reader, _transfers)?;
        let t1 = <T1 as T_JsData_>::read_from_js(reader, _transfers)?;
        let t2 = <T2 as T_JsData_>::read_from_js(reader, _transfers)?;
        Ok((t0, t1, t2))
    }
}

impl<T0: T_JsData_, T1: T_JsData_, T2: T_JsData_, T3: T_JsData_> T_JsData_ for (T0, T1, T2, T3) {
    #[inline(always)]
    fn write_to_js(&self, writer: T_JsData_Write, _transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err> {
        self.0.write_to_js(writer, _transfers)?;
        self.1.write_to_js(writer, _transfers)?;
        self.2.write_to_js(writer, _transfers)?;
        self.3.write_to_js(writer, _transfers)?;
        Ok(())
    }

    #[inline(always)]
    fn read_from_js(reader: T_JsData_Read, _transfers: &mut VecDeque<wb::JsValue>) -> Result<(T0, T1, T2, T3), L_JsData_Err> {
        let t0 = <T0 as T_JsData_>::read_from_js(reader, _transfers)?;
        let t1 = <T1 as T_JsData_>::read_from_js(reader, _transfers)?;
        let t2 = <T2 as T_JsData_>::read_from_js(reader, _transfers)?;
        let t3 = <T3 as T_JsData_>::read_from_js(reader, _transfers)?;
        Ok((t0, t1, t2, t3))
    }
}

impl<T0: T_JsData_, T1: T_JsData_, T2: T_JsData_, T3: T_JsData_, T4: T_JsData_> T_JsData_ for (T0, T1, T2, T3, T4) {
    #[inline(always)]
    fn write_to_js(&self, writer: T_JsData_Write, _transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err> {
        self.0.write_to_js(writer, _transfers)?;
        self.1.write_to_js(writer, _transfers)?;
        self.2.write_to_js(writer, _transfers)?;
        self.3.write_to_js(writer, _transfers)?;
        self.4.write_to_js(writer, _transfers)?;
        Ok(())
    }

    #[inline(always)]
    fn read_from_js(reader: T_JsData_Read, _transfers: &mut VecDeque<wb::JsValue>) -> Result<(T0, T1, T2, T3, T4), L_JsData_Err> {
        let t0 = <T0 as T_JsData_>::read_from_js(reader, _transfers)?;
        let t1 = <T1 as T_JsData_>::read_from_js(reader, _transfers)?;
        let t2 = <T2 as T_JsData_>::read_from_js(reader, _transfers)?;
        let t3 = <T3 as T_JsData_>::read_from_js(reader, _transfers)?;
        let t4 = <T4 as T_JsData_>::read_from_js(reader, _transfers)?;
        Ok((t0, t1, t2, t3, t4))
    }
}

impl<T0: T_JsData_, T1: T_JsData_, T2: T_JsData_, T3: T_JsData_, T4: T_JsData_, T5: T_JsData_> T_JsData_ for (T0, T1, T2, T3, T4, T5) {
    #[inline(always)]
    fn write_to_js(&self, writer: T_JsData_Write, _transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err> {
        self.0.write_to_js(writer, _transfers)?;
        self.1.write_to_js(writer, _transfers)?;
        self.2.write_to_js(writer, _transfers)?;
        self.3.write_to_js(writer, _transfers)?;
        self.4.write_to_js(writer, _transfers)?;
        self.5.write_to_js(writer, _transfers)?;
        Ok(())
    }

    #[inline(always)]
    fn read_from_js(reader: T_JsData_Read, _transfers: &mut VecDeque<wb::JsValue>) -> Result<(T0, T1, T2, T3, T4, T5), L_JsData_Err> {
        let t0 = <T0 as T_JsData_>::read_from_js(reader, _transfers)?;
        let t1 = <T1 as T_JsData_>::read_from_js(reader, _transfers)?;
        let t2 = <T2 as T_JsData_>::read_from_js(reader, _transfers)?;
        let t3 = <T3 as T_JsData_>::read_from_js(reader, _transfers)?;
        let t4 = <T4 as T_JsData_>::read_from_js(reader, _transfers)?;
        let t5 = <T5 as T_JsData_>::read_from_js(reader, _transfers)?;
        Ok((t0, t1, t2, t3, t4, t5))
    }
}

impl<T0: T_JsData_, T1: T_JsData_, T2: T_JsData_, T3: T_JsData_, T4: T_JsData_, T5: T_JsData_, T6: T_JsData_> T_JsData_ for (T0, T1, T2, T3, T4, T5, T6) {
    #[inline(always)]
    fn write_to_js(&self, writer: T_JsData_Write, _transfers: &mut VecDeque<wb::JsValue>) -> Result<(), L_JsData_Err> {
        self.0.write_to_js(writer, _transfers)?;
        self.1.write_to_js(writer, _transfers)?;
        self.2.write_to_js(writer, _transfers)?;
        self.3.write_to_js(writer, _transfers)?;
        self.4.write_to_js(writer, _transfers)?;
        self.5.write_to_js(writer, _transfers)?;
        self.6.write_to_js(writer, _transfers)?;
        Ok(())
    }

    #[inline(always)]
    fn read_from_js(reader: T_JsData_Read, _transfers: &mut VecDeque<wb::JsValue>) -> Result<(T0, T1, T2, T3, T4, T5, T6), L_JsData_Err> {
        let t0 = <T0 as T_JsData_>::read_from_js(reader, _transfers)?;
        let t1 = <T1 as T_JsData_>::read_from_js(reader, _transfers)?;
        let t2 = <T2 as T_JsData_>::read_from_js(reader, _transfers)?;
        let t3 = <T3 as T_JsData_>::read_from_js(reader, _transfers)?;
        let t4 = <T4 as T_JsData_>::read_from_js(reader, _transfers)?;
        let t5 = <T5 as T_JsData_>::read_from_js(reader, _transfers)?;
        let t6 = <T6 as T_JsData_>::read_from_js(reader, _transfers)?;
        Ok((t0, t1, t2, t3, t4, t5, t6))
    }
}
