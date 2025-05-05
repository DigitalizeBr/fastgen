from fastapi import FastAPI

app = FastAPI()

@app.get("/")
def root():
    return {"msg": "Olá do {{ project_name }}!"}