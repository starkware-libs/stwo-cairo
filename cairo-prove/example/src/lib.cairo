
#[executable]
fn main(mut n: felt252) -> felt252 {
    let res = fib(1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,n);
    return res;
}

fn fib(mut a: felt252, mut b: felt252, mut c: felt252, mut d: felt252, mut e: felt252, mut f: felt252, mut g: felt252, mut h: felt252, mut i: felt252, mut j: felt252, mut k: felt252, mut l: felt252, mut m: felt252, mut o: felt252, mut p: felt252, mut q: felt252, mut n: felt252) -> felt252 {

    while n != 1 {
        let temp = a + b + c + d + e + f + g + h + i + j + k + l + m + o + p + q;
        a = b+1;
        b = c+1;
        c = d+1;
        d = e+1;
        e = f+1;
        f = g+1;
        g = h+1;
        h = i+1;
        i = j+1;
        j = k+1;
        k = l+1;
        l = m+1;
        m = o+1;
        o = p+1;
        p = q+1;
        q = temp + 1;
        n = n - 1;
    }
    return q;
}