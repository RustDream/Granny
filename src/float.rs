struct Ix {
    base: i8,
    tail: Vec<u8>,
}

impl Ix {
	fn new() -> Self {
		Ix{
			base: 1,
			tail: Vec::new(),
		}
	}
    fn set(value: &str) -> Self {
		let (foo, sign_flag, point_flag) = get_num(value);
		if point_flag {
			panic!("Not a integer");
		}
        let mut bar = Ix::new();
		if sign_flag {
			bar.base = -1;
		}
		
		bar
    }
}

fn get_num(value: &str) -> (String, bool, bool) {
    let mut sign_flag = false;
    let mut point_flag = false;
    let mut foo = "".to_string();
    for c in value.chars() {
        match c {
            '-' => {
                if sign_flag {
                    panic!("Not a value");
                }
                sign_flag = true;
            }
            e @ '0'...'9' => {
                if sign_flag {
                    panic!("Not a value");
                }
                foo = foo + &*format!("{}", e);
            }
            '.' => {
                if point_flag {
                    panic!("Not a value");
                }
                point_flag = true;
                foo = foo + ".";
            }
            _ => panic!("Not a value"),
        }
    }
    (foo, sign_flag, point_flag)
}
