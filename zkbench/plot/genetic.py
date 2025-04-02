import json

from matplotlib import pyplot as plt


def plot_genetic(stats: str): 
    stats = json.loads(open(stats).read())

    plt.plot(stats['values'], marker='o', linestyle='-')

    plt.xlabel('Iteration')
    plt.ylabel('Metric value')
    # TODO: include specific configuration in stats file too
    plt.title('Metric over Iterations')

    plt.grid()
    plt.show()
