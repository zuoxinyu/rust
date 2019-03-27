fn main() {
    variable_and_mutability();
    let _ = lifetime(&2, &3);
    ownership();
    lexical_scope_and_lifetime();
}

#[derive(Debug)]
struct S {
    i : i32
}

#[derive(Debug)]
enum E {
    Integer(i32),
    Structure(S),
    Nothing,
}

fn variable_and_mutability() {
    let const_i = 3; // const int const_i = 3;
    // const_i = 2; // illegal, change an immutable variable
    println!("&const_i is {:p}", (&const_i) as * const i32); 

    let mut mut_i = 3; // int j = 3;
    println!("&mut_i is {:p}", (&mut_i) as * const i32); 
    mut_i = 4;
    println!("&mut_i is {:p}", (&mut_i) as * const i32); 

    let string : &'static str = "static string"; // string is on the .data segment
    println!("string is {:p}", string as * const str); 
    let static_ref_i : &'static i32 = &3; // *static_ref_i is on the .data segment
    println!("static_ref_i is {:p}", static_ref_i as * const i32); 
    let static_ref_j : &'static i32 = &3; // static_ref_j referenced the same data 3 
    println!("static_ref_j is {:p}", static_ref_j as * const i32); 

    let y = 3;
    let ref_const_i : &i32 = &y; // const int y = 3; const int * const ref_i = &y; y is on stack
    // ref_const_i = &4;
    println!("ref_const_i is {:p}", ref_const_i as * const i32); 

    let ref_mut_i : &mut i32 = &mut 3; // int x = 3; int * const ref_mut_i = &x; the data 3 is on stack since it would be dropped at the end of scope
    println!("ref_mut_i is {:p}", ref_mut_i as * const i32); 
    *ref_mut_i = 2;
    // mut_ref_i = &mut 4; // illegal
    println!("ref_mut_i is {:p}", ref_mut_i as * const i32);

    let mut mut_ref_i : &'static i32 = &3; //const int x = 3; const int * mut_binding_i = &x;
    // *mut_ref_i = 4; // illegal
    println!("mut_ref_i is {:p}", mut_ref_i as * const i32);
    mut_ref_i = &4; 
    println!("mut_ref_i is {:p}", mut_ref_i as * const i32);

    let const_s = S{i:3}; // const const_s = S{3}
    // const_s.i = 3;
    println!("&const_s is {:p}", &const_s as * const S); 

    let const_ref_s : &'static S = &S{i:3}; // the data S{i:3} is on the .data segment
    println!("&const_ref_s is {:p}", const_ref_s as * const S); 

    let mut mut_s = S{i:3}; // S mut_s = S{3}
    println!("&mut_s is {:p}", &mut_s as * const S); 
    mut_s.i = 0;
    println!("&mut_s is {:p}", &mut_s as * const S); 

    let mut_ref_s = &mut S{i:3}; // S s = S{3}; S * const mut_ref_s = &s;
    println!("mut_ref_s is {:p}", mut_ref_s as * const S);
    mut_ref_s.i = 2; // (*mut_ref_s).i = 2; auto dereffed
    println!("mut_ref_s is {:p}", mut_ref_s as * const S);

    let mut mut_binding_ref_s = &mut S{i:3}; // S s = S{3}; S * mut_binding_ref_s = &s;
    println!("mut_binding_ref_s is {:p}", mut_binding_ref_s as * const S);
    let mut mut_s = S{i:4};
    mut_binding_ref_s = &mut mut_s;
    println!("mut_binding_ref_s is {:p}", mut_binding_ref_s as * const S);

    let const_arr : [i32;3] = [1, 2, 3]; // const int const_arr[3] = {1,2,3} , const_arr is on stack
    println!("&const_arr is {:p}", &const_arr as * const [i32;3]);

    let ref_const_arr : &'static [i32;3] = &[1,2,3];
    println!("ref_cosnt_arr is {:p}", ref_const_arr as * const [i32;3]);

    let mut mut_arr : [i32;3] = [1, 2, 3];
    println!("&mut_arr is {:p}", &mut_arr as * const [i32;3]);
    mut_arr[0] = 4;
    println!("&mut_arr is {:p}", &mut_arr as * const [i32;3]);

    let const_e = E::Integer(3);
    println!("&const_e is {:p}", &const_e as * const E);
    let const_e_nothing = E::Nothing;
    let another_const_e_nothing = E::Nothing;
    println!("&const_e_nothing is {:p}", &const_e_nothing as * const E);
    println!("&another_const_e_nothing is {:p}", &another_const_e_nothing as * const E);
}

fn lifetime<'a>(i :&'a i32, _j :&i32) -> &'a i32 {
    i
}

fn ownership() {
    let mut x = Box::new(1); // x owns the mem
    {
        let ref mut mut_ref_x = x;
        //*x = 3;
        **mut_ref_x = 4;
    }
    println!("x is {}", *x);
}

fn lexical_scope_and_lifetime() {
    let s = "hello".to_string();
    {
        s; // move occurs here
    }
    // println!("{}", s);
    
    let y:String = "hello".to_string();
}
