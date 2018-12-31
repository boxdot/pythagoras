extern "C" {
    fn printf(fmt: *const i8, ...) -> i32;
}

pub fn main() {
    print_triples();
}

fn print_triples() {
    let mut i = 0 as i32;
    for z in 1.. {
        for x in 1..z + 1 {
            for y in x..z + 1 {
                if x * x + y * y == z * z {
                    unsafe { printf("(%i, %i, %i)\n".as_ptr() as *const i8, x, y, z) };
                    i = i + 1;
                    if i == 1000 {
                        return;
                    }
                }
            }
        }
    }
}
