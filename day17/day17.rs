fn main() {
    let x0 = 169;
    let x1 = 206;
    let y0 = -108;
    let y1 = -68;

    let mut max = 0;

    let mut velocities = Vec::new();

    for dy in -800..800 {
        for dx in 1..=x1 {
            let mut _dx: isize = dx;
            let mut _dy: isize = dy;

            let mut x = 0;
            let mut y = 0;
            let mut max_y = 0;

            while x <= x1 && y >= y0 {
                x += _dx;
                y += _dy;
                if x <= x1 && y >= y1 {
                    max_y = max_y.max(y);
                }

                if _dx != 0 {
                    if _dx < 0 {
                        _dx += 1;
                    } else {
                        _dx -= 1;
                    }
                }
                _dy -= 1;

                if x >= x0 && x <= x1 && y >= y0 && y <= y1 {
                    velocities.push((dx, dy));
                    max = max.max(max_y);
                    break;
                }
            }
        }
    }

    println!("part 1: {}", max);
    println!("part 2: {}", velocities.len());
}
