pub fn test1() {
    struct NewStruct {
        field_one: i32,
        field_two: String,
    }

    let mut st = NewStruct {
        field_one: 34,
        field_two: String::from("seres"),
    };

    st.field_one = 10;

    println!("{}", st.field_two);

    let mut vc = vec![1, 2, 3];
    vc.push(2);
    println!("vc: {:?}", vc);
    let arr = [0, 1, 2, 3];
    let mid = &arr[1..3];
    println!("{:?}", mid);
    println!("{}", arr[2]);
    let mut strobj = "jgwk".to_string();
    println!("{}", strobj);
    strobj.pop();
    println!("{:?}", strobj);
}

pub fn add_one(x: &i32) -> i32 {
    x + 1
}

pub fn test2() {
    let x = 1;
    let y = add_one(&x);
    println!("{}", y);
}

macro_rules! customlist {
    // $x 是变量
    // :expr 是关键字语法, 表示表达式
    // * 表示零次或多次表达式匹配
    ($($x:expr), *) => {
        {
            let mut temp_vec = Vec::new();
            $(                          
                println!("{}", $x);
                temp_vec.push($x);
            )* // 多次匹配会多次运行这个代码块.
            temp_vec
        }
    }
}

fn main() {
    //test2()
    let x = customlist!(1,2,3);
    println!("{:?}", x)
}
