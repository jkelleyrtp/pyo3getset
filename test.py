import hashcore

from multiprocessing import Process


def runbehavior(name):
    state = hashcore.PyAgentState()

    def behavior(state):
        state.set("age", 100)
        state.set("age1", 100)
        state.set("age2", 100)
        state.set("age3", 100)
        state.set("age4", 100)
        state.set("age5", 100)
        age = state.get("age")
        age = state.get("age1")
        age = state.get("age2")
        age = state.get("age3")
        age = state.get("age4")
        age = state.get("age5")

    behavior(state)


if __name__ == '__main__':
    p = Process(target=runbehavior)
    p.start()
    p.join()
