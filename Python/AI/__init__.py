import langchain
import langchain_community
import langchain_community.llms
import langchain_core
import langchain_core.prompts
import subprocess
llm_llama = langchain_community.llms.Ollama(model="llama2")
llm_mistral_ger = langchain_community.llms.Ollama(model="marco/em_german_mistral_v01")
subprocess.run(["service", "ollama", "start"])


def detect_language(question: str):
    prompt = langchain_core.prompts.ChatPromptTemplate.from_messages([
        ("system",
         "You are a helpful assistan that sorts questions by their language. You need to state in a short sentence which language the question is."),
        ("user", "{question}")
    ])
    return (prompt | llm_llama).invoke({"question": question})


for question in ["Was ist Google?","What is google?","How do you start a computer?","Wie startet man einen Computer?"]:
    print(question,detect_language(question))
subprocess.run(["service", "ollama", "stop"])
