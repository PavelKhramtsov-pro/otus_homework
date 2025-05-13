//структура кольцевого буфера
struct RingBuffer {
    read_idx: usize,
    write_idx: usize,
    data: Vec<u8>,
}

//создание кольцевого буфера
fn create(size: usize) -> RingBuffer {
    RingBuffer {
        read_idx: 0,
        write_idx: 0,
        data: vec![0; size],
    }
}

//чтение из буфера
fn read(rb: &mut RingBuffer, cnt: usize) -> String {
    let mut cnt_read: usize = 0;
    let mut result: Vec<u8> = Vec::new();

    //кол-во записанных данных
    let mut cnt_data: usize = if rb.write_idx > rb.read_idx {
        rb.write_idx - rb.read_idx
    } else {
        (rb.data.len() - rb.read_idx) + rb.write_idx
    };

    //Чтение данных
    while cnt_read < cnt && cnt_data > 0 {
        result.push(rb.data[rb.read_idx]);
        rb.read_idx = (rb.read_idx + 1) % rb.data.len();
        cnt_read += 1;
        cnt_data -= 1;
    }

    String::from_utf8(result).unwrap()
}

//запись
fn write(rb: &mut RingBuffer, data: &str) -> usize {
    let mut cnt_write: usize = 0;

    //определение свободного места
    let mut cnt_free: usize = if rb.write_idx < rb.read_idx {
        (rb.data.len() - rb.read_idx) + rb.write_idx
    } else {
        (rb.data.len() - rb.write_idx) + rb.read_idx
    };

    //запись данных
    while cnt_write < data.len() && cnt_free > 0 {
        rb.data[rb.write_idx] = data.as_bytes()[cnt_write];
        rb.write_idx = (rb.write_idx + 1) % rb.data.len();
        cnt_write += 1;
        cnt_free -= 1;
    }
    cnt_write
}

fn main() {
    let mut buf = create(3);

    let cnt = write(&mut buf, "abc");
    assert_eq!(cnt, 3);

    let res = read(&mut buf, 1);
    assert_eq!(res, String::from("a"));

    let cnt = write(&mut buf, "d");
    assert_eq!(cnt, 1);

    let res = read(&mut buf, 3);
    assert_eq!(res, String::from("bcd"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    //тест на верный размер созданного буфера
    fn test_create_cnt() {
        let buf = create(3);
        assert_eq!(buf.data.len(), 3);
    }

    #[test]
    //тест на запись
    fn test_write() {
        let mut buf = create(3);
        let cnt_test = write(&mut buf, "abcd");
        assert_eq!(cnt_test, 3);
    }

    #[test]
    //тест на чтение
    fn test_read() {
        let mut buf = create(3);
        write(&mut buf, "abc");
        let res = read(&mut buf, 4);
        let res_test = String::from("abc");
        assert_eq!(res, res_test);
    }

    #[test]
    //тест на переход через конец
    fn test_write_read() {
        let mut buf = create(3);

        let cnt = write(&mut buf, "abc");
        assert_eq!(cnt, 3);

        let res = read(&mut buf, 1);
        assert_eq!(res, String::from("a"));

        let cnt = write(&mut buf, "d");
        assert_eq!(cnt, 1);

        let res = read(&mut buf, 3);
        assert_eq!(res, String::from("bcd"));
    }
}
