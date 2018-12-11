pub fn entry() {
    let serial = 18;//1723;

    const MAX : usize = 300;
    let mut grid : [[i32; MAX]; MAX] = [[0 ; MAX]; MAX];

    for x in 0..MAX {
        for y in 0..MAX {
            let rack_id : i32 = x as i32 + 11;
            let total : i32 = (rack_id * (y as i32 +1) + serial) * rack_id;
            let shifted = (total / 100) % 10 as i32 - 5;
            grid[x][y] = shifted;
        }
    }

    let mut max = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_s = 0;
    for s in 0..MAX {
        if s % 10 == 0 {
            println!("Trying size {}", s);
        }
        for x in 0..MAX-s {
            for y in 0..MAX-s {
                let mut val = 0;
                for ix in 0..s {
                    for iy in 0..s {
                        val += grid[x+ix][y+iy];
                    }
                }
                if val > max {
                    max = val;
                    max_x = x+1;
                    max_y = y+1;
                    max_s = s;
                }
            }
        }
    }
    println!("Max {} {}x{}x{}", max, max_x, max_y,max_s);
}