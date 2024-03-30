import core
import my_turtle


@core.build
def main():
    import milans_pyo3_library
    # my_turtle.AnalogClock().run()
    for i in range(1,64):
        print(i,milans_pyo3_library.prime_divisors((1<<i)-1))
        print(i,milans_pyo3_library.prime_divisors((1<<i)+1))


if __name__ == '__main__':
    main()
    input("Press any key to exit...")

# See PyCharm help at https://www.jetbrains.com/help/pycharm/
