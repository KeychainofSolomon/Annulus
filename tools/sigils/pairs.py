import cv2, numpy as np
A = "/mnt/user-data/attachments/895ef19f-f12d-5eee-8009-17869487ca69/"
P1 = (A + "25f4b79b-image.jpg", [158, 530, 912, 1296, 1679, 2064, 2446], [96, 479, 859, 1248, 1624])
P2 = (A + "d0c7b923-image.jpg", [171, 551, 934, 1322, 1708, 2090, 2474], [104, 485, 867, 1253, 1634])
P3 = (A + "e8dc6e53-image.jpg", [119, 489, 859], [47, 421, 792])
pairs = [
    ("Paimon (spirit 9)", P1, (2, 0), (2, 1), "fig. 9", "fig. 10"),
    ("Beleth (spirit 13)", P1, (3, 1), (3, 2), "fig. 14", "fig. 15"),
    ("Leraje (spirit 14)", P1, (3, 3), (4, 0), "fig. 16", "fig. 17"),
    ("Bathin (spirit 18)", P1, (5, 0), (5, 1), "fig. 21", "fig. 22"),
    ("Bune (spirit 26)", P2, (0, 1), (0, 2), "fig. 30", "fig. 31"),
    ("Vepar (spirit 42)", P2, (2, 2), (2, 3), "fig. 47", "fig. 48"),
    ("Vual (spirit 47)", P2, (3, 0), (3, 1), "fig. 53", "fig. 54"),
    ("Seere (spirit 70)", P3, (1, 0), (1, 1), "fig. 77", "fig. 78"),
]
cache = {}
def cell(plate, rc):
    path, rows, cols = plate
    if path not in cache: cache[path] = cv2.imread(path)
    r, c = rc
    crop = cache[path][rows[r] + 4:rows[r + 1] - 4, cols[c] + 4:cols[c + 1] - 4]
    return cv2.resize(crop, (360, 360), interpolation=cv2.INTER_AREA)
tiles = []
for name, plate, a, b, la, lb in pairs:
    label = np.full((360, 300, 3), 255, np.uint8)
    cv2.putText(label, name.split(" (")[0], (12, 160), cv2.FONT_HERSHEY_SIMPLEX, 1.1, (0, 0, 0), 2, cv2.LINE_AA)
    cv2.putText(label, "(" + name.split(" (")[1], (12, 205), cv2.FONT_HERSHEY_SIMPLEX, 0.75, (60, 60, 60), 1, cv2.LINE_AA)
    left, right = cell(plate, a), cell(plate, b)
    for img, text in ((left, la), (right, lb)):
        cv2.putText(img, text, (250, 350), cv2.FONT_HERSHEY_SIMPLEX, 0.7, (0, 0, 200), 2, cv2.LINE_AA)
    tiles.append(np.hstack([label, left, np.full((360, 20, 3), 255, np.uint8), right]))
halves = [np.vstack(tiles[:4]), np.vstack(tiles[4:])]
sheet = np.hstack([halves[0], np.full((halves[0].shape[0], 40, 3), 255, np.uint8), halves[1]])
cv2.imwrite("out/alternate-forms.png", sheet)
print(sheet.shape)
