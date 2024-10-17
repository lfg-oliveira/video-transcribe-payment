from pydantic import BaseModel

class Plan(BaseModel):
    id: int
    amount: str = "5.00"
class PixInfo(BaseModel):
    user_id: int = 1
    plan: Plan 