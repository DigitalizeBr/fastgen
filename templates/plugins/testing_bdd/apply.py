import shutil
import os

def apply(project_path: str):
    src_dir = os.path.join(os.path.dirname(__file__), 'files')
    dst_dir = os.path.join(project_path)

    # Copia os arquivos do plugin para o projeto
    shutil.copytree(src_dir, dst_dir, dirs_exist_ok=True)

    print("[plugin-testing-bdd] Testes unitários e BDD configurados com sucesso.")
