from tokenize import String
from typing import Literal
import sqlalchemy
from sqlalchemy.orm import DeclarativeBase
from sqlalchemy.orm import Mapped
from sqlalchemy.orm import mapped_column
from sqlalchemy.orm import relationship


engine = sqlalchemy.create_engine('mysql+pymysql://root@localhost/video_transcribe')

class Base(DeclarativeBase):
    pass


class PaymentLog(Base):
    __tablename__="payment_log"
    txid: Mapped[str] = mapped_column(sqlalchemy.String(255), primary_key=True)
    user_id: Mapped[int] = mapped_column(sqlalchemy.Integer, primary_key=True)
    status: Mapped[Literal["Paid"] | Literal["Waiting"]] = mapped_column(sqlalchemy.String(len("Waiting")), primary_key=True)
    plan_id: Mapped[int] = mapped_column(sqlalchemy.Integer)
    created_at = sqlalchemy.Column(sqlalchemy.DateTime(timezone=True), server_default=sqlalchemy.func.now())

Base.metadata.create_all(engine)