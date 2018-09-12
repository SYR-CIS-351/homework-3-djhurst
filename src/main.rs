// Program Modified
//By: Daneil Hurst

use std::cmp;

fn main() {
    let initial = [66, 70, 52, 93, 44, 67, 47, 10, 11, 13, 94, 99, 51, 12];
    let mut to_sort;
    println!("initial:          {:?}",initial);

    to_sort  = initial.clone();
    bubble_sort(&mut to_sort);
    println!("bubble-sorted:    {:?}",to_sort);

    to_sort  = initial.clone();
    sel_sort(&mut to_sort);
    println!("selection-sorted: {:?}",to_sort);

    to_sort  = initial.clone();
    insert_sort(&mut to_sort);
    println!("insertion-sorted: {:?}",to_sort);

    println!();
    println!("unordered search:");
    report_search(44,unordered_search(44,&initial[..]));
    report_search(43,unordered_search(43,&initial[..]));

    println!();
    println!("binary search:");
    report_search(44,binary_search(44,&to_sort[..]));
    report_search(43,binary_search(43,&to_sort[..]));

    println!();
    println!("the min and max of initial are {:?}",
             min_max(&initial[..]));
}

/*
// NOTE!! The following will not compile: It needs lifetime annotations.
// We'll fix this later on.
fn swap(x :&mut u32, y : &mut u32) {
    let t = x;
    x = y;
    y = t;
}
*/

fn bubble_sort(a : &mut [u32]) {
    let len = a.len();
    for i in 0..len {
        for j in 0..(len-i-1) {
            if a[j]>a[j+1] {
                // swap the values of a[j] and a[j+1]
                let t = a[j];
                a[j] = a[j+1];
                a[j+1] = t;
            }
        }
    }
}

fn report_search(x : u32, r : Option<usize>) {
    print!("\t {} ",x);
    match r {
        None    => { println!("not found"); },
        Some(i) => { println!("found at index {}",i); },
    }
}

fn unordered_search(x : u32, a : &[u32]) -> Option<usize> {
    for i in 0..a.len() {
        if x==a[i] { return Some(i); }
    }
    None
}


fn sel_sort(a : &mut [u32]) {
	let len = a.len();
	for i in 0..(len-1){
		let mut iMin = i;
		for j in (i+1)..len{
			if a[j] < a[iMin]{
				iMin = j;
			}
		}
		if iMin != i{
			let t = a[i];
			a[i] = a[iMin];
			a[iMin] = t;
		}
		
	}
}

fn insert_sort(a : &mut [u32]) {
	let mut i=0;
	let len = a.len();
	let mut j;
	while i < len{
		j = i;
		while j > 0 && a[j-1] > a[j]{
			let t = a[j];
            a[j] = a[j-1];
            a[j-1] = t;
            j = j - 1;
		}
		i += 1;
	}
}

fn binary_search(x : u32, a : &[u32]) -> Option<usize> {
    let len = a.len();
    let mut l = 0;
    let mut r = a.len()-1;
    
    while l <= r {
    	let m = (l + r) / 2;
    	if a[m] < x {
    		l = m + 1;
    	}else if a[m] > x {
    		r = m - 1;
    	}else{
    		return Some(m);
    	}
    }


    None
}

fn min_max(a : &[u32]) -> (u32,u32) {
    let len = a.len();
    assert!(len>0);
    let mut m1 : (u32,u32);
    let mut m2 : (u32,u32);

    if len <= 2 {
    	return ( min((a[0],a[len-1])) , max((a[0],a[len-1])) );
    }else{
    	m1 = min_max( &a[0..(len/2)] );
     	m2 = min_max( &a[((len/2))..(len)]);
    }
 
    return (min((m1.0, m2.0)), max((m1.1,m2.1)) );
}

fn min(a : (u32,u32)) -> u32{
	if a.0 <= a.1 {
		return a.0;
	}else{
		return a.1;
	}
}

fn max(a : (u32,u32)) -> u32{
	if a.0 >= a.1 {
		return a.0;
	}else{
		return a.1;
	}
}

// NOTE:
// cmp::min(a,b) returns the minimum of a and b
// cmp::max(a,b) returns the maximum of a and b

