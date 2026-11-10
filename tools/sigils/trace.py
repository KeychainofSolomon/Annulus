import sys, cv2, numpy as np

SCALE = 4

def load_cell(path, x0, y0, x1, y1, inset=6):
    gray = cv2.imread(path, cv2.IMREAD_GRAYSCALE)
    return gray[y0 + inset:y1 - inset, x0 + inset:x1 - inset]

def binarize(cell):
    big = cv2.resize(cell, None, fx=SCALE, fy=SCALE, interpolation=cv2.INTER_CUBIC)
    big = cv2.GaussianBlur(big, (0, 0), SCALE * 0.6)
    _, ink = cv2.threshold(big, 0, 255, cv2.THRESH_BINARY_INV | cv2.THRESH_OTSU)
    return ink

def isolate_seal(ink):
    contours, _ = cv2.findContours(ink, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_NONE)
    outer = max(contours, key=cv2.contourArea)
    (cx, cy), radius = cv2.minEnclosingCircle(outer)
    mask = np.zeros_like(ink)
    cv2.circle(mask, (int(cx), int(cy)), int(radius + 2), 255, -1)
    return cv2.bitwise_and(ink, mask), (cx, cy, radius)

def vectorize(ink, epsilon=1.2):
    contours, hierarchy = cv2.findContours(ink, cv2.RETR_CCOMP, cv2.CHAIN_APPROX_NONE)
    paths = []
    for contour in contours:
        if cv2.contourArea(contour) < (SCALE * 1.5) ** 2:
            continue
        simplified = cv2.approxPolyDP(contour, epsilon, True)
        if len(simplified) >= 3:
            paths.append(simplified.reshape(-1, 2))
    return paths

def svg(paths, size, circle):
    cx, cy, r = circle
    vb = f"{cx - r - 4:.1f} {cy - r - 4:.1f} {2 * r + 8:.1f} {2 * r + 8:.1f}"
    d = " ".join("M" + " L".join(f"{x},{y}" for x, y in p) + " Z" for p in paths)
    return f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="{vb}"><path fill="#000" fill-rule="evenodd" d="{d}"/></svg>'

def render(paths, shape):
    canvas = np.zeros(shape, np.uint8)
    cv2.fillPoly(canvas, [p.astype(np.int32) for p in paths], 255)
    return canvas

if __name__ == "__main__":
    path, name = sys.argv[1], sys.argv[2]
    x0, y0, x1, y1 = map(int, sys.argv[3:7])
    out = sys.argv[7]
    cell = load_cell(path, x0, y0, x1, y1)
    ink, circle = isolate_seal(binarize(cell))
    paths = vectorize(ink)
    traced = render(paths, ink.shape)
    union = np.count_nonzero(cv2.bitwise_or(ink, traced))
    inter = np.count_nonzero(cv2.bitwise_and(ink, traced))
    open(f"{out}/{name}.svg", "w").write(svg(paths, ink.shape, circle))
    original = cv2.resize(cell, None, fx=SCALE, fy=SCALE, interpolation=cv2.INTER_CUBIC)
    trace_view = 255 - traced
    overlay = cv2.cvtColor(original, cv2.COLOR_GRAY2BGR)
    overlay[(traced > 0) & (ink == 0)] = (0, 0, 255)
    overlay[(traced == 0) & (ink > 0)] = (255, 0, 0)
    sheet = np.hstack([cv2.cvtColor(original, cv2.COLOR_GRAY2BGR), cv2.cvtColor(trace_view, cv2.COLOR_GRAY2BGR), overlay])
    sheet = cv2.resize(sheet, None, fx=0.5, fy=0.5, interpolation=cv2.INTER_AREA)
    cv2.imwrite(f"{out}/{name}-compare.png", sheet)
    print(f"{name}: contours={len(paths)} points={sum(len(p) for p in paths)} IoU={inter/union:.4f}")
