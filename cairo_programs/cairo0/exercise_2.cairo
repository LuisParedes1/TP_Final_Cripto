func main() {
    let result: felt = factorial(5);

    assert result = 120;
    ret;
}

func factorial(n) -> felt {
    if (n == 1){
        return 1;
    }
    let aux: felt = factorial(n - 1);
    return n * aux;
}
