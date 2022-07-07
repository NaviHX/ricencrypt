use image::{self, GenericImageView, GenericImage};

mod logistic_chaos;

fn select_sort_trans(v: &mut Vec<f64>) -> Vec<usize> {
    let mut v: Vec<(usize, f64)> = v.iter().map(|r| r.clone()).enumerate().collect();
    let mut ret: Vec<usize> = Vec::new();
    let len = v.len();
    ret.reserve(len);

    for i in 0..len {
        let mut min_idx = i;
        for j in 1..len {
            if v[j].1 < v[min_idx].1 {
                min_idx = j;
            }
        }

        ret[i] = v[min_idx].0;
        v.swap(i, min_idx);
    }

    ret

}

fn quick_sort_trans(v: &mut Vec<f64>) -> Vec<usize> {
    // `sort_by` is `O(nlogn)` according to the rust doc
    let mut v: Vec<(usize, f64)> = v.iter().map(|r| r.clone()).enumerate().collect();
    v.sort_by(|(_, fa), (_, fb)| fa.partial_cmp(fb).unwrap());
    v.into_iter().map(|(id, _)| id).collect()
}

pub enum Direction {
    Row,
    Col,
    Both,
}

pub fn encrypt(mut pic: image::DynamicImage, direction: Direction, x: f64, u: f64) -> image::DynamicImage {
    let mut gen = logistic_chaos::LogisticChaosIter::new(x, u);
    let (width, height) = pic.dimensions();

    match direction {
        Direction::Row | Direction::Both => {
            for i in 0..height {

                // get logistic map x1 .. xn and clone pixels of this row
                let mut v = vec!();
                let mut p = vec!();
                for j in 0..width {
                    v.push(gen.next().unwrap());
                    p.push(pic.get_pixel(j, i));
                }

                // sort the map
                let sort_map = quick_sort_trans(&mut v);

                for j in 0..width {
                    pic.put_pixel(sort_map[j as usize] as u32, i, p[j as usize]);
                }
            }
        },

        Direction::Col => {
            for i in 0..width {
                let mut v = vec!();
                let mut p = vec!();
                for j in 0..height {
                    v.push(gen.next().unwrap());
                    p.push(pic.get_pixel(i, j));
                }
                let sort_map = quick_sort_trans(&mut v);
                for j in 0..width {
                    pic.put_pixel(i, sort_map[j as usize] as u32, p[j as usize]);
                }
            }
        }
    }

    if let Direction::Both = direction {
        return encrypt(pic, Direction::Col, x, u);
    }

    pic
}

pub fn dencrypt(mut pic: image::DynamicImage, direction: Direction, x: f64, u: f64) -> image::DynamicImage {
    let mut gen = logistic_chaos::LogisticChaosIter::new(x, u);
    let (width, height) = pic.dimensions();

    match direction {
        Direction::Row | Direction::Both => {
            for i in 0..height {

                // get logistic map x1 .. xn and clone pixels of this row
                let mut v = vec!();
                let mut p = vec!();
                for j in 0..width {
                    v.push(gen.next().unwrap());
                    p.push(pic.get_pixel(j, i));
                }

                // sort the map
                let sort_map = quick_sort_trans(&mut v);

                for j in 0..width {
                    pic.put_pixel(j, i, p[sort_map[j as usize]]);
                }
            }
        },

        Direction::Col => {
            for i in 0..width {
                let mut v = vec!();
                let mut p = vec!();
                for j in 0..height {
                    v.push(gen.next().unwrap());
                    p.push(pic.get_pixel(i, j));
                }
                let sort_map = quick_sort_trans(&mut v);
                for j in 0..width {
                    pic.put_pixel(i, sort_map[j as usize] as u32, p[j as usize]);
                }
            }
        }
    }

    if let Direction::Both = direction {
        return encrypt(pic, Direction::Col, x, u);
    }

    pic
}
