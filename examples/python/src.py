def solve(factors: list[int], upper_bound: int) -> int:
    s = 0
    for multiple in range(1,upper_bound):
        for factor in factors:
            if multiple % factor == 0:
                s = s + multiple
                break
    return s
