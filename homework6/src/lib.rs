//юнит-структура
pub struct Convector;

impl Convector{
    // принимает 32-х битное целое знаковое число и возвращает 32-х битное целое знаковое число, равное удвоенному входному.
    pub fn double_int32(&self, val: i32) -> i32 {
        val * 2
    }

    // принимает 32-х битное целое знаковое число и возвращает 64-х битное целое знаковое число, равное удвоенному входному.
    pub fn double_int64(&self, val: i32) -> i64 {
        (val as i64) * 2
    }

    // принимает 32-х битное число с плавающей точкой и возвращает 32-х битное число с плавающей точкой, равное удвоенному входному.
    pub fn double_float32(&self, val: f32) -> f32 {
        val * 2.0f32
    }

    // принимает 32-х битное число с плавающей точкой и возвращает 64-х битное число с плавающей точкой, равное удвоенному входному.
    pub fn double_float64(&self, val: f32) -> f64 {
        (val as f64) * 2.0f64
    }

    // принимает 32-х битное целое знаковое число и 32-х битное число с плавающей точкой. Возвращает 64-х битное число с плавающей точкой, равное сумме входных.
    pub fn int_plus_float_to_float(&self, val_i: i32, val_f32: f32) -> f64 {
        (val_i as f64) + (val_f32 as f64)
    }

    // принимает 32-х битное целое знаковое число и 32-х битное число с плавающей точкой. Возвращает 64-х битное целое знаковое число, равное сумме входных.
    pub fn int_plus_float_to_int(&self, val_i: i32, val_f32: f32) -> i64 {
        (val_i as i64) + (val_f32 as i64)
    }

    // принимает кортеж из двух целых чисел. Возвращает целое число, равное сумме чисел во входном кортеже.
    pub fn tuple_sum(&self, val: (i32, i32)) -> i64 {
        (val.0 as i64) + (val.1 as i64)
    }

    // принимает массив из трёх целых чисел. Возвращает целое число, равное сумме чисел во входном массиве.
    pub fn array_sum(&self, val: [i16; 3]) -> i64 {
        (val[0] as i64) + (val[1] as i64) + (val[2] as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_int32(){
        let convector = Convector;
        let res = convector.double_int32(2);
        assert_eq!(res, 4);
    }

    #[test]
    fn test_double_int64(){
        let convector = Convector;
        let res = convector.double_int64(3);
        assert_eq!(res, 6);
    }

    #[test]
    fn test_double_float32(){
        let convector = Convector;
        let res = convector.double_float32(3.1);
        assert_eq!(res, 6.2);
    }

    #[test]
    fn test_double_float64(){
        let convector = Convector;
        let res = convector.double_float64(3.1);
        assert!( (res-6.2).abs() < 0.000001 );
    }

    #[test]
    fn test_int_plus_float_to_float(){
        let convector = Convector;
        let res = convector.int_plus_float_to_float(2, 3.1);
        assert!((res-5.1).abs() < 0.000001);
    }

    #[test]
    fn test_int_plus_float_to_int(){
        let convector = Convector;
        let res = convector.int_plus_float_to_int(2, 3.1);
        assert_eq!(res, 5);
    }

    #[test]
    fn test_tuple_sum(){
        let convector = Convector;
        let res = convector.tuple_sum((32, 32));
        assert_eq!(res, 64);
    }

    #[test]
    fn test_array_sum(){
        let convector = Convector;
        let res = convector.array_sum([1, 2, 3]);
        assert_eq!(res, 6);
    }

}
