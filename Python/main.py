import core
import my_turtle


@core.build
def main():
    import milans_pyo3_library
    # my_turtle.AnalogClock().run()
    step=2**20
    result=0
    for start in range(0,2**24,step):
        result+=sum(map(milans_pyo3_library.miller_rabin_test,range(start,start+step)))
        print(hex(start),result)


if __name__ == '__main__':
    main()
    input("Press any key to exit...")

# See PyCharm help at https://www.jetbrains.com/help/pycharm/
