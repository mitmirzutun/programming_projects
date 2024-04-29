import subprocess

import core
import my_turtle
import AI


#@core.build
def main():
    with AI.LangchainSingleton() as langchain_singleton:
        print(langchain_singleton.detect_language("Hello World"))
        print(langchain_singleton.generate_story("The hunters of Athemis from Percy Jackson find Camp Jupiter before Heroes of Olympus"))


if __name__ == '__main__':
    main()
    #input("Press any key to exit...")

# See PyCharm help at https://www.jetbrains.com/help/pycharm/
