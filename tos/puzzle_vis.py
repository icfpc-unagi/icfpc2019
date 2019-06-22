import numpy as np
import matplotlib.pyplot as plt

with open("../data/chain-puzzle-examples/puzzle.cond") as f:
    s = f.read()
    nums, isqs, osqs = s.split('#')
    tsize = int(nums.split(',')[2])
    isqs = eval('[{}]'.format(isqs))
    osqs = eval('[{}]'.format(osqs))
    print(isqs)
    print(osqs)
    img = np.zeros((tsize, tsize, 3), np.uint8)
    for x, y in isqs:
        img[x, y] = [0, 0, 255]
    for x, y in osqs:
        img[x, y] = [255, 0, 0]
    plt.imshow(img)
    plt.show()
