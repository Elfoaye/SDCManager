import pandas as pd
import sqlite3
import os

# print(os.getcwd())
file = "database_import/FichePartenairesSDC.xlsx"
# print(os.path.exists(file))

ex = pd.read_excel(file)
print(ex)

imported_column = {
    "Détail Mise à disposition": "nom",
    "Type": "type",
    "Dispo": "total",
    "Val.Remp.": "valeur",
    "Contrib.": "contrib"
}

ex["total"] = ex["Dispo"]
ex["dispo"] = ex["Dispo"]
ex = ex[list(imported_column.keys())]
ex = ex.rename(columns=imported_column)

db = sqlite3.connect("sync_data/database.db")
cursor = db.cursor()

ex.to_sql("Materiel", db, if_exists="append", index=False)

db.close()