from urllib import request
from fastapi import FastAPI
import requests
import sqlalchemy.orm

from definitions import CERT_PATH, CLIENT_ID, CLIENT_SECRET
import log
from pix_info import PixInfo
import sqlalchemy
app = FastAPI()

cert = 'homologacao-619113-VTH_cert.pem'
s = requests.Session()
s.cert = CERT_PATH
base = 'https://pix-h.api.efipay.com.br'
chave = '9c61202f-9c2a-4503-8003-f6b1f79e8899'

@app.get('/')
def healthcheck():
    return "API online"

@app.post('/create-pix')
def get_pix(pix_info: PixInfo) -> str:
    s.auth = (CLIENT_ID, CLIENT_SECRET)
    token = s.post(base+'/oauth/token', json={"grant_type": "client_credentials"}).json()
    s.auth = None
    res = s.post(base+'/v2/cob', json={
        "calendario": {
            "expiracao": 3600
        },
        "valor": {
            "original": pix_info.plan.amount
        },
        "chave": chave
    }, headers={"Authorization": "Bearer "+token['access_token']}).json()

    with sqlalchemy.orm.Session(log.engine) as session:
        session.begin()
        session.add(log.PaymentLog(txid=res['txid'], user_id=pix_info.user_id, plan_id=pix_info.plan.id, status="Waiting"))
        session.commit()

    
    return res['pixCopiaECola']