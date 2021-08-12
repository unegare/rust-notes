//rust 1.56

use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;

fn main() {
    const LEN: usize = 1_000_000_usize;
    let mut v: Vec<u64> = Vec::with_capacity(LEN);
    let mut rng = rand::thread_rng();
    for _i in 0..LEN {
        v.push(rng.gen());
    }

    let mut times: Vec<SystemTime> = Vec::with_capacity(10);

    // debug: 55 ms, release: 23 ms 
    {
      let v_copy = v.clone();
      let t1 = SystemTime::now();
      let v2 = v_copy.into_iter().map(|x| (x as f64, (x as f64).powf(3_f64), 3_f64*(x as f64))).collect::<Vec<(f64, f64, f64)>>();
      let t2 = SystemTime::now();
      times.push(t1);
      times.push(t2);
      println!("rand: {:?}", v2[rng.gen::<usize>() % LEN]);
    }

    // debug: 53 ms, release: 23 ms
    {
      let v_copy = v.clone();
      let t1 = SystemTime::now();
      let v2 = v_copy.into_iter().map(|x| {let el = x as f64; (el, el.powf(3_f64), 3_f64*el)}).collect::<Vec<(f64, f64, f64)>>();
      let t2 = SystemTime::now();
      times.push(t1);
      times.push(t2);
      println!("rand: {:?}", v2[rng.gen::<usize>() % LEN]);
    }

    // debug: 66 ms, release: 17 ms
    {
      let t1 = SystemTime::now();
      let mut v3: Vec<(f64, f64, f64)> = Vec::with_capacity(LEN);
      let mut i = 0_usize;
      while i < LEN {
          let el = v[i] as f64;
          v3.push((el, el.powf(3_f64), 3_f64*el));
          i += 1;
      }
      let t2 = SystemTime::now();
      times.push(t1);
      times.push(t2);
      println!("rand: {:?}", v3[rng.gen::<usize>() % LEN]);
    }

    // debug: 67 ms, release: 17 ms
    {
      let t1 = SystemTime::now();
      let mut v3: Vec<(f64, f64, f64)> = Vec::with_capacity(LEN);
      let mut i = 0_usize;
      while i < LEN {
          let el = v[i];
          v3.push((el as f64, (el as f64).powf(3_f64), 3_f64*(el as f64)));
          i += 1;
      }
      let t2 = SystemTime::now();
      times.push(t1);
      times.push(t2);
      println!("rand: {:?}", v3[rng.gen::<usize>() % LEN]);
    }

    // debug: 118 ms, release: 17 ms
    {
      let t1 = SystemTime::now();
      let mut v4: Vec<(f64, f64, f64)> = Vec::with_capacity(LEN);
      let mut i = 0_usize;
      while i < LEN {
          v4.push((v[i] as f64, (v[i] as f64).powf(3_f64), 3_f64*(v[i] as f64)));
          i += 1;
      }
      let t2 = SystemTime::now();
      times.push(t1);
      times.push(t2);
      println!("rand: {:?}", v4[rng.gen::<usize>() % LEN]);
    }

    // debug: 134 ms, release: 17 ms
    {
      let t1 = SystemTime::now();
      let mut v5: Vec<(f64, f64, f64)> = Vec::with_capacity(LEN);
      for i in 0..LEN {
          v5.push((v[i] as f64, (v[i] as f64).powf(3_f64), 3_f64*(v[i] as f64)));
      }
      let t2 = SystemTime::now();
      times.push(t1);
      times.push(t2);
      println!("rand: {:?}", v5[rng.gen::<usize>() % LEN]);
    }

    for i in (0..times.len()).step_by(2) {
      println!("{}", times[i+1].duration_since(UNIX_EPOCH).unwrap().as_millis() - times[i].duration_since(UNIX_EPOCH).unwrap().as_millis());
    }
}
