fn main() {
    let text = "it is what it is\0"; //hola como estas? lola
    println!("{}", text);

    let decimal_list = text.as_bytes();

    println!("{:?}", decimal_list);

    let mut compressed_decimal_list: Vec<u16> = Vec::new();
    let mut memory: Vec<Vec<u16>> = Vec::new();
    let mut k: u16 = decimal_list[0] as u16;
    for byte in &decimal_list[1..] {

        let w= k;
        k = *byte as u16;

        let aux_list: Vec<u16> = vec![w as u16, k as u16];

        let mut is_found = false;
        let mut index = 0;
        for (i,list) in memory.iter().enumerate() {
            if list.eq(&aux_list) {
                is_found = true;
                index = i;
            }
        }
        if is_found {
            k = 256 + index as u16 ;
        } else {
            memory.push(aux_list.clone());
            compressed_decimal_list.push(w as u16);
        }

    }

    //println!("{:?}", memory);
    println!("{:?}", compressed_decimal_list);
}
