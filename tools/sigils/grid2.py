import sys, cv2, numpy as np
im = cv2.imread(sys.argv[1], cv2.IMREAD_GRAYSCALE)
bw = (im < 120).astype(np.uint8)
h, w = bw.shape
def lines(profile, length, frac):
    idx = np.where(profile > length * frac)[0]
    if not len(idx): return []
    groups, cur = [], [idx[0]]
    for i in idx[1:]:
        if i - cur[-1] <= 4: cur.append(i)
        else: groups.append(cur); cur = [i]
    groups.append(cur)
    return [int(np.mean(g)) for g in groups]
hk = cv2.morphologyEx(bw, cv2.MORPH_OPEN, cv2.getStructuringElement(cv2.MORPH_RECT, (60, 1)))
vk = cv2.morphologyEx(bw, cv2.MORPH_OPEN, cv2.getStructuringElement(cv2.MORPH_RECT, (1, 60)))
print("rows", lines(hk.sum(1), w, 0.3))
print("cols", lines(vk.sum(0), h, 0.3))
