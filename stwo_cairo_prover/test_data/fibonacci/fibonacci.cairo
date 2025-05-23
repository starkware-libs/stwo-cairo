func fibonacci(n: felt) -> felt {
    tempvar n = n - 2;
    tempvar i = 1;
    tempvar j = 1;

    body:
    let n = [ap - 3];
    let i = [ap - 2];
    let j = [ap - 1];
    let next = i + j;

    tempvar n = n - 1;
    tempvar i = j;
    tempvar j = next;

    static_assert n == [ap - 3];
    static_assert i == [ap - 2];
    static_assert j == [ap - 1];

    jmp body if n != 0;

    end:
    let n = [ap - 3];
    let i = [ap - 2];
    let j = [ap - 1];

    return j;
}

func main() {
    fibonacci(4);
    return ();
}
