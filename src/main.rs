use std::io::{self, Write};

fn main() {
    let mut a: f64 = 0.0;
    let mut b: f64 = 0.0;
    let mut z: [f64; 1760] = [0.0; 1760];
    let mut d: [char; 1760] = [' '; 1760];

    loop {
        for i in 0..628 {
            for j in 0..628 {
                let sin_i = f64::sin(i as f64 / 100.0);
                let cos_j = f64::cos(j as f64 / 100.0);
                let sin_a = f64::sin(a);
                let sin_j = f64::sin(j as f64 / 100.0);
                let cos_a = f64::cos(a);
                let cos_j2 = cos_j + 2.0;
                let mess = 1.0 / (sin_i * cos_j2 * sin_a + sin_j * cos_a + 5.0);
                let cos_i = f64::cos(i as f64 / 100.0);
                let cos_b = f64::cos(b);
                let sin_b = f64::sin(b);
                let t = sin_i * cos_j2 * cos_a - sin_j * sin_a;
                let x = (40.0 + 30.0 * mess * (cos_i * cos_j2 * cos_b - t * sin_b)) as usize;
                let y = (12.0 + 15.0 * mess * (cos_i * cos_j2 * sin_b + t * cos_b)) as usize;
                let o = x + 80 * y;
                let n = (8.0 * ((sin_j * sin_a - sin_i * cos_j * cos_a) * cos_b
                    - sin_i * cos_j * sin_a - sin_j * cos_a - cos_i * cos_j * sin_b))
                    as usize;
                if y < 22 && y > 0 && x > 0 && x < 80 && mess > z[o] {
                    z[o] = mess;
                    d[o] = vec!['.', ',', '-', '~', ':', ';', '=', '!', '*', '#', '$', '@']
                        .get(n)
                        .cloned()
                        .unwrap_or(' ');
                }
            }
        }

        //print!("\x1b[H");
        print!("\x1b[1G");  // Moves the cursor to the first column of the current line

        for k in 0..1760 {
            print!("{}", if k % 80 == 0 { '\n' } else { d[k] });
        }
        io::stdout().flush().unwrap();

        a += 0.04;
        b += 0.02;
    }
}
