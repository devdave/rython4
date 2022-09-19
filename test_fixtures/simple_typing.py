

def add(a: int, b: int) -> int:
    c = a + b # type: int

    return c


if __name__ == '__main__':
    assert (r := add(5, 2)) == 7, f"Expected 7 but got {r}"

