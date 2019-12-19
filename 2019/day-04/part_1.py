from input import min, max

if __name__ == "__main__":
    print(f"min: {min}, max: {max}")
    passwords = 0
    for i in range(min, max):
        password = str(i)
        pair = False
        increasing = True
        for loc in range(1, len(password)):
            if int(password[loc]) < int(password[loc-1]):
                increasing = False
                break
            if password[loc] == password[loc-1]:
                pair = True
        if increasing and pair:
            passwords += 1

    print(f"passwords: {passwords}")
