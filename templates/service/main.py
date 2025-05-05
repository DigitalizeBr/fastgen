from fastapi import FastAPI

app = FastAPI()

@app.get("/")
def root():
    return {"message": "Olá de {{ service_name }}!"}