import langchain
import langchain_community
import langchain_community.llms
import langchain_core
import langchain_core.prompts
import subprocess


class LangchainSingleton:
    __instance = None
    __count = 0

    def __new__(cls):
        LangchainSingleton.__count += 1
        if LangchainSingleton.__instance is None:
            LangchainSingleton.__instance = super(LangchainSingleton, cls).__new__(cls)
            subprocess.run(["service", "ollama", "start"])
            instance = LangchainSingleton.__instance
            instance.__llm_llama = langchain_community.llms.Ollama(model="llama2")
            instance.__llm_mistral_ger = langchain_community.llms.Ollama(model="marco/em_german_mistral_v01")
            instance.__language_prompt = langchain_core.prompts.ChatPromptTemplate.from_messages([
                ("system",
                 "You are a helpful assistant that sorts questions by their language. You are stating the language of the question to your boss,"),
                ("user", "{question}")
            ])
        return LangchainSingleton.__instance

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        if LangchainSingleton.__count > 1:
            LangchainSingleton.__count -= 1
        elif LangchainSingleton.__count == 1:
            LangchainSingleton.__instance = None
            LangchainSingleton.__count = 0
            subprocess.run(["service", "ollama", "stop"])

    def detect_language(self, question: str):
        return (self.__language_prompt | self.__llm_llama).invoke({"question": question})
    def en(self):
        return self.__llm_llama
    def generate_story(self,story_prompt):
        prompt=langchain_core.prompts.ChatPromptTemplate.from_messages([
            ("system","You are a writer that writes stories of your choice. These stories are 10000 words and longer"),
            ("user","{story_prompt}")
        ])
        return (prompt | self.__llm_llama).invoke({"story_prompt":story_prompt})