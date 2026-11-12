import sys, json, cv2, numpy as np
sys.path.insert(0, __import__("os").path.dirname(__import__("os").path.abspath(__file__)))
import trace as T
U = sys.argv[1] + "/front-{}-large.jpg"
plates = {
    "3": ((159, 2447, 6), (94, 1619, 4), lambda r, c: 1 + 4 * r + c),
    "4": ((137, 2439, 6), (43, 1564, 4), lambda r, c: 25 + 8 * r + c),
    "5": ((172, 2474, 6), (104, 1633, 4), lambda r, c: 29 + 8 * r + c),
    "6": ((119, 859, 2), (48, 1530, 4), lambda r, c: 73 + 4 * r + c),
}
report = {}
for key, ((y0, y1, nr), (x0, x1, nc), fig) in plates.items():
    gray = cv2.imread(U.format(key), cv2.IMREAD_GRAYSCALE)
    for r in range(nr):
        for c in range(nc):
            n = fig(r, c)
            h, w = (y1 - y0) / nr, (x1 - x0) / nc
            cy, cx = int(y0 + (r + .5) * h), int(x0 + (c + .5) * w)
            half = int(max(h, w) / 2) + 40
            cell = gray[max(cy - half, 0):cy + half, max(cx - half, 0):cx + half].copy()
            ink = T.binarize(cell)
            S = T.SCALE
            for k in [(260 * S, 1), (1, 260 * S)]:
                ln = cv2.morphologyEx(ink, cv2.MORPH_OPEN, cv2.getStructuringElement(cv2.MORPH_RECT, k))
                ink = cv2.subtract(ink, cv2.dilate(ln, np.ones((5, 5), np.uint8)))
            cs, _ = cv2.findContours(ink, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_NONE)
            best = None
            for cn in cs:
                (ex, ey), er = cv2.minEnclosingCircle(cn)
                if 168 * S < er < 205 * S and cv2.contourArea(cn) / (np.pi * er * er) > 0.8:
                    if best is None or er > best[2]: best = (ex, ey, er)
            if best is None:
                hc = cv2.HoughCircles(cv2.GaussianBlur(cell, (5, 5), 1.5), cv2.HOUGH_GRADIENT, 1, 5, param1=100, param2=30, minRadius=165, maxRadius=200)
                if hc is None:
                    report[n] = "no ring"; continue
                top = hc[0][:6]
                hx, hy, hr = np.median(top[:, 0]), np.median(top[:, 1]), top[:, 2].max()
                best = (hx * S, hy * S, (hr + 4) * S)
            ex, ey, er = best
            mask = np.zeros_like(ink); cv2.circle(mask, (int(ex), int(ey)), int(er + 2 * S), 255, -1)
            ink = cv2.bitwise_and(ink, mask)
            paths = T.vectorize(ink)
            traced = T.render(paths, ink.shape)
            iou = np.count_nonzero(ink & traced) / np.count_nonzero(ink | traced)
            open(f"{sys.argv[2]}/fig-{n:02d}.svg", "w").write(T.svg(paths, ink.shape, (ex, ey, er)))
            report[n] = round(iou, 4)
print({k: v for k, v in report.items() if not isinstance(v, float) or v < 0.97})
print(len([v for v in report.values() if isinstance(v, float)]), "traced; min IoU", min(v for v in report.values() if isinstance(v, float)))
