import subprocess

import core
import my_turtle
import AI


@core.build
def main():
    import milans_pyo3_library
    # my_turtle.AnalogClock().run()
    for i in range(1, 16):
        if not milans_pyo3_library.miller_rabin_test((1 << i)-1):
            print(i, (1 << i)-1, milans_pyo3_library.prime_divisors((1 << i)-1))
        else:
            print(i, (1 << i)-1)
        if not milans_pyo3_library.miller_rabin_test((1 << i)+1):
            print(i, (1 << i)+1, milans_pyo3_library.prime_divisors((1 << i)+1))
        else:
            print(i, (1 << i)+1)


if __name__ == '__main__':
    main()
    #input("Press any key to exit...")

# See PyCharm help at https://www.jetbrains.com/help/pycharm/
