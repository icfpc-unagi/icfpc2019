import sys

import numpy as np
import matplotlib.pyplot as plt


def plot_sqs(xys):
    xs, ys = np.array(xys).T + 0.5
    plt.scatter(xs, ys, s=3)


def plot_ans(path):
    with open(path) as f:
        s = f.read()
        po = s.split('#')[0]
        po = eval('[{}]'.format(po))
        po.append(po[0])
        xs, ys = zip(*po)
        plt.plot(xs, ys, color='C2', linewidth=1)


argv = dict(enumerate(sys.argv))
path = argv.get(1, "../data/chain-puzzle-examples/puzzle.cond")
anspath = argv.get(2)

with open(path) as f:
    s = f.read()
    nums, isqs, osqs = s.split('#')
    tsize = int(nums.split(',')[2])
    isqs = eval('[{}]'.format(isqs))
    osqs = eval('[{}]'.format(osqs))
    plt.gca().set_aspect('equal')
    plt.plot([0, tsize, tsize, 0, 0], [0, 0, tsize, tsize, 0], color='C3')
    if anspath:
        plot_ans(anspath)
    plot_sqs(isqs)
    plot_sqs(osqs)
    """
    print(isqs)
    print(osqs)
    img = np.zeros((tsize, tsize, 3), np.uint8)
    for x, y in isqs:
        img[x, y] = [0, 128, 255]
    for x, y in osqs:
        img[x, y] = [255, 0, 0]
    plt.imshow(img)
    """
    plt.show()
