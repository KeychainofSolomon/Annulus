import sys, cv2, numpy as np
path = sys.argv[1]
im = cv2.imread(path, cv2.IMREAD_GRAYSCALE)
bw = (im < 110).astype(np.uint8)
h, w = bw.shape
def lines(profile, length, minfrac):
    idx = np.where(profile > length * minfrac)[0]
    groups, cur = [], [idx[0]] if len(idx) else []
    for i in idx[1:]:
        if i - cur[-1] <= 3: cur.append(i)
        else: groups.append(cur); cur = [i]
    if cur: groups.append(cur)
    return [int(np.mean(g)) for g in groups]
hk = cv2.morphologyEx(bw, cv2.MORPH_OPEN, cv2.getStructuringElement(cv2.MORPH_RECT, (w // 6, 1)))
vk = cv2.morphologyEx(bw, cv2.MORPH_OPEN, cv2.getStructuringElement(cv2.MORPH_RECT, (1, h // 20)))
print("rows", lines(hk.sum(1), w, 0.5))
print("cols", lines(vk.sum(0), h, 0.25))
