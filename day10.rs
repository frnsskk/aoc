use std::fs::File;
use std::io::Read;

#[derive(Clone,Copy,Debug)]
pub struct Asteroid {
	x: i32,
	y: i32,
}

impl Asteroid {
	pub fn new (x: i32, y: i32) -> Asteroid {
		Asteroid {
			x,
			y,
		}
	}

	pub fn line (&self, o: &Asteroid) -> (f32,f32) {
		(o.x as f32 - self.x as f32,o.y as f32 - self.y as f32)
	}

	pub fn dist (&self, o: &Asteroid) -> i32 {
		(o.x - self.x).abs() + (o.y - self.y).abs()
	}
}

fn parse (s: &str) -> Vec<Asteroid> {
	let mut v = Vec::new();
	let (mut x,mut y) = (0,0);
	for c in s.chars() {
		match c as u8 {
			10			=>	{
								y += 1;
								x = 0;
							},
			35			=>	{
								v.push(Asteroid::new(x,y));
								x += 1;
							},
			46			=>	x += 1,
			_			=>	(),
		};
	}
	v
}

fn main () -> std::io::Result<()> {
	let mut data = String::new();
	let mut file = File::open("input10")?;
	file.read_to_string(&mut data)?;

	let v = parse(&data);
	let mut ns = Vec::new();
	for i in 0..v.len() {
		let mut n = 0;
		'outer: for j in 0..v.len() {
			if i != j {
				let mut nn = 0;
				let a = v[i].line(&v[j]);
				for k in 0..v.len() {
					if i != k && j != k {
						let b = v[i].line(&v[k]);
						if a.0 == 0.0 && b.0 == 0.0
						&& (a.1.signum() == b.1.signum())
						|| a.0 != 0.0 && b.0 != 0.0
						&& (a.0.signum() == b.0.signum() && a.1.signum() == b.1.signum())
						&& a.1/a.0 == b.1/b.0 {
							if v[i].dist(&v[j]) < v[i].dist(&v[k]) {
								nn += 1;
							}
							else {
								continue 'outer;
							}
						}
					}
				}
				n += nn;
			}
		}
		ns.push((i,v.len() as i32 - n - 1));
	}
	println!("{:?}",ns);
	let mut most = (-1,0);
	for n in ns {
		if n.1 > most.1 {
			most = (n.0 as i32,n.1);
		}
	}
	println!("{:?}",most);

	let mut v2 = Vec::new();
	let a = &v[most.0 as usize];
	'outer2: for b in &v {
		let ab = a.line(&b);
		if ab != (0.0,0.0) {
			for c in &v {
				let ac = a.line(&c);
				if (ab.0 == 0.0 && ac.0 == 0.0 && ab.1.signum() == ac.1.signum()
				|| ab.0 != 0.0 && ac.0 != 0.0 && ab.0.signum() == ac.0.signum()
				&& ab.1.signum() == ac.1.signum() && ab.1/ab.0 == ac.1/ac.0)
				&& a.dist(&c) < a.dist(&b) {
					continue 'outer2;
				}
			}
			let mut ds = (1800.0*(-b.y as f32 + a.y as f32).atan2(b.x as f32 - a.x as f32)/std::f32::consts::PI) as i32 - 900;
			if ds <= 0 {
				ds += 3600;
			}
			v2.push((b,ds));
		}
	}
	v2.sort_by_key(|x| -x.1);
	println!("{:?}",v2);
	println!("{}",100*v2[198].0.x + v2[198].0.y);

	Ok(())
}
