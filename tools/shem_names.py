import re
v19="ויסע מלאך האלהים ההלך לפני מחנה ישראל וילך מאחריהם ויסע עמוד הענן מפניהם ויעמד מאחריהם"
v20="ויבא בין מחנה מצרים ובין מחנה ישראל ויהי הענן והחשך ויאר את הלילה ולא קרב זה אל זה כל הלילה"
v21="ויט משה את ידו על הים "+"ויולךיהוהאתהיםברוחקדיםעזהכלהלילהוישםאתהיםלחרבהויבקעוהמים"
fin=str.maketrans("ךםןףץ","כמנפצ")
L=lambda s: re.sub(r"[^א-ת]","",s).translate(fin)
a,b,c=L(v19),L(v20)[::-1],L(v21)
print(len(a),len(L(v20)),len(c))
names=[a[i]+b[i]+c[i] for i in range(72)]
rudd={5:"Mahasiah/Marbas",6:"Lelahel/Valefar",7:"Achaiah/Amon"}
for i,n in enumerate(names,1):
    print(i,n,end="  " if i%8 else "\n")
