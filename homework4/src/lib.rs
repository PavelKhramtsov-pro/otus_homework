///Принимает мутабельную ссылку на кортеж и bool значение.
///Если false, возвращает мутабельную ссылку на первый элемент кортежа.
///Если true, возвращает мутабельную ссылку на второй элемент кортежа.
pub fn func1(tuple: &mut (u32, u32), key: bool)->&mut u32{
	if key {&mut tuple.0 } else {&mut tuple.1}
}

///Принимает мутабельную ссылку на слайс и число N. Возвращает мутабельную ссылку на N-ый элемент.
pub fn func2(slice: &mut [u32], idx: usize)-> &mut u32{
	&mut slice[idx]
}

///Принимает слайс и число N. Возвращает ссылку на N-ый элемент слайса с конца.
pub fn func3(slice: &mut [u32], idx: usize)-> &u32{
	&slice[slice.len()-idx-1]
}

///Принимает слайс и число N. Возвращает два слайса с элементами:
///с нулевого по N-1;
///с N-го по последний;
pub fn func4(slice: &[u32], idx: usize)-> (&[u32],&[u32]){
	(&slice[..idx], &slice[idx..])
}

///Принимает слайс и возвращает массив слайсов, содержащий четыре равные (насколько возможно) части исходного слайса.
pub fn func5(slice: &[u32])->[&[u32];4]{
	let cnt_part: usize = 4;
	let step:usize = (slice.len()+1)/cnt_part;
	let mut res:[&[u32];4] = [&[]; 4];
	for idx in 0..(cnt_part-1){
		res[idx] = &slice[idx*step..(idx+1)*step]
	}
	res[cnt_part-1] = &slice[(cnt_part-1)*step..];
	res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_func1() {
        let mut val = (3, 2);
	    let &mut res = func1(&mut val, true);
        assert_eq!(res, 3);

        //Копилятор не позволяет вернуть более одной мутабельной ссылки на один объект
        //let res:&mut u32 = func1(&mut val, true);
        //let res2:&mut u32 = func1(&mut val, true);
        //println!("{}",res2);
        //println!("{}", res);
    }

    #[test]
    fn test_func2() {
        let mut val = [3, 2, 1, 0];
        let &mut res = func2(&mut val[1..3], 1);
        assert_eq!(res, 1);

        //Копилятор не позволяет вернуть более одной мутабельной ссылки на один объект
        //let res = func2(&mut val[1..3], 1);
        //let res2 = func2(&mut val[1..3], 1);
        //println!("{}",res2);
        //println!("{}", res);
    }

    #[test]
    fn test_func3() {
        let mut val = [3, 2, 1, 0];
        let res = func3(&mut val[1..3], 1);
        let test_val:u32 = 2;
        assert_eq!(res, &test_val);
    }

    #[test]
    fn test_func4() {
        let val = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        let test_left = &[8, 7, 6, 5, 4];
        let test_right = &[3, 2];
        let (left, right) = func4( &val[1..8], 5);
        assert_eq!(left, test_left);
        assert_eq!(right, test_right);
    }

    #[test]
    fn test_func5() {
        //test function 5
        let val = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        let res = func5( &val[1..8]);
        //let test_val:[&[u32]; 4] = [&[8,7], &[6, 5], &[4, 3], &[2]];
        //assert_eq!(res, test_val);
        println!("{:#?}", res)
    }
}
